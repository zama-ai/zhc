use super::super::{EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage, SpecInteger};
use super::EmulatedCiphertextIntegerStorage;
use std::fmt::Debug;
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
/// Each block keeps all its bits: message, carry and padding. A block with non-zero carry or
/// padding bits is *dirty*. A dirty block can be written into a ciphertext, but the ciphertext
/// then has no integer value: [`as_storage`](Self::as_storage) panics until the carries are
/// propagated and the block is clean again.
///
/// Two ciphertexts are equal if they have the same spec and the same blocks, carry and padding
/// bits included.
///
/// # Debug formatting
///
/// - Default format: `{block_n}_.._{block_0}_ctint` (decimal block values, MSB first)
/// - Alternate format (`{:#?}`): binary representation, padded to the message size
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EmulatedCiphertextInteger {
    pub(crate) blocks: Vec<EmulatedCiphertextBlockStorage>,
    pub(crate) spec: SpecInteger,
}

impl EmulatedCiphertextInteger {
    pub(crate) fn from_storage(spec: SpecInteger, int: EmulatedCiphertextIntegerStorage) -> Self {
        let block_spec = spec.block_spec();
        let blocks = (0..spec.block_count().sas::<u32>())
            .map(|i| {
                let shift = i * block_spec.message_size().sas::<u32>();
                (int >> shift) as EmulatedCiphertextBlockStorage & block_spec.message_mask()
            })
            .collect();
        Self { blocks, spec }
    }

    /// Returns the number of blocks in this ciphertext.
    pub fn len(&self) -> u8 {
        self.spec.block_count()
    }

    /// Returns the block at the given index.
    ///
    /// Block 0 is the least significant block. The returned block is the block as stored, carry
    /// and padding bits included.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= len()`.
    pub fn get_block(&self, ith: u8) -> EmulatedCiphertextBlock {
        assert!(ith < self.len(), "Tried to get nonexistent block.");
        EmulatedCiphertextBlock {
            storage: self.blocks[ith.sas::<usize>()],
            spec: self.spec.block_spec(),
        }
    }

    /// Replaces the block at the given index.
    ///
    /// Block 0 is the least significant block. The block is stored as is, carry and padding bits
    /// included. A dirty block makes [`as_storage`](Self::as_storage) panic until it is replaced
    /// by a clean block.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `ith >= len()`
    /// - the spec of `block` is not the block spec of this ciphertext
    pub fn set_block(&mut self, ith: u8, block: EmulatedCiphertextBlock) {
        assert!(ith < self.len(), "Tried to set nonexistent block.");
        assert_eq!(block.spec, self.spec.block_spec(), "Spec mismatch.");
        self.blocks[ith.sas::<usize>()] = block.storage;
    }

    /// Returns the specification describing this ciphertext's layout.
    pub fn spec(&self) -> SpecInteger {
        self.spec
    }

    /// Returns the integer value reconstructed from the message bits of all blocks.
    ///
    /// # Panics
    ///
    /// Panics if a block is dirty, that is if it has non-zero carry or padding bits.
    pub fn as_storage(&self) -> EmulatedCiphertextIntegerStorage {
        let block_spec = self.spec.block_spec();
        self.blocks.iter().enumerate().fold(0, |acc, (i, block)| {
            assert!(
                block & !block_spec.message_mask() == 0,
                "Tried to get the value of an integer, but block {i} is dirty."
            );
            let shift = i.sas::<u32>() * block_spec.message_size().sas::<u32>();
            acc | ((*block).sas::<EmulatedCiphertextIntegerStorage>() << shift)
        })
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

impl Dumpable for EmulatedCiphertextInteger {
    fn dump_to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
