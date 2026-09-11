//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

mod commons;
mod hpu;
mod misc;
mod multi_hpu;
mod pipeline;
mod vm;

pub use commons::*;
pub use hpu::metrics::HpuMetrics;
pub use misc::*;
pub use pipeline::Pipeline;
pub use vm::scheduler::VmExecutionPlan;

#[doc(hidden)]
pub mod passes {
    pub use crate::hpu::allocator::allocate_registers as hpu_allocate_registers;
    pub use crate::hpu::lowering::lower_iop_to_hpu;
    pub use crate::hpu::scheduler::legacy::{
        batch as hpu_batch_legacy, schedule as hpu_schedule_legacy,
        schedule_batched as hpu_schedule_batched_legacy,
    };
    pub use crate::hpu::scheduler::regular::schedule as hpu_schedule;
    pub use crate::hpu::translation_table::decode_translation_table as hpu_decode_translation_table;
    pub use crate::hpu::translation_table::generate_translation_table as hpu_generate_translation_table;
}
