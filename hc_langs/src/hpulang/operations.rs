use std::fmt::{Debug, Display};

use hc_ir::{DialectOperations, IR, Signature, sig};
use hc_utils::iter::CollectInSmallVec;

use super::{Hpulang, types::Types};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Immediate(pub usize);

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
pub enum Operations {
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
    Batch { block: Box<IR<Hpulang>> },
    BatchArg { pos: u8, ty: Types },
    BatchRet { pos: u8, ty: Types },
}

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::AddCt => write!(f, "add_ct"),
            Operations::SubCt => write!(f, "sub_ct"),
            Operations::Mac { cst } => write!(f, "mac<{cst}>"),
            Operations::AddPt => write!(f, "add_pt"),
            Operations::SubPt => write!(f, "sub_pt"),
            Operations::PtSub => write!(f, "pt_sub"),
            Operations::MulPt => write!(f, "mul_pt"),
            Operations::AddCst { cst } => write!(f, "add_cst<{cst}>"),
            Operations::SubCst { cst } => write!(f, "subs_cst<{cst}>"),
            Operations::CstSub { cst } => write!(f, "cst_sub<{cst}>"),
            Operations::MulCst { cst } => write!(f, "mul_cst<{cst}>"),
            Operations::ImmLd { from } => write!(f, "imm_ld<{from}>"),
            Operations::SrcLd { from } => write!(f, "src_ld<{from}>"),
            Operations::DstSt { to } => write!(f, "dst_st<{to}>"),
            Operations::Pbs { lut } => write!(f, "pbs<{lut}>"),
            Operations::Pbs2 { lut } => write!(f, "pbs_2<{lut}>"),
            Operations::Pbs4 { lut } => write!(f, "pbs_4<{lut}>"),
            Operations::Pbs8 { lut } => write!(f, "pbs_8<{lut}>"),
            Operations::PbsF { lut } => write!(f, "pbs_f<{lut}>"),
            Operations::Pbs2F { lut } => write!(f, "pbs_2f<{lut}>"),
            Operations::Pbs4F { lut } => write!(f, "pbs_4f<{lut}>"),
            Operations::Pbs8F { lut } => write!(f, "pbs_8f<{lut}>"),
            Operations::Batch { block, .. } => write!(
                f,
                "batch {{\n        {}}}",
                block
                    .to_string()
                    .replace("\n", "\n        ")
                    .strip_suffix("        ")
                    .unwrap()
            ),
            Operations::BatchArg { pos, ty } => write!(f, "batch_arg<{pos}, {ty}>"),
            Operations::BatchRet { pos, ty } => write!(f, "batch_ret<{pos}, {ty}>"),
        }
    }
}
impl DialectOperations for Operations {
    type Types = Types;

    fn get_signature(&self) -> Signature<Self::Types> {
        use Types::*;
        match self {
            Operations::AddCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            Operations::SubCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            Operations::Mac { .. } => sig![(CtRegister, CtRegister) -> (CtRegister)],
            Operations::AddPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            Operations::SubPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            Operations::PtSub => sig![(PtImmediate, CtRegister) -> (CtRegister)],
            Operations::MulPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            Operations::AddCst { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::SubCst { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::CstSub { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::MulCst { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::DstSt { .. } => sig![(CtRegister) -> ()],
            Operations::SrcLd { .. } => sig![() -> (CtRegister)],
            Operations::ImmLd { .. } => sig![() -> (PtImmediate)],
            Operations::Pbs { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::Pbs2 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            Operations::Pbs4 { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            Operations::Pbs8 { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            Operations::PbsF { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::Pbs2F { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            Operations::Pbs4F { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            Operations::Pbs8F { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            Operations::Batch { block } => {
                let mut inputs = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_operation() {
                        Operations::BatchArg { pos, ty } => Some((pos, ty)),
                        _ => None,
                    })
                    .cosvec();
                inputs.sort_unstable_by_key(|a| a.0);
                let inputs = inputs.into_iter().map(|a| a.1).cosvec();
                let mut outputs = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_operation() {
                        Operations::BatchRet { pos, ty } => Some((pos, ty)),
                        _ => None,
                    })
                    .cosvec();
                outputs.sort_unstable_by_key(|a| a.0);
                let outputs = outputs.into_iter().map(|a| a.1).cosvec();
                Signature(inputs, outputs)
            }
            Operations::BatchArg { ty, .. } => sig![() -> (ty.clone())],
            Operations::BatchRet { ty, .. } => sig![(ty.clone()) -> ()],
        }
    }
}
