use zhc_abc as ABC;
use zhc_crypto::integer_semantics::SpecBlock;
use zhc_ir::{
    IR,
    evaluation::{EvalOutcome, Evaluable, EvaluatesTo, Evaluation},
    visualization::{DynamicElement, VisualAnnotation},
};
use zhc_langs::ioplang::{IopInstructionSet, IopLang, IopTypeSystem};
use zhc_utils::{
    Dumpable, FastMap, existential_enum, iter::CollectInSmallVec, small::SmallVec, svec,
};

use crate::symbolic::{Block, Bool, Circuit, Integer};

#[existential_enum]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum IopSymValue {
    CiphertextInteger(Integer),
    CiphertextBool(Bool),
    PlaintextInteger(Integer),
    CiphertextBlock(Block),
    PlaintextBlock(Block),
}

impl Evaluation for IopSymValue {}

impl Dumpable for IopSymValue {
    fn dump_to_string(&self) -> String {
        format!("{self:?}")
    }
}

impl VisualAnnotation for IopSymValue {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}

impl EvaluatesTo<IopSymValue> for IopTypeSystem {
    fn type_of(val: &IopSymValue) -> Self {
        match val {
            IopSymValue::CiphertextInteger(..) => Self::CiphertextInteger,
            IopSymValue::CiphertextBool(..) => Self::CiphertextBool,
            IopSymValue::PlaintextInteger(..) => Self::PlaintextInteger,
            IopSymValue::CiphertextBlock(..) => Self::CiphertextBlock,
            IopSymValue::PlaintextBlock(..) => Self::PlaintextBlock,
        }
    }
}

#[derive(Debug)]
pub struct IopSymContext {
    pub circuit: Circuit,
    pub inputs: FastMap<usize, IopSymValue>,
    pub outputs: FastMap<usize, IopSymValue>,
}

#[derive(Clone, Copy)]
enum InputKind {
    CiphertextInteger(u16),
    CiphertextBool,
    PlaintextInteger(u16),
}

impl IopSymContext {
    pub fn new(ir: &IR<IopLang>, spec: SpecBlock) -> Self {
        let mut slots: Vec<(usize, InputKind)> = ir
            .walk_ops_linear()
            .filter_map(|op| match op.get_instruction() {
                IopInstructionSet::InputCiphertextInteger { pos, int_size } => {
                    Some((*pos, InputKind::CiphertextInteger(*int_size)))
                }
                IopInstructionSet::InputCiphertextBool { pos } => {
                    Some((*pos, InputKind::CiphertextBool))
                }
                IopInstructionSet::InputPlaintextInteger { pos, int_size } => {
                    Some((*pos, InputKind::PlaintextInteger(*int_size)))
                }
                _ => None,
            })
            .collect();
        slots.sort_by_key(|(pos, _)| *pos);
        slots.dedup_by_key(|(pos, _)| *pos);

        let mut circuit = Circuit::new(spec);
        let mut inputs = FastMap::default();
        for (pos, kind) in slots {
            let value = match kind {
                InputKind::CiphertextInteger(int_size) => IopSymValue::CiphertextInteger(
                    circuit.input_integer(spec.integer_spec(int_size)),
                ),
                InputKind::CiphertextBool => IopSymValue::CiphertextBool(circuit.input_bool()),
                InputKind::PlaintextInteger(int_size) => IopSymValue::PlaintextInteger(
                    circuit.input_integer(spec.integer_spec(int_size)),
                ),
            };
            inputs.insert(pos, value);
        }
        Self {
            circuit,
            inputs,
            outputs: FastMap::default(),
        }
    }

    pub fn finish(mut self) -> ABC::Circuit {
        let mut positions: Vec<usize> = self.outputs.keys().copied().collect();
        positions.sort_unstable();
        for pos in positions {
            match &self.outputs[&pos] {
                IopSymValue::CiphertextInteger(ct) => self.circuit.output_integer(ct),
                IopSymValue::CiphertextBool(ct) => self.circuit.output_bool(ct),
                other => panic!("Output {pos} is not a ciphertext: {other:?}"),
            }
        }
        self.circuit.into_circuit()
    }
}

impl Evaluable<IopSymValue> for IopInstructionSet {
    type Context = IopSymContext;

    fn eval(
        &self,
        context: &mut Self::Context,
        arguments: SmallVec<&IopSymValue>,
    ) -> EvalOutcome<IopSymValue> {
        use IopInstructionSet::*;
        let ct = IopSymValue::CiphertextBlock;
        let c = &mut context.circuit;
        let results = match self {
            InputCiphertextInteger { pos, .. }
            | InputCiphertextBool { pos }
            | InputPlaintextInteger { pos, .. } => {
                let value = context
                    .inputs
                    .get(pos)
                    .unwrap_or_else(|| panic!("Input {pos} is missing from context."));
                svec![value.clone()]
            }
            OutputCiphertextInteger { pos } | OutputCiphertextBool { pos } => {
                assert!(
                    !context.outputs.contains_key(pos),
                    "Output {pos} already returned in context."
                );
                context.outputs.insert(*pos, arguments[0].clone());
                svec![]
            }
            _Consume { .. } => panic!("Tried to interpret a _consume operation"),
            Inspect { .. } | _Copy { .. } => arguments.iter().map(|a| (*a).clone()).cosvec(),
            DeclareCiphertextInteger { int_size } => {
                let n = c.spec().integer_spec(*int_size).block_count();
                let blocks = (0..n).map(|_| c.const_block(0)).collect();
                svec![IopSymValue::CiphertextInteger(Integer { blocks })]
            }
            LetPlaintextBlock { value } => {
                svec![IopSymValue::PlaintextBlock(c.const_block(*value as u16))]
            }
            LetCiphertextBlock { value } => svec![ct(c.const_block(*value as u16))],
            AddCt { .. } => svec![ct(c.add(
                arguments[0].unwrap_ciphertext_block_ref(),
                arguments[1].unwrap_ciphertext_block_ref()
            ))],
            AddPt { .. } => svec![ct(c.add(
                arguments[0].unwrap_ciphertext_block_ref(),
                arguments[1].unwrap_plaintext_block_ref()
            ))],
            SubCt { .. } => svec![ct(c.sub(
                arguments[0].unwrap_ciphertext_block_ref(),
                arguments[1].unwrap_ciphertext_block_ref()
            ))],
            SubPt { .. } => svec![ct(c.sub(
                arguments[0].unwrap_ciphertext_block_ref(),
                arguments[1].unwrap_plaintext_block_ref()
            ))],
            PtSub { .. } => svec![ct(c.sub(
                arguments[0].unwrap_plaintext_block_ref(),
                arguments[1].unwrap_ciphertext_block_ref()
            ))],
            MulPt { .. } => svec![ct(c.mul(
                arguments[0].unwrap_ciphertext_block_ref(),
                arguments[1].unwrap_plaintext_block_ref()
            ))],
            ShlCt { amount, .. } => {
                svec![ct(
                    c.shl(arguments[0].unwrap_ciphertext_block_ref(), *amount)
                )]
            }
            PackCt { mul, .. } => svec![ct(c.mac(
                arguments[0].unwrap_ciphertext_block_ref(),
                *mul as u16,
                arguments[1].unwrap_ciphertext_block_ref()
            ))],
            ExtractCiphertextIntegerBlock { index } => {
                let value = arguments[0].unwrap_ciphertext_integer_ref();
                svec![ct(value.blocks[*index as usize].clone())]
            }
            ExtractPlaintextIntegerBlock { index } => {
                let value = arguments[0].unwrap_plaintext_integer_ref();
                svec![IopSymValue::PlaintextBlock(
                    value.blocks[*index as usize].clone()
                )]
            }
            StoreCiphertextIntegerBlock { index } => {
                let block = arguments[0].unwrap_ciphertext_block_ref();
                let mut value = arguments[1].unwrap_ciphertext_integer_ref().clone();
                value.blocks[*index as usize] = block.clone();
                svec![IopSymValue::CiphertextInteger(value)]
            }
            StoreCiphertextBoolBlock => svec![IopSymValue::CiphertextBool(Bool {
                block: arguments[0].unwrap_ciphertext_block_ref().clone()
            })],
            ExtractCiphertextBoolBlock => {
                svec![ct(arguments[0].unwrap_ciphertext_bool_ref().block.clone())]
            }
            Pbs { lut, .. } => svec![ct(c.lut1(lut, arguments[0].unwrap_ciphertext_block_ref()))],
            Pbs2 { lut, .. } => c
                .lut2(lut, arguments[0].unwrap_ciphertext_block_ref())
                .into_iter()
                .map(ct)
                .cosvec(),
            Pbs4 { lut, .. } => c
                .lut4(lut, arguments[0].unwrap_ciphertext_block_ref())
                .into_iter()
                .map(ct)
                .cosvec(),
            Pbs8 { lut, .. } => c
                .lut8(lut, arguments[0].unwrap_ciphertext_block_ref())
                .into_iter()
                .map(ct)
                .cosvec(),
        };
        EvalOutcome::Evaluated(results)
    }
}

pub fn build_circuit(ir: &IR<IopLang>, spec: SpecBlock) -> ABC::Circuit {
    let mut context = IopSymContext::new(ir, spec);
    if let Err(eval_ir) = ir.evaluate::<IopSymValue>(&mut context) {
        eval_ir.dump_and_panic();
    }
    context.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc_abc::{CheckOptions, Verdict, check_equivalence};
    use zhc_builder::{add, sub};
    use zhc_crypto::integer_semantics::SpecInteger;

    #[test]
    fn add_has_the_expected_shape() {
        let spec = SpecBlock(2, 2);
        let ir = add(SpecInteger::new(8, 2, 2)).optimize_ir();
        let circuit = build_circuit(&ir, spec);
        assert_eq!(circuit.n_inputs(), 2 * 4 * spec.message_size() as usize);
        assert_eq!(circuit.n_outputs(), 4 * spec.complete_size() as usize);
        assert!(circuit.n_ands() > 0);
    }

    #[test]
    fn add_matches_a_ripple_adder_with_clean_carries() {
        let spec = SpecBlock(2, 2);
        let ir = add(SpecInteger::new(16, 2, 2)).optimize_ir();
        let left = build_circuit(&ir, spec);

        let mut right = ABC::Circuit::new();
        let a: Vec<ABC::Lit> = (0..16).map(|_| right.input()).collect();
        let b: Vec<ABC::Lit> = (0..16).map(|_| right.input()).collect();
        let mut sum = Vec::new();
        let mut carry = right.constant(false);
        for i in 0..16 {
            let axb = right.xor(a[i], b[i]);
            sum.push(right.xor(axb, carry));
            let t = right.and(a[i], b[i]);
            let u = right.and(carry, axb);
            carry = right.or(t, u);
        }
        for block in 0..8 {
            right.output(sum[2 * block]);
            right.output(sum[2 * block + 1]);
            right.output(right.constant(false));
            right.output(right.constant(false));
            right.output(right.constant(false));
        }

        let verdict = check_equivalence(&left, &right, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn add_differs_from_sub() {
        let spec = SpecBlock(2, 2);
        let left = build_circuit(&add(SpecInteger::new(8, 2, 2)).optimize_ir(), spec);
        let right = build_circuit(&sub(SpecInteger::new(8, 2, 2)).optimize_ir(), spec);
        let verdict = check_equivalence(&left, &right, &CheckOptions::default()).unwrap();
        assert!(matches!(verdict, Verdict::NotEquivalent(_)));
    }
}
