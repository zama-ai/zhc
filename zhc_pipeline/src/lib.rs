//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use allocator::allocate_registers;
use std::f64;
use std::path::Path;
use zhc_builder::{Builder, CiphertextSpec, PartitionId, mh_mul};
use zhc_ir::IR;
use zhc_ir::cse::eliminate_common_subexpressions;
use zhc_ir::dce::eliminate_dead_code;
use zhc_langs::doplang::{DopLang, emit_assembly};
use zhc_langs::hpulang::{HpuLang, get_batch_statistics};
use zhc_langs::ioplang::{
    IopInstructionSet, IopLang, cut_transfers, eliminate_aliases, insert_transfers,
    isolate_subgraphs, skip_redundant_stores, skip_store_load,
};
use zhc_sim::MHz;
use zhc_sim::hpu::{HpuConfig, PhysicalConfig};
use zhc_utils::Dumpable;

use crate::scheduler::{SchedPolicy, one_step};

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

#[allow(unused)]
fn multi_hpu_regular_pipeline(mut ir: IR<IopLang>, config: &HpuConfig) -> Vec<IR<DopLang>> {
    cut_transfers(&mut ir);
    let components = isolate_subgraphs(&ir, |op| {
        use IopInstructionSet::*;
        match op {
            InputCiphertext { .. }
            | InputPlaintext { .. }
            | ExtractCtBlock { .. }
            | ExtractPtBlock { .. }
            | DeclareCiphertext { .. }
            | LetCiphertextBlock { .. }
            | LetPlaintextBlock { .. } => true,
            _ => false,
        }
    });
    components
        .into_iter()
        .map(|ir| regular_pipeline(ir, config).1)
        .collect()
}

#[cfg(test)]
mod test;

#[test]
fn pipeline_mh_mul() {
    const INT_SIZE: u16 = 16;
    const MH_FACTOR: u8 = 4;

    let mut hpu_config = HpuConfig::from(PhysicalConfig::tuniform_64b_pfail128_psi64());
    hpu_config.pbs_min_batch_size = 12;
    let builder = mh_mul(CiphertextSpec::new(INT_SIZE, 2, 2), MH_FACTOR);

    builder.draw("mh_mul_ir.html");
    builder.draw_partitions("mh_mul_ir_raw_part.html");

    // Hpu 0
    builder.merge_partition_group(
        &[0, 17, 18, 19, 21]
            .iter()
            .map(|x| PartitionId(*x))
            .collect::<Vec<_>>(),
    );
    builder.merge_partition_group(
        &[2, 9, 10, 12, 15]
            .iter()
            .map(|x| PartitionId(*x))
            .collect::<Vec<_>>(),
    );
    builder.merge_partition_group(
        &[5, 6, 7, 16, 20]
            .iter()
            .map(|x| PartitionId(*x))
            .collect::<Vec<_>>(),
    );
    builder.merge_partition_group(
        &[3, 4, 13, 14, 8, 11]
            .iter()
            .map(|x| PartitionId(*x))
            .collect::<Vec<_>>(),
    );
    builder.draw_partitions("mh_mul_ir_grp_part.html");

    let mut ir = builder.ir().clone();
    insert_transfers(&mut ir, builder.partitions());
    cut_transfers(&mut ir);
    // ir.dump_and_wait();

    let components = isolate_subgraphs(&ir, |op| {
        use IopInstructionSet::*;
        match op {
            InputCiphertext { .. }
            | InputPlaintext { .. }
            | ExtractCtBlock { .. }
            | ExtractPtBlock { .. }
            | DeclareCiphertext { .. }
            | LetCiphertextBlock { .. }
            | LetPlaintextBlock { .. } => true,
            _ => false,
        }
    });

    for (i, mut comp) in components.into_iter().rev().enumerate() {
        eliminate_aliases(&mut comp);
        skip_store_load(&mut comp);
        eliminate_dead_code(&mut comp);
        skip_redundant_stores(&mut comp);
        eliminate_dead_code(&mut comp);
        eliminate_common_subexpressions(&mut comp);
        eliminate_dead_code(&mut comp);

        let unscheduled = translation::lower_iop_to_hpu(&comp);
        let scheduled =
            one_step::schedule(&unscheduled, &hpu_config, SchedPolicy::AsLateAsPossible);
        let allocated = allocate_registers(&scheduled, &hpu_config);
        use std::fs::File;
        use std::io::Write;
        let filename = format!("mhmul_hid{}.asm", i);
        let mut file = File::create(&filename).expect("Failed to create .asm file");
        file.write_all(emit_assembly(&allocated).as_bytes())
            .expect("Failed to write to .asm file");
    }
}
