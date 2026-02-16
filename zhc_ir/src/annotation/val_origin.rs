use crate::{AnnOpRef, Annotation, Dialect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnValOriginRef<'s, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    pub opref: AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>,
    pub position: u8,
}
