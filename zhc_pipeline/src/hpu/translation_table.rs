//! Translation table generation for device operations.
//!
//! This module provides functionality to generate binary instruction encodings
//! from device operation intermediate representations. It defines the binary
//! formats for different instruction types and converts the IR into executable
//! machine code for the target HPU hardware.

use bitfield_struct::bitfield;
use zhc_crypto::integer_semantics::lut::LutId;
use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_utils::SafeAs;

/// Binary representation of a device operation instruction.
pub type DOpRepr = u32;

#[allow(non_camel_case_types, dead_code)]
enum DOpCode {
    ADD = 0b00_0001,
    SUB = 0b00_0010,
    MAC = 0b00_0101,
    ADDS = 0b00_1001,
    SUBS = 0b00_1010,
    SSUB = 0b00_1011,
    MULS = 0b00_1100,
    LD = 0b10_0000,
    ST = 0b10_0001,
    SYNC = 0b10_1111,
    NOTIFY = 0b01_0000,
    WAIT = 0b01_0001,
    LD_B2B = 0b01_1000,
    EXTEND = 0b01_1111,
    PBS = 0b11_0000,
    PBS_ML2 = 0b11_0001,
    PBS_ML4 = 0b11_0010,
    PBS_ML8 = 0b11_0011,
    PBS_F = 0b11_1000,
    PBS_ML2_F = 0b11_1001,
    PBS_ML4_F = 0b11_1010,
    PBS_ML8_F = 0b11_1011,
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
        use zhc_langs::doplang::Argument::*;
        use zhc_langs::doplang::DopInstructionSet::*;
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
                        .with_opcode(DOpCode::ADD as u8)
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
                        .with_opcode(DOpCode::SUB as u8)
                        .0,
                );
            }
            MAC {
                dst: CtReg { addr: dst, .. },
                src1: CtReg { addr: src1, .. },
                src2: CtReg { addr: src2, .. },
                cst: PtConst { val: cst },
            } => {
                output.push(
                    PeArithHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src0_rid((*src1).sas())
                        .with_src1_rid((*src2).sas())
                        .with_mul_factor((*cst).sas())
                        .with_opcode(DOpCode::MAC as u8)
                        .0,
                );
            }
            ADDS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(DOpCode::ADDS as u8)
                    .0,
            ),
            ADDS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtSrcVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(DOpCode::ADDS as u8)
                    .0,
            ),
            SUBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(DOpCode::SUBS as u8)
                    .0,
            ),
            SUBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtSrcVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(DOpCode::SUBS as u8)
                    .0,
            ),
            SSUB {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(DOpCode::SSUB as u8)
                    .0,
            ),
            SSUB {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtSrcVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(DOpCode::SSUB as u8)
                    .0,
            ),
            MULS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst((*cst).sas())
                    .with_opcode(DOpCode::MULS as u8)
                    .0,
            ),
            MULS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtSrcVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid((*dst).sas())
                    .with_src_rid((*src).sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                    .with_opcode(DOpCode::MULS as u8)
                    .0,
            ),
            LD {
                dst: CtReg { addr: dst, .. },
                src: CtHeap { addr: src },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*dst).sas())
                        .with_mode(MEM_HEAP)
                        .with_slot((*src).sas())
                        .with_opcode(DOpCode::LD as u8)
                        .0,
                );
            }
            LD {
                dst: CtReg { addr: dst, .. },
                src: CtIo { addr: src },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*dst).sas())
                        .with_mode(MEM_ADDR)
                        .with_slot((*src).sas())
                        .with_opcode(DOpCode::LD as u8)
                        .0,
                );
            }
            LD {
                dst: CtReg { addr: dst, .. },
                src:
                    CtSrcVar {
                        id: tid,
                        block: bid,
                    },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*dst).sas())
                        .with_mode(MEM_SRC)
                        .with_slot((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                        .with_opcode(DOpCode::LD as u8)
                        .0,
                );
            }
            ST {
                dst: CtHeap { addr: dst },
                src: CtReg { addr: src, .. },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*src).sas())
                        .with_mode(MEM_HEAP)
                        .with_slot((*dst).sas())
                        .with_opcode(DOpCode::ST as u8)
                        .0,
                );
            }
            ST {
                dst: CtIo { addr: dst },
                src: CtReg { addr: src, .. },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*src).sas())
                        .with_mode(MEM_ADDR)
                        .with_slot((*dst).sas())
                        .with_opcode(DOpCode::ST as u8)
                        .0,
                );
            }
            ST {
                dst:
                    CtDstVar {
                        id: tid,
                        block: bid,
                    },
                src: CtReg { addr: src, .. },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid((*src).sas())
                        .with_mode(MEM_DST)
                        .with_slot((((*tid).sas::<u16>()) << 8) + (*bid).sas::<u16>())
                        .with_opcode(DOpCode::ST as u8)
                        .0,
                );
            }
            PBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                let gid = match lut_relocation {
                    Some(reloc) => reloc.get(*gid).unwrap().0,
                    None => *gid,
                };
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid(gid.sas())
                        .with_opcode(DOpCode::PBS as u8)
                        .0,
                );
            }
            PBS_ML2 {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(DOpCode::PBS_ML2 as u8)
                        .0,
                );
            }
            PBS_ML4 {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(DOpCode::PBS_ML4 as u8)
                        .0,
                );
            }
            PBS_ML8 {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(DOpCode::PBS_ML8 as u8)
                        .0,
                );
            }
            PBS_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(DOpCode::PBS_F as u8)
                        .0,
                );
            }
            PBS_ML2_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(DOpCode::PBS_ML2_F as u8)
                        .0,
                );
            }
            PBS_ML4_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(DOpCode::PBS_ML4_F as u8)
                        .0,
                );
            }
            PBS_ML8_F {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid((*dst).sas())
                        .with_src_rid((*src).sas())
                        .with_gid((*gid).sas())
                        .with_opcode(DOpCode::PBS_ML8_F as u8)
                        .0,
                );
            }

            // Multi-hpu related DOp
            WAIT {
                slot,
                flag: UserFlag { flag },
            } => {
                let (has_data, mode_hex, slot_hex) = match slot {
                    Some(CtIo { addr }) => (1, MEM_ADDR, *addr as u16),
                    Some(CtHeap { addr }) => (1, MEM_HEAP, *addr as u16),
                    Some(_) => panic!("Unexpected slot argument in WAIT"),
                    None => (0, 0, 0),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex)
                        .with_mode(mode_hex)
                        .with_flag(*flag)
                        .with_hid(has_data)
                        .with_opcode(DOpCode::WAIT as u8)
                        .0,
                );
            }
            NOTIFY {
                virt_id: VirtId { id: vid },
                flag: UserFlag { flag },
                slot,
            } => {
                let (mode, slot_hex) = match slot {
                    CtIo { addr } => (MEM_ADDR, *addr as u16),
                    CtHeap { addr } => (MEM_HEAP, *addr as u16),
                    CtSrcVar { id, block } => (MEM_HEAP, ((*id as u16) << 8) + *block as u16),
                    _ => panic!("Unexpected slot argument in NOTIFY"),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex as u16)
                        .with_mode(mode)
                        .with_flag(*flag)
                        .with_hid(*vid)
                        .with_opcode(DOpCode::NOTIFY as u8)
                        .0,
                );
            }
            LD_B2B {
                flag: UserFlag { flag },
                slot,
            } => {
                let (mode, slot_hex) = match slot {
                    CtIo { addr } => (MEM_ADDR, *addr as u16),
                    CtHeap { addr } => (MEM_HEAP, *addr as u16),
                    _ => panic!("Unexpected slot argument in LD_B2B"),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex as u16)
                        .with_mode(mode)
                        .with_flag(*flag)
                        .with_hid(0) // Unused
                        .with_opcode(DOpCode::LD_B2B as u8)
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
