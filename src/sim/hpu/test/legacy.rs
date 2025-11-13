use crate::sim::hpu::{Argument, DOpId, RawDOp, MASK_NONE, MASK_PBS2, MASK_PBS4, MASK_PBS8};

mod legacy {
    pub struct RegId(pub u8);

    pub struct MulFactor(pub u8);

    pub struct CtId(pub u16);

    pub struct PbsGid(pub u16);

    pub struct SyncId(pub u32);

    pub enum ImmId {
        Cst(u16),
        Var { tid: u8, bid: u8 },
    }

    impl ImmId {
        pub fn unwrap_cst(&self) -> usize {
            match self {
                ImmId::Cst(c) => *c as usize,
                _ => panic!(),
            }
        }
    }

    pub enum MemId {
        Addr(CtId),
        Heap { bid: u16 },
        Src { tid: u8, bid: u8 },
        Dst { tid: u8, bid: u8 },
    }

    impl MemId {
        pub fn unwrap_addr(&self) -> usize {
            match self {
                MemId::Addr(ct_id) => ct_id.0 as usize,
                _ => panic!()
            }
        }
    }

    pub enum DOpType {
        ARITH = 0b00,
        SYNCT = 0b01,
        MEM = 0b10,
        PBST = 0b11,
    }

    pub struct Opcode {
        pub optype: DOpType,
        pub subtype: u8,
    }

    pub struct PeArithInsn {
        pub dst_rid: RegId,
        pub src0_rid: RegId,
        pub src1_rid: RegId,
        pub mul_factor: MulFactor,
        pub opcode: Opcode,
    }

    pub struct PeArithMsgInsn {
        pub dst_rid: RegId,
        pub src_rid: RegId,
        pub msg_cst: ImmId,
        pub opcode: Opcode,
    }

    pub struct PeMemInsn {
        pub rid: RegId,
        pub slot: MemId,
        pub opcode: Opcode,
    }

    pub struct PePbsInsn {
        pub dst_rid: RegId,
        pub src_rid: RegId,
        pub gid: PbsGid,
        pub opcode: Opcode,
    }

    pub struct PeSyncInsn {
        pub sid: SyncId,
        pub opcode: Opcode,
    }

    pub struct DOpAdd(pub PeArithInsn);
    pub struct DOpSub(pub PeArithInsn);
    pub struct DOpMac(pub PeArithInsn);
    pub struct DOpAdds(pub PeArithMsgInsn);
    pub struct DOpSubs(pub PeArithMsgInsn);
    pub struct DOpSsub(pub PeArithMsgInsn);
    pub struct DOpMuls(pub PeArithMsgInsn);
    pub struct DOpLd(pub PeMemInsn);
    pub struct DOpSt(pub PeMemInsn);
    pub struct DOpPbs(pub PePbsInsn);
    pub struct DOpPbsMl2(pub PePbsInsn);
    pub struct DOpPbsMl4(pub PePbsInsn);
    pub struct DOpPbsMl8(pub PePbsInsn);
    pub struct DOpPbsF(pub PePbsInsn);
    pub struct DOpPbsMl2F(pub PePbsInsn);
    pub struct DOpPbsMl4F(pub PePbsInsn);
    pub struct DOpPbsMl8F(pub PePbsInsn);
    pub struct DOpSync(pub PeSyncInsn);

    #[allow(non_camel_case_types)]
    pub enum DOp {
        ADD(DOpAdd),
        SUB(DOpSub),
        MAC(DOpMac),
        ADDS(DOpAdds),
        SUBS(DOpSubs),
        SSUB(DOpSsub),
        MULS(DOpMuls),
        LD(DOpLd),
        ST(DOpSt),
        PBS(DOpPbs),
        PBS_ML2(DOpPbsMl2),
        PBS_ML4(DOpPbsMl4),
        PBS_ML8(DOpPbsMl8),
        PBS_F(DOpPbsF),
        PBS_ML2_F(DOpPbsMl2F),
        PBS_ML4_F(DOpPbsMl4F),
        PBS_ML8_F(DOpPbsMl8F),
        SYNC(DOpSync),
    }
}

impl From<legacy::DOp> for RawDOp {
    fn from(value: legacy::DOp) -> Self {
        match value {
            legacy::DOp::ADD(dop) => RawDOp::ADD {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src1: Argument::Register { mask: MASK_NONE, addr: dop.0.src0_rid.0 as usize },
                src2: Argument::Register { mask: MASK_NONE, addr: dop.0.src1_rid.0 as usize },
            },
            legacy::DOp::SUB(dop) => RawDOp::SUB {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src1: Argument::Register { mask: MASK_NONE, addr: dop.0.src0_rid.0 as usize },
                src2: Argument::Register { mask: MASK_NONE, addr: dop.0.src1_rid.0 as usize },
            },
            legacy::DOp::MAC(dop) => RawDOp::MAC {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src1: Argument::Register { mask: MASK_NONE, addr: dop.0.src0_rid.0 as usize },
                src2: Argument::Register { mask: MASK_NONE, addr: dop.0.src1_rid.0 as usize },
                cst: Argument::Immediate { val: dop.0.mul_factor.0 as usize },
            },
            legacy::DOp::ADDS(dop) => RawDOp::ADDS {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
                cst: Argument::Immediate { val: dop.0.msg_cst.unwrap_cst() },
            },
            legacy::DOp::SUBS(dop) => RawDOp::SUBS {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
                cst: Argument::Immediate { val: dop.0.msg_cst.unwrap_cst() },
            },
            legacy::DOp::SSUB(dop) => RawDOp::SSUB {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
                cst: Argument::Immediate { val: dop.0.msg_cst.unwrap_cst() },
            },
            legacy::DOp::MULS(dop) => RawDOp::MULS {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
                cst: Argument::Immediate { val: dop.0.msg_cst.unwrap_cst() },
            },
            legacy::DOp::LD(dop) => RawDOp::LD {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.rid.0 as usize },
                src: Argument::Memory { addr: dop.0.slot.unwrap_addr() }
            },
            legacy::DOp::ST(dop) => RawDOp::ST {
                dst: Argument::Memory { addr: dop.0.slot.unwrap_addr() },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.rid.0 as usize },
            },
            legacy::DOp::PBS(dop) => RawDOp::PBS {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::PBS_ML2(dop) => RawDOp::PBS_ML2 {
                dst: Argument::Register { mask: MASK_PBS2, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::PBS_ML4(dop) => RawDOp::PBS_ML4 {
                dst: Argument::Register { mask: MASK_PBS4, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::PBS_ML8(dop) => RawDOp::PBS_ML8 {
                dst: Argument::Register { mask: MASK_PBS8, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::PBS_F(dop) => RawDOp::PBS_F {
                dst: Argument::Register { mask: MASK_NONE, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::PBS_ML2_F(dop) => RawDOp::PBS_ML2_F {
                dst: Argument::Register { mask: MASK_PBS2, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::PBS_ML4_F(dop) => RawDOp::PBS_ML4_F {
                dst: Argument::Register { mask: MASK_PBS4, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::PBS_ML8_F(dop) => RawDOp::PBS_ML8_F {
                dst: Argument::Register { mask: MASK_PBS8, addr: dop.0.dst_rid.0 as usize },
                src: Argument::Register { mask: MASK_NONE, addr: dop.0.src_rid.0 as usize },
            },
            legacy::DOp::SYNC(_) => RawDOp::SYNC,
        }
    }
}

pub fn adds() -> impl Iterator<Item = crate::sim::hpu::DOp> {
    use legacy::*;
    use legacy::DOp::*;
    use legacy::DOpType::*;
    use legacy::MemId::*;
    use legacy::ImmId::*;
    let a: Vec<legacy::DOp> = include!("streams/adds.rs");
    a.into_iter().enumerate().map(|(id, op)| crate::sim::hpu::DOp { raw: op.into(), id: DOpId(id as u16) })
}
