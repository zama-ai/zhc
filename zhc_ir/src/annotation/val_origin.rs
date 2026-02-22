use crate::{AnnOpRef, Annotation, Dialect};

/// Annotated producing operation and return position of a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnValOriginRef<'s, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    /// Annotated reference to the operation that produces this value.
    pub opref: AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>,
    /// Zero-based index into the producing operation's return values.
    pub position: u8,
}
