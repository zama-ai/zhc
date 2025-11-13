use std::fmt::Display;

pub const MASK_NONE: usize = usize::MAX;
pub const MASK_PBS2: usize = usize::MAX << 1;
pub const MASK_PBS4: usize = usize::MAX << 2;
pub const MASK_PBS8: usize = usize::MAX << 3;

use serde::Serialize;
#[derive(Debug, Clone, Eq)]
pub enum Argument {
    Immediate { val: usize },
    Memory { addr: usize },
    Register { mask: usize, addr: usize },
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
            _ => unreachable!()
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum RawDOp {
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
    SYNC,
}

impl Display for RawDOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawDOp::ADD { dst, src1, src2 } => write!(f, "ADD   {}, {}, {}", dst, src1, src2),
            RawDOp::SUB { dst, src1, src2 } => write!(f, "SUB   {}, {}, {}", dst, src1, src2),
            RawDOp::MAC {
                dst,
                src1,
                src2,
                cst,
            } => write!(f, "MAC   {}, {}, {}, {}", dst, src1, src2, cst),
            RawDOp::ADDS { dst, src, cst } => write!(f, "ADDS  {}, {}, {}", dst, src, cst),
            RawDOp::SUBS { dst, src, cst } => write!(f, "SUBS  {}, {}, {}", dst, src, cst),
            RawDOp::SSUB { dst, src, cst } => write!(f, "SSUB  {}, {}, {}", dst, src, cst),
            RawDOp::MULS { dst, src, cst } => write!(f, "MULS  {}, {}, {}", dst, src, cst),
            RawDOp::LD { dst, src } => write!(f, "LD    {}, {}", dst, src),
            RawDOp::ST { dst, src } => write!(f, "ST    {}, {}", dst, src),
            RawDOp::PBS { dst, src } => write!(f, "PBS   {}, {}", dst, src),
            RawDOp::PBS_ML2 { dst, src } => write!(f, "PBS2  {}, {}", dst, src),
            RawDOp::PBS_ML4 { dst, src } => write!(f, "PBS4  {}, {}", dst, src),
            RawDOp::PBS_ML8 { dst, src } => write!(f, "PBS8  {}, {}", dst, src),
            RawDOp::PBS_F { dst, src } => write!(f, "PBSF  {}, {}", dst, src),
            RawDOp::PBS_ML2_F { dst, src } => write!(f, "PBS2F {}, {}", dst, src),
            RawDOp::PBS_ML4_F { dst, src } => write!(f, "PBS4F {}, {}", dst, src),
            RawDOp::PBS_ML8_F { dst, src } => write!(f, "PBS8F {}, {}", dst, src),
            RawDOp::SYNC => write!(f, "SYNC"),
        }
    }
}

impl RawDOp {
    pub fn is_pbs_flush(&self) -> bool {
        match self {
            RawDOp::PBS_F { .. }
            | RawDOp::PBS_ML2_F { .. }
            | RawDOp::PBS_ML4_F { .. }
            | RawDOp::PBS_ML8_F { .. } => true,
            _ => false,
        }
    }

    pub fn affinity(&self) -> Affinity {
        use Affinity::*;
        match self {
            RawDOp::ADD { .. } => Alu,
            RawDOp::SUB { .. } => Alu,
            RawDOp::MAC { .. } => Alu,
            RawDOp::ADDS { .. } => Alu,
            RawDOp::SUBS { .. } => Alu,
            RawDOp::SSUB { .. } => Alu,
            RawDOp::MULS { .. } => Alu,
            RawDOp::LD { .. } => Mem,
            RawDOp::ST { .. } => Mem,
            RawDOp::PBS { .. } => Pbs,
            RawDOp::PBS_ML2 { .. } => Pbs,
            RawDOp::PBS_ML4 { .. } => Pbs,
            RawDOp::PBS_ML8 { .. } => Pbs,
            RawDOp::PBS_F { .. } => Pbs,
            RawDOp::PBS_ML2_F { .. } => Pbs,
            RawDOp::PBS_ML4_F { .. } => Pbs,
            RawDOp::PBS_ML8_F { .. } => Pbs,
            RawDOp::SYNC => Ctl,
        }
    }

    pub fn has_source(&self, arg: &Argument) -> bool {
        match self {
            RawDOp::ADD { src1, src2,  .. } => arg == src1 || arg == src2,
            RawDOp::SUB { src1, src2, .. } => arg == src1 || arg == src2,
            RawDOp::MAC { src1, src2, .. } => arg == src1 || arg == src2,
            RawDOp::ADDS { src, .. } => arg == src,
            RawDOp::SUBS { src, .. } => arg == src,
            RawDOp::SSUB { src, .. } => arg == src,
            RawDOp::MULS {  src, .. } => arg == src,
            RawDOp::LD { src, .. } => arg == src,
            RawDOp::ST { src, .. } => arg == src,
            RawDOp::PBS { src, .. } => arg == src,
            RawDOp::PBS_ML2 { src, .. } => arg == src,
            RawDOp::PBS_ML4 { src, .. } => arg == src,
            RawDOp::PBS_ML8 { src, .. } => arg == src,
            RawDOp::PBS_F { src, .. } => arg == src,
            RawDOp::PBS_ML2_F { src, .. } => arg == src,
            RawDOp::PBS_ML4_F { src, .. } => arg == src,
            RawDOp::PBS_ML8_F { src, .. } => arg == src,
            RawDOp::SYNC => false,
        }
    }

    pub fn get_dst(&self) -> Option<&Argument> {
        match self {
            RawDOp::ADD { dst,  .. } => Some(dst),
            RawDOp::SUB { dst, .. } => Some(dst),
            RawDOp::MAC { dst, .. } => Some(dst),
            RawDOp::ADDS { dst, .. } => Some(dst),
            RawDOp::SUBS { dst, .. } => Some(dst),
            RawDOp::SSUB { dst, .. } => Some(dst),
            RawDOp::MULS { dst, .. } => Some(dst),
            RawDOp::LD { dst, .. } => Some(dst),
            RawDOp::ST { dst, .. } => Some(dst),
            RawDOp::PBS { dst, .. } => Some(dst),
            RawDOp::PBS_ML2 { dst, .. } => Some(dst),
            RawDOp::PBS_ML4 { dst, .. } => Some(dst),
            RawDOp::PBS_ML8 { dst, .. } => Some(dst),
            RawDOp::PBS_F { dst, .. } => Some(dst),
            RawDOp::PBS_ML2_F { dst, .. } => Some(dst),
            RawDOp::PBS_ML4_F { dst, .. } => Some(dst),
            RawDOp::PBS_ML8_F { dst, .. } => Some(dst),
            RawDOp::SYNC => None,
        }
    }

    pub fn get_src1(&self) -> Option<&Argument> {
        match self {
            RawDOp::ADD { src1, .. } => Some(src1),
            RawDOp::SUB { src1, .. } => Some(src1),
            RawDOp::MAC { src1, .. } => Some(src1),
            RawDOp::ADDS { src, .. } => Some(src),
            RawDOp::SUBS { src, .. } => Some(src),
            RawDOp::SSUB { src, .. } => Some(src),
            RawDOp::MULS {  src, .. } => Some(src),
            RawDOp::LD { src, .. } => Some(src),
            RawDOp::ST { src, .. } => Some(src),
            RawDOp::PBS { src, .. } => Some(src),
            RawDOp::PBS_ML2 { src, .. } => Some(src),
            RawDOp::PBS_ML4 { src, .. } => Some(src),
            RawDOp::PBS_ML8 { src, .. } => Some(src),
            RawDOp::PBS_F { src, .. } => Some(src),
            RawDOp::PBS_ML2_F { src, .. } => Some(src),
            RawDOp::PBS_ML4_F { src, .. } => Some(src),
            RawDOp::PBS_ML8_F { src, .. } => Some(src),
            RawDOp::SYNC => None,
        }
    }

    pub fn get_src2(&self) -> Option<&Argument> {
        match self {
            RawDOp::ADD { src2, .. } => Some(src2),
            RawDOp::SUB { src2, .. } => Some(src2),
            RawDOp::MAC { src2, .. } => Some(src2),
            RawDOp::ADDS { .. } => None,
            RawDOp::SUBS { .. } => None,
            RawDOp::SSUB { .. } => None,
            RawDOp::MULS { .. } => None,
            RawDOp::LD { .. } => None,
            RawDOp::ST { .. } => None,
            RawDOp::PBS { .. } => None,
            RawDOp::PBS_ML2 { .. } => None,
            RawDOp::PBS_ML4 { .. } => None,
            RawDOp::PBS_ML8 { .. } => None,
            RawDOp::PBS_F { .. } => None,
            RawDOp::PBS_ML2_F { .. } => None,
            RawDOp::PBS_ML4_F { .. } => None,
            RawDOp::PBS_ML8_F { .. } => None,
            RawDOp::SYNC => None,
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct DOpId(pub u16);

impl Display for DOpId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl Serialize for DOpId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DOp {
    pub raw: RawDOp,
    pub id: DOpId,
}

impl Display for DOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.raw)
    }
}

impl Serialize for DOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
