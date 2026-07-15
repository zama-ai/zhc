use zhc_langs::hpulang::HpuInstructionSet;
use zhc_langs::hpulang::HpuInstructionSet::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Affinity {
    Pea,
    Pem,
    Pep,
    Ctl,
}

impl Affinity {
    pub fn extract(op: &HpuInstructionSet) -> Self {
        match op {
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
            | MulCst { .. } => Affinity::Pea,
            CstCt { .. } => Affinity::Ctl,
            ImmLd { .. } | DstSt { .. } | SrcLd { .. } => Affinity::Pem,
            Pbs { .. }
            | Pbs2 { .. }
            | Pbs4 { .. }
            | Pbs8 { .. }
            | PbsF { .. }
            | Pbs2F { .. }
            | Pbs4F { .. }
            | Pbs8F { .. } => Affinity::Pep,
            _ => unreachable!(),
        }
    }
}
