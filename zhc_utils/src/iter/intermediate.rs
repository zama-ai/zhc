//! Materialize lazy iterators into owned, concrete iterators.
//!
//! This module provides the [`Intermediate`] trait, which allows breaking iterator chains by
//! collecting elements into a temporary buffer and returning a new owned iterator. This is useful
//! when you need to decouple borrowing, iterate multiple times over the same elements, or simply
//! force evaluation of a lazy iterator at a specific point in a chain.
//!
//! The intermediate buffer uses [`SmallVec`] internally, so small sequences avoid heap allocation
//! entirely.
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_utils::iter::Intermediate;
//! // Break an iterator chain to release borrows early
//! let data = vec![1, 2, 3, 4, 5];
//! let doubled: Vec<_> = data.iter()
//!     .map(|x| x * 2)
//!     .intermediate()  // materializes here, releasing borrow on `data`
//!     .filter(|&x| x > 4)
//!     .collect();
//! ```
//!
//! [`SmallVec`]: crate::small::SmallVec

use crate::{iter::collectors::CollectInSmallVec, small::SmallVecIntoIter};

/// Extension trait for materializing a lazy iterator into an owned iterator.
///
/// This trait is automatically implemented for all iterators, providing the
/// [`intermediate`](Self::intermediate) method to collect elements into a temporary buffer and
/// yield them through a new owned iterator. The resulting [`SmallVecIntoIter`] implements
/// [`DoubleEndedIterator`], so you can traverse elements in reverse after materialization.
///
/// [`SmallVecIntoIter`]: crate::small::SmallVecIntoIter
pub trait Intermediate
where
    Self: Iterator,
{
    /// Materializes all elements into an owned iterator.
    ///
    /// Collects the iterator's elements into a [`SmallVec`] buffer, then returns an owned iterator
    /// over that buffer. This effectively "snapshots" the lazy iterator, allowing subsequent
    /// operations to proceed without borrowing the original data source.
    ///
    /// The returned iterator implements [`DoubleEndedIterator`], enabling reverse traversal
    /// regardless of whether the original iterator supported it.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::Intermediate;
    /// let words = ["hello", "world"];
    /// let lengths = words.iter()
    ///     .map(|s| s.len())
    ///     .intermediate()
    ///     .rev()  // reverse traversal now available
    ///     .collect::<Vec<_>>();
    /// assert_eq!(lengths, vec![5, 5]);
    /// ```
    ///
    /// [`SmallVec`]: crate::small::SmallVec
    fn intermediate(self) -> SmallVecIntoIter<Self::Item>;
}

impl<I: Iterator> Intermediate for I {
    fn intermediate(self) -> SmallVecIntoIter<Self::Item> {
        self.cosvec().into_iter()
    }
}
