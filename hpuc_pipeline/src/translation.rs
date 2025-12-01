use hpuc_ir::{IR, OpId, ValId, translation::Translator};
use hpuc_langs::{
    hpulang::{Hpulang, Immediate, LutMemoryAdress, TDstId, TImmId, TSrcId},
    ioplang::{Ioplang, Litteral},
};
use hpuc_utils::{FastMap, svec};

pub struct IoplangToHpulang;

impl Translator for IoplangToHpulang {
    type InputDialect = Ioplang;
    type OutputDialect = Hpulang;

    fn translate(
        &mut self,
        input: &hpuc_ir::IR<Self::InputDialect>,
    ) -> hpuc_ir::IR<Self::OutputDialect> {
        use hpuc_langs::hpulang::Operations as HpuOp;
        use hpuc_langs::ioplang::Operations as IopOp;
        use hpuc_langs::ioplang::Types as IopTy;

        // This translator performs a flow-following translation of an IR in Ioplang to an IR in
        // Hpulang. It is very simple, and as such pretty fast. Every operation is matched
        // against its optype, and translated to an equivalent operation in the Hpulang.
        let mut output = IR::empty();
        let mut map = input.empty_valmap::<ValId>();

        // Ioplang has a value semantics. This means that dst are defined by use, and as such, only
        // known at the end of the program when `output` ops are given. Hpulang has a
        // register semantics, and as such, the return position must be known beforehand.
        // For this reason, we need to gather the output position for each `let` ops
        // upfront, to be able to correctly set the TDstId of the `dst_st` ops
        let let_map: FastMap<OpId, usize> = input
            .walk_ops_linear()
            .filter(|op| {
                // Keep the ciphertext output ops.
                matches!(
                    op.get_operation(),
                    IopOp::Output {
                        typ: IopTy::Ciphertext,
                        ..
                    }
                )
            })
            .map(|oup_op| {
                // For the output, we search the let reaching this output.
                let let_pred = oup_op
                    .get_inc_reaching_iter()
                    .find(|pr| {
                        matches!(
                            pr.get_operation(),
                            IopOp::Let {
                                typ: IopTy::Ciphertext
                            }
                        )
                    })
                    .expect("Failed to find the `let` predecessor of an `output` op.");
                let IopOp::Output { pos, .. } = oup_op.get_operation() else {
                    unreachable!()
                };
                (let_pred.get_id(), pos)
            })
            .collect();

        for op in input.walk_ops_topological() {
            match op.get_operation() {
                IopOp::Input { .. } | IopOp::Let { .. } | IopOp::Constant { .. } => {
                    // Handled in consumers.
                }
                IopOp::Output { .. } => {
                    // Nop
                }
                IopOp::GenerateLut { .. } => {
                    // TODO : Perform lut registering.
                }
                IopOp::AddCt => {
                    let (_, valids) = output
                        .add_op(
                            HpuOp::AddCt,
                            svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::SubCt => {
                    let (_, valids) = output
                        .add_op(
                            HpuOp::SubCt,
                            svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::Mac => {
                    let IopOp::Constant {
                        value: Litteral::PlaintextBlock(cst),
                    } = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_origin()
                        .get_operation()
                    else {
                        unreachable!()
                    };
                    let (_, valids) = output
                        .add_op(
                            HpuOp::Mac {
                                cst: Immediate(cst),
                            },
                            svec![map[op.get_arg_valids()[1]], map[op.get_arg_valids()[2]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::AddPt => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[1]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuOp::AddPt,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopOp::Constant {
                            value: Litteral::PlaintextBlock(cst),
                        } = op
                            .get_args_iter()
                            .nth(1)
                            .unwrap()
                            .get_origin()
                            .get_operation()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuOp::AddCst {
                                    cst: Immediate(cst),
                                },
                                svec![map[op.get_arg_valids()[0]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::SubPt => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[1]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuOp::SubPt,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopOp::Constant {
                            value: Litteral::PlaintextBlock(cst),
                        } = op
                            .get_args_iter()
                            .nth(1)
                            .unwrap()
                            .get_origin()
                            .get_operation()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuOp::SubCst {
                                    cst: Immediate(cst),
                                },
                                svec![map[op.get_arg_valids()[0]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::PtSub => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[0]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuOp::PtSub,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopOp::Constant {
                            value: Litteral::PlaintextBlock(cst),
                        } = op
                            .get_args_iter()
                            .nth(0)
                            .unwrap()
                            .get_origin()
                            .get_operation()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuOp::CstSub {
                                    cst: Immediate(cst),
                                },
                                svec![map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::MulPt => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[1]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuOp::MulPt,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopOp::Constant {
                            value: Litteral::PlaintextBlock(cst),
                        } = op
                            .get_args_iter()
                            .nth(1)
                            .unwrap()
                            .get_origin()
                            .get_operation()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuOp::MulCst {
                                    cst: Immediate(cst),
                                },
                                svec![map[op.get_arg_valids()[0]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::ExtractCtBlock => {
                    let src_pos = op
                        .get_args_iter()
                        .nth(0) // ct arg.
                        .unwrap()
                        .get_origin()
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_operation() {
                            IopOp::Input {
                                typ: IopTy::Ciphertext,
                                pos,
                            } => Some(pos),
                            _ => None,
                        })
                        .unwrap();
                    let block_pos = op
                        .get_args_iter()
                        .nth(1) // index arg.
                        .unwrap()
                        .get_origin()
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_operation() {
                            IopOp::Constant {
                                value: Litteral::Index(i),
                            } => Some(i),
                            _ => None,
                        })
                        .unwrap();
                    let (_, valids) = output
                        .add_op(
                            HpuOp::SrcLd {
                                from: TSrcId { src_pos, block_pos },
                            },
                            svec![],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::ExtractPtBlock => {
                    let imm_pos = op
                        .get_args_iter()
                        .nth(0) // pt arg.
                        .unwrap()
                        .get_origin()
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_operation() {
                            IopOp::Input {
                                typ: IopTy::Plaintext,
                                pos,
                            } => Some(pos),
                            _ => None,
                        })
                        .unwrap();
                    let block_pos = op
                        .get_args_iter()
                        .nth(1) // index arg.
                        .unwrap()
                        .get_origin()
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_operation() {
                            IopOp::Constant {
                                value: Litteral::Index(i),
                            } => Some(i),
                            _ => None,
                        })
                        .unwrap();
                    let (_, valids) = output
                        .add_op(
                            HpuOp::ImmLd {
                                from: TImmId { imm_pos, block_pos },
                            },
                            svec![],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::StoreCtBlock => {
                    let dst_pos = op
                        .get_args_iter()
                        .nth(1) // ct arg.
                        .unwrap()
                        .get_origin()
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_operation() {
                            IopOp::Let {
                                typ: IopTy::Ciphertext,
                            } => let_map.get(&op.get_id()).cloned(),
                            _ => None,
                        })
                        .unwrap();
                    let block_pos = op
                        .get_args_iter()
                        .nth(2) // index arg.
                        .unwrap()
                        .get_origin()
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_operation() {
                            IopOp::Constant {
                                value: Litteral::Index(i),
                            } => Some(i),
                            _ => None,
                        })
                        .unwrap();
                    output
                        .add_op(
                            HpuOp::DstSt {
                                to: TDstId { dst_pos, block_pos },
                            },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                }
                IopOp::Pbs => {
                    let (_, valids) = output
                        .add_op(
                            HpuOp::Pbs {
                                lut: LutMemoryAdress(0),
                            },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopOp::Pbs2 => {
                    let (_, valids) = output
                        .add_op(
                            HpuOp::Pbs2 {
                                lut: LutMemoryAdress(0),
                            },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                    map.insert(op.get_return_valids()[1], valids[1]);
                }
                IopOp::Pbs4 => {
                    let (_, valids) = output
                        .add_op(
                            HpuOp::Pbs4 {
                                lut: LutMemoryAdress(0),
                            },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                    map.insert(op.get_return_valids()[1], valids[1]);
                    map.insert(op.get_return_valids()[2], valids[2]);
                    map.insert(op.get_return_valids()[3], valids[3]);
                }
                IopOp::Pbs8 => {
                    let (_, valids) = output
                        .add_op(
                            HpuOp::Pbs8 {
                                lut: LutMemoryAdress(0),
                            },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                    map.insert(op.get_return_valids()[1], valids[1]);
                    map.insert(op.get_return_valids()[2], valids[2]);
                    map.insert(op.get_return_valids()[3], valids[3]);
                    map.insert(op.get_return_valids()[4], valids[4]);
                    map.insert(op.get_return_valids()[5], valids[5]);
                    map.insert(op.get_return_valids()[6], valids[6]);
                    map.insert(op.get_return_valids()[7], valids[7]);
                }
            }
        }

        return output;
    }
}

#[cfg(test)]
mod test {
    use hpuc_ir::translation::Translator;

    use crate::test::{get_add_ir, get_cmp_ir, get_sub_ir};

    use super::IoplangToHpulang;

    #[test]
    fn test_translate_add_ir() {
        let ir = get_add_ir(16, 2, 2);
        let ir = IoplangToHpulang.translate(&ir);
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<0.7_tsrc>();
            %8 : CtRegister = src_ld<1.0_tsrc>();
            %9 : CtRegister = src_ld<1.1_tsrc>();
            %10 : CtRegister = src_ld<1.2_tsrc>();
            %11 : CtRegister = src_ld<1.3_tsrc>();
            %12 : CtRegister = src_ld<1.4_tsrc>();
            %13 : CtRegister = src_ld<1.5_tsrc>();
            %14 : CtRegister = src_ld<1.6_tsrc>();
            %15 : CtRegister = src_ld<1.7_tsrc>();
            %16 : CtRegister = add_ct(%0, %8);
            %17 : CtRegister = add_ct(%1, %9);
            %18 : CtRegister = add_ct(%2, %10);
            %19 : CtRegister = add_ct(%3, %11);
            %20 : CtRegister = add_ct(%4, %12);
            %21 : CtRegister = add_ct(%5, %13);
            %22 : CtRegister = add_ct(%6, %14);
            %23 : CtRegister = add_ct(%7, %15);
            %24 : CtRegister, %25 : CtRegister = pbs_2<Lut@0>(%16);
            %26 : CtRegister = pbs<Lut@0>(%17);
            %27 : CtRegister = pbs<Lut@0>(%18);
            %28 : CtRegister = pbs<Lut@0>(%19);
            %29 : CtRegister = pbs<Lut@0>(%20);
            %30 : CtRegister = pbs<Lut@0>(%21);
            %31 : CtRegister = pbs<Lut@0>(%22);
            %32 : CtRegister = pbs<Lut@0>(%23);
            %33 : CtRegister = add_ct(%26, %25);
            %34 : CtRegister = add_ct(%30, %29);
            %35 : CtRegister = add_ct(%17, %25);
            dst_st<0.0_tdst>(%24);
            %36 : CtRegister = add_ct(%27, %33);
            %37 : CtRegister = add_ct(%31, %34);
            %38 : CtRegister = pbs<Lut@0>(%33);
            dst_st<0.1_tdst>(%35);
            %39 : CtRegister = add_ct(%28, %36);
            %40 : CtRegister = add_ct(%32, %37);
            %41 : CtRegister = pbs<Lut@0>(%36);
            %42 : CtRegister = add_ct(%18, %38);
            %43 : CtRegister = pbs<Lut@0>(%39);
            %44 : CtRegister = pbs<Lut@0>(%40);
            %45 : CtRegister = add_ct(%19, %41);
            dst_st<0.2_tdst>(%42);
            %46 : CtRegister = add_cst<1_imm>(%44);
            %47 : CtRegister = add_ct(%29, %43);
            %48 : CtRegister = add_ct(%34, %43);
            %49 : CtRegister = add_ct(%37, %43);
            dst_st<0.3_tdst>(%45);
            %50 : CtRegister = mac<4_imm>(%43, %46);
            %51 : CtRegister = pbs<Lut@0>(%47);
            %52 : CtRegister = pbs<Lut@0>(%48);
            %53 : CtRegister = pbs<Lut@0>(%49);
            %54 : CtRegister = pbs<Lut@0>(%50);
            %55 : CtRegister = add_ct(%20, %51);
            %56 : CtRegister = add_ct(%21, %52);
            %57 : CtRegister = add_ct(%22, %53);
            dst_st<0.4_tdst>(%55);
            dst_st<0.5_tdst>(%56);
            dst_st<0.6_tdst>(%57);
        ",
        );
    }

    #[test]
    fn test_translate_cmp_ir() {
        let ir = get_cmp_ir(16, 2, 2);
        let ir = IoplangToHpulang.translate(&ir);
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<0.7_tsrc>();
            %8 : CtRegister = src_ld<1.0_tsrc>();
            %9 : CtRegister = src_ld<1.1_tsrc>();
            %10 : CtRegister = src_ld<1.2_tsrc>();
            %11 : CtRegister = src_ld<1.3_tsrc>();
            %12 : CtRegister = src_ld<1.4_tsrc>();
            %13 : CtRegister = src_ld<1.5_tsrc>();
            %14 : CtRegister = src_ld<1.6_tsrc>();
            %15 : CtRegister = src_ld<1.7_tsrc>();
            %16 : CtRegister = mac<4_imm>(%1, %0);
            %17 : CtRegister = mac<4_imm>(%3, %2);
            %18 : CtRegister = mac<4_imm>(%5, %4);
            %19 : CtRegister = mac<4_imm>(%7, %6);
            %20 : CtRegister = mac<4_imm>(%9, %8);
            %21 : CtRegister = mac<4_imm>(%11, %10);
            %22 : CtRegister = mac<4_imm>(%13, %12);
            %23 : CtRegister = mac<4_imm>(%15, %14);
            %24 : CtRegister = sub_ct(%16, %20);
            %25 : CtRegister = sub_ct(%17, %21);
            %26 : CtRegister = sub_ct(%18, %22);
            %27 : CtRegister = sub_ct(%19, %23);
            %28 : CtRegister = pbs<Lut@0>(%24);
            %29 : CtRegister = pbs<Lut@0>(%25);
            %30 : CtRegister = pbs<Lut@0>(%26);
            %31 : CtRegister = pbs<Lut@0>(%27);
            %32 : CtRegister = mac<4_imm>(%29, %28);
            %33 : CtRegister = mac<4_imm>(%31, %30);
            %34 : CtRegister = pbs<Lut@0>(%32);
            %35 : CtRegister = pbs<Lut@0>(%33);
            %36 : CtRegister = mac<4_imm>(%35, %34);
            %37 : CtRegister = pbs<Lut@0>(%36);
            dst_st<0.0_tdst>(%37);
        ",
        );
    }

    #[test]
    fn test_translate_sub_ir() {
        let ir = get_sub_ir(16, 2, 2);
        let ir = IoplangToHpulang.translate(&ir);
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<0.7_tsrc>();
            %8 : CtRegister = src_ld<1.0_tsrc>();
            %9 : CtRegister = src_ld<1.1_tsrc>();
            %10 : CtRegister = src_ld<1.2_tsrc>();
            %11 : CtRegister = src_ld<1.3_tsrc>();
            %12 : CtRegister = src_ld<1.4_tsrc>();
            %13 : CtRegister = src_ld<1.5_tsrc>();
            %14 : CtRegister = src_ld<1.6_tsrc>();
            %15 : CtRegister = src_ld<1.7_tsrc>();
            %16 : CtRegister = cst_sub<3_imm>(%8);
            %17 : CtRegister = cst_sub<3_imm>(%9);
            %18 : CtRegister = cst_sub<3_imm>(%10);
            %19 : CtRegister = cst_sub<3_imm>(%11);
            %20 : CtRegister = cst_sub<3_imm>(%12);
            %21 : CtRegister = cst_sub<3_imm>(%13);
            %22 : CtRegister = cst_sub<3_imm>(%14);
            %23 : CtRegister = cst_sub<3_imm>(%15);
            %24 : CtRegister = add_ct(%0, %16);
            %25 : CtRegister = add_ct(%1, %17);
            %26 : CtRegister = add_ct(%2, %18);
            %27 : CtRegister = add_ct(%3, %19);
            %28 : CtRegister = add_ct(%4, %20);
            %29 : CtRegister = add_ct(%5, %21);
            %30 : CtRegister = add_ct(%6, %22);
            %31 : CtRegister = add_ct(%7, %23);
            %32 : CtRegister, %33 : CtRegister = pbs_2<Lut@0>(%24);
            %34 : CtRegister = pbs<Lut@0>(%25);
            %35 : CtRegister = pbs<Lut@0>(%26);
            %36 : CtRegister = pbs<Lut@0>(%27);
            %37 : CtRegister = pbs<Lut@0>(%28);
            %38 : CtRegister = pbs<Lut@0>(%29);
            %39 : CtRegister = pbs<Lut@0>(%30);
            %40 : CtRegister = pbs<Lut@0>(%31);
            %41 : CtRegister = add_ct(%34, %33);
            %42 : CtRegister = add_ct(%38, %37);
            %43 : CtRegister = add_ct(%25, %33);
            %44 : CtRegister = pbs<Lut@0>(%32);
            %45 : CtRegister = add_ct(%35, %41);
            %46 : CtRegister = add_ct(%39, %42);
            %47 : CtRegister = pbs<Lut@0>(%41);
            %48 : CtRegister = pbs<Lut@0>(%43);
            dst_st<0.0_tdst>(%44);
            %49 : CtRegister = add_ct(%36, %45);
            %50 : CtRegister = add_ct(%40, %46);
            %51 : CtRegister = pbs<Lut@0>(%45);
            %52 : CtRegister = add_ct(%26, %47);
            dst_st<0.1_tdst>(%48);
            %53 : CtRegister = pbs<Lut@0>(%49);
            %54 : CtRegister = pbs<Lut@0>(%50);
            %55 : CtRegister = add_ct(%27, %51);
            %56 : CtRegister = pbs<Lut@0>(%52);
            %57 : CtRegister = add_cst<1_imm>(%54);
            %58 : CtRegister = add_ct(%37, %53);
            %59 : CtRegister = add_ct(%42, %53);
            %60 : CtRegister = add_ct(%46, %53);
            %61 : CtRegister = pbs<Lut@0>(%55);
            dst_st<0.2_tdst>(%56);
            %62 : CtRegister = mac<4_imm>(%53, %57);
            %63 : CtRegister = pbs<Lut@0>(%58);
            %64 : CtRegister = pbs<Lut@0>(%59);
            %65 : CtRegister = pbs<Lut@0>(%60);
            dst_st<0.3_tdst>(%61);
            %66 : CtRegister = pbs<Lut@0>(%62);
            %67 : CtRegister = add_ct(%28, %63);
            %68 : CtRegister = add_ct(%29, %64);
            %69 : CtRegister = add_ct(%30, %65);
            %70 : CtRegister = pbs<Lut@0>(%67);
            %71 : CtRegister = pbs<Lut@0>(%68);
            %72 : CtRegister = pbs<Lut@0>(%69);
            dst_st<0.4_tdst>(%70);
            dst_st<0.5_tdst>(%71);
            dst_st<0.6_tdst>(%72);
        ",
        );
    }
}
