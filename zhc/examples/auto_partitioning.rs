use zhc::prelude::*;
use zhc_builder::CiphertextBlock;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::{
    AnnIR, Annotation, IR, OpMap,
    partitioning::{PartitionAnnotation, PartitionId, PartitionPlacement},
    visualization::{Hierarchy, VisualAnnotation, draw_ann_ir_to_svg, draw_ir_to_svg},
};
use zhc_langs::{
    hpulang::{HpuInstructionSet, HpuLocality},
    ioplang::IopLang,
    tasklang::coarsen_ioplang,
};
use zhc_pipeline::{
    PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy,
    materialize_partitioning, partition,
    passes::{lower_iop_to_multi_hpu, multi_hpu_schedule},
};
use zhc_utils::{files::FileHandle, units::Cycle};

const INT_SIZE: u16 = 2;
const N_HPUS: u8 = 2;
const TASKS_PER_BATCH: u16 = 1;

fn add(bd: &Builder, lhs: CiphertextBlock, rhs: CiphertextBlock) -> CiphertextBlock {
    let sum = bd.block_add(lhs, rhs);
    bd.block_lookup(sum, Lut1::msg_only(*bd.spec()))
}

fn circuit() -> Builder {
    let bd = Builder::new(SpecBlock(2, 2));
    let inputs: Vec<_> = (0..3)
        .map(|_| {
            let input = bd.ciphertext_integer_input(INT_SIZE);
            bd.ciphertext_integer_split(input)[0]
        })
        .collect();
    let ab = add(&bd.comment("a + b"), inputs[0], inputs[1]);
    let bc = add(&bd.comment("b + c"), inputs[1], inputs[2]);
    let sum = add(&bd.comment("(a + b) + (b + c)"), ab, bc);
    let output = bd.ciphertext_integer_join([sum], Some(INT_SIZE));
    bd.ciphertext_integer_output(output);
    bd
}

fn partitioner_config() -> PartitionerConfig {
    PartitionerConfig {
        specs: std::iter::repeat(PartitionSpec {
            parallelism: TASKS_PER_BATCH,
            latency: Cycle(1),
        })
        .take(N_HPUS as usize)
        .collect(),
        sched_policy: SchedPolicy::AsSoonAsPossible,
        tie_policy: TiePolicy::Concentrate,
        spread_policy: SpreadPolicy::MinimizeTransfers,
    }
}

fn hpu_colors(localities: &OpMap<HpuLocality>) -> OpMap<PartitionPlacement> {
    localities.clone().map(|locality| match locality {
        HpuLocality::OnHpu(hid) => PartitionPlacement::Exclusive(PartitionId(hid.0 as u16)),
        HpuLocality::Transfer { src, dst } => [src, dst]
            .into_iter()
            .map(|hid| PartitionId(hid.0 as u16))
            .collect(),
    })
}

fn draw_annotated<A: Annotation + VisualAnnotation>(
    ir: &IR<IopLang>,
    annotations: OpMap<A>,
    hierarchy: Option<OpMap<Hierarchy>>,
) -> FileHandle {
    let ann_ir = AnnIR::new(ir, annotations, ir.filled_valmap(()));
    draw_ann_ir_to_svg(&ann_ir.view(), hierarchy)
}

fn main() {
    let bd = circuit();
    let ir = bd.optimize_ir();
    let hierarchy = bd.hierarchy();

    draw_ir_to_svg(&ir, Some(hierarchy.clone())).open().unwrap();

    let tasks = coarsen_ioplang(&ir).output;
    draw_ir_to_svg(&tasks, None).open().unwrap();

    let config = partitioner_config();
    let placement = partition(&ir, &config);
    draw_annotated(&ir, placement.clone(), Some(hierarchy))
        .open()
        .unwrap();

    let (materialized, partitions) = materialize_partitioning(&ir, &placement);
    draw_annotated(&materialized, partitions.clone(), None)
        .open()
        .unwrap();

    let (hpulang, localities) = lower_iop_to_multi_hpu(&materialized, &partitions);
    let ann_hpulang = AnnIR::new(&hpulang, hpu_colors(&localities), hpulang.filled_valmap(()));
    draw_ann_ir_to_svg(&ann_hpulang.view(), None)
        .open()
        .unwrap();
    let n_transfers = hpulang
        .walk_ops_linear()
        .filter(|op| matches!(op.get_instruction(), HpuInstructionSet::Transfer { .. }))
        .count();

    let multi_hpu_config = MultiHpuConfig {
        n_hpus: N_HPUS,
        ..Default::default()
    };
    let programs = multi_hpu_schedule(
        &hpulang,
        &localities,
        &multi_hpu_config,
        SchedPolicy::AsSoonAsPossible,
    );
    for program in programs.iter() {
        draw_ir_to_svg(program, None).open().unwrap();
    }

    let annotations = placement.map(|placement| match placement {
        PartitionPlacement::Exclusive(pid) => PartitionAnnotation::new_exclusive(pid, "auto"),
        _ => PartitionAnnotation::new_irrelevant(),
    });
    let mut pipeline = Pipeline::new()
        .with_unchecked_ioplang(ir)
        .with_prototype(bd.signature())
        .with_ciphertext_block_spec(*bd.spec())
        .with_partitions(annotations)
        .with_builder_partitions(true)
        .with_multi_hpu_config(multi_hpu_config);
    for assembly in pipeline.get_multi_hpu_assembly().iter() {
        assembly.open().unwrap();
    }
    pipeline.get_multi_hpu_trace().open().unwrap();
    assert!(pipeline.get_partitions() == &partitions);

    println!("{n_transfers} inter-HPU transfers");
    pipeline.get_multi_hpu_metrics().dump();
}
