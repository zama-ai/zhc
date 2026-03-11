use super::signature::Signature;
use crate::Format;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// The type system of a dialect.
///
/// Implementors are typically enums whose variants represent the individual
/// types available in the dialect.
pub trait DialectTypeSystem: Clone + Debug + Display + PartialEq + Eq + Hash + 'static {}

/// The instruction set of a dialect.
///
/// Implementors are typically enums whose variants represent the individual
/// instructions available in the dialect.
pub trait DialectInstructionSet: Clone + Debug + Format + PartialEq + Eq + Hash + 'static {
    /// The type system associated with this instruction set.
    type TypeSystem: DialectTypeSystem;

    /// Returns the signature of this instruction.
    ///
    /// The signature specifies the argument types this instruction accepts
    /// and the result types it produces.
    fn get_signature(&self) -> Signature<Self::TypeSystem>;
}

/// A dialect combining a type system and an instruction set.
///
/// Implementors are typically unit structs that bind together a [`DialectTypeSystem`]
/// and a [`DialectInstructionSet`].
pub trait Dialect: Clone + Debug + PartialEq + Eq + Hash + 'static {
    /// The type system for this dialect.
    type TypeSystem: DialectTypeSystem;

    /// The instruction set for this dialect.
    type InstructionSet: DialectInstructionSet<TypeSystem = Self::TypeSystem>;
}
