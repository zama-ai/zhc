use crate::{OpId, traversal::OpWalker};

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

    pub(crate) fn issue_selected(&mut self, selected: impl Iterator<Item = Selected>) {
        self.0.extend(selected.map(|a| a.0));
    }

    /// Returns the number of operations in the schedule.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a walker in schedule order.
    pub fn get_walker(&self) -> impl OpWalker {
        self.0.iter().copied()
    }
}

impl FromIterator<OpId> for Schedule {
    fn from_iter<T: IntoIterator<Item = OpId>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
