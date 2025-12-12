use hpuc_langs::doplang::Affinity;
use hpuc_utils::Fifo;

use crate::Dispatch;

use super::*;

/// Memory Processing Element.
#[derive(Debug, Serialize)]
pub struct PeMem {
    queue: Fifo<DOp>,
    current: Option<DOp>,
    read_latency: ConstantLatency,
    write_latency: ConstantLatency,
}

impl PeMem {
    /// Creates a new memory processing element with the specified parameters.
    ///
    /// The memory PE is initialized with the given `fifo_capacity`, `read_latency`, and `write_latency`.
    pub fn new(
        fifo_capacity: usize,
        read_latency: ConstantLatency,
        write_latency: ConstantLatency,
    ) -> Self {
        PeMem {
            queue: Fifo::with_capacity(fifo_capacity),
            current: None,
            read_latency,
            write_latency,
        }
    }

    pub fn has_dops_waiting(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn has_room_for_dops(&self) -> bool {
        !self.queue.is_full()
    }

    pub fn busy(&self) -> bool {
        self.current.is_some()
    }

    pub fn available(&self) -> bool {
        self.current.is_none()
    }

    pub fn push_dop(&mut self, dop: DOp) {
        self.queue.push_back(dop);
    }
}

impl Simulatable for PeMem {
    type Event = Events;

    fn handle(&mut self, dispatcher: &mut impl Dispatch<Event = Self::Event>, trigger: Trigger<Self::Event>) {
        match trigger.event {
            Events::IscIssueDOp(dop) if dop.raw.affinity() == Affinity::Mem => {
                assert!(
                    self.has_room_for_dops(),
                    "Dispatch Error: Dispatched on a filled PE"
                );
                dispatcher.dispatch_now(Events::IscUnlockIssue(dop.id));
                self.push_dop(dop.clone());
                if !self.busy() {
                    dispatcher.dispatch_now(Events::PeMemLaunchProcessing);
                }
                if self.queue.is_full() {
                    dispatcher.dispatch_now(Events::PeMemUnavailable);
                }
            }
            Events::PeMemLaunchProcessing => {
                if self.queue.is_full() {
                    dispatcher.dispatch_now(Events::PeMemAvailable);
                }
                let mdop = self.queue.pop_front();
                assert!(
                    self.current.replace(mdop.clone()).is_none(),
                    "Start Error: Started op while still busy"
                );
                dispatcher.dispatch_after(
                    self.read_latency.compute_latency(),
                    Events::IscUnlockRead(mdop.id),
                );
                dispatcher.dispatch_after(
                    self.write_latency.compute_latency(),
                    Events::IscUnlockWrite(mdop.id),
                );
                dispatcher.dispatch_after(
                    self.write_latency.compute_latency(),
                    Events::PeMemLandProcessing,
                );
            }
            Events::PeMemLandProcessing => {
                self.current.take().expect("Land Error: Empty PE");
                if self.has_dops_waiting() {
                    dispatcher.dispatch_now(Events::PeMemLaunchProcessing);
                }
            }
            _ => {}
        }
    }

    fn name(&self) -> String {
        "PeMem".into()
    }
}
