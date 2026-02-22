use crate::{Dialect, OpId, OpRef};

/// A consuming operation and argument position for a value (owned IDs).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValUse {
    /// The operation that consumes this value.
    pub opid: OpId,
    /// Zero-based index into the consuming operation's argument list.
    pub position: u8,
}

/// A consuming operation and argument position for a value (borrowed references).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValUseRef<'s, D: Dialect> {
    /// Reference to the operation that consumes this value.
    pub opref: OpRef<'s, D>,
    /// Zero-based index into the consuming operation's argument list.
    pub position: u8,
}
