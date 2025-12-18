use hpuc_langs::doplang::Affinity;
use hpuc_utils::Fifo;

use crate::Dispatch;

use super::*;

/// Alu Processing Element
#[derive(Debug, Serialize)]
pub struct PeAlu {
    queue: Fifo<DOp>,
    current: Option<DOp>,
    read_latency: ConstantLatency,
    write_latency: ConstantLatency,
}

impl PeAlu {
    /// Creates a new ALU processing element with the specified parameters.
    ///
    /// The ALU is initialized with the given `queue_capacity`, `read_latency`, and `write_latency`.
    pub fn new(
        fifo_capacity: usize,
        read_latency: ConstantLatency,
        write_latency: ConstantLatency,
    ) -> Self {
        PeAlu {
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

impl Simulatable for PeAlu {
    type Event = Events;

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match trigger.event {
            Events::IscIssueDOp(dop) if dop.raw.affinity() == Affinity::Alu => {
                assert!(
                    self.has_room_for_dops(),
                    "Dispatch Error: Dispatched on a filled PE"
                );
                dispatcher.dispatch_now(Events::IscUnlockIssue(dop.id));
                self.push_dop(dop.clone());
                if !self.busy() {
                    dispatcher.dispatch_now(Events::PeAluLaunchProcessing);
                }
                if self.queue.is_full() {
                    dispatcher.dispatch_now(Events::PeAluUnavailable);
                }
            }
            Events::PeAluLaunchProcessing => {
                if self.queue.is_full() {
                    dispatcher.dispatch_now(Events::PeAluAvailable);
                }
                let mdop = self.queue.pop_front();

                assert!(
                    self.current.replace(mdop.clone()).is_none(),
                    "PeAlu Start Error: Started op while still busy"
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
                    Events::PeAluLandProcessing,
                );
            }
            Events::PeAluLandProcessing => {
                self.current.take().expect("Land Error: Empty PE");
                if self.has_dops_waiting() {
                    dispatcher.dispatch_now(Events::PeAluLaunchProcessing);
                }
            }
            _ => {}
        }
    }

    fn name(&self) -> String {
        "PeAlu".into()
    }
}
