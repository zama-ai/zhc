use super::{CiphertextBlockSpec, EmulatedCiphertextBlockStorage};
use std::fmt::Debug;

/// A semantic equivalent to a ciphertext block.
#[derive(Clone, Copy)]
pub struct EmulatedCiphertextBlock {
    pub(crate) storage: EmulatedCiphertextBlockStorage,
    pub(crate) spec: CiphertextBlockSpec,
}

impl EmulatedCiphertextBlock {
    pub(crate) fn raw_mask_message(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.message_mask()
    }

    pub(crate) fn raw_mask_carry(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.carry_mask()
    }

    pub(crate) fn raw_mask_padding(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.padding_mask()
    }

    pub(crate) fn raw_mask_data(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.data_mask()
    }

    pub(crate) fn raw_mask_complete(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.complete_mask()
    }

    pub(crate) fn raw_message_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_message()
    }

    pub(crate) fn raw_carry_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_carry() >> self.spec.message_size()
    }

    pub(crate) fn raw_padding_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_padding() >> self.spec.data_size()
    }

    #[allow(unused)]
    pub(crate) fn raw_data_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_data()
    }

    pub(crate) fn raw_complete_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_complete()
    }

    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }

    pub fn mask_message(&self) -> EmulatedCiphertextBlock {
        EmulatedCiphertextBlock {
            storage: self.raw_mask_message(),
            spec: self.spec,
        }
    }

    pub fn mask_carry(&self) -> EmulatedCiphertextBlock {
        EmulatedCiphertextBlock {
            storage: self.raw_mask_carry(),
            spec: self.spec,
        }
    }

    pub fn move_carry_to_message(&self) -> EmulatedCiphertextBlock {
        EmulatedCiphertextBlock {
            storage: self.raw_mask_carry() >> self.spec.message_size(),
            spec: self.spec,
        }
    }

    pub fn is_message_only(&self) -> bool {
        (self.raw_complete_bits() >> self.spec.message_size()) == 0
    }
}

impl Debug for EmulatedCiphertextBlock {
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

impl PartialEq for EmulatedCiphertextBlock {
    fn eq(&self, other: &Self) -> bool {
        self.raw_complete_bits() == other.raw_complete_bits() && self.spec == other.spec
    }
}

impl Eq for EmulatedCiphertextBlock {}

impl PartialOrd for EmulatedCiphertextBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec == other.spec {
            self.raw_complete_bits()
                .partial_cmp(&other.raw_complete_bits())
        } else {
            None
        }
    }
}

impl std::hash::Hash for EmulatedCiphertextBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_complete_bits().hash(state);
        self.spec.hash(state);
    }
}
