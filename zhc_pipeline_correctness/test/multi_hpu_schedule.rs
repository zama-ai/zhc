use zhc::compat::Iop;
use zhc_builder::CiphertextSpec;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::IR;
use zhc_langs::{
    hpulang::{HpuInstructionSet, HpuLang},
    ioplang::IopLang,
};
use zhc_pipeline::{
    PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy,
    passes::{lower_iop_to_multi_hpu, multi_hpu_schedule, partition_and_materialize},
};
use zhc_pipeline_correctness::equivalence_check::check_iop_multi_hpu_equivalence;
use zhc_pipeline_correctness_macro::test_matrix;
use zhc_utils::units::Cycle;

fn partitioner_config(
    n_hpus: usize,
    schedp: SchedPolicy,
    tiep: TiePolicy,
    spreadp: SpreadPolicy,
) -> PartitionerConfig {
    PartitionerConfig {
        specs: std::iter::repeat(PartitionSpec {
            parallelism: 12,
            latency: Cycle(1),
        })
        .take(n_hpus)
        .collect(),
        sched_policy: schedp,
        tie_policy: tiep,
        spread_policy: spreadp,
    }
}

fn pipeline(ir: &IR<IopLang>, config: &PartitionerConfig) -> Vec<IR<HpuLang>> {
    let (remat, partitions) = partition_and_materialize(ir, config);
    let (hpu_ir, localities) = lower_iop_to_multi_hpu(&remat, &partitions);
    let multi_config = MultiHpuConfig {
        n_hpus: config.specs.len() as u8,
        ..MultiHpuConfig::default()
    };
    multi_hpu_schedule(&hpu_ir, &localities, &multi_config, config.sched_policy)
}

fn count_transfers(hpu_irs: &[IR<HpuLang>]) -> (usize, usize) {
    let mut ins = 0;
    let mut outs = 0;
    for ir in hpu_irs {
        for op in ir.walk_ops_linear() {
            match op.get_instruction() {
                HpuInstructionSet::TransferIn { .. } => ins += 1,
                HpuInstructionSet::TransferOut { .. } => outs += 1,
                _ => {}
            }
        }
    }
    (ins, outs)
}

#[test]
fn smoke() {
    let spec = CiphertextSpec::new(8, 2, 2);
    let iop_ir = Iop::Add.to_builder(spec).optimize_ir();
    let config = partitioner_config(
        2,
        SchedPolicy::AsSoonAsPossible,
        TiePolicy::Balance,
        SpreadPolicy::FavorBandwidth,
    );
    let hpu_irs = pipeline(&iop_ir, &config);
    let (ins, outs) = count_transfers(&hpu_irs);
    assert!(
        ins > 0,
        "The schedule has no transfer; the test is vacuous."
    );
    assert_eq!(ins, outs, "Unbalanced transfers");
    check_iop_multi_hpu_equivalence(&iop_ir, &hpu_irs, spec.block_spec(), 10);
}

#[test_matrix(
    iop = @all_iops,
    size = @main_precs,
    schedp = @all_schedps,
    tiep = [tiep_concentrate = TiePolicy::Concentrate, tiep_balance = TiePolicy::Balance],
    spreadp = [spreadp_latency = SpreadPolicy::FavorLatency, spreadp_bandwidth = SpreadPolicy::FavorBandwidth],
    n_hpus = [m2 = 2, m4 = 4, m8 = 8])
]
#[ignore]
fn correctness(
    iop: Iop,
    size: u16,
    schedp: SchedPolicy,
    tiep: TiePolicy,
    spreadp: SpreadPolicy,
    n_hpus: usize,
) {
    let spec = CiphertextSpec::new(size, 2, 2);
    let iop_ir = iop.to_builder(spec).optimize_ir();
    let config = partitioner_config(n_hpus, schedp, tiep, spreadp);
    let hpu_irs = pipeline(&iop_ir, &config);
    check_iop_multi_hpu_equivalence(&iop_ir, &hpu_irs, spec.block_spec(), 100);
}
