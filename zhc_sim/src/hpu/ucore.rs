use std::collections::VecDeque;

use zhc_langs::{
    doplang::{CtMem, DopInstructionSet, UserFlag, VirtId},
    hpulang::N_FLAGS,
};
use zhc_utils::{FastMap, Store, existential_enum, fsm};

use crate::Dispatch;

use super::*;

#[existential_enum]
#[derive(Debug, Clone, Serialize)]
pub enum FlagState {
    Forbidden(()),
    Transfer(TransferState)
}

#[fsm]
#[derive(Debug, Clone, Serialize)]
pub enum TransferState {
    Fresh,
    Notified{src: HpuId, src_slot: CtMem},
    SyncWaiting,
    TransferDstReady{dst_slot: CtMem},
    Transfering{src: HpuId, src_slot: CtMem, dst_slot: CtMem},
    Transfered{dst_slot: CtMem},
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
    flags_table: Store<UserFlag, FlagState>,
    notify_table: FastMap<DOpId, (HpuId, UserFlag, CtMem)>,
    mhdma_latency: ConstantLatency,
    id: HpuId
}

impl UCore {
    /// Creates an idle micro-core whose inbound transfers each take
    /// `mhdma_latency` to load.
    pub fn new(id: HpuId, mhdma_latency: ConstantLatency) -> Self {
        let flags_table = std::iter::once(FlagState::Forbidden(()))
            .chain(std::iter::repeat(FlagState::Transfer(TransferState::Fresh)))
            .take(N_FLAGS as usize)
            .collect();
        UCore {
            dops: VecDeque::new(),
            flags_table,
            notify_table: FastMap::default(),
            mhdma_latency,
            id
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
                dispatcher.dispatch_now(Events::UCoreProcessDOps);
            }
            Events::UCoreProcessDOps => {
                loop {
                    match self.dops.front().map(|dop| &dop.raw) {
                        None => {
                            break;
                        }
                        Some(LD_B2B { .. }) => {
                            let LD_B2B {
                                flag,
                                slot
                            } = self.dops.pop_front().unwrap().raw
                            else {
                                unreachable!()
                            };
                            assert!(self.flags_table[flag].is_transfer(), "LD_B2B on non-transfer flag...");
                            self.flags_table[flag].unwrap_transfer_mut().transition(|old| match old {
                                TransferState::Fresh => {
                                    TransferState::TransferDstReady{dst_slot: slot}
                                }
                                TransferState::Notified{src, src_slot} => {
                                    dispatcher.dispatch_after(
                                        self.mhdma_latency.compute_latency(),
                                        Events::UCoreDmaCompleted(flag)
                                    );
                                    TransferState::Transfering{src, src_slot, dst_slot: slot}
                                }
                                s => panic!("Encountered unexpected flag state {s:?} while handling LD_B2B"),
                            });
                        }
                        Some(WAIT { flag, slot }) => {
                            let should_resume = match &mut self.flags_table[flag] {
                                FlagState::Forbidden(_) => panic!("Encountered WAIT on forbidden flag."),
                                FlagState::Transfer(transfer_state) => {
                                    transfer_state.transition_with(|old| match old {
                                        TransferState::Fresh => {
                                            // Transition only occurs when no LD_B2B exist. Meaning it's a sync.
                                            // And it's waiting.
                                            assert!(slot.is_none());
                                            (TransferState::SyncWaiting, false)
                                        }
                                        TransferState::SyncWaiting => {
                                            assert!(slot.is_none());
                                            (TransferState::SyncWaiting, false)
                                        }
                                        TransferState::Notified { .. } => {
                                            // Transition only occurs when no LD_B2B exist. Meaning it's a sync.
                                            // And it's complete.
                                            assert!(slot.is_none());
                                            (TransferState::Fresh, true)
                                        }
                                        TransferState::TransferDstReady { dst_slot } => {
                                            assert!(slot.is_some());
                                            assert_eq!(dst_slot, *slot.as_ref().unwrap());
                                            (TransferState::TransferDstReady { dst_slot }, false)
                                        }
                                        TransferState::Transfering { src, src_slot, dst_slot } => {
                                            assert!(slot.is_some());
                                            (TransferState::Transfering { src, src_slot, dst_slot }, false)
                                        },
                                        TransferState::Transfered { .. } => {
                                            assert!(slot.is_some());
                                            (TransferState::Fresh, true)
                                        },
                                        s => panic!("Encountered unexpected flag state {s:?} while handling WAIT on transfer flag"),
                                    })
                                },
                            };
                            if should_resume {
                                self.dops.pop_front().unwrap();
                                continue;
                            } else {
                                break;
                            }
                        }
                        Some(NOTIFY { .. }) => {
                            let DOp {
                                raw:
                                    NOTIFY {
                                        virt_id: VirtId { id: hid },
                                        flag,
                                        slot
                                    },
                                id,
                            } = self.dops.pop_front().unwrap()
                            else {
                                unreachable!()
                            };

                            self.notify_table.insert(id, (HpuId(hid), flag, slot));
                            dispatcher.dispatch_now(Events::IscPushDOp(DOp {
                                raw: SYNC {
                                    is_inner: true,
                                    flag,
                                    hid: VirtId { id: hid },
                                    iid: 0,
                                },
                                id,
                            }));
                            continue;
                        }
                        Some(_) => {
                            let dop = self.dops.pop_front().unwrap();
                            dispatcher.dispatch_now(Events::IscPushDOp(dop));
                        }
                    }
                }
            }
            Events::UCoreNotified(flag, hid, slot) => {
                match &mut self.flags_table[flag] {
                    FlagState::Forbidden(_) => panic!("Notified on forbidden flag..."),
                    FlagState::Transfer(transfer_state) => {
                        transfer_state.transition(|old| match old {
                            TransferState::Fresh => {
                                TransferState::Notified { src: hid, src_slot: slot }
                            },
                            TransferState::SyncWaiting => {
                                dispatcher.dispatch_now(Events::UCoreProcessDOps);
                                TransferState::Notified { src: hid, src_slot: slot }
                            }
                            TransferState::TransferDstReady { dst_slot } => {
                                dispatcher.dispatch_after(
                                    self.mhdma_latency.compute_latency(),
                                    Events::UCoreDmaCompleted(flag)
                                );
                                TransferState::Transfering{src: hid, src_slot: slot, dst_slot}
                            },
                            s => panic!("Encountered unexpected flag state {s:?} while handling notify on transfer flag"),
                        });
                    },
                }
            }
            Events::UCoreDmaCompleted(flag) => {
                self.flags_table[flag].unwrap_transfer_mut()
                    .transition(|old| match old {
                        TransferState::Transfering { dst_slot, .. } => TransferState::Transfered{ dst_slot },
                        _ => unreachable!(),
                    });
                dispatcher.dispatch_now(Events::UCoreProcessDOps);
            }
            Events::IscRetireDOp(ref dop) => {
                // A notification goes out when the inner `SYNC` standing in for its `NOTIFY` retires.
                if let Some((hid, flag, slot)) = self.notify_table.remove(&dop.id) {
                    dispatcher.dispatch_now(Events::UCoreNotify(hid, flag, slot));
                }
            }
            Events::IscStarved => {
                // A board with notifications still in flight is not done: the peers waiting on
                // them have not been signalled yet.
                if self.dops.is_empty() && self.notify_table.is_empty() {
                    dispatcher.dispatch_now(Events::UCoreStarved);
                }
            },
            _ => {}
        }
    }

    fn name(&self) -> String {
        "UCore".into()
    }
}
