//! Correctness and snapshot tests for compilation passes using builder-generated circuits.

pub mod equivalence_check;
pub mod ioplang;
pub mod symbolic;

#[cfg(test)]
mod pipeline_invalidation;
