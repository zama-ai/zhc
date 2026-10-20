use zhc_builder::SpecBlock;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_crypto::integer_semantics::{EmulatedCiphertextBool, EmulatedPlaintextInteger};
use zhc_ir::{IR, evaluation::EffectfulEvaluator};
use zhc_langs::{
    doplang::{DopInterpreterContext, DopLang, DopValue},
    hpulang::{HpuInterpreterContext, HpuLang, HpuValue, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopInterepreterContext, IopLang, IopValue},
};
use zhc_utils::{Dumpable, FastMap, SafeAs};

#[derive(Clone, Copy)]
enum InputKind {
    CiphertextInteger(u16),
    CiphertextBool,
    PlaintextInteger(u16),
}

impl InputKind {
    fn random_value(self, spec: SpecBlock) -> IopValue {
        match self {
            InputKind::CiphertextInteger(int_size) => {
                IopValue::CiphertextInteger(spec.integer_spec(int_size).random())
            }
            InputKind::CiphertextBool => {
                IopValue::CiphertextBool(EmulatedCiphertextBool::random(spec))
            }
            InputKind::PlaintextInteger(int_size) => IopValue::PlaintextInteger(
                EmulatedPlaintextInteger::random(spec.integer_spec(int_size)),
            ),
        }
    }
}

fn discover_input_slots(ir: &IR<IopLang>) -> Vec<(usize, InputKind)> {
    ir.walk_ops_linear()
        .filter_map(|op| match op.get_instruction() {
            IopInstructionSet::InputCiphertextInteger { pos, int_size } => {
                Some((*pos, InputKind::CiphertextInteger(*int_size)))
            }
            IopInstructionSet::InputCiphertextBool { pos } => {
                Some((*pos, InputKind::CiphertextBool))
            }
            IopInstructionSet::InputPlaintextInteger { pos, int_size } => {
                Some((*pos, InputKind::PlaintextInteger(*int_size)))
            }
            _ => None,
        })
        .collect()
}

/// Checks an IOP program against a reference IOP program.
///
/// Runs `nreps` rounds. Each round draws random values for the inputs of
/// `reference_ir`, interprets both programs on these values under `spec`, and
/// compares their outputs. Panics if an operation of either program fails, or
/// if the outputs of `candidate_ir` differ from those of `reference_ir`.
pub fn check_iop_equivalence(
    reference_ir: &IR<IopLang>,
    candidate_ir: &IR<IopLang>,
    spec: SpecBlock,
    nreps: usize,
) {
    let mut input_slots = discover_input_slots(reference_ir);
    input_slots.sort_by_key(|(pos, _)| *pos);

    for _ in 0..nreps {
        let inputs: FastMap<usize, IopValue> = input_slots
            .iter()
            .map(|(pos, kind)| (*pos, kind.random_value(spec)))
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
    spec: SpecBlock,
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
    spec: SpecBlock,
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
    spec: SpecBlock,
    nreps: usize,
    evaluate: impl Fn(&IR<HpuLang>, &mut HpuInterpreterContext),
) {
    let mut input_slots = discover_input_slots(iop_ir);
    input_slots.sort_by_key(|(pos, _)| *pos);

    for _ in 0..nreps {
        // Generate random IOP inputs.
        let iop_inputs: Vec<IopValue> = input_slots
            .iter()
            .map(|(_, kind)| kind.random_value(spec))
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
                IopValue::CiphertextInteger(ct) => {
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
                IopValue::PlaintextInteger(pt) => {
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
                IopValue::CiphertextBool(ct) => {
                    hpu_ctx.sources.insert(
                        TSrcId {
                            src_pos: ct_idx.sas(),
                            block_pos: 0,
                        },
                        ct.get_block(),
                    );
                    ct_idx += 1;
                }
                _ => panic!("Unexpected input type"),
            }
        }

        // Interpret HPU.
        evaluate(hpu_ir, &mut hpu_ctx);

        // Compare: check each output block matches.
        for (pos, iop_output) in &iop_ctx.outputs {
            let expected_blocks = match iop_output {
                IopValue::CiphertextInteger(ct) => {
                    (0..ct.len()).map(|i| ct.get_block(i)).collect::<Vec<_>>()
                }
                IopValue::CiphertextBool(ct) => vec![ct.get_block()],
                _ => panic!("Expected ciphertext output at position {pos}"),
            };
            for (i, expected_block) in expected_blocks.into_iter().enumerate() {
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
                    expected_block,
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
    spec: SpecBlock,
    num_registers: usize,
    nreps: usize,
) {
    let mut input_slots = discover_input_slots(iop_ir);
    input_slots.sort_by_key(|(pos, _)| *pos);

    for _ in 0..nreps {
        // Generate random IOP inputs.
        let iop_inputs: Vec<IopValue> = input_slots
            .iter()
            .map(|(_, kind)| kind.random_value(spec))
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
                IopValue::CiphertextInteger(ct) => {
                    for i in 0..ct.len() {
                        dop_ctx.sources.insert((ct_idx, i.sas()), ct.get_block(i));
                    }
                    ct_idx += 1;
                }
                IopValue::PlaintextInteger(pt) => {
                    for i in 0..pt.len() {
                        dop_ctx
                            .pt_sources
                            .insert((pt_idx, i.sas()), pt.get_block(i));
                    }
                    pt_idx += 1;
                }
                IopValue::CiphertextBool(ct) => {
                    dop_ctx.sources.insert((ct_idx, 0), ct.get_block());
                    ct_idx += 1;
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
            let expected_blocks = match iop_output {
                IopValue::CiphertextInteger(ct) => {
                    (0..ct.len()).map(|i| ct.get_block(i)).collect::<Vec<_>>()
                }
                IopValue::CiphertextBool(ct) => vec![ct.get_block()],
                _ => panic!("Expected ciphertext output at position {pos}"),
            };
            for (i, expected_block) in expected_blocks.into_iter().enumerate() {
                let dop_block = dop_ctx
                    .destinations
                    .get(&(*pos, i.sas()))
                    .unwrap_or_else(|| panic!("Missing DOP output at pos={pos}, block={i}"));
                assert_eq!(
                    dop_block.mask_message(),
                    expected_block,
                    "Output mismatch at pos={pos}, block={i}"
                );
            }
        }
    }
}
