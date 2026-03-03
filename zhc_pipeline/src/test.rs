use zhc_builder::{CiphertextBlockSpec, CiphertextSpec, add, cmp_gt};
use zhc_ir::IR;
use zhc_langs::{
    hpulang::{HpuInterpreterContext, HpuLang, HpuValue, LutId, TDstId, TImmId, TSrcId},
    ioplang::{IopInstructionSet, IopInterepreterContext, IopLang, IopValue, Lut1Def, Lut2Def},
};
use zhc_utils::{FastMap, assert_display_is};

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
            Err((ann_ir, _)) => ann_ir.dump(),
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

#[test]
fn test_add_ir() {
    let ir = add(CiphertextSpec::new(16, 2, 2)).into_ir();
    assert_display_is!(
        ir.format(),
        r#"
            %0 : Ct = input_ciphertext<0, 16>();
            %1 : Ct = input_ciphertext<1, 16>();
            %89 : Ct = decl_ct<16>();
            %2 : CtBlock = extract_ct_block<0>(%0 : Ct);
            %3 : CtBlock = extract_ct_block<1>(%0 : Ct);
            %4 : CtBlock = extract_ct_block<2>(%0 : Ct);
            %5 : CtBlock = extract_ct_block<3>(%0 : Ct);
            %6 : CtBlock = extract_ct_block<4>(%0 : Ct);
            %7 : CtBlock = extract_ct_block<5>(%0 : Ct);
            %8 : CtBlock = extract_ct_block<6>(%0 : Ct);
            %9 : CtBlock = extract_ct_block<7>(%0 : Ct);
            %10 : CtBlock = extract_ct_block<0>(%1 : Ct);
            %11 : CtBlock = extract_ct_block<1>(%1 : Ct);
            %12 : CtBlock = extract_ct_block<2>(%1 : Ct);
            %13 : CtBlock = extract_ct_block<3>(%1 : Ct);
            %14 : CtBlock = extract_ct_block<4>(%1 : Ct);
            %15 : CtBlock = extract_ct_block<5>(%1 : Ct);
            %16 : CtBlock = extract_ct_block<6>(%1 : Ct);
            %17 : CtBlock = extract_ct_block<7>(%1 : Ct);
            %18 : CtBlock = add_ct(%2 : CtBlock, %10 : CtBlock);
            %19 : CtBlock = add_ct(%3 : CtBlock, %11 : CtBlock);
            %20 : CtBlock = add_ct(%4 : CtBlock, %12 : CtBlock);
            %21 : CtBlock = add_ct(%5 : CtBlock, %13 : CtBlock);
            %22 : CtBlock = add_ct(%6 : CtBlock, %14 : CtBlock);
            %23 : CtBlock = add_ct(%7 : CtBlock, %15 : CtBlock);
            %24 : CtBlock = add_ct(%8 : CtBlock, %16 : CtBlock);
            %25 : CtBlock = add_ct(%9 : CtBlock, %17 : CtBlock);
            %27 : CtBlock, %28 : CtBlock = pbs2<ManyCarryMsg>(%18 : CtBlock);
            %29 : CtBlock = pbs<Protect, ExtractPropGroup0>(%19 : CtBlock);
            %30 : CtBlock = pbs<Protect, ExtractPropGroup1>(%20 : CtBlock);
            %31 : CtBlock = pbs<Protect, ExtractPropGroup2>(%21 : CtBlock);
            %32 : CtBlock = pbs<Protect, ExtractPropGroup0>(%22 : CtBlock);
            %33 : CtBlock = pbs<Protect, ExtractPropGroup1>(%23 : CtBlock);
            %34 : CtBlock = pbs<Protect, ExtractPropGroup2>(%24 : CtBlock);
            %36 : CtBlock = add_ct(%28 : CtBlock, %29 : CtBlock);
            %44 : CtBlock = add_ct(%32 : CtBlock, %33 : CtBlock);
            %74 : CtBlock = add_ct(%19 : CtBlock, %28 : CtBlock);
            %81 : CtBlock = pbs<Protect, MsgOnly>(%27 : CtBlock);
            %37 : CtBlock = add_ct(%36 : CtBlock, %30 : CtBlock);
            %45 : CtBlock = add_ct(%44 : CtBlock, %34 : CtBlock);
            %56 : CtBlock = pbs<Protect, SolvePropGroupFinal0>(%36 : CtBlock);
            %82 : CtBlock = pbs<Protect, MsgOnly>(%74 : CtBlock);
            %90 : Ct = store_ct_block<0>(%81 : CtBlock, %89 : Ct);
            %38 : CtBlock = temper_add_ct(%37 : CtBlock, %31 : CtBlock);
            %57 : CtBlock = pbs<Protect, SolvePropGroupFinal1>(%37 : CtBlock);
            %75 : CtBlock = add_ct(%20 : CtBlock, %56 : CtBlock);
            %91 : Ct = store_ct_block<1>(%82 : CtBlock, %90 : Ct);
            %39 : CtBlock = pbs<Protect, SolvePropGroupFinal2>(%38 : CtBlock);
            %76 : CtBlock = add_ct(%21 : CtBlock, %57 : CtBlock);
            %83 : CtBlock = pbs<Protect, MsgOnly>(%75 : CtBlock);
            %62 : CtBlock = add_ct(%32 : CtBlock, %39 : CtBlock);
            %64 : CtBlock = add_ct(%44 : CtBlock, %39 : CtBlock);
            %66 : CtBlock = add_ct(%45 : CtBlock, %39 : CtBlock);
            %77 : CtBlock = add_ct(%22 : CtBlock, %39 : CtBlock);
            %84 : CtBlock = pbs<Protect, MsgOnly>(%76 : CtBlock);
            %92 : Ct = store_ct_block<2>(%83 : CtBlock, %91 : Ct);
            %63 : CtBlock = pbs<Protect, SolvePropGroupFinal0>(%62 : CtBlock);
            %65 : CtBlock = pbs<Protect, SolvePropGroupFinal1>(%64 : CtBlock);
            %67 : CtBlock = pbs<Protect, SolvePropGroupFinal2>(%66 : CtBlock);
            %85 : CtBlock = pbs<Protect, MsgOnly>(%77 : CtBlock);
            %93 : Ct = store_ct_block<3>(%84 : CtBlock, %92 : Ct);
            %78 : CtBlock = add_ct(%23 : CtBlock, %63 : CtBlock);
            %79 : CtBlock = add_ct(%24 : CtBlock, %65 : CtBlock);
            %80 : CtBlock = add_ct(%25 : CtBlock, %67 : CtBlock);
            %94 : Ct = store_ct_block<4>(%85 : CtBlock, %93 : Ct);
            %86 : CtBlock = pbs<Protect, MsgOnly>(%78 : CtBlock);
            %87 : CtBlock = pbs<Protect, MsgOnly>(%79 : CtBlock);
            %88 : CtBlock = pbs<Protect, MsgOnly>(%80 : CtBlock);
            %95 : Ct = store_ct_block<5>(%86 : CtBlock, %94 : Ct);
            %96 : Ct = store_ct_block<6>(%87 : CtBlock, %95 : Ct);
            %97 : Ct = store_ct_block<7>(%88 : CtBlock, %96 : Ct);
            output<0>(%97 : Ct);
        "#
    );
}

#[test]
fn test_cmp_ir() {
    let ir = cmp_gt(CiphertextSpec::new(16, 2, 2)).into_ir();
    assert_display_is!(
        ir.format(),
        r#"
            %0 : Ct = input_ciphertext<0, 16>();
            %1 : Ct = input_ciphertext<1, 16>();
            %36 : PtBlock = let_pt_block<1>();
            %56 : Ct = decl_ct<2>();
            %2 : CtBlock = extract_ct_block<0>(%0 : Ct);
            %3 : CtBlock = extract_ct_block<1>(%0 : Ct);
            %4 : CtBlock = extract_ct_block<2>(%0 : Ct);
            %5 : CtBlock = extract_ct_block<3>(%0 : Ct);
            %6 : CtBlock = extract_ct_block<4>(%0 : Ct);
            %7 : CtBlock = extract_ct_block<5>(%0 : Ct);
            %8 : CtBlock = extract_ct_block<6>(%0 : Ct);
            %9 : CtBlock = extract_ct_block<7>(%0 : Ct);
            %10 : CtBlock = extract_ct_block<0>(%1 : Ct);
            %11 : CtBlock = extract_ct_block<1>(%1 : Ct);
            %12 : CtBlock = extract_ct_block<2>(%1 : Ct);
            %13 : CtBlock = extract_ct_block<3>(%1 : Ct);
            %14 : CtBlock = extract_ct_block<4>(%1 : Ct);
            %15 : CtBlock = extract_ct_block<5>(%1 : Ct);
            %16 : CtBlock = extract_ct_block<6>(%1 : Ct);
            %17 : CtBlock = extract_ct_block<7>(%1 : Ct);
            %18 : CtBlock = pack_ct<4>(%3 : CtBlock, %2 : CtBlock);
            %20 : CtBlock = pack_ct<4>(%5 : CtBlock, %4 : CtBlock);
            %22 : CtBlock = pack_ct<4>(%7 : CtBlock, %6 : CtBlock);
            %24 : CtBlock = pack_ct<4>(%9 : CtBlock, %8 : CtBlock);
            %26 : CtBlock = pack_ct<4>(%11 : CtBlock, %10 : CtBlock);
            %28 : CtBlock = pack_ct<4>(%13 : CtBlock, %12 : CtBlock);
            %30 : CtBlock = pack_ct<4>(%15 : CtBlock, %14 : CtBlock);
            %32 : CtBlock = pack_ct<4>(%17 : CtBlock, %16 : CtBlock);
            %19 : CtBlock = pbs<Protect, None>(%18 : CtBlock);
            %21 : CtBlock = pbs<Protect, None>(%20 : CtBlock);
            %23 : CtBlock = pbs<Protect, None>(%22 : CtBlock);
            %25 : CtBlock = pbs<Protect, None>(%24 : CtBlock);
            %27 : CtBlock = pbs<Protect, None>(%26 : CtBlock);
            %29 : CtBlock = pbs<Protect, None>(%28 : CtBlock);
            %31 : CtBlock = pbs<Protect, None>(%30 : CtBlock);
            %33 : CtBlock = pbs<Protect, None>(%32 : CtBlock);
            %34 : CtBlock = sub_ct(%19 : CtBlock, %27 : CtBlock);
            %38 : CtBlock = sub_ct(%21 : CtBlock, %29 : CtBlock);
            %42 : CtBlock = sub_ct(%23 : CtBlock, %31 : CtBlock);
            %46 : CtBlock = sub_ct(%25 : CtBlock, %33 : CtBlock);
            %35 : CtBlock = pbs<Protect, CmpSign>(%34 : CtBlock);
            %39 : CtBlock = pbs<Protect, CmpSign>(%38 : CtBlock);
            %43 : CtBlock = pbs<Protect, CmpSign>(%42 : CtBlock);
            %47 : CtBlock = pbs<Protect, CmpSign>(%46 : CtBlock);
            %37 : CtBlock = add_pt(%35 : CtBlock, %36 : PtBlock);
            %41 : CtBlock = add_pt(%39 : CtBlock, %36 : PtBlock);
            %45 : CtBlock = add_pt(%43 : CtBlock, %36 : PtBlock);
            %49 : CtBlock = add_pt(%47 : CtBlock, %36 : PtBlock);
            %50 : CtBlock = pack_ct<4>(%41 : CtBlock, %37 : CtBlock);
            %51 : CtBlock = pack_ct<4>(%49 : CtBlock, %45 : CtBlock);
            %52 : CtBlock = pbs<Protect, CmpReduce>(%50 : CtBlock);
            %53 : CtBlock = pbs<Protect, CmpReduce>(%51 : CtBlock);
            %54 : CtBlock = pack_ct<4>(%53 : CtBlock, %52 : CtBlock);
            %55 : CtBlock = pbs<Protect, CmpGtMrg>(%54 : CtBlock);
            %57 : Ct = store_ct_block<0>(%55 : CtBlock, %56 : Ct);
            output<0>(%57 : Ct);
        "#
    );
}
