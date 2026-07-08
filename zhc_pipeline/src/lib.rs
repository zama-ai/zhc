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
    use zhc_langs::doplang::emit_assembly;
    use zhc_sim::{
        Simulator,
        hpu::{DOp, DOpId, MultiHpuConfig},
        multi_hpu::{Events, MultiHpu},
    };
    use zhc_utils::Dumpable;

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
            builder.new_partition("A: (a+b)");
            let (apb, _) = builder.comment(format!("apb")).iop_add_raw(
                spec.int_size(),
                src_a_blocks,
                src_b_blocks,
                None,
            );

            // Partition B
            builder.new_partition("B: (c+d)");
            let (cpd, _) = builder.comment(format!("cpd")).iop_add_raw(
                spec.int_size(),
                src_c_blocks,
                src_d_blocks,
                None,
            );

            // Partition C
            builder.new_partition("C: A+B");
            let (ap_b, _) =
                builder
                    .comment(format!("ApB"))
                    .iop_add_raw(spec.int_size(), &apb, &cpd, None);

            // Partition D
            // Output on hpu A
            builder.new_partition("D: Out_A");
            builder.ciphertext_output(builder.ciphertext_join(ap_b, Some(spec.int_size())));

            // // Partition E
            // // NB: Tricky part reintroduce ciphertext_join
            // // WARN: Not supported yet, must rely on _raw version of iop
            // builder.new_partition("E: A - B");
            // let AmB = builder.comment(format!("A&B")).iop_sub(
            //     &builder.ciphertext_join(apb, Some(spec.int_size())),
            //     &builder.ciphertext_join(cpd, Some(spec.int_size())),
            // );

            // // Partition F
            // // Output on hpu B
            // builder.new_partition("E: Out_B");
            // builder.ciphertext_output(AmB);

            builder
        }

        let builder = mh_dbg(CiphertextSpec::new(INT_SIZE, 2, 2));

        // Draw unoptimized ir without partitions
        builder.draw("mh_dbg_ir_raw.html");

        // Draw optimized ir with raw partitions (no manual grouping)
        let ir = builder.optimize_ir();
        builder.draw_partitions(&ir, "mh_dbg_ir_part.html");

        // Group partitions
        // Hpu 0
        builder.group_partitions_id(&[0, 1, 3, 4]);
        builder.group_partitions_id(&[2]);
        builder.draw_partitions(&ir, "mh_dbg_ir_part_grp.html");

        let partitions = builder.partitions(&ir);
        let (mhir, localities) = lower_iop_to_multi_hpu(&ir, &partitions);

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
        const INT_SIZE: u16 = 64;
        const MH_FACTOR: u8 = 8;
        const DEBUG: bool = false;
        const DEBUG_SIM: bool = true;

        let ct_spec = CiphertextSpec::new(INT_SIZE, 2, 2);
        let schoolbook_depth = std::cmp::max(2, MH_FACTOR / 2) as usize;

        let config = MultiHpuConfig {
            n_hpus: MH_FACTOR,
            ..Default::default()
        };
        let builder = mh_mul(ct_spec, schoolbook_depth);

        if DEBUG {
            let ir = builder.ir();
            println!("Dump RAW partition table");
            builder.partitions_table(&ir).dump_and_wait();
            builder.draw("mh_mul_ir_raw.html");
        }

        let ir = builder.optimize_ir().clone();

        println!("Dump partition table");
        builder.partitions_table(&ir).dump();
        if DEBUG {
            builder.draw_partitions(&ir, "mh_mul_ir_part.html");
        }

        // Partition gathering is currently a hand-made process
        // NB: dummy part (i.e. inputs/outputs) are added to first grp
        match MH_FACTOR {
            2 => {
                builder.group_partitions_id(&[1, 2, 7, 0, 9]);
                builder.group_partitions_id(&[3, 4, 6, 7]);
            }
            4 => {
                builder.group_partitions_id(&[4, 7, 0, 9]);
                builder.group_partitions_id(&[2]);

                builder.group_partitions_id(&[3, 6]);

                builder.group_partitions_id(&[1]);
            }
            8 => {
                builder.group_partitions_id(&[1, 8, 24, 0, 36]);
                builder.group_partitions_id(&[2, 3, 23]);
                builder.group_partitions_id(&[4, 5, 25, 27, 28]);
                builder.group_partitions_id(&[9, 10, 26]);
                builder.group_partitions_id(&[14, 19, 33, 34]);
                builder.group_partitions_id(&[15, 16, 31]);
                builder.group_partitions_id(&[11, 12, 30, 32]);
                builder.group_partitions_id(&[6, 7, 29]);
            }
            _ => {
                panic!(
                    "MH_FACTOR {MH_FACTOR} is out-of-range. Frogs only contains up to 8
        nodes"
                );
            }
        }
        println!("Dump group partition table");
        builder.partitions_table(&ir).dump();

        if DEBUG {
            builder.draw_partitions(&ir, "mh_mul_ir_part_grp.html");
        }

        let partitions = builder.partitions(&ir);
        let (mhir, localities) = lower_iop_to_multi_hpu(&ir, &partitions);

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
            simulator.now_us().dump_and_wait();
            // simulator.dump_trace("mh_mul_trace.json");
        }
    }
}
