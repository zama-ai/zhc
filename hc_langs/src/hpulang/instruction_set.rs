use std::fmt::{Debug, Display};

use hc_ir::{DialectInstructionSet, IR, Signature, sig};
use hc_utils::iter::CollectInSmallVec;

use super::{HpuLang, type_system::HpuTypeSystem};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Immediate(pub u8);

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_imm", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TSrcId {
    pub src_pos: u32,
    pub block_pos: u32,
}

impl Display for TSrcId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_tsrc", self.src_pos, self.block_pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TDstId {
    pub dst_pos: u32,
    pub block_pos: u32,
}

impl Display for TDstId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_tdst", self.dst_pos, self.block_pos)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LutId(pub usize);

impl Display for LutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lut@{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TImmId {
    pub imm_pos: u32,
    pub block_pos: u32,
}

impl Display for TImmId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_timm", self.imm_pos, self.block_pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HpuInstructionSet {
    AddCt,
    SubCt,
    Mac { cst: Immediate },
    AddPt,
    SubPt,
    PtSub,
    MulPt,
    AddCst { cst: Immediate },
    SubCst { cst: Immediate },
    CstSub { cst: Immediate },
    MulCst { cst: Immediate },
    ImmLd { from: TImmId },
    DstSt { to: TDstId },
    SrcLd { from: TSrcId },
    Pbs { lut: LutId },
    Pbs2 { lut: LutId },
    Pbs4 { lut: LutId },
    Pbs8 { lut: LutId },
    PbsF { lut: LutId },
    Pbs2F { lut: LutId },
    Pbs4F { lut: LutId },
    Pbs8F { lut: LutId },
    Batch { block: Box<IR<HpuLang>> },
    BatchArg { pos: u8, ty: HpuTypeSystem },
    BatchRet { pos: u8, ty: HpuTypeSystem },
}

impl Display for HpuInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HpuInstructionSet::AddCt => write!(f, "add_ct"),
            HpuInstructionSet::SubCt => write!(f, "sub_ct"),
            HpuInstructionSet::Mac { cst } => write!(f, "mac<{cst}>"),
            HpuInstructionSet::AddPt => write!(f, "add_pt"),
            HpuInstructionSet::SubPt => write!(f, "sub_pt"),
            HpuInstructionSet::PtSub => write!(f, "pt_sub"),
            HpuInstructionSet::MulPt => write!(f, "mul_pt"),
            HpuInstructionSet::AddCst { cst } => write!(f, "add_cst<{cst}>"),
            HpuInstructionSet::SubCst { cst } => write!(f, "subs_cst<{cst}>"),
            HpuInstructionSet::CstSub { cst } => write!(f, "cst_sub<{cst}>"),
            HpuInstructionSet::MulCst { cst } => write!(f, "mul_cst<{cst}>"),
            HpuInstructionSet::ImmLd { from } => write!(f, "imm_ld<{from}>"),
            HpuInstructionSet::SrcLd { from } => write!(f, "src_ld<{from}>"),
            HpuInstructionSet::DstSt { to } => write!(f, "dst_st<{to}>"),
            HpuInstructionSet::Pbs { lut } => write!(f, "pbs<{lut}>"),
            HpuInstructionSet::Pbs2 { lut } => write!(f, "pbs_2<{lut}>"),
            HpuInstructionSet::Pbs4 { lut } => write!(f, "pbs_4<{lut}>"),
            HpuInstructionSet::Pbs8 { lut } => write!(f, "pbs_8<{lut}>"),
            HpuInstructionSet::PbsF { lut } => write!(f, "pbs_f<{lut}>"),
            HpuInstructionSet::Pbs2F { lut } => write!(f, "pbs_2f<{lut}>"),
            HpuInstructionSet::Pbs4F { lut } => write!(f, "pbs_4f<{lut}>"),
            HpuInstructionSet::Pbs8F { lut } => write!(f, "pbs_8f<{lut}>"),
            HpuInstructionSet::Batch { block, .. } => {
                write!(f, "batch {{\n        {}}}", block.format().to_string())
            }
            HpuInstructionSet::BatchArg { pos, ty } => write!(f, "batch_arg<{pos}, {ty}>"),
            HpuInstructionSet::BatchRet { pos, ty } => write!(f, "batch_ret<{pos}, {ty}>"),
        }
    }
}
impl DialectInstructionSet for HpuInstructionSet {
    type TypeSystem = HpuTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use HpuTypeSystem::*;
        match self {
            HpuInstructionSet::AddCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            HpuInstructionSet::SubCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            HpuInstructionSet::Mac { .. } => sig![(CtRegister, CtRegister) -> (CtRegister)],
            HpuInstructionSet::AddPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            HpuInstructionSet::SubPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            HpuInstructionSet::PtSub => sig![(PtImmediate, CtRegister) -> (CtRegister)],
            HpuInstructionSet::MulPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            HpuInstructionSet::AddCst { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::SubCst { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::CstSub { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::MulCst { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::DstSt { .. } => sig![(CtRegister) -> ()],
            HpuInstructionSet::SrcLd { .. } => sig![() -> (CtRegister)],
            HpuInstructionSet::ImmLd { .. } => sig![() -> (PtImmediate)],
            HpuInstructionSet::Pbs { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::Pbs2 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            HpuInstructionSet::Pbs4 { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::Pbs8 { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::PbsF { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::Pbs2F { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            HpuInstructionSet::Pbs4F { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::Pbs8F { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::Batch { block } => {
                let mut inputs = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_operation() {
                        HpuInstructionSet::BatchArg { pos, ty } => Some((pos, ty)),
                        _ => None,
                    })
                    .cosvec();
                inputs.sort_unstable_by_key(|a| a.0);
                let inputs = inputs.into_iter().map(|a| a.1).cosvec();
                let mut outputs = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_operation() {
                        HpuInstructionSet::BatchRet { pos, ty } => Some((pos, ty)),
                        _ => None,
                    })
                    .cosvec();
                outputs.sort_unstable_by_key(|a| a.0);
                let outputs = outputs.into_iter().map(|a| a.1).cosvec();
                Signature(inputs, outputs)
            }
            HpuInstructionSet::BatchArg { ty, .. } => sig![() -> (ty.clone())],
            HpuInstructionSet::BatchRet { ty, .. } => sig![(ty.clone()) -> ()],
        }
    }
}
