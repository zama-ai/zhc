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

#[cfg(test)]
mod tests;
