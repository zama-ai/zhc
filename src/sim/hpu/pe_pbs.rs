use std::{cell::Cell, collections::VecDeque, fmt::Display, ops::Index};

use crate::{sim::Cycle, utils::Fifo};

use super::*;

#[derive(Debug, Clone, Serialize)]
pub enum Policy {
    /// Flush occurs on full batches.
    Wait,
    /// Flush occurs on full batches, or on timeout, n cycles after the last element was pushed.
    Timeout(Cycle),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TimeoutId(u8);

thread_local! {
    static TIMEOUT_ID_GRAB_STATE: Cell<u8> = Cell::new(0);
}

impl TimeoutId {
    pub fn grab() -> Self {
        TIMEOUT_ID_GRAB_STATE.with(|c| {
            let id = c.get();
            c.set(id.wrapping_add(1));
            Self(id)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hint {
    MustWait,
    CanLaunchIncompleteBatch(BatchSize),
    MustLaunchBatch(BatchSize),
}
#[derive(Debug)]
pub struct PePbsMemory {
    // push_back -> |             FIFO            | -> pop_front
    //              | waiting | working | parking |
    //              |  area   |  area   |  area   |
    //                        ^         ^
    //                    working     parking
    //                   boundary     boundary
    //             <------------- id --------------
    memory: Fifo<DOp>,
    working_boundary: usize,
    parking_boundary: usize,
    max_batch_size: BatchSize,
}

impl PePbsMemory {
    pub fn n_waiting(&self) -> usize {
        self.len() - self.working_boundary
    }

    pub fn n_working(&self) -> usize {
        self.working_boundary - self.parking_boundary
    }

    pub fn n_parking(&self) -> usize {
        self.parking_boundary
    }

    pub fn new(capacity: usize, max_batch_size: BatchSize) -> Self {
        assert!(max_batch_size <= capacity);
        PePbsMemory {
            memory: Fifo::with_capacity(capacity),
            working_boundary: 0,
            parking_boundary: 0,
            max_batch_size,
        }
    }

    pub fn push_back(&mut self, dop: DOp) {
        self.memory.push_back(dop);
        // self.working_boundary += 1;
        // self.parking_boundary += 1;
    }

    pub fn launch_work(&mut self, batch_size: BatchSize) {
        assert!(self.n_waiting() >= batch_size);
        self.working_boundary += batch_size;
    }

    pub fn land_work(&mut self) {
        self.parking_boundary = self.working_boundary;
    }

    pub fn pop_front(&mut self) -> DOp {
        assert!(self.len() > 0);
        self.working_boundary -= 1;
        self.parking_boundary -= 1;
        self.memory.pop_front()
    }

    pub fn waitings(&self) -> PePbsMemoryView {
        PePbsMemoryView {
            memory: self,
            range_bottom: self.working_boundary,
            range_top: self.len(),
        }
    }

    pub fn workings(&self) -> PePbsMemoryView {
        PePbsMemoryView {
            memory: self,
            range_bottom: self.parking_boundary,
            range_top: self.working_boundary,
        }
    }

    pub fn parkings(&self) -> PePbsMemoryView {
        PePbsMemoryView {
            memory: self,
            range_bottom: 0,
            range_top: self.parking_boundary,
        }
    }

    pub fn what_now(&self) -> Hint {
        if self.is_working() {
            return Hint::MustWait;
        }
        for i in 0..self.n_waiting() {
            if i == self.max_batch_size - 1 {
                return Hint::MustLaunchBatch(self.max_batch_size);
            } else if self.memory[i as usize].raw.is_pbs_flush() {
                return Hint::MustLaunchBatch(i + 1);
            }
        }
        Hint::CanLaunchIncompleteBatch(self.n_waiting())
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn is_working(&self) -> bool {
        self.n_working() > 0
    }

    pub fn may_push(&self) -> bool {
        !self.memory.is_full()
    }
}

pub struct PePbsMemoryView<'m> {
    memory: &'m PePbsMemory,
    range_bottom: usize,
    range_top: usize,
}

impl<'m> PePbsMemoryView<'m> {
    pub fn iter(&self) -> impl Iterator<Item = &'m DOp> {
        (self.range_bottom..self.range_top).map(|i| &self.memory.memory[i as usize])
    }

    pub fn len(&self) -> usize {
        self.range_top - self.range_bottom
    }
}

impl<'m> Index<usize> for PePbsMemoryView<'m> {
    type Output = DOp;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "Tried to index with an invalid index");
        &self.memory.memory[self.range_bottom + index]
    }
}

impl Serialize for PePbsMemory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("PePbsMemory", 3)?;

        let waitings: Vec<&DOp> = self.waitings().iter().collect();
        state.serialize_field("waitings", &waitings)?;

        let workings: Vec<&DOp> = self.workings().iter().collect();
        state.serialize_field("workings", &workings)?;

        let parkings: Vec<&DOp> = self.parkings().iter().collect();
        state.serialize_field("parkings", &parkings)?;

        state.end()
    }
}

#[derive(Debug)]
pub struct ActiveTimeouts(VecDeque<TimeoutId>);

impl Display for ActiveTimeouts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.0.iter() {
            write!(f, "{}, ", i.0)?;
        }
        Ok(())
    }
}

impl Serialize for ActiveTimeouts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(&self.to_string())
    }
}


#[derive(Debug, Serialize)]
pub struct PePbs {
    queue: Fifo<DOp>,
    memory: PePbsMemory,
    policy: Policy,
    load_unload_latency: ConstantLatency,
    processing_latency: FlatLinLatency,
    active_timeouts: ActiveTimeouts,
    max_batch_size: BatchSize,
}

impl PePbs {
    pub fn new(
        fifo_capacity: usize,
        memory_capacity: usize,
        max_batch_size: BatchSize,
        policy: Policy,
        load_unload_latency: ConstantLatency,
        processing_latency: FlatLinLatency,
    ) -> Self {
        assert!(max_batch_size as usize <= memory_capacity);
        PePbs {
            queue: Fifo::with_capacity(fifo_capacity),
            memory: PePbsMemory::new(memory_capacity, max_batch_size),
            load_unload_latency,
            processing_latency,
            policy,
            active_timeouts: ActiveTimeouts(VecDeque::new()),
            max_batch_size,
        }
    }
}

impl Simulatable for PePbs {
    type Event = Events;

    fn handle(&mut self, dispatcher: &mut Dispatcher<Self::Event>, trigger: Trigger<Self::Event>) {
        match trigger.event {
            Events::IscIssueDOp(dop) if dop.raw.affinity() == Affinity::Pbs => {
                assert!(
                    !self.queue.is_full(),
                    "Issue Error: Dispatched on a filled PE"
                );
                self.queue.push_back(dop.clone());
                if self.memory.may_push() {
                    dispatcher.dispatch_later(
                        self.load_unload_latency.compute_latency(),
                        Events::PePbsLoadMemory,
                    );
                }
                if self.queue.is_full() {
                    dispatcher.dispatch_now(Events::PePbsUnavailable);
                }
            }
            Events::PePbsLoadMemory => {
                assert!(
                    self.memory.may_push(),
                    "LoadMemory Error: Load in a filled memory"
                );
                assert!(
                    !self.queue.is_empty(),
                    "LoadMemory Error: Load on an empty queue"
                );

                if self.queue.is_full() {
                    dispatcher.dispatch_now(Events::PePbsAvailable);
                }
                let dop = self.queue.pop_front();
                dispatcher.dispatch_now(Events::IscUnlockRead(dop.id));
                if let Policy::Timeout(offset) = self.policy {
                    let timeout = TimeoutId::grab();
                    dispatcher.dispatch_later(offset, Events::PePbsTimeout(timeout.clone()));
                    self.active_timeouts.0.push_back(timeout);
                }
                self.memory.push_back(dop.clone());

                if !self.memory.is_working() {
                    // We are not currently processing.
                    if dop.raw.is_pbs_flush() {
                        // We received a flush.
                        assert!(self.memory.waitings().len() <= self.max_batch_size);
                        dispatcher.dispatch_now(Events::PePbsLaunchProcessing(
                            self.memory.len() as BatchSize
                        ));
                    } else if self.memory.waitings().len() == self.max_batch_size {
                        // We just loaded the last ciphertext of a full batch.
                        dispatcher.dispatch_now(Events::PePbsLaunchProcessing(self.max_batch_size));
                    }
                }
            }
            Events::PePbsLaunchProcessing(batch_size) => {
                if let Policy::Timeout(_) = self.policy {
                    for _ in 0..batch_size {
                        self.active_timeouts.0.pop_front();
                    }
                }
                self.memory.launch_work(batch_size);
                dispatcher.dispatch_later(
                    self.processing_latency.compute_latency(batch_size),
                    Events::PePbsLandProcessing(batch_size),
                );
            }
            Events::PePbsLandProcessing(batch_size) => {
                let mut offset = Cycle(0);
                for i in 0..batch_size {
                    let n_outputs = match self.memory.workings()[i].raw {
                        RawDOp::PBS { .. } | RawDOp::PBS_F { .. } => 1usize,
                        RawDOp::PBS_ML2 { .. } | RawDOp::PBS_ML2_F { .. } => 2,
                        RawDOp::PBS_ML4 { .. } | RawDOp::PBS_ML4_F { .. } => 4,
                        RawDOp::PBS_ML8 { .. } | RawDOp::PBS_ML8_F { .. } => 8,
                        _ => unreachable!(),
                    };
                    offset = offset + self.load_unload_latency.compute_latency() * n_outputs;
                    dispatcher.dispatch_later(offset, Events::PePbsUnloadMemory);
                }
                self.memory.land_work();
                if let Hint::MustLaunchBatch(batch_size) = self.memory.what_now() {
                    dispatcher.dispatch_now(Events::PePbsLaunchProcessing(batch_size));
                }
            }
            Events::PePbsTimeout(timeout) => {
                if self.active_timeouts.0.contains(&timeout) {
                    // TO REVIEW: This assumes that the timeout occurs while the unit is not busy.
                    // It is unclear to me whether this is correct or not.
                    let Hint::CanLaunchIncompleteBatch(batch_size) = self.memory.what_now() else {
                        panic!("Unexpected state: {:?}", self.memory.what_now());
                    };
                    dispatcher.dispatch_now(Events::PePbsLaunchProcessing(batch_size));
                }
            }
            Events::PePbsUnloadMemory => {
                let dop = self.memory.pop_front();
                dispatcher.dispatch_now(Events::IscUnlockWrite(dop.id));
                if self.memory.may_push() && !self.queue.is_empty() {
                    dispatcher.dispatch_now(Events::PePbsLoadMemory);
                }
            }
            _ => {}
        }
    }

    fn name(&self) -> String {
        "PePbs".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_dop(id: u16, is_flush: bool) -> DOp {
        if is_flush {
            DOp {
                raw: RawDOp::PBS_F {
                    dst: Argument::Immediate { val: 0 },
                    src: Argument::Immediate { val: 0 },
                },
                id: DOpId(id),
            }
        } else {
            DOp {
                raw: RawDOp::SYNC,
                id: DOpId(id),
            }
        }
    }

    #[test]
    fn test_new_memory() {
        let memory = PePbsMemory::new(10, 2);
        assert_eq!(memory.len(), 0);
        assert_eq!(memory.n_waiting(), 0);
        assert_eq!(memory.n_working(), 0);
        assert_eq!(memory.n_parking(), 0);
        assert!(!memory.is_working());
        assert!(memory.may_push());
    }

    #[test]
    fn test_push_back() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop = create_mock_dop(1, false);

        memory.push_back(dop.clone());

        assert_eq!(memory.len(), 1);
        assert_eq!(memory.n_waiting(), 1);
        assert_eq!(memory.n_working(), 0);
        assert_eq!(memory.n_parking(), 0);

        // Verify the actual element is in waitings
        assert_eq!(memory.waitings().len(), 1);
        assert_eq!(memory.waitings()[0].id, DOpId(1));
    }

    #[test]
    fn test_launch_work() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);

        memory.push_back(dop1.clone());
        memory.push_back(dop2.clone());

        // Before launch - should be in waitings
        assert_eq!(memory.waitings().len(), 2);
        assert_eq!(memory.waitings()[0].id, DOpId(1));
        assert_eq!(memory.waitings()[1].id, DOpId(2));

        memory.launch_work(2);

        assert_eq!(memory.n_waiting(), 0);
        assert_eq!(memory.n_working(), 2);
        assert_eq!(memory.n_parking(), 0);
        assert!(memory.is_working());

        // After launch - should be in workings
        assert_eq!(memory.workings().len(), 2);
        assert_eq!(memory.workings()[0].id, DOpId(1));
        assert_eq!(memory.workings()[1].id, DOpId(2));
    }

    #[test]
    fn test_land_work() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);

        memory.push_back(dop1.clone());
        memory.push_back(dop2.clone());
        memory.launch_work(2);
        memory.land_work();

        assert_eq!(memory.n_waiting(), 0);
        assert_eq!(memory.n_working(), 0);
        assert_eq!(memory.n_parking(), 2);
        assert!(!memory.is_working());

        // After landing - should be in parkings
        assert_eq!(memory.parkings().len(), 2);
        assert_eq!(memory.parkings()[0].id, DOpId(1));
        assert_eq!(memory.parkings()[1].id, DOpId(2));
    }

    #[test]
    fn test_pop_front() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop = create_mock_dop(1, false);

        memory.push_back(dop.clone());
        memory.launch_work(1);
        memory.land_work();

        // Before pop - should be in parkings
        assert_eq!(memory.parkings().len(), 1);
        assert_eq!(memory.parkings()[0].id, DOpId(1));

        let popped = memory.pop_front();
        assert_eq!(popped.id, DOpId(1));
        assert_eq!(memory.len(), 0);
    }

    #[test]
    fn test_iterators() {
        let mut memory = PePbsMemory::new(10, 4);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);
        let dop3 = create_mock_dop(3, false);
        let dop4 = create_mock_dop(4, false);

        memory.push_back(dop1.clone());
        memory.push_back(dop2.clone());
        memory.push_back(dop3.clone());
        memory.push_back(dop4.clone());

        memory.launch_work(2);
        memory.land_work();

        assert_eq!(memory.waitings().len(), 2);
        assert_eq!(memory.workings().len(), 0);
        assert_eq!(memory.parkings().len(), 2);

        // Verify the correct elements are in each section
        assert_eq!(memory.waitings()[0].id, DOpId(3)); // Elements 3 and 4 should be waiting
        assert_eq!(memory.waitings()[1].id, DOpId(4));
        assert_eq!(memory.parkings()[0].id, DOpId(1)); // Elements 1 and 2 should be parking
        assert_eq!(memory.parkings()[1].id, DOpId(2));
    }

    #[test]
    fn test_may_launch_when_working() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);

        memory.push_back(dop1);
        memory.push_back(dop2);
        memory.launch_work(1);

        assert_eq!(memory.what_now(), Hint::MustWait);
    }

    #[test]
    fn test_may_launch_full_batch() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);
        let dop3 = create_mock_dop(3, false);

        memory.push_back(dop1);
        memory.push_back(dop2);
        memory.push_back(dop3);

        assert_eq!(memory.what_now(), Hint::CanLaunchIncompleteBatch(3));

        let mut memory = PePbsMemory::new(10, 2);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);
        let dop3 = create_mock_dop(3, false);

        memory.push_back(dop1);
        memory.push_back(dop2);
        memory.push_back(dop3);

        assert_eq!(memory.what_now(), Hint::MustLaunchBatch(2));
    }

    #[test]
    fn test_may_launch_with_flush() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, true); // flush operation

        memory.push_back(dop1);
        memory.push_back(dop2);

        assert_eq!(memory.what_now(), Hint::MustLaunchBatch(2));
    }

    #[test]
    fn test_complex_workflow() {
        let mut memory = PePbsMemory::new(10, 3);

        // Add operations with different IDs
        let dop1 = create_mock_dop(10, false);
        let dop2 = create_mock_dop(20, false);
        let dop3 = create_mock_dop(30, false);
        let dop4 = create_mock_dop(40, false);
        let dop5 = create_mock_dop(50, false);

        memory.push_back(dop1);
        memory.push_back(dop2);
        memory.push_back(dop3);
        memory.push_back(dop4);
        memory.push_back(dop5);

        // All should be waiting initially
        assert_eq!(memory.waitings().len(), 5);
        assert_eq!(memory.waitings()[0].id, DOpId(10));
        assert_eq!(memory.waitings()[4].id, DOpId(50));

        // Launch first batch
        memory.launch_work(3);

        // Check working section has correct elements
        assert_eq!(memory.workings().len(), 3);
        assert_eq!(memory.workings()[0].id, DOpId(10));
        assert_eq!(memory.workings()[1].id, DOpId(20));
        assert_eq!(memory.workings()[2].id, DOpId(30));

        // Check remaining waiting elements
        assert_eq!(memory.waitings().len(), 2);
        assert_eq!(memory.waitings()[0].id, DOpId(40));
        assert_eq!(memory.waitings()[1].id, DOpId(50));

        // Land the work
        memory.land_work();

        // Check parkings section has correct elements
        assert_eq!(memory.parkings().len(), 3);
        assert_eq!(memory.parkings()[0].id, DOpId(10));
        assert_eq!(memory.parkings()[1].id, DOpId(20));
        assert_eq!(memory.parkings()[2].id, DOpId(30));

        // Pop elements and verify order
        let popped1 = memory.pop_front();
        let popped2 = memory.pop_front();
        let popped3 = memory.pop_front();

        assert_eq!(popped1.id, DOpId(10));
        assert_eq!(popped2.id, DOpId(20));
        assert_eq!(popped3.id, DOpId(30));

        // Verify remaining elements are still waiting
        assert_eq!(memory.waitings().len(), 2);
        assert_eq!(memory.waitings()[0].id, DOpId(40));
        assert_eq!(memory.waitings()[1].id, DOpId(50));
    }

    #[test]
    #[should_panic]
    fn test_launch_work_insufficient_waiting() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop = create_mock_dop(1, false);

        memory.push_back(dop);
        memory.launch_work(2); // Should panic - only 1 waiting
    }

    #[test]
    #[should_panic]
    fn test_pop_front_insufficient_parking() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop = create_mock_dop(1, false);

        memory.push_back(dop);
        memory.pop_front(); // Should panic - nothing in parking area
    }

}
