use zhc_crypto::integer_semantics::{
    CiphertextBlockRange, CiphertextBlockSpec, CiphertextRange, PlaintextBlockRange,
    PlaintextRange, lut::LookupCheck,
};
use zhc_ir::{Analysing, AnnIR, AnnOpRef, IR};
use zhc_utils::{SafeAs, existential_enum, svec};

use super::{IopInstructionSet, IopLang};

/// The range annotation of an `IopLang` value.
#[existential_enum]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValAnn {
    Ciphertext(CiphertextRange),
    CiphertextBlock(CiphertextBlockRange),
    Plaintext(PlaintextRange),
    PlaintextBlock(PlaintextBlockRange),
}

/// Computes the set of values every value of the program can take.
///
/// Block ranges are exact images of the input ranges under each operation, but independent
/// operands are combined pairwise, so correlations between values are lost: `x - x` is not
/// `{0}`. The result is a sound over-approximation of the reachable values.
pub fn analyze_values(
    ir: &IR<IopLang>,
    block_spec: CiphertextBlockSpec,
) -> AnnIR<'_, IopLang, (), ValAnn> {

    // Single-output lookups are explored on the whole complete space, so padding checks are
    // disabled. Many-LUT lookups reject inputs with an active padding or index bit themselves.
    const LUT1_CHECK: LookupCheck = LookupCheck::AllowBothPadding;
    const LUTN_CHECK: LookupCheck = LookupCheck::AllowOutputPadding;

    ir.forward_dataflow_analysis(
        |op: AnnOpRef<'_, '_, IopLang, Analysing<()>, Analysing<ValAnn>>| {
            use IopInstructionSet::*;
            let valanns = match op.get_instruction() {
                InputCiphertext { int_size, .. } => svec![ValAnn::Ciphertext(
                    CiphertextRange::input_from_spec(block_spec.ciphertext_spec(*int_size))
                )],
                InputPlaintext { int_size, .. } => {
                    svec![ValAnn::Plaintext(PlaintextRange::input_from_spec(
                        block_spec
                            .matching_plaintext_block_spec()
                            .plaintext_spec(*int_size)
                    ))]
                }
                OutputCiphertext { .. } => svec![],
                _Consume { .. } => svec![],
                Inspect { .. } => svec![
                    op.get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                ],
                DeclareCiphertext { int_size } => svec![ValAnn::Ciphertext(
                    CiphertextRange::zero_from_spec(block_spec.ciphertext_spec(*int_size))
                )],
                LetCiphertextBlock { value } => svec![ValAnn::CiphertextBlock(
                    CiphertextBlockRange::from_single(block_spec, *value)
                )],
                LetPlaintextBlock { value } => {
                    svec![ValAnn::PlaintextBlock(PlaintextBlockRange::from_single(
                        block_spec.complete_plaintext_block_spec(),
                        *value
                    ))]
                }
                AddCt { .. } => {
                    let lhs = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let rhs = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    svec![ValAnn::CiphertextBlock(lhs.wrapping_add(rhs))]
                }
                SubCt { .. } => {
                    let lhs = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let rhs = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    svec![ValAnn::CiphertextBlock(lhs.wrapping_sub(rhs))]
                }
                ShlCt { amount, .. } => {
                    let inp = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    svec![ValAnn::CiphertextBlock(inp.wrapping_shl(*amount))]
                }
                PackCt { mul, .. } => {
                    let lhs = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let rhs = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    svec![ValAnn::CiphertextBlock(lhs.wrapping_mac(*mul, rhs))]
                }
                AddPt { .. } => {
                    let lhs = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let rhs = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_plaintext_block();
                    svec![ValAnn::CiphertextBlock(lhs.wrapping_add_pt(rhs))]
                }
                SubPt { .. } => {
                    let lhs = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let rhs = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_plaintext_block();
                    svec![ValAnn::CiphertextBlock(lhs.wrapping_sub_pt(rhs))]
                }
                PtSub { .. } => {
                    let lhs = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_plaintext_block();
                    let rhs = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    svec![ValAnn::CiphertextBlock(lhs.wrapping_sub_ct(rhs))]
                }
                MulPt { .. } => {
                    let lhs = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let rhs = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_plaintext_block();
                    svec![ValAnn::CiphertextBlock(lhs.wrapping_mul_pt(rhs))]
                }
                ExtractCtBlock { index } => {
                    let inp = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext();
                    svec![ValAnn::CiphertextBlock(inp.get_block(*index))]
                }
                ExtractPtBlock { index } => {
                    let inp = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_plaintext();
                    svec![ValAnn::PlaintextBlock(inp.get_block(*index))]
                }
                StoreCtBlock { index } => {
                    let block = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let mut ct = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext();
                    ct.set_block(*index, block);
                    svec![ValAnn::Ciphertext(ct)]
                }
                Pbs { lut, .. } => {
                    let inp = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let func = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUT1_CHECK).raw_complete_bits().sas()
                    };
                    svec![ValAnn::CiphertextBlock(inp.apply(func))]
                }
                Pbs2 { lut, .. } => {
                    let inp = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let f0 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).0.raw_complete_bits().sas()
                    };
                    let f1 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).1.raw_complete_bits().sas()
                    };
                    svec![
                        ValAnn::CiphertextBlock(inp.apply(f0)),
                        ValAnn::CiphertextBlock(inp.apply(f1)),
                    ]
                }
                Pbs4 { lut, .. } => {
                    let inp = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let f0 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).0.raw_complete_bits().sas()
                    };
                    let f1 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).1.raw_complete_bits().sas()
                    };
                    let f2 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).2.raw_complete_bits().sas()
                    };
                    let f3 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).3.raw_complete_bits().sas()
                    };
                    svec![
                        ValAnn::CiphertextBlock(inp.apply(f0)),
                        ValAnn::CiphertextBlock(inp.apply(f1)),
                        ValAnn::CiphertextBlock(inp.apply(f2)),
                        ValAnn::CiphertextBlock(inp.apply(f3)),
                    ]
                }
                Pbs8 { lut, .. } => {
                    let inp = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .clone()
                        .unwrap_analyzed()
                        .unwrap_ciphertext_block();
                    let f0 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).0.raw_complete_bits().sas()
                    };
                    let f1 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).1.raw_complete_bits().sas()
                    };
                    let f2 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).2.raw_complete_bits().sas()
                    };
                    let f3 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).3.raw_complete_bits().sas()
                    };
                    let f4 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).4.raw_complete_bits().sas()
                    };
                    let f5 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).5.raw_complete_bits().sas()
                    };
                    let f6 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).6.raw_complete_bits().sas()
                    };
                    let f7 = |v: u8| {
                        let inp = block_spec.from_complete(v.sas());
                        lut.lookup(inp, LUTN_CHECK).7.raw_complete_bits().sas()
                    };
                    svec![
                        ValAnn::CiphertextBlock(inp.apply(f0)),
                        ValAnn::CiphertextBlock(inp.apply(f1)),
                        ValAnn::CiphertextBlock(inp.apply(f2)),
                        ValAnn::CiphertextBlock(inp.apply(f3)),
                        ValAnn::CiphertextBlock(inp.apply(f4)),
                        ValAnn::CiphertextBlock(inp.apply(f5)),
                        ValAnn::CiphertextBlock(inp.apply(f6)),
                        ValAnn::CiphertextBlock(inp.apply(f7)),
                    ]
                }
            };
            ((), valanns)
        },
    )
}
