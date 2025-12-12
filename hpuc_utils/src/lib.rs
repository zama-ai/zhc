//! Utility data structures and algorithms for compiler infrastructure.
//!
//! This crate provides specialized collections and utilities optimized for
//! compiler workloads, including stack-allocated containers for small data,
//! type-safe indices, and iterator extensions.

mod all_eq;
mod chunk;
mod collectors;
mod erasable;
mod fast_hash;
mod fifo;
mod interleave;
mod mzip;
mod small_map;
mod small_set;
mod small_vec;
mod stack_map;
mod stack_set;
mod stack_vec;
mod store;
pub mod tracing;
mod type_name;

pub use all_eq::*;
pub use chunk::*;
pub use collectors::*;
pub use erasable::*;
pub use fast_hash::*;
pub use fifo::*;
pub use interleave::*;
pub use mzip::*;
pub use small_map::*;
pub use small_set::*;
pub use small_vec::*;
pub use stack_map::*;
pub use stack_set::*;
pub use stack_vec::*;
pub use store::*;
pub use type_name::*;
