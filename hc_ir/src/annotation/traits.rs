use std::fmt::Debug;
use std::hash::Hash;

pub trait Annotation: PartialEq + Debug + Hash {}

impl<T> Annotation for T where T: PartialEq + Eq + Debug + Hash {}
