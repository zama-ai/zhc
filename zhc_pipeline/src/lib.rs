//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

mod commons;
pub mod compat;
mod hpu;
mod misc;
mod multi_hpu;
mod pipeline;
mod vm;

pub use commons::*;
pub use hpu::metrics::HpuMetrics;
pub use hpu::translation_table::hpu_stream_heap_usage;
pub use misc::PbsMetrics;
pub use pipeline::Pipeline;
pub use vm::scheduler::VmExecutionPlan;

#[cfg(test)]
mod test;
