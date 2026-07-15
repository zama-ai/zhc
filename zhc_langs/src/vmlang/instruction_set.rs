use std::fmt::{Debug, Display};

use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature, sig};
use super::type_system::VmTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmInstructionSet {
    AddCt,
    SubCt,
    Mac { cst: u8 },
    AddPt,
    SubPt,
    PtSub,
    MulPt,
    AddCst { cst: u8 },
    SubCst { cst: u8 },
    CstSub { cst: u8 },
    MulCst { cst: u8 },
    CstCt { cst: u8 },
    ImmLd { from_pos: u32, from_block: u32 },
    DstSt { to_pos: u32, to_block: u32 },
    SrcLd { from_pos: u32, from_block: u32 },
    Ks,
    Pbs { lut: usize },
    Pbs2 { lut: usize },
}

impl VmInstructionSet {
    pub fn is_pbs(&self) -> bool {
        match self {
            VmInstructionSet::Pbs { .. }
            | VmInstructionSet::Pbs2 { .. } => true,
            _ => false,
        }
    }
}

impl Format for VmInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        use VmInstructionSet::*;
        match self {
            AddCt => write!(f, "add_ct"),
            SubCt => write!(f, "sub_ct"),
            Mac { cst } => write!(f, "mac<{cst}>"),
            AddPt => write!(f, "add_pt"),
            SubPt => write!(f, "sub_pt"),
            PtSub => write!(f, "pt_sub"),
            MulPt => write!(f, "mul_pt"),
            AddCst { cst } => write!(f, "add_cst<{cst}>"),
            SubCst { cst } => write!(f, "subs_cst<{cst}>"),
            CstSub { cst } => write!(f, "cst_sub<{cst}>"),
            MulCst { cst } => write!(f, "mul_cst<{cst}>"),
            CstCt { cst } => write!(f, "cst_ct<{cst}>"),
            ImmLd { from_block, from_pos } => write!(f, "imm_ld<{from_pos}, {from_block}>"),
            SrcLd { from_block, from_pos } => write!(f, "src_ld<{from_pos}, {from_block}>"),
            DstSt { to_block, to_pos } => write!(f, "dst_st<{to_pos}, {to_block}>"),
            Ks => write!(f, "ks"),
            Pbs { lut } => write!(f, "pbs<{lut}>"),
            Pbs2 { lut } => write!(f, "pbs_2<{lut}>"),
        }
    }
}

impl Display for VmInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DialectInstructionSet for VmInstructionSet {
    type TypeSystem = VmTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use VmInstructionSet::*;
        use VmTypeSystem::*;
        match self {
            AddCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            SubCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            Mac { .. } => sig![(CtRegister, CtRegister) -> (CtRegister)],
            AddPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            SubPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            PtSub => sig![(PtImmediate, CtRegister) -> (CtRegister)],
            MulPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            AddCst { .. } => sig![(CtRegister) -> (CtRegister)],
            SubCst { .. } => sig![(CtRegister) -> (CtRegister)],
            CstSub { .. } => sig![(CtRegister) -> (CtRegister)],
            MulCst { .. } => sig![(CtRegister) -> (CtRegister)],
            CstCt { .. } => sig![() -> (CtRegister)],
            DstSt { .. } => sig![(CtRegister) -> ()],
            SrcLd { .. } => sig![() -> (CtRegister)],
            ImmLd { .. } => sig![() -> (PtImmediate)],
            Pbs { .. } => sig![(CtRegister) -> (CtRegister)],
            Pbs2 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            Ks => sig![(CtRegister) -> (CtRegister)],
        }
    }
}
