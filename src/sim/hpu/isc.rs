use super::*;
use crate::{sim::Cycle, utils::FastSet};
use serde::Serialize;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Serialize)]
pub struct PredLock(FastSet<DOpId>);

impl PredLock {
    pub fn empty() -> Self {
        PredLock(FastSet::new())
    }

    pub fn unlock(&mut self, id: &DOpId) {
        self.0.remove(id);
    }

    pub fn is_locked(&self) -> bool {
        self.0.len() != 0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for PredLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for id in self.0.iter() {
            write!(f, "{id},")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Pool {
    slots: Vec<Slot>,
}

impl Pool {
    pub fn with_capacity(cap: usize) -> Self {
        Pool {
            slots: Vec::with_capacity(cap),
        }
    }

    pub fn is_full(&self) -> bool {
        self.slots.len() == self.slots.capacity()
    }

    pub fn slots_available(&self) -> bool {
        !self.is_full()
    }

    pub fn refill(&mut self, dop: DOp) {
        assert!(self.slots_available());
        self.slots.push(Slot {
            read_lock: self.init_read_lock(&dop.raw),
            write_lock: self.init_write_lock(&dop.raw),
            state: State::init(),
            dop,
        });
    }

    fn init_read_lock(&self, op: &RawDOp) -> PredLock {
        // An instruction _read lock_ is the number of instructions before, that need to read into our destination.
        match op.get_dst() {
            Some(dst) => {
                let raws = self
                    .slots
                    .iter()
                    .filter(|s| s.state < State::Working && s.dop.raw.has_source(dst))
                    .map(|s| s.dop.id)
                    .collect();
                PredLock(raws)
            }
            None => PredLock::empty(),
        }
    }

    fn init_write_lock(&self, op: &RawDOp) -> PredLock {
        // An instruction _write lock_ is the number of instructions before, that need to write into our sources / destination.
        match (op, op.get_dst(), op.get_src1(), op.get_src2()) {
            (RawDOp::SYNC, _, _, _) => {
                // Special case of the sync op -> it write-locks on every dop in the pool.
                PredLock(self.slots.iter().map(|s| s.dop.id).collect())
            }
            (_, Some(dst), Some(src1), Some(src2)) => {
                let raws = self
                    .slots
                    .iter()
                    .filter(|s| {
                        if let Some(d) = s.dop.raw.get_dst()
                            && [dst, src1, src2].contains(&d)
                        {
                            true
                        } else {
                            false
                        }
                    })
                    .map(|s| s.dop.id)
                    .collect();
                PredLock(raws)
            }
            (_, Some(dst), Some(src), None) => {
                let raws = self
                    .slots
                    .iter()
                    .filter(|s| {
                        if let Some(d) = s.dop.raw.get_dst()
                            && [dst, src].contains(&d)
                        {
                            true
                        } else {
                            false
                        }
                    })
                    .map(|s| s.dop.id)
                    .collect();
                PredLock(raws)
            }
            _ => PredLock::empty(),
        }
    }

    pub fn read_unlock(&mut self, opid: DOpId) {
        self.slots
            .iter_mut()
            .for_each(|s| s.read_lock.unlock(&opid));
        let index = self.slots.iter().position(|s| s.dop.id == opid).unwrap();
        assert_eq!(self.slots[index].state, State::Issued);
        self.slots[index].state.transition();
    }

    pub fn write_unlock(&mut self, opid: DOpId) {
        self.slots
            .iter_mut()
            .for_each(|s| s.write_lock.unlock(&opid));
        let index = self.slots.iter().position(|s| s.dop.id == opid).unwrap();
        assert_eq!(self.slots[index].state, State::Working);
        self.slots[index].state.transition();
    }

    pub fn maybe_issue(&mut self, filt: AffinityFilter) -> Option<DOp> {
        self.slots
            .iter_mut()
            .find(|s| s.state == State::Pending && !s.is_locked() && filt.is_avail(s.affinity()))
            .map(|s| {
                s.state.transition();
                s.dop.clone()
            })
    }

    pub fn retire(&mut self, opid: DOpId) -> DOp {
        let index = self.slots.iter().position(|s| s.dop.id == opid).unwrap();
        assert_eq!(self.slots[index].state, State::Finished);
        self.slots.remove(index).dop
    }
}

#[derive(Debug, Clone)]
pub struct Slot {
    dop: DOp,
    read_lock: PredLock,
    write_lock: PredLock,
    state: State,
}

impl Slot {
    pub fn is_locked(&self) -> bool {
        self.read_lock.is_locked() || self.write_lock.is_locked()
    }

    pub fn affinity(&self) -> Affinity {
        self.dop.raw.affinity()
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} ]   [ RLK: {} ]   [ WLK: {} ]    {}", self.state, self.read_lock.len(), self.write_lock.len(), self.dop)
    }
}

impl Serialize for Slot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum State {
    /// First state. The DOp just landed in the isc pool, but was not yet dispatched to a PE. It is likely waiting for PE availability.
    Pending = 0,
    /// Second state. The DOp was issued to a PE. It is likely waiting for its sources to be loaded, or for the PE to pick it up.
    Issued = 1,
    /// Third state. The DOp was picked up by the PE. It is being worked on, and its destinations are getting writen to.
    Working = 2,
    /// Fourth state. The DOp was completed by the PE. The desinations can be read, and the op retired.
    Finished = 3,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Pending =>  write!(f, "PEN"),
            State::Issued =>   write!(f, "ISS"),
            State::Working =>  write!(f, "WOR"),
            State::Finished => write!(f, "FIN"),
        }
    }
}

impl State {
    pub fn init() -> Self {
        Self::Pending
    }

    pub fn transition(&mut self) {
        match self {
            State::Pending => *self = State::Issued,
            State::Issued => *self = State::Working,
            State::Working => *self = State::Finished,
            State::Finished => unreachable!("Tried to transition while in final state"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PeTracker {
    available: bool,
}

pub struct AffinityFilter {
    mem: bool,
    alu: bool,
    pbs: bool,
    ctl: bool,
}

impl AffinityFilter {
    pub fn is_avail(&self, affinity: Affinity) -> bool {
        match affinity {
            Affinity::Alu => self.alu,
            Affinity::Mem => self.mem,
            Affinity::Pbs => self.pbs,
            Affinity::Ctl => self.ctl,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InstructionScheduler {
    query_period: Cycle,
    front_buffer: VecDeque<DOp>,
    tracker_mem: PeTracker,
    tracker_alu: PeTracker,
    tracker_pbs: PeTracker,
    write_unlock_buffer: VecDeque<DOpId>,
    read_unlock_buffer: VecDeque<DOpId>,
    pool: Pool,
}

impl InstructionScheduler {
    pub fn has_write_unlocks(&self) -> bool {
        !self.write_unlock_buffer.is_empty()
    }

    pub fn has_read_unlocks(&self) -> bool {
        !self.read_unlock_buffer.is_empty()
    }

    pub fn has_pending_dops(&self) -> bool {
        !self.front_buffer.is_empty()
    }

    pub fn may_issue(&self) -> bool {
        self.tracker_alu.available || self.tracker_mem.available || self.tracker_pbs.available
    }

    pub fn get_filter(&self) -> AffinityFilter {
        AffinityFilter {
            mem: self.tracker_mem.available,
            alu: self.tracker_alu.available,
            pbs: self.tracker_pbs.available,
            ctl: true,
        }
    }
}

impl Simulatable for InstructionScheduler {
    type Event = Events;

    fn power_up(&self, dispatcher: &mut Dispatcher<Self::Event>) {
        dispatcher.dispatch_later(Cycle(1), Events::IscQuery);
    }

    fn handle(&mut self, dispatcher: &mut Dispatcher<Self::Event>, trigger: Trigger<Self::Event>) {
        match trigger.event {
            Events::IscPushDOps(small_vec) => self.front_buffer.extend(small_vec.into_iter()),
            Events::IscUnlockWrite(dopid) => {
                // The unlocks are buffered to be later processed during the isc query.
                self.write_unlock_buffer.push_back(dopid);
            }
            Events::IscUnlockRead(dopid) => {
                // The unlocks are buffered to be later processed during the isc query.
                self.read_unlock_buffer.push_back(dopid);
            }
            Events::PePbsAvailable => {
                self.tracker_pbs.available = true;
            }
            Events::PePbsUnavailable => {
                self.tracker_pbs.available = false;
            }
            Events::PeAluAvailable => {
                self.tracker_alu.available = true;
            }
            Events::PeAluUnavailable => {
                self.tracker_alu.available = false;
            }
            Events::PeMemAvailable => {
                self.tracker_mem.available = true;
            }
            Events::PeMemUnavailable => {
                self.tracker_mem.available = false;
            }
            Events::IscQuery => {
                if !self.front_buffer.is_empty(){
                    dispatcher.dispatch_later(self.query_period, Events::IscQuery);
                }
                if self.has_read_unlocks() {
                    dispatcher.dispatch_now(Events::IscQueryUnlockRead);
                } else if self.has_write_unlocks() {
                    dispatcher.dispatch_now(Events::IscQueryUnlockWrite);
                } else if self.pool.slots_available() && self.has_pending_dops() {
                    dispatcher.dispatch_now(Events::IscQueryRefill);
                } else if self.may_issue() {
                    dispatcher.dispatch_now(Events::IscQueryIssue);
                }
            }
            Events::IscQueryUnlockRead => {
                let opid = self.read_unlock_buffer.pop_front().unwrap();
                self.pool.read_unlock(opid);
            }
            Events::IscQueryUnlockWrite => {
                let opid = self.write_unlock_buffer.pop_front().unwrap();
                self.pool.write_unlock(opid);
                let dop = self.pool.retire(opid);
                dispatcher.dispatch_now(Events::IscRetireDOp(dop));
            }
            Events::IscQueryRefill => {
                let dop = self.front_buffer.pop_front().unwrap();
                self.pool.refill(dop);
            }
            Events::IscQueryIssue => if let Some(dop) = self.pool.maybe_issue(self.get_filter()) {
                dispatcher.dispatch_now(Events::IscIssueDOp(dop));
            },
            _ => {}
        };
    }
}

impl InstructionScheduler {
    pub fn new(query_period: Cycle, pool_capacity: usize) -> Self {
        InstructionScheduler {
            front_buffer: VecDeque::new(),
            tracker_mem: PeTracker { available: true },
            tracker_pbs: PeTracker { available: true },
            tracker_alu: PeTracker { available: true },
            query_period,
            write_unlock_buffer: VecDeque::new(),
            read_unlock_buffer: VecDeque::new(),
            pool: Pool::with_capacity(pool_capacity),
        }
    }
}
