use std::cmp::{max_by, min_by};

use zhc_utils::{
    FastMap,
    graphics::{Frame, Height, Position},
    iter::{CollectInSmallVec, Interleave, Slide, SliderExt},
    svec,
};

use crate::{
    AnnIR, AnnIRView, AnnOpRef, AnnValRef, AnnValUseRef, OpId, OpMap,
    visualization::{
        LayersMap, LayoutDialect, LayoutInstructionSet, OpContent,
        composition::annotation::CompositionVariable,
        placement::{Place, PlacementSolution},
    },
};

use super::*;

/// Converts a layout IR into a fully positioned scene graph.
pub fn compose(layout_ir: &AnnIR<'_, LayoutDialect, PlacementSolution, ()>) -> Scene {
    let (layers, ann_ir) = gen_layers(&layout_ir.view());

    // Generate curves (they use VariableWatch to read positions after solving)
    let curves = Inert::new(gen_curves(&ann_ir.view()));

    // Build scene and solve once
    let mut scene = Scene::new(None, layers, curves);
    scene.solve_size();
    let size = scene.get_size();
    scene.solve_frame(Frame {
        position: Position::ORIGIN,
        size,
    });

    scene
}

fn gen_layers<'ir, 'ann>(
    ir: &AnnIRView<'ir, 'ann, LayoutDialect, PlacementSolution, ()>,
) -> (Layers, AnnIR<'ir, LayoutDialect, CompositionVariable, ()>) {
    let layers_map = LayersMap::extract_from_ir(ir);
    let mut opmap = ir.empty_opmap();
    let mut variables = Vec::new();
    for layer in layers_map.iter_layers() {
        let mut order = ir
            .walk_ops_with(layer.walker())
            .map(|op| (op.get_annotation().get_place(), op.get_id()))
            .cosvec();
        order.sort_unstable_by_key(|(place, _)| *place);
        let (variable, assocs) = gen_layer(ir.walk_ops_with(order.into_iter().map(|(_, id)| id)));
        variables.push(LayerMemberLayer(variable));
        assocs.into_iter().for_each(|(k, v)| {
            opmap.insert(k, v);
        });
    }
    let variables = variables
        .into_iter()
        .interleave_with(
            layers_map
                .iter_layers()
                .slide::<2>()
                .skip_noncompletes()
                .map(|layers| {
                    let [layer_before, layer_after] = layers.unwrap_complete().into_array();
                    let is_group_input_sep = ir.walk_ops_with(layer_before.walker()).all(|op| {
                        matches!(
                            op.get_instruction(),
                            LayoutInstructionSet::GroupInput { .. }
                        )
                    });
                    let is_group_output_sep = ir.walk_ops_with(layer_after.walker()).all(|op| {
                        matches!(
                            op.get_instruction(),
                            LayoutInstructionSet::GroupOutput { .. }
                        )
                    });
                    let width_before = layer_before.walker().count() as f64;
                    let width_after = layer_after.walker().count() as f64;
                    let traffic = ir
                        .walk_ops_with(layer_after.walker())
                        .map(|op| op.get_args_arity())
                        .sum::<usize>() as f64;
                    let width_min =
                        min_by(width_before, width_after, |a, b| a.partial_cmp(b).unwrap());
                    let width_max =
                        max_by(width_before, width_after, |a, b| a.partial_cmp(b).unwrap());
                    let spread = (width_max / width_min.max(1.)).max(1.).sqrt();
                    let sep_height = if is_group_input_sep | is_group_output_sep {
                        Height::new(0.)
                    } else {
                        Height::new(20.0 + 10.0 * traffic * spread)
                    };
                    LayerMemberSeparator(LayerSeparator::vertical(None, sep_height))
                }),
        )
        .collect();
    (
        Layers::new(None, variables),
        AnnIR::new(ir.ir, opmap, ir.filled_valmap(())),
    )
}

fn gen_layer<'ir, 'ann>(
    ops: impl Iterator<Item = AnnOpRef<'ir, 'ann, LayoutDialect, PlacementSolution, ()>>,
) -> (Layer, FastMap<OpId, CompositionVariable>) {
    let mut assocs = FastMap::new();
    let mut variables = Vec::new();
    for op in ops {
        let (variable, ann) = gen_node(op.clone());
        variables.push(variable);
        assocs.insert(op.id, ann);
    }
    (Layer::new(None, variables), assocs)
}

/// Generates a node from a layout instruction.
fn gen_node<'ir, 'ann>(
    op: AnnOpRef<'ir, 'ann, LayoutDialect, PlacementSolution, ()>,
) -> (Node, CompositionVariable) {
    match (op.get_instruction(), op.get_annotation()) {
        (LayoutInstructionSet::Operation { op, .. }, _) if op.args.is_empty() => {
            // InputOp = V3<OpBody, Optional<OpComment>, OpOutputs>
            let variable = gen_input_op_node(&op);
            let ann = CompositionVariable::Op {
                sol: variable.get_variable_cell(),
                args: svec![],
                rets: variable
                    .e3
                    .content
                    .iter()
                    .map(|s| s.get_variable_cell())
                    .collect(),
                body: variable.e1.get_variable_cell(),
                comment: variable.e2.maybe_variable_cell(),
            };
            (NodeInputOpVar(variable), ann)
        }
        (LayoutInstructionSet::Operation { op, .. }, _) if op.returns.is_empty() => {
            // EffectOp = V3<OpInputs, OpBody, Optional<OpComment>>
            let variable = gen_effect_op_node(&op);
            let ann = CompositionVariable::Op {
                sol: variable.get_variable_cell(),
                args: variable
                    .e1
                    .content
                    .iter()
                    .map(|s| s.get_variable_cell())
                    .collect(),
                rets: svec![],
                body: variable.e2.get_variable_cell(),
                comment: variable.e3.maybe_variable_cell(),
            };
            (NodeEffectOpVar(variable), ann)
        }
        (LayoutInstructionSet::Operation { op, .. }, _) => {
            let variable = gen_op_node(&op);
            let ann = CompositionVariable::Op {
                sol: variable.get_variable_cell(),
                args: variable
                    .e1
                    .content
                    .iter()
                    .map(|s| s.get_variable_cell())
                    .collect(),
                body: variable.e2.get_variable_cell(),
                comment: variable.e3.maybe_variable_cell(),
                rets: variable
                    .e4
                    .content
                    .iter()
                    .map(|s| s.get_variable_cell())
                    .collect(),
            };
            (NodeOpVar(variable), ann)
        }
        (LayoutInstructionSet::Dummy { .. }, _) => {
            let variable = Dummy::new(None);
            let ann = CompositionVariable::Dummy {
                sol: variable.get_variable_cell(),
            };
            (NodeDummyVar(variable), ann)
        }
        (
            LayoutInstructionSet::Group { ir, name, .. },
            PlacementSolution::Group {
                maps,
                inputs,
                outputs,
                ..
            },
        ) => {
            let ann_ir = AnnIRView::new(&ir, &maps.0, &maps.1);
            let (variable, map) = gen_group_node(&ann_ir, name.as_str());
            let ann = CompositionVariable::Group {
                sol: variable.get_variable_cell(),
                inputs: inputs
                    .iter()
                    .map(|p| variable.0.e2.content[p.0 as usize].get_variable_cell())
                    .collect(),
                outputs: outputs
                    .iter()
                    .map(|p| variable.0.e4.content[p.0 as usize].get_variable_cell())
                    .collect(),
                maps: (map, ir.filled_valmap(())),
            };
            (NodeGroupVar(variable), ann)
        }
        (LayoutInstructionSet::GroupInput { .. }, _) => {
            let variable = GroupInputPort::new(None);
            let ann = CompositionVariable::GroupInput {
                sol: variable.get_variable_cell(),
            };
            (NodeGroupInputPortVar(variable), ann)
        }
        (LayoutInstructionSet::GroupOutput { .. }, _) => {
            let variable = GroupOutputPort::new(None);
            let ann = CompositionVariable::GroupOutput {
                sol: variable.get_variable_cell(),
            };
            (NodeGroupOutputPortVar(variable), ann)
        }
        _ => unreachable!(),
    }
}

fn gen_input_op_node(orig_op: &OpContent) -> InputOp {
    let body = OpBody::new(None, orig_op.call.clone());
    let comment = orig_op
        .comment
        .as_ref()
        .cloned()
        .map(|a| OpComment::new(None, a));
    let outputs = OpOutputs::new(
        None,
        orig_op
            .returns
            .iter()
            .cloned()
            .map(|a| TextBox::new(None, a))
            .collect(),
    );
    InputOp::new(None, body, comment.into(), outputs)
}

fn gen_effect_op_node(orig_op: &OpContent) -> EffectOp {
    let body = OpBody::new(None, orig_op.call.clone());
    let comment = orig_op
        .comment
        .as_ref()
        .cloned()
        .map(|a| OpComment::new(None, a));
    let inputs = OpInputs::new(
        None,
        orig_op
            .args
            .iter()
            .cloned()
            .map(|a| TextBox::new(None, a))
            .collect(),
    );
    EffectOp::new(None, inputs, body, comment.into())
}

fn gen_op_node(orig_op: &OpContent) -> Op {
    let body = OpBody::new(None, orig_op.call.clone());
    let comment = orig_op
        .comment
        .as_ref()
        .cloned()
        .map(|a| OpComment::new(None, a));
    let inputs = OpInputs::new(
        None,
        orig_op
            .args
            .iter()
            .cloned()
            .map(|a| TextBox::new(None, a))
            .collect(),
    );
    let outputs = OpOutputs::new(
        None,
        orig_op
            .returns
            .iter()
            .cloned()
            .map(|a| TextBox::new(None, a))
            .collect(),
    );
    Op::new(None, inputs, body, comment.into(), outputs)
}

/// Generates a node for a group (compound graph region).
fn gen_group_node<'ir, 'ann>(
    ir: &AnnIRView<'ir, 'ann, LayoutDialect, PlacementSolution, ()>,
    name: &str,
) -> (Group, OpMap<CompositionVariable>) {
    let (content, ann_ir) = gen_layers(&ir);
    let mut opmap = ann_ir.into_opmap();

    // Extract GroupInput and GroupOutput from the inner IR
    let mut group_inputs: Vec<(Place, OpId)> = Vec::new();
    let mut group_outputs: Vec<(Place, OpId)> = Vec::new();

    for op in ir.walk_ops_linear() {
        match (op.get_instruction(), op.get_annotation()) {
            (
                LayoutInstructionSet::GroupInput { .. },
                PlacementSolution::NonGroup { op: lp, .. },
            ) => {
                group_inputs.push((*lp, op.get_id()));
            }
            (
                LayoutInstructionSet::GroupOutput { .. },
                PlacementSolution::NonGroup { op: lp, .. },
            ) => {
                group_outputs.push((*lp, op.get_id()));
            }
            _ => {}
        }
    }

    group_inputs.sort_by_key(|(pos, _)| *pos);
    group_outputs.sort_by_key(|(pos, _)| *pos);

    // Build title
    let title = GroupTitle::new(None, name.to_string());

    // Build input ports
    let inputs = GroupInputs::new(
        None,
        group_inputs
            .iter()
            .map(|(_, opid)| {
                let variable = GroupInputPort::new(None);
                opmap.insert(
                    *opid,
                    CompositionVariable::GroupInput {
                        sol: variable.get_variable_cell(),
                    },
                );
                variable
            })
            .collect(),
    );

    // Build output ports
    let outputs = GroupOutputs::new(
        None,
        group_outputs
            .iter()
            .map(|(_, opid)| {
                let variable = GroupOutputPort::new(None);
                opmap.insert(
                    *opid,
                    CompositionVariable::GroupOutput {
                        sol: variable.get_variable_cell(),
                    },
                );
                variable
            })
            .collect(),
    );

    (Group(V4::new(None, title, inputs, content, outputs)), opmap)
}

/// Generates curves by tracing value flows through the IR.
fn gen_curves<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionVariable, ()>,
) -> Bag<Curve> {
    let mut curves = Vec::new();
    gen_curves_recursive(view, &mut curves);
    Bag::new(curves)
}

fn gen_curves_recursive<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionVariable, ()>,
    curves: &mut Vec<Curve>,
) {
    for op in view.walk_ops_linear() {
        match (op.get_instruction(), op.get_annotation()) {
            // Operation outputs -> trace forward
            (
                LayoutInstructionSet::Operation { returns, .. },
                CompositionVariable::Op { rets, .. },
            ) => {
                for ((orig_val_id, ret_val), ret_cell) in
                    returns.iter().zip(op.get_returns_iter()).zip(rets.iter())
                {
                    trace_curve(*orig_val_id, vec![ret_cell.watch()], ret_val, curves);
                }
            }
            // GroupInput inside a group -> trace forward
            (
                LayoutInstructionSet::GroupInput { valid, .. },
                CompositionVariable::GroupInput { sol },
            ) => {
                for ret_val in op.get_returns_iter() {
                    trace_curve(valid, vec![sol.watch()], ret_val, curves);
                }
            }
            // Group outputs (from outside) -> trace forward
            (
                LayoutInstructionSet::Group { .. },
                CompositionVariable::Group { outputs, maps, .. },
            ) => {
                // Group returns use the ValIds from the GroupOutput instructions inside
                // We trace from the group's output ports but need the original ValIds
                for (ret_val, out_cell) in op.get_returns_iter().zip(outputs.iter()) {
                    // The original ValId comes from the nested GroupOutput instruction
                    // For now, we need to find it by matching position
                    let ir = match op.get_instruction() {
                        LayoutInstructionSet::Group { ir, .. } => ir,
                        _ => unreachable!(),
                    };
                    // Find the GroupOutput at this position to get original valid
                    let orig_val_id = ir
                        .walk_ops_linear()
                        .filter_map(|inner_op| match inner_op.get_instruction() {
                            LayoutInstructionSet::GroupOutput { valid, .. } => Some(valid),
                            _ => None,
                        })
                        .nth(ret_val.get_origin().position as usize);
                    if let Some(orig_val_id) = orig_val_id {
                        trace_curve(orig_val_id, vec![out_cell.watch()], ret_val, curves);
                    }
                }
                // Recurse into group
                let ir = match op.get_instruction() {
                    LayoutInstructionSet::Group { ir, .. } => ir,
                    _ => unreachable!(),
                };
                let nested_view = AnnIRView::new(&ir, &maps.0, &maps.1);
                gen_curves_recursive(&nested_view, curves);
            }
            _ => {}
        }
    }
}

fn trace_curve<'ir, 'ann>(
    val_id: crate::ValId,
    waypoints: Vec<VariableWatch>,
    val: AnnValRef<'ir, 'ann, LayoutDialect, CompositionVariable, ()>,
    curves: &mut Vec<Curve>,
) {
    for AnnValUseRef { opref, position } in val.get_uses_iter() {
        match (opref.get_instruction(), opref.get_annotation()) {
            // Dummy: add waypoint and continue
            (LayoutInstructionSet::Dummy { .. }, CompositionVariable::Dummy { sol }) => {
                let mut new_waypoints = waypoints.clone();
                new_waypoints.push(sol.watch());
                if let Some(next_val) = opref.get_returns_iter().next() {
                    trace_curve(val_id, new_waypoints, next_val, curves);
                }
            }
            // Operation input: finish curve
            (LayoutInstructionSet::Operation { .. }, CompositionVariable::Op { args, .. }) => {
                if let Some(arg_cell) = args.get(position as usize) {
                    let mut final_waypoints = waypoints.clone();
                    final_waypoints.push(arg_cell.watch());
                    curves.push(Curve::new(None, final_waypoints, Some(val_id)));
                }
            }
            // Group input: finish curve
            (LayoutInstructionSet::Group { .. }, CompositionVariable::Group { inputs, .. }) => {
                if let Some(in_cell) = inputs.get(position as usize) {
                    let mut final_waypoints = waypoints.clone();
                    final_waypoints.push(in_cell.watch());
                    curves.push(Curve::new(None, final_waypoints, Some(val_id)));
                }
            }
            // GroupOutput: finish curve
            (
                LayoutInstructionSet::GroupOutput { .. },
                CompositionVariable::GroupOutput { sol },
            ) => {
                let mut final_waypoints = waypoints.clone();
                final_waypoints.push(sol.watch());
                curves.push(Curve::new(None, final_waypoints, Some(val_id)));
            }
            _ => {}
        }
    }
}
