use super::{CiphertextBlockSpec, CiphertextBlockStorage};
use std::fmt::Display;

/// A semantic equivalent to a ciphertext block.
#[derive(Clone, Copy, Debug)]
pub struct CiphertextBlock {
    pub(crate) storage: CiphertextBlockStorage,
    pub(crate) spec: CiphertextBlockSpec,
}

impl CiphertextBlock {
    pub(crate) fn raw_mask_message(&self) -> CiphertextBlockStorage {
        self.storage & self.spec.message_mask()
    }

    pub(crate) fn raw_mask_carry(&self) -> CiphertextBlockStorage {
        self.storage & self.spec.carry_mask()
    }

    pub(crate) fn raw_mask_padding(&self) -> CiphertextBlockStorage {
        self.storage & self.spec.padding_mask()
    }

    pub(crate) fn raw_mask_data(&self) -> CiphertextBlockStorage {
        self.storage & self.spec.data_mask()
    }

    pub(crate) fn raw_mask_complete(&self) -> CiphertextBlockStorage {
        self.storage & self.spec.complete_mask()
    }

    pub(crate) fn raw_message_bits(&self) -> CiphertextBlockStorage {
        self.raw_mask_message()
    }

    pub(crate) fn raw_carry_bits(&self) -> CiphertextBlockStorage {
        self.raw_mask_carry() >> self.spec.message_size()
    }

    pub(crate) fn raw_padding_bits(&self) -> CiphertextBlockStorage {
        self.raw_mask_padding() >> self.spec.data_size()
    }

    #[allow(unused)]
    pub(crate) fn raw_data_bits(&self) -> CiphertextBlockStorage {
        self.raw_mask_data()
    }

    pub(crate) fn raw_complete_bits(&self) -> CiphertextBlockStorage {
        self.raw_mask_complete()
    }

    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }

    pub fn mask_message(&self) -> CiphertextBlock {
        CiphertextBlock {
            storage: self.raw_mask_message(),
            spec: self.spec,
        }
    }

    pub fn mask_carry(&self) -> CiphertextBlock {
        CiphertextBlock {
            storage: self.raw_mask_carry(),
            spec: self.spec,
        }
    }

    pub fn move_carry_to_message(&self) -> CiphertextBlock {
        CiphertextBlock {
            storage: self.raw_mask_carry() >> self.spec.message_size(),
            spec: self.spec,
        }
    }

    pub fn is_message_only(&self) -> bool {
        (self.raw_complete_bits() >> self.spec.message_size()) == 0
    }
}

impl Display for CiphertextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{:0padding_size$b}_{:0carry_size$b}_{:0message_size$b}_cblk",
                self.raw_padding_bits(),
                self.raw_carry_bits(),
                self.raw_message_bits(),
                padding_size = self.spec.padding_size() as usize,
                carry_size = self.spec.carry_size() as usize,
                message_size = self.spec.message_size() as usize
            )
        } else {
            write!(
                f,
                "{}_{}_{}_cblk",
                self.raw_padding_bits(),
                self.raw_carry_bits(),
                self.raw_message_bits()
            )
        }
    }
}

impl PartialEq for CiphertextBlock {
    fn eq(&self, other: &Self) -> bool {
        self.raw_complete_bits() == other.raw_complete_bits() && self.spec == other.spec
    }
}

impl Eq for CiphertextBlock {}

impl PartialOrd for CiphertextBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec == other.spec {
            self.raw_complete_bits()
                .partial_cmp(&other.raw_complete_bits())
        } else {
            None
        }
    }
}

impl std::hash::Hash for CiphertextBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_complete_bits().hash(state);
        self.spec.hash(state);
    }
}
