use crate::hpulang::LutId;

use super::type_system::DopTypeSystem;
use serde::Serialize;
use std::fmt::{Debug, Display};
use zhc_ir::{DialectInstructionSet, Signature, sig};

const LUT_ALIASES: [&str; 76] = [
    "None",
    "MsgOnly",
    "CarryOnly",
    "CarryInMsg",
    "MultCarryMsg",
    "MultCarryMsgLsb",
    "MultCarryMsgMsb",
    "BwAnd",
    "BwOr",
    "BwXor",
    "CmpSign",
    "CmpReduce",
    "CmpGt",
    "CmpGte",
    "CmpLt",
    "CmpLte",
    "CmpEq",
    "CmpNeq",
    "ManyGenProp",
    "ReduceCarry2",
    "ReduceCarry3",
    "ReduceCarryPad",
    "GenPropAdd",
    "IfTrueZeroed",
    "IfFalseZeroed",
    "Ripple2GenProp",
    "ManyCarryMsg",
    "CmpGtMrg",
    "CmpGteMrg",
    "CmpLtMrg",
    "CmpLteMrg",
    "CmpEqMrg",
    "CmpNeqMrg",
    "IsSome",
    "CarryIsSome",
    "CarryIsNone",
    "MultCarryMsgIsSome",
    "MultCarryMsgMsbIsSome",
    "IsNull",
    "IsNullPos1",
    "NotNull",
    "MsgNotNull",
    "MsgNotNullPos1",
    "ManyMsgSplitShift1",
    "SolvePropGroupFinal0",
    "SolvePropGroupFinal1",
    "SolvePropGroupFinal2",
    "ExtractPropGroup0",
    "ExtractPropGroup1",
    "ExtractPropGroup2",
    "ExtractPropGroup3",
    "SolveProp",
    "SolvePropCarry",
    "SolveQuotient",
    "SolveQuotientPos1",
    "IfPos1FalseZeroed",
    "IfPos1FalseZeroedMsgCarry1",
    "ShiftLeftByCarryPos0Msg",
    "ShiftLeftByCarryPos0MsgNext",
    "ShiftRightByCarryPos0Msg",
    "ShiftRightByCarryPos0MsgNext",
    "IfPos0TrueZeroed",
    "IfPos0FalseZeroed",
    "IfPos1TrueZeroed",
    "ManyInv1CarryMsg",
    "ManyInv2CarryMsg",
    "ManyInv3CarryMsg",
    "ManyInv4CarryMsg",
    "ManyInv5CarryMsg",
    "ManyInv6CarryMsg",
    "ManyInv7CarryMsg",
    "ManyMsgSplit",
    "Manym2lPropBit1MsgSplit",
    "Manym2lPropBit0MsgSplit",
    "Manyl2mPropBit1MsgSplit",
    "Manyl2mPropBit0MsgSplit",
];

/// Register address mask that compares all bits (single-output PBS or
/// plain register).
pub const MASK_NONE: usize = usize::MAX;
/// Register address mask that ignores the lowest bit, grouping pairs
/// of consecutive registers produced by a 2-output PBS.
pub const MASK_PBS2: usize = usize::MAX << 1;
/// Register address mask that ignores the two lowest bits, grouping
/// quads of consecutive registers produced by a 4-output PBS.
pub const MASK_PBS4: usize = usize::MAX << 2;
/// Register address mask that ignores the three lowest bits, grouping
/// octets of consecutive registers produced by an 8-output PBS.
pub const MASK_PBS8: usize = usize::MAX << 3;

/// Inline operand carried by DOP instructions.
///
/// Supports two stream modes. *Unpatched* streams use symbolic
/// variables (`CtVar`, `PtVar`) that the microcontroller resolves at
/// load time into physical addresses and constants. *Patched* streams
/// carry resolved memory locations (`CtHeap`, `CtIo`) and constant
/// immediates (`PtConst`), as produced by the microcontroller or read
/// back from execution traces.
///
/// `CtReg` equality uses a masked comparison: only the bits selected
/// by the intersection of both masks are compared. This allows
/// multi-output PBS results (which occupy consecutive aligned
/// registers) to compare equal when their masks reflect the output
/// arity.
#[derive(Debug, Clone, Eq, Hash)]
pub enum Argument {
    /// Constant immediate plaintext value.
    PtConst {
        val: u8,
    },
    /// Ciphertext block located on the heap.
    CtHeap {
        addr: usize,
    },
    /// Ciphertext block located in I/O memory.
    CtIo {
        addr: usize,
    },
    /// Symbolic ciphertext variable, patched to a physical address by
    /// the microcontroller.
    CtSrcVar {
        id: usize,
        block: usize,
    },
    /// Symbolic ciphertext variable, patched to a physical address by
    /// the microcontroller.
    CtDstVar {
        id: usize,
        block: usize,
    },
    /// Symbolic plaintext variable, patched to a `PtConst` by the
    /// microcontroller.
    PtSrcVar {
        id: usize,
        block: usize,
    },
    /// Physical ciphertext register with an alignment mask.
    CtReg {
        mask: usize,
        addr: usize,
    },
    /// Lookup table identifier.
    LutId {
        id: usize,
    },
    // Event uid
    UserFlag {
        flag: u8,
    },
    // Board identifier
    VirtId {
        id: u8,
    },
}

impl Argument {
    /// Creates a `CtReg` with [`MASK_NONE`] (all bits significant).
    pub fn ct_reg(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_NONE,
            addr: addr.into(),
        }
    }

    /// Creates a `CtReg` with [`MASK_PBS2`] (lowest bit ignored).
    pub fn ct_reg2(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_PBS2,
            addr: addr.into(),
        }
    }

    /// Creates a `CtReg` with [`MASK_PBS4`] (two lowest bits ignored).
    pub fn ct_reg4(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_PBS4,
            addr: addr.into(),
        }
    }

    /// Creates a `CtReg` with [`MASK_PBS8`] (three lowest bits ignored).
    pub fn ct_reg8(addr: impl Into<usize>) -> Self {
        Argument::CtReg {
            mask: MASK_PBS8,
            addr: addr.into(),
        }
    }

    /// Creates a symbolic ciphertext source variable operand.
    pub fn ct_src_var(id: usize, block: usize) -> Self {
        Argument::CtSrcVar { id, block }
    }

    /// Creates a symbolic ciphertext destination variable operand.
    pub fn ct_dst_var(id: usize, block: usize) -> Self {
        Argument::CtDstVar { id, block }
    }

    /// Creates a symbolic plaintext source variable operand.
    pub fn pt_src_var(id: usize, block: usize) -> Self {
        Argument::PtSrcVar { id, block }
    }

    /// Creates a heap-addressed ciphertext operand.
    pub fn ct_heap(heap_slot: usize) -> Self {
        Argument::CtHeap { addr: heap_slot }
    }

    /// Creates an I/O-addressed ciphertext operand.
    pub fn ct_io(io_slot: usize) -> Self {
        Argument::CtIo { addr: io_slot }
    }

    /// Creates a constant plaintext immediate operand.
    pub fn pt_const(val: u8) -> Self {
        Argument::PtConst { val }
    }

    /// Creates a lookup table identifier operand from a [`LutId`].
    pub fn lut_id(val: LutId) -> Self {
        Argument::LutId { id: val.0 }
    }

    pub fn asm(&self) -> String {
        match self {
            Argument::PtConst { val } => format!("{val}"),
            Argument::CtHeap { addr } => format!("TH.{addr}"),
            Argument::CtIo { addr } => format!("@{addr}"),
            Argument::CtSrcVar { id, block } => format!("TS[{id}].{block}"),
            Argument::CtDstVar { id, block } => format!("TD[{id}].{block}"),
            Argument::PtSrcVar { id, block } => format!("TI[{id}].{block}"),
            Argument::CtReg { addr, .. } => format!("R{addr}"),
            Argument::LutId { id } => LUT_ALIASES[*id].into(),
            Argument::UserFlag { flag } => format!("F{flag}"),
            Argument::VirtId { id } => format!("N{id}"),
        }
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
                Argument::CtSrcVar {
                    id: lhs_id,
                    block: lhs_block,
                },
                Argument::CtSrcVar {
                    id: rhs_id,
                    block: rhs_block,
                },
            ) => (lhs_id, lhs_block) == (rhs_id, rhs_block),
            (
                Argument::CtDstVar {
                    id: lhs_id,
                    block: lhs_block,
                },
                Argument::CtDstVar {
                    id: rhs_id,
                    block: rhs_block,
                },
            ) => (lhs_id, lhs_block) == (rhs_id, rhs_block),
            (
                Argument::PtSrcVar {
                    id: lhs_id,
                    block: lhs_block,
                },
                Argument::PtSrcVar {
                    id: rhs_id,
                    block: rhs_block,
                },
            ) => (lhs_id, lhs_block) == (rhs_id, rhs_block),
            (Argument::LutId { id: lhs_id }, Argument::LutId { id: rhs_id }) => lhs_id == rhs_id,
            (Argument::UserFlag { flag: lhs_flag }, Argument::UserFlag { flag: rhs_flag }) => {
                lhs_flag == rhs_flag
            }
            (Argument::VirtId { id: lhs_id }, Argument::VirtId { id: rhs_id }) => lhs_id == rhs_id,
            _ => false,
        }
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::PtConst { val } => write!(f, "PT_I({val})"),
            Argument::CtHeap { addr } => write!(f, "CT_H({addr})"),
            Argument::CtIo { addr } => write!(f, "CT_IO({addr})"),
            Argument::CtReg { mask, addr } if *mask == MASK_NONE => write!(f, "R({addr})"),
            Argument::CtReg { mask, addr } if *mask == MASK_PBS2 => write!(f, "R({addr}, 2)"),
            Argument::CtReg { mask, addr } if *mask == MASK_PBS4 => write!(f, "R({addr}, 4)"),
            Argument::CtReg { mask, addr } if *mask == MASK_PBS8 => write!(f, "R({addr}, 8)"),
            Argument::CtReg { .. } => unreachable!(),
            Argument::CtSrcVar { id, block } | Argument::CtDstVar { id, block } => {
                write!(f, "TC({id}, {block})")
            }
            Argument::PtSrcVar { id, block } => write!(f, "TI({id}, {block})"),
            Argument::LutId { id } => write!(f, "LUT({id})"),
            Argument::UserFlag { flag } => write!(f, "F({flag})"),
            Argument::VirtId { id } => write!(f, "N({id})"),
        }
    }
}

/// Hardware pipeline lane to which a DOP instruction is dispatched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Affinity {
    /// Arithmetic-logic unit: register-to-register operations.
    Alu,
    /// Memory unit: load and store operations.
    Mem,
    /// Programmable bootstrapping unit.
    Pbs,
    /// Control: synchronization and initialization.
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

/// HPU hardware instruction set.
///
/// Each variant corresponds to a single hardware opcode. Operands are
/// carried inline as [`Argument`] values rather than through IR SSA
/// references — the DOP stream is a flat, register-allocated
/// instruction sequence.
///
/// Instructions fall into four categories matching the [`Affinity`]
/// lanes: register arithmetic (`ADD`, `SUB`, `MAC`, `ADDS`, `SUBS`,
/// `SSUB`, `MULS`), memory transfer (`LD`, `ST`), programmable
/// bootstrapping (`PBS` family), and control (`_INIT`, `SYNC`).
/// Scalar-operand arithmetic variants (`ADDS`, `SUBS`, `SSUB`,
/// `MULS`) take a plaintext constant in `cst`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum DopInstructionSet {
    /// `dst = src1 + src2` — ciphertext addition.
    ADD {
        dst: Argument,
        src1: Argument,
        src2: Argument,
    },
    /// `dst = src1 - src2` — ciphertext subtraction.
    SUB {
        dst: Argument,
        src1: Argument,
        src2: Argument,
    },
    /// `dst = src1 * cst + src2` — multiply-accumulate.
    MAC {
        dst: Argument,
        src1: Argument,
        src2: Argument,
        cst: Argument,
    },
    /// `dst = src + cst` — ciphertext-scalar addition.
    ADDS {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    /// `dst = src - cst` — ciphertext minus scalar.
    SUBS {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    /// `dst = cst - src` — scalar minus ciphertext.
    SSUB {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    /// `dst = src * cst` — ciphertext-scalar multiplication.
    MULS {
        dst: Argument,
        src: Argument,
        cst: Argument,
    },
    /// Loads a ciphertext block from memory into a register.
    LD { dst: Argument, src: Argument },
    /// Stores a ciphertext register to memory.
    ST { dst: Argument, src: Argument },
    /// Single-output programmable bootstrapping.
    PBS {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// 2-output many-LUT programmable bootstrapping.
    PBS_ML2 {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// 4-output many-LUT programmable bootstrapping.
    PBS_ML4 {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// 8-output many-LUT programmable bootstrapping.
    PBS_ML8 {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// Single-output PBS with flush (batch boundary marker).
    PBS_F {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// 2-output many-LUT PBS with flush.
    PBS_ML2_F {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// 4-output many-LUT PBS with flush.
    PBS_ML4_F {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// 8-output many-LUT PBS with flush.
    PBS_ML8_F {
        dst: Argument,
        src: Argument,
        lut: Argument,
    },
    /// Stream initialization marker. Produces the initial context
    /// token.
    _INIT,
    /// Synchronization barrier. Consumes the context token, ensuring
    /// all preceding instructions have completed.
    SYNC,
    /// Wait virtual op for Multi-HPU
    WAIT {
        flag: Argument,
        slot: Option<Argument>,
    },
    /// Notify virtual op for Multi-HPU
    NOTIFY {
        virt_id: Argument,
        flag: Argument,
        slot: Argument,
    },
    /// Load B2B virtual op for Multi-HPU
    LOAD_B2B { flag: Argument, slot: Argument },
}

impl Display for DopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DopInstructionSet::*;
        match self {
            ADD { dst, src1, src2 } => write!(f, "ADD<{dst}, {src1}, {src2}>"),
            SUB { dst, src1, src2 } => write!(f, "SUB<{dst}, {src1}, {src2}>"),
            MAC {
                dst,
                src1,
                src2,
                cst,
            } => write!(f, "MAC<{dst}, {src1}, {src2}, {cst}>"),
            ADDS { dst, src, cst } => write!(f, "ADDS<{dst}, {src}, {cst}>"),
            SUBS { dst, src, cst } => write!(f, "SUBS<{dst}, {src}, {cst}>"),
            SSUB { dst, src, cst } => write!(f, "SSUB<{dst}, {src}, {cst}>"),
            MULS { dst, src, cst } => write!(f, "MULS<{dst}, {src}, {cst}>"),
            LD { dst, src } => write!(f, "LD<{dst}, {src}>"),
            ST { dst, src } => write!(f, "ST<{dst}, {src}>"),
            PBS { dst, src, lut } => write!(f, "PBS<{dst}, {src}, {lut}>"),
            PBS_ML2 { dst, src, lut } => write!(f, "PBS2<{dst}, {src}, {lut}>"),
            PBS_ML4 { dst, src, lut } => write!(f, "PBS4<{dst}, {src}, {lut}>"),
            PBS_ML8 { dst, src, lut } => write!(f, "PBS8<{dst}, {src}, {lut}>"),
            PBS_F { dst, src, lut } => write!(f, "PBSF<{dst}, {src}, {lut}>"),
            PBS_ML2_F { dst, src, lut } => write!(f, "PBS2F<{dst}, {src}, {lut}>"),
            PBS_ML4_F { dst, src, lut } => write!(f, "PBS4F<{dst}, {src}, {lut}>"),
            PBS_ML8_F { dst, src, lut } => write!(f, "PBS8F<{dst}, {src}, {lut}>"),
            _INIT => write!(f, "_INIT"),
            SYNC => write!(f, "SYNC"),
            WAIT { flag, slot } => match slot {
                Some(slot) => write!(f, "WAIT<{flag}, {slot}>"),
                None => write!(f, "WAIT<{flag}>"),
            },
            NOTIFY {
                virt_id,
                flag,
                slot,
            } => write!(f, "NOTIFY<{virt_id}, {flag}, {slot}>"),
            LOAD_B2B { flag, slot } => write!(f, "LOAD_B2B<{flag}, {slot}>"),
        }
    }
}

impl DopInstructionSet {
    /// Returns true if this instruction is a PBS flush variant.
    pub fn is_pbs_flush(&self) -> bool {
        use DopInstructionSet::*;
        match self {
            PBS_F { .. } | PBS_ML2_F { .. } | PBS_ML4_F { .. } | PBS_ML8_F { .. } => true,
            _ => false,
        }
    }

    /// Returns the hardware pipeline lane for this instruction.
    pub fn affinity(&self) -> Affinity {
        use Affinity::*;
        use DopInstructionSet::*;
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
            NOTIFY { .. } => Ctl,
            WAIT { .. } => Ctl,
            LOAD_B2B { .. } => Ctl,
        }
    }

    /// Returns true if this instruction reads from the given argument.
    ///
    /// Only checks ciphertext source operands (`src`, `src1`, `src2`),
    /// not `cst` or `lut` fields. Returns false for `_INIT` and `SYNC`.
    pub fn has_source(&self, arg: &Argument) -> bool {
        use DopInstructionSet::*;
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
            LOAD_B2B { .. } | WAIT { .. } | NOTIFY { .. } => panic!(),
        }
    }

    /// Returns the destination operand, or `None` for `_INIT` and `SYNC`.
    pub fn get_dst(&self) -> Option<&Argument> {
        use DopInstructionSet::*;
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
            LOAD_B2B { .. } | WAIT { .. } | NOTIFY { .. } => panic!(),
        }
    }

    /// Returns the first source operand, or `None` for `_INIT` and
    /// `SYNC`.
    pub fn get_src1(&self) -> Option<&Argument> {
        use DopInstructionSet::*;
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
            LOAD_B2B { .. } | WAIT { .. } | NOTIFY { .. } => panic!(),
        }
    }

    /// Returns the second source operand, or `None` for instructions
    /// with fewer than two ciphertext sources.
    ///
    /// Only `ADD`, `SUB`, and `MAC` carry a second source.
    pub fn get_src2(&self) -> Option<&Argument> {
        use DopInstructionSet::*;
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
            LOAD_B2B { .. } | WAIT { .. } | NOTIFY { .. } => panic!(),
        }
    }
}

impl DialectInstructionSet for DopInstructionSet {
    type TypeSystem = DopTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use DopInstructionSet::*;
        use DopTypeSystem::*;
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
            | PBS_ML8_F { .. }
            | WAIT { .. }
            | NOTIFY { .. }
            | LOAD_B2B { .. } => sig![(Ctx(0)) -> (Ctx(0))],
            _INIT => sig![() -> (Ctx(0))],
            SYNC => sig![(Ctx(0)) -> ()],
        }
    }
}
