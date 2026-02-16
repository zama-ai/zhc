//! Instruction scheduling algorithms and data structures.
//!
//! This module provides a framework for implementing instruction scheduling
//! algorithms that can reorder operations while respecting dependencies.
//! The primary implementation is forward list scheduling, which maintains
//! correctness while allowing optimization-driven reordering.

pub mod forward;
mod schedule;
mod tags;

pub use schedule::*;
pub use tags::*;
