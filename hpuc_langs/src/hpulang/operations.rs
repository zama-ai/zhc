use std::fmt::{Debug, Display};

use hpuc_ir::{sig, DialectOperations, Signature};

use super::types::Types;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Immediate(pub usize);

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_imm", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TSrcId{
    pub src_pos: usize,
    pub block_pos: usize
}

impl Display for TSrcId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_tsrc", self.src_pos, self.block_pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TDstId{
    pub dst_pos: usize,
    pub block_pos: usize
}

impl Display for TDstId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_tdst", self.dst_pos, self.block_pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct LutMemoryAdress(pub usize);

impl Display for LutMemoryAdress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lut@{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TImmId{
    pub imm_pos: usize,
    pub block_pos: usize
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
    HeapSt,
    HeapLd,
    Pbs { lut: LutMemoryAdress },
    Pbs2 { lut: LutMemoryAdress },
    Pbs4 { lut: LutMemoryAdress },
    Pbs8 { lut: LutMemoryAdress },
    PbsF { lut: LutMemoryAdress },
    Pbs2F { lut: LutMemoryAdress },
    Pbs4F { lut: LutMemoryAdress },
    Pbs8F { lut: LutMemoryAdress },
}

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::AddCt => write!(f, "add_ct"),
            Operations::SubCt => write!(f, "sub_ct"),
            Operations::Mac { cst }  => write!(f, "mac<{}>", cst),
            Operations::AddPt => write!(f, "add_pt"),
            Operations::SubPt => write!(f, "sub_pt"),
            Operations::PtSub => write!(f, "pt_sub"),
            Operations::MulPt => write!(f, "mul_pt"),
            Operations::AddCst { cst } => write!(f, "add_cst<{}>", cst),
            Operations::SubCst { cst } => write!(f, "subs_cst<{}>", cst),
            Operations::CstSub { cst } => write!(f, "cst_sub<{}>", cst),
            Operations::MulCst { cst } => write!(f, "mul_cst<{}>", cst),
            Operations::ImmLd { from } => write!(f, "imm_ld<{}>", from),
            Operations::SrcLd { from } => write!(f, "src_ld<{}>", from),
            Operations::DstSt { to } => write!(f, "dst_st<{}>", to),
            Operations::HeapLd => write!(f, "heap_ld"),
            Operations::HeapSt => write!(f, "heap_st"),
            Operations::Pbs { lut } => write!(f, "pbs<{}>", lut),
            Operations::Pbs2 { lut } => write!(f, "pbs_2<{}>", lut),
            Operations::Pbs4 { lut } => write!(f, "pbs_4<{}>", lut),
            Operations::Pbs8 { lut } => write!(f, "pbs_8<{}>", lut),
            Operations::PbsF { lut } => write!(f, "pbs_f<{}>", lut),
            Operations::Pbs2F { lut } => write!(f, "pbs_2f<{}>", lut),
            Operations::Pbs4F { lut } => write!(f, "pbs_4f<{}>", lut),
            Operations::Pbs8F { lut } => write!(f, "pbs_8f<{}>", lut),
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
            Operations::HeapSt => sig![(CtRegister) -> (CtHeap)],
            Operations::HeapLd => sig![(CtHeap) -> (CtRegister)],
            Operations::Pbs { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::Pbs2 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            Operations::Pbs4 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)],
            Operations::Pbs8 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)],
            Operations::PbsF { .. } => sig![(CtRegister) -> (CtRegister)],
            Operations::Pbs2F { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            Operations::Pbs4F { .. } => sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)],
            Operations::Pbs8F { .. } => sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)],
        }
    }
}
