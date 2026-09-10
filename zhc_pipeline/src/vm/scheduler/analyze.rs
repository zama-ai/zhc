use zhc_ir::height_depth::analyze_height_depth;
use zhc_ir::{AnnIR, AsOpRef, IR, OpIdRaw};
use zhc_langs::vmlang::{VmInstructionSet, VmLang};

pub(super) static PBS_COST: OpIdRaw = 200;
pub(super) static KS_COST: OpIdRaw = 10;
pub(super) static ALU_COST: OpIdRaw = 5;
pub(super) static MEM_COST: OpIdRaw = 1;

pub use zhc_ir::height_depth::HeightDepth as Stats;

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
    analyze_height_depth(ir, |op, score| score.strict_add(compute_cost(op)))
}
