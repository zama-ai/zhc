use super::super::{EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage, SpecInteger};
use super::EmulatedCiphertextIntegerStorage;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use zhc_utils::iter::Separate;
use zhc_utils::{Dumpable, SafeAs};

/// An emulated TFHE radix ciphertext representing a large integer.
///
/// This structure models a multi-block ciphertext where a large integer is decomposed into
/// multiple [`EmulatedCiphertextBlock`] values using a fixed radix. Each block holds a portion
/// of the integer's message bits according to a shared [`SpecInteger`].
///
/// Ciphertexts are created via [`SpecInteger::from_int`] or [`SpecInteger::random`]. Individual
/// blocks can be accessed with [`get_block`](Self::get_block) and modified with
/// [`set_block`](Self::set_block). Block 0 is the least significant.
///
/// Only *clean* blocks (message-only, with zero carry and padding) can be written back into a
/// ciphertext. This enforces the invariant that carries must be explicitly propagated before
/// reassembling the integer.
///
/// Two ciphertexts can be compared for equality or ordering only if they share the same spec.
///
/// # Debug formatting
///
/// - Default format: `{block_n}_.._{block_0}_ctint` (decimal block values, MSB first)
/// - Alternate format (`{:#?}`): binary representation with proper bit widths
#[derive(Clone, Copy)]
pub struct EmulatedCiphertextInteger {
    pub(crate) storage: EmulatedCiphertextIntegerStorage,
    pub(crate) spec: SpecInteger,
}

impl EmulatedCiphertextInteger {
    /// Returns the number of blocks in this ciphertext.
    pub fn len(&self) -> u8 {
        self.spec.block_count()
    }

    /// Returns the block at the given index.
    ///
    /// Block 0 is the least significant block. The returned block contains only message bits;
    /// carry and padding are zero.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= len()`.
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

    /// Replaces the block at the given index.
    ///
    /// Block 0 is the least significant block. The provided block must be *clean* — that is,
    /// it must have zero carry and padding bits. Use [`EmulatedCiphertextBlock::is_message_only`]
    /// to check, or [`EmulatedCiphertextBlock::mask_message`] to extract just the message.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `ith >= len()`
    /// - `block` has non-zero carry or padding bits
    pub fn set_block(&mut self, ith: u8, block: EmulatedCiphertextBlock) {
        assert!(ith < self.len(), "Tried to set nonexistent block.");
        assert!(block.is_message_only(), "Tried to set a dirty block.");
        let clearing = self.storage & self.spec.block_mask(ith);
        self.storage -= clearing;
        self.storage += (block.storage.sas::<EmulatedCiphertextIntegerStorage>())
            << (ith * self.spec.block_spec().message_size());
    }

    /// Returns the specification describing this ciphertext's layout.
    pub fn spec(&self) -> SpecInteger {
        self.spec
    }

    /// Returns the integer value reconstructed from all blocks' message bits.
    pub fn as_storage(&self) -> EmulatedCiphertextIntegerStorage {
        self.storage & self.spec.int_mask()
    }
}

impl Debug for EmulatedCiphertextInteger {
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
        write!(f, "_ctint")
    }
}

impl PartialEq for EmulatedCiphertextInteger {
    fn eq(&self, other: &Self) -> bool {
        self.as_storage() == other.as_storage() && self.spec == other.spec
    }
}

impl Eq for EmulatedCiphertextInteger {}

impl Hash for EmulatedCiphertextInteger {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_storage().hash(state);
        self.spec.hash(state);
    }
}

impl PartialOrd for EmulatedCiphertextInteger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec == other.spec {
            self.as_storage().partial_cmp(&other.as_storage())
        } else {
            None
        }
    }
}

impl Dumpable for EmulatedCiphertextInteger {
    fn dump_to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
