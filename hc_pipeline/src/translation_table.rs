//! Translation table generation for device operations.
//!
//! This module provides functionality to generate binary instruction encodings
//! from device operation intermediate representations. It defines the binary
//! formats for different instruction types and converts the IR into executable
//! machine code for the target HPU hardware.

use bitfield_struct::bitfield;
use hc_ir::IR;
use hc_langs::doplang::DopLang;

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
    SYNC = 0b01_0000,
    LD = 0b10_0000,
    ST = 0b10_0001,
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

/// PeSync instructions
#[bitfield(u32)]
pub struct PeSyncHex {
    #[bits(26)]
    sid: u32,
    #[bits(6)]
    opcode: u8,
}

/// Generates binary instruction encodings from device operation IR.
///
/// Converts the intermediate representation `ir` containing device operations
/// into a vector of binary instruction representations suitable for execution
/// on the target hardware.
pub fn generate_translation_table(ir: &IR<DopLang>) -> Vec<DOpRepr> {
    let mut output = Vec::with_capacity(ir.n_ops() as usize);
    output.push(0); // reserve room for the length of the stream at the beginning of the stream.
    for op in ir.walk_ops_topological() {
        use hc_langs::doplang::Argument::*;
        use hc_langs::doplang::DopInstructionSet::*;
        match op.get_operation() {
            ADD {
                dst: CtReg { addr: dst, .. },
                src1: CtReg { addr: src1, .. },
                src2: CtReg { addr: src2, .. },
            } => {
                output.push(
                    PeArithHex::new()
                        .with_dst_rid(dst as u8)
                        .with_src0_rid(src1 as u8)
                        .with_src1_rid(src2 as u8)
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
                        .with_dst_rid(dst as u8)
                        .with_src0_rid(src1 as u8)
                        .with_src1_rid(src2 as u8)
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
                        .with_dst_rid(dst as u8)
                        .with_src0_rid(src1 as u8)
                        .with_src1_rid(src2 as u8)
                        .with_mul_factor(cst as u8)
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
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst as u16)
                    .with_opcode(DOpCode::ADDS as u8)
                    .0,
            ),
            ADDS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid as u16) << 8) + bid as u16)
                    .with_opcode(DOpCode::ADDS as u8)
                    .0,
            ),
            SUBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst as u16)
                    .with_opcode(DOpCode::SUBS as u8)
                    .0,
            ),
            SUBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid as u16) << 8) + bid as u16)
                    .with_opcode(DOpCode::SUBS as u8)
                    .0,
            ),
            SSUB {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst as u16)
                    .with_opcode(DOpCode::SSUB as u8)
                    .0,
            ),
            SSUB {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid as u16) << 8) + bid as u16)
                    .with_opcode(DOpCode::SSUB as u8)
                    .0,
            ),
            MULS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst as u16)
                    .with_opcode(DOpCode::MULS as u8)
                    .0,
            ),
            MULS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst:
                    PtVar {
                        id: tid,
                        block: bid,
                    },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst as u8)
                    .with_src_rid(src as u8)
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid as u16) << 8) + bid as u16)
                    .with_opcode(DOpCode::MULS as u8)
                    .0,
            ),
            LD {
                dst: CtReg { addr: dst, .. },
                src: CtHeap { addr: src },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid(dst as u8)
                        .with_mode(MEM_HEAP)
                        .with_slot(src as u16)
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
                        .with_rid(dst as u8)
                        .with_mode(MEM_ADDR)
                        .with_slot(src as u16)
                        .with_opcode(DOpCode::LD as u8)
                        .0,
                );
            }
            LD {
                dst: CtReg { addr: dst, .. },
                src:
                    CtVar {
                        id: tid,
                        block: bid,
                    },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid(dst as u8)
                        .with_mode(MEM_SRC)
                        .with_slot(((tid as u16) << 8) + bid as u16)
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
                        .with_rid(src as u8)
                        .with_mode(MEM_HEAP)
                        .with_slot(dst as u16)
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
                        .with_rid(src as u8)
                        .with_mode(MEM_ADDR)
                        .with_slot(dst as u16)
                        .with_opcode(DOpCode::ST as u8)
                        .0,
                );
            }
            ST {
                dst:
                    CtVar {
                        id: tid,
                        block: bid,
                    },
                src: CtReg { addr: src, .. },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid(src as u8)
                        .with_mode(MEM_DST)
                        .with_slot(((tid as u16) << 8) + bid as u16)
                        .with_opcode(DOpCode::ST as u8)
                        .0,
                );
            }
            PBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                lut: LutId { id: gid },
            } => {
                output.push(
                    PePbsHex::new()
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
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
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
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
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
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
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
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
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
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
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
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
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
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
                        .with_dst_rid(dst as u8)
                        .with_src_rid(src as u8)
                        .with_gid(gid as u16)
                        .with_opcode(DOpCode::PBS_ML8_F as u8)
                        .0,
                );
            }
            SYNC | _INIT => {}
            _ => {
                panic!("Unexpected Doplang Operation encountered")
            }
        };
    }
    output[0] = (output.len() - 1) as u32;
    output
}

#[cfg(test)]
mod test {

    use hc_ir::{IR, translation::Translator};
    use hc_langs::ioplang::IopLang;
    use hc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{
        allocator::allocate_registers,
        batcher::batch,
        scheduler::schedule,
        test::{get_add_ir, get_cmp_ir},
        translation::IoplangToHpulang,
    };

    use super::generate_translation_table;

    fn pipeline(ir: &IR<IopLang>) -> Vec<u32> {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        let scheduled = schedule(&ir, &config);
        let batched = batch(&scheduled);
        let allocated = allocate_registers(&batched, &config);
        generate_translation_table(&allocated)
    }

    #[test]
    fn test_hex_add_ir() {
        let hex = pipeline(&get_add_ir(16, 2, 2));
        // for a in hex.iter() {
        //     println!("{:#034b},", a);
        // }
        assert_eq!(
            hex,
            vec![
                0b00000000000000000000000001000100,
                0b10000000000000000000001000000000,
                0b10000000000000000000011000000001,
                0b10000000000000000000101000000010,
                0b10000000000000000000111000000011,
                0b10000000000000000001001000000100,
                0b10000000000000000001011000000101,
                0b10000000000000000001101000000110,
                0b10000000000000000001111000000111,
                0b10000000000001000000001000001000,
                0b00000100000000100000000000001001,
                0b10000000000001000000011000000000,
                0b10000000000001000000101000001000,
                0b10000000000001000000111000001010,
                0b10000000000001000001001000001011,
                0b00000100000000000000000010001100,
                0b10000000000001000001011000000000,
                0b10000000000001000001101000000001,
                0b10000000000001000001111000001101,
                0b00000100000000100000000100001110,
                0b00000100000000101000000110000010,
                0b00000100000000101100001000000011,
                0b00000100000000000000001010000100,
                0b00000100000000000100001100000000,
                0b00000100000000110100001110000001,
                0b11000100000001101000010010000110,
                0b11000000000010111100011000000101,
                0b11000000000011000000011100001000,
                0b11000000000011000100000100001010,
                0b11000000000010111100000110001011,
                0b11000000000011000000001000001101,
                0b11100000000011000100000000001111,
                0b00000100000000011100011000001001,
                0b00000100000000010100001110001100,
                0b00000100000000110100010110000101,
                0b11000000000000000100001100000111,
                0b11000000000000000100010010001101,
                0b11100000000010110000011000010000,
                0b00000100000000100000011000000110,
                0b00000100000000111100001010001000,
                0b10000100000000000000001100000111,
                0b10000100000000000000011100001101,
                0b00000100000000101000001100000111,
                0b00000100000001000000011100001001,
                0b11000000000010110100001100001010,
                0b11000000000010111000001110001100,
                0b11100000000000000100010010001101,
                0b00000100000000101000000100000110,
                0b10000100000000000000101100001101,
                0b00000100000000110000010110000010,
                0b00000100000000110000001010000111,
                0b00000100000000110000010000000101,
                0b00000100000000110000000110001000,
                0b11000000000000000100001100000011,
                0b11000000000010110000000100001001,
                0b11000000000010110100001110001010,
                0b11000000000010111000001010001011,
                0b11100000000000000100010000001100,
                0b10000100000000000000111100000011,
                0b00000100000000100100001000000010,
                0b10000100000000000001001100001100,
                0b00000100000000101000000000000011,
                0b00000100000000101100000010000000,
                0b11000000000000000100000100000001,
                0b11000000000000000100000110000100,
                0b11100000000000000100000000000101,
                0b10000100000000000001011100000001,
                0b10000100000000000001101100000100,
                0b10000100000000000001111100000101,
            ]
        );
    }

    #[test]
    fn test_hex_cmp_ir() {
        let hex = pipeline(&get_cmp_ir(16, 2, 2));
        // for a in hex.iter() {
        //     println!("{:#034b},", a);
        // }
        assert_eq!(
            hex,
            vec![
                0b00000000000000000000000000110011,
                0b10000000000000000000001000000000,
                0b10000000000000000000011000000001,
                0b00010100100000000000000010000010,
                0b10000000000000000000101000000000,
                0b10000000000000000000111000000001,
                0b10000000000000000001001000000011,
                0b10000000000000000001011000000100,
                0b00010100100000000000000010000101,
                0b10000000000000000001101000000000,
                0b10000000000000000001111000000001,
                0b10000000000001000000001000000110,
                0b10000000000001000000011000000111,
                0b00010100100000001100001000001000,
                0b10000000000001000000101000000011,
                0b10000000000001000000111000000100,
                0b10000000000001000001001000001001,
                0b10000000000001000001011000001010,
                0b00010100100000000000000010001011,
                0b10000000000001000001101000000000,
                0b10000000000001000001111000000001,
                0b00010100100000011000001110001100,
                0b00010100100000001100001000000110,
                0b00010100100000100100010100000011,
                0b00010100100000000000000010000100,
                0b11000000000000000000000100000000,
                0b11000000000000000000001010000001,
                0b11000000000000000000010000000111,
                0b11000000000000000000010110001001,
                0b11000000000000000000011000001010,
                0b11000000000000000000001100001101,
                0b11000000000000000000000110001110,
                0b11100000000000000000001000001111,
                0b00001000000000101000000000000010,
                0b00001000000000110100000010000000,
                0b00001000000000111000001110000001,
                0b00001000000000111100010010000011,
                0b11000000000000101000000100000100,
                0b11000000000000101000000000000101,
                0b11000000000000101000000010000110,
                0b11100000000000101000000110000111,
                0b00100100000000001000001000000000,
                0b00100100000000001000001010000001,
                0b00100100000000001000001100000010,
                0b00100100000000001000001110000011,
                0b00010100100000000000000010000100,
                0b00010100100000001000000110000000,
                0b11000000000000101100001000000001,
                0b11100000000000101100000000000010,
                0b00010100100000000100000100000000,
                0b11100000000001101100000000000001,
                0b10000100000000000000001100000001,
            ]
        );
    }
}
