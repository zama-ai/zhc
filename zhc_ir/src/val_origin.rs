use crate::{Dialect, OpId, OpRef};

/// The producing operation and return position of a value (owned IDs).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValOrigin {
    /// The operation that produces this value.
    pub opid: OpId,
    /// Zero-based index into the producing operation's return values.
    pub position: u8,
}

/// The producing operation and return position of a value (borrowed references).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValOriginRef<'s, D: Dialect> {
    /// Reference to the operation that produces this value.
    pub opref: OpRef<'s, D>,
    /// Zero-based index into the producing operation's return values.
    pub position: u8,
}
