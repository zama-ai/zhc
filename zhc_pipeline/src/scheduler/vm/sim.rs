use super::*;
use serde::Serialize;
use std::{collections::VecDeque, fmt::Display};
use zhc_ir::{AnnIR, AnnOpRef, OpIdRaw, OpMap, ValMap};
use zhc_langs::vmlang::{VmByteCode, VmInstructionSet};
use zhc_sim::{Cycle, Tracer, TracingLevel};
use zhc_utils::{
    Dumpable, SafeAs, Store, StoreIndex, fsm,
    iter::{CollectInVec, DedupedByKey, ReconcilerOf2},
    small::SmallVec,
};

type StatOpRef<'a, 'b> = AnnOpRef<'a, 'b, VmLang, Stats, ()>;

#[fsm]
#[derive(Debug, Clone, Serialize)]
enum ThreadState<'a, 'b> {
    Idle,
    Running(StatOpRef<'a, 'b>),
}

impl<'a, 'b> ThreadState<'a, 'b> {
    pub fn is_idle(&self) -> bool {
        matches!(self, ThreadState::Idle)
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

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegId(pub u16);

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadId(pub u8);

impl StoreIndex for ThreadId {
    type Raw = u8;

    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    fn as_usize(&self) -> usize {
        self.0 as usize
    }

    fn raw_from_usize(val: usize) -> Self::Raw {
        val.sas()
    }

    fn from_usize(val: usize) -> Self {
        ThreadId(val.sas())
    }
}

#[fsm]
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
enum ValState {
    Preparing,
    InFlight(RegId, usize),
    Retired,
}

impl Dumpable for ValState {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Events {
    Start,
    LandThread(ThreadId),
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl zhc_sim::Event for Events {}

#[derive(Clone, Serialize)]
pub struct LightVm<'a, 'b> {
    ready: VecDeque<StatOpRef<'a, 'b>>,
    #[serde(skip)]
    ir: &'b AnnIR<'a, VmLang, Stats, ()>,
    threads: Store<ThreadId, ThreadState<'a, 'b>>,
    op_states: OpMap<OpState>,
    val_states: ValMap<ValState>,
    policy: SchedPolicy,
    // Free register pool. Each free register carries the anti-dependency set of its previous
    // occupant: the ops that must complete (finish reading/writing it) before it may be reused.
    #[serde(skip)]
    available_regs: VecDeque<(RegId, SmallVec<OpIdRaw>)>,
    // Anti-dependencies accumulated by `acquire_reg` for the op currently being scheduled.
    #[serde(skip)]
    pending_anti_deps: SmallVec<OpIdRaw>,
    pub last_introduced_reg: RegId,
    n_threads: u8,
    pub schedules: Store<ThreadId, VecDeque<VmByteCode>>,
    pub locks_table: Vec<u8>,
    #[serde(skip)]
    pub successors_table: Vec<SmallVec<OpIdRaw>>,
}

impl<'a, 'b> LightVm<'a, 'b> {
    pub fn new(ir: &'b AnnIR<'a, VmLang, Stats, ()>, n_threads: u8, policy: SchedPolicy) -> Self {
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
        let min_regs = (n_threads * 3) as u16;
        let tables_len = ir.walk_ops_linear().map(|a| a.get_id().0).max().unwrap() + 1;
        LightVm {
            val_states,
            schedules: Store::with_value(VecDeque::new(), n_threads as usize),
            ir,
            ready: VecDeque::new(),
            policy,
            threads: Store::with_value(ThreadState::Idle, n_threads as usize),
            op_states,
            n_threads,
            available_regs: (0..min_regs).map(|i| (RegId(i), SmallVec::new())).collect(),
            pending_anti_deps: SmallVec::new(),
            last_introduced_reg: RegId(min_regs),
            locks_table: vec![0; tables_len as usize],
            successors_table: vec![SmallVec::new(); tables_len as usize],
        }
    }

    fn acquire_reg(&mut self) -> RegId {
        match self.available_regs.pop_back() {
            // Reusing a register: the op being scheduled must wait for the previous occupant's
            // readers (WAR) / writer (WAW) before it may overwrite it. Record those as
            // anti-dependencies of the current op.
            Some((reg, anti)) => {
                self.pending_anti_deps.extend(anti.into_iter());
                reg
            }
            None => {
                let output = self.last_introduced_reg;
                self.last_introduced_reg.0 += 1;
                output
            }
        }
    }

    fn sched(&mut self, op: &StatOpRef<'a, 'b>, thread: ThreadId) {
        // Anti-dependencies picked up by the `acquire_reg` calls below accumulate here.
        self.pending_anti_deps.clear();
        let instr = match op.get_instruction() {
            VmInstructionSet::AddCt => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src1, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    op.dump_and_wait();
                    self
                                        .val_states
                                        .get(op.get_args_iter().nth(0).unwrap())
                                        .unwrap().dump_and_wait();

                    unreachable!()
                };
                let ValState::InFlight(src2, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(1).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::ADD {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src1: src1.0,
                    src2: src2.0,
                }
            }
            VmInstructionSet::SubCt => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src1, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                let ValState::InFlight(src2, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(1).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::SUB {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src1: src1.0,
                    src2: src2.0,
                }
            }
            VmInstructionSet::Mac { cst } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src1, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                let ValState::InFlight(src2, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(1).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::MAC {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src1: src1.0,
                    src2: src2.0,
                    cst: cst,
                }
            }
            VmInstructionSet::AddPt => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                let VmInstructionSet::ImmLd {
                    from_pos,
                    from_block
                } = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                else {
                    unreachable!()
                };
                VmByteCode::ADDS {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    s_id: from_pos.sas(),
                    s_blk: from_block.sas(),
                }
            }
            VmInstructionSet::SubPt => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                let VmInstructionSet::ImmLd {
                    from_pos,
                    from_block
                } = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                else {
                    unreachable!()
                };
                VmByteCode::SUBS {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    s_id: from_pos.sas(),
                    s_blk: from_block.sas(),
                }
            }
            VmInstructionSet::PtSub => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(1).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                let VmInstructionSet::ImmLd {
                    from_pos,
                    from_block
                } = op
                    .get_args_iter()
                    .nth(0)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                else {
                    unreachable!()
                };
                VmByteCode::SSUB {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    s_id: from_pos.sas(),
                    s_blk: from_block.sas(),
                }
            }
            VmInstructionSet::MulPt => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                let VmInstructionSet::ImmLd {
                    from_pos,
                    from_block
                } = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                else {
                    unreachable!()
                };
                VmByteCode::MULS {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    s_id: from_pos.sas(),
                    s_blk: from_block.sas(),
                }
            }
            VmInstructionSet::AddCst { cst } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::ADDC {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    cst: cst,
                }
            }
            VmInstructionSet::SubCst { cst } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::SUBC {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    cst: cst,
                }
            }
            VmInstructionSet::CstSub { cst } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::CSUB {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    cst: cst,
                }
            }
            VmInstructionSet::MulCst { cst } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::MULC {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    cst: cst,
                }
            }
            VmInstructionSet::CstCt { cst } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                VmByteCode::DEF {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    cst: cst,
                }
            }
            VmInstructionSet::ImmLd { .. } => {
                // No-op.
                return;
            }
            VmInstructionSet::DstSt { to_block, to_pos } => {
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    panic!("Encountered unexpected val state: {:?}",
                        self.val_states
                        .get(op.get_args_iter().nth(0).unwrap()))
                };
                VmByteCode::ST {
                    id: op.get_id().as_raw(),
                    dst_id: to_pos.sas(),
                    dst_blk: to_block.sas(),
                    src: src.0,
                }
            }
            VmInstructionSet::SrcLd { from_block, from_pos } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                VmByteCode::LD {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src_id: from_pos.sas(),
                    src_blk: from_block.sas(),
                }
            }
            VmInstructionSet::Ks => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::KS {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                }
            }
            VmInstructionSet::Pbs { lut } => {
                let dst = self.acquire_reg();
                let output = op.get_returns_iter().next().unwrap();
                self.val_states
                    .get_mut(&output)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst, output.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::PBS {
                    id: op.get_id().as_raw(),
                    dst: dst.0,
                    src: src.0,
                    lut: lut.sas(),
                }
            }
            VmInstructionSet::Pbs2 { lut } => {
                let dst1 = self.acquire_reg();
                let dst2 = self.acquire_reg();
                let output1 = op.get_returns_iter().nth(0).unwrap();
                let output2 = op.get_returns_iter().nth(1).unwrap();
                self.val_states
                    .get_mut(&output1)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst1, output1.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                self.val_states
                    .get_mut(&output2)
                    .unwrap()
                    .transition(|old| match old {
                        ValState::Preparing => ValState::InFlight(dst2, output2.get_users_iter().count()),
                        _ => unreachable!(),
                    });
                let ValState::InFlight(src, ..) = self
                    .val_states
                    .get(op.get_args_iter().nth(0).unwrap())
                    .unwrap()
                else {
                    unreachable!()
                };
                VmByteCode::PBS_ML2 {
                    id: op.get_id().as_raw(),
                    dst1: dst1.0,
                    dst2: dst2.0,
                    src: src.0,
                    lut: lut.sas(),
                }
            },
        };
        let op_id = op.get_id().as_raw();
        self.locks_table[op_id as usize] = op.get_predecessors_iter().count().sas();
        self.successors_table[op_id as usize].extend(op.get_users_iter().map(|u| u.get_id().0));
        // Encode the anti-dependencies gathered from recycled registers: each such predecessor
        // must decrement this op's lock, so it also gains this op as a successor. Deduplicated to
        // keep the lock count minimal (duplicates would still be consistent, but tighter is safer
        // for the u8 lock counter).
        let mut anti = std::mem::replace(&mut self.pending_anti_deps, SmallVec::new());
        anti.sort_unstable();
        let mut prev: Option<OpIdRaw> = None;
        let mut n_anti: u8 = 0;
        for a in anti.iter() {
            if Some(*a) != prev {
                self.successors_table[*a as usize].push(op_id);
                n_anti += 1;
                prev = Some(*a);
            }
        }
        self.locks_table[op_id as usize] += n_anti;
        match self.policy {
            SchedPolicy::AsSoonAsPossible => {
                self.schedules[thread].push_back(instr);
            }
            SchedPolicy::AsLateAsPossible => {
                self.schedules[thread].push_front(instr);
            }
        }
    }

    fn pop(&mut self) -> StatOpRef<'a, 'b> {
        self.ready
            .make_contiguous()
            .sort_by_key(|op| match self.policy {
                SchedPolicy::AsSoonAsPossible => op.get_annotation().height,
                SchedPolicy::AsLateAsPossible => op.get_annotation().depth,
            });
        self.ready.pop_back().unwrap()
    }

    fn get_initials(&self) -> impl Iterator<Item = StatOpRef<'a, 'b>> {
        self.op_states
            .iter()
            .filter(move |(_, state)| **state == OpState::Ready)
            .map(|(opid, _)| self.ir.get_op(opid).into())
    }

    fn land_op(&mut self, op: StatOpRef<'a, 'b>) {
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
                    OpState::Waiting(1) => {
                        self.ready.push_front(user.into());
                        OpState::Ready
                    }
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
                    ValState::InFlight(r, 0) => {
                        // The register becomes free, but any op that later reuses it must first
                        // wait for this value's readers to finish (WAR); if the value had no
                        // readers, it must wait for its producer's write instead (WAW).
                        let mut anti: SmallVec<OpIdRaw> =
                            val.get_users_iter().map(|u| u.get_id().0).collect();
                        if anti.is_empty() {
                            anti.push(val.get_origin().opref.get_id().0);
                        }
                        self.available_regs.push_back((r, anti));
                        ValState::Retired
                    }
                    ValState::InFlight(r, n) => ValState::InFlight(r, n - 1),
                    state => {
                        unreachable!("Found unexpected state {state:?} for val: {}", val.format())
                    }
                })
        }
    }

    fn should_fire_thread(&self, id: ThreadId) -> bool {
        self.threads[id].is_idle() && !self.ready.is_empty()
    }
}

impl<'a, 'b> zhc_sim::Simulatable for LightVm<'a, 'b> {
    type Event = Events;

    fn power_up(&mut self, dispatch: &mut impl zhc_sim::Dispatch<Event = Self::Event>) {
        self.ready.extend(self.get_initials().covec().into_iter());
        dispatch.dispatch_after(Cycle(1), Events::Start);
    }

    fn handle(
        &mut self,
        dispatcher: &mut impl zhc_sim::Dispatch<Event = Self::Event>,
        trigger: zhc_sim::Trigger<Self::Event>,
    ) {
        if let Events::LandThread(i) = trigger.event {
            let op = self.threads[i].transition_with(|old| match old {
                ThreadState::Running(op) => (ThreadState::Idle, op),
                _ => unreachable!(),
            });
            self.land_op(op);
        };

        for i in (0..self.n_threads).map(ThreadId) {
            if self.should_fire_thread(i) {
                let op = self.pop();
                self.sched(&op, i);
                self.op_states
                    .get_mut(&op)
                    .unwrap()
                    .transition(|old| match old {
                        OpState::Ready => OpState::Running,
                        _ => unreachable!(),
                    });
                self.threads[i].transition(|old| match old {
                    ThreadState::Idle => ThreadState::Running(op.clone()),
                    _ => unreachable!(),
                });
                use VmInstructionSet::*;
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
                    | MulCst { .. } => {
                            dispatcher
                                .dispatch_after(Cycle(ALU_COST.sas()), Events::LandThread(i));
                    }
                    | CstCt { .. }
                    | ImmLd { .. }
                    | DstSt { .. }
                    | SrcLd { .. } => {
                            dispatcher
                                .dispatch_after(Cycle(MEM_COST.sas()), Events::LandThread(i));
                    }
                    Ks => {
                        dispatcher
                            .dispatch_after(Cycle(KS_COST.sas()), Events::LandThread(i));
                    }
                    Pbs { .. } | Pbs2 { .. } => {
                        dispatcher
                            .dispatch_after(Cycle(PBS_COST.sas()), Events::LandThread(i));
                    }
                }
            }
        }
    }

    fn report<'t>(&self, at: Cycle, tracer: &mut Tracer, tracing_level: TracingLevel) {
        tracer.add_state(tracing_level, at, None, self.name(), self);
    }
}
