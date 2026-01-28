use hc_ir::DialectTypes;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Types {
    Ciphertext,
    Plaintext,
    CiphertextBlock,
    PlaintextBlock,
    Index,
    Lut1,
    Lut2,
    Lut4,
    Lut8,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Ciphertext => write!(f, "CtInt"),
            Types::Plaintext => write!(f, "PtInt"),
            Types::Index => write!(f, "Index"),
            Types::Lut1 => write!(f, "Lut1"),
            Types::Lut2 => write!(f, "Lut2"),
            Types::Lut4 => write!(f, "Lut4"),
            Types::Lut8 => write!(f, "Lut8"),
            Types::CiphertextBlock => write!(f, "CtBlock"),
            Types::PlaintextBlock => write!(f, "PtBlock"),
        }
    }
}

impl DialectTypes for Types {}
