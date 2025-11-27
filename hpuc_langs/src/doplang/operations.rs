use super::types::Types;
use hpuc_ir::{DialectOperations, Signature, sig};
use serde::Serialize;
use std::fmt::{Debug, Display};

pub const MASK_NONE: usize = usize::MAX;
pub const MASK_PBS2: usize = usize::MAX << 1;
pub const MASK_PBS4: usize = usize::MAX << 2;
pub const MASK_PBS8: usize = usize::MAX << 3;

#[derive(Debug, Clone, Eq, Hash)]
pub enum Argument {
    Immediate { val: usize },
    Memory { addr: usize },
    Register { mask: usize, addr: usize },
}

impl Argument {
    pub const IMM_ZERO: Self = Argument::Immediate { val: 0 };
    pub const MEM_ZERO: Self = Argument::Memory { addr: 0 };
    pub fn reg(addr: impl Into<usize>) -> Self {
        Argument::Register {
            mask: MASK_NONE,
            addr: addr.into(),
        }
    }
    pub fn reg2(addr: impl Into<usize>) -> Self {
        Argument::Register {
            mask: MASK_PBS2,
            addr: addr.into(),
        }
    }
    pub fn reg4(addr: impl Into<usize>) -> Self {
        Argument::Register {
            mask: MASK_PBS4,
            addr: addr.into(),
        }
    }
    pub fn reg8(addr: impl Into<usize>) -> Self {
        Argument::Register {
            mask: MASK_PBS8,
            addr: addr.into(),
        }
    }
}

impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Argument::Immediate { val: lhs }, Argument::Immediate { val: rhs }) if lhs == rhs => {
                true
            }
            (Argument::Memory { addr: lhs }, Argument::Memory { addr: rhs }) if lhs == rhs => true,
            (
                Argument::Register {
                    mask: lhs_m,
                    addr: lhs,
                },
                Argument::Register {
                    mask: rhs_m,
                    addr: rhs,
                },
            ) => ((lhs ^ rhs) & (lhs_m & rhs_m)) == 0,
            _ => false,
        }
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Immediate { val } => write!(f, "I({})", val),
            Argument::Memory { addr } => write!(f, "M({})", addr),
            Argument::Register { mask, addr } if *mask == MASK_NONE => write!(f, "R({})", addr),
            Argument::Register { mask, addr } if *mask == MASK_PBS2 => write!(f, "R({}, 2)", addr),
            Argument::Register { mask, addr } if *mask == MASK_PBS4 => write!(f, "R({}, 4)", addr),
            Argument::Register { mask, addr } if *mask == MASK_PBS8 => write!(f, "R({}, 8)", addr),
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
            | PBS_ML8_F { .. } => sig![(Ctx) -> (Ctx)],
            _INIT => sig![() -> (Ctx)],
            SYNC => sig![(Ctx) -> ()],
        }
    }
}
