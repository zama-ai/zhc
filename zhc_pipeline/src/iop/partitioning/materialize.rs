use zhc_ir::{
    AnnIRView, IR, OpMap, ValId,
    partitioning::{PartitionId, PartitionPlacement},
    translation::{Order, Translation, translate_ann},
};
use zhc_langs::ioplang::{IopInstructionSet, IopLang};
use zhc_utils::{FastMap, svec};

pub fn materialize_partitioning(
    ir: &IR<IopLang>,
    placement: &OpMap<PartitionPlacement>,
) -> (IR<IopLang>, OpMap<PartitionId>) {
    let val_annotations = ir.filled_valmap(());
    let annotated = AnnIRView::new(ir, placement, &val_annotations);
    assert!(
        annotated
            .walk_ops_linear()
            .all(|op| op.get_annotation().is_relevant()),
        "Cannot materialize incomplete partitioning"
    );

    let mut values: FastMap<(ValId, PartitionId), ValId> = FastMap::default();
    let mut output_placements = Vec::new();

    let Translation { output, .. } =
        translate_ann(annotated, Order::Topological, |op, translator| {
            let mut pids = op.get_annotation().iter_pids().collect::<Vec<_>>();
            pids.sort_unstable();
            for pid in pids {
                let mut args = svec![];
                for arg in op.get_args_iter() {
                    let key = (arg.get_id(), pid);
                    let materialized = if let Some(value) = values.get(&key).copied() {
                        value
                    } else {
                        // See [1] for why only exclusive are possible.
                        let source_pid = match arg.get_origin().opref.get_annotation() {
                            PartitionPlacement::Exclusive(source_pid) => *source_pid,
                            PartitionPlacement::Shared(_) => panic!(
                                "Shared producer of value {:?} has no replica in partition {:?}",
                                arg.get_id(),
                                pid
                            ),
                            _ => unreachable!(),
                        };
                        let source =
                            *values.get(&(arg.get_id(), source_pid)).unwrap_or_else(|| {
                                panic!(
                                    "No materialized source for value {:?} in partition {:?}",
                                    arg.get_id(),
                                    source_pid
                                )
                            });
                        let returns = translator.add_op(
                            IopInstructionSet::_Copy {
                                typ: arg.get_type(),
                            },
                            svec![source],
                        );
                        output_placements.push(pid);
                        let copied = returns[0];
                        values.insert(key, copied);
                        copied
                    };
                    args.push(materialized);
                }

                let new_returns = translator.add_op(op.get_instruction().clone(), args);
                output_placements.push(pid);
                for (old, new) in op
                    .get_return_valids()
                    .iter()
                    .zip(new_returns.iter().copied())
                {
                    assert!(
                        values.insert((*old, pid), new).is_none(),
                        "Materialized a value twice in partition {:?}",
                        pid
                    );
                }
            }
        });

    let mut materialized_placement = output.empty_opmap();
    assert_eq!(output.n_ops() as usize, output_placements.len());
    for (op, pid) in output.walk_ops_linear().zip(output_placements) {
        materialized_placement.insert(&op, pid);
    }
    (output, materialized_placement)
}

// Notes:
// ======
//
// [1]: About Materialization: Given how the auto-partitioning currently work, transfers (copies) between partitions
// can only ever happen after an exclusively allocated operation. There is no situation for now in
// which an already shared operation gets its results further shared with another partition. For
// this reason if a value isn't yet materialized it is expected to come from an Exclusive op, and as
// such there should be no further logic to select the transfer origin.
