use std::{fmt::Debug, u8};
use serde::Serialize;
use zhc_ir::OpIdRaw;
use zhc_utils::Dumpable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[allow(non_camel_case_types)]
pub enum VmByteCode {
    ADD {
        id: OpIdRaw,
        dst: u16,
        src1: u16,
        src2: u16,
    },
    SUB {
        id: OpIdRaw,
        dst: u16,
        src1: u16,
        src2: u16,
    },
    MAC {
        id: OpIdRaw,
        dst: u16,
        src1: u16,
        src2: u16,
        cst: u8,
    },
    ADDS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    SUBS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    SSUB {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    MULS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        s_id: u16,
        s_blk: u8,
    },
    ADDC {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    SUBC {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    CSUB {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    MULC {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        cst: u8,
    },
    LD {
        id: OpIdRaw,
        dst: u16,
        src_id: u16,
        src_blk: u8,
    },
    ST {
        id: OpIdRaw,
        dst_id: u16,
        dst_blk: u8,
        src: u16,
    },
    KS {
        id: OpIdRaw,
        dst: u16,
        src: u16
    },
    PBS {
        id: OpIdRaw,
        dst: u16,
        src: u16,
        lut: u8,
    },
    PBS_ML2 {
        id: OpIdRaw,
        dst1: u16,
        dst2: u16,
        src: u16,
        lut: u8,
    },
    DEF {
        id: OpIdRaw,
        dst: u16,
        cst: u8,
    },
}

impl VmByteCode {
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
            | KS {id, ..}
            | DEF { id, .. } => *id,
        }
    }

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

    pub fn get_dst2(&self) -> Option<u16> {
        use VmByteCode::*;
        match self {
            PBS_ML2 { dst2, .. } => Some(*dst2),
            _ => None,
        }
    }

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
            KS {src, ..} => Some(*src),
            ADDC { src, .. } => Some(*src),
            SUBC { src, .. } => Some(*src),
            CSUB { src, .. } => Some(*src),
            MULC { src, .. } => Some(*src),
            _ => None,
        }
    }

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
            ADD { id, dst, src1, src2 } =>
                format!("ADD id={} dst={} src1={} src2={}", id, dst, src1, src2),
            SUB { id, dst, src1, src2 } =>
                format!("SUB id={} dst={} src1={} src2={}", id, dst, src1, src2),
            MAC { id, dst, src1, src2, cst } =>
                format!("MAC id={} dst={} src1={} src2={} cst={}", id, dst, src1, src2, cst),
            ADDS { id, dst, src, s_id, s_blk } =>
                format!("ADDS id={} dst={} src={} s_id={} s_blk={}", id, dst, src, s_id, s_blk),
            SUBS { id, dst, src, s_id, s_blk } =>
                format!("SUBS id={} dst={} src={} s_id={} s_blk={}", id, dst, src, s_id, s_blk),
            SSUB { id, dst, src, s_id, s_blk } =>
                format!("SSUB id={} dst={} src={} s_id={} s_blk={}", id, dst, src, s_id, s_blk),
            MULS { id, dst, src, s_id, s_blk } =>
                format!("MULS id={} dst={} src={} s_id={} s_blk={}", id, dst, src, s_id, s_blk),
            ADDC { id, dst, src, cst } =>
                format!("ADDC id={} dst={} src={} cst={}", id, dst, src, cst),
            SUBC { id, dst, src, cst } =>
                format!("SUBC id={} dst={} src={} cst={}", id, dst, src, cst),
            CSUB { id, dst, src, cst } =>
                format!("CSUB id={} dst={} src={} cst={}", id, dst, src, cst),
            MULC { id, dst, src, cst } =>
                format!("MULC id={} dst={} src={} cst={}", id, dst, src, cst),
            LD { id, dst, src_id, src_blk } =>
                format!("LD id={} dst={} src_id={} src_blk={}", id, dst, src_id, src_blk),
            ST { id, dst_id, dst_blk, src } =>
                format!("ST id={} dst_id={} dst_blk={} src={}", id, dst_id, dst_blk, src),
            KS { id, dst, src } =>
                format!("KS id={} dst={} src={}", id, dst, src),
            PBS { id, dst, src, lut } =>
                format!("PBS id={} dst={} src={} lut={}", id, dst, src, lut),
            PBS_ML2 { id, dst1, dst2, src, lut } =>
                format!("PBS_ML2 id={} dst1={} dst2={} src={} lut={}", id, dst1, dst2, src, lut),
            DEF { id, dst, cst } =>
                format!("DEF id={} dst={} cst={}", id, dst, cst),
        }

    }
}
