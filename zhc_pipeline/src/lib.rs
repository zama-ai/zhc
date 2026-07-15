//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use crate::scheduler::SchedPolicy;
use crate::scheduler::vm::VmExecutionPlan;
use allocator::allocate_registers;
use zhc_ir::cse::eliminate_common_subexpressions;

use std::f64;
use std::path::Path;
use zhc_builder::Builder;
use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_langs::hpulang::{HpuLang, get_batch_statistics};
use zhc_langs::ioplang::IopLang;
use zhc_sim::MHz;
use zhc_sim::hpu::HpuConfig;

pub mod allocator;
pub mod compat;
pub mod draw_slack;
pub mod gpu_metrics;
pub mod hpu_metrics;
pub mod latency;
pub mod pbs_metrics;
pub mod scheduler;
pub mod tracing;
pub mod translation;
pub mod translation_table;

/// Computes HPU-level performance metrics for a circuit.
///
/// Runs the full compilation pipeline and simulates execution to collect timing
/// and batching statistics. Uses default HPU configuration.
pub fn compute_hpu_metrics(builder: &Builder) -> hpu_metrics::HpuMetrics {
    let ir = builder.optimize_ir();
    let (scheduled, allocated) = regular_pipeline(ir, &HpuConfig::default());
    hpu_metrics::compute_hpu_metrics(&allocated, &scheduled)
}

/// Computes GPU-level performance metrics for a circuit.
///
/// Returns batch statistics.
pub fn compute_gpu_metrics(
    builder: &Builder,
    optimal_batch_size: usize,
) -> gpu_metrics::GpuMetrics {
    let ir = builder.optimize_ir();
    let mut config = HpuConfig::default();
    config.pbs_min_batch_size = optimal_batch_size;
    config.pbs_max_batch_size = optimal_batch_size;
    let (scheduled, _) = regular_pipeline(ir, &config);
    let stats = get_batch_statistics(&scheduled);
    gpu_metrics::GpuMetrics {
        batch_stats: stats,
        ir: scheduled,
    }
}

/// Computes PBS-level metrics for a circuit.
///
/// Analyzes the optimized IOP-level IR to compute PBS count, critical path length,
/// and slack distribution.
pub fn compute_pbs_metrics(builder: &Builder) -> pbs_metrics::PbsMetrics {
    let ir = builder.optimize_ir();
    pbs_metrics::compute_pbs_metrics(&ir)
}

/// Traces the execution of a computation graph to a perfetto file.
///
/// This function runs the full compilation pipeline on the provided IR and
/// generates an execution trace showing how operations execute on the HPU.
/// The trace is written to the specified path and can be opened in perfetto.
pub fn trace_execution(builder: &Builder, config: HpuConfig, path: impl AsRef<Path>) {
    let ir = builder.optimize_ir();
    let (_, allocated) = regular_pipeline(ir, &config);
    tracing::trace_execution(&allocated, &config, path);
}

/// Computes the estimated latency of a computation graph.
///
/// This function runs the full compilation pipeline and calculates the total
/// execution time in seconds based on the HPU configuration and clock frequency.
/// Returns the latency as a floating-point number of micro-seconds.
pub fn compute_latency(builder: &Builder, config: HpuConfig, freq: MHz) -> f64 {
    let ir = builder.optimize_ir();
    let (_, allocated) = regular_pipeline(ir, &config);
    latency::compute_latency(&allocated, &config)
        .0
        .as_ts(freq.period())
}

pub fn draw_slack(builder: &Builder, path: impl AsRef<Path>) {
    draw_slack::draw_slack(builder, path);
}

pub fn regular_pipeline(ir: IR<IopLang>, config: &HpuConfig) -> (IR<HpuLang>, IR<DopLang>) {
    let unscheduled = translation::lower_iop_to_hpu(&ir);
    let scheduled =
        scheduler::one_step::schedule(&unscheduled, config, SchedPolicy::AsLateAsPossible);
    let allocated = allocate_registers(&scheduled, config);
    (scheduled, allocated)
}

pub fn vm_pipeline(ir: IR<IopLang>, n_threads: u8) -> VmExecutionPlan {
    let mut vmir = translation::lower_iop_to_vm(&ir);
    eliminate_common_subexpressions(&mut vmir);
    scheduler::vm::schedule(&vmir, n_threads, SchedPolicy::AsSoonAsPossible)
}

pub fn alternative_pipeline(ir: IR<IopLang>, config: &HpuConfig) -> (IR<HpuLang>, IR<DopLang>) {
    let unscheduled = translation::lower_iop_to_hpu(&ir);
    let scheduled = scheduler::two_step::schedule(
        &unscheduled,
        config,
        SchedPolicy::AsLateAsPossible,
        SchedPolicy::AsSoonAsPossible,
    );
    let allocated = allocate_registers(&scheduled, config);
    (scheduled, allocated)
}

#[cfg(test)]
mod test;
