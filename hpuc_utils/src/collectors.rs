use std::collections::VecDeque;

use crate::SmallVec;

pub trait CollectInVec<T> {
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

pub trait CollectInSmallVec<T> {
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

pub trait CollectInDeque<T> {
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
