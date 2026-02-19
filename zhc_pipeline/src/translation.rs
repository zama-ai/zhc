//! Translation from IOP language to HPU language.
//!
//! This module provides translation capabilities that convert intermediate
//! representations from the integer operation language (IOP) to the HPU
//! hardware language. The translation maps high-level operations to
//! low-level hardware primitives while preserving semantic correctness.

use std::{collections::HashMap, sync::LazyLock};

use zhc_ir::{IR, OpId, ValId, translation::Translator};
use zhc_langs::{
    hpulang::{HpuInstructionSet, HpuLang, Immediate, LutId, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopLang, IopTypeSystem, Lut1Def, Lut2Def, Lut4Def, Lut8Def},
};
use zhc_utils::{FastMap, svec};

static GIDS1: LazyLock<FastMap<Lut1Def, LutId>> = LazyLock::new(|| {
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

static GIDS2: LazyLock<FastMap<Lut2Def, LutId>> = LazyLock::new(|| {
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

static GIDS4: LazyLock<FastMap<Lut4Def, LutId>> = LazyLock::new(|| HashMap::from([]));

static GIDS8: LazyLock<FastMap<Lut8Def, LutId>> = LazyLock::new(|| HashMap::from([]));

/// Translator from IOP language to HPU language intermediate representations.
pub struct IoplangToHpulang;

impl Translator for IoplangToHpulang {
    type InputDialect = IopLang;
    type OutputDialect = HpuLang;

    fn translate(
        &mut self,
        input: &zhc_ir::IR<Self::InputDialect>,
    ) -> zhc_ir::IR<Self::OutputDialect> {
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
                    op.get_instruction(),
                    IopInstructionSet::Output {
                        typ: IopTypeSystem::Ciphertext,
                        ..
                    }
                )
            })
            .map(|oup_op| {
                // For the output, we search the let reaching this output.
                let let_pred = oup_op
                    .get_inc_reaching_iter()
                    .find(|pr| matches!(pr.get_instruction(), IopInstructionSet::DeclareCiphertext))
                    .expect("Failed to find the declaration predecessor of an `output` op.");
                let IopInstructionSet::Output { pos, .. } = oup_op.get_instruction() else {
                    unreachable!()
                };
                (let_pred.get_id(), pos)
            })
            .collect();

        for op in input.walk_ops_topological() {
            match op.get_instruction() {
                IopInstructionSet::Input { .. } | IopInstructionSet::LetPlaintextBlock { .. } => {
                    // Handled in consumers.
                }
                IopInstructionSet::Output { .. } => {
                    // No-op
                }
                IopInstructionSet::DeclareCiphertext => {
                    // DeclareCiphertext has no semantics in hpulang.
                    // We just verify that it is not used in an unexpected way.
                    assert!(
                        op.get_reached_iter().all(|reached| matches!(
                            reached.get_instruction(),
                            IopInstructionSet::StoreCtBlock { .. }
                                | IopInstructionSet::Output { .. }
                        )),
                        "Unexpectd use of DeclareCiphertext encountered."
                    )
                }
                IopInstructionSet::Alias { .. } => {
                    // Aliases have no semantics in hpulang. And they may prevent CSE so there
                    // should be no aliases remaining here,
                    panic!("Unexpected Alias op encountered.");
                }
                IopInstructionSet::LetCiphertextBlock { value } => {
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::CstCt {
                                cst: Immediate(value),
                            },
                            svec![],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::AddCt
                | IopInstructionSet::WrappingAddCt
                | IopInstructionSet::TemperAddCt => {
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::AddCt,
                            svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::SubCt => {
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::SubCt,
                            svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::PackCt { mul } => {
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::Mac {
                                cst: Immediate(mul as u8),
                            },
                            svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::AddPt | IopInstructionSet::WrappingAddPt => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[1]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuInstructionSet::AddPt,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopInstructionSet::LetPlaintextBlock { value: cst } = op
                            .get_args_iter()
                            .nth(1)
                            .unwrap()
                            .get_origin()
                            .opref
                            .get_instruction()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuInstructionSet::AddCst {
                                    cst: Immediate(cst as u8),
                                },
                                svec![map[op.get_arg_valids()[0]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::SubPt => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[1]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuInstructionSet::SubPt,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopInstructionSet::LetPlaintextBlock { value: cst } = op
                            .get_args_iter()
                            .nth(1)
                            .unwrap()
                            .get_origin()
                            .opref
                            .get_instruction()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuInstructionSet::SubCst {
                                    cst: Immediate(cst as u8),
                                },
                                svec![map[op.get_arg_valids()[0]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::PtSub => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[0]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuInstructionSet::PtSub,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopInstructionSet::LetPlaintextBlock { value: cst } = op
                            .get_args_iter()
                            .nth(0)
                            .unwrap()
                            .get_origin()
                            .opref
                            .get_instruction()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuInstructionSet::CstSub {
                                    cst: Immediate(cst as u8),
                                },
                                svec![map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::MulPt => {
                    let (_, valids) = if map.contains_key(&op.get_arg_valids()[1]) {
                        // The plaintext input is not constant.
                        output
                            .add_op(
                                HpuInstructionSet::MulPt,
                                svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                            )
                            .unwrap()
                    } else {
                        // The plaintext input is constant.
                        let IopInstructionSet::LetPlaintextBlock { value: cst } = op
                            .get_args_iter()
                            .nth(1)
                            .unwrap()
                            .get_origin()
                            .opref
                            .get_instruction()
                        else {
                            unreachable!()
                        };
                        output
                            .add_op(
                                HpuInstructionSet::MulCst {
                                    cst: Immediate(cst as u8),
                                },
                                svec![map[op.get_arg_valids()[0]]],
                            )
                            .unwrap()
                    };
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::ExtractCtBlock { index } => {
                    let src_pos = op
                        .get_args_iter()
                        .nth(0) // ct arg.
                        .unwrap()
                        .get_origin()
                        .opref
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_instruction() {
                            IopInstructionSet::Input {
                                typ: IopTypeSystem::Ciphertext,
                                pos,
                            } => Some(pos),
                            _ => None,
                        })
                        .unwrap();
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::SrcLd {
                                from: TSrcId {
                                    src_pos: src_pos.try_into().unwrap(),
                                    block_pos: index.try_into().unwrap(),
                                },
                            },
                            svec![],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::ExtractPtBlock { index } => {
                    let imm_pos = op
                        .get_args_iter()
                        .nth(0) // pt arg.
                        .unwrap()
                        .get_origin()
                        .opref
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_instruction() {
                            IopInstructionSet::Input {
                                typ: IopTypeSystem::Plaintext,
                                pos,
                            } => Some(pos),
                            _ => None,
                        })
                        .unwrap();
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::ImmLd {
                                from: TImmId {
                                    imm_pos: imm_pos.try_into().unwrap(),
                                    block_pos: index.try_into().unwrap(),
                                },
                            },
                            svec![],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::StoreCtBlock { index } => {
                    let dst_pos = op
                        .get_args_iter()
                        .nth(1) // ct arg.
                        .unwrap()
                        .get_origin()
                        .opref
                        .get_inc_reaching_iter()
                        .find_map(|op| match op.get_instruction() {
                            IopInstructionSet::DeclareCiphertext => {
                                let_map.get(&op.get_id()).cloned()
                            }
                            _ => None,
                        })
                        .unwrap();
                    output
                        .add_op(
                            HpuInstructionSet::DstSt {
                                to: TDstId {
                                    dst_pos: dst_pos.try_into().unwrap(),
                                    block_pos: index.try_into().unwrap(),
                                },
                            },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                }
                IopInstructionSet::Pbs { lut, .. } => {
                    let lut = match GIDS1.get(&lut) {
                        Some(v) => *v,
                        None => panic!("Failed to lookup the gid for key: {lut:?}"),
                    };
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::Pbs { lut },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                }
                IopInstructionSet::Pbs2 { lut } => {
                    let lut = match GIDS2.get(&lut) {
                        Some(v) => *v,
                        None => panic!("Failed to lookup the gid for key: {lut:?}"),
                    };
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::Pbs2 { lut },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                    map.insert(op.get_return_valids()[1], valids[1]);
                }
                IopInstructionSet::Pbs4 { lut } => {
                    let lut = match GIDS4.get(&lut) {
                        Some(v) => *v,
                        None => panic!("Failed to lookup the gid for key: {lut:?}"),
                    };
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::Pbs4 { lut },
                            svec![map[op.get_arg_valids()[0]]],
                        )
                        .unwrap();
                    map.insert(op.get_return_valids()[0], valids[0]);
                    map.insert(op.get_return_valids()[1], valids[1]);
                    map.insert(op.get_return_valids()[2], valids[2]);
                    map.insert(op.get_return_valids()[3], valids[3]);
                }
                IopInstructionSet::Pbs8 { lut } => {
                    let lut = match GIDS8.get(&lut) {
                        Some(v) => *v,
                        None => panic!("Failed to lookup the gid for key: {lut:?}"),
                    };
                    let (_, valids) = output
                        .add_op(
                            HpuInstructionSet::Pbs8 { lut },
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
    use zhc_ir::{IR, translation::Translator};
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_utils::assert_display_is;

    use crate::test::{get_add_ir, get_cmp_ir};

    use super::IoplangToHpulang;

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        IoplangToHpulang.translate(&ir)
    }

    #[test]
    fn test_translate_add_ir() {
        let ir = pipeline(&get_add_ir(16, 2, 2));
        assert_display_is!(
            ir.format(),
            r#"
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
                %16 : CtRegister = add_ct(%0 : CtRegister, %8 : CtRegister);
                %17 : CtRegister = add_ct(%1 : CtRegister, %9 : CtRegister);
                %18 : CtRegister = add_ct(%2 : CtRegister, %10 : CtRegister);
                %19 : CtRegister = add_ct(%3 : CtRegister, %11 : CtRegister);
                %20 : CtRegister = add_ct(%4 : CtRegister, %12 : CtRegister);
                %21 : CtRegister = add_ct(%5 : CtRegister, %13 : CtRegister);
                %22 : CtRegister = add_ct(%6 : CtRegister, %14 : CtRegister);
                %23 : CtRegister = add_ct(%7 : CtRegister, %15 : CtRegister);
                %24 : CtRegister, %25 : CtRegister = pbs_2<Lut@26>(%16 : CtRegister);
                %26 : CtRegister = pbs<Lut@47>(%17 : CtRegister);
                %27 : CtRegister = pbs<Lut@48>(%18 : CtRegister);
                %28 : CtRegister = pbs<Lut@49>(%19 : CtRegister);
                %29 : CtRegister = pbs<Lut@47>(%20 : CtRegister);
                %30 : CtRegister = pbs<Lut@48>(%21 : CtRegister);
                %31 : CtRegister = pbs<Lut@49>(%22 : CtRegister);
                %32 : CtRegister = add_ct(%25 : CtRegister, %26 : CtRegister);
                %33 : CtRegister = add_ct(%29 : CtRegister, %30 : CtRegister);
                %34 : CtRegister = add_ct(%17 : CtRegister, %25 : CtRegister);
                %35 : CtRegister = pbs<Lut@1>(%24 : CtRegister);
                %36 : CtRegister = add_ct(%32 : CtRegister, %27 : CtRegister);
                %37 : CtRegister = add_ct(%33 : CtRegister, %31 : CtRegister);
                %38 : CtRegister = pbs<Lut@44>(%32 : CtRegister);
                %39 : CtRegister = pbs<Lut@1>(%34 : CtRegister);
                dst_st<0.0_tdst>(%35 : CtRegister);
                %40 : CtRegister = add_ct(%36 : CtRegister, %28 : CtRegister);
                %41 : CtRegister = pbs<Lut@45>(%36 : CtRegister);
                %42 : CtRegister = add_ct(%18 : CtRegister, %38 : CtRegister);
                dst_st<0.1_tdst>(%39 : CtRegister);
                %43 : CtRegister = pbs<Lut@46>(%40 : CtRegister);
                %44 : CtRegister = add_ct(%19 : CtRegister, %41 : CtRegister);
                %45 : CtRegister = pbs<Lut@1>(%42 : CtRegister);
                %46 : CtRegister = add_ct(%29 : CtRegister, %43 : CtRegister);
                %47 : CtRegister = add_ct(%33 : CtRegister, %43 : CtRegister);
                %48 : CtRegister = add_ct(%37 : CtRegister, %43 : CtRegister);
                %49 : CtRegister = add_ct(%20 : CtRegister, %43 : CtRegister);
                %50 : CtRegister = pbs<Lut@1>(%44 : CtRegister);
                dst_st<0.2_tdst>(%45 : CtRegister);
                %51 : CtRegister = pbs<Lut@44>(%46 : CtRegister);
                %52 : CtRegister = pbs<Lut@45>(%47 : CtRegister);
                %53 : CtRegister = pbs<Lut@46>(%48 : CtRegister);
                %54 : CtRegister = pbs<Lut@1>(%49 : CtRegister);
                dst_st<0.3_tdst>(%50 : CtRegister);
                %55 : CtRegister = add_ct(%21 : CtRegister, %51 : CtRegister);
                %56 : CtRegister = add_ct(%22 : CtRegister, %52 : CtRegister);
                %57 : CtRegister = add_ct(%23 : CtRegister, %53 : CtRegister);
                dst_st<0.4_tdst>(%54 : CtRegister);
                %58 : CtRegister = pbs<Lut@1>(%55 : CtRegister);
                %59 : CtRegister = pbs<Lut@1>(%56 : CtRegister);
                %60 : CtRegister = pbs<Lut@1>(%57 : CtRegister);
                dst_st<0.5_tdst>(%58 : CtRegister);
                dst_st<0.6_tdst>(%59 : CtRegister);
                dst_st<0.7_tdst>(%60 : CtRegister);
            "#
        );
    }

    #[test]
    fn test_translate_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        assert_display_is!(
            ir.format(),
            r#"
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
            %16 : CtRegister = mac<4_imm>(%1 : CtRegister, %0 : CtRegister);
            %17 : CtRegister = mac<4_imm>(%3 : CtRegister, %2 : CtRegister);
            %18 : CtRegister = mac<4_imm>(%5 : CtRegister, %4 : CtRegister);
            %19 : CtRegister = mac<4_imm>(%7 : CtRegister, %6 : CtRegister);
            %20 : CtRegister = mac<4_imm>(%9 : CtRegister, %8 : CtRegister);
            %21 : CtRegister = mac<4_imm>(%11 : CtRegister, %10 : CtRegister);
            %22 : CtRegister = mac<4_imm>(%13 : CtRegister, %12 : CtRegister);
            %23 : CtRegister = mac<4_imm>(%15 : CtRegister, %14 : CtRegister);
            %24 : CtRegister = pbs<Lut@0>(%16 : CtRegister);
            %25 : CtRegister = pbs<Lut@0>(%17 : CtRegister);
            %26 : CtRegister = pbs<Lut@0>(%18 : CtRegister);
            %27 : CtRegister = pbs<Lut@0>(%19 : CtRegister);
            %28 : CtRegister = pbs<Lut@0>(%20 : CtRegister);
            %29 : CtRegister = pbs<Lut@0>(%21 : CtRegister);
            %30 : CtRegister = pbs<Lut@0>(%22 : CtRegister);
            %31 : CtRegister = pbs<Lut@0>(%23 : CtRegister);
            %32 : CtRegister = sub_ct(%24 : CtRegister, %28 : CtRegister);
            %33 : CtRegister = sub_ct(%25 : CtRegister, %29 : CtRegister);
            %34 : CtRegister = sub_ct(%26 : CtRegister, %30 : CtRegister);
            %35 : CtRegister = sub_ct(%27 : CtRegister, %31 : CtRegister);
            %36 : CtRegister = pbs<Lut@10>(%32 : CtRegister);
            %37 : CtRegister = pbs<Lut@10>(%33 : CtRegister);
            %38 : CtRegister = pbs<Lut@10>(%34 : CtRegister);
            %39 : CtRegister = pbs<Lut@10>(%35 : CtRegister);
            %40 : CtRegister = add_cst<1_imm>(%36 : CtRegister);
            %41 : CtRegister = add_cst<1_imm>(%37 : CtRegister);
            %42 : CtRegister = add_cst<1_imm>(%38 : CtRegister);
            %43 : CtRegister = add_cst<1_imm>(%39 : CtRegister);
            %44 : CtRegister = mac<4_imm>(%41 : CtRegister, %40 : CtRegister);
            %45 : CtRegister = mac<4_imm>(%43 : CtRegister, %42 : CtRegister);
            %46 : CtRegister = pbs<Lut@11>(%44 : CtRegister);
            %47 : CtRegister = pbs<Lut@11>(%45 : CtRegister);
            %48 : CtRegister = mac<4_imm>(%47 : CtRegister, %46 : CtRegister);
            %49 : CtRegister = pbs<Lut@27>(%48 : CtRegister);
            dst_st<0.0_tdst>(%49 : CtRegister);
        "#
        );
    }
}
