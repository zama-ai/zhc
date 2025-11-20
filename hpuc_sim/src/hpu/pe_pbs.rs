use std::{cell::Cell, collections::VecDeque, fmt::Display, ops::Index};

use hpuc_utils::Fifo;

use crate::Cycle;

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
    AlreadyWorking,
    CanLaunchIncompleteBatch(BatchSize),
    MustLaunchBatch(BatchSize),
    NoWaitings,
}

#[derive(Debug)]
pub struct PePbsMemory {
    // push_back -> |                       FIFO                        | -> pop_front
    //              | loading | waiting | working | parking | unloading |
    //              |  area   |  area   |  area   |  area   |  area     |
    //                        ^         ^         ^         ^
    //                  loading     working     parking     unloading
    //                 boundary    boundary     boundary    boundary
    //             <----------------------- id --------------------------
    memory: Fifo<DOp>,
    loading_boundary: usize,
    working_boundary: usize,
    parking_boundary: usize,
    unloading_boundary: usize,
    max_batch_size: BatchSize,
}

impl PePbsMemory {
    pub fn n_loading(&self) -> usize {
        self.len() - self.loading_boundary
    }

    pub fn has_loading(&self) -> bool {
        self.n_loading() > 0
    }

    pub fn n_waiting(&self) -> usize {
        self.loading_boundary - self.working_boundary
    }

    pub fn has_waiting(&self) -> bool {
        self.n_waiting() > 0
    }

    pub fn n_working(&self) -> usize {
        self.working_boundary - self.parking_boundary
    }

    pub fn has_working(&self) -> bool {
        self.n_working() > 0
    }

    pub fn n_parking(&self) -> usize {
        self.parking_boundary - self.unloading_boundary
    }

    pub fn has_parking(&self) -> bool {
        self.n_parking() > 0
    }

    pub fn n_unloading(&self) -> usize {
        self.unloading_boundary
    }

    pub fn has_unloading(&self) -> bool {
        self.n_unloading() > 0
    }

    pub fn new(capacity: usize, max_batch_size: BatchSize) -> Self {
        assert!(max_batch_size <= capacity);
        PePbsMemory {
            memory: Fifo::with_capacity(capacity),
            loading_boundary: 0,
            working_boundary: 0,
            parking_boundary: 0,
            unloading_boundary: 0,
            max_batch_size,
        }
    }

    pub fn launch_load(&mut self, dop: DOp) {
        assert!(!self.memory.is_full());
        assert!(!self.has_loading());
        self.memory.push_back(dop);
    }

    pub fn land_load(&mut self) {
        assert!(self.has_loading());
        self.loading_boundary += 1;
    }

    pub fn launch_work(&mut self, batch_size: BatchSize) {
        assert!(!self.has_working());
        assert!(self.n_waiting() >= batch_size);
        assert!(batch_size > 0);
        self.working_boundary += batch_size;
    }

    pub fn land_work(&mut self) {
        assert!(self.has_working());
        self.parking_boundary = self.working_boundary;
    }

    pub fn launch_unload(&mut self) {
        assert!(self.has_parking());
        assert!(!self.has_unloading());
        self.unloading_boundary += 1;
    }

    pub fn land_unload(&mut self) -> DOp {
        assert!(self.has_unloading());
        self.loading_boundary -= 1;
        self.working_boundary -= 1;
        self.parking_boundary -= 1;
        self.unloading_boundary -= 1;
        self.memory.pop_front()
    }

    pub fn loadings(&self) -> PePbsMemoryView {
        PePbsMemoryView {
            memory: self,
            range_bottom: self.loading_boundary,
            range_top: self.len(),
        }
    }

    pub fn waitings(&self) -> PePbsMemoryView {
        PePbsMemoryView {
            memory: self,
            range_bottom: self.working_boundary,
            range_top: self.loading_boundary,
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
            range_bottom: self.unloading_boundary,
            range_top: self.parking_boundary,
        }
    }

    pub fn unloadings(&self) -> PePbsMemoryView {
        PePbsMemoryView {
            memory: self,
            range_bottom: 0,
            range_top: self.unloading_boundary,
        }
    }

    pub fn what_now(&self) -> Hint {
        if self.is_working() {
            return Hint::AlreadyWorking;
        }
        if self.n_waiting() == 0 {
            return Hint::NoWaitings;
        }
        for (i, waiting) in self.waitings().iter().enumerate() {
            if i == self.max_batch_size - 1 {
                return Hint::MustLaunchBatch(self.max_batch_size);
            } else if waiting.raw.is_pbs_flush() {
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

    pub fn is_loading(&self) -> bool {
        self.n_loading() > 0
    }

    pub fn is_unloading(&self) -> bool {
        self.n_unloading() > 0
    }

    pub fn may_load(&self) -> bool {
        !self.memory.is_full() && !self.is_loading()
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

        let mut state = serializer.serialize_struct("PePbsMemory", 5)?;

        let loadings: Vec<&DOp> = self.loadings().iter().collect();
        state.serialize_field("loadings", &loadings)?;

        let waitings: Vec<&DOp> = self.waitings().iter().collect();
        state.serialize_field("waitings", &waitings)?;

        let workings: Vec<&DOp> = self.workings().iter().collect();
        state.serialize_field("workings", &workings)?;

        let parkings: Vec<&DOp> = self.parkings().iter().collect();
        state.serialize_field("parkings", &parkings)?;

        let unloadings: Vec<&DOp> = self.unloadings().iter().collect();
        state.serialize_field("unloadings", &unloadings)?;

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
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// The PBS Processing Element.
///
/// The model mainly relies on two elements:
/// + A fifo queue allowing to store upcoming dops.
/// + A memory allowing to manage the lifecycle of dops inputs and outputs.
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
        queue_capacity: usize,
        memory_capacity: usize,
        max_batch_size: BatchSize,
        policy: Policy,
        load_unload_latency: ConstantLatency,
        processing_latency: FlatLinLatency,
    ) -> Self {
        assert!(max_batch_size as usize <= memory_capacity);
        PePbs {
            queue: Fifo::with_capacity(queue_capacity),
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
                // -------------------------------------------------------------
                // The ISC just dispatched a DOP to the PE.
                // -------------------------------------------------------------

                assert!(!self.queue.is_full(), "Dispatched on a filled PE");

                dispatcher.dispatch_now(Events::IscUnlockIssue(dop.id));

                // The dop is pushed into the queue.
                self.queue.push_back(dop.clone());

                if self.queue.is_full() {
                    // PE got full. We notify the ISC.
                    dispatcher.dispatch_now(Events::PePbsUnavailable);
                }

                if self.memory.may_load() {
                    dispatcher.dispatch_now(Events::PePbsLaunchLoadMemory);
                }
            }
            Events::PePbsLaunchLoadMemory => {
                // -------------------------------------------------------------
                // DOp inputs start to be loaded in the memory.
                // -------------------------------------------------------------

                assert!(
                    self.queue.has_elements(),
                    "Launch Load Error: No elements in queue"
                );
                assert!(
                    self.memory.may_load(),
                    "Launch Load Error: Memory may not load"
                );

                if self.queue.is_full() {
                    // PE getting popped. We notify the ISC.
                    dispatcher.dispatch_now(Events::PePbsAvailable);
                }

                let dop = self.queue.pop_front();
                self.memory.launch_load(dop.clone());
                dispatcher.dispatch_later(
                    self.load_unload_latency.compute_latency(),
                    Events::PePbsLandLoadMemory(dop),
                );
            }
            Events::PePbsLandLoadMemory(dop) => {
                assert!(
                    self.memory.is_loading(),
                    "Load Error: landed load in non-loading memory"
                );

                // Remove the instruction from the queue. Notify the ISC.
                dispatcher.dispatch_now(Events::IscUnlockRead(dop.id));

                // Schedule a timeout.
                if let Policy::Timeout(offset) = self.policy {
                    let timeout = TimeoutId::grab();
                    dispatcher.dispatch_later(offset, Events::PePbsTimeout(timeout.clone()));
                    self.active_timeouts.0.push_back(timeout);
                }

                // Land the load in memory.
                self.memory.land_load();

                if self.memory.may_load() && !self.queue.is_empty() {
                    // Note that memory may not load directly after a load land (if no location is
                    // available in memory).
                    dispatcher.dispatch_now(Events::PePbsLaunchLoadMemory);
                }

                if let Hint::MustLaunchBatch(batch_size) = self.memory.what_now() {
                    // We just loaded the last ciphertext of a full batch.
                    // We can start processing the batch.
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
            Events::PePbsLaunchProcessing(batch_size) => {
                assert!(batch_size > 0, "Launch Error: Launched an empty batch");
                assert!(
                    self.memory.n_waiting() >= batch_size,
                    "Launch Error: batch_size mismatch"
                );
                if let Policy::Timeout(_) = self.policy {
                    // We can cancel the pending timeouts for the batch.
                    for _ in 0..batch_size {
                        self.active_timeouts.0.pop_front();
                    }
                }
                // We update the memory
                self.memory.launch_work(batch_size);
                // We schedule the land
                dispatcher.dispatch_later(
                    self.processing_latency.compute_latency(batch_size),
                    Events::PePbsLandProcessing(batch_size),
                );
            }
            Events::PePbsLandProcessing(batch_size) => {
                assert_eq!(
                    batch_size,
                    self.memory.n_working(),
                    "Land Error: Batch size mismatch"
                );

                // We land the work in memory.
                self.memory.land_work();

                // We launch the first unload immediately.
                dispatcher.dispatch_now(Events::PePbsLaunchUnloadMemory);

                // If necessary we immediately launch the next batch.
                if let Hint::MustLaunchBatch(batch_size) = self.memory.what_now() {
                    dispatcher.dispatch_now(Events::PePbsLaunchProcessing(batch_size));
                }
            }
            Events::PePbsLaunchUnloadMemory => {
                assert!(self.memory.has_parking());
                assert!(!self.memory.has_unloading());
                let unload = self.memory.parkings()[0].clone();
                let offset =
                    self.load_unload_latency.compute_latency() * get_dop_number_outputs(&unload);
                self.memory.launch_unload();
                dispatcher.dispatch_later(offset, Events::PePbsLandUnloadMemory(unload.id));
            }
            Events::PePbsLandUnloadMemory(_) => {
                assert!(self.memory.has_unloading());
                // We land the unload in memory.
                let dop = self.memory.land_unload();
                // We notify the ISC that memory has been unloaded.
                dispatcher.dispatch_now(Events::IscUnlockWrite(dop.id));
                if self.memory.has_parking() {
                    dispatcher.dispatch_now(Events::PePbsLaunchUnloadMemory);
                }
                if self.memory.may_load() && !self.queue.is_empty() {
                    dispatcher.dispatch_now(Events::PePbsLaunchLoadMemory);
                }
            }
            _ => {}
        }
    }

    fn name(&self) -> String {
        "PePbs".into()
    }
}

fn get_dop_number_outputs(dop: &DOp) -> usize {
    match dop.raw {
        RawDOp::PBS { .. } | RawDOp::PBS_F { .. } => 1usize,
        RawDOp::PBS_ML2 { .. } | RawDOp::PBS_ML2_F { .. } => 2,
        RawDOp::PBS_ML4 { .. } | RawDOp::PBS_ML4_F { .. } => 4,
        RawDOp::PBS_ML8 { .. } | RawDOp::PBS_ML8_F { .. } => 8,
        _ => unreachable!(),
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
        assert_eq!(memory.n_loading(), 0);
        assert_eq!(memory.n_waiting(), 0);
        assert_eq!(memory.n_working(), 0);
        assert_eq!(memory.n_parking(), 0);
        assert_eq!(memory.n_unloading(), 0);
        assert!(!memory.is_loading());
        assert!(!memory.is_working());
        assert!(!memory.is_unloading());
        assert!(memory.may_load());
    }

    #[test]
    fn test_loading_area_functionality() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop = create_mock_dop(1, false);

        // Initially no loading
        assert_eq!(memory.n_loading(), 0);
        assert!(!memory.has_loading());
        assert!(!memory.is_loading());
        assert!(memory.may_load());

        // After launch_load, should be in loading area
        memory.launch_load(dop.clone());
        assert_eq!(memory.n_loading(), 1);
        assert!(memory.has_loading());
        assert!(memory.is_loading());
        assert!(!memory.may_load()); // Can't load while already loading
        assert_eq!(memory.n_waiting(), 0); // Not yet waiting

        // Verify loadings view
        assert_eq!(memory.loadings().len(), 1);
        assert_eq!(memory.loadings()[0].id, DOpId(1));

        // After land_load, should move to waiting area
        memory.land_load();
        assert_eq!(memory.n_loading(), 0);
        assert!(!memory.has_loading());
        assert!(!memory.is_loading());
        assert!(memory.may_load()); // Can load again
        assert_eq!(memory.n_waiting(), 1); // Now waiting

        // Verify the element moved from loading to waiting
        assert_eq!(memory.loadings().len(), 0);
        assert_eq!(memory.waitings().len(), 1);
        assert_eq!(memory.waitings()[0].id, DOpId(1));
    }

    #[test]
    fn test_unloading_area_functionality() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop = create_mock_dop(1, false);

        // Setup: load -> work -> land work to get element in parking
        memory.launch_load(dop.clone());
        memory.land_load();
        memory.launch_work(1);
        memory.land_work();

        // Initially no unloading
        assert_eq!(memory.n_unloading(), 0);
        assert!(!memory.has_unloading());
        assert!(!memory.is_unloading());
        assert_eq!(memory.n_parking(), 1);

        // After launch_unload, should be in unloading area
        memory.launch_unload();
        assert_eq!(memory.n_unloading(), 1);
        assert!(memory.has_unloading());
        assert!(memory.is_unloading());
        assert_eq!(memory.n_parking(), 0); // Moved from parking to unloading

        // Verify unloadings view
        assert_eq!(memory.unloadings().len(), 1);
        assert_eq!(memory.unloadings()[0].id, DOpId(1));

        // After land_unload, should be completely removed
        let popped = memory.land_unload();
        assert_eq!(popped.id, DOpId(1));
        assert_eq!(memory.n_unloading(), 0);
        assert!(!memory.has_unloading());
        assert!(!memory.is_unloading());
        assert_eq!(memory.len(), 0);
    }

    #[test]
    fn test_no_waitings_hint() {
        let memory = PePbsMemory::new(10, 2);

        // Empty memory should return NoWaitings
        assert_eq!(memory.what_now(), Hint::NoWaitings);

        // Memory with only loading elements should also return NoWaitings
        let mut memory = PePbsMemory::new(10, 2);
        let dop = create_mock_dop(1, false);
        memory.launch_load(dop);
        assert_eq!(memory.what_now(), Hint::NoWaitings);
    }

    #[test]
    fn test_may_load_constraints() {
        let mut memory = PePbsMemory::new(2, 2); // Small capacity
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);

        // Initially can load
        assert!(memory.may_load());

        // After launching load, cannot load again (already loading)
        memory.launch_load(dop1);
        assert!(!memory.may_load());

        // After landing load, can load again
        memory.land_load();
        assert!(memory.may_load());

        // Fill memory to capacity
        memory.launch_load(dop2);
        memory.land_load();
        assert!(!memory.may_load()); // Full memory
    }

    #[test]
    fn test_all_memory_areas_simultaneously() {
        let mut memory = PePbsMemory::new(10, 3);

        // Create multiple DOPs
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);
        let dop3 = create_mock_dop(3, false);
        let dop4 = create_mock_dop(4, false);
        let _ = create_mock_dop(5, false);

        // Fill waiting area
        memory.launch_load(dop1);
        memory.land_load();
        memory.launch_load(dop2);
        memory.land_load();
        memory.launch_load(dop3);
        memory.land_load();

        // Start working on some
        memory.launch_work(2);
        memory.land_work();

        // Start unloading one
        memory.launch_unload();

        // Add one to loading
        memory.launch_load(dop4);

        // Now we should have elements in multiple areas:
        assert_eq!(memory.n_loading(), 1); // dop4
        assert_eq!(memory.n_waiting(), 1); // dop3
        assert_eq!(memory.n_working(), 0); // none (already landed)
        assert_eq!(memory.n_parking(), 1); // dop2 (dop1 is unloading)
        assert_eq!(memory.n_unloading(), 1); // dop1

        // Verify views show correct elements
        assert_eq!(memory.loadings()[0].id, DOpId(4));
        assert_eq!(memory.waitings()[0].id, DOpId(3));
        assert_eq!(memory.parkings()[0].id, DOpId(2));
        assert_eq!(memory.unloadings()[0].id, DOpId(1));
    }

    #[test]
    fn test_push_back() {
        let mut memory = PePbsMemory::new(10, 2);
        let dop = create_mock_dop(1, false);

        memory.launch_load(dop.clone());
        memory.land_load();

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

        memory.launch_load(dop1.clone());
        memory.land_load();
        memory.launch_load(dop2.clone());
        memory.land_load();

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

        memory.launch_load(dop1.clone());
        memory.land_load();
        memory.launch_load(dop2.clone());
        memory.land_load();
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

        memory.launch_load(dop.clone());
        memory.land_load();
        memory.launch_work(1);
        memory.land_work();

        // Before pop - should be in parkings
        assert_eq!(memory.parkings().len(), 1);
        assert_eq!(memory.parkings()[0].id, DOpId(1));

        memory.launch_unload();
        let popped = memory.land_unload();
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

        memory.launch_load(dop1.clone());
        memory.land_load();
        memory.launch_load(dop2.clone());
        memory.land_load();
        memory.launch_load(dop3.clone());
        memory.land_load();
        memory.launch_load(dop4.clone());
        memory.land_load();

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

        memory.launch_load(dop1);
        memory.land_load();
        memory.launch_load(dop2);
        memory.land_load();
        memory.launch_work(1);

        assert_eq!(memory.what_now(), Hint::AlreadyWorking);
    }

    #[test]
    fn test_may_launch_full_batch() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);
        let dop3 = create_mock_dop(3, false);

        memory.launch_load(dop1);
        memory.land_load();
        memory.launch_load(dop2);
        memory.land_load();
        memory.launch_load(dop3);
        memory.land_load();

        assert_eq!(memory.what_now(), Hint::CanLaunchIncompleteBatch(3));

        let mut memory = PePbsMemory::new(10, 2);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);
        let dop3 = create_mock_dop(3, false);

        memory.launch_load(dop1);
        memory.land_load();
        memory.launch_load(dop2);
        memory.land_load();
        memory.launch_load(dop3);
        memory.land_load();

        assert_eq!(memory.what_now(), Hint::MustLaunchBatch(2));
    }

    #[test]
    fn test_may_launch_with_flush() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, true); // flush operation

        memory.launch_load(dop1);
        memory.land_load();
        memory.launch_load(dop2);
        memory.land_load();

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

        memory.launch_load(dop1);
        memory.land_load();
        memory.launch_load(dop2);
        memory.land_load();
        memory.launch_load(dop3);
        memory.land_load();
        memory.launch_load(dop4);
        memory.land_load();
        memory.launch_load(dop5);
        memory.land_load();

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
        memory.launch_unload();
        let popped1 = memory.land_unload();
        memory.launch_unload();
        let popped2 = memory.land_unload();
        memory.launch_unload();
        let popped3 = memory.land_unload();

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

        memory.launch_load(dop);
        memory.land_load();
        memory.launch_work(2); // Should panic - only 1 waiting
    }

    #[test]
    #[should_panic]
    fn test_launch_load_while_loading() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);

        memory.launch_load(dop1);
        memory.launch_load(dop2); // Should panic - already loading
    }

    #[test]
    #[should_panic]
    fn test_launch_unload_while_unloading() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop1 = create_mock_dop(1, false);
        let dop2 = create_mock_dop(2, false);

        // Setup two elements in parking
        memory.launch_load(dop1);
        memory.land_load();
        memory.launch_load(dop2);
        memory.land_load();
        memory.launch_work(2);
        memory.land_work();

        memory.launch_unload();
        memory.launch_unload(); // Should panic - already unloading
    }

    #[test]
    #[should_panic]
    fn test_pop_front_insufficient_parking() {
        let mut memory = PePbsMemory::new(10, 5);
        let dop = create_mock_dop(1, false);

        memory.launch_load(dop);
        memory.land_load();
        memory.launch_unload(); // Should panic - nothing in parking area
    }
}
