use zhc_abc as ABC;
use zhc_crypto::integer_semantics::{SpecBlock, Type};
use zhc_ir::{
    IR, Signature,
    evaluation::{EvalOutcome, Evaluable, EvaluatesTo, Evaluation},
    visualization::{DynamicElement, VisualAnnotation},
};
use zhc_langs::vmlang::{VmInstructionSet, VmLang, VmTypeSystem};
use zhc_utils::{
    Dumpable, FastMap, existential_enum, iter::CollectInSmallVec, small::SmallVec, svec,
};

use crate::symbolic::{Block, Circuit};

#[existential_enum]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VmSymValue {
    CtRegister(Block),
    PtImmediate(Block),
}

impl Evaluation for VmSymValue {}

impl Dumpable for VmSymValue {
    fn dump_to_string(&self) -> String {
        format!("{self:?}")
    }
}

impl VisualAnnotation for VmSymValue {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}

impl EvaluatesTo<VmSymValue> for VmTypeSystem {
    fn type_of(val: &VmSymValue) -> Self {
        match val {
            VmSymValue::CtRegister(..) => Self::CtRegister,
            VmSymValue::PtImmediate(..) => Self::PtImmediate,
        }
    }
}

#[derive(Debug)]
pub struct VmSymContext {
    circuit: Circuit,
    sources: FastMap<(u32, u32), Block>,
    immediates: FastMap<(u32, u32), Block>,
    destinations: FastMap<(u32, u32), Block>,
}

impl VmSymContext {
    pub fn new(spec: SpecBlock, prototype: &Signature<Type>) -> Self {
        let mut circuit = Circuit::new(spec);
        let mut sources = FastMap::default();
        let mut immediates = FastMap::default();
        let mut ct_pos = 0u32;
        let mut pt_pos = 0u32;
        for arg in prototype.get_args() {
            match arg {
                Type::CiphertextInteger(int_spec) => {
                    assert_eq!(int_spec.block_spec(), spec, "Spec mismatch.");
                    for block_pos in 0..int_spec.block_count() as u32 {
                        sources.insert((ct_pos, block_pos), circuit.input_block());
                    }
                    ct_pos += 1;
                }
                Type::CiphertextBool(block_spec) => {
                    assert_eq!(*block_spec, spec, "Spec mismatch.");
                    sources.insert((ct_pos, 0), circuit.input_block());
                    ct_pos += 1;
                }
                Type::PlaintextInteger(int_spec) => {
                    assert_eq!(int_spec.block_spec(), spec, "Spec mismatch.");
                    for block_pos in 0..int_spec.block_count() as u32 {
                        immediates.insert((pt_pos, block_pos), circuit.input_block());
                    }
                    pt_pos += 1;
                }
            }
        }
        Self {
            circuit,
            sources,
            immediates,
            destinations: FastMap::default(),
        }
    }

    pub fn finish(mut self) -> ABC::Circuit {
        let mut ids: Vec<(u32, u32)> = self.destinations.keys().copied().collect();
        ids.sort_unstable();
        for id in ids {
            let block = self.destinations[&id].clone();
            self.circuit.output_block(&block);
        }
        self.circuit.into_circuit()
    }
}

impl Evaluable<VmSymValue> for VmInstructionSet {
    type Context = VmSymContext;

    fn eval(
        &self,
        context: &mut Self::Context,
        arguments: SmallVec<&VmSymValue>,
    ) -> EvalOutcome<VmSymValue> {
        use VmInstructionSet::*;
        let ct = VmSymValue::CtRegister;
        let c = &mut context.circuit;
        let results = match self {
            SrcLd {
                from_pos,
                from_block,
            } => {
                let block = context
                    .sources
                    .get(&(*from_pos, *from_block))
                    .unwrap_or_else(|| {
                        panic!("Source {from_pos}.{from_block} missing from context")
                    });
                svec![ct(block.clone())]
            }
            ImmLd {
                from_pos,
                from_block,
            } => {
                let block = context
                    .immediates
                    .get(&(*from_pos, *from_block))
                    .unwrap_or_else(|| {
                        panic!("Immediate {from_pos}.{from_block} missing from context")
                    });
                svec![VmSymValue::PtImmediate(block.clone())]
            }
            DstSt { to_pos, to_block } => {
                assert!(
                    !context.destinations.contains_key(&(*to_pos, *to_block)),
                    "Destination {to_pos}.{to_block} already written"
                );
                let block = arguments[0].unwrap_ct_register_ref().clone();
                context.destinations.insert((*to_pos, *to_block), block);
                svec![]
            }
            CstCt { cst } => svec![ct(c.const_block(*cst as u16))],
            AddCt => svec![ct(c.add(
                arguments[0].unwrap_ct_register_ref(),
                arguments[1].unwrap_ct_register_ref()
            ))],
            SubCt => svec![ct(c.sub(
                arguments[0].unwrap_ct_register_ref(),
                arguments[1].unwrap_ct_register_ref()
            ))],
            Mac { cst } => svec![ct(c.mac(
                arguments[0].unwrap_ct_register_ref(),
                *cst as u16,
                arguments[1].unwrap_ct_register_ref()
            ))],
            AddPt => svec![ct(c.add(
                arguments[0].unwrap_ct_register_ref(),
                arguments[1].unwrap_pt_immediate_ref()
            ))],
            SubPt => svec![ct(c.sub(
                arguments[0].unwrap_ct_register_ref(),
                arguments[1].unwrap_pt_immediate_ref()
            ))],
            PtSub => svec![ct(c.sub(
                arguments[0].unwrap_pt_immediate_ref(),
                arguments[1].unwrap_ct_register_ref()
            ))],
            MulPt => svec![ct(c.mul(
                arguments[0].unwrap_ct_register_ref(),
                arguments[1].unwrap_pt_immediate_ref()
            ))],
            AddCst { cst } => {
                let cst = c.const_block(*cst as u16);
                svec![ct(c.add(arguments[0].unwrap_ct_register_ref(), &cst))]
            }
            SubCst { cst } => {
                let cst = c.const_block(*cst as u16);
                svec![ct(c.sub(arguments[0].unwrap_ct_register_ref(), &cst))]
            }
            CstSub { cst } => {
                let cst = c.const_block(*cst as u16);
                svec![ct(c.sub(&cst, arguments[0].unwrap_ct_register_ref()))]
            }
            MulCst { cst } => {
                let cst = c.const_block(*cst as u16);
                svec![ct(c.mul(arguments[0].unwrap_ct_register_ref(), &cst))]
            }
            Ks => svec![ct(arguments[0].unwrap_ct_register_ref().clone())],
            Pbs { lut } => svec![ct(c.lut1(lut, arguments[0].unwrap_ct_register_ref()))],
            Pbs2 { lut } => c
                .lut2(lut, arguments[0].unwrap_ct_register_ref())
                .into_iter()
                .map(ct)
                .cosvec(),
        };
        EvalOutcome::Evaluated(results)
    }
}

pub fn build_circuit(
    ir: &IR<VmLang>,
    spec: SpecBlock,
    prototype: &Signature<Type>,
) -> ABC::Circuit {
    let mut context = VmSymContext::new(spec, prototype);
    if let Err(eval_ir) = ir.evaluate::<VmSymValue>(&mut context) {
        eval_ir.dump_and_panic();
    }
    context.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc::compat::Iop;
    use zhc_abc::{CheckOptions, Verdict, check_equivalence};
    use zhc_crypto::integer_semantics::SpecInteger;
    use zhc_pipeline::passes::{lower_iop_to_vm, optimize_iop};

    #[test]
    fn lowered_add_matches_iop_add() {
        let builder = Iop::Add.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let vm_ir = lower_iop_to_vm(&iop_ir);

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let vm = build_circuit(&vm_ir, spec, &builder.signature());

        let verdict = check_equivalence(&iop, &vm, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn lowered_add_differs_from_iop_sub() {
        let add_builder = Iop::Add.to_builder(SpecInteger::new(16, 2, 2));
        let sub_builder = Iop::Sub.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *add_builder.spec();
        let vm_ir = lower_iop_to_vm(&optimize_iop(&add_builder.ir()));

        let iop = crate::ioplang::build_circuit(&sub_builder.ir(), spec);
        let vm = build_circuit(&vm_ir, spec, &add_builder.signature());

        let verdict = check_equivalence(&iop, &vm, &CheckOptions::default()).unwrap();
        assert!(matches!(verdict, Verdict::NotEquivalent(_)));
    }

    #[test]
    fn lowered_adds_matches_iop_adds() {
        let builder = Iop::Adds.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let vm_ir = lower_iop_to_vm(&iop_ir);
        assert!(
            vm_ir
                .walk_ops_linear()
                .any(|op| matches!(op.get_instruction(), VmInstructionSet::ImmLd { .. })),
            "The program has no immediate load; the test is vacuous."
        );

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let vm = build_circuit(&vm_ir, spec, &builder.signature());

        let verdict = check_equivalence(&iop, &vm, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }
}
