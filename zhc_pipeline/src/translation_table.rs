//! Translation table generation for device operations.
//!
//! This module provides functionality to generate binary instruction encodings
//! from device operation intermediate representations. It defines the binary
//! formats for different instruction types and converts the IR into executable
//! machine code for the target HPU hardware.

use bitfield_struct::bitfield;
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
pub fn generate_translation_table(ir: &IR<DopLang>) -> Vec<DOpRepr> {
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
                        .with_dst_rid(dst.sas())
                        .with_src0_rid(src1.sas())
                        .with_src1_rid(src2.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src0_rid(src1.sas())
                        .with_src1_rid(src2.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src0_rid(src1.sas())
                        .with_src1_rid(src2.sas())
                        .with_mul_factor(cst.sas())
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
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst.sas())
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
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid.sas::<u16>()) << 8) + bid.sas::<u16>())
                    .with_opcode(DOpCode::ADDS as u8)
                    .0,
            ),
            SUBS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst.sas())
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
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid.sas::<u16>()) << 8) + bid.sas::<u16>())
                    .with_opcode(DOpCode::SUBS as u8)
                    .0,
            ),
            SSUB {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst.sas())
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
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid.sas::<u16>()) << 8) + bid.sas::<u16>())
                    .with_opcode(DOpCode::SSUB as u8)
                    .0,
            ),
            MULS {
                dst: CtReg { addr: dst, .. },
                src: CtReg { addr: src, .. },
                cst: PtConst { val: cst },
            } => output.push(
                PeArithMsgHex::new()
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_CST)
                    .with_msg_cst(cst.sas())
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
                    .with_dst_rid(dst.sas())
                    .with_src_rid(src.sas())
                    .with_msg_mode(IMM_VAR)
                    .with_msg_cst(((tid.sas::<u16>()) << 8) + bid.sas::<u16>())
                    .with_opcode(DOpCode::MULS as u8)
                    .0,
            ),
            LD {
                dst: CtReg { addr: dst, .. },
                src: CtHeap { addr: src },
            } => {
                output.push(
                    PeMemHex::new()
                        .with_rid(dst.sas())
                        .with_mode(MEM_HEAP)
                        .with_slot(src.sas())
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
                        .with_rid(dst.sas())
                        .with_mode(MEM_ADDR)
                        .with_slot(src.sas())
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
                        .with_rid(dst.sas())
                        .with_mode(MEM_SRC)
                        .with_slot(((tid.sas::<u16>()) << 8) + bid.sas::<u16>())
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
                        .with_rid(src.sas())
                        .with_mode(MEM_HEAP)
                        .with_slot(dst.sas())
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
                        .with_rid(src.sas())
                        .with_mode(MEM_ADDR)
                        .with_slot(dst.sas())
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
                        .with_rid(src.sas())
                        .with_mode(MEM_DST)
                        .with_slot(((tid.sas::<u16>()) << 8) + bid.sas::<u16>())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
                        .with_gid(gid.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
                        .with_gid(gid.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
                        .with_gid(gid.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
                        .with_gid(gid.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
                        .with_gid(gid.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
                        .with_gid(gid.sas())
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
                        .with_dst_rid(dst.sas())
                        .with_src_rid(src.sas())
                        .with_gid(gid.sas())
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
                    Some(CtIo { addr }) => (1, MEM_ADDR, addr as u16),
                    Some(CtHeap { addr }) => (1, MEM_HEAP, addr as u16),
                    Some(_) => panic!("Unexpected slot argument in WAIT"),
                    None => (0, 0, 0),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex)
                        .with_mode(mode_hex)
                        .with_flag(flag)
                        .with_hid(has_data)
                        .0,
                );
            }
            NOTIFY {
                virt_id: VirtId { id: vid },
                flag: UserFlag { flag },
                slot,
            } => {
                let (mode, slot_hex) = match slot {
                    CtIo { addr } => (MEM_ADDR, addr as u16),
                    CtHeap { addr } => (MEM_HEAP, addr as u16),
                    CtSrcVar { id, block } => (MEM_HEAP, ((id as u16) << 8) + block as u16),
                    _ => panic!("Unexpected slot argument in NOTIFY"),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex as u16)
                        .with_mode(mode)
                        .with_flag(flag)
                        .with_hid(vid)
                        .0,
                );
            }
            LD_B2B {
                flag: UserFlag { flag },
                slot,
            } => {
                let (mode, slot_hex) = match slot {
                    CtIo { addr } => (MEM_ADDR, addr as u16),
                    CtHeap { addr } => (MEM_HEAP, addr as u16),
                    _ => panic!("Unexpected slot argument in LD_B2B"),
                };
                output.push(
                    PeUcoreHex::new()
                        .with_slot(slot_hex as u16)
                        .with_mode(mode)
                        .with_flag(flag)
                        .with_hid(0) // Unused
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

#[cfg(test)]
mod test {

    use zhc_builder::{CiphertextSpec, add, cmp_gt};
    use zhc_ir::IR;
    use zhc_langs::ioplang::IopLang;
    use zhc_sim::hpu::{HpuConfig, PhysicalConfig};
    use zhc_utils::assert_display_is;

    use crate::{
        allocator::allocate_registers,
        scheduler::{self, SchedPolicy},
        translation::lower_iop_to_hpu,
    };

    use super::generate_translation_table;

    fn pipeline(ir: &IR<IopLang>) -> Vec<u32> {
        let ir = lower_iop_to_hpu(&ir).output;
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        let scheduled = scheduler::two_step::schedule(
            &ir,
            &config,
            SchedPolicy::AsLateAsPossible,
            SchedPolicy::AsLateAsPossible,
        );
        let allocated = allocate_registers(&scheduled, &config);
        generate_translation_table(&allocated)
    }

    fn format_binary_vec(inp: &Vec<u32>) -> String {
        inp.iter()
            .map(|a| format!("{:#034b},", a))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_hex_add_ir() {
        let hex = pipeline(&add(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            format_binary_vec(&hex),
            r#"
                0b00000000000000000000000001000100,
                0b10000000000000000000101000000000,
                0b10000000000001000000101000000001,
                0b10000000000000000000001000000010,
                0b00000100000000000100000000000000,
                0b10000000000001000000001000000001,
                0b10000000000000000000111000000011,
                0b00000100000000000100000100000001,
                0b10000000000001000000111000000010,
                0b10000000000000000000011000000100,
                0b00000100000000001000000110000010,
                0b10000000000001000000011000000011,
                0b00000100000000001100001000000011,
                0b10000000000000000001001000000100,
                0b10000000000001000001001000000101,
                0b10000000000000000001011000000110,
                0b00000100000000010100001000000100,
                0b10000000000001000001011000000101,
                0b10000000000000000001101000000111,
                0b00000100000000010100001100000101,
                0b10000000000001000001101000000110,
                0b00000100000000011000001110000110,
                0b11000000000011000100000100000111,
                0b11000000000011000000000000001000,
                0b11000100000001101000000010001010,
                0b11100000000010111100000110001001,
                0b00000100000000100100010110000001,
                0b00000100000000100000000010001000,
                0b00000100000000011100010000000111,
                0b11000000000010111000001110001001,
                0b11000000000011000100001100001100,
                0b11000000000010111100001000001101,
                0b11100000000011000000001010001110,
                0b00000100000000111000011010000111,
                0b00000100000000100100011010001101,
                0b00000100000000110000001110001100,
                0b00000100000000100100001110000111,
                0b00000100000000100100011000001100,
                0b10000000000000000001111000001110,
                0b10000000000001000001111000001111,
                0b00000100000000111100011100001110,
                0b00000100000000101100000110000011,
                0b00000100000000100100001000000100,
                0b11000000000010110000000010001001,
                0b11000000000010110100010000001011,
                0b11000000000010110000011010001111,
                0b11000000000010110100001110010000,
                0b11100000000010111000011000010001,
                0b00000100000000111100001010000001,
                0b00000100000000101100000100000010,
                0b00000100000001000000001100000101,
                0b00000100000000100100000000000000,
                0b00000100000001000100011100000110,
                0b11000000000000000100010100000111,
                0b11000000000000000100000110001000,
                0b11000000000000000100000000001001,
                0b11000000000000000100000100001011,
                0b11000000000000000100001000001100,
                0b11000000000000000100000010001101,
                0b11000000000000000100001010001110,
                0b11100000000000000100001100001111,
                0b10000100000000000000111100001011,
                0b10000100000000000001001100001100,
                0b10000100000000000000101100001001,
                0b10000100000000000001011100001101,
                0b10000100000000000000011100001000,
                0b10000100000000000001101100001110,
                0b10000100000000000000001100000111,
                0b10000100000000000001111100001111,
            "#
        );
    }

    #[test]
    fn test_hex_cmp_ir() {
        let hex = pipeline(&cmp_gt(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            format_binary_vec(&hex),
            r#"
                0b00000000000000000000000000110011,
                0b10000000000001000000101000000000,
                0b10000000000001000000111000000001,
                0b10000000000000000001001000000010,
                0b00010100100000000000000010000000,
                0b10000000000000000001011000000001,
                0b10000000000000000000101000000011,
                0b00010100100000001000000010000001,
                0b10000000000000000000111000000010,
                0b10000000000001000001001000000100,
                0b00010100100000001100000100000010,
                0b10000000000001000001011000000011,
                0b10000000000001000000001000000101,
                0b00010100100000010000000110000011,
                0b10000000000001000000011000000100,
                0b10000000000000000001101000000110,
                0b00010100100000010100001000000100,
                0b10000000000000000001111000000101,
                0b10000000000000000000001000000111,
                0b00010100100000011000001010000101,
                0b10000000000000000000011000000110,
                0b10000000000001000001101000001000,
                0b00010100100000011100001100000110,
                0b10000000000001000001111000000111,
                0b00010100100000100000001110000111,
                0b11000000000000000000001100001000,
                0b11000000000000000000001000001001,
                0b11000000000000000000000100001010,
                0b11000000000000000000000000001011,
                0b11000000000000000000000010001100,
                0b11000000000000000000000110001101,
                0b11000000000000000000001010001110,
                0b11100000000000000000001110001111,
                0b00001000000000101100010100000000,
                0b00001000000000110100011000000001,
                0b00001000000000100100010000000010,
                0b00001000000000111100011100000011,
                0b11000000000010100000000100000100,
                0b11000000000010100000000000000101,
                0b11000000000010100000000010000110,
                0b11100000000010100000000110000111,
                0b00100100000000001000001010000000,
                0b00100100000000001000001110000001,
                0b00100100000000001000001000000010,
                0b00100100000000001000001100000011,
                0b00010100100000001000000000000000,
                0b00010100100000001100000010000001,
                0b11000000000011001100000000000010,
                0b11100000000011001100000010000011,
                0b00010100100000001000000110000000,
                0b11100000000001101100000000000001,
                0b10000100000000000000001100000001,
            "#
        );
    }
}
