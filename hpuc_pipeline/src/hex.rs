use bitfield_struct::bitfield;
use hpuc_ir::IR;
use hpuc_langs::doplang::Doplang;

pub type DOpRepr = u32;

#[allow(non_camel_case_types)]
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

/// DOp raw encoding used for Opcode extraction
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

pub fn codegen(ir: &IR<Doplang>) -> Vec<DOpRepr> {
    let mut output = Vec::with_capacity(ir.n_ops() as usize);
    for op in ir.walk_ops_topological() {
        use hpuc_langs::doplang::Argument::*;
        use hpuc_langs::doplang::Operations::*;
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
            SYNC => {
                output.push(
                    PeSyncHex::new()
                        .with_sid(0)
                        .with_opcode(DOpCode::SYNC as u8)
                        .0,
                );
            }
            _INIT => {}
            _ => {
                panic!("Unexpected Doplang Operation encountered")
            }
        };
    }
    output
}

#[cfg(test)]
mod test {

    use hpuc_ir::{IR, scheduling::forward::ForwardScheduler, translation::Translator};
    use hpuc_langs::ioplang::Ioplang;
    use hpuc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{
        allocator::allocate_registers,
        scheduler::Scheduler,
        test::{get_add_ir, get_cmp_ir, get_sub_ir},
        translation::IoplangToHpulang,
    };

    use super::codegen;

    fn pipeline(ir: &IR<Ioplang>) -> Vec<u32> {
        let mut ir = IoplangToHpulang.translate(&ir);
        let mut config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        config.regf_size = 10;
        let mut scheduler = Scheduler::init(&ir, &config);
        let schedule = scheduler.schedule(&ir);
        let flusher = scheduler.into_flusher();
        flusher.apply_flushes(&mut ir);
        let allocated = allocate_registers(&ir, schedule.get_walker(), &config);
        codegen(&allocated)
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
                0b10000000000000000000001000000000,
                0b10000000000000000000011000000001,
                0b10000000000000000000101000000010,
                0b10000000000000000000111000000011,
                0b10000000000000000001001000000100,
                0b10000000000000000001011000000101,
                0b10000000000000000001101000000110,
                0b10000000000001000000001000000111,
                0b00000100000000011100000000000000,
                0b10000000000001000000011000000111,
                0b10000000000001000000101000001000,
                0b10000000000001000000111000001001,
                0b10000100000000000000000100000110,
                0b10000000000001000001001000000110,
                0b00000100000000011100000010000001,
                0b10000100000000000000010100000110,
                0b11000100000001101000000000000110,
                0b10000000000001000001011000000000,
                0b10000100000000000000100100000110,
                0b10000000000001000001101000000110,
                0b00000100000000100000000100000010,
                0b11000000000010111100000010001000,
                0b00000100000000100100000110000011,
                0b11000000000011000000000100001001,
                0b10000100000000000000110100000010,
                0b10000000000000000000010100000010,
                0b00000100000000001000001000000100,
                0b10000100000000000001000100000011,
                0b10000100000000000001010100001001,
                0b10000000000000000001000100001001,
                0b11000000000011000100010010000011,
                0b00000100000000000000001010000000,
                0b11000000000010111100001000000101,
                0b10000100000000000001100100000100,
                0b10000000000000000000000100000100,
                0b00000100000000011000001000000110,
                0b10000100000000000001110100000000,
                0b10000000000000000001110100001001,
                0b11000000000011000000010010000000,
                0b10000100000000000010000100000110,
                0b10000000000000000010000100001001,
                0b11100000000011000100010010000110,
                0b00000100000000011100000010000001,
                0b10000000000000000000100100001001,
                0b10000100000000000000001100001001,
                0b00000100000000011100010000000111,
                0b10000100000000000000011100000001,
                0b00000100000000010100000000000000,
                0b11100000000010110000001110000001,
                0b10000000000000000001010100001000,
                0b00000100000000011100010000000111,
                0b00000100000000000000001100000110,
                0b10000100000000000010010100000110,
                0b11100000000010110100001110000110,
                0b00000100000000011100000110000011,
                0b10000000000000000000110100000111,
                0b00000100000000000100001110000001,
                0b11100000000010111000000110000011,
                0b10000100000000000010100100000000,
                0b10000000000000000001000100000000,
                0b00000100000000011000000000000110,
                0b10000100000000000000101100000001,
                0b10000100000000000000111100000110,
                0b00000100000000001100001010000001,
                0b10000000000000000010100100000110,
                0b00000100000000001100001100000101,
                0b11000000000010111000000010000001,
                0b10000100000000000010110100000001,
                0b10000000000000000010010100000001,
                0b00000100000000001100000010000011,
                0b11000000000010110000001010000101,
                0b11100000000010110100000110000011,
                0b10000100000000000011000100000011,
                0b10000000000000000001100100001001,
                0b10000000000000000010110100001000,
                0b00000100000000100000010010000011,
                0b10000000000000000001110100001001,
                0b00000100000000010100010010000101,
                0b10000100000000000001001100000011,
                0b10000000000000000010000100001001,
                0b10000000000000000011000100001000,
                0b00000100000000100000010010000011,
                0b10000100000000000001011100000101,
                0b10000100000000000001101100000011,
            ]
        );
    }

    #[test]
    fn test_allocate_sub_ir() {
        let hex = pipeline(&get_sub_ir(16, 2, 2));
        // for a in hex.iter() {
        //     println!("{:#034b},", a);
        // }
        assert_eq!(
            hex,
            vec![
                0b10000000000000000000001000000000,
                0b10000000000000000000011000000001,
                0b10000000000000000000101000000010,
                0b10000000000000000000111000000011,
                0b10000000000000000001001000000100,
                0b10000000000000000001011000000101,
                0b10000000000000000001101000000110,
                0b10000000000001000000001000000111,
                0b00101100000000011000001110000111,
                0b10000000000001000000011000001000,
                0b10000000000001000000101000001001,
                0b10000100000000000000000100000110,
                0b10000000000001000000111000000110,
                0b10000100000000000000010100000101,
                0b10000000000001000001001000000101,
                0b00101100000000011000010000001000,
                0b10000100000000000000100100000100,
                0b10000000000001000001011000000100,
                0b10000100000000000000110100000011,
                0b10000000000001000001101000000011,
                0b00101100000000011000010010001001,
                0b00101100000000011000001100000110,
                0b00101100000000011000001010000101,
                0b00101100000000011000001000000100,
                0b00101100000000011000000110000011,
                0b00000100000000011100000000000000,
                0b00000100000000100000000010000001,
                0b10000100000000000001000100000110,
                0b11000100000001101000000000000110,
                0b00000100000000100100000100000000,
                0b11000000000010111100000010000010,
                0b10000000000000000000110100001001,
                0b10000100000000000001010100000010,
                0b10000000000000000001000100000010,
                0b00000100000000001000010010001000,
                0b10000100000000000001100100000000,
                0b10000100000000000001110100000110,
                0b10000000000000000001100100000110,
                0b11000000000011000000001100000000,
                0b10000000000000000000100100000110,
                0b00000100000000010100001100000101,
                0b10000100000000000010000100001000,
                0b10000100000000000010010100000000,
                0b10000000000000000010000100000000,
                0b11000000000011000100000000001000,
                0b10000000000000000000010100000000,
                0b00000100000000010000000000000100,
                0b10000100000000000010100100000101,
                0b10000100000000000010110100001000,
                0b10000000000000000010100100001000,
                0b11000000000010111100010000000101,
                0b10000000000000000000000100001000,
                0b00000100000000001100010000000011,
                0b10000100000000000011000100000100,
                0b10000100000000000011010100000101,
                0b10000000000000000011000100000101,
                0b11000000000011000000001010000100,
                0b10000100000000000011100100000011,
                0b10000000000000000011100100000101,
                0b11000000000011000100001010000011,
                0b00000100000000011100000010000001,
                0b10000100000000000011110100000011,
                0b10000000000000000001110100000011,
                0b11000000000000000100000110000101,
                0b10000100000000000100000100000101,
                0b10000000000000000001010100000101,
                0b00000100000000011100001010000111,
                0b11000000000000000100000010000001,
                0b10000100000000000100010100000001,
                0b10000000000000000011010100000001,
                0b00000100000000000100001000000100,
                0b11100000000010110000001110000001,
                0b10000100000000000100100100000001,
                0b10000000000000000010010100000001,
                0b00000100000000011100000010000111,
                0b10000100000000000100110100000100,
                0b10000100000000000101000100000111,
                0b10000000000000000011110100000111,
                0b10000000000000000100110100001001,
                0b00000100000000100100001110000100,
                0b10000100000000000101010100000100,
                0b10000000000000000100000100000100,
                0b10000100000000000000001100000100,
                0b10000000000000000100010100001001,
                0b10000100000000000000011100001001,
                0b10000000000000000010110100001000,
                0b10000000000000000101000100000111,
                0b00000100000000011100010000001001,
                0b11000000000010110100001110000111,
                0b10000100000000000101100100000111,
                0b10000100000000000101110100001001,
                0b10000000000000000001100100001001,
                0b10000000000000000100100100001000,
                0b00000100000000100000010010000111,
                0b10000000000000000101110100001000,
                0b11000000000010111000010000001001,
                0b11100000000000000100001110000111,
                0b10000100000000000110000100001001,
                0b10000000000000000010000100001000,
                0b10000100000000000110010100000111,
                0b10000000000000000101100100000111,
                0b00000100000000011100010000001001,
                0b10000100000000000110100100001001,
                0b10000000000000000110010100001001,
                0b10000100000000000000101100001001,
                0b10000000000000000011010100001000,
                0b10000000000000000110000100000111,
                0b00000100000000011100010000001001,
                0b10000100000000000110110100001001,
                0b10000000000000000110100100001000,
                0b11000000000000000100010000001001,
                0b10000100000000000111000100001001,
                0b10000000000000000100110100000111,
                0b10000000000000000110000100001000,
                0b00000100000000100000001110001001,
                0b10000100000000000111010100001001,
                0b10000000000000000110110100001000,
                0b11000000000010111000010000001001,
                0b10000100000000000111100100001001,
                0b10000000000000000101010100001000,
                0b10000000000000000110000100000111,
                0b00000100000000011100010000001001,
                0b10000100000000000111110100001001,
                0b10000000000000000111010100001000,
                0b11000000000010110000010000001001,
                0b10000100000000001000000100001001,
                0b10000000000000000111110100001000,
                0b11100000000010110100010000001001,
                0b10000100000000001000010100001001,
                0b10000000000000000111000100001001,
                0b10000100000000000000111100001001,
                0b10000000000000000010100100001000,
                0b10000000000000000111100100000111,
                0b00000100000000011100010000001001,
                0b10000100000000001000100100001001,
                0b10000000000000000011000100001000,
                0b10000000000000001000000100000111,
                0b00000100000000011100010000001001,
                0b10000100000000001000110100001001,
                0b10000000000000001000100100001000,
                0b11000000000000000100010000001001,
                0b10000100000000001001000100001001,
                0b10000000000000000011100100001000,
                0b10000000000000001000010100000111,
                0b00000100000000011100010000001001,
                0b10000100000000001001010100001001,
                0b10000000000000001000110100001000,
                0b11000000000000000100010000001001,
                0b10000100000000001001100100001001,
                0b10000000000000001001010100001000,
                0b11100000000000000100010000001001,
                0b10000100000000001001110100001001,
                0b10000000000000001001000100001001,
                0b10000100000000000001001100001001,
                0b10000000000000001001100100001001,
                0b10000100000000000001011100001001,
                0b10000000000000001001110100001001,
                0b10000100000000000001101100001001,
            ]
        );
    }

    #[test]
    fn test_allocate_cmp_ir() {
        let hex = pipeline(&get_cmp_ir(16, 2, 2));
        // for a in hex.iter() {
        //     println!("{:#034b},", a);
        // }
        assert_eq!(
            hex,
            vec![
                0b10000000000000000000001000000000,
                0b10000000000000000000011000000001,
                0b00010100100000000000000010000000,
                0b10000000000000000000101000000001,
                0b10000000000000000000111000000010,
                0b10000000000000000001001000000011,
                0b10000000000000000001011000000100,
                0b00010100100000000100000100000001,
                0b10000000000000000001101000000010,
                0b10000000000000000001111000000101,
                0b10000000000001000000001000000110,
                0b10000000000001000000011000000111,
                0b00010100100000001100001000000011,
                0b10000000000001000000101000000100,
                0b10000000000001000000111000001000,
                0b10000000000001000001001000001001,
                0b10000100000000000000000100000011,
                0b10000000000001000001011000000011,
                0b00010100100000001000001010000010,
                0b10000000000001000001101000000101,
                0b10000100000000000000010100000010,
                0b10000000000001000001111000000010,
                0b00010100100000011000001110000110,
                0b00010100100000010000010000000100,
                0b00010100100000100100000110000011,
                0b00010100100000010100000100000010,
                0b00001000000000011000000000000000,
                0b00001000000000010000000010000001,
                0b11000000000000101000000000000000,
                0b10000000000000000000000100000100,
                0b00001000000000001100001000000011,
                0b11000000000000101000000010000001,
                0b10000000000000000000010100000101,
                0b00001000000000001000001010000010,
                0b11000000000000101000000110000011,
                0b11100000000000101000000100000010,
                0b00010100100000000000000010000000,
                0b00010100100000001100000100000001,
                0b11000000000000101100000000000000,
                0b11100000000000101100000010000001,
                0b00010100100000000000000010000000,
                0b11100000000001101100000000000000,
                0b10000100000000000000001100000000,
            ]
        );
    }
}
