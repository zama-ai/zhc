use zhc_utils::{graphics::Frame, small::SmallVec};

use crate::{AnnIR, OpMap, ValMap, visualization::LayoutDialect};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionVariable {
    Op {
        sol: VariableCell,
        args: SmallVec<VariableCell>,
        rets: SmallVec<VariableCell>,
        body: VariableCell,
        comment: Option<VariableCell>,
    },
    Dummy {
        sol: VariableCell,
    },
    GroupInput {
        sol: VariableCell,
    },
    GroupOutput {
        sol: VariableCell,
    },
    Group {
        sol: VariableCell,
        inputs: SmallVec<VariableCell>,
        outputs: SmallVec<VariableCell>,
        maps: (OpMap<CompositionVariable>, ValMap<()>),
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionSolution {
    Op {
        sol: Solution,
        args: SmallVec<Solution>,
        rets: SmallVec<Solution>,
        body: Solution,
        comment: Option<Solution>,
    },
    Dummy {
        sol: Solution,
    },
    GroupInput {
        sol: Solution,
    },
    GroupOutput {
        sol: Solution,
    },
    Group {
        sol: Solution,
        inputs: SmallVec<Solution>,
        outputs: SmallVec<Solution>,
        maps: (OpMap<CompositionSolution>, ValMap<()>),
    },
}

impl CompositionSolution {
    /// Returns the overall frame for this element.
    pub fn get_frame(&self) -> Frame {
        match self {
            CompositionSolution::Op { sol, .. } => sol.frame.clone(),
            CompositionSolution::Dummy { sol } => sol.frame.clone(),
            CompositionSolution::GroupInput { sol } => sol.frame.clone(),
            CompositionSolution::GroupOutput { sol } => sol.frame.clone(),
            CompositionSolution::Group { sol, .. } => sol.frame.clone(),
        }
    }
}

fn resolve(input: OpMap<CompositionVariable>) -> OpMap<CompositionSolution> {
    input.map(|v| match v {
        CompositionVariable::Op {
            sol,
            args,
            rets,
            body,
            comment,
        } => CompositionSolution::Op {
            sol: sol.get_solution(),
            args: args.into_iter().map(|a| a.get_solution()).collect(),
            rets: rets.into_iter().map(|a| a.get_solution()).collect(),
            body: body.get_solution(),
            comment: comment.map(|a| a.get_solution()),
        },
        CompositionVariable::Dummy { sol } => CompositionSolution::Dummy {
            sol: sol.get_solution(),
        },
        CompositionVariable::GroupInput { sol } => CompositionSolution::GroupInput {
            sol: sol.get_solution(),
        },
        CompositionVariable::GroupOutput { sol } => CompositionSolution::GroupOutput {
            sol: sol.get_solution(),
        },
        CompositionVariable::Group {
            sol,
            inputs,
            outputs,
            maps,
        } => CompositionSolution::Group {
            sol: sol.get_solution(),
            inputs: inputs.into_iter().map(|a| a.get_solution()).collect(),
            outputs: outputs.into_iter().map(|a| a.get_solution()).collect(),
            maps: (resolve(maps.0), maps.1),
        },
    })
}

pub fn turn_to_solution(
    ir: AnnIR<'_, LayoutDialect, CompositionVariable, ()>,
) -> AnnIR<'_, LayoutDialect, CompositionSolution, ()> {
    let AnnIR {
        ir,
        op_annotations,
        val_annotations,
    } = ir;
    let op_annotations = resolve(op_annotations);
    AnnIR::new(ir, op_annotations, val_annotations)
}
