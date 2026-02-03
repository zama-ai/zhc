use hc_crypto::integer_semantics::{
    Ciphertext, CiphertextBlock, CiphertextSpec, Plaintext, PlaintextBlock, PlaintextBlockStorage,
};
use hc_ir::interpretation::{Interpretable, Interpretation, InterpretsTo};
use hc_utils::small::SmallVec;
use hc_utils::{FastMap, svec};
use std::fmt::Debug;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum IopValue {
    Ciphertext(Ciphertext),
    Plaintext(Plaintext),
    CiphertextBlock(CiphertextBlock),
    PlaintextBlock(PlaintextBlock),
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
            ZeroCiphertext => {
                svec![IopValue::Ciphertext(context.spec.from_int(0))]
            }
            LetPlaintextBlock { value } => {
                svec![IopValue::PlaintextBlock(
                    context
                        .spec
                        .matching_plaintext_spec()
                        .block_spec()
                        .from_message(*value)
                )]
            }
            AddCt => {
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "AddCt: expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_add(right))]
            }
            SubCt => {
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "SubCt: expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_sub(right))]
            }
            PackCt { mul } => {
                assert_eq!(
                    *mul,
                    (2 as PlaintextBlockStorage)
                        .pow(context.spec.block_spec().message_size() as u32)
                );
                let (IopValue::CiphertextBlock(left), IopValue::CiphertextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "PackCt: expected (CiphertextBlock, CiphertextBlock), got:\n{:#?}",
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
                        "AddPt: expected (CiphertextBlock, PlaintextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_add_pt(right))]
            }
            SubPt => {
                let (IopValue::CiphertextBlock(left), IopValue::PlaintextBlock(right)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "SubPt: expected (CiphertextBlock, PlaintextBlock), got:\n{:#?}",
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
                        "PtSub: expected (PlaintextBlock, CiphertextBlock), got:\n{:#?}",
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
                        "MulPt: expected (CiphertextBlock, PlaintextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(left.protect_mul_pt(right))]
            }
            ExtractCtBlock { index } => {
                let IopValue::Ciphertext(ct) = arguments[0] else {
                    panic!(
                        "ExtractCtBlock: expected Ciphertext, got:\n{:#?}",
                        arguments
                    )
                };
                svec![IopValue::CiphertextBlock(ct.get_block(*index))]
            }
            ExtractPtBlock { index } => {
                let IopValue::Plaintext(pt) = arguments[0] else {
                    panic!("ExtractPtBlock: expected Plaintext, got:\n{:#?}", arguments)
                };
                svec![IopValue::PlaintextBlock(pt.get_block(*index))]
            }
            StoreCtBlock { index } => {
                let (IopValue::CiphertextBlock(ctblock), IopValue::Ciphertext(mut ct)) =
                    (arguments[0].clone(), arguments[1].clone())
                else {
                    panic!(
                        "StoreCtBlock: expected (Ciphertext, CiphertextBlock), got:\n{:#?}",
                        arguments
                    )
                };
                ct.set_block(*index, ctblock);
                svec![IopValue::Ciphertext(ct)]
            }
            Pbs { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Pbs: expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let lut0 = lut.get_fn();
                svec![IopValue::CiphertextBlock(lut0(ct))]
            }
            Pbs2 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Pbs2: expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (lut0, lut1) = lut.get_fns();
                svec![
                    IopValue::CiphertextBlock(lut0(ct)),
                    IopValue::CiphertextBlock(lut1(ct))
                ]
            }
            Pbs4 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Pbs4: expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (lut0, lut1, lut2, lut3) = lut.get_fns();
                svec![
                    IopValue::CiphertextBlock(lut0(ct)),
                    IopValue::CiphertextBlock(lut1(ct)),
                    IopValue::CiphertextBlock(lut2(ct)),
                    IopValue::CiphertextBlock(lut3(ct))
                ]
            }
            Pbs8 { lut } => {
                let IopValue::CiphertextBlock(ct) = arguments[0] else {
                    panic!("Pbs8: expected CiphertextBlock, got:\n{:#?}", arguments)
                };
                let (lut0, lut1, lut2, lut3, lut4, lut5, lut6, lut7) = lut.get_fns();
                svec![
                    IopValue::CiphertextBlock(lut0(ct)),
                    IopValue::CiphertextBlock(lut1(ct)),
                    IopValue::CiphertextBlock(lut2(ct)),
                    IopValue::CiphertextBlock(lut3(ct)),
                    IopValue::CiphertextBlock(lut4(ct)),
                    IopValue::CiphertextBlock(lut5(ct)),
                    IopValue::CiphertextBlock(lut6(ct)),
                    IopValue::CiphertextBlock(lut7(ct))
                ]
            }
        }
    }
}
