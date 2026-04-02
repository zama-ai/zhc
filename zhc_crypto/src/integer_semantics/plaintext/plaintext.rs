use super::*;
use std::fmt::Debug;
use zhc_utils::{Dumpable, SafeAs, iter::Separate};

use super::super::{EmulatedPlaintextBlock, EmulatedPlaintextBlockStorage};

/// An emulated plaintext integer for use in ciphertext-plaintext operations.
///
/// This structure models a multi-block plaintext where a large integer is decomposed into
/// multiple [`EmulatedPlaintextBlock`] values using a fixed radix. Each block holds a portion
/// of the integer's bits according to a shared [`PlaintextSpec`].
///
/// Plaintexts are created via [`PlaintextSpec::from_int`] or [`PlaintextSpec::random`].
/// Individual blocks can be accessed with [`get_block`](Self::get_block). Block 0 is the least
/// significant.
///
/// Plaintext integers are used as scalar operands in mixed operations with ciphertexts. A
/// plaintext is compatible with a ciphertext when their specs have matching integer and block
/// message sizes.
///
/// Two plaintexts can be compared for equality or ordering only if they share the same spec.
///
/// # Debug formatting
///
/// - Default format: `{block_n}_.._{block_0}_pint` (decimal block values, MSB first)
/// - Alternate format (`{:#?}`): binary representation with proper bit widths
#[derive(Clone, Copy)]
pub struct EmulatedPlaintext {
    pub(crate) storage: EmulatedPlaintextStorage,
    pub(crate) spec: PlaintextSpec,
}

impl EmulatedPlaintext {
    /// Returns the number of blocks in this plaintext.
    pub fn len(&self) -> u8 {
        self.spec.block_count()
    }

    /// Returns the block at the given index.
    ///
    /// Block 0 is the least significant block.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= len()`.
    pub fn get_block(&self, ith: u8) -> EmulatedPlaintextBlock {
        assert!(ith < self.len(), "Tried to get nonexistent block.");
        let storage = (self.storage >> (ith * self.spec.block_spec().message_size()))
            .sas::<EmulatedPlaintextBlockStorage>()
            & self.spec.block_spec().message_mask();
        EmulatedPlaintextBlock {
            storage,
            spec: self.spec.block_spec(),
        }
    }

    pub(crate) fn raw_mask_int(&self) -> EmulatedPlaintextStorage {
        self.storage & self.spec.int_mask()
    }

    pub(crate) fn raw_int_bits(&self) -> EmulatedPlaintextStorage {
        self.raw_mask_int()
    }

    /// Returns the specification describing this plaintext's layout.
    pub fn spec(&self) -> PlaintextSpec {
        self.spec
    }

    pub fn as_storage(&self) -> EmulatedPlaintextStorage {
        self.storage
    }
}

impl Debug for EmulatedPlaintext {
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
                        width = self.spec.block_spec().message_size().sas::<usize>()
                    )
                } else {
                    format!("{}", block.storage,)
                }
            })
            .separate_with(|| format!("_"))
            .for_each(|string| write!(f, "{}", string).unwrap());
        write!(f, "_pt")
    }
}

impl PartialEq for EmulatedPlaintext {
    fn eq(&self, other: &Self) -> bool {
        self.raw_int_bits() == other.raw_int_bits() && self.spec == other.spec
    }
}

impl Eq for EmulatedPlaintext {}

impl PartialOrd for EmulatedPlaintext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec != other.spec {
            None
        } else {
            self.raw_int_bits().partial_cmp(&other.raw_int_bits())
        }
    }
}

impl std::hash::Hash for EmulatedPlaintext {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_int_bits().hash(state);
        self.spec.hash(state);
    }
}

impl Dumpable for EmulatedPlaintext {
    fn dump_to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
