use std::fmt::Debug;

use zhc_utils::SafeAs;

use crate::integer_semantics::{CiphertextBlockRange, CiphertextSpec};

/// The set of values each block of a radix ciphertext can take.
///
/// Block 0 is the least significant block. Every block range is expressed over the complete
/// width of the block, see [`CiphertextBlockRange`].
#[derive(Clone, PartialEq, Eq)]
pub struct CiphertextRange(Vec<CiphertextBlockRange>);

impl CiphertextRange {
    fn from_block(spec: CiphertextSpec, block: CiphertextBlockRange) -> Self {
        Self(vec![block; spec.block_count().sas()])
    }

    /// Creates the range of a zero-initialized ciphertext: every block is exactly zero.
    pub fn zero_from_spec(spec: CiphertextSpec) -> Self {
        Self::from_block(
            spec,
            CiphertextBlockRange::from_single(spec.block_spec(), 0),
        )
    }

    /// Creates the range of a program input: every block spans the whole message space.
    pub fn input_from_spec(spec: CiphertextSpec) -> Self {
        Self::from_block(
            spec,
            CiphertextBlockRange::message_space_from_spec(spec.block_spec()),
        )
    }

    /// Returns the number of blocks.
    pub fn block_count(&self) -> u8 {
        self.0.len().sas()
    }

    /// Returns the range of the block at `ith`. Block 0 is the least significant block.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= block_count()`.
    pub fn get_block(&self, ith: u8) -> CiphertextBlockRange {
        assert!(ith < self.block_count(), "Tried to get nonexistent block.");
        self.0[ith.sas::<usize>()]
    }

    /// Replaces the range of the block at `ith`. Block 0 is the least significant block.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= block_count()` or if the block width does not match.
    pub fn set_block(&mut self, ith: u8, block: CiphertextBlockRange) {
        assert!(ith < self.block_count(), "Tried to set nonexistent block.");
        assert_eq!(
            self.0[ith.sas::<usize>()].n_bits(),
            block.n_bits(),
            "Spec mismatch."
        );
        self.0[ith.sas::<usize>()] = block;
    }

    /// Iterates over the block ranges, least significant block first.
    pub fn iter_blocks(&self) -> impl Iterator<Item = &CiphertextBlockRange> {
        self.0.iter()
    }
}

impl Debug for CiphertextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, block) in self.0.iter().enumerate().rev() {
            if i + 1 != self.0.len() {
                writeln!(f)?;
            }
            write!(f, "[{i}] {block:?}")?;
        }
        Ok(())
    }
}
