use std::fmt::Display;
use zhc_ir::ValId;
use zhc_utils::{StoreIndex, fsm};

/// Represents the state of a register.
#[fsm]
#[derive(Clone, Copy, Debug)]
pub enum RegState {
    /// The register does not hold any value
    Empty,
    /// The register holds a value.
    Storing(ValId),
    /// The register holds an unspilled value.
    Unspilled(ValId),
    /// The register holds a newly added dst.
    Fresh(ValId),
    /// The register holds an src which is used for the last time (may be an unspilled).
    Retiring(ValId),
    /// The register holds both a sunsetting and a fresh value.
    Transitioning(ValId, ValId),
}

impl Display for RegState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegState::Empty => write!(f, "     "),
            RegState::Fresh(val_id) => write!(f, "\x1b[1m★{:4}\x1b[0m", val_id.as_usize()),
            RegState::Storing(val_id) => write!(f, " {:4}", val_id.as_usize()),
            RegState::Retiring(val_id) => {
                write!(f, "\x1b[1m◌{:4}\x1b[0m", val_id.as_usize())
            }
            RegState::Transitioning(_, val_id) => {
                write!(f, "\x1b[1m⟳{:4}\x1b[0m", val_id.as_usize())
            }
            RegState::Unspilled(val_id) => {
                write!(f, "\x1b[1m✈{:4}\x1b[0m", val_id.as_usize())
            }
            RegState::__INVALID => unreachable!(),
        }
    }
}

impl RegState {
    /// Whether the register can receive a spill.
    pub fn may_receive_unspill(&self) -> bool {
        matches!(self, RegState::Empty)
    }

    /// Whether the register can receive a dst.
    pub fn may_receive_dst(&self, is_batch: bool) -> bool {
        if is_batch {
            matches!(self, RegState::Empty)
        } else {
            matches!(self, RegState::Empty | RegState::Retiring(_))
        }
    }

    /// Whether the register is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, RegState::Empty)
    }

    /// Turn a transitive state to a stable one.
    ///
    /// Returns the value id if a value was retired.
    pub fn stabilize(&mut self) -> Option<ValId> {
        use RegState::*;
        self.transition_with(|transitive| match transitive {
            Empty => (Empty, None),
            Retiring(valid) => (Empty, Some(valid)),
            Storing(valid) | Fresh(valid) | Unspilled(valid) => (Storing(valid), None),
            Transitioning(old, valid) => (Storing(valid), Some(old)),
            _ => unreachable!("{:?}", transitive),
        })
    }

    /// Retire a register that is read for the last time.
    pub fn retire(&mut self) {
        use RegState::*;
        self.transition(|old| match old {
            Storing(valid) | Unspilled(valid) => Retiring(valid),
            _ => unreachable!(),
        });
    }

    /// Evicts a register.
    ///
    /// Returns the ValId stored inside.
    pub fn evict(&mut self) -> ValId {
        use RegState::*;
        self.transition_with(|old| match old {
            Storing(valid) => (Empty, valid),
            _ => unreachable!(),
        })
    }

    /// Acquire a register for unspill.
    pub fn acquire_unspill(&mut self, valid: ValId) {
        use RegState::*;
        self.transition(|old| match old {
            Empty => Unspilled(valid),
            _ => unreachable!(),
        });
    }

    /// Acquire a register for dst
    pub fn acquire_dst(&mut self, valid: ValId) {
        use RegState::*;
        self.transition(|old| match old {
            Empty => Fresh(valid),
            Retiring(old) => Transitioning(old, valid),
            _ => unreachable!(),
        });
    }
}

// Notes
// =====
//
// [1]: The register file structure encodes different types of information depending on the stage of the allocation
// process:
// - Stable states: The Empty/Storing states are the only possible states at the start of an
//   allocation iteration. They
// represent the physical register file's state during execution.
// - Transitive states: The other states only occur within an allocation iteration. They provide
//   additional information
// to help the allocator make optimal decisions.
// In this sense, the transitive states do not represent any meaningful informatino about a physical
// regfile, but rather support the allocator in its work.
