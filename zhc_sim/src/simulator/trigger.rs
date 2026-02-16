use super::*;

/// Represents a scheduled event with its execution time.
#[derive(Debug, Clone)]
pub struct Trigger<E: Event> {
    /// The cycle at which this event should be processed.
    pub at: Cycle,
    /// The event to be triggered.
    pub event: E,
}

impl<E: Event> PartialEq for Trigger<E> {
    fn eq(&self, other: &Self) -> bool {
        self.at.eq(&other.at)
    }
}

impl<E: Event> Eq for Trigger<E> {}

impl<E: Event> PartialOrd for Trigger<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.at.partial_cmp(&self.at)
    }
}

impl<E: Event> Ord for Trigger<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
