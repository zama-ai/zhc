use zhc_ir::DialectTypeSystem;
use zhc_utils::DisplayVariant;

/// Type system for the IOP dialect.
///
/// Distinguishes composite multi-block values (`CiphertextInteger`, `PlaintextInteger`)
/// from their individual scalar blocks (`CiphertextBlock`, `PlaintextBlock`).
/// The `Lut{1,2,4,8}` variants represent lookup table types of increasing
/// output arity, used as operands to PBS instructions.
#[derive(DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
pub enum IopTypeSystem {
    /// Multi-block radix ciphertext (encrypted integer).
    CiphertextInteger,
    /// Encrypted Boolean backed by one clean ciphertext block.
    CiphertextBool,
    /// Multi-block radix plaintext (clear integer).
    PlaintextInteger,
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

impl DialectTypeSystem for IopTypeSystem {}
