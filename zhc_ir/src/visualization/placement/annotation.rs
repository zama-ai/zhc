use std::fmt::Debug;
use zhc_utils::{
    iter::{CollectInSmallVec, CollectInVec},
    small::SmallVec,
    svec,
};

use crate::{
    AnnIR, IR, OpMap, ValMap,
    visualization::{LayoutDialect, LayoutInstructionSet},
};

use super::*;

#[derive(Clone, PartialEq, Eq)]
pub enum PlacementVariable {
    NonGroup {
        // The place cell for the op in question.
        // That is it contains a value between zero and the length of the layer.
        op: PlaceCell,
        // The place cells for the args of the op.
        // That is it contains values between zero and the total number of args of the ops of the
        // layer. Follows the order of the args themselves.
        args: SmallVec<PlaceCell>,
        // The place cells for the rets of the op.
        // That is it contains values between zero and the total number of returns of the ops of
        // the layer. Follows the order of the rets themselves.
        rets: SmallVec<PlaceCell>,
    },
    Group {
        // The place cell for the op in question.
        // That is it contains a value between zero and the length of the layer.
        op: PlaceCell,
        // The place cells for the args of the op.
        // That is it contains values between zero and the total number of args of the ops of the
        // layer. Follows the order of the args themselves.
        args: SmallVec<PlaceCell>,
        // The place cells for the rets of the op.
        // That is it contains values between zero and the total number of returns of the ops of
        // the layer. Follows the order of the rets themselves.
        rets: SmallVec<PlaceCell>,
        // The place cells for the input ops of the group.
        // Note that those are also stored as op of NonGroup variant somewhere else.
        // This is an alias that is used to simplify the bridge with the content of the group when
        // recursing. The values are between zero and number of inputs of the group.
        // Follows the order of the args.
        inputs: SmallVec<PlaceCell>,
        // The place cells for the outputs ops of the group.
        // Note that those are also stored as op of NonGroup variant somewhere else.
        // This is an alias that is used to simplify the bridge with the content of the group when
        // recursing. The values are between zero and number of inputs of the group.
        // Follows the order of the args.
        outputs: SmallVec<PlaceCell>,
        // The maps to pass annotations down to the nested ir.
        maps: (OpMap<PlacementVariable>, ValMap<()>),
    },
}

impl Debug for PlacementVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonGroup { op, args, rets } => {
                write!(f, "op: {op}, args: {args}, rets: {rets}")
            }
            Self::Group {
                op,
                args,
                rets,
                inputs,
                outputs,
                ..
            } => {
                write!(
                    f,
                    "op: {op}, args: {args}, rets: {rets}, inputs: {inputs}, outputs: {outputs}"
                )
            }
        }
    }
}

impl PlacementVariable {
    pub fn get_op_position(&self) -> &PlaceCell {
        match self {
            PlacementVariable::NonGroup { op, .. } | PlacementVariable::Group { op, .. } => op,
        }
    }

    pub fn get_arg_positions(&self) -> &[PlaceCell] {
        match self {
            PlacementVariable::NonGroup { args, .. } | PlacementVariable::Group { args, .. } => {
                args.as_slice()
            }
        }
    }

    pub fn get_ret_positions(&self) -> &[PlaceCell] {
        match self {
            PlacementVariable::NonGroup { rets, .. } | PlacementVariable::Group { rets, .. } => {
                rets.as_slice()
            }
        }
    }
}

pub fn annotate_for_solving(
    ir: &IR<LayoutDialect>,
) -> AnnIR<'_, LayoutDialect, PlacementVariable, ()> {
    ir.forward_dataflow_analysis(|opref| {
        use LayoutInstructionSet::*;
        let opann = match opref.get_instruction() {
            Operation { .. } | Dummy { .. } | GroupInput { .. } | GroupOutput { .. } => {
                PlacementVariable::NonGroup {
                    op: PlaceCell::new(0),
                    args: std::iter::repeat_with(|| PlaceCell::new(0))
                        .take(opref.get_args_arity())
                        .collect(),
                    rets: std::iter::repeat_with(|| PlaceCell::new(0))
                        .take(opref.get_return_arity())
                        .collect(),
                }
            }
            Group { ir, .. } => {
                let (opmap, valmap) = annotate_for_solving(&ir).into_maps();
                let mut inputs = ir
                    .walk_ops_linear()
                    .filter(|a| {
                        matches!(a.get_instruction(), LayoutInstructionSet::GroupInput { .. })
                    })
                    .covec();
                inputs.sort_unstable_by_key(|op| {
                    let LayoutInstructionSet::GroupInput { pos, .. } = op.get_instruction() else {
                        unreachable!()
                    };
                    pos
                });
                let inputs = inputs
                    .into_iter()
                    .map(|op| opmap.get(&*op).unwrap().get_op_position().clone())
                    .cosvec();
                let mut outputs = ir
                    .walk_ops_linear()
                    .filter(|a| {
                        matches!(
                            a.get_instruction(),
                            LayoutInstructionSet::GroupOutput { .. }
                        )
                    })
                    .covec();
                outputs.sort_unstable_by_key(|op| {
                    let LayoutInstructionSet::GroupOutput { pos, .. } = op.get_instruction() else {
                        unreachable!()
                    };
                    pos
                });
                let outputs = outputs
                    .into_iter()
                    .map(|op| opmap.get(&*op).unwrap().get_op_position().clone())
                    .cosvec();
                let op = PlaceCell::new(0);
                let args = std::iter::repeat_with(|| PlaceCell::new(0))
                    .take(opref.get_args_arity())
                    .collect();
                let rets = std::iter::repeat_with(|| PlaceCell::new(0))
                    .take(opref.get_return_arity())
                    .collect();
                PlacementVariable::Group {
                    op,
                    args,
                    rets,
                    inputs,
                    outputs,
                    maps: (opmap, valmap),
                }
            }
        };
        (opann, svec![(); opref.get_return_arity()])
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementSolution {
    NonGroup {
        op: Place,
    },
    Group {
        op: Place,
        inputs: SmallVec<Place>,
        outputs: SmallVec<Place>,
        maps: (OpMap<PlacementSolution>, ValMap<()>),
    },
}

impl PlacementSolution {
    pub fn get_place(&self) -> Place {
        match self {
            PlacementSolution::NonGroup { op } | PlacementSolution::Group { op, .. } => *op,
        }
    }
}

/// Convert raw layer positions to ranks (0..n-1) based on sorted order.
fn positions_to_ranks(positions: SmallVec<Place>) -> SmallVec<Place> {
    let mut indexed = positions.iter().enumerate().map(|(i, p)| (i, *p)).covec();
    indexed.sort_by_key(|(_, pos)| *pos);
    let mut ranks = vec![Place(0.0); positions.len()];
    for (rank, (orig_idx, _)) in indexed.iter().enumerate() {
        ranks[*orig_idx] = Place(rank as f64);
    }
    ranks.into_iter().collect()
}

fn resolve(input: OpMap<PlacementVariable>) -> OpMap<PlacementSolution> {
    input.map(|v| match v {
        PlacementVariable::NonGroup { op, .. } => PlacementSolution::NonGroup { op: op.get_val() },
        PlacementVariable::Group {
            op,
            maps,
            inputs,
            outputs,
            ..
        } => {
            // Convert raw layer positions to ranks, since other ops on the same
            // layer can create gaps in the position sequence.
            let input_positions = inputs.into_iter().map(|a| a.get_val()).cosvec();
            let output_positions = outputs.into_iter().map(|a| a.get_val()).cosvec();
            PlacementSolution::Group {
                op: op.get_val(),
                inputs: positions_to_ranks(input_positions),
                outputs: positions_to_ranks(output_positions),
                maps: (resolve(maps.0), maps.1.map(|_| ())),
            }
        }
    })
}

pub fn turn_to_solution(
    ir: AnnIR<'_, LayoutDialect, PlacementVariable, ()>,
) -> AnnIR<'_, LayoutDialect, PlacementSolution, ()> {
    let AnnIR {
        ir,
        op_annotations,
        val_annotations,
    } = ir;

    let op_annotations = resolve(op_annotations);
    AnnIR::new(ir, op_annotations, val_annotations.map(|_| ()))
}
