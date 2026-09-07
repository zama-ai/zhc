use zhc_crypto::integer_semantics::CiphertextBlockSpec;
use zhc_ir::{AnnIR, IR};
use zhc_utils::{SafeAs, ValueSet, existential_enum, svec};

use super::{IopLang, IopInstructionSet};


#[existential_enum]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValAnn {
    InputCiphertext(()),
    CiphertextBlock(ValueSet),
    InputPlaintext(()),
    PlaintextBlock(ValueSet),
    // Overflown,
    // Underflown,
    // Poisoned
}

pub fn analyze_values(ir: &IR<IopLang>, block_spec: CiphertextBlockSpec) -> AnnIR<'_, IopLang, (), ValAnn> {
    let mut input_block_range = ValueSet::new_empty(block_spec.complete_size());
    for v in block_spec.iter_message_space() {
        input_block_range.insert(v.raw_complete_bits().sas());
    }
    ir.backward_dataflow_analysis(|opref: zhc_ir::AnnOpRef<'_, '_, IopLang, zhc_ir::Analysing<()>, zhc_ir::Analysing<ValAnn>>|{
        use IopInstructionSet::*;
        let valanns = match opref.get_instruction() {
            InputCiphertext { .. } => svec![ValAnn::InputCiphertext(())],
            InputPlaintext { .. } => svec![ValAnn::InputPlaintext(())],
            OutputCiphertext { .. } => svec![],
            _Consume { .. } => svec![],
            Inspect { .. } => svec![opref.get_args_iter().next().unwrap().get_annotation().clone().unwrap_analyzed()],
            DeclareCiphertext { .. } => svec![ValAnn::InputCiphertext(())],
            LetCiphertextBlock { value } => svec![ValAnn::CiphertextBlock(ValueSet::from_single(block_spec.complete_size(), *value))],
            LetPlaintextBlock { value } => svec![ValAnn::PlaintextBlock(ValueSet::from_single(block_spec.complete_size(), *value))],
            PackCt { mul, .. } => {
                let lhs = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let rhs = opref.get_args_iter().nth(1).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let oup =  ValueSet::wrapping_add(lhs, rhs.wrapping_mul_scalar(*mul));
                svec![ValAnn::CiphertextBlock(oup)]
            },
            ExtractCtBlock { .. } => {
                let _inp = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_input_ciphertext_ref();
                svec![ValAnn::CiphertextBlock(input_block_range)]
            },
            ExtractPtBlock { .. } => {
                let _inp = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_input_plaintext_ref();
                svec![ValAnn::CiphertextBlock(input_block_range)]
            },
            StoreCtBlock { .. } => {
                svec![ValAnn::InputCiphertext(())]
            },
            Pbs { check, lut } => {
                let inp = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let func = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).raw_complete_bits().sas()
                };
                svec![ValAnn::CiphertextBlock(inp.apply(func))]
            },
            Pbs2 { check, lut } => {
                let inp = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let f0 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).0.raw_complete_bits().sas()
                };
                let f1 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).1.raw_complete_bits().sas()
                };
                svec![
                    ValAnn::CiphertextBlock(inp.apply(f0)),
                    ValAnn::CiphertextBlock(inp.apply(f1))
                ]
            },
            Pbs4 { check, lut } => {
                let inp = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let f0 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).0.raw_complete_bits().sas()
                };
                let f1 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).1.raw_complete_bits().sas()
                };
                let f2 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).2.raw_complete_bits().sas()
                };
                let f3 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).3.raw_complete_bits().sas()
                };
                svec![
                    ValAnn::CiphertextBlock(inp.apply(f0)),
                    ValAnn::CiphertextBlock(inp.apply(f1)),
                    ValAnn::CiphertextBlock(inp.apply(f2)),
                    ValAnn::CiphertextBlock(inp.apply(f3))
                ]
            },
            Pbs8 { check, lut } => {
                let inp = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let f0 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).0.raw_complete_bits().sas()
                };
                let f1 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).1.raw_complete_bits().sas()
                };
                let f2 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).2.raw_complete_bits().sas()
                };
                let f3 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).3.raw_complete_bits().sas()
                };
                let f4 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).4.raw_complete_bits().sas()
                };
                let f5 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).5.raw_complete_bits().sas()
                };
                let f6 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).6.raw_complete_bits().sas()
                };
                let f7 = |v: u8| {
                    let inp = block_spec.from_complete(v.sas());
                    lut.lookup(inp, *check).7.raw_complete_bits().sas()
                };
                svec![
                    ValAnn::CiphertextBlock(inp.apply(f0)),
                    ValAnn::CiphertextBlock(inp.apply(f1)),
                    ValAnn::CiphertextBlock(inp.apply(f2)),
                    ValAnn::CiphertextBlock(inp.apply(f3)),
                    ValAnn::CiphertextBlock(inp.apply(f4)),
                    ValAnn::CiphertextBlock(inp.apply(f5)),
                    ValAnn::CiphertextBlock(inp.apply(f6)),
                    ValAnn::CiphertextBlock(inp.apply(f7))
                ]
            },
            AddCt { flavor } => {
                let lhs = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let rhs = opref.get_args_iter().nth(1).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                svec![ValAnn::CiphertextBlock(ValueSet::wrapping_add(lhs, rhs))]
            },
            SubCt { flavor } => {
                let lhs = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let rhs = opref.get_args_iter().nth(1).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                svec![ValAnn::CiphertextBlock(ValueSet::wrapping_sub(lhs, rhs))]
            },
            AddPt { flavor } => {
                let lhs = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let rhs = opref.get_args_iter().nth(1).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_plaintext_block();
                svec![ValAnn::CiphertextBlock(ValueSet::wrapping_add(lhs, rhs))]
            },
            SubPt { flavor } => {
                let lhs = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let rhs = opref.get_args_iter().nth(1).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_plaintext_block();
                svec![ValAnn::CiphertextBlock(ValueSet::wrapping_sub(lhs, rhs))]
            },
            PtSub { flavor } => {
                let lhs = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_plaintext_block();
                let rhs = opref.get_args_iter().nth(1).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                svec![ValAnn::CiphertextBlock(ValueSet::wrapping_sub(lhs, rhs))]
            },
            MulPt { flavor } => {
                let lhs = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                let rhs = opref.get_args_iter().nth(1).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_plaintext_block();
                svec![ValAnn::CiphertextBlock(ValueSet::wrapping_mul(lhs, rhs))]
            },
            ShlCt { amount, flavor } => {
                let inp = opref.get_args_iter().nth(0).unwrap().get_annotation().clone().unwrap_analyzed().unwrap_ciphertext_block();
                svec![ValAnn::CiphertextBlock(inp.wrapping_mul_scalar(2u8.pow(*amount as u32)))]
            },
        };
        ((), valanns)
    })
}
