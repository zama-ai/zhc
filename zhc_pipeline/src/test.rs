use zhc_builder::{CiphertextBlockSpec, CiphertextSpec, add, cmp_gt};
use zhc_ir::IR;
use zhc_langs::{
    doplang::{DopInterpreterContext, DopLang, DopValue},
    hpulang::{HpuInterpreterContext, HpuLang, HpuValue, LutId, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopInterepreterContext, IopLang, IopValue, Lut1Def, Lut2Def},
};
use zhc_utils::{Dumpable, FastMap, assert_display_is};

use crate::translation::{GIDS1, GIDS2};

pub fn check_iop_hpu_equivalence(
    iop_ir: &IR<IopLang>,
    hpu_ir: &IR<HpuLang>,
    spec: CiphertextBlockSpec,
    nreps: usize,
) {
    // Build reverse LUT tables.
    let lut1: FastMap<LutId, Lut1Def> = GIDS1.iter().map(|(k, v)| (*v, *k)).collect();
    let lut2: FastMap<LutId, Lut2Def> = GIDS2.iter().map(|(k, v)| (*v, *k)).collect();

    // Discover input slots from the IOP IR.
    let mut input_slots: Vec<(usize, bool, u16)> = Vec::new(); // (pos, is_ct, int_size)
    for op in iop_ir.walk_ops_linear() {
        match op.get_instruction() {
            IopInstructionSet::InputCiphertext { pos, int_size } => {
                input_slots.push((pos, true, int_size));
            }
            IopInstructionSet::InputPlaintext { pos, int_size } => {
                input_slots.push((pos, false, int_size));
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
        let iop_ctx = IopInterepreterContext {
            spec,
            inputs: iop_inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::new(),
        };
        let (_, iop_ctx) = iop_ir
            .interpret::<IopValue>(iop_ctx)
            .expect("IOP interpretation failed");

        // Populate HPU context: decompose IOP inputs into block-level entries.
        let mut hpu_ctx = HpuInterpreterContext::new(spec);
        hpu_ctx.lut1_table = lut1.clone();
        hpu_ctx.lut2_table = lut2.clone();
        for (pos, val) in iop_inputs.iter().enumerate() {
            match val {
                IopValue::Ciphertext(ct) => {
                    for i in 0..ct.len() {
                        hpu_ctx.sources.insert(
                            TSrcId {
                                src_pos: pos as u32,
                                block_pos: i as u32,
                            },
                            ct.get_block(i),
                        );
                    }
                }
                IopValue::Plaintext(pt) => {
                    for i in 0..pt.len() {
                        hpu_ctx.immediates.insert(
                            TImmId {
                                imm_pos: pos as u32,
                                block_pos: i as u32,
                            },
                            pt.get_block(i),
                        );
                    }
                }
                _ => panic!("Unexpected input type"),
            }
        }

        // Interpret HPU.
        let hpu_ctx = match hpu_ir.interpret::<HpuValue>(hpu_ctx) {
            Ok((_, ctx)) => ctx,
            Err((ann_ir, _)) => ann_ir.dump_and_panic(),
        };

        // println!("iop_outputs:{:#?}", iop_ctx.outputs);
        // println!("hpu_outputs:{:#?}", hpu_ctx.destinations);

        // Compare: check each output block matches.
        for (pos, iop_output) in &iop_ctx.outputs {
            let IopValue::Ciphertext(expected_ct) = iop_output else {
                panic!("Expected Ciphertext output at position {pos}");
            };
            for i in 0..expected_ct.len() {
                let tdst = TDstId {
                    dst_pos: *pos as u32,
                    block_pos: i as u32,
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
    // Build reverse LUT tables.
    let lut1: FastMap<LutId, Lut1Def> = GIDS1.iter().map(|(k, v)| (*v, *k)).collect();
    let lut2: FastMap<LutId, Lut2Def> = GIDS2.iter().map(|(k, v)| (*v, *k)).collect();

    // Discover input slots from the IOP IR.
    let mut input_slots: Vec<(usize, bool, u16)> = Vec::new();
    for op in iop_ir.walk_ops_linear() {
        match op.get_instruction() {
            IopInstructionSet::InputCiphertext { pos, int_size } => {
                input_slots.push((pos, true, int_size));
            }
            IopInstructionSet::InputPlaintext { pos, int_size } => {
                input_slots.push((pos, false, int_size));
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
        let iop_ctx = IopInterepreterContext {
            spec,
            inputs: iop_inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::new(),
        };
        let (_, iop_ctx) = iop_ir
            .interpret::<IopValue>(iop_ctx)
            .expect("IOP interpretation failed");

        // Populate DOP context: decompose IOP inputs into block-level entries.
        let mut dop_ctx = DopInterpreterContext::new(spec, num_registers);
        dop_ctx.lut1_table = lut1.clone();
        dop_ctx.lut2_table = lut2.clone();
        for (pos, val) in iop_inputs.iter().enumerate() {
            match val {
                IopValue::Ciphertext(ct) => {
                    for i in 0..ct.len() {
                        dop_ctx.sources.insert((pos, i as usize), ct.get_block(i));
                    }
                }
                IopValue::Plaintext(pt) => {
                    for i in 0..pt.len() {
                        dop_ctx
                            .pt_sources
                            .insert((pos, i as usize), pt.get_block(i));
                    }
                }
                _ => panic!("Unexpected input type"),
            }
        }

        // Interpret DOP.
        let dop_ctx = match dop_ir.interpret::<DopValue>(dop_ctx) {
            Ok((_, ctx)) => ctx,
            Err((ann_ir, _)) => ann_ir.dump_and_panic(),
        };

        // Compare: check each output block matches.
        for (pos, iop_output) in &iop_ctx.outputs {
            let IopValue::Ciphertext(expected_ct) = iop_output else {
                panic!("Expected Ciphertext output at position {pos}");
            };
            for i in 0..expected_ct.len() {
                let dop_block = dop_ctx
                    .destinations
                    .get(&(*pos, i as usize))
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

#[test]
fn test_add_ir() {
    let ir = add(CiphertextSpec::new(16, 2, 2)).into_ir();
    assert_display_is!(
        ir.format(),
        r#"
                                      | %0 = input_ciphertext<0, 16>();
                                      | %1 = input_ciphertext<1, 16>();
                                      | %2 = extract_ct_block<0>(%0);
                                      | %3 = extract_ct_block<1>(%0);
                                      | %4 = extract_ct_block<2>(%0);
                                      | %5 = extract_ct_block<3>(%0);
                                      | %6 = extract_ct_block<4>(%0);
                                      | %7 = extract_ct_block<5>(%0);
                                      | %8 = extract_ct_block<6>(%0);
                                      | %9 = extract_ct_block<7>(%0);
                                      | %10 = extract_ct_block<0>(%1);
                                      | %11 = extract_ct_block<1>(%1);
                                      | %12 = extract_ct_block<2>(%1);
                                      | %13 = extract_ct_block<3>(%1);
                                      | %14 = extract_ct_block<4>(%1);
                                      | %15 = extract_ct_block<5>(%1);
                                      | %16 = extract_ct_block<6>(%1);
                                      | %17 = extract_ct_block<7>(%1);
            // Raw sum                | %18 = add_ct(%2, %10);
            // Raw sum                | %19 = add_ct(%3, %11);
            // Raw sum                | %20 = add_ct(%4, %12);
            // Raw sum                | %21 = add_ct(%5, %13);
            // Raw sum                | %22 = add_ct(%6, %14);
            // Raw sum                | %23 = add_ct(%7, %15);
            // Raw sum                | %24 = add_ct(%8, %16);
            // Raw sum                | %25 = add_ct(%9, %17);
            // Block States / G0-B0   | %27, %28 = pbs2<ManyCarryMsg>(%18);
            // Block States / G0-B1   | %29 = pbs<Protect, ExtractPropGroup0>(%19);
            // Block States / G0-B2   | %30 = pbs<Protect, ExtractPropGroup1>(%20);
            // Block States / G0-B3   | %31 = pbs<Protect, ExtractPropGroup2>(%21);
            // Block States / GN-B0   | %32 = pbs<Protect, ExtractPropGroup0>(%22);
            // Block States / GN-B1   | %33 = pbs<Protect, ExtractPropGroup1>(%23);
            // Block States / GN-B2   | %34 = pbs<Protect, ExtractPropGroup2>(%24);
            // Group states           | %36 = add_ct(%28, %29);
            // Group states           | %37 = add_ct(%36, %30);
            // Group states           | %38 = temper_add_ct(%37, %31);
            // Group states           | %39 = pbs<Protect, SolvePropGroupFinal2>(%38);
            // Group states           | %44 = add_ct(%32, %33);
            // Group states           | %45 = add_ct(%44, %34);
            // Final resolution       | %56 = pbs<Protect, SolvePropGroupFinal0>(%36);
            // Final resolution       | %57 = pbs<Protect, SolvePropGroupFinal1>(%37);
            // Final resolution       | %62 = add_ct(%32, %39);
            // Final resolution       | %63 = pbs<Protect, SolvePropGroupFinal0>(%62);
            // Final resolution       | %64 = add_ct(%44, %39);
            // Final resolution       | %65 = pbs<Protect, SolvePropGroupFinal1>(%64);
            // Final resolution       | %66 = add_ct(%45, %39);
            // Final resolution       | %67 = pbs<Protect, SolvePropGroupFinal2>(%66);
            // Carry propagation      | %74 = add_ct(%19, %28);
            // Carry propagation      | %75 = add_ct(%20, %56);
            // Carry propagation      | %76 = add_ct(%21, %57);
            // Carry propagation      | %77 = add_ct(%22, %39);
            // Carry propagation      | %78 = add_ct(%23, %63);
            // Carry propagation      | %79 = add_ct(%24, %65);
            // Carry propagation      | %80 = add_ct(%25, %67);
            // Cleanup                | %81 = pbs<Protect, MsgOnly>(%27);
            // Cleanup                | %82 = pbs<Protect, MsgOnly>(%74);
            // Cleanup                | %83 = pbs<Protect, MsgOnly>(%75);
            // Cleanup                | %84 = pbs<Protect, MsgOnly>(%76);
            // Cleanup                | %85 = pbs<Protect, MsgOnly>(%77);
            // Cleanup                | %86 = pbs<Protect, MsgOnly>(%78);
            // Cleanup                | %87 = pbs<Protect, MsgOnly>(%79);
            // Cleanup                | %88 = pbs<Protect, MsgOnly>(%80);
            // Join                   | %89 = decl_ct<16>();
            // Join                   | %90 = store_ct_block<0>(%81, %89);
            // Join                   | %91 = store_ct_block<1>(%82, %90);
            // Join                   | %92 = store_ct_block<2>(%83, %91);
            // Join                   | %93 = store_ct_block<3>(%84, %92);
            // Join                   | %94 = store_ct_block<4>(%85, %93);
            // Join                   | %95 = store_ct_block<5>(%86, %94);
            // Join                   | %96 = store_ct_block<6>(%87, %95);
            // Join                   | %97 = store_ct_block<7>(%88, %96);
                                      | output<0>(%97);
        "#
    );
}

#[test]
fn test_cmp_ir() {
    let ir = cmp_gt(CiphertextSpec::new(16, 2, 2)).into_ir();
    assert_display_is!(
        ir.format(),
        r#"
                                       | %0 = input_ciphertext<0, 16>();
                                       | %1 = input_ciphertext<1, 16>();
                                       | %2 = extract_ct_block<0>(%0);
                                       | %3 = extract_ct_block<1>(%0);
                                       | %4 = extract_ct_block<2>(%0);
                                       | %5 = extract_ct_block<3>(%0);
                                       | %6 = extract_ct_block<4>(%0);
                                       | %7 = extract_ct_block<5>(%0);
                                       | %8 = extract_ct_block<6>(%0);
                                       | %9 = extract_ct_block<7>(%0);
                                       | %10 = extract_ct_block<0>(%1);
                                       | %11 = extract_ct_block<1>(%1);
                                       | %12 = extract_ct_block<2>(%1);
                                       | %13 = extract_ct_block<3>(%1);
                                       | %14 = extract_ct_block<4>(%1);
                                       | %15 = extract_ct_block<5>(%1);
                                       | %16 = extract_ct_block<6>(%1);
                                       | %17 = extract_ct_block<7>(%1);
            // Pack A                  | %18 = pack_ct<4>(%3, %2);
            // Pack A                  | %19 = pbs<Protect, None>(%18);
            // Pack A                  | %20 = pack_ct<4>(%5, %4);
            // Pack A                  | %21 = pbs<Protect, None>(%20);
            // Pack A                  | %22 = pack_ct<4>(%7, %6);
            // Pack A                  | %23 = pbs<Protect, None>(%22);
            // Pack A                  | %24 = pack_ct<4>(%9, %8);
            // Pack A                  | %25 = pbs<Protect, None>(%24);
            // Pack B                  | %26 = pack_ct<4>(%11, %10);
            // Pack B                  | %27 = pbs<Protect, None>(%26);
            // Pack B                  | %28 = pack_ct<4>(%13, %12);
            // Pack B                  | %29 = pbs<Protect, None>(%28);
            // Pack B                  | %30 = pack_ct<4>(%15, %14);
            // Pack B                  | %31 = pbs<Protect, None>(%30);
            // Pack B                  | %32 = pack_ct<4>(%17, %16);
            // Pack B                  | %33 = pbs<Protect, None>(%32);
            // Compare blocks / 0-th   | %34 = sub_ct(%19, %27);
            // Compare blocks / 0-th   | %35 = pbs<Protect, CmpSign>(%34);
            // Compare blocks / 0-th   | %36 = let_pt_block<1>();
            // Compare blocks / 0-th   | %37 = add_pt(%35, %36);
            // Compare blocks / 1-th   | %38 = sub_ct(%21, %29);
            // Compare blocks / 1-th   | %39 = pbs<Protect, CmpSign>(%38);
            // Compare blocks / 1-th   | %41 = add_pt(%39, %36);
            // Compare blocks / 2-th   | %42 = sub_ct(%23, %31);
            // Compare blocks / 2-th   | %43 = pbs<Protect, CmpSign>(%42);
            // Compare blocks / 2-th   | %45 = add_pt(%43, %36);
            // Compare blocks / 3-th   | %46 = sub_ct(%25, %33);
            // Compare blocks / 3-th   | %47 = pbs<Protect, CmpSign>(%46);
            // Compare blocks / 3-th   | %49 = add_pt(%47, %36);
            // Reduce comparison       | %50 = pack_ct<4>(%41, %37);
            // Reduce comparison       | %51 = pack_ct<4>(%49, %45);
            // Reduce comparison       | %52 = pbs<Protect, CmpReduce>(%50);
            // Reduce comparison       | %53 = pbs<Protect, CmpReduce>(%51);
                                       | %54 = pack_ct<4>(%53, %52);
                                       | %55 = pbs<Protect, CmpGtMrg>(%54);
                                       | %56 = decl_ct<2>();
                                       | %57 = store_ct_block<0>(%55, %56);
                                       | output<0>(%57);
        "#
    );
}
