use zhc::compat::Iop;
use zhc_builder::SpecInteger;
use zhc_ir::IR;
use zhc_langs::ioplang::IopLang;
use zhc_pipeline::{PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy};
use zhc_pipeline_correctness::equivalence_check::check_iop_equivalence;
use zhc_pipeline_correctness_macro::test_matrix;
use zhc_utils::units::Cycle;

fn pipeline(ir: &IR<IopLang>, config: &PartitionerConfig) -> IR<IopLang> {
    zhc_pipeline::passes::partition_and_materialize(ir, config).0
}

#[test_matrix(
    iop = @all_iops,
    size = @main_precs,
    schedp = @all_schedps,
    tiep = [tiep_concentrate = TiePolicy::Concentrate, tiep_balance = TiePolicy::Balance],
    spreadp = [spreadp_min_transfers = SpreadPolicy::MinimizeTransfers, spreadp_min_work = SpreadPolicy::MinimizeWork],
    n = [m2 = 2, m4 = 4, m8 = 8])
]
#[ignore]
fn correctness(
    iop: Iop,
    size: u16,
    schedp: SchedPolicy,
    tiep: TiePolicy,
    spreadp: SpreadPolicy,
    n: usize,
) {
    let config = PartitionerConfig {
        specs: std::iter::repeat(PartitionSpec {
            parallelism: 12,
            latency: Cycle(1),
        })
        .take(n)
        .collect(),
        sched_policy: schedp,
        tie_policy: tiep,
        spread_policy: spreadp,
    };
    let builder = iop.to_builder(SpecInteger::new(size, 2, 2));
    let spec = *builder.spec();
    let original = zhc_pipeline::passes::optimize_iop(&builder.ir());
    let materialized = pipeline(&original, &config);
    check_iop_equivalence(&original, &materialized, spec, 10);
}
