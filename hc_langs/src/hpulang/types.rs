use std::fmt::Display;

use hc_ir::DialectTypes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Types {
    CtRegister,
    PtImmediate,
    CtHeap,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::CtRegister => write!(f, "CtRegister"),
            Types::CtHeap => write!(f, "CtHeap"),
            Types::PtImmediate => write!(f, "PtImmediate"),
        }
    }
}

impl DialectTypes for Types {}
