use zhc::compat::Iop;
use zhc_builder::SpecInteger;
use zhc_config::{
    hpu::{HpuConfig, PhysicalConfig},
    multi_hpu::MultiHpuConfig,
};
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::IR;
use zhc_langs::{
    doplang::{DopInstructionSet, DopLang},
    hpulang::N_TRANSFER_FLAGS,
    ioplang::IopLang,
};
use zhc_pipeline::{
    PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy, extract_lut_registry,
    passes::{
        hpu_allocate_registers, lower_iop_to_multi_hpu, multi_hpu_schedule, optimize_iop,
        partition_and_materialize,
    },
};
use zhc_utils::units::Cycle;
use zhc_validation::equivalence_check::check_iop_multi_dop_equivalence;
use zhc_validation_macro::test_matrix;

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

fn pipeline(
    ir: &IR<IopLang>,
    config: &PartitionerConfig,
) -> (Vec<IR<DopLang>>, LutRegistry, HpuConfig) {
    let lut_reg = extract_lut_registry(ir);
    let (remat, partitions) = partition_and_materialize(ir, config);
    let (hpu_ir, localities) = lower_iop_to_multi_hpu(&remat, &partitions);
    let multi_config = MultiHpuConfig {
        hpu_config: HpuConfig::from(PhysicalConfig::gaussian_64b()),
        n_hpus: config.specs.len() as u8,
    };
    let scheduled = multi_hpu_schedule(&hpu_ir, &localities, &multi_config, config.sched_policy);
    let allocated = scheduled
        .iter()
        .map(|ir| {
            hpu_allocate_registers(
                ir,
                &multi_config.hpu_config,
                &lut_reg,
                multi_config.n_hpus as usize,
                N_TRANSFER_FLAGS as usize,
            )
        })
        .collect();
    (allocated, lut_reg, multi_config.hpu_config)
}

fn count_notifies(dop_irs: &[IR<DopLang>]) -> usize {
    dop_irs
        .iter()
        .flat_map(|ir| ir.walk_ops_linear())
        .filter(|op| matches!(op.get_instruction(), DopInstructionSet::NOTIFY { .. }))
        .count()
}

#[test]
fn smoke() {
    let spec = SpecInteger::new(8, 2, 2);
    let iop_ir = optimize_iop(&Iop::Add.to_builder(spec).ir());
    let config = partitioner_config(
        2,
        SchedPolicy::AsSoonAsPossible,
        TiePolicy::Balance,
        SpreadPolicy::MinimizeWork,
    );
    let (dop_irs, lut_reg, hpu_config) = pipeline(&iop_ir, &config);
    assert!(
        count_notifies(&dop_irs) > 0,
        "The streams have no transfer; the test is vacuous."
    );
    check_iop_multi_dop_equivalence(
        &iop_ir,
        &dop_irs,
        &lut_reg,
        spec.block_spec(),
        hpu_config.regf_size,
    );
}

#[test_matrix(
    iop = @all_iops,
    size = @main_precs,
    schedp = @all_schedps,
    tiep = [tiep_concentrate = TiePolicy::Concentrate, tiep_balance = TiePolicy::Balance],
    spreadp = [spreadp_min_transfers = SpreadPolicy::MinimizeTransfers, spreadp_min_work = SpreadPolicy::MinimizeWork],
    n_hpus = [m2 = 2, m4 = 4, m8 = 8])
]
#[ignore]
fn validate_multi_hpu_allocate_register(
    iop: Iop,
    size: u16,
    schedp: SchedPolicy,
    tiep: TiePolicy,
    spreadp: SpreadPolicy,
    n_hpus: usize,
) {
    let spec = SpecInteger::new(size, 2, 2);
    let iop_ir = optimize_iop(&iop.to_builder(spec).ir());
    let config = partitioner_config(n_hpus, schedp, tiep, spreadp);
    let (dop_irs, lut_reg, hpu_config) = pipeline(&iop_ir, &config);
    check_iop_multi_dop_equivalence(
        &iop_ir,
        &dop_irs,
        &lut_reg,
        spec.block_spec(),
        hpu_config.regf_size,
    );
}
