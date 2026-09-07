use std::fmt::Debug;

use zhc_utils::{SafeAs, ValueSet};

use crate::integer_semantics::{CiphertextBlockSpec, PlaintextBlockRange};

/// The set of values a ciphertext block can take.
///
/// Values are tracked over the complete width of the block, `[padding | carry | message]`, so
/// every member is a raw complete-bits value. All arithmetic wraps modulo `2^complete_size`,
/// which mirrors the `Wrapping` flavor of block operations and stays sound for every flavor.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CiphertextBlockRange(ValueSet);

impl CiphertextBlockRange {
    /// Wraps a value set already expressed over the complete width of a block.
    pub fn new(set: ValueSet) -> Self {
        Self(set)
    }

    /// Creates a range with no possible value.
    pub fn empty_from_spec(spec: CiphertextBlockSpec) -> Self {
        Self(ValueSet::new_empty(spec.complete_size()))
    }

    /// Creates a range holding exactly one complete-bits value.
    pub fn from_single(spec: CiphertextBlockSpec, value: u8) -> Self {
        Self(ValueSet::from_single(spec.complete_size(), value))
    }

    /// Creates the range of a clean block: every message value, zero carry and padding.
    pub fn message_space_from_spec(spec: CiphertextBlockSpec) -> Self {
        let mut set = ValueSet::new_empty(spec.complete_size());
        for v in spec.iter_message_space() {
            set.insert(v.raw_complete_bits().sas());
        }
        Self(set)
    }

    /// Returns the underlying value set.
    pub fn as_value_set(&self) -> ValueSet {
        self.0
    }

    /// Returns the complete width of the block in bits.
    pub fn n_bits(&self) -> u8 {
        self.0.n_bits()
    }

    /// Pairwise wrapping sum of two ciphertext block ranges.
    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self(ValueSet::wrapping_add(self.0, rhs.0))
    }

    /// Pairwise wrapping difference of two ciphertext block ranges.
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(ValueSet::wrapping_sub(self.0, rhs.0))
    }

    /// Multiply-accumulate `self * mul + rhs`, wrapping.
    pub fn wrapping_mac(self, mul: u8, rhs: Self) -> Self {
        Self(ValueSet::wrapping_add(
            self.0.wrapping_mul_scalar(mul),
            rhs.0,
        ))
    }

    /// Truncating left shift of every member by `amount` bits.
    pub fn wrapping_shl(self, amount: u8) -> Self {
        Self(self.0.wrapping_shl_scalar(amount))
    }

    /// Pairwise wrapping sum with a plaintext block range.
    pub fn wrapping_add_pt(self, rhs: PlaintextBlockRange) -> Self {
        Self(ValueSet::wrapping_add(
            self.0,
            rhs.widened_to(self.n_bits()),
        ))
    }

    /// Pairwise wrapping difference with a plaintext block range.
    pub fn wrapping_sub_pt(self, rhs: PlaintextBlockRange) -> Self {
        Self(ValueSet::wrapping_sub(
            self.0,
            rhs.widened_to(self.n_bits()),
        ))
    }

    /// Pairwise wrapping product with a plaintext block range.
    pub fn wrapping_mul_pt(self, rhs: PlaintextBlockRange) -> Self {
        Self(ValueSet::wrapping_mul(
            self.0,
            rhs.widened_to(self.n_bits()),
        ))
    }

    /// Image of the range under a function on complete-bits values.
    pub fn apply(self, func: impl Fn(u8) -> u8) -> Self {
        Self(self.0.apply(func))
    }
}

impl Debug for CiphertextBlockRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
