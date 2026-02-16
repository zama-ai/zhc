use zhc_ir::Dialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DopLang;

impl Dialect for DopLang {
    type TypeSystem = super::DopTypeSystem;
    type InstructionSet = super::DopInstructionSet;
}
