use super::{EmulatedPlaintextBlockStorage, PlaintextBlockSpec};
use std::fmt::Debug;

/// A plaintext block containing only message data.
#[derive(Clone, Copy)]
pub struct EmulatedPlaintextBlock {
    pub(crate) storage: EmulatedPlaintextBlockStorage,
    pub(crate) spec: PlaintextBlockSpec,
}

impl EmulatedPlaintextBlock {
    /// Returns the raw message bits.
    pub(crate) fn raw_message_bits(&self) -> EmulatedPlaintextBlockStorage {
        self.raw_mask_message()
    }

    pub(crate) fn raw_mask_message(&self) -> EmulatedPlaintextBlockStorage {
        self.storage & self.spec.message_mask()
    }

    pub fn spec(&self) -> PlaintextBlockSpec {
        self.spec
    }
}

impl Debug for EmulatedPlaintextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{:0message_size$b}_pblk",
                self.raw_message_bits(),
                message_size = self.spec.message_size() as usize
            )
        } else {
            write!(f, "{}_pblk", self.raw_message_bits())
        }
    }
}

impl PartialEq for EmulatedPlaintextBlock {
    fn eq(&self, other: &Self) -> bool {
        self.raw_message_bits() == other.raw_message_bits() && self.spec == other.spec
    }
}

impl Eq for EmulatedPlaintextBlock {}

impl PartialOrd for EmulatedPlaintextBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if !self.spec.eq(&other.spec) {
            return None;
        }
        self.raw_message_bits()
            .partial_cmp(&other.raw_message_bits())
    }
}

impl std::hash::Hash for EmulatedPlaintextBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_message_bits().hash(state);
        self.spec.hash(state);
    }
}
