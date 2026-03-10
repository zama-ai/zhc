use std::fmt::Debug;
use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, EmulatedCiphertext, EmulatedCiphertextBlock,
    EmulatedCiphertextBlockStorage, EmulatedPlaintext, EmulatedPlaintextBlock,
    EmulatedPlaintextBlockStorage,
};
use zhc_ir::interpretation::{Interpretable, Interpretation, InterpretsTo};
use zhc_utils::small::SmallVec;
use zhc_utils::{FastMap, svec};

/// Interpretation domain for IOP programs.
///
/// Wraps `zhc_crypto` emulated values so that an `IR<IopLang>` can be
/// executed via the `zhc_ir` interpretation framework. Each variant
/// corresponds to the matching [`IopTypeSystem`](super::IopTypeSystem)
/// type.
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum IopValue {
    Ciphertext(EmulatedCiphertext),
    Plaintext(EmulatedPlaintext),
    CiphertextBlock(EmulatedCiphertextBlock),
    PlaintextBlock(EmulatedPlaintextBlock),
}

impl IopValue {
    /// Extracts the inner `EmulatedCiphertext`.
    ///
    /// # Panics
    ///
    /// Panics if self is not the `Ciphertext` variant.
    pub fn unwrap_ciphertext(self) -> EmulatedCiphertext {
        match self {
            Self::Ciphertext(v) => v,
            _ => panic!("Expected Ciphertext, got {:?}", self),
        }
    }

    /// Extracts the inner `EmulatedPlaintext`.
    ///
    /// # Panics
    ///
    /// Panics if self is not the `Plaintext` variant.
    pub fn unwrap_plaintext(self) -> EmulatedPlaintext {
        match self {
            Self::Plaintext(v) => v,
            _ => panic!("Expected Plaintext, got {:?}", self),
        }
    }

    /// Extracts the inner `EmulatedCiphertextBlock`.
    ///
    /// # Panics
    ///
    /// Panics if self is not the `CiphertextBlock` variant.
    pub fn unwrap_ciphertext_block(self) -> EmulatedCiphertextBlock {
        match self {
            Self::CiphertextBlock(v) => v,
            _ => panic!("Expected CiphertextBlock, got {:?}", self),
        }
    }

    /// Extracts the inner `EmulatedPlaintextBlock`.
    ///
    /// # Panics
    ///
    /// Panics if self is not the `PlaintextBlock` variant.
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

/// Execution context for IOP program interpretation.
///
/// Holds the cryptographic parameters (`spec`), the program inputs
/// keyed by positional slot, and the outputs collected during execution.
/// The `inputs` map must contain an entry for every `Input` operation in
/// the IR, with values whose types match the builder signature. The
/// `outputs` map should be empty before interpretation; each `Output`
/// operation inserts its value at the corresponding slot.
///
/// # Panics
///
/// Interpretation panics if an `Input` slot is missing from `inputs`,
/// if an `Output` slot is written twice, or if an input value does not
/// match the expected type.
#[derive(Debug)]
pub struct IopInterepreterContext {
    pub spec: CiphertextBlockSpec,
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
            InputCiphertext { pos, int_size } => {
                assert!(
                    context.inputs.contains_key(pos),
                    "Input {pos} is missing from context."
                );
                let input_value = context.inputs.get(pos).unwrap();
                let IopValue::Ciphertext(ct) = input_value else {
                    panic!("Expected Ciphertext, got:\n{:#?}", input_value);
                };
                assert_eq!(
                    context.spec.ciphertext_spec(*int_size),
                    ct.spec(),
                    "Spec mismatch."
                );
                svec![input_value.clone()]
            }
            InputPlaintext { pos, int_size } => {
                assert!(
                    context.inputs.contains_key(pos),
                    "Input {pos} is missing from context."
                );
                let input_value = context.inputs.get(pos).unwrap();
                let IopValue::Plaintext(pt) = input_value else {
                    panic!("Expected Planitext, got:\n{:#?}", input_value);
                };
                assert_eq!(
                    context
                        .spec
                        .matching_plaintext_block_spec()
                        .plaintext_spec(*int_size),
                    pt.spec(),
                    "Spec mismatch"
                );
                svec![input_value.clone()]
            }
            OutputCiphertext { pos, .. } => {
                assert!(
                    !context.outputs.contains_key(pos),
                    "Output {pos} already returned in interpreter context."
                );
                context.outputs.insert(*pos, arguments[0].clone());
                svec![]
            }
            _Consume { .. } => panic!("Tried to interpret a _consume operation"),
            DeclareCiphertext { int_size } => {
                svec![IopValue::Ciphertext(
                    context.spec.ciphertext_spec(*int_size).from_int(0)
                )]
            }
            LetPlaintextBlock { value } => {
                svec![IopValue::PlaintextBlock(
                    context
                        .spec
                        .matching_plaintext_block_spec()
                        .from_message(*value as EmulatedPlaintextBlockStorage)
                )]
            }
            LetCiphertextBlock { value } => {
                svec![IopValue::CiphertextBlock(
                    context
                        .spec
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
                    (2 as EmulatedPlaintextBlockStorage).pow(context.spec.message_size() as u32)
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
                    left.protect_shl(context.spec.message_size())
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
            Pbs { check, lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let ct0 = lut.lookup(ct, *check);
                svec![IopValue::CiphertextBlock(ct0)]
            }
            Pbs2 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1) = lut.lookup(ct);
                svec![
                    IopValue::CiphertextBlock(ct0),
                    IopValue::CiphertextBlock(ct1)
                ]
            }
            Pbs4 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (ct0, ct1, ct2, ct3) = lut.lookup(ct);
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
                let (ct0, ct1, ct2, ct3, ct4, ct5, ct6, ct7) = lut.lookup(ct);
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
            Inspect { .. } | Transfer => arguments,
            TransferIn { .. } | TransferOut { .. } => {
                panic!("Interpretation of multi-hpu graphs is not supported.")
            }
        }
    }
}
