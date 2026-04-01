//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use std::f64;
use std::path::Path;

use allocator::allocate_registers;
use zhc_builder::Builder;
use zhc_ir::IR;
use zhc_ir::cse::eliminate_common_subexpressions;
use zhc_ir::dce::eliminate_dead_code;
use zhc_langs::doplang::DopLang;
use zhc_langs::hpulang::get_batch_statistics;
use zhc_langs::ioplang::IopLang;
use zhc_langs::ioplang::eliminate_aliases;
use zhc_sim::MHz;
use zhc_sim::hpu::HpuConfig;

pub mod allocator;
pub mod batch_scheduler;
pub mod batcher;
pub mod compat;
pub mod gpu_metrics;
pub mod hpu_metrics;
pub mod latency;
pub mod pbs_metrics;
pub mod tracing;
pub mod translation;
pub mod translation_table;

/// Computes HPU-level performance metrics for a circuit.
///
/// Runs the full compilation pipeline and simulates execution to collect timing
/// and batching statistics. Uses default HPU configuration.
pub fn compute_hpu_metrics(builder: &Builder) -> hpu_metrics::HpuMetrics {
    let mut ir = builder.ir().to_owned();
    eliminate_aliases(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    let unscheduled = translation::lower_iop_to_hpu(&ir);
    let batched = batcher::batch(&unscheduled, &HpuConfig::default());
    let scheduled = batch_scheduler::schedule(&batched, &HpuConfig::default());
    let allocated = allocate_registers(&scheduled, &HpuConfig::default());
    hpu_metrics::compute_hpu_metrics(&allocated, &batched)
}

/// Computes GPU-level performance metrics for a circuit.
///
/// Returns batch statistics.
pub fn compute_gpu_metrics(
    builder: &Builder,
    optimal_batch_size: usize,
) -> gpu_metrics::GpuMetrics {
    let mut ir = builder.ir().to_owned();
    eliminate_aliases(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    let unscheduled = translation::lower_iop_to_hpu(&ir);
    let mut config = HpuConfig::default();
    config.pbs_min_batch_size = optimal_batch_size;
    config.pbs_max_batch_size = optimal_batch_size;
    let batched = batcher::batch(&unscheduled, &config);
    let stats = get_batch_statistics(&batched);
    gpu_metrics::GpuMetrics {
        batch_stats: stats,
        ir: batched,
    }
}

/// Computes PBS-level metrics for a circuit.
///
/// Analyzes the optimized IOP-level IR to compute PBS count, critical path length,
/// and slack distribution.
pub fn compute_pbs_metrics(builder: &Builder) -> pbs_metrics::PbsMetrics {
    let mut ir = builder.ir().to_owned();
    eliminate_aliases(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    pbs_metrics::compute_pbs_metrics(&ir)
}

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
    latency::compute_latency(&allocated, &config)
        .0
        .as_ts(freq.period())
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
