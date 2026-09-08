use super::{CiphertextSpec, PlaintextSpec};
use std::fmt::Debug;

/// A circuit I/O type, either encrypted or plaintext.
///
/// Describes the types of a circuit's inputs and outputs. Each variant carries the corresponding
/// specification that fully describes the integer's bit-width and per-block layout.
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    /// An encrypted integer with the given [`CiphertextSpec`].
    Ciphertext(CiphertextSpec),
    /// A plaintext integer with the given [`PlaintextSpec`].
    Plaintext(PlaintextSpec),
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Ciphertext(spec) => write!(
                f,
                "Ciphertext<{}, {}, {}>",
                spec.int_size(),
                spec.block_spec().carry_size(),
                spec.block_spec().message_size()
            ),
            Type::Plaintext(spec) => write!(
                f,
                "Plaintext<{}, {}>",
                spec.int_size(),
                spec.block_spec().message_size()
            ),
        }
    }
}
