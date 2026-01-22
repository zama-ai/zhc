use std::ops::Deref;

use crate::{AnnOpRef, Dialect, OpId, OpRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValOrigin {
    pub opid: OpId,
    pub position: u8
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValOriginRef<'s, D: Dialect> {
    pub opref: OpRef<'s, D>,
    pub position: u8
}

impl<'s, D: Dialect> Deref for ValOriginRef<'s, D> {
    type Target = OpRef<'s, D>;

    fn deref(&self) -> &Self::Target {
        &self.opref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnnValOriginRef<'s, 'ann, D: Dialect, OpAnn, ValAnn> {
    pub opref: AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>,
    pub position: u8
}

impl<'s, 'ann, D: Dialect, OpAnn, ValAnn> Deref for AnnValOriginRef<'s, 'ann, D, OpAnn, ValAnn> {
    type Target = AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>;

    fn deref(&self) -> &Self::Target {
        &self.opref
    }
}
