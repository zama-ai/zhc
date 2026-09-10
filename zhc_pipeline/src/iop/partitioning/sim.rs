use std::{cmp::Reverse, fmt::Display};

use super::*;
use serde::Serialize;
use zhc_ir::{AsOpId, AsValId};
use zhc_sim::*;
use zhc_utils::{
    fsm,
    iter::{Intermediate, MultiZip, ReconcilerOf2},
    small::SmallSet,
    units::Cycle,
};

type PartOpRef<'a, 'b> = AnnOpRef<'a, 'b, TaskLang, HeightDepth, ()>;
type PartIR<'a> = AnnIR<'a, TaskLang, HeightDepth, ()>;

#[fsm]
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum OpState {
    Landed(PartitionId),
    Running(PartitionId),
    Ready,
    Waiting(usize),
}

impl Dumpable for OpState {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[fsm]
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
enum ValState {
    Preparing,
    Available(SmallSet<PartitionId>),
}

impl ValState {
    pub fn available_is_on_partition(&self, pid: PartitionId) -> bool {
        match self {
            ValState::Available(pids) => pids.contains(&pid),
            _ => unreachable!(),
        }
    }

    pub fn available_make_available(&mut self, pid: PartitionId) {
        match self {
            ValState::Available(pids) => pids.insert(pid),
            _ => unreachable!(),
        };
    }
}

#[fsm]
#[derive(Debug, Clone)]
pub enum PartitionState {
    Idle(Vec<OpId>),
    Running(Vec<OpId>),
}

impl PartitionState {
    pub fn idle_fill(&self) -> usize {
        match self {
            PartitionState::Idle(op_ids) => op_ids.len(),
            _ => unreachable!(),
        }
    }

    pub fn idle_push(&mut self, opid: OpId) {
        match self {
            PartitionState::Idle(op_ids) => {
                assert!(op_ids.capacity() > op_ids.len());
                op_ids.push(opid);
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Events {
    Start,
    Land(PartitionId),
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Event for Events {}

pub struct Partitioner<'ir, 'ann> {
    ready: Vec<PartOpRef<'ir, 'ann>>,
    ir: &'ann PartIR<'ir>,
    partition_states: Store<PartitionId, PartitionState>,
    op_states: OpMap<OpState>,
    val_states: ValMap<ValState>,
    config: PartitionerConfig,
}

impl<'a, 'b> Serialize for Partitioner<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        "PartitionCluster".serialize(serializer)
    }
}

impl<'ir, 'ann> Partitioner<'ir, 'ann> {
    pub fn new(ir: &'ann PartIR<'ir>, config: &PartitionerConfig) -> Self {
        let op_states = match config.sched_policy {
            SchedPolicy::AsSoonAsPossible => {
                ir.totally_mapped_opmap(|op| match op.get_predecessors_iter().count() {
                    0 => OpState::Ready,
                    n => OpState::Waiting(n),
                })
            }
            SchedPolicy::AsLateAsPossible => {
                ir.totally_mapped_opmap(|op| match op.get_users_iter().count() {
                    0 => OpState::Ready,
                    n => OpState::Waiting(n),
                })
            }
        };
        let val_states = ir.filled_valmap(ValState::Preparing);
        Partitioner {
            ready: Vec::new(),
            ir,
            partition_states: config
                .specs
                .iter()
                .map(|p| Vec::with_capacity(p.parallelism as usize))
                .map(PartitionState::Idle)
                .collect(),
            op_states,
            val_states,
            config: config.clone(),
        }
    }

    pub fn get_initials(&self) -> impl Iterator<Item = PartOpRef<'ir, 'ann>> {
        self.op_states
            .iter()
            .filter(|(_, state)| **state == OpState::Ready)
            .map(|(opid, _)| self.ir.get_op(opid).into())
    }

    pub fn is_stale(&self) -> bool {
        self.partition_states
            .iter()
            .all(|s| matches!(s, PartitionState::Idle(_)))
    }

    pub fn into_partition_map(self) -> OpMap<PartitionId> {
        self.op_states.map(|s| match s {
            OpState::Landed(partition_id) => partition_id,
            _ => unreachable!(),
        })
    }
}

impl<'ir, 'ann> Simulatable for Partitioner<'ir, 'ann> {
    type Event = Events;

    fn power_up(&mut self, dispatch: &mut impl Dispatch<Event = Self::Event>) {
        self.ready.extend(self.get_initials().intermediate());
        dispatch.dispatch_after(Cycle(1), Events::Start);
    }

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match trigger.event {
            Events::Land(pid) => {
                self.partition_states[pid].transition(|old| match old {
                    PartitionState::Running(mut op_ids) => {
                        for opid in op_ids.drain(..) {
                            self.op_states[opid].transition(|old| match old {
                                OpState::Running(partition_id) => OpState::Landed(partition_id),
                                _ => unreachable!(),
                            });
                            let op_iter = match self.config.sched_policy {
                                SchedPolicy::AsSoonAsPossible => {
                                    self.ir.get_op(opid).get_users_iter().reconcile_1_of_2()
                                }
                                SchedPolicy::AsLateAsPossible => self
                                    .ir
                                    .get_op(opid)
                                    .get_predecessors_iter()
                                    .reconcile_2_of_2(),
                            };
                            for op in op_iter {
                                self.op_states[op.get_id()].transition(|old| match old {
                                    OpState::Waiting(1) => {
                                        self.ready.push(op);
                                        OpState::Ready
                                    }
                                    OpState::Waiting(n) => OpState::Waiting(n - 1),
                                    _ => unreachable!(),
                                });
                            }
                            let val_iter = match self.config.sched_policy {
                                SchedPolicy::AsSoonAsPossible => {
                                    self.ir.get_op(opid).get_returns_iter().reconcile_1_of_2()
                                }
                                SchedPolicy::AsLateAsPossible => {
                                    self.ir.get_op(opid).get_args_iter().reconcile_2_of_2()
                                }
                            };
                            for valid in val_iter {
                                self.val_states[valid].transition(|old| match old {
                                    ValState::Preparing => {
                                        ValState::Available(std::iter::once(pid).collect())
                                    }
                                    ValState::Available(mut pids) => {
                                        pids.insert(pid);
                                        ValState::Available(pids)
                                    }
                                    _ => unreachable!(),
                                });
                            }
                        }
                        PartitionState::Idle(op_ids)
                    }
                    _ => unreachable!(),
                });
                if self.is_stale() {
                    dispatcher.dispatch_now(Events::Start);
                }
            }
            Events::Start => {
                self.ready.sort_by_key(|op| {
                    let criticallity = match self.config.sched_policy {
                        SchedPolicy::AsSoonAsPossible => op.get_annotation().height,
                        SchedPolicy::AsLateAsPossible => op.get_annotation().depth,
                    };
                    criticallity
                });

                loop {
                    let Some(op) = self.ready.pop() else { break };
                    #[derive(PartialEq, Eq, PartialOrd, Ord)]
                    struct PartitionScore {
                        latency: Reverse<Cycle>,
                        operands_available: usize,
                        tie_preference: isize,
                    }
                    let maybe_pid = (
                        self.partition_states.iter(),
                        self.config.specs.enumerate_iter(),
                    )
                        .mzip()
                        .map(|(s, (pid, c))| (pid, s, c))
                        .filter(|(_, s, c)| c.parallelism as usize > s.idle_fill())
                        .max_by_key(|(pid, state, c)| {
                            let operands_available = match self.config.sched_policy {
                                SchedPolicy::AsSoonAsPossible => op
                                    .get_arg_valids()
                                    .iter()
                                    .filter(|a| self.val_states[*a].available_is_on_partition(*pid))
                                    .count(),
                                SchedPolicy::AsLateAsPossible => op
                                    .get_return_valids()
                                    .iter()
                                    .filter(|a| self.val_states[*a].available_is_on_partition(*pid))
                                    .count(),
                            };
                            let fill = state.idle_fill() as isize;
                            let tie_preference = match self.config.tie_policy {
                                TiePolicy::Concentrate => fill,
                                TiePolicy::Spread => -fill,
                            };
                            PartitionScore {
                                latency: Reverse(c.latency),
                                operands_available,
                                tie_preference,
                            }
                        })
                        .map(|(pid, _, _)| pid);
                    match maybe_pid {
                        Some(pid) => {
                            self.partition_states[pid].idle_push(op.op_id());
                            self.op_states[op.op_id()].transition(|old| match old {
                                OpState::Ready => OpState::Running(pid),
                                _ => unreachable!(),
                            });
                            let val_iter = match self.config.sched_policy {
                                SchedPolicy::AsSoonAsPossible => {
                                    op.get_args_iter().reconcile_1_of_2()
                                }
                                SchedPolicy::AsLateAsPossible => {
                                    op.get_returns_iter().reconcile_2_of_2()
                                }
                            };
                            for val in val_iter {
                                self.val_states[val.val_id()].available_make_available(pid)
                            }
                        }
                        None => {
                            self.ready.push(op);
                            break;
                        }
                    }
                }

                self.partition_states
                    .enumerate_iter_mut()
                    .for_each(|(pid, s)| {
                        s.transition(|old| match old {
                            PartitionState::Idle(op_ids) => {
                                if !op_ids.is_empty() {
                                    dispatcher.dispatch_after(
                                        self.config.specs[pid].latency,
                                        Events::Land(pid),
                                    );
                                    PartitionState::Running(op_ids)
                                } else {
                                    PartitionState::Idle(op_ids)
                                }
                            }
                            _ => unreachable!(),
                        })
                    });
            }
        }
    }

    fn report<'t>(&self, _at: Cycle, _tracer: &mut Tracer, _tracing_level: TracingLevel) {}
}
