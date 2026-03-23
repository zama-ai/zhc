use super::*;
use crate::{Cycle, Dispatch};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};
use zhc_langs::doplang::Affinity;
use zhc_utils::FastSet;

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
        assert!(
            !matches!(
                dop.raw,
                RawDOp::NOTIFY { .. } | RawDOp::WAIT { .. } | RawDOp::LD_B2B { .. }
            ),
            "Multi-HPU is not yet supported in simulation."
        );
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

/// Provides a read-only view of instruction slot state for external analysis.
#[derive(Debug, Default)]
pub struct SlotProperties {
    pub rd_lock: u32,
    pub wr_lock: u32,
    pub issue_lock: u32,
    pub state: State,
    pub pdg: bool,
    pub rd_pdg: bool,
    pub vld: bool,
}
impl From<&Slot> for SlotProperties {
    fn from(value: &Slot) -> Self {
        // TODO rework isc flag encoding
        // Proper generation if these bitflag could required to carry extra info in the Slot
        let (pdg, rd_pdg, vld) = match value.state {
            State::Pending => (true, true, true),
            State::Issued => (true, true, true),
            State::Loaded => (true, false, true),
            State::Finished => (false, false, true),
        };
        Self {
            rd_lock: value.read_lock.len() as u32,
            wr_lock: value.write_lock.len() as u32,
            issue_lock: value.issue_lock.len() as u32,
            state: value.state.clone(),
            pdg,
            rd_pdg,
            vld,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord, Copy, Default)]
#[repr(u8)]
pub enum State {
    /// First state. The DOp just landed in the isc pool, but was not yet dispatched to a PE. It is
    /// likely waiting for PE availability.
    Pending = 0,
    /// Second state. The DOp was issued to a PE. It is likely waiting for its sources to be loaded.
    Issued = 1,
    /// Third state. The DOp inputs were loaded to the PE. It is likely waiting for the PE to pick
    /// it up, or is being worked on and its destinations are getting written to.
    Loaded = 2,
    /// Forth state. The DOp was completed by the PE. The desinations can be read, and the op
    /// retired.
    #[default]
    Finished = 3,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Pending => write!(f, "PEN"),
            State::Loaded => write!(f, "LOA"),
            State::Issued => write!(f, "ISS"),
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
            State::Loaded => *self = State::Finished,
            State::Finished => unreachable!("Tried to transition while in final state"),
        }
    }
}

/// Commands issued by the instruction scheduler to manage operation state transitions.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum IscCommand {
    #[default]
    None = 0,
    RdUnlock,
    Retire,
    Refill,
    Issue,
}

impl Display for IscCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IscCommand::None => write!(f, "NON"),
            IscCommand::RdUnlock => write!(f, "RDU"),
            IscCommand::Retire => write!(f, "RET"),
            IscCommand::Refill => write!(f, "REF"),
            IscCommand::Issue => write!(f, "ISS"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
/// Tracks the availability status of a processing element.
pub struct PeTracker {
    available: bool,
}

/// Filters operations based on their processing element affinity requirements.
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
/// Manages instruction scheduling and dispatch across processing elements.
pub struct InstructionScheduler {
    query_period: Cycle,
    front_buffer: VecDeque<DOp>,
    tracker_mem: PeTracker,
    tracker_alu: PeTracker,
    tracker_pbs: PeTracker,
    write_unlock_buffer: VecDeque<DOpId>,
    read_unlock_buffer: VecDeque<DOpId>,
    pool: Pool,
    dop_processed: usize,
    dop_target: usize,
}

impl InstructionScheduler {
    /// Creates a new instruction scheduler with the given `query_period` and `pool_capacity`.
    ///
    /// The scheduler initializes with all processing elements available and empty buffers.
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
            dop_processed: 0,
            dop_target: 0,
        }
    }

    /// Checks if there are pending write unlock operations.
    pub fn has_write_unlocks(&self) -> bool {
        !self.write_unlock_buffer.is_empty()
    }

    /// Checks if there are pending read unlock operations.
    pub fn has_read_unlocks(&self) -> bool {
        !self.read_unlock_buffer.is_empty()
    }

    /// Checks if there are operations waiting to be scheduled.
    pub fn has_pending_dops(&self) -> bool {
        !self.front_buffer.is_empty()
    }

    /// Checks if any processing element is available to accept new operations.
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
    pub fn get_slot_properties(&self, dop_id: DOpId) -> Option<SlotProperties> {
        if let Some(index) = self.pool.slots.iter().position(|s| s.dop.id == dop_id) {
            Some((&self.pool.slots[index]).into())
        } else {
            None
        }
    }
}

impl Simulatable for InstructionScheduler {
    type Event = Events;

    fn power_up(&self, dispatcher: &mut impl Dispatch<Event = Events>) {
        dispatcher.dispatch_after(Cycle(1), Events::IscQuery);
    }

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        // NB: Each event that triggered side effect rearm IscQuery if none is already pending
        match trigger.event {
            Events::IscPushDOps(small_vec) => {
                self.dop_target += small_vec.len();
                self.front_buffer.extend(small_vec.into_iter());
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            Events::IscUnlockWrite(dopid) => {
                // The unlocks are buffered to be later processed during the isc query.
                self.write_unlock_buffer.push_back(dopid);
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            Events::IscUnlockRead(dopid) => {
                // The unlocks are buffered to be later processed during the isc query.
                self.read_unlock_buffer.push_back(dopid);
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            Events::IscUnlockIssue(dopid) => {
                self.pool.issue_unlock(dopid);
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            Events::PePbsAvailable => {
                self.tracker_pbs.available = true;
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            Events::PePbsUnavailable => {
                self.tracker_pbs.available = false;
            }
            Events::PeAluAvailable => {
                self.tracker_alu.available = true;
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            Events::PeAluUnavailable => {
                self.tracker_alu.available = false;
            }
            Events::PeMemAvailable => {
                self.tracker_mem.available = true;
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            Events::PeMemUnavailable => {
                self.tracker_mem.available = false;
            }
            Events::IscQuery => {
                if self.has_read_unlocks() {
                    let opid = self.read_unlock_buffer.pop_front().unwrap();
                    self.pool.read_unlock(opid);
                    dispatcher.dispatch_now(Events::NotifyIsc(opid, IscCommand::RdUnlock));
                    dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
                } else if self.pool.slots_available() && self.has_pending_dops() {
                    let dop = self.front_buffer.pop_front().unwrap();
                    // Used ?
                    dispatcher.dispatch_now(Events::IscRefillDOp(dop.clone()));
                    dispatcher.dispatch_now(Events::NotifyIsc(dop.id, IscCommand::Refill));
                    self.pool.refill(dop);
                    dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
                } else if self.has_write_unlocks() {
                    let opid = self.write_unlock_buffer.pop_front().unwrap();
                    self.pool.write_unlock(opid);
                    let dop = self.pool.retire(opid);
                    dispatcher.dispatch_now(Events::NotifyIsc(opid, IscCommand::Retire));
                    dispatcher.dispatch_now(Events::IscRetireDOp(dop));
                    dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
                } else if self.may_issue() {
                    if let Some(dop) = self.pool.get_issuable(self.get_filter()) {
                        dispatcher.dispatch_now(Events::NotifyIsc(dop.id, IscCommand::Issue));
                        dispatcher.dispatch_now(Events::IscIssueDOp(dop));
                        dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
                    }
                }
            }
            Events::IscRetireDOp(_) => {
                self.dop_processed += 1;
                if self.dop_processed == self.dop_target {
                    dispatcher.dispatch_next(Events::IscProcessOver);
                }
                dispatcher.dispatch_after_if_no_there(self.query_period, Events::IscQuery);
            }
            _ => {}
        };
    }
}
