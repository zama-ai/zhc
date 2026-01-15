use crate::OpId;

use super::Selected;

/// A sequence of operations in scheduled order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule(Vec<OpId>);

impl Schedule {
    /// Creates an empty schedule.
    ///
    /// Returns a new schedule with no operations, ready to be populated
    /// by the scheduling algorithm.
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Issues selected operations by adding them to the schedule.
    pub(crate) fn issue_selected(&mut self, selected: impl Iterator<Item = Selected>) {
        self.0.extend(selected.map(|a| a.0));
    }

    /// Returns the number of operations in the schedule.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the schedule contains no operations.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a walker that yields operations in scheduled order.
    pub fn get_walker(&self) -> impl Iterator<Item=OpId> {
        self.0.iter().copied()
    }
}

impl FromIterator<OpId> for Schedule {
    fn from_iter<T: IntoIterator<Item = OpId>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
