use std::ops::Deref;

use crate::{Dialect, OpId, OpRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValUse {
    pub opid: OpId,
    pub position: u8
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValUseRef<'s, D: Dialect> {
    pub opref: OpRef<'s, D>,
    pub position: u8
}


impl<'s, D: Dialect> Deref for ValUseRef<'s, D> {
    type Target = OpRef<'s, D>;

    fn deref(&self) -> &Self::Target {
        &self.opref
    }
}
