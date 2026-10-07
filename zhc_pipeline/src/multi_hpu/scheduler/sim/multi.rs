use std::{cmp::max, collections::VecDeque, fmt::Display};

use serde::Serialize;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::{AnnIR, OpId, OpMap};
use zhc_langs::hpulang::{HpuId, HpuLang, TransferId};
use zhc_sim::{Dispatch, Event, MapDispatch, Simulatable, Tracer, TracingLevel, Trigger};
use zhc_utils::{FastMap, Store, iter::MultiZip, units::Cycle};

use crate::{
    SchedPolicy,
    multi_hpu::scheduler::{HpuEvents, LightHpu, Stats},
};

#[derive(Debug, Clone, Serialize)]
struct Knowledge(Vec<Option<TransferId>>, HpuId);

impl Knowledge {
    pub fn new(n_hpus: usize, hid: HpuId) -> Self {
        Knowledge(vec![None; n_hpus], hid)
    }

    pub fn can_transfer_freely<'s, 'd>(src: &'s Knowledge, dst: &'d Knowledge) -> bool {
        let next_dst_tid = dst.get_self().unwrap();
        if next_dst_tid.is_first_gen() {
            return true;
        }
        let Some(last_src_seen) = src.get_dst(dst.1) else {
            return false;
        };
        *last_src_seen >= next_dst_tid.of_previous_gen()
    }

    pub fn get_dst(&self, dst: HpuId) -> &Option<TransferId> {
        assert_ne!(self.1, dst);
        &self.0[dst.0 as usize]
    }

    pub fn get_self(&self) -> &Option<TransferId> {
        &self.0[self.1.0 as usize]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<TransferId>> {
        self.0.iter()
    }

    pub fn inc(&mut self) {
        match self.get_self_mut() {
            Some(t) => *t = t.inc().unwrap(),
            slot @ None => *slot = Some(TransferId::FIRST),
        }
    }

    pub fn get_self_mut(&mut self) -> &mut Option<TransferId> {
        &mut self.0[self.1.0 as usize]
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<TransferId>> {
        self.0.iter_mut()
    }

    pub fn merge_with<'b>(&mut self, other: &'b Knowledge) {
        for (src_k, dst_k) in (other.iter(), self.iter_mut()).mzip() {
            match (src_k, dst_k) {
                (Some(src_k), Some(dst_k)) => {
                    *dst_k = max(*src_k, *dst_k);
                }
                (Some(src_k), slot @ None) => {
                    *slot = Some(*src_k);
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum MultiHpuEvents {
    Hpu(HpuId, HpuEvents),
}

impl Display for MultiHpuEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiHpuEvents::Hpu(id, event) => write!(f, "Hpu({id}, {event})"),
        }
    }
}

impl Event for MultiHpuEvents {}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Transfer {
    pub tid: TransferId,
    pub notifies: Vec<HpuId>,
    pub waits: bool,
}

#[derive(Serialize)]
pub struct LightMultiHpu<'a, 'b> {
    pub hpus: Store<HpuId, LightHpu<'a, 'b>>,
    pub transfers_map: OpMap<Transfer>,
    inverse_transfers_map: FastMap<(HpuId, TransferId), OpId>,
    current_knowledge: Store<HpuId, Knowledge>,
    pending_knowledge: Store<HpuId, VecDeque<(HpuId, Knowledge)>>,
    policy: SchedPolicy,
}

impl<'a, 'b> LightMultiHpu<'a, 'b> {
    pub fn new(
        ir: &'b AnnIR<'a, HpuLang, Stats, ()>,
        config: &MultiHpuConfig,
        policy: SchedPolicy,
    ) -> Self {
        let hpus = (0..config.n_hpus)
            .map(|i| LightHpu::new(ir, &config.hpu_config, policy, HpuId(i)))
            .collect();
        let transfers_map = ir.empty_opmap();
        let current_knowledge = (0..config.n_hpus)
            .map(|i| Knowledge::new(config.n_hpus as usize, HpuId(i)))
            .collect();
        let pending_knowledge = Store::with_value(VecDeque::new(), config.n_hpus as usize);
        let inverse_transfers_map = FastMap::default();
        LightMultiHpu {
            hpus,
            transfers_map,
            inverse_transfers_map,
            current_knowledge,
            pending_knowledge,
            policy,
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
        match trigger.event {
            MultiHpuEvents::Hpu(dst_hid, HpuEvents::LandTransferIn(opid)) => {
                self.current_knowledge[dst_hid].inc();
                let (src_hid, src_k) = self.pending_knowledge[dst_hid].pop_back().unwrap();
                let dst_k = &self.current_knowledge[dst_hid];
                let tid = dst_k.get_self().unwrap();

                self.transfers_map.insert(
                    opid,
                    Transfer {
                        tid,
                        notifies: Vec::new(),
                        waits: false,
                    },
                );
                self.inverse_transfers_map.insert((dst_hid, tid), opid);

                if !Knowledge::can_transfer_freely(&src_k, &dst_k) {
                    self.transfers_map.get_mut(opid).unwrap().waits = true;
                    let ancestor_tid = tid.of_previous_gen();
                    let ancestor_opid = self
                        .inverse_transfers_map
                        .get(&(dst_hid, ancestor_tid))
                        .unwrap();
                    self.transfers_map
                        .get_mut(ancestor_opid)
                        .unwrap()
                        .notifies
                        .push(src_hid);
                }
                self.current_knowledge[dst_hid].merge_with(&src_k);

                self.hpus[dst_hid].handle(
                    &mut dispatcher.map(|e| MultiHpuEvents::Hpu(dst_hid, e)),
                    Trigger {
                        at: trigger.at,
                        issue: trigger.issue,
                        event: HpuEvents::LandTransferIn(opid),
                    },
                );
            }
            MultiHpuEvents::Hpu(src_hid, HpuEvents::TransferOut(dst_hid, opid)) => {
                self.pending_knowledge[dst_hid]
                    .push_front((src_hid, self.current_knowledge[src_hid].clone()));
                dispatcher.dispatch_now(MultiHpuEvents::Hpu(dst_hid, HpuEvents::TransferIn(opid)));
            }
            MultiHpuEvents::Hpu(hpu_id, hpu_event) => {
                self.hpus[hpu_id].handle(
                    &mut dispatcher.map(|e| MultiHpuEvents::Hpu(hpu_id, e)),
                    Trigger {
                        at: trigger.at,
                        issue: trigger.issue,
                        event: hpu_event,
                    },
                );
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
