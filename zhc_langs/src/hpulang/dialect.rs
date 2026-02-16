use zhc_ir::Dialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HpuLang;

impl Dialect for HpuLang {
    type TypeSystem = super::HpuTypeSystem;
    type InstructionSet = super::HpuInstructionSet;
}
