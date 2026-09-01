use zhc_ir::{
    IR,
    translation::{Order, translate_ann},
};
use zhc_langs::{
    ioplang::{IopInstructionSet, IopLang},
    vmlang::{VmInstructionSet, VmLang},
};
use zhc_utils::{SafeAs, small::SmallMap, svec};

pub fn lower_iop_to_vm(ir: &IR<IopLang>) -> IR<VmLang> {
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
                translator.direct_translation(&op, VmInstructionSet::CstCt { cst: *value });
            }
            IopInstructionSet::AddCt
            | IopInstructionSet::WrappingAddCt
            | IopInstructionSet::TemperAddCt => {
                translator.direct_translation(&op, VmInstructionSet::AddCt);
            }
            IopInstructionSet::SubCt | IopInstructionSet::WrappingSubCt => {
                translator.direct_translation(&op, VmInstructionSet::SubCt);
            }
            IopInstructionSet::PackCt { mul } => {
                translator.direct_translation(&op, VmInstructionSet::Mac { cst: (*mul).sas() });
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
                            VmInstructionSet::AddCst {
                                cst: (*value).sas(),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, VmInstructionSet::AddPt);
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
                            VmInstructionSet::SubCst {
                                cst: (*value).sas(),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, VmInstructionSet::SubPt);
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
                            VmInstructionSet::CstSub {
                                cst: (*value).sas(),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[1])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, VmInstructionSet::PtSub);
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
                            VmInstructionSet::MulCst {
                                cst: (*value).sas(),
                            },
                            svec![translator.translate_val(op.get_arg_valids()[0])],
                        );
                        translator.register_translation(op.get_return_valids()[0], new_rets[0]);
                    }
                    _ => {
                        translator.direct_translation(&op, VmInstructionSet::MulPt);
                    }
                }
            }
            IopInstructionSet::ExtractCtBlock { index } => {
                let new_rets = translator.add_op(
                    VmInstructionSet::SrcLd {
                        from_pos: op.get_annotation().unwrap().try_into().unwrap(),
                        from_block: (*index).sas(),
                    },
                    svec![],
                );
                translator.register_translation(op.get_return_valids()[0], new_rets[0]);
            }
            IopInstructionSet::ExtractPtBlock { index } => {
                let new_rets = translator.add_op(
                    VmInstructionSet::ImmLd {
                        from_pos: op.get_annotation().unwrap().try_into().unwrap(),
                        from_block: (*index).sas(),
                    },
                    svec![],
                );
                translator.register_translation(op.get_return_valids()[0], new_rets[0]);
            }
            IopInstructionSet::StoreCtBlock { index } => {
                let new_arg = translator.translate_val(op.get_arg_valids()[0]);
                translator.add_op(
                    VmInstructionSet::DstSt {
                        to_pos: op.get_annotation().unwrap().try_into().unwrap(),
                        to_block: (*index).sas(),
                    },
                    svec![new_arg],
                );
            }
            IopInstructionSet::Pbs { lut, .. } => {
                let new_arg = translator.translate_val(op.get_arg_valids()[0]);
                let rets = translator.add_op(VmInstructionSet::Ks, svec![new_arg]);
                let rets = translator.add_op(VmInstructionSet::Pbs { lut: lut.clone() }, rets);
                translator.register_translation(op.get_return_valids()[0], rets[0]);
            }
            IopInstructionSet::Pbs2 { lut, .. } => {
                let new_arg = translator.translate_val(op.get_arg_valids()[0]);
                let rets = translator.add_op(VmInstructionSet::Ks, svec![new_arg]);
                let rets = translator.add_op(VmInstructionSet::Pbs2 { lut: lut.clone() }, rets);
                translator.register_translation(op.get_return_valids()[0], rets[0]);
                translator.register_translation(op.get_return_valids()[1], rets[1]);
            }
        }
    })
    .output
}
