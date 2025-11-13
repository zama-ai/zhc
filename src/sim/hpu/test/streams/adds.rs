vec![
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    1,
                ),
                src0_rid: RegId(
                    1,
                ),
                src1_rid: RegId(
                    1,
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
                    1,
                ),
                src_rid: RegId(
                    1,
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
                    3,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    9,
                ),
                src0_rid: RegId(
                    9,
                ),
                src1_rid: RegId(
                    9,
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
                    9,
                ),
                src_rid: RegId(
                    9,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    14,
                ),
                src0_rid: RegId(
                    14,
                ),
                src1_rid: RegId(
                    14,
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
                    14,
                ),
                src_rid: RegId(
                    14,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    18,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    19,
                ),
                src0_rid: RegId(
                    19,
                ),
                src1_rid: RegId(
                    19,
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
                    19,
                ),
                src_rid: RegId(
                    19,
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
                    22,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    20,
                ),
                src0_rid: RegId(
                    18,
                ),
                src1_rid: RegId(
                    19,
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
                    21,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    24,
                ),
                src0_rid: RegId(
                    24,
                ),
                src1_rid: RegId(
                    24,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    26,
                ),
                src_rid: RegId(
                    20,
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
                    25,
                ),
                src0_rid: RegId(
                    21,
                ),
                src1_rid: RegId(
                    24,
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
                    28,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    29,
                ),
                src0_rid: RegId(
                    29,
                ),
                src1_rid: RegId(
                    29,
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
                    29,
                ),
                src_rid: RegId(
                    29,
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
                    32,
                ),
                src_rid: RegId(
                    25,
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
                    30,
                ),
                src0_rid: RegId(
                    28,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    31,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    34,
                ),
                src0_rid: RegId(
                    34,
                ),
                src1_rid: RegId(
                    34,
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
                    34,
                ),
                src_rid: RegId(
                    34,
                ),
                msg_cst: Cst(
                    3,
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
                    36,
                ),
                src_rid: RegId(
                    30,
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
                    35,
                ),
                src0_rid: RegId(
                    31,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    38,
                ),
                slot: Addr(
                    CtId(
                        8,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    39,
                ),
                src0_rid: RegId(
                    39,
                ),
                src1_rid: RegId(
                    39,
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
                    39,
                ),
                src_rid: RegId(
                    39,
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
                    42,
                ),
                src_rid: RegId(
                    35,
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
                    40,
                ),
                src0_rid: RegId(
                    38,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    41,
                ),
                slot: Addr(
                    CtId(
                        9,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    44,
                ),
                src0_rid: RegId(
                    44,
                ),
                src1_rid: RegId(
                    44,
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
                    44,
                ),
                src_rid: RegId(
                    44,
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
                    46,
                ),
                src_rid: RegId(
                    40,
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
                    45,
                ),
                src0_rid: RegId(
                    41,
                ),
                src1_rid: RegId(
                    44,
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
                    48,
                ),
                slot: Addr(
                    CtId(
                        10,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
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
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    49,
                ),
                src_rid: RegId(
                    49,
                ),
                msg_cst: Cst(
                    3,
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
                    52,
                ),
                src_rid: RegId(
                    45,
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
                    50,
                ),
                src0_rid: RegId(
                    48,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    51,
                ),
                slot: Addr(
                    CtId(
                        11,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    54,
                ),
                src0_rid: RegId(
                    54,
                ),
                src1_rid: RegId(
                    54,
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
                    54,
                ),
                src_rid: RegId(
                    54,
                ),
                msg_cst: Cst(
                    3,
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
                    56,
                ),
                src_rid: RegId(
                    50,
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
                    55,
                ),
                src0_rid: RegId(
                    51,
                ),
                src1_rid: RegId(
                    54,
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
                    58,
                ),
                slot: Addr(
                    CtId(
                        12,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    59,
                ),
                src0_rid: RegId(
                    59,
                ),
                src1_rid: RegId(
                    59,
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
                    59,
                ),
                src_rid: RegId(
                    59,
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
                    62,
                ),
                src_rid: RegId(
                    55,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    60,
                ),
                src0_rid: RegId(
                    58,
                ),
                src1_rid: RegId(
                    59,
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
                    61,
                ),
                slot: Addr(
                    CtId(
                        13,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
                },
            },
        ),
    ),
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
                    2,
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
                    1,
                ),
                src0_rid: RegId(
                    61,
                ),
                src1_rid: RegId(
                    0,
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
                        14,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
                    subtype: 0,
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
                    1,
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
                    8,
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
                    9,
                ),
                slot: Addr(
                    CtId(
                        15,
                    ),
                ),
                opcode: Opcode {
                    optype: MEM,
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
    ADDS(
        DOpAdds(
            PeArithMsgInsn {
                dst_rid: RegId(
                    11,
                ),
                src_rid: RegId(
                    11,
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
                    14,
                ),
                src0_rid: RegId(
                    9,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    18,
                ),
                src_rid: RegId(
                    60,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    28,
                ),
                src_rid: RegId(
                    1,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    38,
                ),
                src_rid: RegId(
                    8,
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
    PBS_ML2(
        DOpPbsMl2(
            PePbsInsn {
                dst_rid: RegId(
                    44,
                ),
                src_rid: RegId(
                    14,
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
    MULS(
        DOpMuls(
            PeArithMsgInsn {
                dst_rid: RegId(
                    21,
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
                    21,
                ),
                src_rid: RegId(
                    21,
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
                    31,
                ),
                src_rid: RegId(
                    21,
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
                    24,
                ),
                src0_rid: RegId(
                    12,
                ),
                src1_rid: RegId(
                    21,
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
                    41,
                ),
                src_rid: RegId(
                    24,
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
                    34,
                ),
                src0_rid: RegId(
                    16,
                ),
                src1_rid: RegId(
                    24,
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
                    49,
                ),
                src_rid: RegId(
                    34,
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
                    48,
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
                    51,
                ),
                src_rid: RegId(
                    48,
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
                    51,
                ),
                slot: Addr(
                    CtId(
                        16,
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
                    54,
                ),
                src_rid: RegId(
                    49,
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
                    58,
                ),
                src0_rid: RegId(
                    22,
                ),
                src1_rid: RegId(
                    54,
                ),
                mul_factor: MulFactor(
                    2,
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
                    61,
                ),
                src_rid: RegId(
                    58,
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
                    59,
                ),
                src0_rid: RegId(
                    26,
                ),
                src1_rid: RegId(
                    58,
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
                    3,
                ),
                src_rid: RegId(
                    59,
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
                    0,
                ),
                src0_rid: RegId(
                    32,
                ),
                src1_rid: RegId(
                    59,
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
                    9,
                ),
                src_rid: RegId(
                    0,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    4,
                ),
                src0_rid: RegId(
                    54,
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
                    55,
                ),
                src_rid: RegId(
                    4,
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
                    11,
                ),
                src0_rid: RegId(
                    41,
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
                    40,
                ),
                src_rid: RegId(
                    11,
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
                    50,
                ),
                src0_rid: RegId(
                    31,
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
                    35,
                ),
                src_rid: RegId(
                    50,
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
                    35,
                ),
                slot: Addr(
                    CtId(
                        17,
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
                        19,
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
                    40,
                ),
                slot: Addr(
                    CtId(
                        18,
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
                    30,
                ),
                src_rid: RegId(
                    9,
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
                    25,
                ),
                src0_rid: RegId(
                    36,
                ),
                src1_rid: RegId(
                    30,
                ),
                mul_factor: MulFactor(
                    2,
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
                    15,
                ),
                src_rid: RegId(
                    25,
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
                    20,
                ),
                src0_rid: RegId(
                    42,
                ),
                src1_rid: RegId(
                    25,
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
                    5,
                ),
                src_rid: RegId(
                    20,
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
                    10,
                ),
                src0_rid: RegId(
                    46,
                ),
                src1_rid: RegId(
                    20,
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
                    6,
                ),
                src_rid: RegId(
                    10,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    2,
                ),
                src0_rid: RegId(
                    30,
                ),
                src1_rid: RegId(
                    37,
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
                    16,
                ),
                src_rid: RegId(
                    2,
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
                    12,
                ),
                src0_rid: RegId(
                    3,
                ),
                src1_rid: RegId(
                    33,
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
                    48,
                ),
                src_rid: RegId(
                    12,
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
                    7,
                ),
                src0_rid: RegId(
                    61,
                ),
                src1_rid: RegId(
                    27,
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
                    7,
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
                        20,
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
                    16,
                ),
                slot: Addr(
                    CtId(
                        22,
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
                    48,
                ),
                slot: Addr(
                    CtId(
                        21,
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
                    24,
                ),
                src_rid: RegId(
                    6,
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
                    21,
                ),
                src0_rid: RegId(
                    52,
                ),
                src1_rid: RegId(
                    24,
                ),
                mul_factor: MulFactor(
                    2,
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
                    14,
                ),
                src_rid: RegId(
                    21,
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
                    44,
                ),
                src0_rid: RegId(
                    56,
                ),
                src1_rid: RegId(
                    21,
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
                    1,
                ),
                src_rid: RegId(
                    44,
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
                    8,
                ),
                src0_rid: RegId(
                    62,
                ),
                src1_rid: RegId(
                    44,
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
                    51,
                ),
                src_rid: RegId(
                    8,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    60,
                ),
                src0_rid: RegId(
                    24,
                ),
                src1_rid: RegId(
                    53,
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
                    22,
                ),
                src_rid: RegId(
                    60,
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
                    49,
                ),
                src0_rid: RegId(
                    5,
                ),
                src1_rid: RegId(
                    47,
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
                    32,
                ),
                src_rid: RegId(
                    49,
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
                    26,
                ),
                src0_rid: RegId(
                    15,
                ),
                src1_rid: RegId(
                    43,
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
                    54,
                ),
                src_rid: RegId(
                    26,
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
                    54,
                ),
                slot: Addr(
                    CtId(
                        23,
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
                    22,
                ),
                slot: Addr(
                    CtId(
                        25,
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
                    32,
                ),
                slot: Addr(
                    CtId(
                        24,
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
                    23,
                ),
                src_rid: RegId(
                    51,
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
                    41,
                ),
                src0_rid: RegId(
                    18,
                ),
                src1_rid: RegId(
                    23,
                ),
                mul_factor: MulFactor(
                    2,
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
                    41,
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
                    17,
                ),
                src0_rid: RegId(
                    28,
                ),
                src1_rid: RegId(
                    41,
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
                    50,
                ),
                src_rid: RegId(
                    17,
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
                    13,
                ),
                src0_rid: RegId(
                    38,
                ),
                src1_rid: RegId(
                    17,
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
                    4,
                ),
                src_rid: RegId(
                    13,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    11,
                ),
                src0_rid: RegId(
                    23,
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
                    59,
                ),
                src_rid: RegId(
                    11,
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
                    0,
                ),
                src0_rid: RegId(
                    1,
                ),
                src1_rid: RegId(
                    63,
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
                    0,
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
                    58,
                ),
                src0_rid: RegId(
                    14,
                ),
                src1_rid: RegId(
                    57,
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
                    55,
                ),
                src_rid: RegId(
                    58,
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
                    55,
                ),
                slot: Addr(
                    CtId(
                        26,
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
                    59,
                ),
                slot: Addr(
                    CtId(
                        28,
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
                        27,
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
                    40,
                ),
                src_rid: RegId(
                    4,
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
                    9,
                ),
                src0_rid: RegId(
                    40,
                ),
                src1_rid: RegId(
                    45,
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
                    42,
                ),
                src_rid: RegId(
                    9,
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
                    50,
                ),
                src1_rid: RegId(
                    39,
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
                    30,
                ),
                src_rid: RegId(
                    36,
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
                    46,
                ),
                src0_rid: RegId(
                    31,
                ),
                src1_rid: RegId(
                    29,
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
                    37,
                ),
                src_rid: RegId(
                    46,
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
                    37,
                ),
                slot: Addr(
                    CtId(
                        29,
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
                    42,
                ),
                slot: Addr(
                    CtId(
                        31,
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
                    30,
                ),
                slot: Addr(
                    CtId(
                        30,
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
