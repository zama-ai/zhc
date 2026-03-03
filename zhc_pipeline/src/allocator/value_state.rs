use zhc_utils::fsm;

use crate::allocator::{heap::HeapSlot, register_file::RegId};

#[fsm]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValState {
    Unseen,
    Registered { reg: RegId },
    Spilled { slot: HeapSlot },
    Retired,
}

impl ValState {
    pub fn rid(&self) -> RegId {
        let ValState::Registered { reg } = self else {
            unreachable!()
        };
        *reg
    }

    pub fn register(&mut self, reg: RegId) {
        use ValState::*;
        self.transition(|old| match old {
            Unseen => Registered { reg },
            _ => unreachable!(),
        });
    }

    pub fn is_spilled(&self) -> bool {
        matches!(self, ValState::Spilled { .. })
    }

    pub fn spill(&mut self, slot: HeapSlot) -> RegId {
        use ValState::*;
        self.transition_with(|old| match old {
            Registered { reg } => (Spilled { slot }, reg),
            _ => unreachable!(),
        })
    }

    pub fn unspill(&mut self, reg: RegId) -> HeapSlot {
        use ValState::*;
        self.transition_with(|old| match old {
            Spilled { slot } => (Registered { reg }, slot),
            _ => unreachable!(),
        })
    }

    pub fn retire(&mut self) {
        use ValState::*;
        self.transition(|old| match old {
            Registered { .. } => Retired,
            _ => unreachable!(),
        });
    }
}
