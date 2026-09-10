use zhc_ir::Dialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskLang;

impl Dialect for TaskLang {
    type TypeSystem = super::TaskTypeSystem;
    type InstructionSet = super::TaskInstructionSet;
}
