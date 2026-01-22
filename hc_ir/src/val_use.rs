use std::ops::Deref;

use crate::{AnnOpRef, Dialect, OpId, OpRef};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnnValUseRef<'s, 'ann, D: Dialect, OpAnn, ValAnn> {
    pub opref: AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>,
    pub position: u8
}


impl<'s, 'ann, D: Dialect, OpAnn, ValAnn> Deref for AnnValUseRef<'s, 'ann, D, OpAnn, ValAnn> {
    type Target = AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>;

    fn deref(&self) -> &Self::Target {
        &self.opref
    }
}
