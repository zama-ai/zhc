use hpuc_utils::tracing::Microseconds;
use serde::Serialize;
use std::ops::{Add, Mul, Sub};

/// Represents a discrete simulation cycle with zero-based counting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Hash)]
pub struct Cycle(pub usize);

impl Cycle {
    /// The zero cycle representing simulation start.
    pub const ZERO: Cycle = Cycle(0);
    /// The first cycle after simulation start.
    pub const ONE: Cycle = Cycle(1);

    /// Checks if this cycle is the zero cycle.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Creates an iterator over cycles from `from` to `to` (exclusive).
    pub fn range(from: Cycle, to: Cycle) -> impl Iterator<Item = Cycle> {
        (from.0..to.0).map(Cycle)
    }

    /// Creates an iterator from this cycle to `to` (exclusive).
    pub fn range_to(&self, to: Cycle) -> impl Iterator<Item = Cycle> {
        Cycle::range(*self, to)
    }

    /// Converts this cycle to a timestamp using the given `cycle_duration`.
    pub fn as_ts(&self, cycle_duration: Microseconds) -> Microseconds {
        self.0 as f64 * cycle_duration
    }
}

impl Add<usize> for Cycle {
    type Output = Cycle;

    fn add(self, rhs: usize) -> Self::Output {
        Cycle(self.0 + rhs)
    }
}

impl Mul<usize> for Cycle {
    type Output = Cycle;

    fn mul(self, rhs: usize) -> Self::Output {
        Cycle(self.0 * rhs)
    }
}

impl Mul<u16> for Cycle {
    type Output = Cycle;

    fn mul(self, rhs: u16) -> Self::Output {
        Cycle(self.0 * rhs as usize)
    }
}

impl Add<Cycle> for Cycle {
    type Output = Cycle;

    fn add(self, rhs: Cycle) -> Self::Output {
        Cycle(self.0 + rhs.0)
    }
}

impl Sub<Cycle> for Cycle {
    type Output = Cycle;

    fn sub(self, rhs: Cycle) -> Self::Output {
        Cycle(self.0 - rhs.0)
    }
}
