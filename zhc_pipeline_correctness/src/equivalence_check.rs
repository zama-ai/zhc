use zhc_builder::CiphertextBlockSpec;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::{IR, evaluation::EffectfulEvaluator};
use zhc_langs::{
    doplang::{DopInterpreterContext, DopLang, DopValue},
    hpulang::{HpuInterpreterContext, HpuLang, HpuValue, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopInterepreterContext, IopLang, IopValue},
};
use zhc_utils::{Dumpable, FastMap, SafeAs};

pub fn check_iop_equivalence(
    reference_ir: &IR<IopLang>,
    candidate_ir: &IR<IopLang>,
    spec: CiphertextBlockSpec,
    nreps: usize,
) {
    let mut input_slots = reference_ir
        .walk_ops_linear()
        .filter_map(|op| match op.get_instruction() {
            IopInstructionSet::InputCiphertext { pos, int_size } => Some((*pos, true, *int_size)),
            IopInstructionSet::InputPlaintext { pos, int_size } => Some((*pos, false, *int_size)),
            _ => None,
        })
        .collect::<Vec<_>>();
    input_slots.sort_by_key(|(pos, _, _)| *pos);

    for _ in 0..nreps {
        let inputs: FastMap<usize, IopValue> = input_slots
            .iter()
            .map(|(pos, is_ct, int_size)| {
                let value = if *is_ct {
                    IopValue::Ciphertext(spec.ciphertext_spec(*int_size).random())
                } else {
                    IopValue::Plaintext(
                        spec.matching_plaintext_block_spec()
                            .plaintext_spec(*int_size)
                            .random(),
                    )
                };
                (*pos, value)
            })
            .collect();

        let evaluate = |ir: &IR<IopLang>| {
            let mut context = IopInterepreterContext {
                spec,
                inputs: inputs.clone(),
                outputs: FastMap::default(),
            };
            if let Err(eval_ir) = ir.evaluate::<IopValue>(&mut context) {
                eval_ir.dump_and_panic();
            }
            context.outputs
        };

        assert_eq!(
            evaluate(candidate_ir),
            evaluate(reference_ir),
            "IOP outputs differ"
        );
    }
}

pub fn check_iop_hpu_equivalence(
    iop_ir: &IR<IopLang>,
    hpu_ir: &IR<HpuLang>,
    spec: CiphertextBlockSpec,
    nreps: usize,
) {
    check_iop_hpu_equivalence_with(iop_ir, hpu_ir, spec, nreps, |ir, ctx| {
        if let Err(eval_ir) = ir.evaluate::<HpuValue>(ctx) {
            eval_ir.dump_and_panic();
        }
    });
}

/// Checks a multi-HPU program against its IOP source.
///
/// The per-HPU programs are concatenated into one IR and interpreted with an
/// [`EffectfulEvaluator`], so `TransferOut` and `TransferIn` pairs exchange
/// blocks through the context mailbox. Panics if some transfers can never
/// complete (a deadlock), if a posted block is never taken, if an operation
/// fails, or if an output block differs from the IOP result.
pub fn check_iop_multi_hpu_equivalence(
    iop_ir: &IR<IopLang>,
    hpu_irs: &[IR<HpuLang>],
    spec: CiphertextBlockSpec,
    nreps: usize,
) {
    let hpu_ir = IR::concat(hpu_irs);
    check_iop_hpu_equivalence_with(iop_ir, &hpu_ir, spec, nreps, |ir, ctx| {
        let mut evaluator = EffectfulEvaluator::from_ir(ir);
        if let Err(blocked) = evaluator.push_to_completion(ctx) {
            let blocked = blocked
                .iter()
                .map(|opid| format!("  {}", ir.get_op(opid).format()))
                .collect::<Vec<_>>()
                .join("\n");
            panic!("Multi-HPU deadlock: these transfers never completed:\n{blocked}");
        }
        if !evaluator.is_ok() {
            evaluator.into_eval_ir().dump_and_panic();
        }
        assert!(
            !ctx.has_pending_transfers(),
            "Some transfers were posted but never taken"
        );
    });
}

fn check_iop_hpu_equivalence_with(
    iop_ir: &IR<IopLang>,
    hpu_ir: &IR<HpuLang>,
    spec: CiphertextBlockSpec,
    nreps: usize,
    evaluate: impl Fn(&IR<HpuLang>, &mut HpuInterpreterContext),
) {
    // Discover input slots from the IOP IR.
    let mut input_slots: Vec<(usize, bool, u16)> = Vec::new(); // (pos, is_ct, int_size)
    for op in iop_ir.walk_ops_linear() {
        match op.get_instruction() {
            IopInstructionSet::InputCiphertext { pos, int_size } => {
                input_slots.push((*pos, true, *int_size));
            }
            IopInstructionSet::InputPlaintext { pos, int_size } => {
                input_slots.push((*pos, false, *int_size));
            }
            _ => {}
        }
    }
    input_slots.sort_by_key(|(pos, _, _)| *pos);

    for _ in 0..nreps {
        // Generate random IOP inputs.
        let iop_inputs: Vec<IopValue> = input_slots
            .iter()
            .map(|(_, is_ct, int_size)| {
                if *is_ct {
                    IopValue::Ciphertext(spec.ciphertext_spec(*int_size).random())
                } else {
                    IopValue::Plaintext(
                        spec.matching_plaintext_block_spec()
                            .plaintext_spec(*int_size)
                            .random(),
                    )
                }
            })
            .collect();

        // Interpret IOP.
        let mut iop_ctx = IopInterepreterContext {
            spec,
            inputs: iop_inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::default(),
        };
        iop_ir
            .evaluate::<IopValue>(&mut iop_ctx)
            .expect("IOP interpretation failed");

        // Populate HPU context: decompose IOP inputs into block-level entries.
        let mut hpu_ctx = HpuInterpreterContext::new(spec);
        let mut ct_idx = 0usize;
        let mut pt_idx = 0usize;
        for val in iop_inputs.iter() {
            match val {
                IopValue::Ciphertext(ct) => {
                    for i in 0..ct.len() {
                        hpu_ctx.sources.insert(
                            TSrcId {
                                src_pos: ct_idx.sas(),
                                block_pos: i.sas(),
                            },
                            ct.get_block(i),
                        );
                    }
                    ct_idx += 1;
                }
                IopValue::Plaintext(pt) => {
                    for i in 0..pt.len() {
                        hpu_ctx.immediates.insert(
                            TImmId {
                                imm_pos: pt_idx.sas(),
                                block_pos: i.sas(),
                            },
                            pt.get_block(i),
                        );
                    }
                    pt_idx += 1;
                }
                _ => panic!("Unexpected input type"),
            }
        }

        // Interpret HPU.
        evaluate(hpu_ir, &mut hpu_ctx);

        // Compare: check each output block matches.
        for (pos, iop_output) in &iop_ctx.outputs {
            let IopValue::Ciphertext(expected_ct) = iop_output else {
                panic!("Expected Ciphertext output at position {pos}");
            };
            for i in 0..expected_ct.len() {
                let tdst = TDstId {
                    dst_pos: (*pos).sas(),
                    block_pos: i.sas(),
                };
                let hpu_block = hpu_ctx
                    .destinations
                    .get(&tdst)
                    .unwrap_or_else(|| panic!("Missing HPU output at {tdst}"));
                assert_eq!(
                    hpu_block.mask_message(),
                    expected_ct.get_block(i),
                    "Output mismatch at pos={pos}, block={i}"
                );
            }
        }
    }
}

pub fn check_iop_dop_equivalence(
    iop_ir: &IR<IopLang>,
    dop_ir: &IR<DopLang>,
    lut_reg: &LutRegistry,
    spec: CiphertextBlockSpec,
    num_registers: usize,
    nreps: usize,
) {
    // Discover input slots from the IOP IR.
    let mut input_slots: Vec<(usize, bool, u16)> = Vec::new();
    for op in iop_ir.walk_ops_linear() {
        match op.get_instruction() {
            IopInstructionSet::InputCiphertext { pos, int_size } => {
                input_slots.push((*pos, true, *int_size));
            }
            IopInstructionSet::InputPlaintext { pos, int_size } => {
                input_slots.push((*pos, false, *int_size));
            }
            _ => {}
        }
    }
    input_slots.sort_by_key(|(pos, _, _)| *pos);

    for _ in 0..nreps {
        // Generate random IOP inputs.
        let iop_inputs: Vec<IopValue> = input_slots
            .iter()
            .map(|(_, is_ct, int_size)| {
                if *is_ct {
                    IopValue::Ciphertext(spec.ciphertext_spec(*int_size).random())
                } else {
                    IopValue::Plaintext(
                        spec.matching_plaintext_block_spec()
                            .plaintext_spec(*int_size)
                            .random(),
                    )
                }
            })
            .collect();

        // Interpret IOP.
        let mut iop_ctx = IopInterepreterContext {
            spec,
            inputs: iop_inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::default(),
        };
        iop_ir
            .evaluate::<IopValue>(&mut iop_ctx)
            .expect("IOP interpretation failed");

        // Populate DOP context: decompose IOP inputs into block-level entries.
        let mut dop_ctx = DopInterpreterContext::new(spec, num_registers, lut_reg);
        let mut ct_idx = 0usize;
        let mut pt_idx = 0usize;
        for val in iop_inputs.iter() {
            match val {
                IopValue::Ciphertext(ct) => {
                    for i in 0..ct.len() {
                        dop_ctx.sources.insert((ct_idx, i.sas()), ct.get_block(i));
                    }
                    ct_idx += 1;
                }
                IopValue::Plaintext(pt) => {
                    for i in 0..pt.len() {
                        dop_ctx
                            .pt_sources
                            .insert((pt_idx, i.sas()), pt.get_block(i));
                    }
                    pt_idx += 1;
                }
                _ => panic!("Unexpected input type"),
            }
        }

        // Interpret DOP.
        match dop_ir.evaluate::<DopValue>(&mut dop_ctx) {
            Err(eval_ir) => eval_ir.dump_and_panic(),
            Ok(_) => {}
        };

        // Compare: check each output block matches.
        for (pos, iop_output) in &iop_ctx.outputs {
            let IopValue::Ciphertext(expected_ct) = iop_output else {
                panic!("Expected Ciphertext output at position {pos}");
            };
            for i in 0..expected_ct.len() {
                let dop_block = dop_ctx
                    .destinations
                    .get(&(*pos, i.sas()))
                    .unwrap_or_else(|| panic!("Missing DOP output at pos={pos}, block={i}"));
                assert_eq!(
                    dop_block.mask_message(),
                    expected_ct.get_block(i),
                    "Output mismatch at pos={pos}, block={i}"
                );
            }
        }
    }
}
