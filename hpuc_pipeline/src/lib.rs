//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use allocator::allocate_registers;
use batcher::batch;
pub use hpuc_builder::builder::BlockConfig;
use hpuc_builder::iops::cmp::{cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq};
use hpuc_builder::iops::if_then_else::if_then_else;
use hpuc_builder::iops::if_then_zero::if_then_zero;
use hpuc_ir::translation::Translator;
use scheduler::schedule;
use translation::IoplangToHpulang;
use translation_table::{DOpRepr, generate_translation_table};

pub mod allocator;
pub mod batcher;
pub mod latency;
pub mod scheduler;
pub mod translation;
pub mod translation_table;

pub use hpuc_sim::hpu::HpuConfig;
pub use hpuc_sim::{Cycle, MHz};

/// Iops supported by the pipeline.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Iop {
    CmpGt,
    CmpGte,
    CmpLt,
    CmpLte,
    CmpEq,
    CmpNeq,
    IfThenElse,
    IfThenZero,
}

fn pipeline(
    hpu_config: &HpuConfig,
    integer_width: u8,
    block_config: &BlockConfig,
    iop: Iop,
) -> Vec<DOpRepr> {
    let ir = match iop {
        Iop::CmpGt => cmp_gt(integer_width, block_config),
        Iop::CmpGte => cmp_gte(integer_width, block_config),
        Iop::CmpLt => cmp_lt(integer_width, block_config),
        Iop::CmpLte => cmp_lte(integer_width, block_config),
        Iop::CmpEq => cmp_eq(integer_width, block_config),
        Iop::CmpNeq => cmp_neq(integer_width, block_config),
        Iop::IfThenElse => if_then_else(integer_width, block_config),
        Iop::IfThenZero => if_then_zero(integer_width, block_config),
    };
    let unscheduled = IoplangToHpulang.translate(&ir);
    let scheduled = schedule(&unscheduled, hpu_config);
    let batched = batch(&scheduled);
    let allocated = allocate_registers(&batched, &hpu_config);
    generate_translation_table(&allocated)
}

/// Generates a translation table for the specified operation configuration.
///
/// Takes the HPU hardware configuration in `hpu_config`, integer arithmetic
/// configuration in `integer_config`, and the desired operation `iop` to
/// produce an hex stream.
pub fn get_translation_table(
    hpu_config: &HpuConfig,
    integer_width: u8,
    block_config: &BlockConfig,
    iop: Iop,
) -> Vec<DOpRepr> {
    pipeline(hpu_config, integer_width, block_config, iop)
}

#[cfg(test)]
mod test;
