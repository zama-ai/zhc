use super::*;
use serde::Serialize;
use std::fmt::Display;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::{AnnIR, OpId, OpMap};
use zhc_langs::hpulang::{
    HpuId, HpuLang, N_TRANSFER_FLAGS, Slot, SlotGen, SlotId, SyncNotify, SyncWait,
};
use zhc_sim::{Dispatch, Event, MapDispatch, Simulatable, Tracer, TracingLevel, Trigger};
use zhc_utils::{Store, units::Cycle};

use crate::{
    SchedPolicy,
    multi_hpu::scheduler::{HpuEvents, LightHpu, Stats},
};

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum MultiHpuEvents {
    Hpu(HpuId, HpuEvents),
    ScheduleComplete,
}

impl Display for MultiHpuEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiHpuEvents::Hpu(id, event) => write!(f, "Hpu({id}, {event})"),
            MultiHpuEvents::ScheduleComplete => write!(f, "ScheduleComplete"),
        }
    }
}

impl Event for MultiHpuEvents {}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TransferSpec {
    pub slot: Slot,
    pub notifies: Vec<SyncNotify>,
    pub waits: Vec<SyncWait>,
}

#[derive(Clone)]
pub struct LightMultiHpu<'a, 'b> {
    pub hpus: Store<HpuId, LightHpu<'a, 'b>>,
    pub slots: Store<HpuId, Store<SlotId, SlotState>>,
    pub transfers_map: OpMap<TransferSpec>,
    policy: SchedPolicy,
    pending: Vec<TransferQuery>,
    expected_lands: usize,
    last_landed_in: Store<HpuId, Option<OpId>>,
}

impl<'a, 'b> Serialize for LightMultiHpu<'a, 'b> {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        panic!()
    }
}

impl<'a, 'b> LightMultiHpu<'a, 'b> {
    pub fn new(
        ir: &'b AnnIR<'a, HpuLang, Stats, ()>,
        config: &MultiHpuConfig,
        policy: SchedPolicy,
    ) -> Self {
        let hpus = (0..config.n_hpus)
            .map(|i| LightHpu::new(ir, &config, policy, HpuId(i)))
            .collect();
        let slots = (0..config.n_hpus)
            .map(|_| {
                Store::with_value(
                    SlotState::Available {
                        next_gen: SlotGen(0),
                    },
                    N_TRANSFER_FLAGS as usize,
                )
            })
            .collect();
        let transfers_map = ir.empty_opmap();
        let pending = Vec::new();
        let expected_lands = ir
            .walk_ops_linear()
            .map(|op| {
                if op.get_instruction().is_transfer() {
                    2
                } else {
                    1
                }
            })
            .sum();
        let last_landed_in = Store::with_value(None, config.n_hpus as usize);
        LightMultiHpu {
            hpus,
            slots,
            transfers_map,
            pending,
            expected_lands,
            policy,
            last_landed_in,
        }
    }
}

impl<'a, 'b> Simulatable for LightMultiHpu<'a, 'b> {
    type Event = MultiHpuEvents;

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match &trigger.event {
            MultiHpuEvents::Hpu(dst_hid, HpuEvents::LandTransferIn(transfer)) => {
                let flags_on = match self.policy {
                    SchedPolicy::AsSoonAsPossible => transfer.lander,
                    SchedPolicy::AsLateAsPossible => transfer.firer,
                };
                self.slots[flags_on][transfer.slot.1].transition(|old| match old {
                    SlotState::Busy { on_gen } => SlotState::Available {
                        next_gen: on_gen.next(),
                    },
                    _ => unreachable!(),
                });
                self.hpus[dst_hid].handle(
                    &mut dispatcher.map(|e| MultiHpuEvents::Hpu(*dst_hid, e)),
                    Trigger {
                        at: trigger.at,
                        issue: trigger.issue,
                        event: HpuEvents::LandTransferIn(transfer.clone()),
                    },
                );
                self.last_landed_in[dst_hid] = Some(transfer.opid);
                self.process_pending_transfers(dispatcher);
            }
            MultiHpuEvents::Hpu(_, HpuEvents::TransferQuery(query)) => {
                self.pending.push(query.clone());
                self.process_pending_transfers(dispatcher);
            }
            MultiHpuEvents::Hpu(_, HpuEvents::TransferOut(transfer)) => {
                dispatcher.dispatch_now(MultiHpuEvents::Hpu(
                    transfer.lander,
                    HpuEvents::TransferIn(transfer.clone()),
                ));
            }
            MultiHpuEvents::Hpu(hpu_id, hpu_event) => {
                self.hpus[hpu_id].handle(
                    &mut dispatcher.map(|e| MultiHpuEvents::Hpu(*hpu_id, e)),
                    Trigger {
                        at: trigger.at,
                        issue: trigger.issue,
                        event: hpu_event.clone(),
                    },
                );
            }
            _ => {}
        }

        if let MultiHpuEvents::Hpu(_, e) = &trigger.event {
            if e.is_land() {
                let all_landed: usize = self.hpus.iter().map(|h| h.landed).sum();
                if all_landed == self.expected_lands {
                    dispatcher.dispatch_now(MultiHpuEvents::ScheduleComplete);
                }
            }
        }
    }

    fn power_up(&mut self, dispatcher: &mut impl Dispatch<Event = MultiHpuEvents>) {
        for (hid, hpu) in self.hpus.enumerate_iter_mut() {
            hpu.power_up(&mut dispatcher.map(|e| MultiHpuEvents::Hpu(hid, e)));
        }
    }

    fn report<'t>(&self, at: Cycle, tracer: &mut Tracer, tracing_level: TracingLevel) {
        for hpu in self.hpus.iter() {
            hpu.report(at, tracer, tracing_level);
        }
    }
}

impl<'a, 'b> LightMultiHpu<'a, 'b> {
    pub fn stall_report(&self) -> String {
        let landed: usize = self.hpus.iter().map(|h| h.landed).sum();
        let mut report = format!(
            "Multi-HPU schedule stalled: {landed}/{} ops landed, {} transfers held:\n",
            self.expected_lands,
            self.pending.len()
        );
        for query in self.pending.iter() {
            report.push_str(&format!(
                "  {} -> {}: {:?}: {}\n",
                query.firer,
                query.lander,
                query.opid,
                self.explain_hold(query)
            ));
        }
        report
    }

    fn describe_table(&self, firer: HpuId, flags_on: HpuId) -> String {
        let knowledge = &self.hpus[firer].knowledge;
        self.slots[flags_on]
            .enumerate_iter()
            .map(|(sid, state)| match state {
                SlotState::Available { next_gen } => {
                    let known = next_gen.is_first_gen()
                        || knowledge.do_knows(&flags_on, &Slot(next_gen.previous(), sid));
                    format!("A{}{}", next_gen.0, if known { "+" } else { "?" })
                }
                SlotState::Busy { on_gen } => format!("B{}", on_gen.0),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn explain_hold(&self, query: &TransferQuery) -> String {
        let flags_on = self.data_flags_on(query);
        let data = self.describe_table(query.firer, flags_on);
        let (notifier, waiter) = (query.lander, query.firer);
        let sync_flags_on = self.sync_flags_on(notifier, waiter);
        let sync = if self.last_landed_in[notifier].is_none() {
            "no anchor".to_string()
        } else {
            format!(
                "sync table on {sync_flags_on} [{}]",
                self.describe_table(notifier, sync_flags_on)
            )
        };
        format!("data table on {flags_on} [{data}], {sync}")
    }

    fn data_flags_on(&self, query: &TransferQuery) -> HpuId {
        match self.policy {
            SchedPolicy::AsSoonAsPossible => query.lander,
            SchedPolicy::AsLateAsPossible => query.firer,
        }
    }

    fn sync_flags_on(&self, notifier: HpuId, waiter: HpuId) -> HpuId {
        match self.policy {
            SchedPolicy::AsSoonAsPossible => waiter,
            SchedPolicy::AsLateAsPossible => notifier,
        }
    }

    pub fn process_pending_transfers(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = MultiHpuEvents>,
    ) {
        let pending = std::mem::take(&mut self.pending);
        for query in pending {
            if !self.try_grant_transfer(&query, dispatcher) {
                self.pending.push(query);
            }
        }
    }

    fn try_grant_transfer(
        &mut self,
        query: &TransferQuery,
        dispatcher: &mut impl Dispatch<Event = MultiHpuEvents>,
    ) -> bool {
        let flags_on = self.data_flags_on(query);
        let mut waits = vec![];
        let slot = match self.grant_slot(query.firer, flags_on) {
            Some(slot) => slot,
            None => {
                let notifiers = std::iter::once(query.lander).chain(
                    (0..self.hpus.len() as u8)
                        .map(HpuId)
                        .filter(|h| *h != query.lander && *h != query.firer),
                );
                let Some(wait) = notifiers
                    .into_iter()
                    .find_map(|notifier| self.sync(notifier, query.firer, flags_on))
                else {
                    return false;
                };
                waits.push(wait);
                self.grant_slot(query.firer, flags_on)
                    .expect("A sync must unlock a slot on the data table")
            }
        };
        let knowledge = self.hpus[query.firer].knowledge.clone();
        dispatcher.dispatch_now(MultiHpuEvents::Hpu(
            query.firer,
            HpuEvents::TransferGranted(query.clone().make_transfer(slot, knowledge)),
        ));
        let mut spec = TransferSpec {
            slot,
            notifies: vec![],
            waits: vec![],
        };
        match self.policy {
            SchedPolicy::AsSoonAsPossible => spec.waits = waits,
            SchedPolicy::AsLateAsPossible => {
                spec.notifies = waits
                    .into_iter()
                    .map(|w| SyncNotify {
                        dst: w.src,
                        slot: w.slot,
                    })
                    .collect()
            }
        }
        self.transfers_map.insert(query.opid, spec);
        true
    }

    fn grant_slot(&mut self, firer: HpuId, flags_on: HpuId) -> Option<Slot> {
        let knowledge = &self.hpus[firer].knowledge;
        for (sid, state) in self.slots[flags_on].enumerate_iter_mut() {
            let granted = state.transition_with(|old| match old {
                SlotState::Available { next_gen }
                    if next_gen.is_first_gen()
                        || knowledge.do_knows(&flags_on, &Slot(next_gen.previous(), sid)) =>
                {
                    (
                        SlotState::Busy { on_gen: next_gen },
                        Some(Slot(next_gen, sid)),
                    )
                }
                other => (other, None),
            });
            if granted.is_some() {
                return granted;
            }
        }
        None
    }

    fn sync(&mut self, notifier: HpuId, waiter: HpuId, data_flags_on: HpuId) -> Option<SyncWait> {
        let anchor = self.last_landed_in[notifier]?;
        let notifier_knowledge = &self.hpus[notifier].knowledge;
        let useful = self.slots[data_flags_on]
            .enumerate_iter()
            .any(|(sid, state)| {
                matches!(state, SlotState::Available { next_gen }
                if !next_gen.is_first_gen()
                    && notifier_knowledge.do_knows(&data_flags_on, &Slot(next_gen.previous(), sid)))
            });
        if !useful {
            return None;
        }
        let flags_on = self.sync_flags_on(notifier, waiter);
        let slot = self.grant_slot(notifier, flags_on)?;
        self.slots[flags_on][slot.1].transition(|old| match old {
            SlotState::Busy { on_gen } => SlotState::Available {
                next_gen: on_gen.next(),
            },
            _ => unreachable!(),
        });
        let spec = self.transfers_map.get_mut(anchor).unwrap();
        match self.policy {
            SchedPolicy::AsSoonAsPossible => {
                spec.notifies.push(SyncNotify { dst: waiter, slot });
            }
            SchedPolicy::AsLateAsPossible => {
                spec.waits.push(SyncWait { src: waiter, slot });
            }
        }
        let notifier_knowledge = self.hpus[notifier].knowledge.clone();
        let waiter_knowledge = &mut self.hpus[waiter].knowledge;
        waiter_knowledge.record(&flags_on, &slot);
        waiter_knowledge.merge_with(&notifier_knowledge);
        Some(SyncWait {
            src: notifier,
            slot,
        })
    }
}
