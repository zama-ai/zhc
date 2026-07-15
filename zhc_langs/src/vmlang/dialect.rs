use zhc_ir::{Dialect, cse::AllowCse};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VmLang;

impl Dialect for VmLang {
    type TypeSystem = super::VmTypeSystem;
    type InstructionSet = super::VmInstructionSet;
}

impl AllowCse for VmLang {}
