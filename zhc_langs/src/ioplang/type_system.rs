use std::fmt::Display;
use zhc_ir::DialectTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IopTypeSystem {
    Ciphertext,
    Plaintext,
    CiphertextBlock,
    PlaintextBlock,
    Lut1,
    Lut2,
    Lut4,
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
