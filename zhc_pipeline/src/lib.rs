//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use std::path::Path;

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
pub mod batcher;
pub mod interpreter;
pub mod latency;
pub mod statistics;
pub mod tracing;
pub mod translation;
pub mod translation_table;

pub use zhc_builder::CiphertextSpec;
pub use zhc_sim::hpu::HpuConfig;
pub use zhc_sim::{Cycle, MHz};

/// Traces the execution of a computation graph to a perfetto file.
///
/// This function runs the full compilation pipeline on the provided IR and
/// generates an execution trace showing how operations execute on the HPU.
/// The trace is written to the specified path and can be opened in perfetto.
pub fn trace_execution(builder: &Builder, config: HpuConfig, path: impl AsRef<Path>) {
    let ir = builder.ir().to_owned();
    let allocated = regular_pipeline(ir, &config);
    tracing::trace_execution(&allocated, &config, path);
}

/// Computes the estimated latency of a computation graph.
///
/// This function runs the full compilation pipeline and calculates the total
/// execution time in seconds based on the HPU configuration and clock frequency.
/// Returns the latency as a floating-point number of micro-seconds.
pub fn compute_latency(builder: &Builder, config: HpuConfig, freq: MHz) -> f64 {
    let ir = builder.ir().to_owned();
    let allocated = regular_pipeline(ir, &config);
    latency::compute_latency(&allocated, &config).as_ts(freq.period())
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

fn regular_pipeline(mut ir: IR<IopLang>, config: &HpuConfig) -> IR<DopLang> {
    eliminate_aliases(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    let unscheduled = translation::lower_iop_to_hpu(&ir);
    let batched = batcher::batch(&unscheduled, config);
    let scheduled = batch_scheduler::schedule(&batched, config);
    allocate_registers(&scheduled, config)
}

#[cfg(test)]
mod test;

#[test]
fn test_dump_trace() {
    let bd = zhc_builder::mul_lsb(CiphertextSpec::new(64, 2, 2));
    trace_execution(
        &bd,
        HpuConfig::from(zhc_sim::hpu::PhysicalConfig::tuniform_64b_pfail128_psi64()),
        "test.json",
    );
}
