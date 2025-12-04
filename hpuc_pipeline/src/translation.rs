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
                IopOp::GenerateLut2 { .. } => {
                    // TODO : Perform lut registering.
                }
                IopOp::GenerateLut4 { .. } => {
                    // TODO : Perform lut registering.
                }
                IopOp::GenerateLut8 { .. } => {
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
    use hpuc_ir::{translation::Translator, IR};
    use hpuc_langs::{hpulang::Hpulang, ioplang::Ioplang};

    use crate::test::{get_add_ir, get_cmp_ir, get_sub_ir};

    use super::IoplangToHpulang;

    fn pipeline(ir: &IR<Ioplang>) -> IR<Hpulang> {
        IoplangToHpulang.translate(&ir)
    }

    #[test]
    fn test_translate_add_ir() {
        let ir = pipeline(&get_add_ir(16, 2, 2));
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<1.0_tsrc>();
            %8 : CtRegister = src_ld<1.1_tsrc>();
            %9 : CtRegister = src_ld<1.2_tsrc>();
            %10 : CtRegister = src_ld<1.3_tsrc>();
            %11 : CtRegister = src_ld<1.4_tsrc>();
            %12 : CtRegister = src_ld<1.5_tsrc>();
            %13 : CtRegister = src_ld<1.6_tsrc>();
            %14 : CtRegister = add_ct(%0, %7);
            %15 : CtRegister = add_ct(%1, %8);
            %16 : CtRegister = add_ct(%2, %9);
            %17 : CtRegister = add_ct(%3, %10);
            %18 : CtRegister = add_ct(%4, %11);
            %19 : CtRegister = add_ct(%5, %12);
            %20 : CtRegister = add_ct(%6, %13);
            %21 : CtRegister, %22 : CtRegister = pbs_2<Lut@0>(%14);
            %23 : CtRegister = pbs<Lut@0>(%15);
            %24 : CtRegister = pbs<Lut@0>(%16);
            %25 : CtRegister = pbs<Lut@0>(%17);
            %26 : CtRegister = pbs<Lut@0>(%18);
            %27 : CtRegister = pbs<Lut@0>(%19);
            %28 : CtRegister = pbs<Lut@0>(%20);
            %29 : CtRegister = add_ct(%23, %22);
            %30 : CtRegister = add_ct(%27, %26);
            %31 : CtRegister = add_ct(%15, %22);
            dst_st<0.0_tdst>(%21);
            %32 : CtRegister = add_ct(%24, %29);
            %33 : CtRegister = add_ct(%28, %30);
            %34 : CtRegister = pbs<Lut@0>(%29);
            dst_st<0.1_tdst>(%31);
            %35 : CtRegister = add_ct(%25, %32);
            %36 : CtRegister = pbs<Lut@0>(%32);
            %37 : CtRegister = add_ct(%16, %34);
            %38 : CtRegister = pbs<Lut@0>(%35);
            %39 : CtRegister = add_ct(%17, %36);
            dst_st<0.2_tdst>(%37);
            %40 : CtRegister = add_ct(%26, %38);
            %41 : CtRegister = add_ct(%30, %38);
            %42 : CtRegister = add_ct(%33, %38);
            dst_st<0.3_tdst>(%39);
            %43 : CtRegister = pbs<Lut@0>(%40);
            %44 : CtRegister = pbs<Lut@0>(%41);
            %45 : CtRegister = pbs<Lut@0>(%42);
            %46 : CtRegister = add_ct(%18, %43);
            %47 : CtRegister = add_ct(%19, %44);
            %48 : CtRegister = add_ct(%20, %45);
            dst_st<0.4_tdst>(%46);
            dst_st<0.5_tdst>(%47);
            dst_st<0.6_tdst>(%48);
        ",
        );
    }

    #[test]
    fn test_translate_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
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
        let ir = pipeline(&get_sub_ir(16, 2, 2));
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<1.0_tsrc>();
            %8 : CtRegister = src_ld<1.1_tsrc>();
            %9 : CtRegister = src_ld<1.2_tsrc>();
            %10 : CtRegister = src_ld<1.3_tsrc>();
            %11 : CtRegister = src_ld<1.4_tsrc>();
            %12 : CtRegister = src_ld<1.5_tsrc>();
            %13 : CtRegister = src_ld<1.6_tsrc>();
            %14 : CtRegister = cst_sub<3_imm>(%7);
            %15 : CtRegister = cst_sub<3_imm>(%8);
            %16 : CtRegister = cst_sub<3_imm>(%9);
            %17 : CtRegister = cst_sub<3_imm>(%10);
            %18 : CtRegister = cst_sub<3_imm>(%11);
            %19 : CtRegister = cst_sub<3_imm>(%12);
            %20 : CtRegister = cst_sub<3_imm>(%13);
            %21 : CtRegister = add_ct(%0, %14);
            %22 : CtRegister = add_ct(%1, %15);
            %23 : CtRegister = add_ct(%2, %16);
            %24 : CtRegister = add_ct(%3, %17);
            %25 : CtRegister = add_ct(%4, %18);
            %26 : CtRegister = add_ct(%5, %19);
            %27 : CtRegister = add_ct(%6, %20);
            %28 : CtRegister, %29 : CtRegister = pbs_2<Lut@0>(%21);
            %30 : CtRegister = pbs<Lut@0>(%22);
            %31 : CtRegister = pbs<Lut@0>(%23);
            %32 : CtRegister = pbs<Lut@0>(%24);
            %33 : CtRegister = pbs<Lut@0>(%25);
            %34 : CtRegister = pbs<Lut@0>(%26);
            %35 : CtRegister = pbs<Lut@0>(%27);
            %36 : CtRegister = add_ct(%30, %29);
            %37 : CtRegister = add_ct(%34, %33);
            %38 : CtRegister = add_ct(%22, %29);
            %39 : CtRegister = pbs<Lut@0>(%28);
            %40 : CtRegister = add_ct(%31, %36);
            %41 : CtRegister = add_ct(%35, %37);
            %42 : CtRegister = pbs<Lut@0>(%36);
            %43 : CtRegister = pbs<Lut@0>(%38);
            dst_st<0.0_tdst>(%39);
            %44 : CtRegister = add_ct(%32, %40);
            %45 : CtRegister = pbs<Lut@0>(%40);
            %46 : CtRegister = add_ct(%23, %42);
            dst_st<0.1_tdst>(%43);
            %47 : CtRegister = pbs<Lut@0>(%44);
            %48 : CtRegister = add_ct(%24, %45);
            %49 : CtRegister = pbs<Lut@0>(%46);
            %50 : CtRegister = add_ct(%33, %47);
            %51 : CtRegister = add_ct(%37, %47);
            %52 : CtRegister = add_ct(%41, %47);
            %53 : CtRegister = pbs<Lut@0>(%48);
            dst_st<0.2_tdst>(%49);
            %54 : CtRegister = pbs<Lut@0>(%50);
            %55 : CtRegister = pbs<Lut@0>(%51);
            %56 : CtRegister = pbs<Lut@0>(%52);
            dst_st<0.3_tdst>(%53);
            %57 : CtRegister = add_ct(%25, %54);
            %58 : CtRegister = add_ct(%26, %55);
            %59 : CtRegister = add_ct(%27, %56);
            %60 : CtRegister = pbs<Lut@0>(%57);
            %61 : CtRegister = pbs<Lut@0>(%58);
            %62 : CtRegister = pbs<Lut@0>(%59);
            dst_st<0.4_tdst>(%60);
            dst_st<0.5_tdst>(%61);
            dst_st<0.6_tdst>(%62);
        ",
        );
    }
}
