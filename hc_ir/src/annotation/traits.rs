use std::fmt::Debug;
use std::hash::Hash;

pub trait Annotation: PartialEq + Eq + Debug + Hash + Clone + 'static {}

impl<T> Annotation for T where T: PartialEq + Eq + Debug + Hash + Clone + 'static {}
