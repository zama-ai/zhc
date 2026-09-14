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

/// Lowers integer operations to HPU operations, returning the IR and translation maps.
pub fn lower_iop_to_hpu(ir: &IR<IopLang>) -> Translation<HpuLang> {
    use IopInstructionSet::*;
    let remap = ir
        .walk_ops_linear()
        .filter(|op| {
            matches!(
                op.get_instruction(),
                InputIntegerCiphertext { .. }
                    | InputBoolCiphertext { .. }
                    | InputIntegerPlaintext { .. }
            )
        })
        .scan((0, 0), |(ct_id, pt_id), op| match op.get_instruction() {
            InputIntegerCiphertext { pos, .. } | InputBoolCiphertext { pos } => {
                *ct_id += 1;
                Some((*pos, *ct_id - 1))
            }
            InputIntegerPlaintext { pos, .. } => {
                *pt_id += 1;
                Some((*pos, *pt_id - 1))
            }
            _ => unreachable!(),
        })
        .collect::<SmallMap<usize, usize>>();
    let ann_ir = ir
        .forward_dataflow_analysis(|a| {
            let opann = match a.get_instruction() {
                InputIntegerCiphertext { pos, .. }
                | InputBoolCiphertext { pos }
                | InputIntegerPlaintext { pos, .. } => Some(*remap.get(&pos).unwrap()),
                ExtractIntegerCiphertextBlock { .. } | ExtractIntegerPlaintextBlock { .. } => a
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
                OutputIntegerCiphertext { pos, .. } => Some(*pos),
                StoreIntegerCiphertextBlock { .. } => {
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
            IopInstructionSet::InputIntegerCiphertext { .. }
            | IopInstructionSet::InputIntegerPlaintext { .. }
            | IopInstructionSet::LetPlaintextBlock { .. } => {
                // Handled in consumers.
            }
            IopInstructionSet::OutputIntegerCiphertext { .. } => {
                // No-op
            }
            IopInstructionSet::InputBoolCiphertext { .. } => {
                let new_rets = translator.add_op(
                    HpuInstructionSet::SrcLd {
                        from: TSrcId {
                            src_pos: op.get_annotation().unwrap().try_into().unwrap(),
                            block_pos: 0,
                        },
                    },
                    svec![],
                );
                translator.register_translation(op.get_return_valids()[0], new_rets[0]);
            }
            IopInstructionSet::OutputBoolCiphertext { pos } => {
                translator.add_op(
                    HpuInstructionSet::DstSt {
                        to: TDstId {
                            dst_pos: (*pos).sas(),
                            block_pos: 0,
                        },
                    },
                    svec![translator.translate_val(op.get_arg_valids()[0])],
                );
            }
            IopInstructionSet::BoolFromBlock | IopInstructionSet::ExtractBoolBlock => {
                translator.register_translation(
                    op.get_return_valids()[0],
                    translator.translate_val(op.get_arg_valids()[0]),
                );
            }
            IopInstructionSet::DeclareIntegerCiphertext { .. } => {
                // DeclareIntegerCiphertext has no semantics in hpulang.
                // We just verify that it is not used in an unexpected way.
                assert!(
                    op.get_reached_iter().all(|reached| matches!(
                        reached.get_instruction(),
                        IopInstructionSet::StoreIntegerCiphertextBlock { .. }
                            | IopInstructionSet::OutputIntegerCiphertext { .. }
                    )),
                    "Unexpectd use of DeclareIntegerCiphertext encountered."
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
            IopInstructionSet::ExtractIntegerCiphertextBlock { index } => {
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
            IopInstructionSet::ExtractIntegerPlaintextBlock { index } => {
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
            IopInstructionSet::StoreIntegerCiphertextBlock { index } => {
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
