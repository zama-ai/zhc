use zhc_crypto::integer_semantics::CiphertextBlockSpec;
use zhc_ir::{AnnIR, IR};
use zhc_utils::{ValueSet, svec};

use super::{IopLang, IopInstructionSet};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValAnn {
    InputCiphertext,
    CiphertextBlock(ValueSet),
    InputPlaintext,
    Overflown,
    Underflown,
    Poisoned
}

pub fn analyze_values(ir: &IR<IopLang>, block_spec: CiphertextBlockSpec) -> AnnIR<'_, IopLang, (), ValAnn> {
    ir.backward_dataflow_analysis(|opref|{
        use IopInstructionSet::*;
        let valanns = match opref.get_instruction() {
            InputCiphertext { .. } => svec![ValAnn::InputCiphertext],
            InputPlaintext { .. } => svec![ValAnn::InputPlaintext],
            OutputCiphertext { .. } => svec![],
            _Consume { .. } => svec![],
            Inspect { .. } => svec![opref.get_args_iter().next().unwrap().get_annotation().clone().unwrap_analyzed()],
            DeclareCiphertext { .. } => svec![ValAnn::InputCiphertext],
            LetCiphertextBlock { value } => svec![ValAnn::CiphertextBlock(ValueSet::from_single(block_spec.complete_size(), *value))],
            LetPlaintextBlock { value } => svec![ValAnn::NotConcerned],
            AddCt => todo!(),
            WrappingAddCt => todo!(),
            TemperAddCt => todo!(),
            SubCt => todo!(),
            WrappingSubCt => todo!(),
            PackCt { mul } => todo!(),
            AddPt => todo!(),
            WrappingAddPt => todo!(),
            SubPt => todo!(),
            PtSub => todo!(),
            MulPt => todo!(),
            ExtractCtBlock { index } => todo!(),
            ExtractPtBlock { index } => todo!(),
            StoreCtBlock { index } => todo!(),
            Pbs { check, lut } => todo!(),
            Pbs2 { check, lut } => todo!(),
        };
        ((), valanns)
    })
}
