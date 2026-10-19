use std::fmt::Debug;
use zhc_crypto::integer_semantics::{
    EmulatedCiphertextBlock, EmulatedCiphertextBool, EmulatedCiphertextInteger,
    EmulatedPlaintextBlock, EmulatedPlaintextInteger, SpecBlock,
};
use zhc_ir::evaluation::{EvalOutcome, Evaluable, EvaluatesTo, Evaluation};
use zhc_ir::visualization::{DynamicElement, VisualAnnotation};
use zhc_utils::iter::CollectInSmallVec;
use zhc_utils::small::SmallVec;
use zhc_utils::{Dumpable, FastMap, SafeAs, svec};

/// Interpretation domain for IOP programs.
///
/// Wraps `zhc_crypto` emulated values so that an `IR<IopLang>` can be
/// executed via the `zhc_ir` interpretation framework. Each variant
/// corresponds to the matching [`IopTypeSystem`](super::IopTypeSystem)
/// type.
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum IopValue {
    CiphertextInteger(EmulatedCiphertextInteger),
    CiphertextBool(EmulatedCiphertextBool),
    PlaintextInteger(EmulatedPlaintextInteger),
    CiphertextBlock(EmulatedCiphertextBlock),
    PlaintextBlock(EmulatedPlaintextBlock),
}

impl IopValue {
    /// Extracts the inner `EmulatedCiphertextInteger`.
    ///
    /// # Panics
    ///
    /// Panics if self is not the `CiphertextInteger` variant.
    pub fn unwrap_ciphertext(self) -> EmulatedCiphertextInteger {
        match self {
            Self::CiphertextInteger(v) => v,
            _ => panic!("Expected CiphertextInteger, got {:?}", self),
        }
    }

    /// Extracts the inner `EmulatedCiphertextBool`.
    pub fn unwrap_ciphertext_bool(self) -> EmulatedCiphertextBool {
        match self {
            Self::CiphertextBool(v) => v,
            _ => panic!("Expected CiphertextBool, got {:?}", self),
        }
    }

    /// Extracts the inner `EmulatedPlaintextInteger`.
    ///
    /// # Panics
    ///
    /// Panics if self is not the `PlaintextInteger` variant.
    pub fn unwrap_plaintext(self) -> EmulatedPlaintextInteger {
        match self {
            Self::PlaintextInteger(v) => v,
            _ => panic!("Expected PlaintextInteger, got {:?}", self),
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
            Self::CiphertextInteger(a) => a.fmt(f),
            Self::CiphertextBool(a) => a.fmt(f),
            Self::PlaintextInteger(a) => a.fmt(f),
            Self::CiphertextBlock(a) => a.fmt(f),
            Self::PlaintextBlock(a) => a.fmt(f),
        }
    }
}

impl Dumpable for IopValue {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Evaluation for IopValue {}

impl VisualAnnotation for IopValue {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}

impl EvaluatesTo<IopValue> for super::IopTypeSystem {
    fn type_of(val: &IopValue) -> Self {
        match val {
            IopValue::CiphertextInteger(..) => Self::CiphertextInteger,
            IopValue::CiphertextBool(..) => Self::CiphertextBool,
            IopValue::PlaintextInteger(..) => Self::PlaintextInteger,
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
    pub spec: SpecBlock,
    pub inputs: FastMap<usize, IopValue>,
    pub outputs: FastMap<usize, IopValue>,
}

/// Unwraps the argument at `idx` as a ciphertext block.
fn ct_arg(arguments: &SmallVec<&IopValue>, idx: usize) -> EmulatedCiphertextBlock {
    match arguments[idx] {
        IopValue::CiphertextBlock(ct) => *ct,
        _ => panic!(
            "Expected CiphertextBlock at argument {idx}, got:\n{:#?}",
            arguments
        ),
    }
}

/// Unwraps the argument at `idx` as a plaintext block.
fn pt_arg(arguments: &SmallVec<&IopValue>, idx: usize) -> EmulatedPlaintextBlock {
    match arguments[idx] {
        IopValue::PlaintextBlock(pt) => *pt,
        _ => panic!(
            "Expected PlaintextBlock at argument {idx}, got:\n{:#?}",
            arguments
        ),
    }
}

impl Evaluable<IopValue> for super::IopInstructionSet {
    type Context = IopInterepreterContext;
    fn eval(
        &self,
        context: &mut Self::Context,
        arguments: SmallVec<&IopValue>,
    ) -> EvalOutcome<IopValue> {
        use super::IopInstructionSet::*;
        let ct = |v: EmulatedCiphertextBlock| IopValue::CiphertextBlock(v);
        let results = match self {
            InputCiphertextInteger { pos, int_size } => {
                assert!(
                    context.inputs.contains_key(pos),
                    "Input {pos} is missing from context."
                );
                let input_value = context.inputs.get(pos).unwrap();
                let IopValue::CiphertextInteger(ct) = input_value else {
                    panic!("Expected CiphertextInteger, got:\n{:#?}", input_value);
                };
                assert_eq!(
                    context.spec.integer_spec(*int_size),
                    ct.spec(),
                    "Spec mismatch."
                );
                svec![input_value.clone()]
            }
            InputCiphertextBool { pos } => {
                assert!(
                    context.inputs.contains_key(pos),
                    "Input {pos} is missing from context."
                );
                let input_value = context.inputs.get(pos).unwrap();
                let IopValue::CiphertextBool(ct) = input_value else {
                    panic!("Expected CiphertextBool, got:\n{:#?}", input_value);
                };
                assert_eq!(context.spec, ct.spec(), "Spec mismatch.");
                svec![input_value.clone()]
            }
            InputPlaintextInteger { pos, int_size } => {
                assert!(
                    context.inputs.contains_key(pos),
                    "Input {pos} is missing from context."
                );
                let input_value = context.inputs.get(pos).unwrap();
                let IopValue::PlaintextInteger(pt) = input_value else {
                    panic!("Expected PlaintextInteger, got:\n{:#?}", input_value);
                };
                assert_eq!(
                    context.spec.integer_spec(*int_size),
                    pt.spec(),
                    "Spec mismatch"
                );
                svec![input_value.clone()]
            }
            OutputCiphertextInteger { pos, .. } | OutputCiphertextBool { pos, .. } => {
                assert!(
                    !context.outputs.contains_key(pos),
                    "Output {pos} already returned in interpreter context."
                );
                context.outputs.insert(*pos, arguments[0].clone());
                svec![]
            }
            _Consume { .. } => panic!("Tried to interpret a _consume operation"),
            Inspect { .. } | _Copy { .. } => arguments.iter().map(|a| (*a).clone()).cosvec(),
            DeclareCiphertextInteger { int_size } => {
                svec![IopValue::CiphertextInteger(
                    context.spec.integer_spec(*int_size).from_int(0)
                )]
            }
            LetPlaintextBlock { value } => {
                svec![IopValue::PlaintextBlock(
                    EmulatedPlaintextBlock::from_complete(context.spec, (*value).sas())
                )]
            }
            LetCiphertextBlock { value } => {
                svec![ct(context.spec.from_complete((*value).sas()))]
            }
            AddCt { flavor } => {
                svec![ct(ct_arg(&arguments, 0).add(ct_arg(&arguments, 1), *flavor))]
            }
            SubCt { flavor } => {
                svec![ct(ct_arg(&arguments, 0).sub(ct_arg(&arguments, 1), *flavor))]
            }
            ShlCt { amount, flavor } => svec![ct(ct_arg(&arguments, 0).shl(*amount, *flavor))],
            PackCt { mul, flavor } => {
                svec![ct(ct_arg(&arguments, 0).mac(
                    *mul,
                    ct_arg(&arguments, 1),
                    *flavor
                ))]
            }
            AddPt { flavor } => {
                svec![ct(
                    ct_arg(&arguments, 0).add_pt(pt_arg(&arguments, 1), *flavor)
                )]
            }
            SubPt { flavor } => {
                svec![ct(
                    ct_arg(&arguments, 0).sub_pt(pt_arg(&arguments, 1), *flavor)
                )]
            }
            PtSub { flavor } => {
                svec![ct(
                    pt_arg(&arguments, 0).sub_ct(ct_arg(&arguments, 1), *flavor)
                )]
            }
            MulPt { flavor } => {
                svec![ct(
                    ct_arg(&arguments, 0).mul_pt(pt_arg(&arguments, 1), *flavor)
                )]
            }
            ExtractCiphertextIntegerBlock { index } => {
                let IopValue::CiphertextInteger(ct) = arguments[0] else {
                    panic!("Expected CiphertextInteger, got:\n{:#?}", arguments)
                };
                svec![IopValue::CiphertextBlock(ct.get_block(*index))]
            }
            ExtractPlaintextIntegerBlock { index } => {
                let IopValue::PlaintextInteger(pt) = arguments[0] else {
                    panic!("Expected PlaintextInteger, got:\n{:#?}", arguments)
                };
                svec![IopValue::PlaintextBlock(pt.get_block(*index))]
            }
            StoreCiphertextIntegerBlock { index } => {
                let (IopValue::CiphertextBlock(ctblock), IopValue::CiphertextInteger(mut ct)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "Expected (CiphertextBlock, CiphertextInteger), got:\n{:#?}",
                        arguments
                    )
                };
                ct.set_block(*index, ctblock);
                svec![IopValue::CiphertextInteger(ct)]
            }
            StoreCiphertextBoolBlock => svec![IopValue::CiphertextBool(
                EmulatedCiphertextBool::from_block(ct_arg(&arguments, 0),)
            )],
            ExtractCiphertextBoolBlock => {
                let IopValue::CiphertextBool(ct) = arguments[0] else {
                    panic!("Expected CiphertextBool, got:\n{:#?}", arguments)
                };
                svec![IopValue::CiphertextBlock(ct.get_block())]
            }
            Pbs { check, lut } => svec![ct(lut.lookup(ct_arg(&arguments, 0), *check))],
            Pbs2 { check, lut } => {
                let (o0, o1) = lut.lookup(ct_arg(&arguments, 0), *check);
                svec![ct(o0), ct(o1)]
            }
            Pbs4 { check, lut } => {
                let (o0, o1, o2, o3) = lut.lookup(ct_arg(&arguments, 0), *check);
                svec![ct(o0), ct(o1), ct(o2), ct(o3)]
            }
            Pbs8 { check, lut } => {
                let (o0, o1, o2, o3, o4, o5, o6, o7) = lut.lookup(ct_arg(&arguments, 0), *check);
                svec![
                    ct(o0),
                    ct(o1),
                    ct(o2),
                    ct(o3),
                    ct(o4),
                    ct(o5),
                    ct(o6),
                    ct(o7)
                ]
            }
        };
        EvalOutcome::Evaluated(results)
    }
}
