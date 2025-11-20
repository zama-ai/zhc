use super::signature::Signature;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait DialectTypes: Clone + Debug + Display + PartialEq + Eq + Hash + 'static {}

pub trait DialectOperations: Clone + Debug + Display + PartialEq + Eq + Hash + 'static {
    type Types: DialectTypes;
    fn get_signature(&self) -> Signature<Self::Types>;
}

pub trait Dialect: Clone + Debug + PartialEq + Eq + Hash + 'static {
    type Types: DialectTypes;
    type Operations: DialectOperations<Types = Self::Types>;
}
