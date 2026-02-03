//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use allocator::allocate_registers;
use batcher::batch;
use hc_builder::builder::CiphertextSpec;
use hc_builder::iops::cmp::{cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq};
use hc_builder::iops::if_then_else::if_then_else;
use hc_builder::iops::if_then_zero::if_then_zero;
use hc_ir::translation::Translator;
use scheduler::schedule;
use translation::IoplangToHpulang;
use translation_table::{DOpRepr, generate_translation_table};

pub mod allocator;
pub mod interpreter;
pub mod batcher;
pub mod latency;
pub mod scheduler;
pub mod translation;
pub mod translation_table;

pub use hc_sim::hpu::HpuConfig;
pub use hc_sim::{Cycle, MHz};

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

fn pipeline(hpu_config: &HpuConfig, spec: CiphertextSpec, iop: Iop) -> Vec<DOpRepr> {
    let ir = match iop {
        Iop::CmpGt => cmp_gt(spec),
        Iop::CmpGte => cmp_gte(spec),
        Iop::CmpLt => cmp_lt(spec),
        Iop::CmpLte => cmp_lte(spec),
        Iop::CmpEq => cmp_eq(spec),
        Iop::CmpNeq => cmp_neq(spec),
        Iop::IfThenElse => if_then_else(spec),
        Iop::IfThenZero => if_then_zero(spec),
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
    spec: CiphertextSpec,
    iop: Iop,
) -> Vec<DOpRepr> {
    pipeline(hpu_config, spec, iop)
}

#[cfg(test)]
mod test;
