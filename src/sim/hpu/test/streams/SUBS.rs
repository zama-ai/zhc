[
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    0,
                ),
                src0_rid: RegId(
                    0,
                ),
                src1_rid: RegId(
                    0,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 2,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    0,
                ),
                src_rid: RegId(
                    0,
                ),
                msg_cst: Cst(
                    1,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 9,
                },
            },
        ),
    ),
    SSUB(
        DOpSsub(
            PeArithMsgInsn {
                dst_rid: RegId(
                    1,
                ),
                src_rid: RegId(
                    0,
                ),
                msg_cst: Cst(
                    3,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 11,
                },
            },
        ),
    ),
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    2,
                ),
                slot: Addr(
                    CtId(
                        0,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    3,
                ),
                src0_rid: RegId(
                    2,
                ),
                src1_rid: RegId(
                    1,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 1,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    4,
                ),
                src0_rid: RegId(
                    4,
                ),
                src1_rid: RegId(
                    4,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 2,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    4,
                ),
                src_rid: RegId(
                    4,
                ),
                msg_cst: Cst(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 9,
                },
            },
        ),
    ),
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    6,
                ),
                src_rid: RegId(
                    3,
                ),
                gid: PbsGid(
                    18,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
                },
            },
        ),
    ),
    SSUB(
        DOpSsub(
            PeArithMsgInsn {
                dst_rid: RegId(
                    5,
                ),
                src_rid: RegId(
                    4,
                ),
                msg_cst: Cst(
                    3,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 11,
                },
            },
        ),
    ),
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    8,
                ),
                slot: Addr(
                    CtId(
                        1,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    9,
                ),
                src0_rid: RegId(
                    8,
                ),
                src1_rid: RegId(
                    5,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 1,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    10,
                ),
                src0_rid: RegId(
                    10,
                ),
                src1_rid: RegId(
                    10,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 2,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    10,
                ),
                src_rid: RegId(
                    10,
                ),
                msg_cst: Cst(
                    1,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 9,
                },
            },
        ),
    ),
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    12,
                ),
                src_rid: RegId(
                    9,
                ),
                gid: PbsGid(
                    18,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
                },
            },
        ),
    ),
    SSUB(
        DOpSsub(
            PeArithMsgInsn {
                dst_rid: RegId(
                    11,
                ),
                src_rid: RegId(
                    10,
                ),
                msg_cst: Cst(
                    3,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 11,
                },
            },
        ),
    ),
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    14,
                ),
                slot: Addr(
                    CtId(
                        2,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    15,
                ),
                src0_rid: RegId(
                    14,
                ),
                src1_rid: RegId(
                    11,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 1,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    16,
                ),
                src0_rid: RegId(
                    16,
                ),
                src1_rid: RegId(
                    16,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 2,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    16,
                ),
                src_rid: RegId(
                    16,
                ),
                msg_cst: Cst(
                    2,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 9,
                },
            },
        ),
    ),
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    18,
                ),
                src_rid: RegId(
                    15,
                ),
                gid: PbsGid(
                    18,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
                },
            },
        ),
    ),
    SSUB(
        DOpSsub(
            PeArithMsgInsn {
                dst_rid: RegId(
                    17,
                ),
                src_rid: RegId(
                    16,
                ),
                msg_cst: Cst(
                    3,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 11,
                },
            },
        ),
    ),
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    20,
                ),
                slot: Addr(
                    CtId(
                        3,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    21,
                ),
                src0_rid: RegId(
                    20,
                ),
                src1_rid: RegId(
                    17,
                ),
                mul_factor: MulFactor(
                    0,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 1,
                },
            },
        ),
    ),
    PBS_ML2_F(
        DOpPbsMl2F(
            PePbsInsn {
                dst_rid: RegId(
                    22,
                ),
                src_rid: RegId(
                    21,
                ),
                gid: PbsGid(
                    18,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 9,
                },
            },
        ),
    ),
    MULS(
        DOpMuls(
            PeArithMsgInsn {
                dst_rid: RegId(
                    24,
                ),
                src_rid: RegId(
                    6,
                ),
                msg_cst: Cst(
                    2,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 12,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    24,
                ),
                src_rid: RegId(
                    24,
                ),
                msg_cst: Cst(
                    2,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 9,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    26,
                ),
                src_rid: RegId(
                    24,
                ),
                gid: PbsGid(
                    19,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    25,
                ),
                src0_rid: RegId(
                    12,
                ),
                src1_rid: RegId(
                    24,
                ),
                mul_factor: MulFactor(
                    4,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 5,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    28,
                ),
                src_rid: RegId(
                    25,
                ),
                gid: PbsGid(
                    20,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    27,
                ),
                src0_rid: RegId(
                    18,
                ),
                src1_rid: RegId(
                    25,
                ),
                mul_factor: MulFactor(
                    8,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 5,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    30,
                ),
                src_rid: RegId(
                    27,
                ),
                gid: PbsGid(
                    21,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    29,
                ),
                src_rid: RegId(
                    7,
                ),
                msg_cst: Cst(
                    8,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 9,
                },
            },
        ),
    ),
    PBS_F(
        DOpPbsF(
            PePbsInsn {
                dst_rid: RegId(
                    31,
                ),
                src_rid: RegId(
                    29,
                ),
                gid: PbsGid(
                    22,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 8,
                },
            },
        ),
    ),
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    31,
                ),
                slot: Addr(
                    CtId(
                        4,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 1,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    32,
                ),
                src_rid: RegId(
                    30,
                ),
                msg_cst: Cst(
                    1,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 9,
                },
            },
        ),
    ),
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    33,
                ),
                src0_rid: RegId(
                    32,
                ),
                src1_rid: RegId(
                    23,
                ),
                mul_factor: MulFactor(
                    4,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 5,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    35,
                ),
                src_rid: RegId(
                    33,
                ),
                gid: PbsGid(
                    22,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    34,
                ),
                src0_rid: RegId(
                    28,
                ),
                src1_rid: RegId(
                    19,
                ),
                mul_factor: MulFactor(
                    4,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 5,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    37,
                ),
                src_rid: RegId(
                    34,
                ),
                gid: PbsGid(
                    22,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    36,
                ),
                src0_rid: RegId(
                    26,
                ),
                src1_rid: RegId(
                    13,
                ),
                mul_factor: MulFactor(
                    4,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 5,
                },
            },
        ),
    ),
    PBS_F(
        DOpPbsF(
            PePbsInsn {
                dst_rid: RegId(
                    38,
                ),
                src_rid: RegId(
                    36,
                ),
                gid: PbsGid(
                    22,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 8,
                },
            },
        ),
    ),
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    38,
                ),
                slot: Addr(
                    CtId(
                        5,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 1,
                },
            },
        ),
    ),
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    35,
                ),
                slot: Addr(
                    CtId(
                        7,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 1,
                },
            },
        ),
    ),
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    37,
                ),
                slot: Addr(
                    CtId(
                        6,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 1,
                },
            },
        ),
    ),
    SYNC(
        DOpSync(
            PeSyncInsn {
                sid: SyncId(
                    0,
                ),
                opcode: Opcode {
                    optype: SYNCT,
                    subtype: 0,
                },
            },
        ),
    ),
]
