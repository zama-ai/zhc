use super::signature::Signature;
use std::fmt::{Debug, Display};

pub trait DialectTypes: Clone + Debug + Display + PartialEq + Eq {}

pub trait DialectOperations: Clone + Debug + Display {
    type Types: DialectTypes;
    fn get_signature(&self) -> Signature<Self::Types>;
}

pub trait Dialect: Clone + Debug {
    type Types: DialectTypes;
    type Operations: DialectOperations<Types = Self::Types>;
}
