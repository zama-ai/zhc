use std::fmt::Debug;
use zhc_crypto::integer_semantics::{
    CiphertextSpec, EmulatedCiphertext, EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage,
    EmulatedPlaintext, EmulatedPlaintextBlock, EmulatedPlaintextBlockStorage,
};
use zhc_ir::interpretation::{Interpretable, Interpretation, InterpretsTo};
use zhc_utils::small::SmallVec;
use zhc_utils::{FastMap, svec};

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum IopValue {
    Ciphertext(EmulatedCiphertext),
    Plaintext(EmulatedPlaintext),
    CiphertextBlock(EmulatedCiphertextBlock),
    PlaintextBlock(EmulatedPlaintextBlock),
}

impl IopValue {
    pub fn unwrap_ciphertext(self) -> EmulatedCiphertext {
        match self {
            Self::Ciphertext(v) => v,
            _ => panic!("Expected Ciphertext, got {:?}", self),
        }
    }

    pub fn unwrap_plaintext(self) -> EmulatedPlaintext {
        match self {
            Self::Plaintext(v) => v,
            _ => panic!("Expected Plaintext, got {:?}", self),
        }
    }

    pub fn unwrap_ciphertext_block(self) -> EmulatedCiphertextBlock {
        match self {
            Self::CiphertextBlock(v) => v,
            _ => panic!("Expected CiphertextBlock, got {:?}", self),
        }
    }

    pub fn unwrap_plaintext_block(self) -> EmulatedPlaintextBlock {
        match self {
            Self::PlaintextBlock(v) => v,
            _ => panic!("Expected PlaintextBlock, got {:?}", self),
        }
    }
}

impl Debug for IopValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ciphertext(a) => a.fmt(f),
            Self::Plaintext(a) => a.fmt(f),
            Self::CiphertextBlock(a) => a.fmt(f),
            Self::PlaintextBlock(a) => a.fmt(f),
        }
    }
}

impl Interpretation for IopValue {}

impl InterpretsTo<IopValue> for super::IopTypeSystem {
    fn type_of(val: &IopValue) -> Self {
        match val {
            IopValue::Ciphertext(..) => Self::Ciphertext,
            IopValue::Plaintext(..) => Self::Plaintext,
            IopValue::CiphertextBlock(..) => Self::CiphertextBlock,
            IopValue::PlaintextBlock(..) => Self::PlaintextBlock,
        }
    }
}

#[derive(Debug)]
pub struct IopInterepreterContext {
    pub spec: CiphertextSpec,
    pub inputs: FastMap<usize, IopValue>,
    pub outputs: FastMap<usize, IopValue>,
}

impl Interpretable<IopValue> for super::IopInstructionSet {
    type Context = IopInterepreterContext;
    fn interpret(
        &self,
        context: &mut Self::Context,
        arguments: SmallVec<IopValue>,
    ) -> SmallVec<IopValue> {
        use super::IopInstructionSet::*;
        match self {
            Input { pos, .. } => {
                assert!(
                    context.inputs.contains_key(pos),
                    "Input {pos} is missing from context."
                );
                let input_value = context.inputs.get(pos).unwrap();
                svec![input_value.clone()]
            }
            Output { pos, .. } => {
                assert!(
                    !context.outputs.contains_key(pos),
                    "Output {pos} already returned in interpreter context."
                );
                context.outputs.insert(*pos, arguments[0].clone());
                svec![]
            }
            DeclareCiphertext => {
                svec![IopValue::Ciphertext(context.spec.from_int(0))]
            }
            LetPlaintextBlock { value } => {
                svec![IopValue::PlaintextBlock(
                    context
                        .spec
                        .matching_plaintext_spec()
                        .block_spec()
                        .from_message(*value as EmulatedPlaintextBlockStorage)
                )]
            }
            LetCiphertextBlock { value } => {
                svec![IopValue::CiphertextBlock(
                    context
                        .spec
                        .block_spec()
                        .from_message(*value as EmulatedCiphertextBlockStorage)
                )]
            }
            AddCt => {
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_add(right))]
            }
            WrappingAddCt => {
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.wrapping_add(right))]
            }
            TemperAddCt => {
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.temper_add(right))]
            }
            SubCt => {
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_sub(right))]
            }
            PackCt { mul } => {
                assert_eq!(
                    *mul as EmulatedPlaintextBlockStorage,
                    (2 as EmulatedPlaintextBlockStorage)
                        .pow(context.spec.block_spec().message_size() as u32)
                );
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(
                    left.protect_shl(context.spec.block_spec().message_size())
                        .protect_add(right)
                )]
            }
            AddPt => {
                let (IopValue::CiphertextBlock(left), IopValue::PlaintextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, PlaintextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_add_pt(right))]
            }
            WrappingAddPt => {
                let (IopValue::CiphertextBlock(left), IopValue::PlaintextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, PlaintextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.wrapping_add_pt(right))]
            }

            SubPt => {
                let (IopValue::CiphertextBlock(left), IopValue::PlaintextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, PlaintextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_sub_pt(right))]
            }
            PtSub => {
                let (IopValue::PlaintextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (PlaintextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_sub_ct(right))]
            }
            MulPt => {
                let (IopValue::CiphertextBlock(left), IopValue::PlaintextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, PlaintextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_mul_pt(right))]
            }
            ExtractCtBlock { index } => {
                let IopValue::Ciphertext(ct) = arguments[0] else {
                    panic!("Expected Ciphertext, got:\n{:#?}", arguments)
                };
                svec![IopValue::CiphertextBlock(ct.get_block(*index))]
            }
            ExtractPtBlock { index } => {
                let IopValue::Plaintext(pt) = arguments[0] else {
                    panic!("Expected Plaintext, got:\n{:#?}", arguments)
                };
                svec![IopValue::PlaintextBlock(pt.get_block(*index))]
            }
            StoreCtBlock { index } => {
                let (IopValue::CiphertextBlock(ctblock), IopValue::Ciphertext(mut ct)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (Ciphertext, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                ct.set_block(*index, ctblock);
                svec![IopValue::Ciphertext(ct)]
            }
            Pbs { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let ct0 = lut.protect_lookup(ct);
                svec![IopValue::CiphertextBlock(ct0)]
            }
            Pbs2 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1) = lut.protect_lookup(ct);
                svec![
                    IopValue::CiphertextBlock(ct0),
                    IopValue::CiphertextBlock(ct1)
                ]
            }
            Pbs4 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1, ct2, ct3) = lut.protect_lookup(ct);
                svec![
                    IopValue::CiphertextBlock(ct0),
                    IopValue::CiphertextBlock(ct1),
                    IopValue::CiphertextBlock(ct2),
                    IopValue::CiphertextBlock(ct3)
                ]
            }
            Pbs8 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1, ct2, ct3, ct4, ct5, ct6, ct7) = lut.protect_lookup(ct);
                svec![
                    IopValue::CiphertextBlock(ct0),
                    IopValue::CiphertextBlock(ct1),
                    IopValue::CiphertextBlock(ct2),
                    IopValue::CiphertextBlock(ct3),
                    IopValue::CiphertextBlock(ct4),
                    IopValue::CiphertextBlock(ct5),
                    IopValue::CiphertextBlock(ct6),
                    IopValue::CiphertextBlock(ct7)
                ]
            }
            WrappingPbs { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let ct0 = lut.wrapping_lookup(ct);
                svec![IopValue::CiphertextBlock(ct0)]
            }
            WrappingPbs2 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1) = lut.wrapping_lookup(ct);
                svec![
                    IopValue::CiphertextBlock(ct0),
                    IopValue::CiphertextBlock(ct1)
                ]
            }
            WrappingPbs4 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1, ct2, ct3) = lut.wrapping_lookup(ct);
                svec![
                    IopValue::CiphertextBlock(ct0),
                    IopValue::CiphertextBlock(ct1),
                    IopValue::CiphertextBlock(ct2),
                    IopValue::CiphertextBlock(ct3)
                ]
            }
            WrappingPbs8 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1, ct2, ct3, ct4, ct5, ct6, ct7) = lut.wrapping_lookup(ct);
                svec![
                    IopValue::CiphertextBlock(ct0),
                    IopValue::CiphertextBlock(ct1),
                    IopValue::CiphertextBlock(ct2),
                    IopValue::CiphertextBlock(ct3),
                    IopValue::CiphertextBlock(ct4),
                    IopValue::CiphertextBlock(ct5),
                    IopValue::CiphertextBlock(ct6),
                    IopValue::CiphertextBlock(ct7)
                ]
            }
            Alias { .. } => arguments,
        }
    }
}
