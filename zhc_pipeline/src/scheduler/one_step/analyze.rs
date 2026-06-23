use std::ops::{Div, Mul};

use zhc_ir::{Analysing, AnnIR, AnnOpRef, AsOpRef, IR, OpIdRaw};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang};
use zhc_utils::svec;

static PBS_COST: OpIdRaw = 1000;
static NON_PBS_COST: OpIdRaw = 1;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Stats {
    pub height: OpIdRaw,
    pub depth: OpIdRaw,
}

fn compute_cost(opref: impl AsOpRef<Dialect = HpuLang>) -> OpIdRaw {
    use HpuInstructionSet::*;
    match opref.op_ref().get_instruction() {
        AddCt
        | SubCt
        | Mac { .. }
        | AddPt
        | SubPt
        | PtSub
        | MulPt
        | AddCst { .. }
        | SubCst { .. }
        | CstSub { .. }
        | MulCst { .. }
        | CstCt { .. }
        | ImmLd { .. }
        | DstSt { .. }
        | SrcLd { .. }
        | TransferIn { .. }
        | TransferOut { .. } => NON_PBS_COST,
        Pbs { .. }
        | Pbs2 { .. }
        | Pbs4 { .. }
        | Pbs8 { .. }
        | PbsF { .. }
        | Pbs2F { .. }
        | Pbs4F { .. }
        | Pbs8F { .. } => PBS_COST,
        _ => unreachable!(),
    }
}

pub fn analyze<'a>(ir: &'a IR<HpuLang>) -> AnnIR<'a, HpuLang, Stats, ()> {
    let heighted = ir.backward_dataflow_analysis(|opref| {
        let op_cost = compute_cost(&opref);
        let mut height = opref
            .get_users_iter()
            .map(|p| p.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0 as OpIdRaw)
            .strict_add(op_cost);
        if opref.get_instruction().is_pbs() {
            height = height.div(PBS_COST).mul(PBS_COST);
        }
        (height, svec![(); opref.get_return_arity()])
    });
    heighted.forward_dataflow_analysis(
        |running_opref: AnnOpRef<'_, '_, _, Analysing<Stats>, _>, previous_opref| {
            let op_cost = compute_cost(&running_opref);
            let mut depth = running_opref
                .get_predecessors_iter()
                .map(|p| p.get_annotation().clone().unwrap_analyzed().depth)
                .max()
                .unwrap_or(0 as OpIdRaw)
                .strict_add(op_cost);
            if running_opref.get_instruction().is_pbs() {
                depth = depth.div(PBS_COST).mul(PBS_COST);
            }
            let height = *previous_opref.get_annotation();
            let stat = Stats { height, depth };
            (stat, svec![(); running_opref.get_return_arity()])
        },
    )
}
