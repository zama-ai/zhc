//! Dialect-parameterized intermediate representation framework.
//!
//! The core type [`IR<D>`] holds a set of typed operations and values for a given
//! [`Dialect`]. Each operation carries a [`Signature`] that describes its argument
//! and return types, and each value tracks its producing operation ([`ValOrigin`])
//! and its consumers ([`ValUse`]). Operations and values are identified by [`OpId`]
//! and [`ValId`] respectively, and follow an active/inactive lifecycle.
//!
//! Immutable views into the IR are provided by [`OpRef`] and [`ValRef`], which
//! expose dependency traversal, reachability queries, and formatting. Mutable
//! operations on the IR — adding operations, replacing value uses, and deleting
//! operations — are methods on [`IR`] itself.
//!
//! The annotation layer ([`AnnIR`], [`AnnOpRef`], [`AnnValRef`]) extends the base
//! IR with per-operation and per-value metadata through parallel [`OpMap`] and
//! [`ValMap`] containers, enabling dataflow analyses that produce typed annotations
//! without modifying the underlying IR.
//!
//! Built-in passes include dead code elimination ([`dce`]), common subexpression
//! elimination ([`cse`]), and an interpretation framework ([`interpretation`]).
//! The [`scheduling`] module provides forward list scheduling, [`translation`]
//! defines cross-dialect translation, and [`traversal`] offers walker verification
//! utilities.

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

pub use annotation::*;
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
#[allow(unused)]
pub(crate) mod testlang;
#[cfg(test)]
mod tests;
