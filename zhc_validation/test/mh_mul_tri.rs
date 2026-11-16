//! Equivalence of the triangular MhMul through partitioning, multi-HPU lowering and scheduling.

use zhc_builder::{SpecInteger, mh_mul_low_depth, mh_mul_tri};
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::IR;
use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
use zhc_pipeline::{
    PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy,
    passes::{lower_iop_to_multi_hpu, multi_hpu_schedule, optimize_iop, partition_and_materialize},
};
use zhc_utils::units::Cycle;
use zhc_validation::equivalence_check::check_iop_multi_hpu_equivalence;

fn pipeline(ir: &IR<IopLang>, n_hpus: usize, spreadp: SpreadPolicy) -> Vec<IR<HpuLang>> {
    let config = PartitionerConfig {
        specs: std::iter::repeat(PartitionSpec { parallelism: 12, latency: Cycle(1) })
            .take(n_hpus)
            .collect(),
        sched_policy: SchedPolicy::AsSoonAsPossible,
        tie_policy: TiePolicy::Balance,
        spread_policy: spreadp,
    };
    let (remat, partitions) = partition_and_materialize(ir, &config);
    let (hpu_ir, localities) = lower_iop_to_multi_hpu(&remat, &partitions);
    let multi_config = MultiHpuConfig { n_hpus: n_hpus as u8, ..MultiHpuConfig::default() };
    multi_hpu_schedule(&hpu_ir, &localities, &multi_config, config.sched_policy)
}

#[test]
fn validate_mh_mul_tri() {
    for (size, split) in [(16u16, 2usize), (32, 2), (32, 4), (64, 2), (64, 4), (128, 2), (128, 4)] {
        let spec = SpecInteger::new(size, 2, 2);
        for (name, b) in [
            ("sequential", mh_mul_tri(spec, split, true)),
            ("low-depth", mh_mul_low_depth(spec, split)),
        ] {
            let iop_ir = optimize_iop(&b.ir());
            for n_hpus in [2, 4, 8] {
                for spreadp in [SpreadPolicy::MinimizeTransfers, SpreadPolicy::MinimizeWork] {
                    let hpu_irs = pipeline(&iop_ir, n_hpus, spreadp);
                    check_iop_multi_hpu_equivalence(
                        &iop_ir,
                        &hpu_irs,
                        spec.block_spec(),
                        &b.signature(),
                    );
                }
            }
            println!("mh_mul {name} limbs, {size} bits, split {split}: equivalent on 2/4/8 HPUs");
        }
    }
}

/// `Iop::MhMul` with its hand-made partition groups (one per board), as used on 2, 4 and 8
/// HPUs: the triangular limbs cross partitions with their division chain.
#[test]
fn validate_mh_mul_manual_partitions() {
    use zhc::compat::Iop;
    use zhc::prelude::*;
    for (size, par_n) in [(32u16, 2u8), (32, 4), (32, 8), (64, 2), (64, 4), (64, 8), (128, 2), (128, 4), (128, 8)] {
        {
            let spec = SpecInteger::new(size, 2, 2);
            let b = Iop::MhMul { par_n }.to_builder(spec);
            let reference = optimize_iop(&b.ir());
            let signature = b.signature();
            let mut pl = Pipeline::new()
                .with_builder(b)
                .with_builder_partitions(true)
                .with_multi_hpu_config(MultiHpuConfig { n_hpus: par_n, ..Default::default() });
            let hpu_irs = pl.get_scheduled_multi_hpulang().clone();
            check_iop_multi_hpu_equivalence(&reference, &hpu_irs, spec.block_spec(), &signature);
            println!("MhMul {size} bits, manual partitions on {par_n} HPUs: equivalent");
        }
    }
}
