use zhc_ir::OpRef;
use zhc_langs::hpulang::HpuInstructionSet::*;
use zhc_langs::hpulang::HpuLang;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Affinity {
    Pea,
    Pem,
    Pep,
    Ctl,
}

impl Affinity {
    pub fn extract<'a>(op: &OpRef<'a, HpuLang>) -> Self {
        match op.get_instruction() {
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
            TransferIn { .. } | TransferOut { .. } => Affinity::Ctl,
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
