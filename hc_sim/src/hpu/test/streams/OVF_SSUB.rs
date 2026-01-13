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
                    4,
                ),
                opcode: Opcode {
                    optype: ARITH,
                    subtype: 11,
                },
            },
        ),
    ),
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    2,
                ),
                src0_rid: RegId(
                    2,
                ),
                src1_rid: RegId(
                    2,
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
                    2,
                ),
                src_rid: RegId(
                    2,
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
                    3,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS_F(
        DOpPbsF(
            PePbsInsn {
                dst_rid: RegId(
                    5,
                ),
                src_rid: RegId(
                    3,
                ),
                gid: PbsGid(
                    1,
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
                    5,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    6,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    8,
                ),
                src0_rid: RegId(
                    8,
                ),
                src1_rid: RegId(
                    8,
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
                    8,
                ),
                src_rid: RegId(
                    8,
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
                    9,
                ),
                src0_rid: RegId(
                    8,
                ),
                src1_rid: RegId(
                    7,
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
                    9,
                ),
                src0_rid: RegId(
                    9,
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
                    3,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS_F(
        DOpPbsF(
            PePbsInsn {
                dst_rid: RegId(
                    11,
                ),
                src_rid: RegId(
                    9,
                ),
                gid: PbsGid(
                    1,
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
                    11,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    12,
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
                    13,
                ),
                src_rid: RegId(
                    12,
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
                    15,
                ),
                src0_rid: RegId(
                    14,
                ),
                src1_rid: RegId(
                    13,
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
                    15,
                ),
                src0_rid: RegId(
                    15,
                ),
                src1_rid: RegId(
                    10,
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
                    16,
                ),
                src_rid: RegId(
                    15,
                ),
                gid: PbsGid(
                    3,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
                },
            },
        ),
    ),
    PBS_F(
        DOpPbsF(
            PePbsInsn {
                dst_rid: RegId(
                    17,
                ),
                src_rid: RegId(
                    15,
                ),
                gid: PbsGid(
                    1,
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
                    17,
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
    LD(
        DOpLd(
            PeMemInsn {
                rid: RegId(
                    18,
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
                    19,
                ),
                src_rid: RegId(
                    18,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    20,
                ),
                src0_rid: RegId(
                    20,
                ),
                src1_rid: RegId(
                    20,
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
                    20,
                ),
                src_rid: RegId(
                    20,
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
                    21,
                ),
                src0_rid: RegId(
                    20,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    21,
                ),
                src0_rid: RegId(
                    21,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    22,
                ),
                src_rid: RegId(
                    21,
                ),
                gid: PbsGid(
                    35,
                ),
                opcode: Opcode {
                    optype: PBST,
                    subtype: 0,
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
    PBS_F(
        DOpPbsF(
            PePbsInsn {
                dst_rid: RegId(
                    23,
                ),
                src_rid: RegId(
                    21,
                ),
                gid: PbsGid(
                    1,
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
                    23,
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
