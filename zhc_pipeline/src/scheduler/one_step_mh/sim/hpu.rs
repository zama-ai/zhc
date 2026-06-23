use super::*;
use serde::Serialize;
use std::{collections::VecDeque, fmt::Display};
use zhc_ir::{AnnIR, AnnOpRef, OpId, OpMap, visualization::VisualAnnotation};
use zhc_langs::hpulang::{HpuId, HpuInstructionSet, HpuLang};
use zhc_sim::{
    Cycle, Tracer, TracingLevel,
    hpu::{ConstantLatency, FlatLinLatency, HpuConfig},
};
use zhc_utils::{
    Dumpable, fsm,
    iter::{CollectInSmallVec, CollectInVec, ReconcilerOf2},
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
}

#[fsm]
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum OpState {
    Landed,
    Running,
    Ready,
    Awaiting,
    Waiting(usize),
    NotConcerned,
}

impl Dumpable for OpState {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl VisualAnnotation for OpState {}

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

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum HpuEvents {
    Start,
    LandPea,
    ReadyPep,
    LandPep,
    LandPem,
    LandCtl,
    LandTransfer,
    TransferOut(HpuId, OpId),
    TransferIn(OpId)
}

impl Display for HpuEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl zhc_sim::Event for HpuEvents {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedElm {
    Op(OpId),
    Batch(SmallVec<OpId>),
}

impl Dumpable for SchedElm {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
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
    pe_transfer_ready: VecDeque<StatOpRef<'a, 'b>>,
    pe_transfer_state: ProcessingElementState<'a, 'b>,
    pe_transfer_cost: ConstantLatency,
    pub op_states: OpMap<OpState>,
    pub schedule: VecDeque<SchedElm>,
    ir: &'b AnnIR<'a, HpuLang, Stats, ()>,
    config: HpuConfig,
    policy: SchedPolicy,
    pub id: HpuId,
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
        id: HpuId,
    ) -> Self {
        let op_states = match policy {
            SchedPolicy::AsSoonAsPossible => ir.totally_mapped_opmap(|op| {
                if !ir
                    .get_op(&op)
                    .get_annotation()
                    .locality
                    .is_on(&id)
                {
                    OpState::NotConcerned
                } else {
                    if let HpuInstructionSet::Transfer{ from, to, .. } = op.get_instruction() {
                        if id == from {
                            OpState::Waiting(1)
                        } else if id == to {
                            OpState::Awaiting
                        } else {
                            unreachable!()
                        }
                    } else {
                        match op.get_predecessors_iter().filter(|a| ir.get_op(a.get_id()).get_annotation().locality.is_on(&id)).count() {
                            0 => OpState::Ready,
                            n => OpState::Waiting(n),
                        }
                    }
                }
            }),
            SchedPolicy::AsLateAsPossible => ir.totally_mapped_opmap(|op| {
                if !ir
                    .get_op(&op)
                    .get_annotation()
                    .locality
                    .is_on(&id)
                {
                    OpState::NotConcerned
                } else {
                    if let HpuInstructionSet::Transfer{ from, to } = op.get_instruction() {
                        if id == to {
                            OpState::Waiting(op.get_users_iter().count())
                        } else if id == from {
                            OpState::Awaiting
                        } else {
                            unreachable!()
                        }
                    } else {
                        match op.get_users_iter().filter(|a| ir.get_op(a.get_id()).get_annotation().locality.is_on(&id)).count() {
                            0 => OpState::Ready,
                            n => OpState::Waiting(n),
                        }
                    }
                }
            }),
        };

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
            pe_transfer_ready: VecDeque::new(),
            pe_transfer_state: ProcessingElementState::Idle,
            pe_transfer_cost: ConstantLatency::new(0),
            op_states,
            schedule: VecDeque::new(),
            ir,
            config: config.to_owned(),
            policy,
            id,
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

    fn pop_transfer(&mut self) -> AnnOpRef<'a, 'b, HpuLang, Stats, ()> {
        self.pe_transfer_ready
            .make_contiguous()
            .sort_by_key(|op| match self.policy {
                SchedPolicy::AsSoonAsPossible => op.get_annotation().height,
                SchedPolicy::AsLateAsPossible => op.get_annotation().depth,
            });
        self.pe_transfer_ready.pop_back().unwrap()
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
                            Affinity::Transfer => {
                                self.pe_transfer_ready.push_front(user.into());
                                OpState::Ready
                            }
                        },
                        OpState::Waiting(n) => OpState::Waiting(n - 1),
                        OpState::NotConcerned => OpState::NotConcerned,
                        state => {
                            unreachable!("Found unexpected state {state:?} for op: {}", op.format())
                        }
                    });
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

    fn should_fire_transfer(&self) -> bool {
        self.pe_transfer_state.is_idle() && !self.pe_transfer_ready.is_empty()
    }

    fn should_fire_pep(&self) -> bool {
        self.pe_pbs_state.is_idle() && !self.pe_pbs_ready.is_empty() && self.is_hpu_stalled()
    }
}

impl<'a, 'b> zhc_sim::Simulatable for LightHpu<'a, 'b> {
    type Event = HpuEvents;

    fn power_up(&mut self, dispatch: &mut impl zhc_sim::Dispatch<Event = Self::Event>) {
        self.pe_mem_ready
            .extend(self.get_initials(Affinity::Pem).covec().into_iter());
        self.pe_alu_ready
            .extend(self.get_initials(Affinity::Pea).covec().into_iter());
        self.pe_pbs_ready
            .extend(self.get_initials(Affinity::Pep).covec().into_iter());
        self.pe_ctl_ready
            .extend(self.get_initials(Affinity::Ctl).covec().into_iter());
        self.pe_transfer_ready
            .extend(self.get_initials(Affinity::Transfer).covec().into_iter());
        dispatch.dispatch_after(Cycle(1), HpuEvents::Start);
    }

    fn handle(
        &mut self,
        dispatcher: &mut impl zhc_sim::Dispatch<Event = Self::Event>,
        trigger: zhc_sim::Trigger<Self::Event>,
    ) {
        let ops_to_land = match trigger.event {
            HpuEvents::ReadyPep => return,
            HpuEvents::Start => vec![],
            HpuEvents::LandPep => self.pe_pbs_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            HpuEvents::LandPea => self.pe_alu_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            HpuEvents::LandPem => self.pe_mem_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            HpuEvents::LandCtl => self.pe_ctl_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            HpuEvents::LandTransfer => self.pe_transfer_state.transition_with(|old| match old {
                ProcessingElementState::Running(ops) => (ProcessingElementState::Idle, ops),
                _ => unreachable!(),
            }),
            _ => vec![]
        };


        self.land_ops(ops_to_land);

        if let HpuEvents::TransferIn(opid) = trigger.event {
            let transfer = self.ir.get_op(opid);
            self.op_states
                .get_mut(&transfer)
                .unwrap()
                .transition(|old| match old {
                    OpState::Awaiting => {
                        self.pe_transfer_ready.push_front(transfer.into());
                        OpState::Ready
                    },
                    state => {
                        unreachable!("Found unexpected state when receiving transfer_in on {}: op {} is {state:?}.", self.id, transfer.format())
                    }
                });
        }

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
            dispatcher.dispatch_after(self.pe_ctl_cost.compute_latency(), HpuEvents::LandCtl);
        }
        if self.should_fire_transfer() {
            let op = self.pop_transfer();
            self.sched(SchedElm::Op(op.get_id()));
            self.op_states
                .get_mut(&op)
                .unwrap()
                .transition(|old| match old {
                    OpState::Ready => OpState::Running,
                    _ => unreachable!(),
                });
            self.pe_transfer_state.transition(|old| match old {
                ProcessingElementState::Idle => ProcessingElementState::Running(vec![op.clone()]),
                _ => unreachable!(),
            });
            let HpuInstructionSet::Transfer{ from, to } = op.get_instruction() else {unreachable!()};
            let target_hid = match self.policy {
                SchedPolicy::AsSoonAsPossible => to,
                SchedPolicy::AsLateAsPossible => from,
            };
            if target_hid != self.id {
                dispatcher.dispatch_after(self.pe_transfer_cost.compute_latency(), HpuEvents::TransferOut(target_hid, op.get_id()));
            }
            dispatcher.dispatch_after(self.pe_transfer_cost.compute_latency(), HpuEvents::LandTransfer);
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
            dispatcher.dispatch_after(self.pe_alu_cost.compute_latency(), HpuEvents::LandPea);
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
            dispatcher.dispatch_after(self.pe_mem_cost.compute_latency(), HpuEvents::LandPem);
        }
        if self.should_fire_pep() {
            let ops = self.pop_pbs();
            dispatcher.dispatch_after(self.pe_pbs_cost.compute_latency(ops.len()), HpuEvents::LandPep);
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

    fn report<'t>(&self, _at: Cycle, _tracer: &mut Tracer, _tracing_level: TracingLevel) {}
}
