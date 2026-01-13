//! Utility data structures and algorithms for compiler infrastructure.
//!
//! This crate provides specialized collections and utilities optimized for
//! compiler workloads, including stack-allocated containers for small data,
//! type-safe indices, and iterator extensions.

mod erasable;
mod fast_hash;
mod fifo;
mod store;
pub mod tracing;
pub mod iter;
pub mod small;

mod type_name;
pub use erasable::*;
pub use fast_hash::*;
pub use fifo::*;
pub use store::*;
pub use type_name::*;
