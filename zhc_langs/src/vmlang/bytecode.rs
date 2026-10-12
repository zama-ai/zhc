use serde::Serialize;
use std::fmt::Debug;
use zhc_ir::OpIdRaw;
use zhc_utils::Dumpable;

/// Executable instruction of the software VM.
///
/// The flat, register-allocated form of a [`VmLang`](super::VmLang)
/// program: `dst`, `src`, `src1`, `src2`, `dst1` and `dst2` are indices
/// into the VM register file, and plaintext operands are resolved
/// inline — either as a constant in `cst`, or as the `(s_id, s_blk)`
/// address of a block inside a plaintext input. Consequently the
/// dialect's `ImmLd` has no counterpart here.
///
/// Registers hold two ciphertext widths. `KS` is the only instruction
/// writing the reduced width, and `PBS`/`PBS_ML2` the only ones reading
/// it; every other instruction reads and writes full-width registers.
/// Feeding a PBS from anything but a `KS` destination, or reading a
/// `KS` destination from anything but a PBS, is a malformed stream.
///
/// Every variant carries the `id` of the dialect operation it was
/// lowered from, which the executor uses to key an instruction's
/// dependency bookkeeping. The `get_*` accessors expose register
/// operands only: input and output slot addresses, inline constants and
/// table indices are not reported by them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[allow(non_camel_case_types)]
pub enum VmByteCode {
    /// `dst = src1 + src2` — ciphertext addition.
    ADD {
        id: OpIdRaw,
        dst: u16,
        src1: u16,
        src2: u16,
    },
    /// `dst = src1 - src2` — ciphertext subtraction.
    SUB {
        id: OpIdRaw,
        dst: u16,
        src1: u16,
        src2: u16,
    },
    /// `dst = src1 * cst + src2` — multiply-accumulate.
    MAC {
        id: OpIdRaw,
        dst: u16,
        src1: u16,
        src2: u16,
        cst: u8,
    },
    /// `dst = src + pt` — ciphertext plus the plaintext block `s_blk`
    /// of input slot `s_id`.
    ADDS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    /// `dst = src - pt` — ciphertext minus the plaintext block `s_blk`
    /// of input slot `s_id`.
    SUBS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    /// `dst = pt - src` — the plaintext block `s_blk` of input slot
    /// `s_id` minus a ciphertext.
    SSUB {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    /// `dst = src * pt` — ciphertext scaled by the plaintext block
    /// `s_blk` of input slot `s_id`.
    MULS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    /// `dst = src + cst` — ciphertext plus an inline constant.
    ADDC {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    /// `dst = src - cst` — ciphertext minus an inline constant.
    SUBC {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    /// `dst = cst - src` — inline constant minus a ciphertext.
    CSUB {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    /// `dst = src * cst` — ciphertext scaled by an inline constant.
    MULC {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    /// Loads the ciphertext block `src_blk` of input slot `src_id` into
    /// register `dst`.
    LD {
        id: OpIdRaw,
        dst: u16,
        src_id: u16,
        src_blk: u8,
    },
    /// Stores register `src` into the ciphertext block `dst_blk` of
    /// output slot `dst_id`.
    ST {
        id: OpIdRaw,
        dst_id: u16,
        dst_blk: u8,
        src: u16,
    },
    /// `dst = keyswitch(src)` — reduces a full-width ciphertext to the
    /// width a PBS consumes.
    KS { id: OpIdRaw, dst: u16, src: u16 },
    /// Single-output programmable bootstrapping of the keyswitched
    /// register `src` through lookup table `lut`.
    PBS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        lut: u8,
    },
    /// Two-output programmable bootstrapping of the keyswitched
    /// register `src` through lookup table `lut`. A single bootstrap
    /// fills both `dst1` and `dst2`.
    PBS_ML2 {
        id: OpIdRaw,
        dst1: u16,
        dst2: u16,
        src: u16,
        lut: u8,
    },
    /// `dst = trivial(cst)` — materializes a trivial encryption of an
    /// inline constant, overwriting the whole register.
    DEF { id: OpIdRaw, dst: u16, cst: u8 },
}

impl VmByteCode {
    /// Returns the identifier of the dialect operation this instruction
    /// was lowered from.
    ///
    /// Every variant carries one, so this never fails. Identifiers are
    /// preserved verbatim across scheduling, which is what lets the
    /// executor recover an instruction's dependencies from the tables
    /// built alongside the stream.
    pub fn get_id(&self) -> OpIdRaw {
        use VmByteCode::*;
        match self {
            ADD { id, .. }
            | SUB { id, .. }
            | MAC { id, .. }
            | ADDS { id, .. }
            | SUBS { id, .. }
            | SSUB { id, .. }
            | MULS { id, .. }
            | ADDC { id, .. }
            | SUBC { id, .. }
            | CSUB { id, .. }
            | MULC { id, .. }
            | LD { id, .. }
            | ST { id, .. }
            | PBS { id, .. }
            | PBS_ML2 { id, .. }
            | KS { id, .. }
            | DEF { id, .. } => *id,
        }
    }

    /// Returns the first register written by this instruction, if any.
    ///
    /// `ST` is the only variant returning `None`: it writes an output
    /// block rather than a register.
    pub fn get_dst1(&self) -> Option<u16> {
        use VmByteCode::*;
        match self {
            ADD { dst, .. } => Some(*dst),
            SUB { dst, .. } => Some(*dst),
            MAC { dst, .. } => Some(*dst),
            ADDS { dst, .. } => Some(*dst),
            SUBS { dst, .. } => Some(*dst),
            SSUB { dst, .. } => Some(*dst),
            MULS { dst, .. } => Some(*dst),
            LD { dst, .. } => Some(*dst),
            PBS { dst, .. } => Some(*dst),
            PBS_ML2 { dst1, .. } => Some(*dst1),
            KS { dst, .. } => Some(*dst),
            ADDC { dst, .. } => Some(*dst),
            SUBC { dst, .. } => Some(*dst),
            CSUB { dst, .. } => Some(*dst),
            MULC { dst, .. } => Some(*dst),
            DEF { dst, .. } => Some(*dst),
            _ => None,
        }
    }

    /// Returns the second register written by this instruction, if any.
    ///
    /// `PBS_ML2` is the only variant returning `Some`, since it is the
    /// only one with two destinations.
    pub fn get_dst2(&self) -> Option<u16> {
        use VmByteCode::*;
        match self {
            PBS_ML2 { dst2, .. } => Some(*dst2),
            _ => None,
        }
    }

    /// Returns the first register read by this instruction, if any.
    ///
    /// `LD` and `DEF` return `None`: their source is respectively an
    /// input block and an inline constant.
    pub fn get_src1(&self) -> Option<u16> {
        use VmByteCode::*;
        match self {
            ADD { src1, .. } => Some(*src1),
            SUB { src1, .. } => Some(*src1),
            MAC { src1, .. } => Some(*src1),
            ADDS { src, .. } => Some(*src),
            SUBS { src, .. } => Some(*src),
            SSUB { src, .. } => Some(*src),
            MULS { src, .. } => Some(*src),
            ST { src, .. } => Some(*src),
            PBS { src, .. } => Some(*src),
            PBS_ML2 { src, .. } => Some(*src),
            KS { src, .. } => Some(*src),
            ADDC { src, .. } => Some(*src),
            SUBC { src, .. } => Some(*src),
            CSUB { src, .. } => Some(*src),
            MULC { src, .. } => Some(*src),
            _ => None,
        }
    }

    /// Returns the second register read by this instruction, if any.
    ///
    /// `ADD`, `SUB` and `MAC` are the only variants returning `Some`,
    /// since they are the only ones with two ciphertext operands.
    pub fn get_src2(&self) -> Option<u16> {
        use VmByteCode::*;
        match self {
            ADD { src2, .. } => Some(*src2),
            SUB { src2, .. } => Some(*src2),
            MAC { src2, .. } => Some(*src2),
            _ => None,
        }
    }
}

impl Dumpable for VmByteCode {
    fn dump_to_string(&self) -> String {
        use VmByteCode::*;
        match self {
            ADD {
                id,
                dst,
                src1,
                src2,
            } => format!("ADD id={} dst={} src1={} src2={}", id, dst, src1, src2),
            SUB {
                id,
                dst,
                src1,
                src2,
            } => format!("SUB id={} dst={} src1={} src2={}", id, dst, src1, src2),
            MAC {
                id,
                dst,
                src1,
                src2,
                cst,
            } => format!(
                "MAC id={} dst={} src1={} src2={} cst={}",
                id, dst, src1, src2, cst
            ),
            ADDS {
                id,
                dst,
                src,
                s_id,
                s_blk,
            } => format!(
                "ADDS id={} dst={} src={} s_id={} s_blk={}",
                id, dst, src, s_id, s_blk
            ),
            SUBS {
                id,
                dst,
                src,
                s_id,
                s_blk,
            } => format!(
                "SUBS id={} dst={} src={} s_id={} s_blk={}",
                id, dst, src, s_id, s_blk
            ),
            SSUB {
                id,
                dst,
                src,
                s_id,
                s_blk,
            } => format!(
                "SSUB id={} dst={} src={} s_id={} s_blk={}",
                id, dst, src, s_id, s_blk
            ),
            MULS {
                id,
                dst,
                src,
                s_id,
                s_blk,
            } => format!(
                "MULS id={} dst={} src={} s_id={} s_blk={}",
                id, dst, src, s_id, s_blk
            ),
            ADDC { id, dst, src, cst } => {
                format!("ADDC id={} dst={} src={} cst={}", id, dst, src, cst)
            }
            SUBC { id, dst, src, cst } => {
                format!("SUBC id={} dst={} src={} cst={}", id, dst, src, cst)
            }
            CSUB { id, dst, src, cst } => {
                format!("CSUB id={} dst={} src={} cst={}", id, dst, src, cst)
            }
            MULC { id, dst, src, cst } => {
                format!("MULC id={} dst={} src={} cst={}", id, dst, src, cst)
            }
            LD {
                id,
                dst,
                src_id,
                src_blk,
            } => format!(
                "LD id={} dst={} src_id={} src_blk={}",
                id, dst, src_id, src_blk
            ),
            ST {
                id,
                dst_id,
                dst_blk,
                src,
            } => format!(
                "ST id={} dst_id={} dst_blk={} src={}",
                id, dst_id, dst_blk, src
            ),
            KS { id, dst, src } => format!("KS id={} dst={} src={}", id, dst, src),
            PBS { id, dst, src, lut } => {
                format!("PBS id={} dst={} src={} lut={}", id, dst, src, lut)
            }
            PBS_ML2 {
                id,
                dst1,
                dst2,
                src,
                lut,
            } => format!(
                "PBS_ML2 id={} dst1={} dst2={} src={} lut={}",
                id, dst1, dst2, src, lut
            ),
            DEF { id, dst, cst } => format!("DEF id={} dst={} cst={}", id, dst, cst),
        }
    }
}
