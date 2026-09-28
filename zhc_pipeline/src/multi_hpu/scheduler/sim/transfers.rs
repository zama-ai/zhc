use std::cmp::max;

use serde::Serialize;
use zhc_ir::OpId;
use zhc_langs::hpulang::{HpuId, N_TRANSFER_FLAGS, Slot, SlotGen, SlotId};
use zhc_utils::{Store, fsm, iter::MultiZip};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Knowledge(pub Store<HpuId, Store<SlotId, Option<SlotGen>>>);

impl Knowledge {
    pub fn new(n_hpus: usize) -> Self {
        Knowledge(Store::with_value(
            Store::with_value(None, N_TRANSFER_FLAGS as usize),
            n_hpus,
        ))
    }

    pub fn do_knows(&self, hpu: &HpuId, slot: &Slot) -> bool {
        self.0[hpu][slot.1].is_some_and(|known| known >= slot.0)
    }

    pub fn record(&mut self, hpu: &HpuId, slot: &Slot) {
        let known = &mut self.0[hpu][slot.1];
        *known = Some(known.map_or(slot.0, |k| max(k, slot.0)));
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

#[fsm]
#[derive(Clone)]
pub enum SlotState {
    Available { next_gen: SlotGen },

    Busy { on_gen: SlotGen },
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Transfer {
    pub firer: HpuId,
    pub lander: HpuId,
    pub opid: OpId,
    pub slot: Slot,
    pub knowledge: Knowledge,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TransferQuery {
    pub firer: HpuId,
    pub lander: HpuId,
    pub opid: OpId,
}

impl TransferQuery {
    pub fn make_transfer(self, slot: Slot, knowledge: Knowledge) -> Transfer {
        Transfer {
            firer: self.firer,
            lander: self.lander,
            opid: self.opid,
            slot,
            knowledge,
        }
    }
}
