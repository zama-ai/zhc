use zhc_builder::CiphertextBlockSpec;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::IR;
use zhc_langs::{
    doplang::{DopInterpreterContext, DopLang, DopValue},
    hpulang::{HpuInterpreterContext, HpuLang, HpuValue, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopInterepreterContext, IopLang, IopValue},
};
use zhc_utils::{Dumpable, FastMap, SafeAs};

#[derive(Clone, Copy)]
enum InputKind {
    IntegerCiphertext(u16),
    BoolCiphertext,
    IntegerPlaintext(u16),
}

pub fn check_iop_hpu_equivalence(
    iop_ir: &IR<IopLang>,
    hpu_ir: &IR<HpuLang>,
    spec: CiphertextBlockSpec,
    nreps: usize,
) {
    // Discover input slots from the IOP IR.
    let mut input_slots: Vec<(usize, InputKind)> = Vec::new();
    for op in iop_ir.walk_ops_linear() {
        match op.get_instruction() {
            IopInstructionSet::InputIntegerCiphertext { pos, int_size } => {
                input_slots.push((*pos, InputKind::IntegerCiphertext(*int_size)));
            }
            IopInstructionSet::InputBoolCiphertext { pos } => {
                input_slots.push((*pos, InputKind::BoolCiphertext));
            }
            IopInstructionSet::InputIntegerPlaintext { pos, int_size } => {
                input_slots.push((*pos, InputKind::IntegerPlaintext(*int_size)));
            }
            _ => {}
        }
    }
    input_slots.sort_by_key(|(pos, _)| *pos);

    for _ in 0..nreps {
        // Generate random IOP inputs.
        let iop_inputs: Vec<IopValue> = input_slots
            .iter()
            .map(|(_, kind)| match kind {
                InputKind::IntegerCiphertext(int_size) => {
                    IopValue::IntegerCiphertext(spec.integer_ciphertext_spec(*int_size).random())
                }
                InputKind::BoolCiphertext => IopValue::BoolCiphertext(
                    zhc_crypto::integer_semantics::EmulatedBoolCiphertext::random(spec),
                ),
                InputKind::IntegerPlaintext(int_size) => IopValue::IntegerPlaintext(
                    spec.matching_plaintext_block_spec()
                        .integer_plaintext_spec(*int_size)
                        .random(),
                ),
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
                IopValue::IntegerCiphertext(ct) => {
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
                IopValue::IntegerPlaintext(pt) => {
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
                IopValue::BoolCiphertext(ct) => {
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
        match hpu_ir.evaluate::<HpuValue>(&mut hpu_ctx) {
            Err(eval_ir) => eval_ir.dump_and_panic(),
            Ok(_) => {}
        };

        // Compare: check each output block matches.
        for (pos, iop_output) in &iop_ctx.outputs {
            let expected_blocks = match iop_output {
                IopValue::IntegerCiphertext(ct) => {
                    (0..ct.len()).map(|i| ct.get_block(i)).collect::<Vec<_>>()
                }
                IopValue::BoolCiphertext(ct) => vec![ct.get_block()],
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
    spec: CiphertextBlockSpec,
    num_registers: usize,
    nreps: usize,
) {
    // Discover input slots from the IOP IR.
    let mut input_slots: Vec<(usize, InputKind)> = Vec::new();
    for op in iop_ir.walk_ops_linear() {
        match op.get_instruction() {
            IopInstructionSet::InputIntegerCiphertext { pos, int_size } => {
                input_slots.push((*pos, InputKind::IntegerCiphertext(*int_size)));
            }
            IopInstructionSet::InputBoolCiphertext { pos } => {
                input_slots.push((*pos, InputKind::BoolCiphertext));
            }
            IopInstructionSet::InputIntegerPlaintext { pos, int_size } => {
                input_slots.push((*pos, InputKind::IntegerPlaintext(*int_size)));
            }
            _ => {}
        }
    }
    input_slots.sort_by_key(|(pos, _)| *pos);

    for _ in 0..nreps {
        // Generate random IOP inputs.
        let iop_inputs: Vec<IopValue> = input_slots
            .iter()
            .map(|(_, kind)| match kind {
                InputKind::IntegerCiphertext(int_size) => {
                    IopValue::IntegerCiphertext(spec.integer_ciphertext_spec(*int_size).random())
                }
                InputKind::BoolCiphertext => IopValue::BoolCiphertext(
                    zhc_crypto::integer_semantics::EmulatedBoolCiphertext::random(spec),
                ),
                InputKind::IntegerPlaintext(int_size) => IopValue::IntegerPlaintext(
                    spec.matching_plaintext_block_spec()
                        .integer_plaintext_spec(*int_size)
                        .random(),
                ),
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
                IopValue::IntegerCiphertext(ct) => {
                    for i in 0..ct.len() {
                        dop_ctx.sources.insert((ct_idx, i.sas()), ct.get_block(i));
                    }
                    ct_idx += 1;
                }
                IopValue::IntegerPlaintext(pt) => {
                    for i in 0..pt.len() {
                        dop_ctx
                            .pt_sources
                            .insert((pt_idx, i.sas()), pt.get_block(i));
                    }
                    pt_idx += 1;
                }
                IopValue::BoolCiphertext(ct) => {
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
                IopValue::IntegerCiphertext(ct) => {
                    (0..ct.len()).map(|i| ct.get_block(i)).collect::<Vec<_>>()
                }
                IopValue::BoolCiphertext(ct) => vec![ct.get_block()],
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
