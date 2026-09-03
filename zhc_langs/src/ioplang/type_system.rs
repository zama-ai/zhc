use zhc_ir::DialectTypeSystem;
use zhc_utils::DisplayVariant;

/// Type system for the IOP dialect.
///
/// Distinguishes composite multi-block values (`Ciphertext`, `Plaintext`)
/// from their individual scalar blocks (`CiphertextBlock`, `PlaintextBlock`).
#[derive(DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
pub enum IopTypeSystem {
    /// Multi-block radix ciphertext (encrypted integer).
    Ciphertext,
    /// Multi-block radix plaintext (clear integer).
    Plaintext,
    /// Single LWE ciphertext block.
    CiphertextBlock,
    /// Single plaintext block.
    PlaintextBlock,
}

impl DialectTypeSystem for IopTypeSystem {}
