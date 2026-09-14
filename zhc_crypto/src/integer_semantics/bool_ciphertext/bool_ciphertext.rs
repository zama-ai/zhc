use rand::RngExt;
use std::fmt::Debug;
use zhc_utils::Dumpable;

use super::super::{CiphertextBlockSpec, EmulatedCiphertextBlock};

/// An emulated encrypted Boolean stored in one clean ciphertext block.
///
/// The block has no carry or padding bits set and its message is exactly zero or one.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EmulatedBoolCiphertext {
    block: EmulatedCiphertextBlock,
}

impl EmulatedBoolCiphertext {
    /// Creates an encrypted Boolean using the supplied block layout.
    pub fn from_bool(value: bool, spec: CiphertextBlockSpec) -> Self {
        Self {
            block: spec.from_message(value.into()),
        }
    }

    /// Wraps a block whose complete value is exactly zero or one.
    ///
    /// # Panics
    ///
    /// Panics if the block has active carry or padding bits, or if its message exceeds one.
    pub fn from_block(block: EmulatedCiphertextBlock) -> Self {
        assert!(
            block.is_message_only() && block.raw_message_bits() <= 1,
            "Tried to create a boolean ciphertext from a non-boolean block."
        );
        Self { block }
    }

    /// Generates a random encrypted Boolean for semantic testing.
    pub fn random(spec: CiphertextBlockSpec) -> Self {
        super::super::PRNG.with_borrow_mut(|prng| Self::from_bool(prng.random::<bool>(), spec))
    }

    /// Returns the sole clean block backing this Boolean.
    pub fn get_block(self) -> EmulatedCiphertextBlock {
        self.block
    }

    /// Returns the block layout used by this Boolean.
    pub fn spec(self) -> CiphertextBlockSpec {
        self.block.spec()
    }

    /// Returns the clear Boolean represented by this emulated value.
    pub fn as_bool(self) -> bool {
        self.block.raw_message_bits() == 1
    }
}

impl Debug for EmulatedBoolCiphertext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_bool_ct", self.as_bool())
    }
}

impl Dumpable for EmulatedBoolCiphertext {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}
