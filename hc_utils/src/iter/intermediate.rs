use crate::{iter::collectors::CollectInSmallVec, small::SmallVecIntoIter};

pub trait Intermediate
where
    Self: Iterator,
{
    fn intermediate(self) -> SmallVecIntoIter<Self::Item>;
}

impl<I: Iterator> Intermediate for I {
    fn intermediate(self) -> SmallVecIntoIter<Self::Item> {
        self.cosvec().into_iter()
    }
}
