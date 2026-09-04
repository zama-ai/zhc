//! Translation from IOP language to HPU language.
//!
//! This module provides translation capabilities that convert intermediate
//! representations from the integer operation language (IOP) to the HPU
//! hardware language. The translation maps high-level operations to
//! low-level hardware primitives while preserving semantic correctness.
use zhc_ir::{
    IR,
    translation::{Order, Translation, translate_ann},
};
use zhc_langs::{
    hpulang::{HpuInstructionSet, HpuLang, Immediate, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopLang},
};
use zhc_utils::{SafeAs, small::SmallMap, svec};

pub(crate) fn lower_iop_to_hpu(ir: &IR<IopLang>) -> Translation<HpuLang> {
    use IopInstructionSet::*;
    let remap = ir
        .walk_ops_linear()
        .filter(|op| {
            matches!(
                op.get_instruction(),
                InputCiphertext { .. } | InputPlaintext { .. }
            )
        })
        .scan((0, 0), |(ct_id, pt_id), op| match op.get_instruction() {
            InputCiphertext { pos, .. } => {
                *ct_id += 1;
                Some((*pos, *ct_id - 1))
            }
            InputPlaintext { pos, .. } => {
                *pt_id += 1;
                Some((*pos, *pt_id - 1))
            }
            _ => unreachable!(),
        })
        .collect::<SmallMap<usize, usize>>();
    let ann_ir = ir
        .forward_dataflow_analysis(|a| {
            let opann = match a.get_instruction() {
                InputCiphertext { pos, .. } | InputPlaintext { pos, .. } => {
                    Some(*remap.get(&pos).unwrap())
                }
                ExtractCtBlock { .. } | ExtractPtBlock { .. } => a
                    .get_args_iter()
                    .next()
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_annotation()
                    .clone()
                    .unwrap_analyzed(),
                _ => None,
            };
            let valanns = svec![(); a.get_return_arity()];
            (opann, valanns)
        })
        .backward_dataflow_analysis(|a, prev| {
            let opann = match a.get_instruction() {
                OutputCiphertext { pos, .. } => Some(*pos),
                StoreCtBlock { .. } => {
                    let ret = a.get_returns_iter().next().unwrap();
                    assert_eq!(ret.get_users_iter().count(), 1);
                    ret.get_users_iter()
                        .next()
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                }
                _ => *prev.get_annotation(),
            };
            let valanns = svec![(); a.get_return_arity()];
            (opann, valanns)
        });
    translate_ann(ann_ir.view(), Order::Linear, |op, translator| {
        match op.get_instruction() {
            IopInstructionSet::_Consume { .. } => {
                panic!("Tried to translate a _consume op");
            }
            IopInstructionSet::InputCiphertext { .. }
            | IopInstructionSet::InputPlaintext { .. }
            | IopInstructionSet::LetPlaintextBlock { .. } => {
                // Handled in consumers.
            }
            IopInstructionSet::OutputCiphertext { .. } => {
                // No-op
            }
            IopInstructionSet::DeclareCiphertext { .. } => {
                // DeclareCiphertext has no semantics in hpulang.
                // We just verify that it is not used in an unexpected way.
                assert!(
                    op.get_reached_iter().all(|reached| matches!(
                        reached.get_instruction(),
                        IopInstructionSet::StoreCtBlock { .. }
                            | IopInstructionSet::OutputCiphertext { .. }
                    )),
                    "Unexpectd use of DeclareCiphertext encountered."
                )
            }
            IopInstructionSet::Inspect { .. } => {
                // Aliases have no semantics in hpulang. And they may prevent CSE so there
                // should be no aliases remaining here,
                panic!("Unexpected Alias op encountered.");
            }
            IopInstructionSet::LetCiphertextBlock { value } => {
                translator.direct_translation(
                    &op,
                    HpuInstructionSet::CstCt {
                        cst: Immediate(*value),
                    },
                );
            }
            IopInstructionSet::AddCt { .. } => {
                translator.direct_translation(&op, HpuInstructionSet::AddCt);
            }
            IopInstructionSet::SubCt { .. } => {
                translator.direct_translation(&op, HpuInstructionSet::SubCt);
            }
            IopInstructionSet::ShlCt { amount, .. } => {
                translator.direct_translation(
                    &op,
                    HpuInstructionSet::MulCst {
                        cst: Immediate(1u8 << *amount),
                    },
                );
            }
            IopInstructionSet::PackCt { mul, .. } => {
                translator.direct_translation(
                    &op,
                    HpuInstructionSet::Mac {
                        cst: Immediate((*mul).sas()),
                    },
                );
            }
            IopInstructionSet::AddPt { .. } => {
                match op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                {
                    IopInstructionSet::LetPlaintextBlock { value } => {
                        let new_rets = translator.add_op(
                            HpuInstructionSet::AddCst {
                                cst: Immediate((*value).sas()),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, HpuInstructionSet::AddPt);
                    }
                }
            }
            IopInstructionSet::SubPt { .. } => {
                match op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                {
                    IopInstructionSet::LetPlaintextBlock { value } => {
                        let new_rets = translator.add_op(
                            HpuInstructionSet::SubCst {
                                cst: Immediate((*value).sas()),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, HpuInstructionSet::SubPt);
                    }
                }
            }
            IopInstructionSet::PtSub { .. } => {
                match op
                    .get_args_iter()
                    .nth(0)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                {
                    IopInstructionSet::LetPlaintextBlock { value } => {
                        let new_rets = translator.add_op(
                            HpuInstructionSet::CstSub {
                                cst: Immediate((*value).sas()),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[1])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, HpuInstructionSet::PtSub);
                    }
                }
            }
            IopInstructionSet::MulPt { .. } => {
                match op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                {
                    IopInstructionSet::LetPlaintextBlock { value } => {
                        let new_rets = translator.add_op(
                            HpuInstructionSet::MulCst {
                                cst: Immediate((*value).sas()),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, HpuInstructionSet::MulPt);
                    }
                }
            }
            IopInstructionSet::ExtractCtBlock { index } => {
                let new_rets = translator.add_op(
                    HpuInstructionSet::SrcLd {
                        from: TSrcId {
                            src_pos: op.get_annotation().unwrap().try_into().unwrap(),
                            block_pos: (*index).sas(),
                        },
                    },
                    svec![],
                );
                translator.register_translation(op.get_return_valids()[0], new_rets[0]);
            }
            IopInstructionSet::ExtractPtBlock { index } => {
                let new_rets = translator.add_op(
                    HpuInstructionSet::ImmLd {
                        from: TImmId {
                            imm_pos: op.get_annotation().unwrap().try_into().unwrap(),
                            block_pos: (*index).sas(),
                        },
                    },
                    svec![],
                );
                translator.register_translation(op.get_return_valids()[0], new_rets[0]);
            }
            IopInstructionSet::StoreCtBlock { index } => {
                let new_arg = translator.translate_val(op.get_arg_valids()[0]);
                translator.add_op(
                    HpuInstructionSet::DstSt {
                        to: TDstId {
                            dst_pos: op.get_annotation().unwrap().try_into().unwrap(),
                            block_pos: (*index).sas(),
                        },
                    },
                    svec![new_arg],
                );
            }
            IopInstructionSet::Pbs { lut, .. } => {
                // as matching is done using LUT fct, we have
                // 10 CmpSign = 40 NotNull
                // 11 CmpReduce = 51 SolveProp
                // ...
                translator.direct_translation(&op, HpuInstructionSet::Pbs { lut: lut.clone() });
            }
            IopInstructionSet::Pbs2 { lut, .. } => {
                translator.direct_translation(&op, HpuInstructionSet::Pbs2 { lut: lut.clone() });
            }
            IopInstructionSet::Pbs4 { lut, .. } => {
                translator.direct_translation(&op, HpuInstructionSet::Pbs4 { lut: lut.clone() });
            }
            IopInstructionSet::Pbs8 { lut, .. } => {
                translator.direct_translation(&op, HpuInstructionSet::Pbs8 { lut: lut.clone() });
            }
        }
    })
}

#[cfg(test)]
mod test {
    use zhc_builder::{
        Builder, CiphertextSpec, add, bitwise_and, bitwise_or, bitwise_xor, cmp_gt, count_0,
        count_1, if_then_else, if_then_zero, mul,
    };
    use zhc_ir::IR;
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_utils::assert_display_is;

    use crate::test::check_iop_hpu_equivalence;

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        super::lower_iop_to_hpu(&ir).output
    }

    #[test]
    fn test_translate_add_ir() {
        let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 = src_ld<0.0_tsrc>();
                %1 = src_ld<0.1_tsrc>();
                %2 = src_ld<0.2_tsrc>();
                %3 = src_ld<0.3_tsrc>();
                %4 = src_ld<0.4_tsrc>();
                %5 = src_ld<0.5_tsrc>();
                %6 = src_ld<0.6_tsrc>();
                %7 = src_ld<0.7_tsrc>();
                %8 = src_ld<1.0_tsrc>();
                %9 = src_ld<1.1_tsrc>();
                %10 = src_ld<1.2_tsrc>();
                %11 = src_ld<1.3_tsrc>();
                %12 = src_ld<1.4_tsrc>();
                %13 = src_ld<1.5_tsrc>();
                %14 = src_ld<1.6_tsrc>();
                %15 = src_ld<1.7_tsrc>();
                %16 = add_ct(%0, %8);
                %17 = add_ct(%1, %9);
                %18 = add_ct(%2, %10);
                %19 = add_ct(%3, %11);
                %20 = add_ct(%4, %12);
                %21 = add_ct(%5, %13);
                %22 = add_ct(%6, %14);
                %23 = add_ct(%7, %15);
                %24, %25 = pbs_2<Lut2("ManyCarryMsg")>(%16);
                %26 = pbs<Lut1("ExtractPropGroup0")>(%17);
                %27 = pbs<Lut1("ExtractPropGroup1")>(%18);
                %28 = pbs<Lut1("ExtractPropGroup2")>(%19);
                %29 = pbs<Lut1("ExtractPropGroup0")>(%20);
                %30 = pbs<Lut1("ExtractPropGroup1")>(%21);
                %31 = pbs<Lut1("ExtractPropGroup2")>(%22);
                %32 = add_ct(%25, %26);
                %33 = add_ct(%32, %27);
                %34 = add_ct(%33, %28);
                %35 = pbs<Lut1("SolvePropGroupFinal2")>(%34);
                %36 = add_ct(%29, %30);
                %37 = add_ct(%36, %31);
                %38 = pbs<Lut1("SolvePropGroupFinal0")>(%32);
                %39 = pbs<Lut1("SolvePropGroupFinal1")>(%33);
                %40 = add_ct(%29, %35);
                %41 = pbs<Lut1("SolvePropGroupFinal0")>(%40);
                %42 = add_ct(%36, %35);
                %43 = pbs<Lut1("SolvePropGroupFinal1")>(%42);
                %44 = add_ct(%37, %35);
                %45 = pbs<Lut1("SolvePropGroupFinal2")>(%44);
                %46 = add_ct(%17, %25);
                %47 = add_ct(%18, %38);
                %48 = add_ct(%19, %39);
                %49 = add_ct(%20, %35);
                %50 = add_ct(%21, %41);
                %51 = add_ct(%22, %43);
                %52 = add_ct(%23, %45);
                %53 = pbs<Lut1("MsgOnly")>(%24);
                %54 = pbs<Lut1("MsgOnly")>(%46);
                %55 = pbs<Lut1("MsgOnly")>(%47);
                %56 = pbs<Lut1("MsgOnly")>(%48);
                %57 = pbs<Lut1("MsgOnly")>(%49);
                %58 = pbs<Lut1("MsgOnly")>(%50);
                %59 = pbs<Lut1("MsgOnly")>(%51);
                %60 = pbs<Lut1("MsgOnly")>(%52);
                dst_st<0.0_tdst>(%53);
                dst_st<0.1_tdst>(%54);
                dst_st<0.2_tdst>(%55);
                dst_st<0.3_tdst>(%56);
                dst_st<0.4_tdst>(%57);
                dst_st<0.5_tdst>(%58);
                dst_st<0.6_tdst>(%59);
                dst_st<0.7_tdst>(%60);
            "#
        );
    }

    #[test]
    fn test_translate_cmp_ir() {
        let ir = pipeline(&cmp_gt(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 = src_ld<0.0_tsrc>();
                %1 = src_ld<0.1_tsrc>();
                %2 = src_ld<0.2_tsrc>();
                %3 = src_ld<0.3_tsrc>();
                %4 = src_ld<0.4_tsrc>();
                %5 = src_ld<0.5_tsrc>();
                %6 = src_ld<0.6_tsrc>();
                %7 = src_ld<0.7_tsrc>();
                %8 = src_ld<1.0_tsrc>();
                %9 = src_ld<1.1_tsrc>();
                %10 = src_ld<1.2_tsrc>();
                %11 = src_ld<1.3_tsrc>();
                %12 = src_ld<1.4_tsrc>();
                %13 = src_ld<1.5_tsrc>();
                %14 = src_ld<1.6_tsrc>();
                %15 = src_ld<1.7_tsrc>();
                %16 = mac<4_imm>(%1, %0);
                %17 = pbs<Lut1("None")>(%16);
                %18 = mac<4_imm>(%3, %2);
                %19 = pbs<Lut1("None")>(%18);
                %20 = mac<4_imm>(%5, %4);
                %21 = pbs<Lut1("None")>(%20);
                %22 = mac<4_imm>(%7, %6);
                %23 = pbs<Lut1("None")>(%22);
                %24 = mac<4_imm>(%9, %8);
                %25 = pbs<Lut1("None")>(%24);
                %26 = mac<4_imm>(%11, %10);
                %27 = pbs<Lut1("None")>(%26);
                %28 = mac<4_imm>(%13, %12);
                %29 = pbs<Lut1("None")>(%28);
                %30 = mac<4_imm>(%15, %14);
                %31 = pbs<Lut1("None")>(%30);
                %32 = sub_ct(%17, %25);
                %33 = pbs<Lut1("CmpSign")>(%32);
                %34 = add_cst<1_imm>(%33);
                %35 = sub_ct(%19, %27);
                %36 = pbs<Lut1("CmpSign")>(%35);
                %37 = add_cst<1_imm>(%36);
                %38 = sub_ct(%21, %29);
                %39 = pbs<Lut1("CmpSign")>(%38);
                %40 = add_cst<1_imm>(%39);
                %41 = sub_ct(%23, %31);
                %42 = pbs<Lut1("CmpSign")>(%41);
                %43 = add_cst<1_imm>(%42);
                %44 = mac<4_imm>(%37, %34);
                %45 = pbs<Lut1("CmpReduce")>(%44);
                %46 = mac<4_imm>(%43, %40);
                %47 = pbs<Lut1("CmpReduce")>(%46);
                %48 = mac<4_imm>(%47, %45);
                %49 = pbs<Lut1("CmpGtMrg")>(%48);
                dst_st<0.0_tdst>(%49);
            "#
        );
    }

    #[test]
    fn correctness() {
        let check = |b: Builder| {
            let spec = *b.spec();
            let iop_ir = b.optimize_ir();
            let hpu_ir = pipeline(&iop_ir);
            check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
        };
        for size in 2..=64 {
            let spec = CiphertextSpec::new(size, 2, 2);
            check(add(spec));
            check(bitwise_and(spec));
            check(bitwise_or(spec));
            check(bitwise_xor(spec));
            check(if_then_else(spec));
            check(if_then_zero(spec));
            check(mul(spec));
            if spec.int_size().is_multiple_of(2) {
                check(count_0(spec));
                check(count_1(spec));
            }
        }
    }
}
