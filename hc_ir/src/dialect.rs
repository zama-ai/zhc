use super::signature::Signature;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Types that can be used within a dialect's type system.
///
/// This trait defines the requirements for types that can represent the data types
/// within a specific dialect.
pub trait DialectTypes: Clone + Debug + Display + PartialEq + Eq + Hash + 'static {}

/// Operations that can be used within a dialect.
///
/// This trait defines the requirements for operations within a specific dialect.
pub trait DialectOperations: Clone + Debug + Display + PartialEq + Eq + Hash + 'static {
    /// The type system associated with this operation dialect.
    type Types: DialectTypes;

    /// Returns the signature of this operation.
    ///
    /// The signature specifies the argument types this operation accepts
    /// and the return value types it produces.
    fn get_signature(&self) -> Signature<Self::Types>;
}

/// A complete dialect definition combining types and operations.
///
/// A dialect represents a coherent set of types and operations that can be used
/// together within an IR. The dialect ensures that operations are properly
/// typed according to the dialect's type system.
pub trait Dialect: Clone + Debug + PartialEq + Eq + Hash + 'static {
    /// The type system for this dialect.
    type Types: DialectTypes;

    /// The operation set for this dialect.
    type Operations: DialectOperations<Types = Self::Types>;
}
