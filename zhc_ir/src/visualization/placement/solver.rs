use zhc_utils::SafeAs;
use zhc_utils::iter::{CollectInSmallVec, Median, MultiZip};

use crate::visualization::placement::Place;
use crate::visualization::placement::annotation::{
    PlacementSolution, PlacementVariable, annotate_for_solving, turn_to_solution,
};
use crate::visualization::{LayersMap, LayoutDialect, LayoutInstructionSet};
use crate::{AnnIR, AnnIRView, AnnValUseRef, IR};

fn place_once_top_down<'ir, 'ann>(ir: AnnIRView<'ir, 'ann, LayoutDialect, PlacementVariable, ()>) {
    let layers_map = LayersMap::extract_from_ir(&*ir);
    let mut layer_ops = Vec::new();
    // We walk through layers from the top to the bottom.
    for layer in layers_map.iter_layers() {
        // We iterate on the ops of the layer.
        for op in ir.walk_ops_with(layer.walker()) {
            // If the op has predecessor we compute its new median position based on those.
            // Otherwise, we leave it. Note that this is valid for groups because when
            // we recursively enter a new group, the position of its inputs have been
            // set based on the position of group arguments.
            if op.get_args_arity() != 0 {
                let new = op
                    .get_args_iter()
                    .map(|a| {
                        let orig = a.get_origin();
                        orig.opref.get_annotation().get_ret_positions()
                            [orig.position.sas::<usize>()]
                        .get_val()
                        .0
                    })
                    .median()
                    .map(Place)
                    .unwrap();
                op.get_annotation().get_op_position().set_val(new);
            }
            // If a group, we also recursively place inside.
            if let (
                LayoutInstructionSet::Group { ir, .. },
                PlacementVariable::Group { inputs, maps, .. },
            ) = (op.get_instruction(), op.get_annotation())
            {
                // First, we set the positions of the input ops inside the group based on the
                // position of the arguments origin outside of the group.
                for (arg, input) in (op.get_args_iter(), inputs.iter()).mzip() {
                    let orig = arg.get_origin();
                    input.set_val(
                        orig.opref.get_annotation().get_ret_positions()
                            [orig.position.sas::<usize>()]
                        .get_val(),
                    );
                }
                let mut inputs_ordered = inputs.clone();
                inputs_ordered.sort_unstable_by_key(|i| i.get_val());
                for (i, op) in inputs_ordered.iter().enumerate() {
                    op.set_val(Place(i as f64));
                }

                // Now we recursively place things inside.
                place_once_top_down(AnnIRView::new(&ir, &maps.0, &maps.1));

                // The output positions can only be accounted for once we have reordered all the ops
                // of the layer
            }
            layer_ops.push(op);
        }

        // Now that we have visited all the operations of the layer, we reorder the operations based
        // on their new positions
        layer_ops
            .as_mut_slice()
            .sort_by_key(|a| a.get_annotation().get_op_position().get_val());
        for (i, op) in layer_ops.iter_mut().enumerate() {
            op.get_annotation()
                .get_op_position()
                .set_val(Place(i as f64));
        }

        // Now that we have reordered all the operations of the layer, we propagate to the rets and
        // the args of the ops. Note that it matters to reorder args also when chaining with
        // the reverse pass.
        let mut rets_i = 0;
        let mut args_i = 0;
        for op in layer_ops.iter_mut() {
            for arg in op.get_annotation().get_arg_positions() {
                arg.set_val(Place(args_i as f64));
                args_i += 1;
            }
            for ret in op.get_annotation().get_ret_positions() {
                ret.set_val(Place(rets_i as f64));
                rets_i += 1;
            }
        }

        // Last, for the group operations of the layer, we reconcile the rets positions with the
        // constraints coming from inside of the group. That is, we reorder the rets
        // position following the order of the group outputs
        for op in layer_ops.iter_mut() {
            if let PlacementVariable::Group { outputs, rets, .. } = op.get_annotation() {
                let rets_original_places = rets.iter().map(|r| r.get_val()).cosvec();
                for (ret, val) in (rets.iter(), outputs.iter()).mzip() {
                    ret.set_val(rets_original_places[val.get_val().0 as usize]);
                }
            }
        }

        layer_ops.clear();
    }
}

fn place_once_bottom_up<'ir, 'ann>(ir: AnnIRView<'ir, 'ann, LayoutDialect, PlacementVariable, ()>) {
    let layers_map = LayersMap::extract_from_ir(&*ir);
    let mut layer_ops = Vec::new();
    // We walk through layers from the bottom to the top.
    for layer in layers_map.iter_layers().rev() {
        // We iterate on the ops of the layer.
        for op in ir.walk_ops_with(layer.walker()) {
            // If the op has users we compute its new median position based on those. Otherwise, we
            // leave it. Note that this is valid for groups because when we recursively
            // enter a new group, the position of its inputs have been set based on the
            // position of group arguments.
            if op.get_users_iter().count() != 0 {
                let new = op
                    .get_returns_iter()
                    .flat_map(|r| r.get_uses_iter())
                    .map(|AnnValUseRef { opref, position }| {
                        opref.get_annotation().get_arg_positions()[position as usize]
                            .get_val()
                            .0
                    })
                    .median()
                    .unwrap();
                op.get_annotation().get_op_position().set_val(Place(new));
            }
            // If a group, we also recursively place inside.
            if let (
                LayoutInstructionSet::Group { ir, .. },
                PlacementVariable::Group { outputs, maps, .. },
            ) = (op.get_instruction(), op.get_annotation())
            {
                // First, we set the positions of the output ops inside the group based on the
                // position of the rets outside of the group.
                for (ret, output) in (op.get_returns_iter(), outputs.iter()).mzip() {
                    let pos = ret
                        .get_uses_iter()
                        .map(|AnnValUseRef { opref, position }| {
                            opref.get_annotation().get_arg_positions()[position as usize]
                                .get_val()
                                .0
                        })
                        .median()
                        .map(Place)
                        .unwrap_or(output.get_val());
                    output.set_val(pos);
                }
                let mut outputs_ordered = outputs.clone();
                outputs_ordered.sort_unstable_by_key(|i| i.get_val());
                for (i, op) in outputs_ordered.iter().enumerate() {
                    op.set_val(Place(i as f64));
                }

                // Now we recursively place things inside.
                place_once_bottom_up(AnnIRView::new(&ir, &maps.0, &maps.1));

                // The inputs positions can only be accounted for once we have reordered all the ops
                // of the layer
            }
            layer_ops.push(op);
        }

        // Now that we have visited all the operations of the layer, we reorder the operations based
        // on their new positions
        layer_ops
            .as_mut_slice()
            .sort_by_key(|a| a.get_annotation().get_op_position().get_val());
        for (i, op) in layer_ops.iter_mut().enumerate() {
            op.get_annotation()
                .get_op_position()
                .set_val(Place(i as f64));
        }

        // Now that we have reordered all the operations of the layer, we propagate to the rets and
        // the args of the ops. Note that it matters to reorder args also when chaining with
        // the reverse pass.
        let mut rets_i = 0;
        let mut args_i = 0;
        for op in layer_ops.iter_mut() {
            for arg in op.get_annotation().get_arg_positions() {
                arg.set_val(Place(args_i as f64));
                args_i += 1;
            }
            for ret in op.get_annotation().get_ret_positions() {
                ret.set_val(Place(rets_i as f64));
                rets_i += 1;
            }
        }

        // Last, for the group operations of the layer, we reconcile the args positions with the
        // constraints coming from inside of the group. That is, we reorder the args
        // position following the order of the group inputs.
        for op in layer_ops.iter_mut() {
            if let PlacementVariable::Group { inputs, args, .. } = op.get_annotation() {
                let args_original_places = args.iter().map(|r| r.get_val()).cosvec();
                for (arg, val) in (args.iter(), inputs.iter()).mzip() {
                    arg.set_val(args_original_places[val.get_val().0 as usize]);
                }
            }
        }

        layer_ops.clear();
    }
}

pub fn place(ir: &IR<LayoutDialect>) -> AnnIR<'_, LayoutDialect, PlacementSolution, ()> {
    let solvable_ir = annotate_for_solving(ir);
    for _ in 0..100 {
        place_once_top_down(solvable_ir.view());
        place_once_bottom_up(solvable_ir.view());
    }
    turn_to_solution(solvable_ir)
}
