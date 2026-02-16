use std::fmt::Display;

use zhc_ir::DialectTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DopTypeSystem {
    Ctx(usize),
}

impl Display for DopTypeSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DopTypeSystem::Ctx(_) => write!(f, "Ctx"),
        }
    }
}

impl DialectTypeSystem for DopTypeSystem {}
