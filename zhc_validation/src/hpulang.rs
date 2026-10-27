use zhc_abc as ABC;
use zhc_crypto::integer_semantics::{SpecBlock, Type};
use zhc_ir::{
    IR, Signature,
    evaluation::{EffectfulEvaluator, EvalOutcome, Evaluable, EvaluatesTo, Evaluation, OpState},
    visualization::{DynamicElement, VisualAnnotation},
};
use zhc_langs::hpulang::{
    HpuId, HpuInstructionSet, HpuLang, HpuTypeSystem, Slot, TDstId, TImmId, TSrcId,
};
use zhc_utils::{
    Dumpable, FastMap, SafeAs, existential_enum, iter::CollectInSmallVec, small::SmallVec, svec,
};

use crate::symbolic::{Block, Circuit};

#[existential_enum]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HpuSymValue {
    CtRegister(Block),
    PtImmediate(Block),
    CtHeap(Block),
}

impl Evaluation for HpuSymValue {}

impl Dumpable for HpuSymValue {
    fn dump_to_string(&self) -> String {
        format!("{self:?}")
    }
}

impl VisualAnnotation for HpuSymValue {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}

impl EvaluatesTo<HpuSymValue> for HpuTypeSystem {
    fn type_of(val: &HpuSymValue) -> Self {
        match val {
            HpuSymValue::CtRegister(..) => Self::CtRegister,
            HpuSymValue::PtImmediate(..) => Self::PtImmediate,
            HpuSymValue::CtHeap(..) => Self::CtHeap,
        }
    }
}

#[derive(Debug)]
pub struct HpuSymContext {
    circuit: Circuit,
    sources: FastMap<TSrcId, Block>,
    immediates: FastMap<TImmId, Block>,
    destinations: FastMap<TDstId, Block>,
    transfers: FastMap<(HpuId, Slot), Block>,
    batch_args: FastMap<u8, HpuSymValue>,
    batch_rets: FastMap<u8, HpuSymValue>,
}

impl HpuSymContext {
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
                    for block_pos in 0..int_spec.block_count() {
                        let id = TSrcId {
                            src_pos: ct_pos,
                            block_pos: block_pos.sas(),
                        };
                        sources.insert(id, circuit.input_block());
                    }
                    ct_pos += 1;
                }
                Type::CiphertextBool(block_spec) => {
                    assert_eq!(*block_spec, spec, "Spec mismatch.");
                    let id = TSrcId {
                        src_pos: ct_pos,
                        block_pos: 0,
                    };
                    sources.insert(id, circuit.input_block());
                    ct_pos += 1;
                }
                Type::PlaintextInteger(int_spec) => {
                    assert_eq!(int_spec.block_spec(), spec, "Spec mismatch.");
                    for block_pos in 0..int_spec.block_count() {
                        let id = TImmId {
                            imm_pos: pt_pos,
                            block_pos: block_pos.sas(),
                        };
                        immediates.insert(id, circuit.input_block());
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
            transfers: FastMap::default(),
            batch_args: FastMap::default(),
            batch_rets: FastMap::default(),
        }
    }

    pub fn has_pending_transfers(&self) -> bool {
        !self.transfers.is_empty()
    }

    pub fn finish(mut self) -> ABC::Circuit {
        assert!(
            !self.has_pending_transfers(),
            "Some transfers were posted but never taken"
        );
        let mut ids: Vec<TDstId> = self.destinations.keys().cloned().collect();
        ids.sort_by_key(|id| (id.dst_pos, id.block_pos));
        for id in ids {
            let block = self.destinations[&id].clone();
            self.circuit.output_block(&block);
        }
        self.circuit.into_circuit()
    }
}

impl Evaluable<HpuSymValue> for HpuInstructionSet {
    type Context = HpuSymContext;

    fn eval(
        &self,
        context: &mut Self::Context,
        arguments: SmallVec<&HpuSymValue>,
    ) -> EvalOutcome<HpuSymValue> {
        use HpuInstructionSet::*;
        let ct = HpuSymValue::CtRegister;
        let c = &mut context.circuit;
        let results = match self {
            TransferOut { dst, slot, .. } => {
                let block = arguments[0].unwrap_ct_register_ref().clone();
                let replaced = context.transfers.insert((*dst, *slot), block);
                assert!(
                    replaced.is_none(),
                    "Transfer {slot:?} to {dst:?} posted twice"
                );
                svec![]
            }
            TransferIn { dst, slot, .. } => {
                let Some(block) = context.transfers.remove(&(*dst, *slot)) else {
                    return EvalOutcome::Blocked;
                };
                svec![ct(block)]
            }
            Transfer { .. } => svec![ct(arguments[0].unwrap_ct_register_ref().clone())],
            SrcLd { from } => {
                let block = context
                    .sources
                    .get(from)
                    .unwrap_or_else(|| panic!("Source {from} missing from context"));
                svec![ct(block.clone())]
            }
            DstSt { to } => {
                assert!(
                    !context.destinations.contains_key(to),
                    "Destination {to} already written"
                );
                let block = arguments[0].unwrap_ct_register_ref().clone();
                context.destinations.insert(to.clone(), block);
                svec![]
            }
            ImmLd { from } => {
                let block = context
                    .immediates
                    .get(from)
                    .unwrap_or_else(|| panic!("Immediate {from} missing from context"));
                svec![HpuSymValue::PtImmediate(block.clone())]
            }
            CstCt { cst } => svec![ct(c.const_block(cst.0 as u16))],
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
                cst.0 as u16,
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
                let cst = c.const_block(cst.0 as u16);
                svec![ct(c.add(arguments[0].unwrap_ct_register_ref(), &cst))]
            }
            SubCst { cst } => {
                let cst = c.const_block(cst.0 as u16);
                svec![ct(c.sub(arguments[0].unwrap_ct_register_ref(), &cst))]
            }
            CstSub { cst } => {
                let cst = c.const_block(cst.0 as u16);
                svec![ct(c.sub(&cst, arguments[0].unwrap_ct_register_ref()))]
            }
            MulCst { cst } => {
                let cst = c.const_block(cst.0 as u16);
                svec![ct(c.mul(arguments[0].unwrap_ct_register_ref(), &cst))]
            }
            Pbs { lut } | PbsF { lut } => {
                svec![ct(c.lut1(lut, arguments[0].unwrap_ct_register_ref()))]
            }
            Pbs2 { lut } | Pbs2F { lut } => c
                .lut2(lut, arguments[0].unwrap_ct_register_ref())
                .into_iter()
                .map(ct)
                .cosvec(),
            Pbs4 { lut } | Pbs4F { lut } => c
                .lut4(lut, arguments[0].unwrap_ct_register_ref())
                .into_iter()
                .map(ct)
                .cosvec(),
            Pbs8 { lut } | Pbs8F { lut } => c
                .lut8(lut, arguments[0].unwrap_ct_register_ref())
                .into_iter()
                .map(ct)
                .cosvec(),
            BatchArg { pos, .. } => {
                let value = context
                    .batch_args
                    .get(pos)
                    .unwrap_or_else(|| panic!("BatchArg {pos} missing from batch context"));
                svec![value.clone()]
            }
            BatchRet { pos, .. } => {
                assert!(
                    !context.batch_rets.contains_key(pos),
                    "BatchRet {pos} already written"
                );
                context.batch_rets.insert(*pos, arguments[0].clone());
                svec![]
            }
            Batch { block } => {
                let saved_args = std::mem::take(&mut context.batch_args);
                let saved_rets = std::mem::take(&mut context.batch_rets);

                let mut arg_positions: Vec<u8> = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_instruction() {
                        BatchArg { pos, .. } => Some(*pos),
                        _ => None,
                    })
                    .collect();
                arg_positions.sort_unstable();
                for (value, pos) in arguments.iter().zip(arg_positions.iter()) {
                    context.batch_args.insert(*pos, (*value).clone());
                }

                if let Err(eval_ir) = block.evaluate::<HpuSymValue>(context) {
                    eval_ir.dump_and_panic();
                }

                let mut ret_positions: Vec<u8> = context.batch_rets.keys().copied().collect();
                ret_positions.sort_unstable();
                let returns = ret_positions
                    .iter()
                    .map(|pos| context.batch_rets.remove(pos).unwrap())
                    .cosvec();

                context.batch_args = saved_args;
                context.batch_rets = saved_rets;
                returns
            }
        };
        EvalOutcome::Evaluated(results)
    }
}

pub fn build_circuit(
    ir: &IR<HpuLang>,
    spec: SpecBlock,
    prototype: &Signature<Type>,
) -> ABC::Circuit {
    let mut context = HpuSymContext::new(spec, prototype);
    if let Err(eval_ir) = ir.evaluate::<HpuSymValue>(&mut context) {
        eval_ir.dump_and_panic();
    }
    context.finish()
}

pub fn build_multi_hpu_circuit(
    irs: &[IR<HpuLang>],
    spec: SpecBlock,
    prototype: &Signature<Type>,
) -> ABC::Circuit {
    let mut context = HpuSymContext::new(spec, prototype);
    let mut evaluator = EffectfulEvaluator::<HpuLang, HpuSymValue>::from_irs(irs);
    if let Err(blocked) = evaluator.push_to_completion(&mut context) {
        let blocked = blocked
            .iter()
            .map(|(hpu, opid)| format!("  HPU {hpu}: {}", irs[*hpu].get_op(opid).format()))
            .collect::<Vec<_>>()
            .join("\n");
        panic!("Multi-HPU deadlock: these transfers never completed:\n{blocked}");
    }
    if !evaluator.is_ok() {
        let failed = evaluator
            .into_eval_irs()
            .into_iter()
            .enumerate()
            .filter(|(_, eval_ir)| {
                !eval_ir
                    .walk_ops_linear()
                    .all(|op| matches!(op.get_annotation(), OpState::Evaluated(_)))
            })
            .map(|(hpu, eval_ir)| {
                eval_ir.dump();
                hpu
            })
            .collect::<Vec<_>>();
        panic!("Multi-HPU evaluation failed on HPUs {failed:?}");
    }
    context.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc::compat::Iop;
    use zhc_abc::{CheckOptions, Verdict, check_equivalence};
    use zhc_config::{
        hpu::{HpuConfig, PhysicalConfig},
        multi_hpu::MultiHpuConfig,
    };
    use zhc_crypto::integer_semantics::SpecInteger;
    use zhc_pipeline::{
        PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy,
        passes::{
            hpu_batch_legacy, hpu_schedule, lower_iop_to_hpu, lower_iop_to_multi_hpu,
            multi_hpu_schedule, optimize_iop, partition_and_materialize,
        },
    };
    use zhc_utils::units::Cycle;

    #[test]
    fn lowered_add_matches_iop_add() {
        let builder = Iop::Add.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let hpu_ir = lower_iop_to_hpu(&iop_ir).output;

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let hpu = build_circuit(&hpu_ir, spec, &builder.signature());

        let verdict = check_equivalence(&iop, &hpu, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn lowered_add_differs_from_iop_sub() {
        let add_builder = Iop::Add.to_builder(SpecInteger::new(16, 2, 2));
        let sub_builder = Iop::Sub.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *add_builder.spec();
        let hpu_ir = lower_iop_to_hpu(&optimize_iop(&add_builder.ir())).output;

        let iop = crate::ioplang::build_circuit(&sub_builder.ir(), spec);
        let hpu = build_circuit(&hpu_ir, spec, &add_builder.signature());

        let verdict = check_equivalence(&iop, &hpu, &CheckOptions::default()).unwrap();
        assert!(matches!(verdict, Verdict::NotEquivalent(_)));
    }

    #[test]
    fn scheduled_mul_matches_iop_mul() {
        let builder = Iop::Mul.to_builder(SpecInteger::new(8, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let hpu_ir = hpu_schedule(
            &lower_iop_to_hpu(&iop_ir).output,
            &config,
            SchedPolicy::AsLateAsPossible,
        );

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let hpu = build_circuit(&hpu_ir, spec, &builder.signature());

        let verdict = check_equivalence(&iop, &hpu, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn batched_add_matches_iop_add() {
        let builder = Iop::Add.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let hpu_ir = hpu_batch_legacy(
            &lower_iop_to_hpu(&iop_ir).output,
            &config,
            SchedPolicy::AsSoonAsPossible,
        );
        assert!(
            hpu_ir
                .walk_ops_linear()
                .any(|op| matches!(op.get_instruction(), HpuInstructionSet::Batch { .. })),
            "The program has no batch; the test is vacuous."
        );

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let hpu = build_circuit(&hpu_ir, spec, &builder.signature());

        let verdict = check_equivalence(&iop, &hpu, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn multi_hpu_add_matches_iop_add() {
        let builder = Iop::Add.to_builder(SpecInteger::new(8, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let partitioner_config = PartitionerConfig {
            specs: std::iter::repeat(PartitionSpec {
                parallelism: 12,
                latency: Cycle(1),
            })
            .take(2)
            .collect(),
            sched_policy: SchedPolicy::AsSoonAsPossible,
            tie_policy: TiePolicy::Balance,
            spread_policy: SpreadPolicy::MinimizeTransfers,
        };
        let (remat, partitions) = partition_and_materialize(&iop_ir, &partitioner_config);
        let (hpu_ir, localities) = lower_iop_to_multi_hpu(&remat, &partitions);
        let multi_config = MultiHpuConfig {
            n_hpus: 2,
            ..MultiHpuConfig::default()
        };
        let hpu_irs = multi_hpu_schedule(
            &hpu_ir,
            &localities,
            &multi_config,
            SchedPolicy::AsSoonAsPossible,
        );
        assert!(
            hpu_irs
                .iter()
                .flat_map(|ir| ir.walk_ops_linear().collect::<Vec<_>>())
                .any(|op| matches!(op.get_instruction(), HpuInstructionSet::TransferIn { .. })),
            "The schedule has no transfer; the test is vacuous."
        );

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let hpu = build_multi_hpu_circuit(&hpu_irs, spec, &builder.signature());

        let verdict = check_equivalence(&iop, &hpu, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }
}
