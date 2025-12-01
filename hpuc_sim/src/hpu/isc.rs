use super::*;
use hpuc_langs::doplang::Affinity;
use crate::{Cycle, Dispatch};
use hpuc_utils::FastSet;
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
            issue_lock: self.init_issue_lock(&dop.raw),
            state: State::init(),
            dop,
        });
    }

    fn init_read_lock(&self, op: &RawDOp) -> PredLock {
        // An instruction _read lock_ is the number of instructions before, that need to read into
        // our destination.
        match op.get_dst() {
            Some(dst) => {
                let raws = self
                    .slots
                    .iter()
                    .filter(|s| s.state < State::Loaded && s.dop.raw.has_source(dst))
                    .map(|s| s.dop.id)
                    .collect();
                PredLock(raws)
            }
            None => PredLock::empty(),
        }
    }

    fn init_issue_lock(&self, op: &RawDOp) -> PredLock {
        // The instruction _issue lock_ only apply to PBSes. It is the number of pbses with opposite
        // flush before. It induces a total order on the pbses. This ensures that a flush
        // does not get issued before the rest of its batch.
        match op.affinity() {
            Affinity::Pbs => {
                let raws = self
                    .slots
                    .iter()
                    .filter(|s| {
                        s.affinity() == Affinity::Pbs
                            && s.state == State::Pending
                            && s.dop.raw.is_pbs_flush() != op.is_pbs_flush()
                    })
                    .map(|s| s.dop.id)
                    .collect();
                PredLock(raws)
            }
            _ => PredLock::empty(),
        }
    }

    fn init_write_lock(&self, op: &RawDOp) -> PredLock {
        // An instruction _write lock_ is the number of instructions before, that need to write into
        // our sources / destination.
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

    pub fn issue_unlock(&mut self, opid: DOpId) {
        self.slots
            .iter_mut()
            .for_each(|s| s.issue_lock.unlock(&opid));
        let index = self.slots.iter().position(|s| s.dop.id == opid).unwrap();
        assert_eq!(
            self.slots[index].state,
            State::Pending,
            "State error encounter with {}",
            self.slots[index]
        );
        self.slots[index].state.transition();
    }

    pub fn read_unlock(&mut self, opid: DOpId) {
        self.slots
            .iter_mut()
            .for_each(|s| s.read_lock.unlock(&opid));
        let index = self.slots.iter().position(|s| s.dop.id == opid).unwrap();
        assert_eq!(
            self.slots[index].state,
            State::Issued,
            "State error encounter with {}",
            self.slots[index]
        );
        self.slots[index].state.transition();
    }

    pub fn write_unlock(&mut self, opid: DOpId) {
        self.slots
            .iter_mut()
            .for_each(|s| s.write_lock.unlock(&opid));
        let index = self.slots.iter().position(|s| s.dop.id == opid).unwrap();
        assert_eq!(
            self.slots[index].state,
            State::Loaded,
            "State error encounter with {}",
            self.slots[index]
        );
        self.slots[index].state.transition();
        self.slots[index].state.transition();
    }

    pub fn get_issuable(&mut self, filt: AffinityFilter) -> Option<DOp> {
        self.slots
            .iter_mut()
            .find(|s| s.state == State::Pending && !s.is_locked() && filt.is_avail(s.affinity()))
            .map(|s| s.dop.clone())
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
    issue_lock: PredLock,
    state: State,
}

impl Slot {
    pub fn is_locked(&self) -> bool {
        self.read_lock.is_locked() || self.write_lock.is_locked() || self.issue_lock.is_locked()
    }

    pub fn affinity(&self) -> Affinity {
        self.dop.raw.affinity()
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ {} ]   [ RLK: {} ]   [ WLK: {} ]   [ ILK: {} ]   {}",
            self.state,
            self.read_lock.len(),
            self.write_lock.len(),
            self.issue_lock.len(),
            self.dop
        )
    }
}

impl Serialize for Slot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum State {
    /// First state. The DOp just landed in the isc pool, but was not yet dispatched to a PE. It is
    /// likely waiting for PE availability.
    Pending = 0,
    /// Second state. The DOp was issued to a PE. It is likely waiting for its sources to be loaded.
    Issued = 1,
    /// Third state. The DOp inputs were loaded to the PE. It is likely waiting for the PE to pick
    /// it up.
    Loaded = 2,
    /// Fourth state. The DOp was picked up by the PE. It is being worked on, and its destinations
    /// are getting writen to.
    Working = 3,
    /// Fifth state. The DOp was completed by the PE. The desinations can be read, and the op
    /// retired.
    Finished = 4,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Pending => write!(f, "PEN"),
            State::Loaded => write!(f, "LOA"),
            State::Issued => write!(f, "ISS"),
            State::Working => write!(f, "WOR"),
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
            State::Issued => *self = State::Loaded,
            State::Loaded => *self = State::Working,
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
    issue_unlock_buffer: VecDeque<DOpId>,
    write_unlock_buffer: VecDeque<DOpId>,
    read_unlock_buffer: VecDeque<DOpId>,
    pool: Pool,
    dop_processed: usize,
    dop_target: usize,
}

impl InstructionScheduler {
    pub fn new(query_period: Cycle, pool_capacity: usize) -> Self {
        InstructionScheduler {
            front_buffer: VecDeque::new(),
            tracker_mem: PeTracker { available: true },
            tracker_pbs: PeTracker { available: true },
            tracker_alu: PeTracker { available: true },
            query_period,
            issue_unlock_buffer: VecDeque::new(),
            write_unlock_buffer: VecDeque::new(),
            read_unlock_buffer: VecDeque::new(),
            pool: Pool::with_capacity(pool_capacity),
            dop_processed: 0,
            dop_target: 0,
        }
    }

    pub fn has_issue_unlocks(&self) -> bool {
        !self.issue_unlock_buffer.is_empty()
    }

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

    fn power_up(&self, dispatcher: &mut impl Dispatch<Event = Events>) {
        dispatcher.dispatch_later(Cycle(1), Events::IscQuery);
    }

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        let may_unlock = match trigger.event {
            Events::IscPushDOps(small_vec) => {
                self.dop_target += small_vec.len();
                self.front_buffer.extend(small_vec.into_iter());
                true
            }
            Events::IscUnlockWrite(dopid) => {
                // The unlocks are buffered to be later processed during the isc query.
                self.write_unlock_buffer.push_back(dopid);
                true
            }
            Events::IscUnlockRead(dopid) => {
                // The unlocks are buffered to be later processed during the isc query.
                self.read_unlock_buffer.push_back(dopid);
                true
            }
            Events::IscUnlockIssue(dopid) => {
                // The unlocks are buffered to be later processed during the isc query.
                self.issue_unlock_buffer.push_back(dopid);
                true
            }
            Events::PePbsAvailable => {
                self.tracker_pbs.available = true;
                true
            }
            Events::PePbsUnavailable => {
                self.tracker_pbs.available = false;
                false
            }
            Events::PeAluAvailable => {
                self.tracker_alu.available = true;
                true
            }
            Events::PeAluUnavailable => {
                self.tracker_alu.available = false;
                false
            }
            Events::PeMemAvailable => {
                self.tracker_mem.available = true;
                true
            }
            Events::PeMemUnavailable => {
                self.tracker_mem.available = false;
                false
            }
            Events::IscQuery => {
                if self.has_issue_unlocks() {
                    let opid = self.issue_unlock_buffer.pop_front().unwrap();
                    self.pool.issue_unlock(opid);
                    true
                } else if self.has_read_unlocks() {
                    let opid = self.read_unlock_buffer.pop_front().unwrap();
                    self.pool.read_unlock(opid);
                    true
                } else if self.has_write_unlocks() {
                    let opid = self.write_unlock_buffer.pop_front().unwrap();
                    self.pool.write_unlock(opid);
                    let dop = self.pool.retire(opid);
                    dispatcher.dispatch_now(Events::IscRetireDOp(dop));
                    true
                } else if self.pool.slots_available() && self.has_pending_dops() {
                    let dop = self.front_buffer.pop_front().unwrap();
                    dispatcher.dispatch_now(Events::IscRefillDOp(dop.clone()));
                    self.pool.refill(dop);
                    true
                } else if self.may_issue() {
                    if let Some(dop) = self.pool.get_issuable(self.get_filter()) {
                        dispatcher.dispatch_now(Events::IscIssueDOp(dop));
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Events::IscRetireDOp(_) => {
                self.dop_processed += 1;
                if self.dop_processed == self.dop_target {
                    dispatcher.dispatch_next(Events::IscProcessOver);
                }
                true
            }
            _ => false,
        };

        // Rearm IscQuery if needed
        // I.e. Current event triggered a side effect and No IscQuery event is pending
        if may_unlock && !dispatcher.contains_event(&Events::IscQuery) {
            dispatcher.dispatch_later(self.query_period, Events::IscQuery);
        }
    }
}
