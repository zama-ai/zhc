use crate::OpId;

/// Represents an operation that has completed execution and can be retired.
///
/// Operations in this state have finished executing and no longer consume
/// execution resources. Retiring an operation may enable its dependent
/// operations to become ready for scheduling.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Retired(pub OpId);

/// Represents an operation that is ready to be scheduled for execution.
///
/// Operations in this state have all their dependencies satisfied and
/// can be considered for selection by the scheduler. The scheduler
/// may choose to issue some or all ready operations in each cycle.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Ready(pub OpId);

/// Represents an operation that has been selected for execution.
///
/// Operations in this state have been chosen by the scheduler for
/// execution and will transition to active state. Selected operations
/// consume execution resources until they complete and retire.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Selected(pub OpId);
