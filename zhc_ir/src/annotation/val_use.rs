use crate::{AnnOpRef, Annotation, Dialect};

/// Annotated consuming operation and argument position of a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnValUseRef<'s, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    /// Annotated reference to the operation that consumes this value.
    pub opref: AnnOpRef<'s, 'ann, D, OpAnn, ValAnn>,
    /// Zero-based index into the consuming operation's argument list.
    pub position: u8,
}
