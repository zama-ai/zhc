use zhc_ir::DialectTypeSystem;
use zhc_utils::DisplayVariant;

#[derive(DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoarseTypeSystem {
    Val,
}

impl DialectTypeSystem for CoarseTypeSystem {}
