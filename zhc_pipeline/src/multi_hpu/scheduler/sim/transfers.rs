use std::cmp::max;

use serde::Serialize;
use zhc_ir::OpId;
use zhc_langs::hpulang::{HpuId, N_TRANSFER_FLAGS};
use zhc_utils::{Store, StoreIndex, fsm, iter::MultiZip};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Knowledge(pub Store<HpuId, Store<SlotId, Option<SlotGen>>>);

impl Knowledge {

    pub fn new(n_hpus: usize) -> Self {
        Knowledge(Store::with_value(
            Store::with_value(None, N_TRANSFER_FLAGS as usize),
            n_hpus,
        ))
    }

    pub fn do_knows(&self, hpu: &HpuId, slot: &SlotId, generation: &SlotGen) -> bool {
        self.0[hpu][slot].is_some_and(|known| known >= *generation)
    }

    pub fn record(&mut self, hpu: &HpuId, slot: &SlotId, generation: &SlotGen) {
        let known = &mut self.0[hpu][slot];
        *known = Some(known.map_or(*generation, |k| max(k, *generation)));
    }

    pub fn merge_with(&mut self, other: &Knowledge) {
        for (mine, theirs) in (self.0.iter_mut(), other.0.iter()).mzip() {
            for (mine, theirs) in (mine.iter_mut(), theirs.iter()).mzip() {
                if let Some(theirs) = theirs {
                    *mine = Some(mine.map_or(*theirs, |m| max(m, *theirs)));
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, StoreIndex)]
pub struct SlotId(pub u8);

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotGen(pub u16);

impl SlotGen {
    pub fn next(&self) -> Self {
        SlotGen(self.0.strict_add(1))
    }

    pub fn is_first_gen(&self) -> bool {
        self.0 == 0
    }

    pub fn previous(&self) -> Self {
        SlotGen(self.0.strict_sub(1))
    }
}

#[fsm]
#[derive(Clone)]
pub enum SlotState {
    Available{next_gen: SlotGen},
    Busy{on_gen: SlotGen}
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Transfer {
    pub firer: HpuId,
    pub lander: HpuId,
    pub opid: OpId,
    pub sid: SlotId,
    pub sgen: SlotGen,
    pub knowledge: Knowledge
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TransferQuery {
    pub firer: HpuId,
    pub lander: HpuId,
    pub opid: OpId,
}

impl TransferQuery {
    pub fn make_transfer(self, sid: SlotId, sgen: SlotGen, knowledge: Knowledge) -> Transfer {
        Transfer { firer: self.firer, lander: self.lander, opid: self.opid, sid, sgen, knowledge }
    }
}
