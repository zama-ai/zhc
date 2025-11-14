[
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    1,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    0,
                ),
                src_rid: RegId(
                    1,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    3,
                ),
                src0_rid: RegId(
                    1,
                ),
                src1_rid: RegId(
                    2,
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
                    4,
                ),
                src_rid: RegId(
                    3,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    5,
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
                    6,
                ),
                src0_rid: RegId(
                    3,
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
                    7,
                ),
                src_rid: RegId(
                    6,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    8,
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
                    9,
                ),
                src0_rid: RegId(
                    6,
                ),
                src1_rid: RegId(
                    8,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    11,
                ),
                src0_rid: RegId(
                    11,
                ),
                src1_rid: RegId(
                    11,
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
                    12,
                ),
                src0_rid: RegId(
                    10,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    13,
                ),
                src_rid: RegId(
                    12,
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
                    14,
                ),
                src0_rid: RegId(
                    7,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    15,
                ),
                src_rid: RegId(
                    14,
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
                    16,
                ),
                src0_rid: RegId(
                    4,
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
                    0,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    20,
                ),
                src_rid: RegId(
                    1,
                ),
                gid: PbsGid(
                    74,
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
                    22,
                ),
                src0_rid: RegId(
                    19,
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
                    24,
                ),
                src_rid: RegId(
                    22,
                ),
                gid: PbsGid(
                    74,
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
                    23,
                ),
                src0_rid: RegId(
                    17,
                ),
                src1_rid: RegId(
                    5,
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
                    23,
                ),
                gid: PbsGid(
                    74,
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
                    28,
                ),
                src0_rid: RegId(
                    15,
                ),
                src1_rid: RegId(
                    8,
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
    PBS_ML2_F(
        DOpPbsMl2F(
            PePbsInsn {
                dst_rid: RegId(
                    30,
                ),
                src_rid: RegId(
                    28,
                ),
                gid: PbsGid(
                    74,
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
                    29,
                ),
                src_rid: RegId(
                    20,
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
                    32,
                ),
                src0_rid: RegId(
                    21,
                ),
                src1_rid: RegId(
                    29,
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
                    33,
                ),
                src0_rid: RegId(
                    24,
                ),
                src1_rid: RegId(
                    32,
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
                    34,
                ),
                src0_rid: RegId(
                    25,
                ),
                src1_rid: RegId(
                    33,
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
                    35,
                ),
                src0_rid: RegId(
                    26,
                ),
                src1_rid: RegId(
                    34,
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
                    36,
                ),
                src0_rid: RegId(
                    27,
                ),
                src1_rid: RegId(
                    35,
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
                    30,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    38,
                ),
                src_rid: RegId(
                    37,
                ),
                gid: PbsGid(
                    70,
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
                    40,
                ),
                src_rid: RegId(
                    31,
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
                    42,
                ),
                src_rid: RegId(
                    40,
                ),
                gid: PbsGid(
                    64,
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
                    41,
                ),
                src_rid: RegId(
                    38,
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
                    44,
                ),
                src0_rid: RegId(
                    42,
                ),
                src1_rid: RegId(
                    41,
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
                    subtype: 1,
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
                    47,
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
                    39,
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
    PBS_ML2_F(
        DOpPbsMl2F(
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
                    subtype: 9,
                },
            },
        ),
    ),
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    46,
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
                    50,
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
                    51,
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
                    49,
                ),
                src0_rid: RegId(
                    49,
                ),
                src1_rid: RegId(
                    49,
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
                    49,
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
