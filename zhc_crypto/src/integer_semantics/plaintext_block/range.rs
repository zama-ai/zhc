use std::fmt::Debug;

use zhc_utils::{SafeAs, ValueSet};

use crate::integer_semantics::{CiphertextBlockRange, PlaintextBlockSpec};

/// The set of values a plaintext block can take.
///
/// Values are tracked over the message width of the block. When combined with a
/// [`CiphertextBlockRange`], the set is widened to the complete width of the ciphertext block,
/// which mirrors how block operations treat plaintext operands.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlaintextBlockRange(ValueSet);

impl PlaintextBlockRange {
    /// Wraps a value set expressed over the message width of a block.
    pub fn new(set: ValueSet) -> Self {
        Self(set)
    }

    /// Creates a range with no possible value.
    pub fn empty_from_spec(spec: PlaintextBlockSpec) -> Self {
        Self(ValueSet::new_empty(spec.message_size()))
    }

    /// Creates a range holding exactly one message value.
    pub fn from_single(spec: PlaintextBlockSpec, value: u8) -> Self {
        Self(ValueSet::from_single(spec.message_size(), value))
    }

    /// Creates the range of every message value.
    pub fn message_space_from_spec(spec: PlaintextBlockSpec) -> Self {
        let mut set = ValueSet::new_empty(spec.message_size());
        for v in spec.iter_message_space() {
            set.insert(v.raw_message_bits().sas());
        }
        Self(set)
    }

    /// Returns the underlying value set.
    pub fn as_value_set(&self) -> ValueSet {
        self.0
    }

    /// Returns the message width of the block in bits.
    pub fn n_bits(&self) -> u8 {
        self.0.n_bits()
    }

    /// Pairwise wrapping difference `self - rhs`, over the width of the ciphertext block.
    pub fn wrapping_sub_ct(self, rhs: CiphertextBlockRange) -> CiphertextBlockRange {
        CiphertextBlockRange::new(ValueSet::wrapping_sub(
            self.widened_to(rhs.n_bits()),
            rhs.as_value_set(),
        ))
    }

    /// Re-interprets the set over the width of a ciphertext block.
    ///
    /// # Panics
    ///
    /// Panics if the plaintext block is wider than `n_bits`.
    pub(crate) fn widened_to(self, n_bits: u8) -> ValueSet {
        assert!(
            self.n_bits() <= n_bits,
            "Spec mismatch. plaintext block: {}, ciphertext block: {}",
            self.n_bits(),
            n_bits
        );
        self.0.widen(n_bits)
    }
}

impl Debug for PlaintextBlockRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
