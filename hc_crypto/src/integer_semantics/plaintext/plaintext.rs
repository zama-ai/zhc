use super::*;
use hc_utils::iter::Separate;
use std::fmt::Debug;

use super::super::{PlaintextBlock, PlaintextBlockStorage};

#[derive(Clone, Copy)]
pub struct Plaintext {
    pub(crate) storage: PlaintextStorage,
    pub(crate) spec: PlaintextSpec,
}

impl Plaintext {
    pub fn len(&self) -> u8 {
        self.spec.block_count()
    }

    pub fn get_block(&self, ith: u8) -> PlaintextBlock {
        assert!(ith < self.len(), "Tried to get nonexistent block.");
        let storage = (self.storage >> (ith * self.spec.block_spec().message_size()))
            as PlaintextBlockStorage
            & self.spec.block_spec().message_mask();
        PlaintextBlock {
            storage,
            spec: self.spec.block_spec(),
        }
    }

    pub(crate) fn raw_mask_int(&self) -> PlaintextStorage {
        self.storage & self.spec.int_mask()
    }

    pub(crate) fn raw_int_bits(&self) -> PlaintextStorage {
        self.raw_mask_int()
    }

    pub fn spec(&self) -> PlaintextSpec {
        self.spec
    }
}

impl Debug for Plaintext {
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
        write!(f, "_pint")
    }
}

impl PartialEq for Plaintext {
    fn eq(&self, other: &Self) -> bool {
        self.raw_int_bits() == other.raw_int_bits() && self.spec == other.spec
    }
}

impl Eq for Plaintext {}

impl PartialOrd for Plaintext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec != other.spec {
            None
        } else {
            self.raw_int_bits().partial_cmp(&other.raw_int_bits())
        }
    }
}

impl std::hash::Hash for Plaintext {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_int_bits().hash(state);
        self.spec.hash(state);
    }
}
