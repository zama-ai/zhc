//! Intermediate representation and optimization framework for compiler infrastructure.
//!
//! This crate provides a generic IR framework that supports multiple dialects,
//! along with common compiler optimizations like dead code elimination and
//! common subexpression elimination. The IR uses a graph-based representation
//! with typed operations and values.

pub mod cse;
pub mod dce;
pub mod interpretation;
pub mod scheduling;
pub mod translation;
pub mod traversal;
pub mod visualization;

mod annotation;
mod dialect;
mod formatting;
mod id;
mod ir;
mod op;
mod op_map;
mod op_mut;
mod op_ref;
mod signature;
mod state;
mod val;
mod val_map;
mod val_mut;
mod val_origin;
mod val_ref;
mod val_use;

pub(crate) use annotation::*;
pub use dialect::*;
pub use formatting::*;
pub use id::*;
pub use ir::*;
pub(crate) use op::*;
pub use op_map::*;
pub(crate) use op_mut::*;
pub use op_ref::*;
pub use signature::*;
pub(crate) use state::*;
pub(crate) use val::*;
pub use val_map::*;
pub(crate) use val_mut::*;
pub use val_origin::*;
pub use val_ref::*;
pub use val_use::*;

/// Errors that can occur during IR construction or manipulation.
#[derive(Clone, Debug)]
pub enum IRError<D: Dialect> {
    /// Operation signature mismatch between expected and received argument types.
    OpSig {
        /// The operation that caused the signature error.
        op: D::InstructionSet,
        /// The types that were actually received as arguments.
        recv: Vec<D::TypeSystem>,
        /// The types that were expected as arguments.
        exp: Vec<D::TypeSystem>,
    },
    /// Value cannot be represented with the specified type.
    Range {
        /// The type that cannot represent the value.
        typ: D::TypeSystem,
    },
}

impl<D: Dialect> std::fmt::Display for IRError<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRError::OpSig { op, recv, exp } => {
                write!(
                    f,
                    "Signature Error: {op} received {recv:?} instead of {exp:?}"
                )
            }
            IRError::Range { typ } => {
                write!(f, "Range Error: value could not be represented with {typ}")
            }
        }
    }
}

impl<D: Dialect> std::error::Error for IRError<D> {}

#[cfg(test)]
mod tests;
