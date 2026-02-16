use std::fmt::Debug;

pub trait Annotation: PartialEq + Eq + Debug + Clone + 'static {}

impl<T> Annotation for T where T: PartialEq + Eq + Debug + Clone + 'static {}
