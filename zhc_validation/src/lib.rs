//! Correctness and snapshot tests for compilation passes using builder-generated circuits.

pub mod doplang;
pub mod equivalence_check;
pub mod hpulang;
pub mod ioplang;
pub mod symbolic;
pub mod vmlang;

#[cfg(test)]
mod pipeline_invalidation;
