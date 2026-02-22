use std::fmt::Display;

use zhc_ir::DialectTypeSystem;

/// Type system for the DOP dialect.
///
/// DOP uses a single opaque context type to thread execution ordering
/// through the IR. The inner `usize` serves as a context generation
/// counter, not as a data type discriminator: every instruction that
/// produces or consumes a value does so through its inline
/// [`Argument`](super::Argument) fields, not through the IR's SSA
/// value system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DopTypeSystem {
    /// Opaque execution context token carrying a generation counter.
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
