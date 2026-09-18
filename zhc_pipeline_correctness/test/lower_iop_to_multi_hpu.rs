use zhc::compat::Iop;
use zhc_builder::CiphertextSpec;
use zhc_ir::IR;
use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
use zhc_pipeline::{
    PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy,
    partition_and_materialize,
};
use zhc_pipeline_correctness_macro::test_matrix;
use zhc_utils::units::Cycle;

use zhc_pipeline_correctness::equivalence_check::check_iop_hpu_equivalence;

fn pipeline(ir: &IR<IopLang>, config: &PartitionerConfig) -> IR<HpuLang> {
    let (remat, partitions) = partition_and_materialize(ir, config);
    zhc_pipeline::passes::lower_iop_to_multi_hpu(&remat, &partitions).0
}

#[test]
fn smoke() {
    let iop = Iop::TrailingZeros;
    let size = 8;
    let schedp = SchedPolicy::AsSoonAsPossible;
    let tiep = TiePolicy::Balance;
    let spreadp = SpreadPolicy::FavorBandwidth;
    let n_hpus = 8;
    let b = iop.to_builder(CiphertextSpec::new(size, 2, 2));
    let iop_ir = b.optimize_ir();
    let config = PartitionerConfig {
        specs: std::iter::repeat(PartitionSpec {
            parallelism: 12,
            latency: Cycle(1),
        })
        .take(n_hpus)
        .collect(),
        sched_policy: schedp,
        tie_policy: tiep,
        spread_policy: spreadp,
    };
    let _hpu_ir = pipeline(&iop_ir, &config);
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
    let b = iop.to_builder(spec);
    let iop_ir = b.optimize_ir();
    let config = PartitionerConfig {
        specs: std::iter::repeat(PartitionSpec {
            parallelism: 12,
            latency: Cycle(1),
        })
        .take(n_hpus)
        .collect(),
        sched_policy: schedp,
        tie_policy: tiep,
        spread_policy: spreadp,
    };
    let hpu_ir = pipeline(&iop_ir, &config);
    check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec.block_spec(), 100);
}
