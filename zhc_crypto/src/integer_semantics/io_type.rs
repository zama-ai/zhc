use super::{CiphertextBlockSpec, IntegerCiphertextSpec, IntegerPlaintextSpec};
use std::fmt::Debug;

/// A circuit I/O type, either encrypted or plaintext.
///
/// Describes the types of a circuit's inputs and outputs. Each variant carries the corresponding
/// specification that fully describes the integer's bit-width and per-block layout.
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    /// An encrypted integer with the given [`IntegerCiphertextSpec`].
    IntegerCiphertext(IntegerCiphertextSpec),
    /// An encrypted Boolean stored in one clean ciphertext block.
    BoolCiphertext(CiphertextBlockSpec),
    /// A plaintext integer with the given [`IntegerPlaintextSpec`].
    IntegerPlaintext(IntegerPlaintextSpec),
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::IntegerCiphertext(spec) => write!(
                f,
                "IntegerCiphertext<{}, {}, {}>",
                spec.int_size(),
                spec.block_spec().carry_size(),
                spec.block_spec().message_size()
            ),
            Type::BoolCiphertext(spec) => write!(
                f,
                "BoolCiphertext<{}, {}>",
                spec.carry_size(),
                spec.message_size()
            ),
            Type::IntegerPlaintext(spec) => write!(
                f,
                "IntegerPlaintext<{}, {}>",
                spec.int_size(),
                spec.block_spec().message_size()
            ),
        }
    }
}
