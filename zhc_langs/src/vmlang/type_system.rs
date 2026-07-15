use std::fmt::Display;

use zhc_ir::DialectTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmTypeSystem {
    CtRegister,
    PtImmediate,
}

impl Display for VmTypeSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use VmTypeSystem::*;
        match self {
            CtRegister => write!(f, "CtRegister"),
            PtImmediate => write!(f, "PtImmediate"),
        }
    }
}

impl DialectTypeSystem for VmTypeSystem {}
