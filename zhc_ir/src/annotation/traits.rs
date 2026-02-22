use std::fmt::Debug;

/// Marker trait for types eligible as IR annotations.
///
/// Automatically implemented for all types satisfying the required bounds.
/// No manual implementation is needed.
pub trait Annotation: PartialEq + Eq + Debug + Clone + 'static {}

impl<T> Annotation for T where T: PartialEq + Eq + Debug + Clone + 'static {}
