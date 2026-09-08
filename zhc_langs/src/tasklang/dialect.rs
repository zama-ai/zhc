use zhc_ir::Dialect;

/// Dialect of coarse tasks, used to partition IOP programs.
///
/// Binds [`TaskTypeSystem`](super::TaskTypeSystem) and
/// [`TaskInstructionSet`](super::TaskInstructionSet).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskLang;

impl Dialect for TaskLang {
    type TypeSystem = super::TaskTypeSystem;
    type InstructionSet = super::TaskInstructionSet;
}
