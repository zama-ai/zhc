//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use allocator::allocate_registers;
use translation_table::{DOpRepr, generate_translation_table};
use zhc_builder::Builder;
use zhc_builder::if_then_else;
use zhc_builder::if_then_zero;
use zhc_builder::{cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq};
use zhc_ir::IR;
use zhc_ir::cse::eliminate_common_subexpressions;
use zhc_ir::dce::eliminate_dead_code;
use zhc_langs::doplang::DopLang;
use zhc_langs::ioplang::IopLang;
use zhc_langs::ioplang::eliminate_aliases;

pub mod allocator;
pub mod batch_scheduler;
pub mod interpreter;
pub mod latency;
pub mod translation;
pub mod translation_table;

pub use zhc_sim::hpu::HpuConfig;
pub use zhc_sim::{Cycle, MHz};

pub fn compute_latency(builder: &Builder, config: HpuConfig, freq: MHz) -> f64 {
    let ir = builder.ir().to_owned();
    let allocated = regular_pipeline(ir, &config);
    latency::compute_latency(&allocated, &config).as_ts(freq.period())
}

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

pub use zhc_builder::CiphertextSpec;

use crate::batch_scheduler::batch_schedule;
use crate::translation::lower_iop_to_hpu;

fn regular_pipeline(mut ir: IR<IopLang>, config: &HpuConfig) -> IR<DopLang> {
    eliminate_aliases(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    let unscheduled = lower_iop_to_hpu(&ir);
    let batched = batch_schedule(&unscheduled, config);
    allocate_registers(&batched, config)
}

fn pipeline(hpu_config: &HpuConfig, spec: CiphertextSpec, iop: Iop) -> Vec<DOpRepr> {
    let ir = match iop {
        Iop::CmpGt => cmp_gt(spec).into_ir(),
        Iop::CmpGte => cmp_gte(spec).into_ir(),
        Iop::CmpLt => cmp_lt(spec).into_ir(),
        Iop::CmpLte => cmp_lte(spec).into_ir(),
        Iop::CmpEq => cmp_eq(spec).into_ir(),
        Iop::CmpNeq => cmp_neq(spec).into_ir(),
        Iop::IfThenElse => if_then_else(spec).into_ir(),
        Iop::IfThenZero => if_then_zero(spec).into_ir(),
    };
    let allocated = regular_pipeline(ir, hpu_config);
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
