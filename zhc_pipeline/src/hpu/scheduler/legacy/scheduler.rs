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
