//! Utility data structures and algorithms for compiler infrastructure.
//!
//! This crate provides specialized collections and utilities optimized for
//! compiler workloads, including stack-allocated containers for small data,
//! type-safe indices, and iterator extensions.

pub mod assert_display;
pub mod data_visulization;
pub mod graphics;
pub mod iter;
pub mod small;
pub mod tracing;

mod bi_map;
mod change_guard;
mod dump;
mod erasable;
mod fast_hash;
mod fifo;
mod misc;
mod safe_as;
mod store;
mod type_name;

pub use bi_map::*;
pub use change_guard::*;
pub use dump::*;
pub use erasable::*;
pub use fast_hash::*;
pub use fifo::*;
pub use misc::*;
pub use safe_as::*;
pub use store::*;
pub use type_name::*;

pub use zhc_utils_macro::assert_display_is;
pub use zhc_utils_macro::fsm;
