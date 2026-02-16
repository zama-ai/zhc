use crate::{ValOrigin, val_use::ValUse};
use zhc_utils::small::SmallVec;

use super::{Dialect, State};

/// Represents a value within the intermediate representation.
///
/// A value is produced by one operation and can be consumed by multiple operations.
/// Values carry type information from their originating dialect and maintain
/// references to all operations that use them for dependency tracking.
#[derive(Debug)]
pub struct Val<D: Dialect> {
    /// Operations that consume this value as an argument.
    pub users: SmallVec<ValUse>,
    /// Operation that produced this value.
    pub origin: ValOrigin,
    /// Type of this value within the dialect's type system.
    pub typ: D::TypeSystem,
    /// Current state of the value.
    pub state: State,
}
