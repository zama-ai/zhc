use crate::OpId;

/// Represents an operation that has completed execution and can be retired.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Retired(pub OpId);

/// Represents an operation that is ready to be scheduled for execution.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Ready(pub OpId);

/// Represents an operation that has been selected for execution.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Selected(pub OpId);
