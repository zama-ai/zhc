use super::super::{EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage};
use super::*;
use hc_utils::iter::Separate;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct EmulatedCiphertext {
    pub(crate) storage: EmulatedCiphertextStorage,
    pub(crate) spec: CiphertextSpec,
}

impl EmulatedCiphertext {
    pub fn len(&self) -> u8 {
        self.spec.block_count()
    }

    pub fn get_block(&self, ith: u8) -> EmulatedCiphertextBlock {
        assert!(ith < self.len(), "Tried to get nonexistent block.");
        let storage = (self.storage >> (ith * self.spec.block_spec().message_size()))
            as EmulatedCiphertextBlockStorage
            & self.spec.block_spec().message_mask();
        EmulatedCiphertextBlock {
            storage,
            spec: self.spec.block_spec(),
        }
    }

    pub fn set_block(&mut self, ith: u8, block: EmulatedCiphertextBlock) {
        assert!(ith < self.len(), "Tried to set nonexistent block.");
        assert!(block.is_message_only(), "Tried to set a dirty block.");
        let clearing = self.storage & self.spec.block_mask(ith);
        self.storage -= clearing;
        self.storage += (block.storage as EmulatedCiphertextStorage)
            << (ith * self.spec.block_spec().message_size());
    }

    pub(crate) fn raw_mask_int(&self) -> EmulatedCiphertextStorage {
        self.storage & self.spec.int_mask()
    }

    pub(crate) fn raw_int_bits(&self) -> EmulatedCiphertextStorage {
        self.raw_mask_int()
    }

    pub fn spec(&self) -> CiphertextSpec {
        self.spec
    }

    pub fn as_storage(&self) -> EmulatedCiphertextStorage {
        self.storage
    }
}

impl Debug for EmulatedCiphertext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alternate = f.alternate();
        (0..self.len())
            .rev()
            .map(|i| self.get_block(i))
            .map(|block| {
                if alternate {
                    format!(
                        "{:0width$b}",
                        block.storage,
                        width = self.spec.block_spec().message_size() as usize
                    )
                } else {
                    format!("{}", block.storage,)
                }
            })
            .separate_with(|| format!("_"))
            .for_each(|string| write!(f, "{}", string).unwrap());
        write!(f, "_cint")
    }
}

impl PartialEq for EmulatedCiphertext {
    fn eq(&self, other: &Self) -> bool {
        self.raw_int_bits() == other.raw_int_bits() && self.spec == other.spec
    }
}

impl Eq for EmulatedCiphertext {}

impl Hash for EmulatedCiphertext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw_int_bits().hash(state);
        self.spec.hash(state);
    }
}

impl PartialOrd for EmulatedCiphertext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec == other.spec {
            self.raw_int_bits().partial_cmp(&other.raw_int_bits())
        } else {
            None
        }
    }
}
