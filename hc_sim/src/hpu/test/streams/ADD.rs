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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    1,
                ),
                slot: Addr(
                    CtId(
                        4,
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
                    2,
                ),
                src0_rid: RegId(
                    0,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    3,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    4,
                ),
                slot: Addr(
                    CtId(
                        5,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
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
                    2,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    5,
                ),
                src0_rid: RegId(
                    3,
                ),
                src1_rid: RegId(
                    4,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    8,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    9,
                ),
                slot: Addr(
                    CtId(
                        6,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
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
                    5,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    10,
                ),
                src0_rid: RegId(
                    8,
                ),
                src1_rid: RegId(
                    9,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    11,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    14,
                ),
                slot: Addr(
                    CtId(
                        7,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    16,
                ),
                src_rid: RegId(
                    10,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    15,
                ),
                src0_rid: RegId(
                    11,
                ),
                src1_rid: RegId(
                    14,
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
                    subtype: 9,
                },
            },
        ),
    ),
    MULS(
        DOpMuls(
            PeArithMsgInsn {
                dst_rid: RegId(
                    20,
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
                    20,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    22,
                ),
                src_rid: RegId(
                    20,
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
                    21,
                ),
                src0_rid: RegId(
                    12,
                ),
                src1_rid: RegId(
                    20,
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
                    24,
                ),
                src_rid: RegId(
                    21,
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
                    23,
                ),
                src0_rid: RegId(
                    16,
                ),
                src1_rid: RegId(
                    21,
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
                    26,
                ),
                src_rid: RegId(
                    23,
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
                    25,
                ),
                src_rid: RegId(
                    7,
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
    PBS_F(
        DOpPbsF(
            PePbsInsn {
                dst_rid: RegId(
                    27,
                ),
                src_rid: RegId(
                    25,
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
                    27,
                ),
                slot: Addr(
                    CtId(
                        8,
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
                    28,
                ),
                src_rid: RegId(
                    26,
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
                    29,
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
                    subtype: 0,
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
                    24,
                ),
                src1_rid: RegId(
                    17,
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
                    33,
                ),
                src_rid: RegId(
                    30,
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
                    32,
                ),
                src0_rid: RegId(
                    22,
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
                    34,
                ),
                src_rid: RegId(
                    32,
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
                    34,
                ),
                slot: Addr(
                    CtId(
                        9,
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
                    31,
                ),
                slot: Addr(
                    CtId(
                        11,
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
                    33,
                ),
                slot: Addr(
                    CtId(
                        10,
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
