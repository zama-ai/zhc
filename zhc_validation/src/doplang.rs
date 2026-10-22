use zhc_abc as ABC;
use zhc_crypto::integer_semantics::{
    SpecBlock, Type,
    lut::{LutId, LutRegistry},
};
use zhc_ir::{
    IR, Signature,
    evaluation::{EffectfulEvaluator, EvalOutcome, Evaluable, EvaluatesTo, Evaluation, OpState},
    visualization::{DynamicElement, VisualAnnotation},
};
use zhc_langs::doplang::{CtMem, CtReg, DopInstructionSet, DopLang, DopTypeSystem, PtArg};
use zhc_utils::{Dumpable, FastMap, small::SmallVec, svec};

use crate::symbolic::{Block, Circuit};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DopSymValue {
    Ctx,
}

impl Evaluation for DopSymValue {}

impl Dumpable for DopSymValue {
    fn dump_to_string(&self) -> String {
        format!("{self:?}")
    }
}

impl VisualAnnotation for DopSymValue {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}

impl EvaluatesTo<DopSymValue> for DopTypeSystem {
    fn type_of(_val: &DopSymValue) -> Self {
        DopTypeSystem::Ctx(0)
    }
}

#[derive(Debug, Clone)]
struct Notification {
    src: usize,
    slot: CtMem,
}

#[derive(Debug)]
struct Board {
    registers: Vec<Option<Block>>,
    heap: FastMap<usize, Block>,
    io: FastMap<usize, Block>,
    armed: FastMap<u8, CtMem>,
    mailbox: FastMap<u8, Notification>,
}

impl Board {
    fn new(num_registers: usize) -> Self {
        Self {
            registers: vec![None; num_registers],
            heap: FastMap::default(),
            io: FastMap::default(),
            armed: FastMap::default(),
            mailbox: FastMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct DopSymContext {
    circuit: Circuit,
    lut_reg: LutRegistry,
    boards: Vec<Board>,
    current: usize,
    sources: FastMap<(usize, usize), Block>,
    pt_sources: FastMap<(usize, usize), Block>,
    destinations: FastMap<(usize, usize), Block>,
}

impl DopSymContext {
    pub fn new(
        spec: SpecBlock,
        num_registers: usize,
        lut_reg: &LutRegistry,
        prototype: &Signature<Type>,
    ) -> Self {
        Self::new_multi(spec, 1, num_registers, lut_reg, prototype)
    }

    pub fn new_multi(
        spec: SpecBlock,
        n_hpus: usize,
        num_registers: usize,
        lut_reg: &LutRegistry,
        prototype: &Signature<Type>,
    ) -> Self {
        assert!(n_hpus > 0, "A program needs at least one HPU");
        let mut circuit = Circuit::new(spec);
        let mut sources = FastMap::default();
        let mut pt_sources = FastMap::default();
        let mut ct_pos = 0usize;
        let mut pt_pos = 0usize;
        for arg in prototype.get_args() {
            match arg {
                Type::CiphertextInteger(int_spec) => {
                    assert_eq!(int_spec.block_spec(), spec, "Spec mismatch.");
                    for block_pos in 0..int_spec.block_count() as usize {
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
                    for block_pos in 0..int_spec.block_count() as usize {
                        pt_sources.insert((pt_pos, block_pos), circuit.input_block());
                    }
                    pt_pos += 1;
                }
            }
        }
        Self {
            circuit,
            lut_reg: lut_reg.clone(),
            boards: (0..n_hpus).map(|_| Board::new(num_registers)).collect(),
            current: 0,
            sources,
            pt_sources,
            destinations: FastMap::default(),
        }
    }

    pub fn enter(&mut self, hpu: usize) {
        assert!(
            hpu < self.boards.len(),
            "HPU {hpu} is out of range for a system of {} HPUs",
            self.boards.len()
        );
        self.current = hpu;
    }

    pub fn finish(mut self) -> ABC::Circuit {
        for (hpu, board) in self.boards.iter().enumerate() {
            for flag in board.mailbox.keys() {
                panic!("Flag F({flag}) of HPU {hpu} was notified but never waited");
            }
            for flag in board.armed.keys() {
                panic!("Flag F({flag}) of HPU {hpu} was armed by LD_B2B but never waited");
            }
        }
        let mut ids: Vec<(usize, usize)> = self.destinations.keys().copied().collect();
        ids.sort_unstable();
        for id in ids {
            let block = self.destinations[&id].clone();
            self.circuit.output_block(&block);
        }
        self.circuit.into_circuit()
    }

    fn read_reg(&mut self, reg: &CtReg) -> Block {
        let slot = reg.addr as usize;
        let board = &mut self.boards[self.current];
        if board.registers[slot].is_none() {
            board.registers[slot] = Some(self.circuit.free_block());
        }
        board.registers[slot].clone().unwrap()
    }

    fn write_reg(&mut self, reg: &CtReg, block: Block) {
        self.boards[self.current].registers[reg.addr as usize] = Some(block);
    }

    fn write_regs(&mut self, reg: &CtReg, blocks: impl IntoIterator<Item = Block>) {
        let base = (reg.addr & reg.mask) as usize;
        let board = &mut self.boards[self.current];
        for (i, block) in blocks.into_iter().enumerate() {
            board.registers[base + i] = Some(block);
        }
    }

    fn read_mem(&self, mem: &CtMem) -> Block {
        self.read_board_mem(self.current, mem)
    }

    fn read_board_mem(&self, hpu: usize, mem: &CtMem) -> Block {
        match mem {
            CtMem::Heap(inner) => self.boards[hpu]
                .heap
                .get(&(inner.addr as usize))
                .unwrap_or_else(|| panic!("Heap CT_H({}) of HPU {hpu} not populated", inner.addr))
                .clone(),
            CtMem::Io(inner) => self.boards[hpu]
                .io
                .get(&(inner.addr as usize))
                .unwrap_or_else(|| panic!("I/O CT_IO({}) of HPU {hpu} not populated", inner.addr))
                .clone(),
            CtMem::Src(inner) => self
                .sources
                .get(&(inner.id as usize, inner.block as usize))
                .unwrap_or_else(|| panic!("Source TC({}, {}) not populated", inner.id, inner.block))
                .clone(),
            CtMem::Dst(_) => panic!("Expected a readable memory location, got {mem:?}"),
        }
    }

    fn write_mem(&mut self, mem: &CtMem, block: Block) {
        match mem {
            CtMem::Heap(inner) => {
                self.boards[self.current]
                    .heap
                    .insert(inner.addr as usize, block);
            }
            CtMem::Io(inner) => {
                self.boards[self.current]
                    .io
                    .insert(inner.addr as usize, block);
            }
            CtMem::Dst(inner) => {
                let replaced = self
                    .destinations
                    .insert((inner.id as usize, inner.block as usize), block);
                assert!(
                    replaced.is_none(),
                    "Destination TC({}, {}) already written",
                    inner.id,
                    inner.block
                );
            }
            CtMem::Src(_) => panic!("Expected a writable memory location, got {mem:?}"),
        }
    }

    fn read_pt(&self, arg: &PtArg) -> Block {
        match arg {
            PtArg::Const(inner) => self.circuit.const_block(inner.val as u16),
            PtArg::Var(inner) => self
                .pt_sources
                .get(&(inner.id as usize, inner.block as usize))
                .unwrap_or_else(|| {
                    panic!("Plaintext TI({}, {}) not populated", inner.id, inner.block)
                })
                .clone(),
        }
    }
}

impl Evaluable<DopSymValue> for DopInstructionSet {
    type Context = DopSymContext;

    fn eval(
        &self,
        context: &mut Self::Context,
        _arguments: SmallVec<&DopSymValue>,
    ) -> EvalOutcome<DopSymValue> {
        use DopInstructionSet::*;
        let results = match self {
            ADD { dst, src1, src2 } => {
                let (a, b) = (context.read_reg(src1), context.read_reg(src2));
                let out = context.circuit.add(&a, &b);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            SUB { dst, src1, src2 } => {
                let (a, b) = (context.read_reg(src1), context.read_reg(src2));
                let out = context.circuit.sub(&a, &b);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            MAC {
                dst,
                src1,
                src2,
                cst,
            } => {
                let PtArg::Const(cst) = cst else {
                    panic!("MAC expects a constant multiplier, got {cst:?}");
                };
                let (a, b) = (context.read_reg(src1), context.read_reg(src2));
                let out = context.circuit.mac(&a, cst.val as u16, &b);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            ADDS { dst, src, cst } => {
                let (a, b) = (context.read_reg(src), context.read_pt(cst));
                let out = context.circuit.add(&a, &b);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            SUBS { dst, src, cst } => {
                let (a, b) = (context.read_reg(src), context.read_pt(cst));
                let out = context.circuit.sub(&a, &b);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            SSUB { dst, src, cst } => {
                let (a, b) = (context.read_pt(cst), context.read_reg(src));
                let out = context.circuit.sub(&a, &b);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            MULS { dst, src, cst } => {
                let (a, b) = (context.read_reg(src), context.read_pt(cst));
                let out = context.circuit.mul(&a, &b);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            LD { dst, src } => {
                let block = context.read_mem(src);
                context.write_reg(dst, block);
                svec![DopSymValue::Ctx]
            }
            ST { dst, src } => {
                let block = context.read_reg(src);
                context.write_mem(dst, block);
                svec![DopSymValue::Ctx]
            }
            PBS { dst, src, lut } | PBS_F { dst, src, lut } => {
                let a = context.read_reg(src);
                let lut = context.lut_reg.get_l1(&LutId(lut.id as usize)).clone();
                let out = context.circuit.lut1(&lut, &a);
                context.write_reg(dst, out);
                svec![DopSymValue::Ctx]
            }
            PBS_ML2 { dst, src, lut } | PBS_ML2_F { dst, src, lut } => {
                let a = context.read_reg(src);
                let lut = context.lut_reg.get_l2(&LutId(lut.id as usize)).clone();
                let outs = context.circuit.lut2(&lut, &a);
                context.write_regs(dst, outs);
                svec![DopSymValue::Ctx]
            }
            PBS_ML4 { dst, src, lut } | PBS_ML4_F { dst, src, lut } => {
                let a = context.read_reg(src);
                let lut = context.lut_reg.get_l4(&LutId(lut.id as usize)).clone();
                let outs = context.circuit.lut4(&lut, &a);
                context.write_regs(dst, outs);
                svec![DopSymValue::Ctx]
            }
            PBS_ML8 { dst, src, lut } | PBS_ML8_F { dst, src, lut } => {
                let a = context.read_reg(src);
                let lut = context.lut_reg.get_l8(&LutId(lut.id as usize)).clone();
                let outs = context.circuit.lut8(&lut, &a);
                context.write_regs(dst, outs);
                svec![DopSymValue::Ctx]
            }
            _START => svec![DopSymValue::Ctx],
            _END => svec![],
            SYNC { .. } => svec![DopSymValue::Ctx],
            NOTIFY {
                virt_id,
                flag,
                slot,
            } => {
                let dst = virt_id.id as usize;
                assert!(
                    dst < context.boards.len(),
                    "NOTIFY targets HPU {dst}, but the system has {} HPUs",
                    context.boards.len()
                );
                let notification = Notification {
                    src: context.current,
                    slot: slot.clone(),
                };
                let replaced = context.boards[dst].mailbox.insert(flag.flag, notification);
                assert!(
                    replaced.is_none(),
                    "Flag {flag} of HPU {dst} notified twice before being waited"
                );
                svec![DopSymValue::Ctx]
            }
            LD_B2B { flag, slot } => {
                let replaced = context.boards[context.current]
                    .armed
                    .insert(flag.flag, slot.clone());
                assert!(
                    replaced.is_none(),
                    "Flag {flag} of HPU {} armed twice before being waited",
                    context.current
                );
                svec![DopSymValue::Ctx]
            }
            WAIT { flag, slot } => {
                let hpu = context.current;
                if !context.boards[hpu].mailbox.contains_key(&flag.flag) {
                    return EvalOutcome::Blocked;
                }
                let notification = context.boards[hpu].mailbox.remove(&flag.flag).unwrap();
                let armed = context.boards[hpu].armed.remove(&flag.flag);
                match (slot, armed) {
                    (None, None) => {}
                    (None, Some(armed)) => {
                        panic!("Sync WAIT on flag {flag} of HPU {hpu} armed by LD_B2B on {armed}")
                    }
                    (Some(slot), None) => {
                        panic!("WAIT on flag {flag} of HPU {hpu} with slot {slot} but no LD_B2B")
                    }
                    (Some(slot), Some(armed)) => {
                        assert_eq!(
                            *slot, armed,
                            "WAIT and LD_B2B on flag {flag} of HPU {hpu} disagree on the slot"
                        );
                        let block = context.read_board_mem(notification.src, &notification.slot);
                        context.write_mem(slot, block);
                    }
                }
                svec![DopSymValue::Ctx]
            }
        };
        EvalOutcome::Evaluated(results)
    }
}

pub fn build_circuit(
    ir: &IR<DopLang>,
    spec: SpecBlock,
    num_registers: usize,
    lut_reg: &LutRegistry,
    prototype: &Signature<Type>,
) -> ABC::Circuit {
    let mut context = DopSymContext::new(spec, num_registers, lut_reg, prototype);
    if let Err(eval_ir) = ir.evaluate::<DopSymValue>(&mut context) {
        eval_ir.dump_and_panic();
    }
    context.finish()
}

pub fn build_multi_hpu_circuit(
    irs: &[IR<DopLang>],
    spec: SpecBlock,
    num_registers: usize,
    lut_reg: &LutRegistry,
    prototype: &Signature<Type>,
) -> ABC::Circuit {
    let mut context = DopSymContext::new_multi(spec, irs.len(), num_registers, lut_reg, prototype);
    let mut evaluator = EffectfulEvaluator::<DopLang, DopSymValue>::from_irs(irs);
    if let Err(blocked) =
        evaluator.push_to_completion_with(&mut context, |context, hpu| context.enter(hpu))
    {
        let blocked = blocked
            .iter()
            .map(|(hpu, opid)| format!("  HPU {hpu}: {}", irs[*hpu].get_op(opid).format()))
            .collect::<Vec<_>>()
            .join("\n");
        panic!("Multi-HPU deadlock: these waits never completed:\n{blocked}");
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
    use zhc_builder::Builder;
    use zhc_config::hpu::{HpuConfig, PhysicalConfig};
    use zhc_crypto::integer_semantics::SpecInteger;
    use zhc_langs::doplang::{CtDstVar, CtSrcVar, UserFlag, VirtId};
    use zhc_pipeline::{
        SchedPolicy, extract_lut_registry,
        passes::{hpu_allocate_registers, hpu_schedule_legacy, lower_iop_to_hpu, optimize_iop},
    };

    fn stream(ops: impl IntoIterator<Item = DopInstructionSet>) -> IR<DopLang> {
        let mut ir = IR::<DopLang>::empty();
        let (_, mut ctx) = ir.add_op(DopInstructionSet::_START, svec![]);
        for op in ops {
            ctx = ir.add_op(op, ctx).1;
        }
        ir.add_op(DopInstructionSet::_END, ctx);
        ir
    }

    fn identity_iop() -> (IR<zhc_langs::ioplang::IopLang>, SpecBlock) {
        let spec = SpecBlock(2, 2);
        let builder = Builder::new(spec);
        let x = builder.ciphertext_integer_input(2);
        builder.ciphertext_integer_output(&x);
        (builder.ir().clone(), spec)
    }

    #[test]
    fn transfer_with_ack_across_two_hpus_matches_iop_identity() {
        let (iop_ir, spec) = identity_iop();
        let sender = stream([
            DopInstructionSet::LD {
                dst: CtReg::new(0u8),
                src: CtMem::src(0, 0),
            },
            DopInstructionSet::ST {
                dst: CtMem::heap(0),
                src: CtReg::new(0u8),
            },
            DopInstructionSet::NOTIFY {
                virt_id: VirtId::new(1),
                flag: UserFlag::new(1),
                slot: CtMem::heap(0),
            },
            DopInstructionSet::WAIT {
                flag: UserFlag::new(2),
                slot: None,
            },
        ]);
        let receiver = stream([
            DopInstructionSet::LD_B2B {
                flag: UserFlag::new(1),
                slot: CtMem::heap(5),
            },
            DopInstructionSet::WAIT {
                flag: UserFlag::new(1),
                slot: Some(CtMem::heap(5)),
            },
            DopInstructionSet::LD {
                dst: CtReg::new(0u8),
                src: CtMem::heap(5),
            },
            DopInstructionSet::NOTIFY {
                virt_id: VirtId::new(0),
                flag: UserFlag::new(2),
                slot: CtMem::heap(0),
            },
            DopInstructionSet::ST {
                dst: CtMem::dst(0, 0),
                src: CtReg::new(0u8),
            },
        ]);

        crate::equivalence_check::check_iop_multi_dop_equivalence(
            &iop_ir,
            &[sender, receiver],
            &extract_lut_registry(&iop_ir),
            spec,
            8,
        );
    }

    #[test]
    #[should_panic(expected = "Multi-HPU deadlock")]
    fn wait_without_notify_is_a_deadlock() {
        let (iop_ir, spec) = identity_iop();
        let sender = stream([
            DopInstructionSet::LD {
                dst: CtReg::new(0u8),
                src: CtMem::src(0, 0),
            },
            DopInstructionSet::ST {
                dst: CtMem::dst(0, 0),
                src: CtReg::new(0u8),
            },
        ]);
        let receiver = stream([DopInstructionSet::WAIT {
            flag: UserFlag::new(1),
            slot: None,
        }]);

        crate::equivalence_check::check_iop_multi_dop_equivalence(
            &iop_ir,
            &[sender, receiver],
            &extract_lut_registry(&iop_ir),
            spec,
            8,
        );
    }

    #[test]
    #[should_panic(expected = "Multi-HPU evaluation failed")]
    fn notifying_a_flag_twice_before_a_wait_is_rejected() {
        let (iop_ir, spec) = identity_iop();
        let sender = stream([
            DopInstructionSet::LD {
                dst: CtReg::new(0u8),
                src: CtMem::src(0, 0),
            },
            DopInstructionSet::ST {
                dst: CtMem::heap(0),
                src: CtReg::new(0u8),
            },
            DopInstructionSet::NOTIFY {
                virt_id: VirtId::new(1),
                flag: UserFlag::new(1),
                slot: CtMem::heap(0),
            },
            DopInstructionSet::NOTIFY {
                virt_id: VirtId::new(1),
                flag: UserFlag::new(1),
                slot: CtMem::heap(0),
            },
        ]);
        let receiver = stream([
            DopInstructionSet::LD_B2B {
                flag: UserFlag::new(1),
                slot: CtMem::heap(5),
            },
            DopInstructionSet::WAIT {
                flag: UserFlag::new(1),
                slot: Some(CtMem::heap(5)),
            },
            DopInstructionSet::LD {
                dst: CtReg::new(0u8),
                src: CtMem::heap(5),
            },
            DopInstructionSet::ST {
                dst: CtMem::dst(0, 0),
                src: CtReg::new(0u8),
            },
        ]);

        crate::equivalence_check::check_iop_multi_dop_equivalence(
            &iop_ir,
            &[sender, receiver],
            &extract_lut_registry(&iop_ir),
            spec,
            8,
        );
    }

    #[test]
    #[should_panic(expected = "IOP and multi-HPU DOP outputs differ")]
    fn transfer_of_the_wrong_slot_is_rejected() {
        let spec = SpecBlock(2, 2);
        let builder = Builder::new(spec);
        let x = builder.ciphertext_integer_input(2);
        let y = builder.ciphertext_integer_input(2);
        builder.ciphertext_integer_output(&x);
        builder.ciphertext_integer_output(&y);
        let iop_ir = builder.ir();
        let sender = stream([
            DopInstructionSet::LD {
                dst: CtReg::new(0u8),
                src: CtMem::src(0, 0),
            },
            DopInstructionSet::LD {
                dst: CtReg::new(1u8),
                src: CtMem::src(1, 0),
            },
            DopInstructionSet::ST {
                dst: CtMem::heap(0),
                src: CtReg::new(0u8),
            },
            DopInstructionSet::ST {
                dst: CtMem::heap(1),
                src: CtReg::new(1u8),
            },
            DopInstructionSet::NOTIFY {
                virt_id: VirtId::new(1),
                flag: UserFlag::new(1),
                slot: CtMem::heap(1),
            },
            DopInstructionSet::ST {
                dst: CtMem::dst(1, 0),
                src: CtReg::new(1u8),
            },
        ]);
        let receiver = stream([
            DopInstructionSet::LD_B2B {
                flag: UserFlag::new(1),
                slot: CtMem::heap(5),
            },
            DopInstructionSet::WAIT {
                flag: UserFlag::new(1),
                slot: Some(CtMem::heap(5)),
            },
            DopInstructionSet::LD {
                dst: CtReg::new(0u8),
                src: CtMem::heap(5),
            },
            DopInstructionSet::ST {
                dst: CtMem::dst(0, 0),
                src: CtReg::new(0u8),
            },
        ]);

        crate::equivalence_check::check_iop_multi_dop_equivalence(
            &iop_ir,
            &[sender, receiver],
            &extract_lut_registry(&iop_ir),
            spec,
            8,
        );
    }

    #[test]
    fn allocated_add_matches_iop_add() {
        let builder = Iop::Add.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let lut_reg = extract_lut_registry(&iop_ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = hpu_schedule_legacy(
            &lower_iop_to_hpu(&iop_ir).output,
            &config,
            SchedPolicy::AsSoonAsPossible,
            SchedPolicy::AsSoonAsPossible,
        );
        let dop_ir = hpu_allocate_registers(&scheduled, &config, &lut_reg, 1, 0);

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let dop = build_circuit(
            &dop_ir,
            spec,
            config.regf_size,
            &lut_reg,
            &builder.signature(),
        );

        let verdict = check_equivalence(&iop, &dop, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn allocated_mul_matches_iop_mul() {
        let builder = Iop::Mul.to_builder(SpecInteger::new(8, 2, 2));
        let spec = *builder.spec();
        let iop_ir = optimize_iop(&builder.ir());
        let lut_reg = extract_lut_registry(&iop_ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = hpu_schedule_legacy(
            &lower_iop_to_hpu(&iop_ir).output,
            &config,
            SchedPolicy::AsSoonAsPossible,
            SchedPolicy::AsSoonAsPossible,
        );
        let dop_ir = hpu_allocate_registers(&scheduled, &config, &lut_reg, 1, 0);

        let iop = crate::ioplang::build_circuit(&builder.ir(), spec);
        let dop = build_circuit(
            &dop_ir,
            spec,
            config.regf_size,
            &lut_reg,
            &builder.signature(),
        );

        let verdict = check_equivalence(&iop, &dop, &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn allocated_add_differs_from_iop_sub() {
        let add_builder = Iop::Add.to_builder(SpecInteger::new(16, 2, 2));
        let sub_builder = Iop::Sub.to_builder(SpecInteger::new(16, 2, 2));
        let spec = *add_builder.spec();
        let iop_ir = optimize_iop(&add_builder.ir());
        let lut_reg = extract_lut_registry(&iop_ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = hpu_schedule_legacy(
            &lower_iop_to_hpu(&iop_ir).output,
            &config,
            SchedPolicy::AsSoonAsPossible,
            SchedPolicy::AsSoonAsPossible,
        );
        let dop_ir = hpu_allocate_registers(&scheduled, &config, &lut_reg, 1, 0);

        let iop = crate::ioplang::build_circuit(&sub_builder.ir(), spec);
        let dop = build_circuit(
            &dop_ir,
            spec,
            config.regf_size,
            &lut_reg,
            &add_builder.signature(),
        );

        let verdict = check_equivalence(&iop, &dop, &CheckOptions::default()).unwrap();
        assert!(matches!(verdict, Verdict::NotEquivalent(_)));
    }

    #[test]
    fn copy_through_a_register_matches_iop_identity() {
        let spec = SpecBlock(2, 2);
        let builder = Builder::new(spec);
        let x = builder.ciphertext_integer_input(2);
        builder.ciphertext_integer_output(&x);
        let iop_ir = builder.ir();

        let mut dop_ir = IR::<DopLang>::empty();
        let (_, ctx) = dop_ir.add_op(DopInstructionSet::_START, svec![]);
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::LD {
                dst: CtReg::new(1u8),
                src: CtMem::Src(CtSrcVar { id: 0, block: 0 }),
            },
            ctx,
        );
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::ST {
                dst: CtMem::Dst(CtDstVar { id: 0, block: 0 }),
                src: CtReg::new(1u8),
            },
            ctx,
        );
        dop_ir.add_op(DopInstructionSet::_END, ctx);

        crate::equivalence_check::check_iop_dop_equivalence(
            &iop_ir,
            &dop_ir,
            &extract_lut_registry(&iop_ir),
            spec,
            8,
        );
    }

    #[test]
    #[should_panic(expected = "IOP and DOP outputs differ")]
    fn output_depending_on_an_unwritten_register_is_rejected() {
        let spec = SpecBlock(2, 2);
        let builder = Builder::new(spec);
        let x = builder.ciphertext_integer_input(2);
        builder.ciphertext_integer_output(&x);
        let iop_ir = builder.ir();

        let mut dop_ir = IR::<DopLang>::empty();
        let (_, ctx) = dop_ir.add_op(DopInstructionSet::_START, svec![]);
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::LD {
                dst: CtReg::new(1u8),
                src: CtMem::Src(CtSrcVar { id: 0, block: 0 }),
            },
            ctx,
        );
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::ADD {
                dst: CtReg::new(2u8),
                src1: CtReg::new(1u8),
                src2: CtReg::new(0u8),
            },
            ctx,
        );
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::ST {
                dst: CtMem::Dst(CtDstVar { id: 0, block: 0 }),
                src: CtReg::new(2u8),
            },
            ctx,
        );
        dop_ir.add_op(DopInstructionSet::_END, ctx);

        crate::equivalence_check::check_iop_dop_equivalence(
            &iop_ir,
            &dop_ir,
            &extract_lut_registry(&iop_ir),
            spec,
            8,
        );
    }

    #[test]
    fn zeroing_an_unwritten_register_before_use_is_accepted() {
        let spec = SpecBlock(2, 2);
        let builder = Builder::new(spec);
        let x = builder.ciphertext_integer_input(2);
        builder.ciphertext_integer_output(&x);
        let iop_ir = builder.ir();

        let mut dop_ir = IR::<DopLang>::empty();
        let (_, ctx) = dop_ir.add_op(DopInstructionSet::_START, svec![]);
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::SUB {
                dst: CtReg::new(0u8),
                src1: CtReg::new(0u8),
                src2: CtReg::new(0u8),
            },
            ctx,
        );
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::LD {
                dst: CtReg::new(1u8),
                src: CtMem::Src(CtSrcVar { id: 0, block: 0 }),
            },
            ctx,
        );
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::ADD {
                dst: CtReg::new(2u8),
                src1: CtReg::new(1u8),
                src2: CtReg::new(0u8),
            },
            ctx,
        );
        let (_, ctx) = dop_ir.add_op(
            DopInstructionSet::ST {
                dst: CtMem::Dst(CtDstVar { id: 0, block: 0 }),
                src: CtReg::new(2u8),
            },
            ctx,
        );
        dop_ir.add_op(DopInstructionSet::_END, ctx);

        crate::equivalence_check::check_iop_dop_equivalence(
            &iop_ir,
            &dop_ir,
            &extract_lut_registry(&iop_ir),
            spec,
            8,
        );
    }
}
