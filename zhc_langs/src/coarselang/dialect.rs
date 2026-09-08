use zhc_ir::Dialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoarseLang;

impl Dialect for CoarseLang {
    type TypeSystem = super::CoarseTypeSystem;
    type InstructionSet = super::CoarseInstructionSet;
}
