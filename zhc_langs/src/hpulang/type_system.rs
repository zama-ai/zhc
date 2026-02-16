use std::fmt::Display;

use zhc_ir::DialectTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HpuTypeSystem {
    CtRegister,
    PtImmediate,
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
