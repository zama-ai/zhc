use zhc_builder::{CiphertextSpec, add, mh_mul, mul};
use zhc_ir::{AnnIRView, IR};
use zhc_langs::ioplang::IopLang;
use zhc_pipeline::{PartitionSpec, PartitionerConfig, SchedPolicy, TiePolicy};
use zhc_utils::units::Cycle;

fn pipeline(ir: &IR<IopLang>) {
    let config = PartitionerConfig {
        specs: [
            PartitionSpec {
                parallelism: 12,
                latency: Cycle(1),
            },
            PartitionSpec {
                parallelism: 12,
                latency: Cycle(1),
            },
        ]
        .into_iter()
        .collect(),
        sched_policy: SchedPolicy::AsSoonAsPossible,
        tie_policy: TiePolicy::Spread
    };
    let opmap = zhc_pipeline::passes::partition(&ir, &config);
    let valmap = ir.filled_valmap(());
    AnnIRView::new(ir, &opmap, &valmap)
        .draw_to_html(None)
        .open();
}

#[test]
fn test_partition_add() {
    pipeline(&mh_mul(CiphertextSpec::new(16, 2, 2), 2).optimize_ir());
}
