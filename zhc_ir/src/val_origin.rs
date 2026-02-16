use crate::{Dialect, OpId, OpRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValOrigin {
    pub opid: OpId,
    pub position: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValOriginRef<'s, D: Dialect> {
    pub opref: OpRef<'s, D>,
    pub position: u8,
}
