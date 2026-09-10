use zhc_ir::height_depth::analyze_height_depth;
use zhc_ir::{AnnIR, AsOpRef, IR, OpIdRaw};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang};

static PBS_COST: OpIdRaw = 1000;
static NON_PBS_COST: OpIdRaw = 1;

pub use zhc_ir::height_depth::HeightDepth as Stats;

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
        | SrcLd { .. } => NON_PBS_COST,
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
    analyze_height_depth(ir, |op, score| {
        let score = score.strict_add(compute_cost(&op));
        if op.get_instruction().is_pbs() {
            score / PBS_COST * PBS_COST
        } else {
            score
        }
    })
}
