//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use allocator::allocate_registers;
use std::f64;
use std::path::Path;
use zhc_builder::Builder;
use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_langs::hpulang::{HpuLang, get_batch_statistics};
use zhc_langs::ioplang::IopLang;
use zhc_sim::MHz;
use zhc_sim::hpu::HpuConfig;

use crate::scheduler::SchedPolicy;

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
    let unscheduled = translation::lower_iop_to_hpu(&ir).output;
    let scheduled =
        scheduler::one_step::schedule(&unscheduled, config, SchedPolicy::AsLateAsPossible);
    let allocated = allocate_registers(&scheduled, config);
    (scheduled, allocated)
}

pub fn alternative_pipeline(ir: IR<IopLang>, config: &HpuConfig) -> (IR<HpuLang>, IR<DopLang>) {
    let unscheduled = translation::lower_iop_to_hpu(&ir).output;
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

#[cfg(test)]
mod test_mh {
    use crate::{
        allocator::allocate_registers,
        scheduler::{SchedPolicy, one_step_mh},
        translation::lower_iop_to_multi_hpu,
    };
    use zhc_builder::{Builder, CiphertextSpec, mh_mul};
    use zhc_ir::{AnnIR, partition::PartitionId};
    use zhc_langs::doplang::emit_assembly;
    use zhc_sim::{
        Simulator,
        hpu::{DOp, DOpId, MultiHpuConfig},
        multi_hpu::{Events, MultiHpu},
    };

    #[test]
    fn pipeline_mh_dbg() {
        // Configuration
        const INT_SIZE: u16 = 8;
        const _MH_FACTOR: u8 = 4;

        let config = MultiHpuConfig::default();

        // Build simple circuit
        // (ra, rb) <= f(a,b,c,d)
        // ra <= (a + b) - (c + d)
        // rb <= (a + b) + (c + d)
        fn mh_dbg(spec: CiphertextSpec) -> Builder {
            let builder = Builder::new(spec.block_spec());
            // Define inputs
            let src_a = builder.ciphertext_input(spec.int_size());
            let src_b = builder.ciphertext_input(spec.int_size());
            let src_c = builder.ciphertext_input(spec.int_size());
            let src_d = builder.ciphertext_input(spec.int_size());

            // extract inputs as array of blk
            let src_a_blocks = builder.ciphertext_split(&src_a);
            let src_b_blocks = builder.ciphertext_split(&src_b);
            let src_c_blocks = builder.ciphertext_split(&src_c);
            let src_d_blocks = builder.ciphertext_split(&src_d);

            // Partition A
            let cur_partition = builder.new_partition();
            println!("Partition A: (a+b) => {cur_partition:?}");
            let (apb, _) = builder.comment(format!("apb")).iop_add_raw(
                spec.int_size(),
                src_a_blocks,
                src_b_blocks,
                None,
            );

            // Partition B
            let cur_partition = builder.new_partition();
            println!("Partition B: (c+d) => {cur_partition:?}");
            let (cpd, _) = builder.comment(format!("cpd")).iop_add_raw(
                spec.int_size(),
                src_c_blocks,
                src_d_blocks,
                None,
            );

            // Partition C
            let cur_partition = builder.new_partition();
            println!("Partition C: A + B => {cur_partition:?}");
            let (ap_b, _) =
                builder
                    .comment(format!("ApB"))
                    .iop_add_raw(spec.int_size(), &apb, &cpd, None);

            // Partition D
            // Output on hpu A
            let cur_partition = builder.new_partition();
            println!("Partition D: Output hpu_A => {cur_partition:?}");
            builder.ciphertext_output(builder.ciphertext_join(ap_b, Some(spec.int_size())));

            // // Partition E
            // // NB: Tricky part reintroduce ciphertext_join
            // // WARN: Not supported yet, must rely on _raw version of iop
            // let cur_partition = builder.new_partition();
            // println!("Partition E: A - B => {cur_partition:?}");
            // let AmB = builder.comment(format!("A&B")).iop_sub(
            //     &builder.ciphertext_join(apb, Some(spec.int_size())),
            //     &builder.ciphertext_join(cpd, Some(spec.int_size())),
            // );

            // // Partition F
            // // Output on hpu B
            // let cur_partition = builder.new_partition();
            // println!("Partition E: Output hpu_B => {cur_partition:?}");
            // builder.ciphertext_output(AmB);

            builder
        }

        let builder = mh_dbg(CiphertextSpec::new(INT_SIZE, 2, 2));

        builder.draw("mh_dbg_ir.html");
        builder.draw_partitions("mh_dbg_ir_raw_part.html");

        // Hpu 0
        builder.merge_partition_group(
            &[0, 1, 3, 4]
                .iter()
                .map(|x| PartitionId(*x))
                .collect::<Vec<_>>(),
        );
        builder.merge_partition_group(
            // &[2, 5, 6]
            &[2].iter().map(|x| PartitionId(*x)).collect::<Vec<_>>(),
        );
        builder.draw_partitions("mh_dbg_ir_grp_part.html");

        let partitions = builder.partitions();
        let ir = builder.optimize_ir();

        let (mhir, localities) = lower_iop_to_multi_hpu(&ir, &partitions);

        AnnIR::new(&mhir, localities.clone(), mhir.filled_valmap(()))
            .draw_ann_to_html(None, "hfdsah.html");

        let scheds =
            one_step_mh::schedule(&mhir, localities, &config, SchedPolicy::AsLateAsPossible);

        let mut streams = Vec::new();
        for scheduled in scheds.into_iter() {
            let allocated = allocate_registers(&scheduled, &config.hpu_config);
            let dops: Vec<DOp> = allocated
                .walk_ops_linear()
                .map(|a| DOp {
                    raw: a.get_instruction(),
                    id: DOpId(a.get_id().into()),
                })
                .collect();
            streams.push(dops);
        }

        let mut simulator = Simulator::from_simulatable(
            config.hpu_config.freq,
            MultiHpu::new(&config),
            zhc_sim::TracingLevel::Events,
        );
        let event = zhc_sim::multi_hpu::Events::PushDOps(streams);
        simulator.dispatch(event);
        simulator.play_until_event(Events::ProcessOver);
        simulator.dump_trace("mh_dbg_trace.json");
    }

    #[test]
    fn pipeline_mh_mul() {
        const INT_SIZE: u16 = 16;
        const MH_FACTOR: u8 = 4;
        const DEBUG: bool = true;
        const DEBUG_SIM: bool = false;

        let config = MultiHpuConfig {
            n_hpus: 5,
            ..Default::default()
        };
        let builder = mh_mul(CiphertextSpec::new(INT_SIZE, 2, 2), MH_FACTOR);

        if DEBUG {
            builder.draw("mh_mul_ir.html");
            builder.draw_partitions_optim("mh_mul_ir_raw_part.html");
        }
        let ir = builder.optimize_ir().clone();

        // Hpu 0
        builder.merge_partition_group(
            &[0, 1, 17, 18, 19, 21]
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
        if DEBUG {
            builder.draw_partitions_optim("mh_mul_ir_grp_part.html");
        }

        let partitions = builder.partitions();

        let (mhir, localities) = lower_iop_to_multi_hpu(&ir, &partitions);

        if DEBUG {
            AnnIR::new(&mhir, localities.clone(), mhir.filled_valmap(()))
                .draw_ann_to_html(None, "mh_mul_ir_flat.html");
        }

        let scheds =
            one_step_mh::schedule(&mhir, localities, &config, SchedPolicy::AsLateAsPossible);

        let mut streams = Vec::new();
        for (hid, scheduled) in scheds.into_iter().enumerate() {
            let allocated = allocate_registers(&scheduled, &config.hpu_config);
            let dops: Vec<DOp> = allocated
                .walk_ops_linear()
                .map(|a| DOp {
                    raw: a.get_instruction(),
                    id: DOpId(a.get_id().into()),
                })
                .collect();

            // TODO clean this part
            use std::fs::File;
            use std::io::Write;
            let filename = format!("mhmul{INT_SIZE}f{MH_FACTOR}_v{hid}.asm");
            let mut file = File::create(&filename).expect("Failed to create .asm file");
            file.write_all(emit_assembly(&allocated).as_bytes())
                .expect("Failed to write to .asm file");

            streams.push(dops);
        }

        if DEBUG_SIM {
            let mut simulator = Simulator::from_simulatable(
                config.hpu_config.freq,
                MultiHpu::new(&config),
                // zhc_sim::TracingLevel::Events,
                zhc_sim::TracingLevel::Load,
            );
            let event = zhc_sim::multi_hpu::Events::PushDOps(streams);
            simulator.dispatch(event);
            simulator.play_until_event(Events::ProcessOver);
            simulator.dump_trace("mh_mul_trace.json");
        }
    }
}
