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
                    4,
                ),
                src_rid: RegId(
                    2,
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
                    4,
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
    SUB(
        DOpSub(
            PeArithInsn {
                dst_rid: RegId(
                    6,
                ),
                src0_rid: RegId(
                    6,
                ),
                src1_rid: RegId(
                    6,
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
                    6,
                ),
                src_rid: RegId(
                    6,
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
                    7,
                ),
                src0_rid: RegId(
                    5,
                ),
                src1_rid: RegId(
                    6,
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
                    7,
                ),
                src0_rid: RegId(
                    7,
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
                    8,
                ),
                src_rid: RegId(
                    7,
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
                    9,
                ),
                src_rid: RegId(
                    7,
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
                    9,
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
                    10,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    12,
                ),
                src0_rid: RegId(
                    12,
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
                    13,
                ),
                src_rid: RegId(
                    12,
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
                    14,
                ),
                src_rid: RegId(
                    12,
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
                    14,
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
                    15,
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
    ADD(
        DOpAdd(
            PeArithInsn {
                dst_rid: RegId(
                    17,
                ),
                src0_rid: RegId(
                    15,
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
                    17,
                ),
                src0_rid: RegId(
                    17,
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
    PBS(
        DOpPbs(
            PePbsInsn {
                dst_rid: RegId(
                    18,
                ),
                src_rid: RegId(
                    17,
                ),
                gid: PbsGid(
                    34,
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
                    18,
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
                    19,
                ),
                src_rid: RegId(
                    17,
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
                    19,
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
