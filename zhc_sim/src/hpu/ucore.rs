use std::collections::VecDeque;

use zhc_langs::{
    doplang::{DopInstructionSet, UserFlag, VirtId},
    hpulang::TransferId,
};
use zhc_utils::{FastMap, fsm};

use crate::Dispatch;

use super::*;

/// Lifecycle of an inbound transfer tracked by a [`UCore`].
///
/// A transfer is `Awaited` once its `LD_B2B` marker is seen, becomes `Loading`
/// when the source board notifies its readiness and the DMA starts, and reaches
/// `Loaded` when the DMA completes and the corresponding `WAIT` may proceed.
#[fsm]
#[derive(Debug, Serialize)]
pub enum InboundTransferState {
    Awaited,
    Loading,
    Loaded,
}

/// Scheduling condition of a [`UCore`].
///
/// `Starved` has no pending work; `Incuring` is actively draining its DOP queue
/// to the instruction scheduler; and `WaitingTransferIn` is blocked until an inbound
/// transfer finishes loading.
#[fsm]
#[derive(Debug, Serialize)]
pub enum UCoreCondition {
    Starved,
    Incuring,
    WaitingTransferIn,
}

/// Micro-core that sequences an HPU's DOP stream and mediates inter-HPU
/// transfers.
///
/// Forwards ordinary compute operations to the instruction scheduler while
/// intercepting the `LD_B2B`, `WAIT`, and `NOTIFY` virtual operations to run the
/// cross-board transfer handshake, stalling the stream on outstanding inbound
/// transfers as tracked by its [`UCoreCondition`] and per-transfer
/// [`InboundTransferState`].
///
/// A `NOTIFY` does not stall it. The micro-core replaces it with an inner `SYNC` pushed to the
/// scheduler and reads on, and the destination board is signalled when that `SYNC` retires. Only a
/// `WAIT` blocks.
#[derive(Debug, Serialize)]
pub struct UCore {
    dops: VecDeque<DOp>,
    inbound_transfers: FastMap<TransferId, InboundTransferState>,
    outbound_transfers: FastMap<DOpId, (HpuId, TransferId)>,
    mhdma_latency: ConstantLatency,
    condition: UCoreCondition,
    /// Inner `SYNC` operations standing in for a `NOTIFY`, and the transfer each one signals.
    pending_notifies: FastMap<DOpId, (HpuId, TransferId)>,
}

impl UCore {
    /// Creates an idle micro-core whose inbound transfers each take
    /// `mhdma_latency` to load.
    pub fn new(mhdma_latency: ConstantLatency) -> Self {
        UCore {
            dops: VecDeque::new(),
            inbound_transfers: FastMap::default(),
            outbound_transfers: FastMap::default(),
            condition: UCoreCondition::Starved,
            mhdma_latency,
            pending_notifies: FastMap::default(),
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
                                flag: UserFlag { flag },
                                ..
                            } = self.dops.pop_front().unwrap().raw
                            else {
                                unreachable!()
                            };
                            match self
                                .inbound_transfers
                                .insert(TransferId(flag), InboundTransferState::Awaited)
                            {
                                None => {
                                    // No transfers registered yet for this tid.
                                }
                                Some(InboundTransferState::Loaded) => {
                                    // Tid already used. Recycling.
                                }
                                s => panic!(
                                    "Encountered invalid transfer state {s:?} for flag {flag}"
                                ),
                            }
                        }
                        Some(WAIT {
                            flag: UserFlag { flag },
                            ..
                        }) => {
                            match self.inbound_transfers.get(&TransferId(*flag)) {
                                Some(InboundTransferState::Loading)
                                | Some(InboundTransferState::Awaited) => {
                                    self.condition.transition(|old| match old {
                                        UCoreCondition::Incuring => {
                                            // UCore stalls, waiting for the transfer.
                                            UCoreCondition::WaitingTransferIn
                                        }
                                        UCoreCondition::WaitingTransferIn => {
                                            // Ucore was stalled, and likely got wake up by a later
                                            // inbound transfer.
                                            UCoreCondition::WaitingTransferIn
                                        }
                                        s => unreachable!("Encountered unexpected state {:?}", s),
                                    });
                                    break;
                                }
                                Some(InboundTransferState::Loaded) => {
                                    self.dops.pop_front().unwrap();
                                    self.condition.transition(|old| match old {
                                        UCoreCondition::Incuring => {
                                            // Fast path -> The transfer landed before the UCore
                                            // arrived to it.
                                            UCoreCondition::Incuring
                                        }
                                        UCoreCondition::WaitingTransferIn => {
                                            // Unstalling of the UCore.
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
                                        virt_id: VirtId { id: hid },
                                        flag: UserFlag { flag },
                                        ..
                                    },
                                id,
                            } = self.dops.pop_front().unwrap()
                            else {
                                unreachable!()
                            };
                            self.outbound_transfers
                                .insert(id, (HpuId(hid), TransferId(flag)));
                            dispatcher.dispatch_now(Events::IscPushDOp(DOp {
                                raw: SYNC {
                                    is_inner: true,
                                    flag: UserFlag { flag },
                                    hid: VirtId { id: hid },
                                    iid: 0,
                                },
                                id,
                            }));
                            continue;
                        }
                        Some(_) => {
                            let dop = self.dops.pop_front().unwrap();
                            self.condition.transition(|old| match old {
                                UCoreCondition::Incuring => UCoreCondition::Incuring,
                                s => unreachable!("Encountered unexpected state {:?}", s),
                            });
                            dispatcher.dispatch_now(Events::IscPushDOp(dop));
                        }
                    }
                }
                assert!(matches!(
                    self.condition,
                    UCoreCondition::Starved | UCoreCondition::WaitingTransferIn
                ));
            }
            Events::UCoreTransferInNotified(tid) => {
                assert!(self.inbound_transfers.contains_key(&tid));
                self.inbound_transfers
                    .get_mut(&tid)
                    .unwrap()
                    .transition(|old| match old {
                        InboundTransferState::Awaited => InboundTransferState::Loading,
                        _ => unreachable!(),
                    });
                dispatcher.dispatch_after(
                    self.mhdma_latency.compute_latency(),
                    Events::UCoreTransferInFinished(tid),
                );
            }
            Events::UCoreTransferInFinished(tid) => {
                assert!(self.inbound_transfers.contains_key(&tid));
                self.inbound_transfers
                    .get_mut(&tid)
                    .unwrap()
                    .transition(|old| match old {
                        InboundTransferState::Loading => InboundTransferState::Loaded,
                        _ => unreachable!(),
                    });
                dispatcher.dispatch_now(Events::UCoreProcessDOps);
            }
            // A notification goes out when the inner `SYNC` standing in for its `NOTIFY` retires.
            Events::IscRetireDOp(ref dop) => {
                if let Some((hid, tid)) = self.outbound_transfers.remove(&dop.id) {
                    dispatcher.dispatch_now(Events::UCoreTransferOutReady(hid, tid));
                }
            }
            Events::IscStarved => match self.condition {
                // A board with notifications still in flight is not done: the peers waiting on
                // them have not been signalled yet.
                UCoreCondition::Starved if self.outbound_transfers.is_empty() => {
                    dispatcher.dispatch_now(Events::UCoreStarved);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn name(&self) -> String {
        "UCore".into()
    }
}
