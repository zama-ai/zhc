use super::*;

/// Represents a scheduled event with its execution time.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Trigger<E: Event> {
    /// The cycle at which this event should be processed.
    pub at: Cycle,
    /// The event to be triggered.
    pub event: E,
}

impl<E: Event> Trigger<E> {
    pub fn map<EE: Event>(self, f: impl Fn(E) -> EE) -> Trigger<EE> {
        Trigger { at: self.at, event: f(self.event) }
    }
}
