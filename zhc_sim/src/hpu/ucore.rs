use std::collections::VecDeque;

use zhc_langs::{
    doplang::{CtMem, DopInstructionSet, UserFlag, VirtId},
    hpulang::{FIRST_FLAG, N_FLAGS},
};
use zhc_utils::{FastMap, fsm};

use crate::Dispatch;

use super::*;

/// Lifecycle of wait/transfer tracked by a [`UCore`].
#[fsm]
#[derive(Debug, Clone, Serialize)]
pub enum WaitState {
    Forbidden,
    None, // Unseen
    Received(HpuId, CtMem),          // Notify arrivé, de qui et a quelle adresse chez la source.
    ReadPending(CtMem),              // Notify pas recu, Mais LD_B2B a déja souscrit. Et l'adresse est locale.
    DmaPending(HpuId, CtMem, CtMem), // Received + ReadPending
    Resolved(CtMem),
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
    Waiting,
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
    waits_table: Vec<WaitState>,
    notify_table: FastMap<DOpId, (HpuId, UserFlag, CtMem)>,
    mhdma_latency: ConstantLatency,
}

impl UCore {
    /// Creates an idle micro-core whose inbound transfers each take
    /// `mhdma_latency` to load.
    pub fn new(mhdma_latency: ConstantLatency) -> Self {
        UCore {
            dops: VecDeque::new(),
            waits_table: std::iter::once(WaitState::Forbidden)
                .chain(std::iter::repeat(WaitState::None))
                .take(N_FLAGS as usize)
                .collect(),
            notify_table: FastMap::default(),
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
                                flag: UserFlag { flag },
                                slot
                            } = self.dops.pop_front().unwrap().raw
                            else {
                                unreachable!()
                            };
                            self.waits_table[flag as usize].transition(|old| match old {
                                WaitState::None | WaitState::Resolved(_) => {
                                    WaitState::ReadPending(slot)
                                }
                                WaitState::Received(hid, src) => {
                                    dispatcher.dispatch_after(
                                        self.mhdma_latency.compute_latency(),
                                        Events::UCoreDmaCompleted(UserFlag{flag})
                                    );
                                    WaitState::DmaPending(hid, src, slot)
                                }
                                w => panic!("1: Encountered unexpected wait state {w:?}"),
                            });
                        }
                        Some(WAIT { flag: UserFlag { flag }, slot }) => {
                            let should_resume = match self.waits_table[*flag as usize] {
                                WaitState::None => {
                                    // Sync
                                    false
                                }
                                WaitState::ReadPending(_) => {
                                    false
                                }
                                WaitState::Received(_, _) | WaitState::DmaPending(_, _, _) => {
                                    slot.is_none()
                                }
                                WaitState::Resolved(_) => {
                                    true
                                }
                                ref w => panic!("2: Encountered unexpected wait state {w:?}"),
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
                                unreachable!("5")
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
            Events::UCoreNotified(flag, hid, src) => {
                self.waits_table[flag.flag as usize]
                    .transition(|old| match old {
                        WaitState::None => {
                            // Fresh sync or transfer.
                            dispatcher.dispatch_now(Events::UCoreProcessDOps);
                            WaitState::Received(hid, src)
                        },
                        WaitState::Received(_, _) => {
                            // Recycling a past sync
                            assert!(flag.flag < FIRST_FLAG);
                            dispatcher.dispatch_now(Events::UCoreProcessDOps);
                            WaitState::Received(hid, src)
                        },
                        WaitState::Resolved(_) => {
                            // Recylcing a past transfer
                            assert!(FIRST_FLAG <= flag.flag );
                            dispatcher.dispatch_now(Events::UCoreProcessDOps);
                            WaitState::Received(hid, src)
                        },
                        WaitState::ReadPending(dst) => {
                            // Usual path for transfer
                            dispatcher.dispatch_after(
                                self.mhdma_latency.compute_latency(),
                                Events::UCoreDmaCompleted(flag)
                            );
                            WaitState::DmaPending(hid, src, dst)
                        },
                        _ => unreachable!()
                    });
            }
            Events::UCoreDmaCompleted(flag) => {
                self.waits_table[flag.flag as usize]
                    .transition(|old| match old {
                        WaitState::DmaPending(_, _, dst) => WaitState::Resolved(dst),
                        _ => unreachable!(),
                    });
                dispatcher.dispatch_now(Events::UCoreProcessDOps);
            }
            Events::IscRetireDOp(ref dop) => {
                // A notification goes out when the inner `SYNC` standing in for its `NOTIFY` retires.
                if let Some((hid, tid, slot)) = self.notify_table.remove(&dop.id) {
                    dispatcher.dispatch_now(Events::UCoreNotify(hid, tid, slot));
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
