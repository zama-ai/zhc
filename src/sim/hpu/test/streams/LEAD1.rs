[
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    0,
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
    SSUB(
        DOpSsub(
            PeArithMsgInsn {
                dst_rid: RegId(
                    3,
                ),
                src_rid: RegId(
                    2,
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
                    4,
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
                    6,
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
    SSUB(
        DOpSsub(
            PeArithMsgInsn {
                dst_rid: RegId(
                    7,
                ),
                src_rid: RegId(
                    6,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    8,
                ),
                src_rid: RegId(
                    7,
                ),
                gid: PbsGid(
                    40,
                ),
                opcode: Opcode {
                    optype: PBST,
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
                    7,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    10,
                ),
                src_rid: RegId(
                    9,
                ),
                gid: PbsGid(
                    40,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    11,
                ),
                src0_rid: RegId(
                    9,
                ),
                src1_rid: RegId(
                    3,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    12,
                ),
                src_rid: RegId(
                    11,
                ),
                gid: PbsGid(
                    40,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    13,
                ),
                src0_rid: RegId(
                    11,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    14,
                ),
                src_rid: RegId(
                    13,
                ),
                gid: PbsGid(
                    40,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    15,
                ),
                src0_rid: RegId(
                    15,
                ),
                src1_rid: RegId(
                    15,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    16,
                ),
                src0_rid: RegId(
                    14,
                ),
                src1_rid: RegId(
                    15,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    17,
                ),
                src_rid: RegId(
                    16,
                ),
                gid: PbsGid(
                    40,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    18,
                ),
                src0_rid: RegId(
                    12,
                ),
                src1_rid: RegId(
                    15,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    19,
                ),
                src_rid: RegId(
                    18,
                ),
                gid: PbsGid(
                    40,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    20,
                ),
                src0_rid: RegId(
                    10,
                ),
                src1_rid: RegId(
                    15,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    21,
                ),
                src_rid: RegId(
                    20,
                ),
                gid: PbsGid(
                    40,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    22,
                ),
                src0_rid: RegId(
                    8,
                ),
                src1_rid: RegId(
                    15,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    23,
                ),
                src_rid: RegId(
                    22,
                ),
                gid: PbsGid(
                    40,
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
                    24,
                ),
                src0_rid: RegId(
                    19,
                ),
                src1_rid: RegId(
                    0,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    26,
                ),
                src_rid: RegId(
                    24,
                ),
                gid: PbsGid(
                    73,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
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
                    21,
                ),
                src1_rid: RegId(
                    2,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    28,
                ),
                src_rid: RegId(
                    25,
                ),
                gid: PbsGid(
                    73,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
                },
            },
        ),
    ),
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    30,
                ),
                src0_rid: RegId(
                    23,
                ),
                src1_rid: RegId(
                    4,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    32,
                ),
                src_rid: RegId(
                    30,
                ),
                gid: PbsGid(
                    73,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
                },
            },
        ),
    ),
    PBS_ML2_F(
        DOpPbsMl2F(
            PePbsInsn {
                dst_rid: RegId(
                    34,
                ),
                src_rid: RegId(
                    6,
                ),
                gid: PbsGid(
                    73,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 9,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    31,
                ),
                src_rid: RegId(
                    26,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    36,
                ),
                src0_rid: RegId(
                    27,
                ),
                src1_rid: RegId(
                    31,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    37,
                ),
                src0_rid: RegId(
                    28,
                ),
                src1_rid: RegId(
                    36,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    38,
                ),
                src0_rid: RegId(
                    29,
                ),
                src1_rid: RegId(
                    37,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    39,
                ),
                src0_rid: RegId(
                    32,
                ),
                src1_rid: RegId(
                    38,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    40,
                ),
                src0_rid: RegId(
                    33,
                ),
                src1_rid: RegId(
                    39,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    41,
                ),
                src0_rid: RegId(
                    34,
                ),
                src1_rid: RegId(
                    40,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    42,
                ),
                src_rid: RegId(
                    41,
                ),
                gid: PbsGid(
                    26,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    44,
                ),
                src_rid: RegId(
                    35,
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
    PBS_ML2_F(
        DOpPbsMl2F(
            PePbsInsn {
                dst_rid: RegId(
                    46,
                ),
                src_rid: RegId(
                    44,
                ),
                gid: PbsGid(
                    26,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 9,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    45,
                ),
                src_rid: RegId(
                    42,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    48,
                ),
                src0_rid: RegId(
                    46,
                ),
                src1_rid: RegId(
                    45,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    50,
                ),
                src_rid: RegId(
                    48,
                ),
                gid: PbsGid(
                    26,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 1,
                },
            },
        ),
    ),
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    49,
                ),
                src_rid: RegId(
                    51,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    52,
                ),
                src0_rid: RegId(
                    43,
                ),
                src1_rid: RegId(
                    49,
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
                    54,
                ),
                src_rid: RegId(
                    52,
                ),
                gid: PbsGid(
                    26,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 9,
                },
            },
        ),
    ),
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    50,
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
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    54,
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
                    55,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    53,
                ),
                src0_rid: RegId(
                    53,
                ),
                src1_rid: RegId(
                    53,
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
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    53,
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
