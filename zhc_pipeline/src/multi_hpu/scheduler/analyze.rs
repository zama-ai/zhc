use zhc_ir::height_depth::analyze_height_depth;
use zhc_ir::{
    AnnIR, AsOpRef, IR, OpIdRaw, OpMap,
    visualization::{DynamicElement, NoClass, TextBox, VisualAnnotation},
};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang, HpuLocality};

static PBS_COST: OpIdRaw = 1000;
static NON_PBS_COST: OpIdRaw = 1;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Stats {
    pub height: OpIdRaw,
    pub depth: OpIdRaw,
    pub locality: HpuLocality,
}

impl VisualAnnotation for Stats {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        Some(Box::new(TextBox::<NoClass>::new(
            None,
            format!("{:?}", self),
        )))
    }
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
        | Transfer { .. } => NON_PBS_COST,
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

pub fn analyze<'a>(
    ir: &'a IR<HpuLang>,
    partitions: &OpMap<HpuLocality>,
) -> AnnIR<'a, HpuLang, Stats, ()> {
    analyze_height_depth(ir, |op, score| {
        let score = score.strict_add(compute_cost(&op));
        if op.get_instruction().is_pbs() {
            score / PBS_COST * PBS_COST
        } else {
            score
        }
    })
    .map_opann(|op| Stats {
        height: op.get_annotation().height,
        depth: op.get_annotation().depth,
        locality: partitions.get(op).unwrap().clone(),
    })
}
