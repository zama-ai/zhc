use zhc_builder::CiphertextBlockSpec;
use zhc_crypto::integer_semantics::lut::{Lut1, Lut2};
use zhc_ir::IR;
use zhc_langs::{
    doplang::{DopInterpreterContext, DopLang, DopValue},
    hpulang::{HpuInterpreterContext, HpuLang, HpuValue, LutId, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopInterepreterContext, IopLang, IopValue, Lut1Def},
};
use zhc_utils::{Dumpable, FastMap, SafeAs};

use crate::hpu::lowering::{GIDS1, GIDS2, lower_iop_to_hpu};

pub fn check_iop_hpu_equivalence(
    iop_ir: &IR<IopLang>,
    hpu_ir: &IR<HpuLang>,
    spec: CiphertextBlockSpec,
    nreps: usize,
) {
    // Build reverse LUT tables, builtins plus the non-builtin ones lowering allocated.
    // Re-lowering assigns the same gids, so they match the stream.
    let mut lut1: FastMap<LutId, Lut1> = GIDS1.iter().map(|(k, v)| (*v, k.clone())).collect();
    for (gid, table) in lower_iop_to_hpu(iop_ir).lut_payload {
        let def = Lut1Def::Table {
            name: format!("dyn_{}", gid.0),
            table,
        };
        lut1.insert(gid, def.into_lut(spec));
    }
    let lut2: FastMap<LutId, Lut2> = GIDS2.iter().map(|(k, v)| (*v, k.clone())).collect();

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
        hpu_ctx.lut1_table = lut1.clone();
        hpu_ctx.lut2_table = lut2.clone();
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
        match hpu_ir.evaluate::<HpuValue>(&mut hpu_ctx) {
            Err(eval_ir) => eval_ir.dump_and_panic(),
            Ok(_) => {}
        };

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
    spec: CiphertextBlockSpec,
    num_registers: usize,
    nreps: usize,
) {
    // Build reverse LUT tables, builtins plus the non-builtin ones lowering allocated.
    // Re-lowering assigns the same gids, so they match the stream.
    let mut lut1: FastMap<LutId, Lut1> = GIDS1.iter().map(|(k, v)| (*v, k.clone())).collect();
    for (gid, table) in lower_iop_to_hpu(iop_ir).lut_payload {
        let def = Lut1Def::Table {
            name: format!("dyn_{}", gid.0),
            table,
        };
        lut1.insert(gid, def.into_lut(spec));
    }
    let lut2: FastMap<LutId, Lut2> = GIDS2.iter().map(|(k, v)| (*v, k.clone())).collect();

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
        let mut dop_ctx = DopInterpreterContext::new(spec, num_registers);
        dop_ctx.lut1_table = lut1.clone();
        dop_ctx.lut2_table = lut2.clone();
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
