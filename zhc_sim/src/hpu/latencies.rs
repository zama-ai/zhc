use crate::Cycle;
use serde::Serialize;

use super::BatchSize;

/// Models a constant processing latency independent of batch size.
#[derive(Debug, Clone, Serialize)]
pub struct ConstantLatency(usize);

impl ConstantLatency {
    /// Creates a new constant latency model with the given cycle count `c`.
    pub fn new(c: usize) -> Self {
        Self(c)
    }

    /// Returns the constant latency in cycles.
    pub fn compute_latency(&self) -> Cycle {
        Cycle(self.0)
    }
}

/// Models a flat-then-linear latency that scales with batch size after a minimum threshold.
///
/// The latency follows the formula: `a * max(batch_size, m) + b` where `a` is the
/// linear coefficient, `b` is the constant offset, and `m` is the minimum batch size.
#[derive(Debug, Clone, Serialize)]
pub struct FlatLinLatency {
    a: usize,
    b: usize,
    m: usize,
}

impl FlatLinLatency {
    /// Creates a new flat-linear latency model with coefficients `a`, `b`, and minimum size `m`.
    pub fn new(a: usize, b: usize, m: usize) -> Self {
        Self { a, b, m }
    }
    /// Calculates the latency in cycles for the given `batch_size`.
    pub fn compute_latency(&self, batch_size: BatchSize) -> Cycle {
        use std::cmp::max;
        Cycle(self.a * max(batch_size, self.m) + self.b)
    }
}
