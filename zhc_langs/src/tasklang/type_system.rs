use zhc_ir::DialectTypeSystem;
use zhc_utils::DisplayVariant;

#[derive(DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskTypeSystem {
    Val,
}

impl DialectTypeSystem for TaskTypeSystem {}
