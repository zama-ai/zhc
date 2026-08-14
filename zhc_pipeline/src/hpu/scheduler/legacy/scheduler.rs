use std::collections::BinaryHeap;

use super::*;
use zhc_ir::{AnnIR, AnnOpRef, IR, OpId, OpIdRaw, scheduler::reschedule};
use zhc_langs::hpulang::HpuLang;
use zhc_sim::hpu::{ConstantLatency, FlatLinLatency};
use zhc_utils::{Dumpable, fsm, svec, units::Cycle};

static PBS_COST: OpIdRaw = 2;
static NON_PBS_COST: OpIdRaw = 1;

type Prio = OpIdRaw;
type PrioOpRef<'a, 'b> = AnnOpRef<'a, 'b, HpuLang, Prio, ()>;
type PrioIR<'a> = AnnIR<'a, HpuLang, Prio, ()>;

fn analyze_prio<'a>(ir: &'a IR<HpuLang>, policy: SchedPolicy) -> PrioIR<'a> {
    use zhc_langs::hpulang::HpuInstructionSet::*;
    match policy {
        SchedPolicy::AsSoonAsPossible => ir.backward_dataflow_analysis(|opref| {
            let prio = opref
                .get_users_iter()
                .map(|p| p.get_annotation().clone().unwrap_analyzed())
                .max();
            match opref.get_instruction() {
                Batch { .. } => (
                    prio.unwrap() + PBS_COST,
                    svec![(); opref.get_return_arity()],
                ),
                _ => (
                    prio.unwrap_or(0) + NON_PBS_COST,
                    svec![(); opref.get_return_arity()],
                ),
            }
        }),
        SchedPolicy::AsLateAsPossible => ir.forward_dataflow_analysis(|opref| {
            let prio: Option<Prio> = opref
                .get_predecessors_iter()
                .map(|p| p.get_annotation().clone().unwrap_analyzed())
                .max();
            match opref.get_instruction() {
                Batch { .. } => (
                    prio.unwrap() + PBS_COST,
                    svec![(); opref.get_return_arity()],
                ),
                _ => (
                    prio.unwrap_or(0) + NON_PBS_COST,
                    svec![(); opref.get_return_arity()],
                ),
            }
        }),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Affinity {
    Pea,
    Pem,
    Pep,
    Ctl,
}

#[fsm]
#[derive(Debug)]
enum ProcessingElementState<'a, 'b> {
    Idle,
    Running(PrioOpRef<'a, 'b>),
}

pub struct ProcessingElement<'a, 'b> {
    state: ProcessingElementState<'a, 'b>,
    ready: Vec<PrioOpRef<'a, 'b>>,
}

impl<'a, 'b> ProcessingElement<'a, 'b> {
    pub fn new(inps: impl Iterator<Item = PrioOpRef<'a, 'b>>) -> Self {
        ProcessingElement {
            state: ProcessingElementState::Idle,
            ready: inps.collect(),
        }
    }

    pub fn land(&mut self) -> PrioOpRef<'a, 'b> {
        self.state.transition_with(|old| match old {
            ProcessingElementState::Running(op) => (ProcessingElementState::Idle, op),
            _ => unreachable!(),
        })
    }

    pub fn kick(&mut self, config: &HpuConfig) -> Option<(OpId, Cycle)> {
        self.ready.reverse();
        self.ready.sort_by_key(|op| *op.get_annotation());
        self.ready.pop().map(|op| {
            let duration = get_op_latency(&op, config);
            let id = op.get_id();
            self.state.transition(|old| match old {
                ProcessingElementState::Idle => ProcessingElementState::Running(op),
                _ => unreachable!(),
            });
            (id, duration)
        })
    }

    pub fn push(&mut self, op: PrioOpRef<'a, 'b>) {
        self.ready.push(op);
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.state, ProcessingElementState::Idle)
    }
}

#[fsm]
#[derive(Debug)]
enum State {
    Scheduled,
    Ready,
    Waiting(usize),
}

impl Dumpable for State {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
struct Wake {
    at: Cycle,
    aff: Affinity,
}

impl Dumpable for Wake {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl PartialEq for Wake {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at && self.aff == other.aff
    }
}

impl Eq for Wake {}

impl PartialOrd for Wake {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.at.partial_cmp(&self.at)
    }
}

impl Ord for Wake {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_op_affinity<'a, 'b>(op: &PrioOpRef<'a, 'b>) -> Affinity {
    use zhc_langs::hpulang::HpuInstructionSet::*;
    match op.get_instruction() {
        AddCt
        | SubCt
        | Mac { .. }
        | AddPt
        | SubPt
        | PtSub
        | MulPt
        | AddCst { .. }
        | SubCst { .. }
        | CstSub { .. }
        | MulCst { .. } => Affinity::Pea,
        CstCt { .. } => Affinity::Ctl,
        ImmLd { .. } | DstSt { .. } | SrcLd { .. } => Affinity::Pem,
        Batch { .. } => Affinity::Pep,
        _ => unreachable!(),
    }
}

fn get_op_latency<'a, 'b>(op: &PrioOpRef<'a, 'b>, config: &HpuConfig) -> Cycle {
    match get_op_affinity(op) {
        Affinity::Pea => ConstantLatency::new(config.alu_write_latency).compute_latency(),
        Affinity::Pem => ConstantLatency::new(config.mem_write_latency).compute_latency(),
        Affinity::Pep => {
            let zhc_langs::hpulang::HpuInstructionSet::Batch { block } = op.get_instruction()
            else {
                unreachable!()
            };
            let batch_size = block
                .walk_ops_linear()
                .filter(|op| op.get_instruction().is_pbs())
                .count();
            FlatLinLatency::new(
                config.pbs_processing_latency_a,
                config.pbs_processing_latency_b,
                config.pbs_processing_latency_m,
            )
            .compute_latency(batch_size)
        }
        Affinity::Ctl => Cycle(0),
    }
}

pub fn schedule<'a>(ir: &'a IR<HpuLang>, config: &HpuConfig, policy: SchedPolicy) -> IR<HpuLang> {
    let air = analyze_prio(ir, policy);
    let schedule = schedule_inner(&air, config, policy);
    match policy {
        SchedPolicy::AsSoonAsPossible => reschedule(ir, schedule.into_iter()).0,
        SchedPolicy::AsLateAsPossible => reschedule(ir, schedule.into_iter().rev()).0,
    }
}

fn schedule_inner<'a, 'b>(
    anir: &'b PrioIR<'a>,
    config: &HpuConfig,
    policy: SchedPolicy,
) -> Vec<OpId> {
    let mut output = Vec::new();

    let mut states = anir.totally_mapped_opmap(|op| {
        let count = match policy {
            SchedPolicy::AsSoonAsPossible => op.get_predecessors_iter().count(),
            SchedPolicy::AsLateAsPossible => op.get_users_iter().count(),
        };
        match count {
            0 => State::Ready,
            n => State::Waiting(n),
        }
    });

    let mut pep = ProcessingElement::new(
        states
            .iter()
            .filter(|(opid, state)| {
                matches!(
                    (state, get_op_affinity(&anir.get_op(*opid))),
                    (State::Ready, Affinity::Pep)
                )
            })
            .map(|(opid, _)| anir.get_op(opid).into()),
    );
    let mut pea = ProcessingElement::new(
        states
            .iter()
            .filter(|(opid, state)| {
                matches!(
                    (state, get_op_affinity(&anir.get_op(*opid))),
                    (State::Ready, Affinity::Pea)
                )
            })
            .map(|(opid, _)| anir.get_op(opid).into()),
    );
    let mut pem = ProcessingElement::new(
        states
            .iter()
            .filter(|(opid, state)| {
                matches!(
                    (state, get_op_affinity(&anir.get_op(*opid))),
                    (State::Ready, Affinity::Pem)
                )
            })
            .map(|(opid, _)| anir.get_op(opid).into()),
    );
    let mut ctl = ProcessingElement::new(
        states
            .iter()
            .filter(|(opid, state)| {
                matches!(
                    (state, get_op_affinity(&anir.get_op(*opid))),
                    (State::Ready, Affinity::Ctl)
                )
            })
            .map(|(opid, _)| anir.get_op(opid).into()),
    );

    let mut events = BinaryHeap::new();

    if let Some((opid, dur)) = ctl.kick(config) {
        output.push(opid);
        events.push(Wake {
            at: dur,
            aff: Affinity::Ctl,
        });
    }
    if let Some((opid, dur)) = pep.kick(config) {
        output.push(opid);
        events.push(Wake {
            at: dur,
            aff: Affinity::Pep,
        });
    }
    if let Some((opid, dur)) = pem.kick(config) {
        output.push(opid);
        events.push(Wake {
            at: dur,
            aff: Affinity::Pem,
        });
    }
    if let Some((opid, dur)) = pea.kick(config) {
        output.push(opid);
        events.push(Wake {
            at: dur,
            aff: Affinity::Pea,
        });
    }

    let process_neighbor = |neighbor: PrioOpRef<'a, 'b>,
                            states: &mut zhc_ir::OpMap<State>,
                            pea: &mut ProcessingElement<'a, 'b>,
                            pem: &mut ProcessingElement<'a, 'b>,
                            pep: &mut ProcessingElement<'a, 'b>,
                            ctl: &mut ProcessingElement<'a, 'b>| {
        states
            .get_mut(&neighbor)
            .unwrap()
            .transition(|old| match old {
                State::Waiting(0) => unreachable!(),
                State::Waiting(1) => match get_op_affinity(&neighbor) {
                    Affinity::Pea => {
                        pea.push(neighbor.into());
                        State::Ready
                    }
                    Affinity::Pem => {
                        pem.push(neighbor.into());
                        State::Ready
                    }
                    Affinity::Pep => {
                        pep.push(neighbor.into());
                        State::Ready
                    }
                    Affinity::Ctl => {
                        ctl.push(neighbor.into());
                        State::Scheduled
                    }
                },
                State::Waiting(n) => State::Waiting(n - 1),
                state => unreachable!("Found unexpected state {state:?}"),
            });
    };

    loop {
        let Some(Wake { at, aff }) = events.pop() else {
            break;
        };
        let current_cycle = at;
        let op = match aff {
            Affinity::Pea => pea.land(),
            Affinity::Pem => pem.land(),
            Affinity::Pep => pep.land(),
            Affinity::Ctl => ctl.land(),
        };

        match policy {
            SchedPolicy::AsSoonAsPossible => {
                for neighbor in op.get_users_iter() {
                    process_neighbor(
                        neighbor,
                        &mut states,
                        &mut pea,
                        &mut pem,
                        &mut pep,
                        &mut ctl,
                    );
                }
            }
            SchedPolicy::AsLateAsPossible => {
                for neighbor in op.get_predecessors_iter() {
                    process_neighbor(
                        neighbor,
                        &mut states,
                        &mut pea,
                        &mut pem,
                        &mut pep,
                        &mut ctl,
                    );
                }
            }
        }

        if ctl.is_idle() {
            if let Some((opid, dur)) = ctl.kick(config) {
                output.push(opid);
                events.push(Wake {
                    at: current_cycle + dur,
                    aff: Affinity::Ctl,
                });
            }
        }
        if pep.is_idle() {
            if let Some((opid, dur)) = pep.kick(config) {
                output.push(opid);
                events.push(Wake {
                    at: current_cycle + dur,
                    aff: Affinity::Pep,
                });
            }
        }
        if pem.is_idle() {
            if let Some((opid, dur)) = pem.kick(config) {
                output.push(opid);
                events.push(Wake {
                    at: current_cycle + dur,
                    aff: Affinity::Pem,
                });
            }
        }
        if pea.is_idle() {
            if let Some((opid, dur)) = pea.kick(config) {
                output.push(opid);
                events.push(Wake {
                    at: current_cycle + dur,
                    aff: Affinity::Pea,
                });
            }
        }
    }

    output
}

#[cfg(test)]
mod test {
    use zhc_builder::{CiphertextSpec, count_0};
    use zhc_config::hpu::PhysicalConfig;
    use zhc_ir::IR;
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_utils::assert_display_is;

    use super::*;
    use crate::{
        hpu::{lowering::lower_iop_to_hpu, scheduler::legacy::batcher::batch},
        test::check_iop_hpu_equivalence,
    };

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = lower_iop_to_hpu(ir).translation.output;
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let batched = batch(&ir, &config, SchedPolicy::AsSoonAsPossible);
        schedule(&batched, &config, SchedPolicy::AsSoonAsPossible)
    }

    #[test]
    fn test_scheduler() {
        let ir = pipeline(&count_0(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 = cst_ct<0_imm>();
                %1 = src_ld<0.0_tsrc>();
                %2 = src_ld<0.7_tsrc>();
                %3 = src_ld<0.1_tsrc>();
                %4 = src_ld<0.6_tsrc>();
                %5 = src_ld<0.2_tsrc>();
                %6 = src_ld<0.5_tsrc>();
                %7 = src_ld<0.3_tsrc>();
                %8 = src_ld<0.4_tsrc>();
                %9, %10, %11, %12, %13, %14, %15, %16, %17, %18, %19, %20, %21, %22, %23, %24 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = batch_arg<5, CtRegister>();
                    %a6 = batch_arg<6, CtRegister>();
                    %a7 = batch_arg<7, CtRegister>();
                    %a8, %a9 = pbs_2<Lut@71>(%a0);
                    %a10, %a11 = pbs_2<Lut@71>(%a1);
                    %a12, %a13 = pbs_2<Lut@71>(%a2);
                    %a14, %a15 = pbs_2<Lut@71>(%a3);
                    %a16, %a17 = pbs_2<Lut@71>(%a4);
                    %a18, %a19 = pbs_2<Lut@71>(%a5);
                    %a20, %a21 = pbs_2<Lut@71>(%a6);
                    %a22, %a23 = pbs_2f<Lut@71>(%a7);
                    batch_ret<0, CtRegister>(%a8);
                    batch_ret<1, CtRegister>(%a9);
                    batch_ret<2, CtRegister>(%a10);
                    batch_ret<3, CtRegister>(%a11);
                    batch_ret<4, CtRegister>(%a12);
                    batch_ret<5, CtRegister>(%a13);
                    batch_ret<6, CtRegister>(%a14);
                    batch_ret<7, CtRegister>(%a15);
                    batch_ret<8, CtRegister>(%a16);
                    batch_ret<9, CtRegister>(%a17);
                    batch_ret<10, CtRegister>(%a18);
                    batch_ret<11, CtRegister>(%a19);
                    batch_ret<12, CtRegister>(%a20);
                    batch_ret<13, CtRegister>(%a21);
                    batch_ret<14, CtRegister>(%a22);
                    batch_ret<15, CtRegister>(%a23);
                }(%1, %3, %5, %7, %8, %6, %4, %2);
                dst_st<0.7_tdst>(%0);
                dst_st<0.3_tdst>(%0);
                dst_st<0.6_tdst>(%0);
                dst_st<0.4_tdst>(%0);
                dst_st<0.5_tdst>(%0);
                %25 = add_ct(%9, %10);
                %26 = add_ct(%16, %17);
                %27 = add_ct(%25, %11);
                %28 = add_ct(%26, %18);
                %29 = add_ct(%27, %12);
                %30 = add_ct(%28, %19);
                %31 = add_ct(%29, %13);
                %32 = add_ct(%30, %20);
                %33 = add_ct(%31, %14);
                %34 = add_ct(%32, %21);
                %35 = add_ct(%33, %15);
                %36 = add_ct(%34, %22);
                %37 = add_ct(%23, %24);
                %38, %39, %40, %41, %42, %43 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3, %a4 = pbs_2<Lut@65>(%a2);
                    %a5, %a6 = pbs_2<Lut@70>(%a0);
                    %a7, %a8 = pbs_2f<Lut@70>(%a1);
                    batch_ret<0, CtRegister>(%a5);
                    batch_ret<1, CtRegister>(%a6);
                    batch_ret<2, CtRegister>(%a7);
                    batch_ret<3, CtRegister>(%a8);
                    batch_ret<4, CtRegister>(%a3);
                    batch_ret<5, CtRegister>(%a4);
                }(%35, %36, %37);
                %44 = add_ct(%38, %40);
                %45 = add_ct(%39, %41);
                %46 = add_ct(%44, %42);
                %47 = add_ct(%45, %43);
                %48, %49, %50, %51 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = pbs<Lut@1>(%a0);
                    %a3 = pbs<Lut@3>(%a0);
                    %a4, %a5 = pbs_2f<Lut@26>(%a1);
                    batch_ret<0, CtRegister>(%a2);
                    batch_ret<1, CtRegister>(%a3);
                    batch_ret<2, CtRegister>(%a4);
                    batch_ret<3, CtRegister>(%a5);
                }(%46, %47);
                dst_st<0.0_tdst>(%48);
                %52 = add_ct(%49, %50);
                %53, %54 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1, %a2 = pbs_2f<Lut@26>(%a0);
                    batch_ret<0, CtRegister>(%a1);
                    batch_ret<1, CtRegister>(%a2);
                }(%52);
                dst_st<0.1_tdst>(%53);
                %55 = add_ct(%54, %51);
                %56, %57 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1, %a2 = pbs_2f<Lut@26>(%a0);
                    batch_ret<0, CtRegister>(%a1);
                    batch_ret<1, CtRegister>(%a2);
                }(%55);
                dst_st<0.2_tdst>(%56);
            "#
        )
    }

    #[test]
    fn correctness() {
        use zhc_builder::*;
        let check = |b: Builder| {
            let spec = *b.spec();
            let iop_ir = b.optimize_ir();
            let hpu_ir = pipeline(&iop_ir);
            check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
        };
        for size in 2..=64 {
            let spec = CiphertextSpec::new(size, 2, 2);
            check(add(spec));
            check(bitwise_and(spec));
            check(bitwise_or(spec));
            check(bitwise_xor(spec));
            check(if_then_else(spec));
            check(if_then_zero(spec));
            check(mul(spec));
            check(div(spec));
        }
    }
}
