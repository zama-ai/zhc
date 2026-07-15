use serde::Serialize;
use zhc_ir::{Analysing, AnnIR, AnnOpRef, AsOpRef, IR, OpIdRaw};
use zhc_langs::vmlang::{VmInstructionSet, VmLang};
use zhc_utils::svec;

pub(super) static PBS_COST: OpIdRaw = 200;
pub(super) static KS_COST: OpIdRaw = 10;
pub(super) static ALU_COST: OpIdRaw = 5;
pub(super) static MEM_COST: OpIdRaw = 1;

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct Stats {
    pub height: OpIdRaw,
    pub depth: OpIdRaw,
}

fn compute_cost(opref: impl AsOpRef<Dialect = VmLang>) -> OpIdRaw {
    use VmInstructionSet::*;
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
        | MulCst { .. } => ALU_COST,
        CstCt { .. } | ImmLd { .. } | DstSt { .. } | SrcLd { .. } => MEM_COST,
        Pbs { .. } | Pbs2 { .. } => PBS_COST,
        Ks => KS_COST,
    }
}

pub fn analyze<'a>(ir: &'a IR<VmLang>) -> AnnIR<'a, VmLang, Stats, ()> {
    let heighted = ir.backward_dataflow_analysis(|opref| {
        let op_cost = compute_cost(&opref);
        let height = opref
            .get_users_iter()
            .map(|p| p.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0 as OpIdRaw)
            .strict_add(op_cost);
        (height, svec![(); opref.get_return_arity()])
    });
    heighted.forward_dataflow_analysis(
        |running_opref: AnnOpRef<'_, '_, _, Analysing<Stats>, _>, previous_opref| {
            let op_cost = compute_cost(&running_opref);
            let depth = running_opref
                .get_predecessors_iter()
                .map(|p| p.get_annotation().clone().unwrap_analyzed().depth)
                .max()
                .unwrap_or(0 as OpIdRaw)
                .strict_add(op_cost);
            let height = *previous_opref.get_annotation();
            let stat = Stats { height, depth };
            (stat, svec![(); running_opref.get_return_arity()])
        },
    )
}
