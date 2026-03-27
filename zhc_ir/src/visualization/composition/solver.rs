use zhc_utils::{
    FastMap,
    graphics::{Frame, Height, Position},
    iter::CollectInSmallVec,
    svec,
};

use crate::{
    AnnIR, AnnIRView, AnnOpRef, OpId, OpMap,
    visualization::{
        LayersMap, LayoutDialect, LayoutInstructionSet, OpContent,
        composition::annotation::{CompositionSolution, CompositionVariable},
        placement::{Place, PlacementSolution},
    },
};

use super::*;

/// Converts a layout IR into a fully positioned diagram element.
pub fn compose<'ir, 'ann>(
    layout_ir: &AnnIR<'ir, LayoutDialect, PlacementSolution, ()>,
    stylesheet: &StyleSheet,
) -> AnnIR<'ir, LayoutDialect, CompositionSolution, ()> {
    let (mut solver, ann_ir) = gen_layers(&layout_ir.view());
    solver.solve_size(stylesheet);
    let size = solver.get_size();
    solver.solve_frame(
        stylesheet,
        Frame {
            position: Position::ORIGIN,
            size,
        },
    );
    turn_to_solution(ann_ir)
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
        let links_out = ir
            .walk_ops_with(layer.walker())
            .flat_map(|op| op.get_returns_iter().flat_map(|v| v.get_uses_iter()))
            .count();
        let sep_height = Height::new(10. * links_out as f64);
        variables.push(LayerMemberSeparator(LayerSeparator::vertical(sep_height)));
        assocs.into_iter().for_each(|(k, v)| {
            opmap.insert(k, v);
        });
    }
    variables.pop();
    (
        Layers::new(variables),
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
    (Layer::new(variables), assocs)
}

/// Generates a node from a layout instruction.
fn gen_node<'ir, 'ann>(
    op: AnnOpRef<'ir, 'ann, LayoutDialect, PlacementSolution, ()>,
) -> (Node, CompositionVariable) {
    match (op.get_instruction(), op.get_annotation()) {
        (LayoutInstructionSet::Operation { op, .. }, _) if op.args.is_empty() => {
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
            let variable = Dummy::new();
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
            let variable = GroupInputPort::new();
            let ann = CompositionVariable::GroupInput {
                sol: variable.get_variable_cell(),
            };
            (NodeGroupInputPortVar(variable), ann)
        }
        (LayoutInstructionSet::GroupOutput { .. }, _) => {
            let variable = GroupOutputPort::new();
            let ann = CompositionVariable::GroupOutput {
                sol: variable.get_variable_cell(),
            };
            (NodeGroupOutputPortVar(variable), ann)
        }
        _ => unreachable!(),
    }
}

fn gen_input_op_node(orig_op: &OpContent) -> InputOp {
    let body = OpBody::new(orig_op.call.clone());
    let comment = orig_op.comment.as_ref().cloned().map(OpComment::new);
    let outputs = OpOutputs::new(orig_op.returns.iter().cloned().map(TextBox::new).collect());
    InputOp::new(body, comment.into(), outputs)
}

fn gen_effect_op_node(orig_op: &OpContent) -> EffectOp {
    let body = OpBody::new(orig_op.call.clone());
    let comment = orig_op.comment.as_ref().cloned().map(OpComment::new);
    let inputs = OpInputs::new(orig_op.args.iter().cloned().map(TextBox::new).collect());
    EffectOp::new(inputs, body, comment.into())
}

fn gen_op_node(orig_op: &OpContent) -> Op {
    let body = OpBody::new(orig_op.call.clone());
    let comment = orig_op.comment.as_ref().cloned().map(OpComment::new);
    let inputs = OpInputs::new(orig_op.args.iter().cloned().map(TextBox::new).collect());
    let outputs = OpOutputs::new(orig_op.returns.iter().cloned().map(TextBox::new).collect());
    Op::new(inputs, body, comment.into(), outputs)
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
            (LayoutInstructionSet::GroupInput { .. }, PlacementSolution::NonGroup { op: lp }) => {
                group_inputs.push((*lp, op.get_id()));
            }
            (LayoutInstructionSet::GroupOutput { .. }, PlacementSolution::NonGroup { op: lp }) => {
                group_outputs.push((*lp, op.get_id()));
            }
            _ => {}
        }
    }

    group_inputs.sort_by_key(|(pos, _)| *pos);
    group_outputs.sort_by_key(|(pos, _)| *pos);

    // Build title
    let title = GroupTitle::new(name.to_string());

    // Build input ports
    let inputs = GroupInputs::new(
        group_inputs
            .iter()
            .map(|(_, opid)| {
                let variable = GroupInputPort::new();
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
        group_outputs
            .iter()
            .map(|(_, opid)| {
                let variable = GroupOutputPort::new();
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

    (Group(V4::new(title, inputs, content, outputs)), opmap)
}
