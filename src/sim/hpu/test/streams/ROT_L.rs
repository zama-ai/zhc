[
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    0,
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
    MAC(
        DOpMac(
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
                    2,
                ),
                gid: PbsGid(
                    57,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
                    2,
                ),
                gid: PbsGid(
                    58,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    6,
                ),
                src0_rid: RegId(
                    0,
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
                    57,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
                    6,
                ),
                gid: PbsGid(
                    58,
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
                    9,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    10,
                ),
                src0_rid: RegId(
                    0,
                ),
                src1_rid: RegId(
                    9,
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
                    11,
                ),
                src_rid: RegId(
                    10,
                ),
                gid: PbsGid(
                    57,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
                    10,
                ),
                gid: PbsGid(
                    58,
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
                    13,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    14,
                ),
                src0_rid: RegId(
                    0,
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
                    57,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
                    14,
                ),
                gid: PbsGid(
                    58,
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
                    17,
                ),
                src0_rid: RegId(
                    3,
                ),
                src1_rid: RegId(
                    16,
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
                    18,
                ),
                src0_rid: RegId(
                    7,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    19,
                ),
                src0_rid: RegId(
                    11,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    20,
                ),
                src0_rid: RegId(
                    15,
                ),
                src1_rid: RegId(
                    12,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    21,
                ),
                src0_rid: RegId(
                    0,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    22,
                ),
                src0_rid: RegId(
                    0,
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
                    23,
                ),
                src_rid: RegId(
                    21,
                ),
                gid: PbsGid(
                    63,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
                    22,
                ),
                gid: PbsGid(
                    55,
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
                    25,
                ),
                src0_rid: RegId(
                    23,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    26,
                ),
                src0_rid: RegId(
                    0,
                ),
                src1_rid: RegId(
                    18,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    27,
                ),
                src0_rid: RegId(
                    0,
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
                    28,
                ),
                src_rid: RegId(
                    26,
                ),
                gid: PbsGid(
                    63,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    29,
                ),
                src_rid: RegId(
                    27,
                ),
                gid: PbsGid(
                    55,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    31,
                ),
                src0_rid: RegId(
                    0,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    32,
                ),
                src0_rid: RegId(
                    0,
                ),
                src1_rid: RegId(
                    18,
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
                    31,
                ),
                gid: PbsGid(
                    63,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    34,
                ),
                src_rid: RegId(
                    32,
                ),
                gid: PbsGid(
                    55,
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
                    35,
                ),
                src0_rid: RegId(
                    33,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    36,
                ),
                src0_rid: RegId(
                    0,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    37,
                ),
                src0_rid: RegId(
                    0,
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
                    38,
                ),
                src_rid: RegId(
                    36,
                ),
                gid: PbsGid(
                    63,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    39,
                ),
                src_rid: RegId(
                    37,
                ),
                gid: PbsGid(
                    55,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    42,
                ),
                src0_rid: RegId(
                    41,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    43,
                ),
                src0_rid: RegId(
                    41,
                ),
                src1_rid: RegId(
                    35,
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
                    44,
                ),
                src_rid: RegId(
                    42,
                ),
                gid: PbsGid(
                    61,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    45,
                ),
                src_rid: RegId(
                    43,
                ),
                gid: PbsGid(
                    62,
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
                    46,
                ),
                src0_rid: RegId(
                    44,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    47,
                ),
                src0_rid: RegId(
                    41,
                ),
                src1_rid: RegId(
                    30,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    48,
                ),
                src0_rid: RegId(
                    41,
                ),
                src1_rid: RegId(
                    40,
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
                    49,
                ),
                src_rid: RegId(
                    47,
                ),
                gid: PbsGid(
                    61,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
                    48,
                ),
                gid: PbsGid(
                    62,
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
                    51,
                ),
                src0_rid: RegId(
                    49,
                ),
                src1_rid: RegId(
                    50,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    52,
                ),
                src0_rid: RegId(
                    41,
                ),
                src1_rid: RegId(
                    35,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    53,
                ),
                src0_rid: RegId(
                    41,
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
                    54,
                ),
                src_rid: RegId(
                    52,
                ),
                gid: PbsGid(
                    61,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
                    53,
                ),
                gid: PbsGid(
                    62,
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
                    56,
                ),
                src0_rid: RegId(
                    54,
                ),
                src1_rid: RegId(
                    55,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    57,
                ),
                src0_rid: RegId(
                    41,
                ),
                src1_rid: RegId(
                    40,
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
    MAC(
        DOpMac(
            PeArithInsn {
                dst_rid: RegId(
                    58,
                ),
                src0_rid: RegId(
                    41,
                ),
                src1_rid: RegId(
                    30,
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
                    57,
                ),
                gid: PbsGid(
                    61,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    60,
                ),
                src_rid: RegId(
                    58,
                ),
                gid: PbsGid(
                    62,
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
                    61,
                ),
                src0_rid: RegId(
                    59,
                ),
                src1_rid: RegId(
                    60,
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
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    61,
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
                    56,
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
    ST(
        DOpSt(
            PeMemInsn {
                rid: RegId(
                    51,
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
                    46,
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
