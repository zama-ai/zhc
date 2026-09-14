//! Translation table generation for device operations.
//!
//! This module provides functionality to generate binary instruction encodings
//! from device operation intermediate representations. It defines the binary
//! formats for different instruction types and converts the IR into executable
//! machine code for the target HPU hardware.

use bitfield_struct::bitfield;
use zhc_crypto::integer_semantics::lut::LutId;
use zhc_ir::IR;
use zhc_langs::doplang::{
    CtDstVar, CtHeap, CtIo, CtMem, CtReg, CtSrcVar, DopInstructionSet, DopLang, LutRef, MASK_PBS2,
    MASK_PBS4, MASK_PBS8, PtArg, PtConst, PtSrcVar, UserFlag, VirtId,
};
use zhc_utils::{SafeAs, svec};

/// Binary representation of a device operation instruction.
pub type DOpRepr = u32;

/// DopCode structure
// DOp are defined with two section: {Type, subtype}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DOpCode {
    optype: DOpType,
    subtype: u8,
}

/// Define Instruction type as C-like enumeration
/// Types are encoded with 2bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum DOpType {
    ARITH = 0b00,
    UCORE = 0b01,
    MEM = 0b10,
    PBS = 0b11,
}

/// Define raw type conversion
/// Opcode is on 6bits
impl From<DOpCode> for u8 {
    fn from(value: DOpCode) -> Self {
        (((value.optype as u8) & 0x3) << 4) + value.subtype
    }
}
impl From<u8> for DOpCode {
    fn from(value: u8) -> Self {
        let subtype = value & 0xf;
        let optype_raw = (value >> 4) & 0x3;
        let optype = match optype_raw {
            x if x == DOpType::ARITH as u8 => DOpType::ARITH,
            x if x == DOpType::UCORE as u8 => DOpType::UCORE,
            x if x == DOpType::MEM as u8 => DOpType::MEM,
            x if x == DOpType::PBS as u8 => DOpType::PBS,
            _ => panic!("Invalid DOpType"),
        };

        Self { optype, subtype }
    }
}

/// Implement helper consts to name Arith DOp
impl DOpCode {
    pub const ADD: Self = Self {
        optype: DOpType::ARITH,
        subtype: 0b0001,
    };
    pub const SUB: Self = Self {
        optype: DOpType::ARITH,
        subtype: 0b0010,
    };
    pub const MAC: Self = Self {
        optype: DOpType::ARITH,
        subtype: 0b0101,
    };
}

/// Implement helper consts to name ArithMsg DOp
impl DOpCode {
    pub const ADDS: Self = Self {
        optype: DOpType::ARITH,
        subtype: 0b1001,
    };
    pub const SUBS: Self = Self {
        optype: DOpType::ARITH,
        subtype: 0b1010,
    };
    pub const SSUB: Self = Self {
        optype: DOpType::ARITH,
        subtype: 0b1011,
    };
    pub const MULS: Self = Self {
        optype: DOpType::ARITH,
        subtype: 0b1100,
    };
}

/// Implement helper consts to name Mem DOp
impl DOpCode {
    pub const LD: Self = Self {
        optype: DOpType::MEM,
        subtype: 0b0000,
    };
    pub const ST: Self = Self {
        optype: DOpType::MEM,
        subtype: 0b0001,
    };
    pub const SYNC: Self = Self {
        optype: DOpType::MEM,
        subtype: 0b1111,
    };
}

/// Implement helper consts to name Ucore DOp
impl DOpCode {
    pub const NOTIFY: Self = Self {
        optype: DOpType::UCORE,
        subtype: 0b0000,
    };
    pub const WAIT: Self = Self {
        optype: DOpType::UCORE,
        subtype: 0b0001,
    };
    pub const LD_B2B: Self = Self {
        optype: DOpType::UCORE,
        subtype: 0b1000,
    };
    pub const EXTEND: Self = Self {
        optype: DOpType::UCORE,
        subtype: 0b1111,
    };
}

/// Implement helper consts to name Pbs DOp
impl DOpCode {
    pub const PBS: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b0000,
    };
    pub const PBS_ML2: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b0001,
    };
    pub const PBS_ML4: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b0010,
    };
    pub const PBS_ML8: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b0011,
    };
    pub const PBS_F: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b1000,
    };
    pub const PBS_ML2_F: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b1001,
    };
    pub const PBS_ML4_F: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b1010,
    };
    pub const PBS_ML8_F: Self = Self {
        optype: DOpType::PBS,
        subtype: 0b1011,
    };
}

/// Raw device operation encoding for opcode extraction.
#[bitfield(u32)]
pub struct DOpRawHex {
    #[bits(26)]
    _reserved: u32,
    #[bits(6)]
    pub opcode: u8,
}

/// PeArith instructions
/// Arithmetic operation that use one destination register and two sources register
/// Have also an extra mul_factor field for MAC insn
#[bitfield(u32)]
pub struct PeArithHex {
    #[bits(7)]
    dst_rid: u8,
    #[bits(7)]
    src0_rid: u8,
    #[bits(7)]
    src1_rid: u8,
    #[bits(5)]
    mul_factor: u8,
    #[bits(6)]
    opcode: u8,
}

/// PeMsg instructions
/// Arithmetic operation that use one destination register, one source register and an immediate
/// value
#[bitfield(u32)]
pub struct PeArithMsgHex {
    #[bits(7)]
    dst_rid: u8,
    #[bits(7)]
    src_rid: u8,
    #[bits(1)]
    msg_mode: bool,
    #[bits(11)]
    msg_cst: u16,
    #[bits(6)]
    opcode: u8,
}

// Define encoding for msg_mode
const IMM_CST: bool = false;
const IMM_VAR: bool = true;

/// PeMem instructions
/// LD/St operation with one register and one memory slot
#[bitfield(u32)]
pub struct PeMemHex {
    #[bits(7)]
    rid: u8,
    #[bits(1)]
    _pad: u8,
    #[bits(2)]
    mode: u8,
    #[bits(16)]
    slot: u16,
    #[bits(6)]
    opcode: u8,
}

// Define encoding for mem_mode
const MEM_ADDR: u8 = 0x0;
const MEM_HEAP: u8 = 0x1;
const MEM_SRC: u8 = 0x2;
const MEM_DST: u8 = 0x3;

/// PePbs instructions
#[bitfield(u32)]
pub struct PePbsHex {
    #[bits(7)]
    dst_rid: u8,
    #[bits(7)]
    src_rid: u8,
    #[bits(12)]
    gid: u16,
    #[bits(6)]
    opcode: u8,
}

/// PeUcore instructions
#[bitfield(u32)]
pub struct PeUcoreHex {
    #[bits(16)]
    slot: u16,
    #[bits(1)]
    mode: u8,
    #[bits(6)]
    flag: u8,
    #[bits(3)]
    hid: u8,
    #[bits(6)]
    opcode: u8,
}

/// PeSync instructions
#[bitfield(u32)]
pub struct PeSyncHex {
    #[bits(11)]
    _pad: u32,
    #[bits(6)]
    flag: u8,
    is_inner_sync: bool,
    #[bits(8)]
    iid: u8,
    #[bits(6)]
    opcode: u8,
}

/// Generates binary instruction encodings from device operation IR.
///
/// Converts the intermediate representation `ir` containing device operations
/// into a vector of binary instruction representations suitable for execution
/// on the target hardware.
pub fn generate_translation_table(
    ir: &IR<DopLang>,
    lut_relocation: Option<&[LutId]>,
) -> Vec<DOpRepr> {
    let mut output = Vec::with_capacity(ir.n_ops().sas());
    output.push(0); // reserve room for the length of the stream at the beginning of the stream.
    for op in ir.walk_ops_topological() {
        use DopInstructionSet::*;
        match op.get_instruction() {
            ADD {
                dst: CtReg { addr: dst, .. },
                src1: CtReg { addr: src1, .. },
                src2: CtReg { addr: src2, .. },
            } => {
                output.push(
                    PeArithHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src0_rid((*src1).sas())
                        .with_src1_rid((*src2).sas())
                        .with_opcode(u8::from(DOpCode::ADD))
                        .0,
                );
            }
            SUB {
                dst: CtReg { addr: dst, .. },
                src1: CtReg { addr: src1, .. },
                src2: CtReg { addr: src2, .. },
            } => {
                output.push(
                    PeArithHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src0_rid((*src1).sas())
                        .with_src1_rid((*src2).sas())
                        .with_opcode(u8::from(DOpCode::SUB))
                        .0,
                );
            }
            MAC {
                dst: CtReg { addr: dst, .. },
                src1: CtReg { addr: src1, .. },
                src2: CtReg { addr: src2, .. },
                cst: PtArg::Const(PtConst { val: cst }),
            } => {
                output.push(
                    PeArithHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src0_rid((*src1).sas())
                        .with_src1_rid((*src2).sas())
                        .with_mul_factor((*cst).sas())
                        .with_opcode(u8::from(DOpCode::MAC))
                        .0,
                );
            }
            // A `TI[..]` template multiplier has no hardware encoding: `mul_factor` is a small
            // literal field, not a runtime-patchable slot.
            MAC {
                cst: PtArg::Var(_), ..
            } => panic!("MAC: a templated (TI[..]) multiplier cannot be hex-encoded"),
            ADDS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtArg::Const(PtConst { val: cst }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(u8::from(DOpCode::ADDS))
                    .0,
            ),
            ADDS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtArg::Var(PtSrcVar {
                        id: tid,
                        block: bid,
                    }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(u8::from(DOpCode::ADDS))
                    .0,
            ),
            SUBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtArg::Const(PtConst { val: cst }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(u8::from(DOpCode::SUBS))
                    .0,
            ),
            SUBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtArg::Var(PtSrcVar {
                        id: tid,
                        block: bid,
                    }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(u8::from(DOpCode::SUBS))
                    .0,
            ),
            SSUB {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtArg::Const(PtConst { val: cst }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(u8::from(DOpCode::SSUB))
                    .0,
            ),
            SSUB {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtArg::Var(PtSrcVar {
                        id: tid,
                        block: bid,
                    }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(u8::from(DOpCode::SSUB))
                    .0,
            ),
            MULS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtArg::Const(PtConst { val: cst }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(u8::from(DOpCode::MULS))
                    .0,
            ),
            MULS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtArg::Var(PtSrcVar {
                        id: tid,
                        block: bid,
                    }),
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(u8::from(DOpCode::MULS))
                    .0,
            ),
            LD {
                dst: CtReg { addr: dst, .. },
                src: CtMem::Heap(CtHeap { addr: src }),
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*dst).sas())
                        .with_mode(MEM_HEAP)
                        .with_slot((*src).sas())
                        .with_opcode(u8::from(DOpCode::LD))
                        .0,
                );
            }
            LD {
                dst: CtReg { addr: dst, .. },
                src: CtMem::Io(CtIo { addr: src }),
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*dst).sas())
                        .with_mode(MEM_ADDR)
                        .with_slot((*src).sas())
                        .with_opcode(u8::from(DOpCode::LD))
                        .0,
                );
            }
            LD {
                dst: CtReg { addr: dst, .. },
                src:
                    CtMem::Src(CtSrcVar {
                        id: tid,
                        block: bid,
                    }),
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*dst).sas())
                        .with_mode(MEM_SRC)
                        .with_slot((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                        .with_opcode(u8::from(DOpCode::LD))
                        .0,
                );
            }
            LD {
                src: CtMem::Dst(_), ..
            } => {
                panic!("LD: a destination template (TD[..]) cannot be a load source")
            }
            ST {
                dst: CtMem::Heap(CtHeap { addr: dst }),
                src: CtReg { addr: src, .. },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*src).sas())
                        .with_mode(MEM_HEAP)
                        .with_slot((*dst).sas())
                        .with_opcode(u8::from(DOpCode::ST))
                        .0,
                );
            }
            ST {
                dst: CtMem::Io(CtIo { addr: dst }),
                src: CtReg { addr: src, .. },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*src).sas())
                        .with_mode(MEM_ADDR)
                        .with_slot((*dst).sas())
                        .with_opcode(u8::from(DOpCode::ST))
                        .0,
                );
            }
            ST {
                dst:
                    CtMem::Dst(CtDstVar {
                        id: tid,
                        block: bid,
                    }),
                src: CtReg { addr: src, .. },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*src).sas())
                        .with_mode(MEM_DST)
                        .with_slot((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                        .with_opcode(u8::from(DOpCode::ST))
                        .0,
                );
            }
            ST {
                dst: CtMem::Src(_), ..
            } => {
                panic!("ST: a source template (TS[..]) cannot be a store destination")
            }
            PBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                let gid = match lut_relocation {
                    Some(reloc) => reloc.get(*gid as usize).unwrap().0,
                    None => *gid as usize,
                };
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid(gid.sas())
                        .with_opcode(u8::from(DOpCode::PBS))
                        .0,
                );
            }
            PBS_ML2 {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(u8::from(DOpCode::PBS_ML2))
                        .0,
                );
            }
            PBS_ML4 {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(u8::from(DOpCode::PBS_ML4))
                        .0,
                );
            }
            PBS_ML8 {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(u8::from(DOpCode::PBS_ML8))
                        .0,
                );
            }
            PBS_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(u8::from(DOpCode::PBS_F))
                        .0,
                );
            }
            PBS_ML2_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(u8::from(DOpCode::PBS_ML2_F))
                        .0,
                );
            }
            PBS_ML4_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(u8::from(DOpCode::PBS_ML4_F))
                        .0,
                );
            }
            PBS_ML8_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutRef { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(u8::from(DOpCode::PBS_ML8_F))
                        .0,
                );
            }

            // Multi-hpu related DOp
            WAIT {
                slot,
                flag: UserFlag { flag },
            } => {
                let (has_data, mode_hex, slot_hex) = match slot {
                    Some(CtMem::Io(CtIo { addr })) => (1, MEM_ADDR, *addr as u16),
                    Some(CtMem::Heap(CtHeap { addr })) => (1, MEM_HEAP, *addr as u16),
                    Some(_) => panic!("Unexpected slot argument in WAIT"),
                    None => (0, 0, 0),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex)
                        .with_mode(mode_hex)
                        .with_flag(*flag)
                        .with_hid(has_data)
                        .with_opcode(u8::from(DOpCode::WAIT))
                        .0,
                );
            }
            NOTIFY {
                virt_id: VirtId { id: vid },
                flag: UserFlag { flag },
                slot,
            } => {
                let (mode, slot_hex) = match slot {
                    CtMem::Io(CtIo { addr }) => (MEM_ADDR, *addr as u16),
                    CtMem::Heap(CtHeap { addr }) => (MEM_HEAP, *addr as u16),
                    CtMem::Src(CtSrcVar { id, block }) => {
                        (MEM_HEAP, ((*id as u16) << 8) + *block as u16)
                    }
                    _ => panic!("Unexpected slot argument in NOTIFY"),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex)
                        .with_mode(mode)
                        .with_flag(*flag)
                        .with_hid(*vid)
                        .with_opcode(u8::from(DOpCode::NOTIFY))
                        .0,
                );
            }
            LD_B2B {
                flag: UserFlag { flag },
                slot,
            } => {
                let (mode, slot_hex) = match slot {
                    CtMem::Io(CtIo { addr }) => (MEM_ADDR, *addr as u16),
                    CtMem::Heap(CtHeap { addr }) => (MEM_HEAP, *addr as u16),
                    _ => panic!("Unexpected slot argument in LD_B2B"),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex)
                        .with_mode(mode)
                        .with_flag(*flag)
                        .with_hid(0) // Unused
                        .with_opcode(u8::from(DOpCode::LD_B2B))
                        .0,
                );
            }
            _START | _END => {}
            a => {
                panic!("Unexpected Doplang Operation encountered: {a}")
            }
        };
    }
    output[0] = (output.len() - 1).sas();
    output
}

/// Decodes a translation-table word stream (as produced by [`generate_translation_table`]) back
/// into an `IR<DopLang>` graph.
///
/// `words` must start with the instruction-count word `generate_translation_table` writes,
/// followed by exactly that many 32-bit DOp encodings.
///
/// `lut_relocation`, when given, is applied in reverse to single-output `PBS`/`PBS_F`
/// destinations only — the physical gid found in the hex is looked up by position in
/// `lut_relocation` to recover the original logical [`LutId`]. Many-LUT `PBS_ML*` variants were
/// never relocated by the encoder (see `generate_translation_table`), so their gid is always
/// taken literally, matching encode.
///
/// # Panics
///
/// Panics if the word count doesn't match the declared length, if an unrecognized opcode or
/// memory mode is encountered, or on `SYNC`: its hardware encoding needs sync metadata
/// (`iid`/`hid`/`flag`) that `DopInstructionSet::SYNC` doesn't carry, so — like
/// `generate_translation_table`, which has no encode arm for it either — it isn't supported in
/// this direction.
pub fn decode_translation_table(
    words: &[DOpRepr],
    lut_relocation: Option<&[LutId]>,
) -> IR<DopLang> {
    let declared_len = words.first().copied().unwrap_or(0) as usize;
    let body: &[DOpRepr] = if words.is_empty() { &[] } else { &words[1..] };
    assert_eq!(
        body.len(),
        declared_len,
        "translation table declares {declared_len} instruction(s) but has {} word(s) following \
         the length prefix",
        body.len()
    );
    let mut ir: IR<DopLang> = IR::empty();
    let (_, start_rets) = ir.add_op(DopInstructionSet::_START, svec![]);
    let mut ctx = start_rets[0];

    for &word in body {
        let instr = instruction_from_dop_repr(word, lut_relocation);
        let (_, rets) = ir.add_op(instr, svec![ctx]);
        ctx = rets[0];
    }
    ir.add_op(DopInstructionSet::_END, svec![ctx]);
    ir
}

/// Decode DopInstructionSet from raw DOpRepr.
///
///
/// `lut_relocation`, when given, is applied in reverse to single-output `PBS`/`PBS_F`
/// destinations only — the physical gid found in the hex is looked up by position in
/// `lut_relocation` to recover the original logical [`LutId`]. Many-LUT `PBS_ML*` variants were
/// never relocated by the encoder (see `generate_translation_table`), so their gid is always
/// taken literally, matching encode.
///
/// Also note a pre-existing ambiguity in the `NOTIFY` encoding: both `CtHeap` and `CtSrcVar`
/// slots encode to the same `MEM_HEAP` mode tag (see `generate_translation_table`), so this
/// function cannot tell them apart and always decodes that mode as `CtHeap`.
pub fn instruction_from_dop_repr(
    word: DOpRepr,
    lut_relocation: Option<&[LutId]>,
) -> DopInstructionSet {
    use DopInstructionSet::*;
    let resolve_gid = |gid: u16| -> usize {
        match lut_relocation {
            Some(reloc) => reloc
                .iter()
                .position(|lid| lid.0 == gid.sas::<usize>())
                .unwrap_or_else(|| panic!("gid {gid} not found in lut_relocation table")),
            None => gid.sas(),
        }
    };
    let ct_reg = |mask: u8, addr: u8| CtReg {
        mask,
        addr: addr.sas(),
    };
    let var = |packed: u16| ((packed >> 8) as u8, (packed & 0xff) as u8);

    let dopcode_raw = DOpRawHex::from_bits(word).opcode();
    let dopcode = DOpCode::from(dopcode_raw);

    match dopcode {
        DOpCode::ADD | DOpCode::SUB => {
            let hex = PeArithHex::from_bits(word);
            let dst = ct_reg(u8::MAX, hex.dst_rid());
            let src1 = ct_reg(u8::MAX, hex.src0_rid());
            let src2 = ct_reg(u8::MAX, hex.src1_rid());
            match dopcode {
                DOpCode::ADD => ADD { dst, src1, src2 },
                DOpCode::SUB => SUB { dst, src1, src2 },
                _ => panic!("Unknown DopCode {dopcode_raw} [{dopcode:?}]"),
            }
        }
        DOpCode::MAC => {
            let hex = PeArithHex::from_bits(word);
            MAC {
                dst: ct_reg(u8::MAX, hex.dst_rid()),
                src1: ct_reg(u8::MAX, hex.src0_rid()),
                src2: ct_reg(u8::MAX, hex.src1_rid()),
                cst: PtArg::Const(PtConst {
                    val: hex.mul_factor(),
                }),
            }
        }
        DOpCode::ADDS | DOpCode::SUBS | DOpCode::SSUB | DOpCode::MULS => {
            let hex = PeArithMsgHex::from_bits(word);
            let dst = ct_reg(u8::MAX, hex.dst_rid());
            let src = ct_reg(u8::MAX, hex.src_rid());
            let cst = match hex.msg_mode() {
                IMM_VAR => {
                    let (id, block) = var(hex.msg_cst());
                    PtArg::Var(PtSrcVar { id, block })
                }
                _ => PtArg::Const(PtConst {
                    val: hex.msg_cst() as u8,
                }),
            };
            match dopcode {
                DOpCode::ADDS => ADDS { dst, src, cst },
                DOpCode::SUBS => SUBS { dst, src, cst },
                DOpCode::SSUB => SSUB { dst, src, cst },
                DOpCode::MULS => MULS { dst, src, cst },
                _ => panic!("Unknown DopCode {dopcode_raw} [{dopcode:?}]"),
            }
        }
        DOpCode::LD => {
            let hex = PeMemHex::from_bits(word);
            let dst = ct_reg(u8::MAX, hex.rid());
            let src = match hex.mode() {
                MEM_HEAP => CtMem::Heap(CtHeap {
                    addr: hex.slot().sas(),
                }),
                MEM_ADDR => CtMem::Io(CtIo {
                    addr: hex.slot().sas(),
                }),
                MEM_SRC => {
                    let (id, block) = var(hex.slot());
                    CtMem::Src(CtSrcVar { id, block })
                }
                m => panic!("LD: unsupported memory mode {m}"),
            };
            LD { dst, src }
        }
        DOpCode::ST => {
            let hex = PeMemHex::from_bits(word);
            let src = ct_reg(u8::MAX, hex.rid());
            let dst = match hex.mode() {
                MEM_HEAP => CtMem::Heap(CtHeap {
                    addr: hex.slot().sas(),
                }),
                MEM_ADDR => CtMem::Io(CtIo {
                    addr: hex.slot().sas(),
                }),
                MEM_DST => {
                    let (id, block) = var(hex.slot());
                    CtMem::Dst(CtDstVar { id, block })
                }
                m => panic!("ST: unsupported memory mode {m}"),
            };
            ST { dst, src }
        }
        DOpCode::PBS
        | DOpCode::PBS_ML2
        | DOpCode::PBS_ML4
        | DOpCode::PBS_ML8
        | DOpCode::PBS_F
        | DOpCode::PBS_ML2_F
        | DOpCode::PBS_ML4_F
        | DOpCode::PBS_ML8_F => {
            let hex = PePbsHex::from_bits(word);
            let src = ct_reg(u8::MAX, hex.src_rid());
            let is_plain = matches!(dopcode, DOpCode::PBS | DOpCode::PBS_F);
            let mask = match dopcode {
                DOpCode::PBS | DOpCode::PBS_F => u8::MAX,
                DOpCode::PBS_ML2 | DOpCode::PBS_ML2_F => MASK_PBS2,
                DOpCode::PBS_ML4 | DOpCode::PBS_ML4_F => MASK_PBS4,
                _ => MASK_PBS8,
            };
            let dst = ct_reg(mask, hex.dst_rid());
            let gid = if is_plain {
                resolve_gid(hex.gid())
            } else {
                hex.gid().sas()
            };
            let lut = LutRef { id: gid as u16 };
            match dopcode {
                DOpCode::PBS => PBS { dst, src, lut },
                DOpCode::PBS_ML2 => PBS_ML2 { dst, src, lut },
                DOpCode::PBS_ML4 => PBS_ML4 { dst, src, lut },
                DOpCode::PBS_ML8 => PBS_ML8 { dst, src, lut },
                DOpCode::PBS_F => PBS_F { dst, src, lut },
                DOpCode::PBS_ML2_F => PBS_ML2_F { dst, src, lut },
                DOpCode::PBS_ML4_F => PBS_ML4_F { dst, src, lut },
                DOpCode::PBS_ML8_F => PBS_ML8_F { dst, src, lut },
                _ => panic!("Unknown DopCode {dopcode_raw} [{dopcode:?}]"),
            }
        }
        DOpCode::WAIT => {
            let hex = PeUcoreHex::from_bits(word);
            let flag = UserFlag { flag: hex.flag() };
            let slot = if hex.hid() != 0 {
                Some(match hex.mode() {
                    MEM_ADDR => CtMem::Io(CtIo {
                        addr: hex.slot().sas(),
                    }),
                    MEM_HEAP => CtMem::Heap(CtHeap {
                        addr: hex.slot().sas(),
                    }),
                    m => panic!("WAIT: unsupported memory mode {m}"),
                })
            } else {
                None
            };
            WAIT { flag, slot }
        }
        DOpCode::NOTIFY => {
            let hex = PeUcoreHex::from_bits(word);
            let slot = match hex.mode() {
                MEM_ADDR => CtMem::Io(CtIo {
                    addr: hex.slot().sas(),
                }),
                // `generate_translation_table` maps both `CtHeap` and `CtSrcVar` to
                // `MEM_HEAP`; the two aren't distinguishable from the hex alone, so this
                // always decodes as `CtHeap` (see this function's doc comment).
                MEM_HEAP => CtMem::Heap(CtHeap {
                    addr: hex.slot().sas(),
                }),
                m => panic!("NOTIFY: unsupported memory mode {m}"),
            };
            NOTIFY {
                virt_id: VirtId { id: hex.hid() },
                flag: UserFlag { flag: hex.flag() },
                slot,
            }
        }
        DOpCode::LD_B2B => {
            let hex = PeUcoreHex::from_bits(word);
            let slot = match hex.mode() {
                MEM_ADDR => CtMem::Io(CtIo {
                    addr: hex.slot().sas(),
                }),
                MEM_HEAP => CtMem::Heap(CtHeap {
                    addr: hex.slot().sas(),
                }),
                m => panic!("LD_B2B: unsupported memory mode {m}"),
            };
            LD_B2B {
                flag: UserFlag { flag: hex.flag() },
                slot,
            }
        }
        _ => {
            panic!("Unsupported or unknown DOp opcode {dopcode_raw:#08b} in word {word:#010x}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc_langs::doplang::DopInstructionSet;

    fn instructions(ir: &IR<DopLang>) -> Vec<DopInstructionSet> {
        ir.walk_ops_linear()
            .map(|op| op.get_instruction().clone())
            .filter(|instr| !matches!(instr, DopInstructionSet::_START | DopInstructionSet::_END))
            .collect()
    }

    #[test]
    fn round_trips_through_generate_and_decode() {
        use DopInstructionSet::*;

        let mut ir: IR<DopLang> = IR::empty();
        let (_, start) = ir.add_op(_START, svec![]);
        let (_, r1) = ir.add_op(
            LD {
                dst: CtReg::new(1_u8),
                src: CtMem::heap(3_u16),
            },
            svec![start[0]],
        );
        let (_, r2) = ir.add_op(
            ADD {
                dst: CtReg::new(2_u8),
                src1: CtReg::new(1_u8),
                src2: CtReg::new(1_u8),
            },
            svec![r1[0]],
        );
        let (_, r3) = ir.add_op(
            MAC {
                dst: CtReg::new(3_u8),
                src1: CtReg::new(1_u8),
                src2: CtReg::new(2_u8),
                cst: PtArg::cst(4),
            },
            svec![r2[0]],
        );
        let (_, r4) = ir.add_op(
            ADDS {
                dst: CtReg::new(4_u8),
                src: CtReg::new(3_u8),
                cst: PtArg::var(2, 1),
            },
            svec![r3[0]],
        );
        let (_, r5) = ir.add_op(
            PBS {
                dst: CtReg::new(5_u8),
                src: CtReg::new(4_u8),
                lut: LutRef::from(LutId(0)),
            },
            svec![r4[0]],
        );
        let (_, r6) = ir.add_op(
            PBS_ML2 {
                dst: CtReg::ml2(6_u8),
                src: CtReg::new(5_u8),
                lut: LutRef::from(LutId(1)),
            },
            svec![r5[0]],
        );
        let (_, r7) = ir.add_op(
            ST {
                dst: CtMem::io(7_u16),
                src: CtReg::new(6_u8),
            },
            svec![r6[0]],
        );
        ir.add_op(_END, svec![r7[0]]);

        let words = generate_translation_table(&ir, None);
        let decoded = decode_translation_table(&words, None);

        assert_eq!(instructions(&ir), instructions(&decoded));
    }
}
