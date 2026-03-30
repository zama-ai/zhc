//! Translation from IOP language to HPU language.
//!
//! This module provides translation capabilities that convert intermediate
//! representations from the integer operation language (IOP) to the HPU
//! hardware language. The translation maps high-level operations to
//! low-level hardware primitives while preserving semantic correctness.

use std::{collections::HashMap, sync::LazyLock};

use zhc_ir::{IR, translation::eager_translate_ann};
use zhc_langs::{
    hpulang::{HpuInstructionSet, HpuLang, Immediate, LutId, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopLang, Lut1Def, Lut2Def, Lut4Def, Lut8Def},
};
use zhc_utils::{FastMap, svec};

pub(crate) static GIDS1: LazyLock<FastMap<Lut1Def, LutId>> = LazyLock::new(|| {
    HashMap::from([
        (Lut1Def::None, LutId(0)),
        (Lut1Def::MsgOnly, LutId(1)),
        (Lut1Def::CarryOnly, LutId(2)),
        (Lut1Def::CarryInMsg, LutId(3)),
        (Lut1Def::MultCarryMsg, LutId(4)),
        (Lut1Def::MultCarryMsgLsb, LutId(5)),
        (Lut1Def::MultCarryMsgMsb, LutId(6)),
        (Lut1Def::BwAnd, LutId(7)),
        (Lut1Def::BwOr, LutId(8)),
        (Lut1Def::BwXor, LutId(9)),
        (Lut1Def::CmpSign, LutId(10)),
        (Lut1Def::CmpReduce, LutId(11)),
        (Lut1Def::CmpGt, LutId(12)),
        (Lut1Def::CmpGte, LutId(13)),
        (Lut1Def::CmpLt, LutId(14)),
        (Lut1Def::CmpLte, LutId(15)),
        (Lut1Def::CmpEq, LutId(16)),
        (Lut1Def::CmpNeq, LutId(17)),
        (Lut1Def::ReduceCarry2, LutId(19)),
        (Lut1Def::ReduceCarry3, LutId(20)),
        (Lut1Def::ReduceCarryPad, LutId(21)),
        (Lut1Def::GenPropAdd, LutId(22)),
        (Lut1Def::IfTrueZeroed, LutId(23)),
        (Lut1Def::IfFalseZeroed, LutId(24)),
        (Lut1Def::Ripple2GenProp, LutId(25)),
        (Lut1Def::CmpGtMrg, LutId(27)),
        (Lut1Def::CmpGteMrg, LutId(28)),
        (Lut1Def::CmpLtMrg, LutId(29)),
        (Lut1Def::CmpLteMrg, LutId(30)),
        (Lut1Def::CmpEqMrg, LutId(31)),
        (Lut1Def::CmpNeqMrg, LutId(32)),
        (Lut1Def::IsSome, LutId(33)),
        (Lut1Def::CarryIsSome, LutId(34)),
        (Lut1Def::CarryIsNone, LutId(35)),
        (Lut1Def::MultCarryMsgIsSome, LutId(36)),
        (Lut1Def::MultCarryMsgMsbIsSome, LutId(37)),
        (Lut1Def::IsNull, LutId(38)),
        (Lut1Def::IsNullPos1, LutId(39)),
        (Lut1Def::NotNull, LutId(40)),
        (Lut1Def::MsgNotNull, LutId(41)),
        (Lut1Def::MsgNotNullPos1, LutId(42)),
        (Lut1Def::SolvePropGroupFinal0, LutId(44)),
        (Lut1Def::SolvePropGroupFinal1, LutId(45)),
        (Lut1Def::SolvePropGroupFinal2, LutId(46)),
        (Lut1Def::ExtractPropGroup0, LutId(47)),
        (Lut1Def::ExtractPropGroup1, LutId(48)),
        (Lut1Def::ExtractPropGroup2, LutId(49)),
        (Lut1Def::ExtractPropGroup3, LutId(50)),
        (Lut1Def::SolveProp, LutId(51)),
        (Lut1Def::SolvePropCarry, LutId(52)),
        (Lut1Def::SolveQuotient, LutId(53)),
        (Lut1Def::SolveQuotientPos1, LutId(54)),
        (Lut1Def::IfPos1FalseZeroed, LutId(55)),
        (Lut1Def::IfPos1FalseZeroedMsgCarry1, LutId(56)),
        (Lut1Def::ShiftLeftByCarryPos0Msg, LutId(57)),
        (Lut1Def::ShiftLeftByCarryPos0MsgNext, LutId(58)),
        (Lut1Def::ShiftRightByCarryPos0Msg, LutId(59)),
        (Lut1Def::ShiftRightByCarryPos0MsgNext, LutId(60)),
        (Lut1Def::IfPos0TrueZeroed, LutId(61)),
        (Lut1Def::IfPos0FalseZeroed, LutId(62)),
        (Lut1Def::IfPos1TrueZeroed, LutId(63)),
    ])
});

pub(crate) static GIDS2: LazyLock<FastMap<Lut2Def, LutId>> = LazyLock::new(|| {
    HashMap::from([
        (Lut2Def::ManyGenProp, LutId(18)),
        (Lut2Def::ManyCarryMsg, LutId(26)),
        (Lut2Def::ManyMsgSplitShift1, LutId(43)),
        (Lut2Def::ManyInv1CarryMsg, LutId(64)),
        (Lut2Def::ManyInv2CarryMsg, LutId(65)),
        (Lut2Def::ManyInv3CarryMsg, LutId(66)),
        (Lut2Def::ManyInv4CarryMsg, LutId(67)),
        (Lut2Def::ManyInv5CarryMsg, LutId(68)),
        (Lut2Def::ManyInv6CarryMsg, LutId(69)),
        (Lut2Def::ManyInv7CarryMsg, LutId(70)),
        (Lut2Def::ManyMsgSplit, LutId(71)),
        (Lut2Def::Manym2lPropBit1MsgSplit, LutId(72)),
        (Lut2Def::Manym2lPropBit0MsgSplit, LutId(73)),
        (Lut2Def::Manyl2mPropBit1MsgSplit, LutId(74)),
        (Lut2Def::Manyl2mPropBit0MsgSplit, LutId(75)),
    ])
});

pub(crate) static GIDS4: LazyLock<FastMap<Lut4Def, LutId>> = LazyLock::new(|| HashMap::from([]));

pub(crate) static GIDS8: LazyLock<FastMap<Lut8Def, LutId>> = LazyLock::new(|| HashMap::from([]));

pub fn lower_iop_to_hpu(ir: &IR<IopLang>) -> IR<HpuLang> {
    let ann_ir = ir
        .forward_dataflow_analysis(|a| {
            use IopInstructionSet::*;
            let opann = match a.get_instruction() {
                InputCiphertext { pos, .. } | InputPlaintext { pos, .. } => Some(pos),
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
            use IopInstructionSet::*;
            let opann = match a.get_instruction() {
                OutputCiphertext { pos, .. } => Some(pos),
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
    eager_translate_ann(&ann_ir, |op, translator| {
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
                    op,
                    HpuInstructionSet::CstCt {
                        cst: Immediate(value),
                    },
                );
            }
            IopInstructionSet::AddCt
            | IopInstructionSet::WrappingAddCt
            | IopInstructionSet::TemperAddCt => {
                translator.direct_translation(op, HpuInstructionSet::AddCt);
            }
            IopInstructionSet::SubCt => {
                translator.direct_translation(op, HpuInstructionSet::SubCt);
            }
            IopInstructionSet::PackCt { mul } => {
                translator.direct_translation(
                    op,
                    HpuInstructionSet::Mac {
                        cst: Immediate(mul as u8),
                    },
                );
            }
            IopInstructionSet::AddPt | IopInstructionSet::WrappingAddPt => {
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
                                cst: Immediate(value as u8),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(op, HpuInstructionSet::AddPt);
                    }
                }
            }
            IopInstructionSet::SubPt => {
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
                                cst: Immediate(value as u8),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(op, HpuInstructionSet::SubPt);
                    }
                }
            }
            IopInstructionSet::PtSub => {
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
                                cst: Immediate(value as u8),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[1])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(op, HpuInstructionSet::PtSub);
                    }
                }
            }
            IopInstructionSet::MulPt => {
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
                                cst: Immediate(value as u8),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(op, HpuInstructionSet::MulPt);
                    }
                }
            }
            IopInstructionSet::ExtractCtBlock { index } => {
                let new_rets = translator.add_op(
                    HpuInstructionSet::SrcLd {
                        from: TSrcId {
                            src_pos: op.get_annotation().unwrap().try_into().unwrap(),
                            block_pos: index.try_into().unwrap(),
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
                            block_pos: index.try_into().unwrap(),
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
                            block_pos: index.try_into().unwrap(),
                        },
                    },
                    svec![new_arg],
                );
            }
            IopInstructionSet::Pbs { lut, .. } => {
                let lut = match GIDS1.get(&lut) {
                    Some(v) => *v,
                    None => panic!("Failed to lookup the gid for key: {lut:?}"),
                };
                translator.direct_translation(op, HpuInstructionSet::Pbs { lut });
            }
            IopInstructionSet::Pbs2 { lut } => {
                let lut = match GIDS2.get(&lut) {
                    Some(v) => *v,
                    None => panic!("Failed to lookup the gid for key: {lut:?}"),
                };
                translator.direct_translation(op, HpuInstructionSet::Pbs2 { lut });
            }
            IopInstructionSet::Pbs4 { lut } => {
                let lut = match GIDS4.get(&lut) {
                    Some(v) => *v,
                    None => panic!("Failed to lookup the gid for key: {lut:?}"),
                };
                translator.direct_translation(op, HpuInstructionSet::Pbs4 { lut });
            }
            IopInstructionSet::Pbs8 { lut } => {
                let lut = match GIDS8.get(&lut) {
                    Some(v) => *v,
                    None => panic!("Failed to lookup the gid for key: {lut:?}"),
                };
                translator.direct_translation(op, HpuInstructionSet::Pbs8 { lut });
            }
        }
    })
}

#[cfg(test)]
mod test {
    use zhc_builder::{
        Builder, CiphertextSpec, add, bitwise_and, bitwise_or, bitwise_xor, cmp_gt, if_then_else,
        if_then_zero, mul_lsb,
    };
    use zhc_ir::IR;
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_utils::assert_display_is;

    use crate::{test::check_iop_hpu_equivalence, translation::lower_iop_to_hpu};

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        lower_iop_to_hpu(&ir)
    }

    #[test]
    fn test_translate_add_ir() {
        let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).into_ir());
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
                %24, %25 = pbs_2<Lut@26>(%16);
                %26 = pbs<Lut@47>(%17);
                %27 = pbs<Lut@48>(%18);
                %28 = pbs<Lut@49>(%19);
                %29 = pbs<Lut@47>(%20);
                %30 = pbs<Lut@48>(%21);
                %31 = pbs<Lut@49>(%22);
                %32 = add_ct(%25, %26);
                %33 = add_ct(%32, %27);
                %34 = add_ct(%33, %28);
                %35 = pbs<Lut@46>(%34);
                %36 = add_ct(%29, %30);
                %37 = add_ct(%36, %31);
                %38 = pbs<Lut@44>(%32);
                %39 = pbs<Lut@45>(%33);
                %40 = add_ct(%29, %35);
                %41 = pbs<Lut@44>(%40);
                %42 = add_ct(%36, %35);
                %43 = pbs<Lut@45>(%42);
                %44 = add_ct(%37, %35);
                %45 = pbs<Lut@46>(%44);
                %46 = add_ct(%17, %25);
                %47 = add_ct(%18, %38);
                %48 = add_ct(%19, %39);
                %49 = add_ct(%20, %35);
                %50 = add_ct(%21, %41);
                %51 = add_ct(%22, %43);
                %52 = add_ct(%23, %45);
                %53 = pbs<Lut@1>(%24);
                %54 = pbs<Lut@1>(%46);
                %55 = pbs<Lut@1>(%47);
                %56 = pbs<Lut@1>(%48);
                %57 = pbs<Lut@1>(%49);
                %58 = pbs<Lut@1>(%50);
                %59 = pbs<Lut@1>(%51);
                %60 = pbs<Lut@1>(%52);
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
        let ir = pipeline(&cmp_gt(CiphertextSpec::new(16, 2, 2)).into_ir());
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
                %17 = pbs<Lut@0>(%16);
                %18 = mac<4_imm>(%3, %2);
                %19 = pbs<Lut@0>(%18);
                %20 = mac<4_imm>(%5, %4);
                %21 = pbs<Lut@0>(%20);
                %22 = mac<4_imm>(%7, %6);
                %23 = pbs<Lut@0>(%22);
                %24 = mac<4_imm>(%9, %8);
                %25 = pbs<Lut@0>(%24);
                %26 = mac<4_imm>(%11, %10);
                %27 = pbs<Lut@0>(%26);
                %28 = mac<4_imm>(%13, %12);
                %29 = pbs<Lut@0>(%28);
                %30 = mac<4_imm>(%15, %14);
                %31 = pbs<Lut@0>(%30);
                %32 = sub_ct(%17, %25);
                %33 = pbs<Lut@10>(%32);
                %34 = add_cst<1_imm>(%33);
                %35 = sub_ct(%19, %27);
                %36 = pbs<Lut@10>(%35);
                %37 = add_cst<1_imm>(%36);
                %38 = sub_ct(%21, %29);
                %39 = pbs<Lut@10>(%38);
                %40 = add_cst<1_imm>(%39);
                %41 = sub_ct(%23, %31);
                %42 = pbs<Lut@10>(%41);
                %43 = add_cst<1_imm>(%42);
                %44 = mac<4_imm>(%37, %34);
                %45 = mac<4_imm>(%43, %40);
                %46 = pbs<Lut@11>(%44);
                %47 = pbs<Lut@11>(%45);
                %48 = mac<4_imm>(%47, %46);
                %49 = pbs<Lut@27>(%48);
                dst_st<0.0_tdst>(%49);
            "#
        );
    }

    #[test]
    fn correctness() {
        let check = |b: Builder| {
            let spec = *b.spec();
            let iop_ir = b.into_ir();
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
            check(mul_lsb(spec));
        }
    }
}
