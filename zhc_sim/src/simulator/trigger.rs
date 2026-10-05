use std::cmp::Ordering;

use zhc_utils::units::Cycle;

use super::*;

/// Represents a scheduled event with its execution time.
#[derive(Debug, Clone)]
pub struct Trigger<E: Event> {
    /// The cycle at which this event should be processed.
    pub at: Cycle,
    pub issue: u64,
    /// The event to be triggered.
    pub event: E,
}

impl<E: Event> Trigger<E> {
    /// Converts the wrapped event through `f`, preserving the scheduled cycle.
    pub fn map<EE: Event>(self, f: impl Fn(E) -> EE) -> Trigger<EE> {
        Trigger {
            at: self.at,
            issue: self.issue,
            event: f(self.event),
        }
    }
}

impl<E: Event> PartialEq for Trigger<E> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<E: Event> Eq for Trigger<E> {}

impl<E: Event> PartialOrd for Trigger<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E: Event> Ord for Trigger<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.at
            .cmp(&other.at)
            .then_with(|| self.issue.cmp(&other.issue))
    }
}
