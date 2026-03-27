use crate::{
    Dialect,
    visualization::layoutlang::{LayoutInstructionSet, LayoutTypeSystem},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LayoutDialect;

impl Dialect for LayoutDialect {
    type TypeSystem = LayoutTypeSystem;
    type InstructionSet = LayoutInstructionSet;
}
