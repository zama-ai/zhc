use std::collections::VecDeque;

use crate::SmallVec;

/// Collects iterator elements into a `Vec`.
pub trait CollectInVec<T> {
    /// Consumes the iterator and collects elements into a `Vec`.
    fn covec(self) -> Vec<T>;
}

impl<T, I> CollectInVec<T> for I
where
    I: Iterator<Item = T>,
{
    fn covec(self) -> Vec<T> {
        self.collect()
    }
}

/// Collects iterator elements into a `SmallVec`.
pub trait CollectInSmallVec<T> {
    /// Consumes the iterator and collects elements into a `SmallVec`.
    fn cosvec(self) -> SmallVec<T>;
}

impl<T, I> CollectInSmallVec<T> for I
where
    I: Iterator<Item = T>,
{
    fn cosvec(self) -> SmallVec<T> {
        self.collect()
    }
}

/// Collects iterator elements into a `VecDeque`.
pub trait CollectInDeque<T> {
    /// Consumes the iterator and collects elements into a `VecDeque`.
    fn codeque(self) -> VecDeque<T>;
}

impl<T, I> CollectInDeque<T> for I
where
    I: Iterator<Item = T>,
{
    fn codeque(self) -> VecDeque<T> {
        self.collect()
    }
}
