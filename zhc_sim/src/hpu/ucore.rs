use std::collections::VecDeque;

use zhc_langs::{
    doplang::{Argument, DopInstructionSet},
    hpulang::TransferId,
};
use zhc_utils::{FastMap, fsm};

use crate::Dispatch;

use super::*;

#[fsm]
#[derive(Debug, Serialize)]
pub enum TransferState {
    Awaited,
    Loading,
    Loaded,
}

#[fsm]
#[derive(Debug, Serialize)]
pub enum UCoreCondition {
    Starved,
    Incuring,
    WaitingTransferIn,
    WaitingTransferOut{hid: HpuId, tid: TransferId},
}

#[derive(Debug, Serialize)]
pub struct UCore {
    dops: VecDeque<DOp>,
    transfers: FastMap<TransferId, TransferState>,
    mhdma_latency: ConstantLatency,
    condition: UCoreCondition,
}

impl UCore {
    pub fn new(mhdma_latency: ConstantLatency) -> Self {
        UCore {
            dops: VecDeque::new(),
            transfers: FastMap::new(),
            condition: UCoreCondition::Starved,
            mhdma_latency,
        }
    }
}

impl Simulatable for UCore {
    type Event = Events;
    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        use DopInstructionSet::*;
        match trigger.event {
            Events::UCorePushDOps(dops) => {
                self.dops.extend(dops);
                self.condition.transition(|old| match old {
                    UCoreCondition::Starved => UCoreCondition::Incuring,
                    _ => unreachable!(),
                });
                dispatcher.dispatch_now(Events::UCoreProcessDOps);
            }
            Events::UCoreProcessDOps => {
                loop {
                    match self.dops.front().map(|dop| &dop.raw) {
                        None => {
                            self.condition.transition(|old| match old {
                                UCoreCondition::Incuring => UCoreCondition::Starved,
                                _ => unreachable!(),
                            });
                            break;
                        }
                        Some(LD_B2B { .. }) => {
                            let LD_B2B {
                                flag: Argument::UserFlag { flag },
                                ..
                            } = self.dops.pop_front().unwrap().raw
                            else {
                                unreachable!()
                            };
                            assert!(
                                self.transfers
                                    .insert(TransferId(flag), TransferState::Awaited)
                                    .is_none()
                            );
                        }
                        Some(WAIT { flag: Argument::UserFlag { flag }, .. }) => {
                            match self.transfers.get(&TransferId(*flag)) {
                                Some(TransferState::Loading) | Some(TransferState::Awaited) => {
                                    self.condition.transition(|old| match old {
                                        UCoreCondition::Incuring | UCoreCondition::WaitingTransferIn => {
                                            UCoreCondition::WaitingTransferIn
                                        }
                                        s => unreachable!("Encountered unexpected state {:?}", s),
                                    });
                                    break;
                                }
                                Some(TransferState::Loaded) => {
                                    self.dops.pop_front().unwrap();
                                    self.condition.transition(|old| match old {
                                        UCoreCondition::Incuring
                                        | UCoreCondition::WaitingTransferIn => {
                                            UCoreCondition::Incuring
                                        }
                                        _ => unreachable!(),
                                    });
                                    continue;
                                }
                                None => panic!("Missing LD_B2B before wait"),
                                _ => unreachable!(),
                            };
                        }
                        Some(NOTIFY { .. }) => {
                            let DOp {
                                raw:
                                    NOTIFY {
                                        virt_id: Argument::VirtId { id: hid },
                                        flag: Argument::UserFlag { flag },
                                        ..
                                    },
                                id,
                            } = self.dops.pop_front().unwrap()
                            else {
                                unreachable!()
                            };
                            dispatcher.dispatch_now(Events::IscPushDOp(DOp { raw: SYNC, id }));
                            self.condition.transition(|old| match old {
                                UCoreCondition::Incuring => UCoreCondition::WaitingTransferOut{ hid: HpuId(hid), tid: TransferId(flag)},
                                s => unreachable!("Encountered unexpected state {:?}", s),
                            });
                            break;
                        }
                        Some(_) => {
                            let dop = self.dops.pop_front().unwrap();
                            self.condition.transition(|old| match old {
                                UCoreCondition::Incuring | UCoreCondition::WaitingTransferOut { .. } => UCoreCondition::Incuring,
                                s => unreachable!("Encountered unexpected state {:?}", s),
                            });
                            dispatcher.dispatch_now(Events::IscPushDOp(dop));
                        }
                    }
                }
                assert!(matches!(
                    self.condition,
                    UCoreCondition::Starved
                        | UCoreCondition::WaitingTransferIn
                        | UCoreCondition::WaitingTransferOut { .. }
                ));
            }
            Events::UCoreTransferInNotified(tid) => {
                assert!(self.transfers.contains_key(&tid));
                self.transfers
                    .get_mut(&tid)
                    .unwrap()
                    .transition(|old| match old {
                        TransferState::Awaited => TransferState::Loading,
                        _ => unreachable!(),
                    });
                dispatcher.dispatch_after(
                    self.mhdma_latency.compute_latency(),
                    Events::UCoreTransferInFinished(tid),
                );
            }
            Events::UCoreTransferInFinished(tid) => {
                assert!(self.transfers.contains_key(&tid));
                self.transfers
                    .get_mut(&tid)
                    .unwrap()
                    .transition(|old| match old {
                        TransferState::Loading => TransferState::Loaded,
                        _ => unreachable!(),
                    });
                dispatcher.dispatch_now(Events::UCoreProcessDOps);
            }
            Events::IscStarved => match self.condition {
                UCoreCondition::WaitingTransferIn => {
                    dispatcher.dispatch_now(Events::UCoreProcessDOps);
                },
                UCoreCondition::WaitingTransferOut {hid, tid} => {
                    dispatcher.dispatch_now(Events::UCoreTransferOutReady(hid, tid));
                    dispatcher.dispatch_now(Events::UCoreProcessDOps);
                },
                UCoreCondition::Starved => {
                    dispatcher.dispatch_now(Events::UCoreStarved);
                },
                _ => unreachable!()
            },
            _ => {}
        }
    }

    fn name(&self) -> String {
        "UCore".into()
    }
}
