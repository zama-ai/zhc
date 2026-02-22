use std::fmt::Display;

use zhc_ir::DialectTypeSystem;

/// Type system for the HPU dialect.
///
/// Models the three storage classes visible at the HPU register level:
/// ciphertext registers, plaintext immediates, and heap-spilled
/// ciphertexts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HpuTypeSystem {
    /// Ciphertext block held in a virtual register.
    CtRegister,
    /// Plaintext scalar loaded from an input slot or inlined as a
    /// constant.
    PtImmediate,
    /// Ciphertext block spilled to the heap.
    CtHeap,
}

impl Display for HpuTypeSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HpuTypeSystem::CtRegister => write!(f, "CtRegister"),
            HpuTypeSystem::CtHeap => write!(f, "CtHeap"),
            HpuTypeSystem::PtImmediate => write!(f, "PtImmediate"),
        }
    }
}

impl DialectTypeSystem for HpuTypeSystem {}
