use super::{SpecBlock, SpecInteger};
use std::fmt::Debug;

/// A circuit I/O type, either encrypted or plaintext.
///
/// Describes the types of a circuit's inputs and outputs. Each variant carries the corresponding
/// specification that fully describes the value's bit-width and per-block layout.
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    /// An encrypted integer with the given [`SpecInteger`].
    CiphertextInteger(SpecInteger),
    /// An encrypted Boolean stored in one clean ciphertext block.
    CiphertextBool(SpecBlock),
    /// A plaintext integer with the given [`SpecInteger`].
    PlaintextInteger(SpecInteger),
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::CiphertextInteger(spec) => write!(
                f,
                "CiphertextInteger<{}, {}, {}>",
                spec.int_size(),
                spec.block_spec().carry_size(),
                spec.block_spec().message_size()
            ),
            Type::CiphertextBool(spec) => write!(
                f,
                "CiphertextBool<{}, {}>",
                spec.carry_size(),
                spec.message_size()
            ),
            Type::PlaintextInteger(spec) => write!(
                f,
                "PlaintextInteger<{}, {}, {}>",
                spec.int_size(),
                spec.block_spec().carry_size(),
                spec.block_spec().message_size()
            ),
        }
    }
}
