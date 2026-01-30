//! Utility data structures and algorithms for compiler infrastructure.
//!
//! This crate provides specialized collections and utilities optimized for
//! compiler workloads, including stack-allocated containers for small data,
//! type-safe indices, and iterator extensions.

mod change_guard;
mod erasable;
mod fast_hash;
mod fifo;
pub mod graphics;
pub mod iter;
pub mod small;
mod store;
pub mod tracing;

mod type_name;
pub use change_guard::*;
pub use erasable::*;
pub use fast_hash::*;
pub use fifo::*;
pub use store::*;
pub use type_name::*;
