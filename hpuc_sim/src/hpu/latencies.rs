use crate::Cycle;
use serde::Serialize;

use super::BatchSize;

/// A constant latency.
/// $ dur(batch_size) = c $
#[derive(Debug, Clone, Serialize)]
pub struct ConstantLatency(usize);

impl ConstantLatency {
    pub fn new(c: usize) -> Self {
        Self(c)
    }

    pub fn compute_latency(&self) -> Cycle {
        Cycle(self.0)
    }
}

/// An flat then linear latency, whose duration depends on the size of the processed batch.
/// $ dur(batch_size) = a * max(batch_size, m) + b $
#[derive(Debug, Clone, Serialize)]
pub struct FlatLinLatency {
    a: usize,
    b: usize,
    m: usize,
}

impl FlatLinLatency {
    pub fn new(a: usize, b: usize, m: usize) -> Self {
        Self { a, b, m }
    }
    pub fn compute_latency(&self, batch_size: BatchSize) -> Cycle {
        use std::cmp::max;
        Cycle(self.a * max(batch_size, self.m) + self.b)
    }
}
