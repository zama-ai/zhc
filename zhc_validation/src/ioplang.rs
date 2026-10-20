use zhc_abc::Circuit;
use zhc_crypto::integer_semantics::CiphertextBlockSpec;
use zhc_ir::{
    IR,
    evaluation::{EvalOutcome, Evaluable, EvaluatesTo, Evaluation},
    visualization::{DynamicElement, VisualAnnotation},
};
use zhc_langs::ioplang::{IopInstructionSet, IopLang, IopTypeSystem};
use zhc_utils::{Dumpable, FastMap, iter::CollectInSmallVec, small::SmallVec, svec};

use crate::symbolic::{
    SymCiphertext, SymCiphertextBlock, SymCircuit, SymPlaintext, SymPlaintextBlock,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum IopSymValue {
    Ciphertext(SymCiphertext),
    Plaintext(SymPlaintext),
    CiphertextBlock(SymCiphertextBlock),
    PlaintextBlock(SymPlaintextBlock),
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
            IopSymValue::Ciphertext(..) => Self::Ciphertext,
            IopSymValue::Plaintext(..) => Self::Plaintext,
            IopSymValue::CiphertextBlock(..) => Self::CiphertextBlock,
            IopSymValue::PlaintextBlock(..) => Self::PlaintextBlock,
        }
    }
}

#[derive(Debug)]
pub struct IopSymContext {
    pub circuit: SymCircuit,
    pub inputs: FastMap<usize, IopSymValue>,
    pub outputs: FastMap<usize, IopSymValue>,
}

impl IopSymContext {
    pub fn new(ir: &IR<IopLang>, spec: CiphertextBlockSpec) -> Self {
        let mut slots: Vec<(usize, bool, u16)> = ir
            .walk_ops_linear()
            .filter_map(|op| match op.get_instruction() {
                IopInstructionSet::InputCiphertext { pos, int_size } => {
                    Some((*pos, true, *int_size))
                }
                IopInstructionSet::InputPlaintext { pos, int_size } => {
                    Some((*pos, false, *int_size))
                }
                _ => None,
            })
            .collect();
        slots.sort_by_key(|(pos, _, _)| *pos);
        slots.dedup_by_key(|(pos, _, _)| *pos);

        let mut circuit = SymCircuit::new(spec);
        let mut inputs = FastMap::default();
        for (pos, is_ct, int_size) in slots {
            let ct_spec = spec.ciphertext_spec(int_size);
            let value = if is_ct {
                IopSymValue::Ciphertext(circuit.input_ciphertext(ct_spec))
            } else {
                IopSymValue::Plaintext(circuit.input_plaintext(ct_spec.matching_plaintext_spec()))
            };
            inputs.insert(pos, value);
        }
        Self {
            circuit,
            inputs,
            outputs: FastMap::default(),
        }
    }

    pub fn finish(mut self) -> Circuit {
        let mut positions: Vec<usize> = self.outputs.keys().copied().collect();
        positions.sort_unstable();
        for pos in positions {
            let IopSymValue::Ciphertext(ct) = &self.outputs[&pos] else {
                panic!("Output {pos} is not a ciphertext");
            };
            self.circuit.output_ciphertext(ct);
        }
        self.circuit.into_circuit()
    }
}

fn ct_arg<'a>(arguments: &'a SmallVec<&IopSymValue>, idx: usize) -> &'a SymCiphertextBlock {
    match arguments[idx] {
        IopSymValue::CiphertextBlock(b) => b,
        other => panic!("Expected CiphertextBlock at argument {idx}, got {other:?}"),
    }
}

fn pt_arg<'a>(arguments: &'a SmallVec<&IopSymValue>, idx: usize) -> &'a SymPlaintextBlock {
    match arguments[idx] {
        IopSymValue::PlaintextBlock(b) => b,
        other => panic!("Expected PlaintextBlock at argument {idx}, got {other:?}"),
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
            InputCiphertext { pos, .. } | InputPlaintext { pos, .. } => {
                let value = context
                    .inputs
                    .get(pos)
                    .unwrap_or_else(|| panic!("Input {pos} is missing from context."));
                svec![value.clone()]
            }
            OutputCiphertext { pos } => {
                assert!(
                    !context.outputs.contains_key(pos),
                    "Output {pos} already returned in context."
                );
                context.outputs.insert(*pos, arguments[0].clone());
                svec![]
            }
            _Consume { .. } => panic!("Tried to interpret a _consume operation"),
            Inspect { .. } | _Copy { .. } => arguments.iter().map(|a| (*a).clone()).cosvec(),
            DeclareCiphertext { int_size } => {
                let spec = c.spec().ciphertext_spec(*int_size);
                svec![IopSymValue::Ciphertext(c.declare_ciphertext(spec))]
            }
            LetPlaintextBlock { value } => {
                let spec = c.spec().complete_plaintext_block_spec();
                svec![IopSymValue::PlaintextBlock(
                    c.plaintext_block_from_message(spec, *value as u16)
                )]
            }
            LetCiphertextBlock { value } => {
                svec![ct(c.ciphertext_block_from_complete(*value as u16))]
            }
            AddCt { .. } => svec![ct(c.add(ct_arg(&arguments, 0), ct_arg(&arguments, 1)))],
            SubCt { .. } => svec![ct(c.sub(ct_arg(&arguments, 0), ct_arg(&arguments, 1)))],
            ShlCt { amount, .. } => svec![ct(c.shl(ct_arg(&arguments, 0), *amount))],
            PackCt { mul, .. } => {
                svec![ct(c.mac(
                    ct_arg(&arguments, 0),
                    *mul,
                    ct_arg(&arguments, 1)
                ))]
            }
            AddPt { .. } => svec![ct(c.add_pt(ct_arg(&arguments, 0), pt_arg(&arguments, 1)))],
            SubPt { .. } => svec![ct(c.sub_pt(ct_arg(&arguments, 0), pt_arg(&arguments, 1)))],
            PtSub { .. } => svec![ct(c.pt_sub(pt_arg(&arguments, 0), ct_arg(&arguments, 1)))],
            MulPt { .. } => svec![ct(c.mul_pt(ct_arg(&arguments, 0), pt_arg(&arguments, 1)))],
            ExtractCtBlock { index } => {
                let IopSymValue::Ciphertext(value) = arguments[0] else {
                    panic!("Expected Ciphertext, got {:?}", arguments[0]);
                };
                svec![ct(value.get_block(*index))]
            }
            ExtractPtBlock { index } => {
                let IopSymValue::Plaintext(value) = arguments[0] else {
                    panic!("Expected Plaintext, got {:?}", arguments[0]);
                };
                svec![IopSymValue::PlaintextBlock(value.get_block(*index))]
            }
            StoreCtBlock { index } => {
                let (IopSymValue::CiphertextBlock(block), IopSymValue::Ciphertext(value)) =
                    (arguments[0], arguments[1])
                else {
                    panic!("Expected (CiphertextBlock, Ciphertext), got {arguments:?}");
                };
                let mut value = value.clone();
                value.set_block(*index, block.clone());
                svec![IopSymValue::Ciphertext(value)]
            }
            Pbs { lut, .. } => svec![ct(c.lut1(lut, ct_arg(&arguments, 0)))],
            Pbs2 { lut, .. } => c
                .lut2(lut, ct_arg(&arguments, 0))
                .into_iter()
                .map(ct)
                .cosvec(),
            Pbs4 { lut, .. } => c
                .lut4(lut, ct_arg(&arguments, 0))
                .into_iter()
                .map(ct)
                .cosvec(),
            Pbs8 { lut, .. } => c
                .lut8(lut, ct_arg(&arguments, 0))
                .into_iter()
                .map(ct)
                .cosvec(),
        };
        EvalOutcome::Evaluated(results)
    }
}

pub fn build_circuit(ir: &IR<IopLang>, spec: CiphertextBlockSpec) -> Circuit {
    let mut context = IopSymContext::new(ir, spec);
    if let Err(eval_ir) = ir.evaluate::<IopSymValue>(&mut context) {
        eval_ir.dump_and_panic();
    }
    context.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc_abc::{CheckOptions, Lit, Verdict, check_equivalence};
    use zhc_builder::{CiphertextSpec, add, sub};

    const SPEC: CiphertextBlockSpec = CiphertextBlockSpec(2, 2);

    fn ripple_adder(c: &mut Circuit, a: &[Lit], b: &[Lit]) -> Vec<Lit> {
        let mut carry = c.constant(false);
        let mut out = Vec::new();
        for i in 0..a.len() {
            let axb = c.xor(a[i], b[i]);
            out.push(c.xor(axb, carry));
            let t = c.and(a[i], b[i]);
            let u = c.and(carry, axb);
            carry = c.or(t, u);
        }
        out
    }

    #[test]
    fn add_has_the_expected_shape() {
        let ir = add(CiphertextSpec::new(8, 2, 2)).optimize_ir();
        let circuit = build_circuit(&ir, SPEC);
        assert_eq!(circuit.n_inputs(), 16);
        assert_eq!(circuit.n_outputs(), 8);
        assert!(circuit.n_ands() > 0);
    }

    #[test]
    fn add_matches_a_ripple_adder() {
        let ir = add(CiphertextSpec::new(16, 2, 2)).optimize_ir();
        let left = build_circuit(&ir, SPEC);

        let mut right = Circuit::new();
        let a: Vec<Lit> = (0..16).map(|_| right.input()).collect();
        let b: Vec<Lit> = (0..16).map(|_| right.input()).collect();
        for l in ripple_adder(&mut right, &a, &b) {
            right.output(l);
        }

        let verdict = check_equivalence(&left, &right, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn add_differs_from_sub() {
        let left = build_circuit(&add(CiphertextSpec::new(8, 2, 2)).optimize_ir(), SPEC);
        let right = build_circuit(&sub(CiphertextSpec::new(8, 2, 2)).optimize_ir(), SPEC);
        let verdict = check_equivalence(&left, &right, &CheckOptions::default()).unwrap();
        assert!(matches!(verdict, Verdict::NotEquivalent(_)));
    }
}
