use super::*;
use std::fmt::Display;
use serde::Serialize;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::{AnnIR, OpMap};
use zhc_langs::hpulang::{HpuId, HpuLang, N_RESERVED_FLAGS, N_TRANSFER_FLAGS, TransferId};
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
            MultiHpuEvents::ScheduleComplete => write!(f, "ScheduleComplete")
        }
    }
}

impl Event for MultiHpuEvents {}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TransferSpec {
    pub tid: TransferId,
    pub notifies: Vec<HpuId>,
    pub waits: bool,
}

#[derive(Clone)]
pub struct LightMultiHpu<'a, 'b> {
    pub hpus: Store<HpuId, LightHpu<'a, 'b>>,
    pub slots: Store<HpuId, Store<SlotId, SlotState>>,
    pub transfers_map: OpMap<TransferSpec>,
    pending: Vec<TransferQuery>,
    expected_lands: usize
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
            .map(|_| Store::with_value(SlotState::Available { next_gen: SlotGen(0) }, N_TRANSFER_FLAGS as usize))
            .collect();
        let transfers_map = ir.empty_opmap();
        let pending = Vec::new();
        let expected_lands = ir.walk_ops_linear().map(|op| if op.get_instruction().is_transfer() {2} else {1}).sum();
        LightMultiHpu {
            hpus,
            slots,
            transfers_map,
            pending,
            expected_lands
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
                self.slots[dst_hid][transfer.sid].transition(|old| match old {
                    SlotState::Busy { on_gen } => SlotState::Available { next_gen: on_gen.next() },
                    _ => unreachable!()
                });
                self.hpus[dst_hid].handle(
                    &mut dispatcher.map(|e| MultiHpuEvents::Hpu(*dst_hid, e)),
                    Trigger {
                        at: trigger.at,
                        issue: trigger.issue,
                        event: HpuEvents::LandTransferIn(transfer.clone()),
                    },
                );
                self.process_pending_transfers(dispatcher);
            }
            MultiHpuEvents::Hpu(_, HpuEvents::TransferQuery(query)) => {
                self.pending.push(query.clone());
                self.process_pending_transfers(dispatcher);
            }
            MultiHpuEvents::Hpu(_, HpuEvents::TransferOut(transfer)) => {
                dispatcher.dispatch_now(MultiHpuEvents::Hpu(transfer.dst, HpuEvents::TransferIn(transfer.clone())));
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
                "  {} -> {}: {:?}\n",
                query.src, query.dst, query.opid
            ));
        }
        report
    }

    pub fn process_pending_transfers(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = MultiHpuEvents>,
    ) {
        self.pending.retain(|query| {
            let src_k = &self.hpus[query.src].knowledge;
            for (sid, slot) in self.slots[query.dst].enumerate_iter_mut() {
                let do_break = slot.transition_with(|old| match old {
                    SlotState::Available { next_gen } => {
                        if next_gen.is_first_gen() || src_k.do_knows(&query.dst, &sid, &next_gen.previous()) {
                            dispatcher.dispatch_now(MultiHpuEvents::Hpu(query.src, HpuEvents::TransferGranted(query.clone(), sid, next_gen)));
                            self.transfers_map.insert(query.opid, TransferSpec { tid: TransferId(next_gen.0, sid.0+N_RESERVED_FLAGS), notifies: vec![], waits: false});
                            (SlotState::Busy { on_gen: next_gen }, true)
                        } else {
                            (SlotState::Available { next_gen }, false)
                        }
                    },
                    SlotState::Busy { on_gen } => (SlotState::Busy { on_gen }, false),
                    _ => unreachable!(),
                });
                if do_break {
                    return false
                }
            }
            true
        });
    }
}
