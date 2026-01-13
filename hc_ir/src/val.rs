use hc_utils::small::SmallVec;

use super::{Dialect, OpId, State};

/// Represents a value within the intermediate representation.
///
/// A value is produced by one operation and can be consumed by multiple operations.
/// Values carry type information from their originating dialect and maintain
/// references to all operations that use them for dependency tracking.
#[derive(Debug)]
pub struct Val<D: Dialect> {
    /// Operations that consume this value as an argument.
    pub users: SmallVec<OpId>,
    /// Operation that produced this value.
    pub origin: OpId,
    /// Type of this value within the dialect's type system.
    pub typ: D::Types,
    /// Current state of the value.
    pub state: State,
}
