use zhc_ir::DialectTypeSystem;
use zhc_utils::DisplayVariant;

/// Type system of the task dialect.
///
/// Has the single type `Val`, so that a task graph records only the dependencies between tasks.
#[derive(DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskTypeSystem {
    Val,
}

impl DialectTypeSystem for TaskTypeSystem {}
