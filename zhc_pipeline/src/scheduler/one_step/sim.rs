use super::*;
use serde::Serialize;
use std::{collections::VecDeque, fmt::Display};
use zhc_ir::{AnnIR, AnnOpRef, OpId, OpMap, ValMap};
use zhc_langs::hpulang::HpuLang;
use zhc_sim::{
    Cycle, Tracer, TracingLevel,
    hpu::{ConstantLatency, FlatLinLatency, HpuConfig},
};
use zhc_utils::{
    Dumpable, fsm,
    iter::{CollectInSmallVec, CollectInVec, DedupedByKey, ReconcilerOf2},
    small::SmallVec,
    svec,
};

type StatOpRef<'a, 'b> = AnnOpRef<'a, 'b, HpuLang, Stats, ()>;

#[fsm]
#[derive(Debug, Clone)]
enum ProcessingElementState<'a, 'b> {
    Idle,
    Running(Vec<StatOpRef<'a, 'b>>),
}

impl<'a, 'b> ProcessingElementState<'a, 'b> {
    pub fn is_idle(&self) -> bool {
        matches!(self, ProcessingElementState::Idle)
    }

    pub fn as_counter(&self) -> f64 {
        match self {
            ProcessingElementState::Idle => 0.,
            ProcessingElementState::Running(ann_op_refs) => ann_op_refs.len() as f64,
            _ => unreachable!(),
        }
    }
}

#[fsm]
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
enum OpState {
    Landed,
    Running,
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
    InFlight(usize),
    Retired,
}

impl Dumpable for ValState {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Events {
    Start,
    ReadyPep,
    LandPep,
    LandPea,
    LandPem,
    LandCtl,
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl zhc_sim::Event for Events {}

#[derive(Debug, Clone)]
pub enum SchedElm {
    Op(OpId),
    Batch(SmallVec<OpId>),
}

#[derive(Clone)]
pub struct LightHpu<'a, 'b> {
    pe_mem_ready: VecDeque<StatOpRef<'a, 'b>>,
    pe_mem_state: ProcessingElementState<'a, 'b>,
    pe_mem_cost: ConstantLatency,
    pe_alu_ready: VecDeque<StatOpRef<'a, 'b>>,
    pe_alu_state: ProcessingElementState<'a, 'b>,
    pe_alu_cost: ConstantLatency,
    pe_pbs_ready: VecDeque<StatOpRef<'a, 'b>>,
    pe_pbs_state: ProcessingElementState<'a, 'b>,
    pe_pbs_latests: SmallVec<OpId>,
    pe_pbs_cost: FlatLinLatency,
    pe_ctl_ready: VecDeque<StatOpRef<'a, 'b>>,
    pe_ctl_state: ProcessingElementState<'a, 'b>,
    pe_ctl_cost: ConstantLatency,
    op_states: OpMap<OpState>,
    val_states: ValMap<ValState>,
    pub schedule: VecDeque<SchedElm>,
    ir: &'b AnnIR<'a, HpuLang, Stats, ()>,
    config: HpuConfig,
    policy: SchedPolicy,
}

impl<'a, 'b> Serialize for LightHpu<'a, 'b> {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        panic!()
    }
}

impl<'a, 'b> LightHpu<'a, 'b> {
    pub fn new(
        ir: &'b AnnIR<'a, HpuLang, Stats, ()>,
        config: &HpuConfig,
        policy: SchedPolicy,
    ) -> Self {
        let op_states = match policy {
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
        LightHpu {
            pe_mem_ready: VecDeque::new(),
            pe_mem_state: ProcessingElementState::Idle,
            pe_mem_cost: ConstantLatency::new(config.mem_write_latency),
            pe_alu_ready: VecDeque::new(),
            pe_alu_state: ProcessingElementState::Idle,
            pe_alu_cost: ConstantLatency::new(config.alu_write_latency),
            pe_pbs_ready: VecDeque::new(),
            pe_pbs_state: ProcessingElementState::Idle,
            pe_pbs_cost: FlatLinLatency::new(
                config.pbs_processing_latency_a,
                config.pbs_processing_latency_b,
                config.pbs_processing_latency_m,
            ),
            pe_pbs_latests: svec![],
            pe_ctl_ready: VecDeque::new(),
            pe_ctl_state: ProcessingElementState::Idle,
            pe_ctl_cost: ConstantLatency::new(0),
            op_states,
            val_states,
            schedule: VecDeque::new(),
            ir,
            config: config.to_owned(),
            policy,
        }
    }

    fn sched(&mut self, elm: SchedElm) {
        match self.policy {
            SchedPolicy::AsSoonAsPossible => {
                self.schedule.push_back(elm);
            }
            SchedPolicy::AsLateAsPossible => {
                self.schedule.push_front(elm);
            }
        }
    }

    fn pop_ctl(&mut self) -> AnnOpRef<'a, 'b, HpuLang, Stats, ()> {
        self.pe_ctl_ready
            .make_contiguous()
            .sort_by_key(|op| match self.policy {
                SchedPolicy::AsSoonAsPossible => op.get_annotation().height,
                SchedPolicy::AsLateAsPossible => op.get_annotation().depth,
            });
        self.pe_ctl_ready.pop_back().unwrap()
    }

    fn pop_alu(&mut self) -> AnnOpRef<'a, 'b, HpuLang, Stats, ()> {
        let mslice = self.pe_alu_ready.make_contiguous();
        mslice.sort_by_key(|op| {
            let criticallity = match self.policy {
                SchedPolicy::AsSoonAsPossible => op.get_annotation().height,
                SchedPolicy::AsLateAsPossible => op.get_annotation().depth,
            };
            criticallity
        });
        self.pe_alu_ready.pop_back().unwrap()
    }

    fn pop_mem(&mut self) -> AnnOpRef<'a, 'b, HpuLang, Stats, ()> {
        let mslice = self.pe_mem_ready.make_contiguous();
        mslice.sort_by_key(|op| {
            let criticallity = match self.policy {
                SchedPolicy::AsSoonAsPossible => op.get_annotation().height,
                SchedPolicy::AsLateAsPossible => op.get_annotation().depth,
            };
            criticallity
        });
        self.pe_mem_ready.pop_back().unwrap()
    }

    fn is_hpu_stalled(&self) -> bool {
        self.op_states
            .iter()
            .all(|(_, st)| !matches!(st, OpState::Running))
    }

    fn pop_pbs(&mut self) -> Vec<AnnOpRef<'a, 'b, HpuLang, Stats, ()>> {
        if self.pe_pbs_ready.len() <= self.config.pbs_max_batch_size {
            return self.pe_pbs_ready.drain(..).covec();
        }
        let mslice = self.pe_pbs_ready.make_contiguous();
        mslice.sort_by_key(|op| {
            let criticallity = match self.policy {
                SchedPolicy::AsSoonAsPossible => op.get_annotation().height,
                SchedPolicy::AsLateAsPossible => op.get_annotation().depth,
            };
            criticallity
        });
        mslice.reverse();
        self.pe_pbs_ready
            .drain(..self.config.pbs_max_batch_size)
            .covec()
    }

    fn get_initials(&self, affinity: Affinity) -> impl Iterator<Item = StatOpRef<'a, 'b>> {
        self.op_states
            .iter()
            .filter(move |(opid, state)| {
                **state == OpState::Ready && Affinity::extract(&self.ir.get_op(*opid)) == affinity
            })
            .map(|(opid, _)| self.ir.get_op(opid).into())
    }

    fn land_ops(&mut self, ops_to_land: Vec<StatOpRef<'a, 'b>>) {
        for op in ops_to_land.into_iter() {
            self.op_states
                .get_mut(&op)
                .unwrap()
                .transition(|old| match old {
                    OpState::Running => OpState::Landed,
                    _ => unreachable!(),
                });
            let iterator = match self.policy {
                SchedPolicy::AsSoonAsPossible => op.get_users_iter().reconcile_1_of_2(),
                SchedPolicy::AsLateAsPossible => op.get_predecessors_iter().reconcile_2_of_2(),
            };
            for user in iterator {
                self.op_states
                    .get_mut(&user)
                    .unwrap()
                    .transition(|old| match old {
                        OpState::Waiting(0) => {
                            unreachable!()
                        }
                        OpState::Waiting(1) => match Affinity::extract(&user) {
                            Affinity::Pea => {
                                self.pe_alu_ready.push_front(user.into());
                                OpState::Ready
                            }
                            Affinity::Pem => {
                                self.pe_mem_ready.push_front(user.into());
                                OpState::Ready
                            }
                            Affinity::Pep => {
                                self.pe_pbs_ready.push_front(user.into());
                                OpState::Ready
                            }
                            Affinity::Ctl => {
                                self.pe_ctl_ready.push_front(user.into());
                                OpState::Ready
                            }
                        },
                        OpState::Waiting(n) => OpState::Waiting(n - 1),
                        state => {
                            unreachable!("Found unexpected state {state:?} for op: {}", op.format())
                        }
                    });
            }

            for val in op
                .get_args_iter()
                .dedup_by_key(|a| a.get_id())
                .chain(op.get_returns_iter())
            {
                self.val_states
                    .get_mut(&val)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => match val.get_users_iter().count() {
                            0 => ValState::Retired,
                            n => ValState::InFlight(n),
                        },
                        ValState::InFlight(0) => {
                            unreachable!()
                        }
                        ValState::InFlight(1) => ValState::Retired,
                        ValState::InFlight(n) => ValState::InFlight(n - 1),
                        state => {
                            unreachable!(
                                "Found unexpected state {state:?} for val: {}",
                                val.format()
                            )
                        }
                    })
            }
        }
    }

    fn should_fire_pea(&self) -> bool {
        self.pe_alu_state.is_idle() && !self.pe_alu_ready.is_empty()
    }

    fn should_fire_pem(&self) -> bool {
        self.pe_mem_state.is_idle() && !self.pe_mem_ready.is_empty()
    }

    fn should_fire_ctl(&self) -> bool {
        self.pe_ctl_state.is_idle() && !self.pe_ctl_ready.is_empty()
    }

    fn should_fire_pep(&self) -> bool {
        self.pe_pbs_state.is_idle() && !self.pe_pbs_ready.is_empty() && self.is_hpu_stalled()
    }
}

impl<'a, 'b> zhc_sim::Simulatable for LightHpu<'a, 'b> {
    type Event = Events;

    fn power_up(&mut self, dispatch: &mut impl zhc_sim::Dispatch<Event = Self::Event>) {
        self.pe_mem_ready
            .extend(self.get_initials(Affinity::Pem).covec().into_iter());
        self.pe_alu_ready
            .extend(self.get_initials(Affinity::Pea).covec().into_iter());
        self.pe_pbs_ready
            .extend(self.get_initials(Affinity::Pep).covec().into_iter());
        self.pe_ctl_ready
            .extend(self.get_initials(Affinity::Ctl).covec().into_iter());
        dispatch.dispatch_after(Cycle(1), Events::Start);
    }

    fn handle(
        &mut self,
        dispatcher: &mut impl zhc_sim::Dispatch<Event = Self::Event>,
        trigger: zhc_sim::Trigger<Self::Event>,
    ) {
        let ops_to_land = match trigger.event {
            Events::ReadyPep => return,
            Events::Start => vec![],
            Events::LandPep => self.pe_pbs_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            Events::LandPea => self.pe_alu_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            Events::LandPem => self.pe_mem_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            Events::LandCtl => self.pe_ctl_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
        };

        self.land_ops(ops_to_land);

        if self.should_fire_ctl() {
            let op = self.pop_ctl();
            self.sched(SchedElm::Op(op.get_id()));
            self.op_states
                .get_mut(&op)
                .unwrap()
                .transition(|old| match old {
                    OpState::Ready => OpState::Running,
                    _ => unreachable!(),
                });
            self.pe_ctl_state.transition(|old| match old {
                ProcessingElementState::Idle => ProcessingElementState::Running(vec![op]),
                _ => unreachable!(),
            });
            dispatcher.dispatch_after(self.pe_ctl_cost.compute_latency(), Events::LandCtl);
        }
        if self.should_fire_pea() {
            let op = self.pop_alu();
            self.sched(SchedElm::Op(op.get_id()));
            self.op_states
                .get_mut(&op)
                .unwrap()
                .transition(|old| match old {
                    OpState::Ready => OpState::Running,
                    _ => unreachable!(),
                });
            self.pe_alu_state.transition(|old| match old {
                ProcessingElementState::Idle => ProcessingElementState::Running(vec![op]),
                _ => unreachable!(),
            });
            dispatcher.dispatch_after(self.pe_alu_cost.compute_latency(), Events::LandPea);
        }
        if self.should_fire_pem() {
            let op = self.pop_mem();
            self.sched(SchedElm::Op(op.get_id()));
            self.op_states
                .get_mut(&op)
                .unwrap()
                .transition(|old| match old {
                    OpState::Ready => OpState::Running,
                    _ => unreachable!(),
                });
            self.pe_mem_state.transition(|old| match old {
                ProcessingElementState::Idle => ProcessingElementState::Running(vec![op]),
                _ => unreachable!(),
            });
            dispatcher.dispatch_after(self.pe_mem_cost.compute_latency(), Events::LandPem);
        }
        if self.should_fire_pep() {
            let ops = self.pop_pbs();
            dispatcher.dispatch_after(self.pe_pbs_cost.compute_latency(ops.len()), Events::LandPep);
            self.pe_pbs_latests = ops.iter().map(|a| a.get_id()).cosvec();
            self.sched(SchedElm::Batch(self.pe_pbs_latests.clone()));
            for op in ops.iter() {
                self.op_states
                    .get_mut(op)
                    .unwrap()
                    .transition(|old| match old {
                        OpState::Ready => OpState::Running,
                        _ => unreachable!(),
                    });
            }
            self.pe_pbs_state.transition(|old| match old {
                ProcessingElementState::Idle => ProcessingElementState::Running(ops),
                _ => unreachable!(),
            });
        }
    }

    fn report<'t>(&self, at: Cycle, tracer: &mut Tracer, tracing_level: TracingLevel) {
        tracer.add_state(tracing_level, at, None, self.name(), self);

        // ── PE occupancy: how many ops each unit is currently executing.
        //    pe_pbs_load doubles as the size of the batch currently running.
        let alu = self.pe_alu_state.as_counter();
        let pbs = self.pe_pbs_state.as_counter();
        tracer.add_counter(tracing_level, at, None, "pe_alu_load", alu);
        tracer.add_counter(tracing_level, at, None, "pe_pbs_load", pbs);

        // ── Ready-queue depths: independent work waiting on each unit.
        //    `pbs_ready` is the rib reservoir — when it collapses toward 0 while
        //    work remains, the PEP is about to starve (the spine tail).
        tracer.add_counter(
            tracing_level,
            at,
            None,
            "ready_pbs",
            self.pe_pbs_ready.len() as f64,
        );
        tracer.add_counter(
            tracing_level,
            at,
            None,
            "ready_alu",
            self.pe_alu_ready.len() as f64,
        );

        // ── Register pressure proxy: values in flight. Crossing regf_size ⇒ spills.
        let live = self
            .val_states
            .iter()
            .filter(|a| matches!(a.1, ValState::InFlight(_)))
            .count();
        tracer.add_counter(tracing_level, at, None, "live_values", live as f64);

        // ── Monotone progress: schedule elements emitted so far.
        tracer.add_counter(tracing_level, at, None, "scheduled", self.schedule.len() as f64);

        // let name = format!("schedule_{}.html", at.0);
        // let ann_ir = AnnIR::new(self.ir, self.op_states.clone(), self.val_states.clone());
        // ann_ir.draw_to_html(None, &name);
        // tracer.add_state(tracing_level, at, "graph", &name);
    }
}
