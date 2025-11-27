use super::types::Types;
use hpuc_ir::{DialectOperations, Signature, sig};
use serde::Serialize;
use std::fmt::{Debug, Display};

pub const MASK_NONE: usize = usize::MAX;
pub const MASK_PBS2: usize = usize::MAX << 1;
pub const MASK_PBS4: usize = usize::MAX << 2;
pub const MASK_PBS8: usize = usize::MAX << 3;

/// A type representing the different kinds of operands that may occur in a dop stream.
///
/// # Note:
///
/// The doplang dialect is meant to support both pathed and unpatched streams:
/// + Unpatched streams are exported to the hpu memory. They support variable ciphertext and plaintext, which are meant to be patched on the fly by the ucore.
/// + Patched streams are streamed by the ucore to the hpu. They contain memory adresses instead of ciphertext variables, and constant immediates instead of plaintext variables.
///
/// Essentially, supporting unpatched streams allows to generate programs for the hpu, and supporting patched streams allows to load execution traces from the hpu.
#[derive(Debug, Clone, Eq, Hash)]
pub enum Argument {
    /// A constant immediate plaintext.
    PtConst { val: usize },
    /// A ciphertext located on the heap.
    CtHeap { addr: usize },
    /// A ciphertext located in the io memory.
    CtIo { addr: usize },
    /// A ciphertext variable. Patched to a CtMemory by the ucore.
    CtVar { id: usize, block: usize },
    /// A plaintext variable. Patched to a PtConst by the ucore.
    PtVar { id: usize, block: usize },
    /// A ciphertext register.
    CtReg { mask: usize, addr: usize },
}

impl Argument {
    pub fn ct_reg(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_NONE,
            addr: addr.into(),
        }
    }

    pub fn ct_reg2(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_PBS2,
            addr: addr.into(),
        }
    }

    pub fn ct_reg4(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_PBS4,
            addr: addr.into(),
        }
    }

    pub fn ct_reg8(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_PBS8,
            addr: addr.into(),
        }
    }

    pub fn ct_var(id: usize, block: usize) -> Self {
        Argument::CtVar { id, block }
    }

    pub fn pt_var(id: usize, block: usize) -> Self {
        Argument::PtVar { id, block }
    }

    pub fn ct_heap(heap_slot: usize) -> Self {
        Argument::CtHeap { addr: heap_slot }
    }

    pub fn ct_io(io_slot: usize) -> Self {
        Argument::CtIo { addr: io_slot }
    }

    pub fn pt_const(val: usize) -> Self {
        Argument::PtConst { val }
    }
}

impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Argument::PtConst { val: lhs }, Argument::PtConst { val: rhs }) => lhs == rhs,
            (Argument::CtHeap { addr: lhs }, Argument::CtHeap { addr: rhs }) => lhs == rhs,
            (Argument::CtIo { addr: lhs }, Argument::CtIo { addr: rhs }) => lhs == rhs,
            (
                Argument::CtReg {
                    mask: lhs_m,
                    addr: lhs,
                },
                Argument::CtReg {
                    mask: rhs_m,
                    addr: rhs,
                },
            ) => ((lhs ^ rhs) & (lhs_m & rhs_m)) == 0,
            (
                Argument::CtVar {
                    id: lhs_id,
                    block: lhs_block,
                },
                Argument::CtVar {
                    id: rhs_id,
                    block: rhs_block,
                },
            ) => (lhs_id, lhs_block) == (rhs_id, rhs_block),
            (
                Argument::PtVar {
                    id: lhs_id,
                    block: lhs_block,
                },
                Argument::PtVar {
                    id: rhs_id,
                    block: rhs_block,
                },
            ) => (lhs_id, lhs_block) == (rhs_id, rhs_block),
            _ => false,
        }
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::PtConst { val } => write!(f, "PT_I({})", val),
            Argument::CtHeap { addr } => write!(f, "CT_H({})", addr),
            Argument::CtIo { addr } => write!(f, "CT_IO({})", addr),
            Argument::CtReg { mask, addr } if *mask == MASK_NONE => write!(f, "R({})", addr),
            Argument::CtReg { mask, addr } if *mask == MASK_PBS2 => write!(f, "R({}, 2)", addr),
            Argument::CtReg { mask, addr } if *mask == MASK_PBS4 => write!(f, "R({}, 4)", addr),
            Argument::CtReg { mask, addr } if *mask == MASK_PBS8 => write!(f, "R({}, 8)", addr),
            Argument::CtVar { id, block } => write!(f, "TC({}, {})", id, block),
            Argument::PtVar { id, block } => write!(f, "TI({}, {})", id, block),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Affinity {
    Alu,
    Mem,
    Pbs,
    Ctl,
}

impl Display for Affinity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Affinity::Alu => write!(f, "Alu"),
            Affinity::Mem => write!(f, "Mem"),
            Affinity::Pbs => write!(f, "Pbs"),
            Affinity::Ctl => write!(f, "Ctl"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Operations {
    ADD {
        dst: Argument,
        src1: Argument,
        src2: Argument,
    },
    SUB {
        dst: Argument,
        src1: Argument,
        src2: Argument,
    },
    MAC {
        dst: Argument,
        src1: Argument,
        src2: Argument,
        cst: Argument,
    },
    ADDS {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    SUBS {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    SSUB {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    MULS {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    LD {
        dst: Argument,
        src: Argument,
    },
    ST {
        dst: Argument,
        src: Argument,
    },
    PBS {
        dst: Argument,
        src: Argument,
    },
    PBS_ML2 {
        dst: Argument,
        src: Argument,
    },
    PBS_ML4 {
        dst: Argument,
        src: Argument,
    },
    PBS_ML8 {
        dst: Argument,
        src: Argument,
    },
    PBS_F {
        dst: Argument,
        src: Argument,
    },
    PBS_ML2_F {
        dst: Argument,
        src: Argument,
    },
    PBS_ML4_F {
        dst: Argument,
        src: Argument,
    },
    PBS_ML8_F {
        dst: Argument,
        src: Argument,
    },
    _INIT,
    SYNC,
}

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operations::*;
        match self {
            ADD { dst, src1, src2 } => write!(f, "ADD<{}, {}, {}>", dst, src1, src2),
            SUB { dst, src1, src2 } => write!(f, "SUB<{}, {}, {}>", dst, src1, src2),
            MAC {
                dst,
                src1,
                src2,
                cst,
            } => write!(f, "MAC<{}, {}, {}, {}>", dst, src1, src2, cst),
            ADDS { dst, src, cst } => write!(f, "ADDS<{}, {}, {}>", dst, src, cst),
            SUBS { dst, src, cst } => write!(f, "SUBS<{}, {}, {}>", dst, src, cst),
            SSUB { dst, src, cst } => write!(f, "SSUB<{}, {}, {}>", dst, src, cst),
            MULS { dst, src, cst } => write!(f, "MULS<{}, {}, {}>", dst, src, cst),
            LD { dst, src } => write!(f, "LD<{}, {}>", dst, src),
            ST { dst, src } => write!(f, "ST<{}, {}>", dst, src),
            PBS { dst, src } => write!(f, "PBS<{}, {}>", dst, src),
            PBS_ML2 { dst, src } => write!(f, "PBS2<{}, {}>", dst, src),
            PBS_ML4 { dst, src } => write!(f, "PBS4<{}, {}>", dst, src),
            PBS_ML8 { dst, src } => write!(f, "PBS8<{}, {}>", dst, src),
            PBS_F { dst, src } => write!(f, "PBSF<{}, {}>", dst, src),
            PBS_ML2_F { dst, src } => write!(f, "PBS2F<{}, {}>", dst, src),
            PBS_ML4_F { dst, src } => write!(f, "PBS4F<{}, {}>", dst, src),
            PBS_ML8_F { dst, src } => write!(f, "PBS8F<{}, {}>", dst, src),
            _INIT => write!(f, "_INIT"),
            SYNC => write!(f, "SYNC"),
        }
    }
}

impl Operations {
    pub fn is_pbs_flush(&self) -> bool {
        use Operations::*;
        match self {
            PBS_F { .. } | PBS_ML2_F { .. } | PBS_ML4_F { .. } | PBS_ML8_F { .. } => true,
            _ => false,
        }
    }

    pub fn affinity(&self) -> Affinity {
        use Affinity::*;
        use Operations::*;
        match self {
            ADD { .. } => Alu,
            SUB { .. } => Alu,
            MAC { .. } => Alu,
            ADDS { .. } => Alu,
            SUBS { .. } => Alu,
            SSUB { .. } => Alu,
            MULS { .. } => Alu,
            LD { .. } => Mem,
            ST { .. } => Mem,
            PBS { .. } => Pbs,
            PBS_ML2 { .. } => Pbs,
            PBS_ML4 { .. } => Pbs,
            PBS_ML8 { .. } => Pbs,
            PBS_F { .. } => Pbs,
            PBS_ML2_F { .. } => Pbs,
            PBS_ML4_F { .. } => Pbs,
            PBS_ML8_F { .. } => Pbs,
            _INIT => Ctl,
            SYNC => Ctl,
        }
    }

    pub fn has_source(&self, arg: &Argument) -> bool {
        use Operations::*;
        match self {
            ADD { src1, src2, .. } => arg == src1 || arg == src2,
            SUB { src1, src2, .. } => arg == src1 || arg == src2,
            MAC { src1, src2, .. } => arg == src1 || arg == src2,
            ADDS { src, .. } => arg == src,
            SUBS { src, .. } => arg == src,
            SSUB { src, .. } => arg == src,
            MULS { src, .. } => arg == src,
            LD { src, .. } => arg == src,
            ST { src, .. } => arg == src,
            PBS { src, .. } => arg == src,
            PBS_ML2 { src, .. } => arg == src,
            PBS_ML4 { src, .. } => arg == src,
            PBS_ML8 { src, .. } => arg == src,
            PBS_F { src, .. } => arg == src,
            PBS_ML2_F { src, .. } => arg == src,
            PBS_ML4_F { src, .. } => arg == src,
            PBS_ML8_F { src, .. } => arg == src,
            _INIT => false,
            SYNC => false,
        }
    }

    pub fn get_dst(&self) -> Option<&Argument> {
        use Operations::*;
        match self {
            ADD { dst, .. } => Some(dst),
            SUB { dst, .. } => Some(dst),
            MAC { dst, .. } => Some(dst),
            ADDS { dst, .. } => Some(dst),
            SUBS { dst, .. } => Some(dst),
            SSUB { dst, .. } => Some(dst),
            MULS { dst, .. } => Some(dst),
            LD { dst, .. } => Some(dst),
            ST { dst, .. } => Some(dst),
            PBS { dst, .. } => Some(dst),
            PBS_ML2 { dst, .. } => Some(dst),
            PBS_ML4 { dst, .. } => Some(dst),
            PBS_ML8 { dst, .. } => Some(dst),
            PBS_F { dst, .. } => Some(dst),
            PBS_ML2_F { dst, .. } => Some(dst),
            PBS_ML4_F { dst, .. } => Some(dst),
            PBS_ML8_F { dst, .. } => Some(dst),
            _INIT => None,
            SYNC => None,
        }
    }

    pub fn get_src1(&self) -> Option<&Argument> {
        use Operations::*;
        match self {
            ADD { src1, .. } => Some(src1),
            SUB { src1, .. } => Some(src1),
            MAC { src1, .. } => Some(src1),
            ADDS { src, .. } => Some(src),
            SUBS { src, .. } => Some(src),
            SSUB { src, .. } => Some(src),
            MULS { src, .. } => Some(src),
            LD { src, .. } => Some(src),
            ST { src, .. } => Some(src),
            PBS { src, .. } => Some(src),
            PBS_ML2 { src, .. } => Some(src),
            PBS_ML4 { src, .. } => Some(src),
            PBS_ML8 { src, .. } => Some(src),
            PBS_F { src, .. } => Some(src),
            PBS_ML2_F { src, .. } => Some(src),
            PBS_ML4_F { src, .. } => Some(src),
            PBS_ML8_F { src, .. } => Some(src),
            _INIT => None,
            SYNC => None,
        }
    }

    pub fn get_src2(&self) -> Option<&Argument> {
        use Operations::*;
        match self {
            ADD { src2, .. } => Some(src2),
            SUB { src2, .. } => Some(src2),
            MAC { src2, .. } => Some(src2),
            ADDS { .. } => None,
            SUBS { .. } => None,
            SSUB { .. } => None,
            MULS { .. } => None,
            LD { .. } => None,
            ST { .. } => None,
            PBS { .. } => None,
            PBS_ML2 { .. } => None,
            PBS_ML4 { .. } => None,
            PBS_ML8 { .. } => None,
            PBS_F { .. } => None,
            PBS_ML2_F { .. } => None,
            PBS_ML4_F { .. } => None,
            PBS_ML8_F { .. } => None,
            _INIT => None,
            SYNC => None,
        }
    }
}

impl DialectOperations for Operations {
    type Types = Types;

    fn get_signature(&self) -> Signature<Self::Types> {
        use Operations::*;
        use Types::*;
        match self {
            ADD { .. }
            | SUB { .. }
            | MAC { .. }
            | ADDS { .. }
            | SUBS { .. }
            | SSUB { .. }
            | MULS { .. }
            | LD { .. }
            | ST { .. }
            | PBS { .. }
            | PBS_ML2 { .. }
            | PBS_ML4 { .. }
            | PBS_ML8 { .. }
            | PBS_F { .. }
            | PBS_ML2_F { .. }
            | PBS_ML4_F { .. }
            | PBS_ML8_F { .. } => sig![(Ctx(0)) -> (Ctx(0))],
            _INIT => sig![() -> (Ctx(0))],
            SYNC => sig![(Ctx(0)) -> ()],
        }
    }
}
