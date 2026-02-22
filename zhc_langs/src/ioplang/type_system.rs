use std::fmt::Display;
use zhc_ir::DialectTypeSystem;

/// Type system for the IOP dialect.
///
/// Distinguishes composite multi-block values (`Ciphertext`, `Plaintext`)
/// from their individual scalar blocks (`CiphertextBlock`, `PlaintextBlock`).
/// The `Lut{1,2,4,8}` variants represent lookup table types of increasing
/// output arity, used as operands to PBS instructions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IopTypeSystem {
    /// Multi-block radix ciphertext (encrypted integer).
    Ciphertext,
    /// Multi-block radix plaintext (clear integer).
    Plaintext,
    /// Single LWE ciphertext block.
    CiphertextBlock,
    /// Single plaintext block.
    PlaintextBlock,
    /// Single-output lookup table.
    Lut1,
    /// Two-output lookup table.
    Lut2,
    /// Four-output lookup table.
    Lut4,
    /// Eight-output lookup table.
    Lut8,
}

impl Display for IopTypeSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IopTypeSystem::Ciphertext => write!(f, "Ct"),
            IopTypeSystem::Plaintext => write!(f, "Pt"),
            IopTypeSystem::Lut1 => write!(f, "Lut1"),
            IopTypeSystem::Lut2 => write!(f, "Lut2"),
            IopTypeSystem::Lut4 => write!(f, "Lut4"),
            IopTypeSystem::Lut8 => write!(f, "Lut8"),
            IopTypeSystem::CiphertextBlock => write!(f, "CtBlock"),
            IopTypeSystem::PlaintextBlock => write!(f, "PtBlock"),
        }
    }
}

impl DialectTypeSystem for IopTypeSystem {}
