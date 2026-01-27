use std::ops::Deref;

use crate::{AnnOpRef, Annotation, Dialect};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnnValUseRef<'s, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    pub opref: AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>,
    pub position: u8
}


impl<'s, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Deref for AnnValUseRef<'s, 'ann, D, OpAnn, ValAnn> {
    type Target = AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>;

    fn deref(&self) -> &Self::Target {
        &self.opref
    }
}
