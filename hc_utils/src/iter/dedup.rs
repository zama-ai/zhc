//! Lazy deduplication for iterators.
//!
//! This module provides [`Dedup`], an iterator adapter that filters out duplicate elements while
//! preserving the order of first occurrences. Unlike collecting into a set and iterating, this
//! approach is lazy — duplicates are filtered on-the-fly as items are consumed.
//!
//! The [`Deduped`] extension trait adds the [`dedup`](Deduped::dedup) method to any iterator whose
//! items implement `Hash + Eq + Clone`.
//!
//! # Example
//!
//! ```rust,no_run
//! # use hc_utils::iter::Deduped;
//! let items = vec![1, 2, 3, 2, 1, 4, 3, 5];
//! let unique: Vec<_> = items.into_iter().dedup().collect();
//! assert_eq!(unique, vec![1, 2, 3, 4, 5]);
//! ```

use std::hash::Hash;

use crate::small::SmallSet;

/// An iterator adapter that yields each unique element only once, in encounter order.
///
/// `Dedup` wraps an underlying iterator and maintains a set of previously seen items. When the
/// underlying iterator produces a value, `Dedup` checks whether it has been seen before: if so,
/// the value is skipped; if not, it is recorded and yielded.
///
/// This adapter is lazy — it processes elements one at a time as they are requested, making it
/// suitable for large or infinite iterators where collecting all elements upfront would be
/// impractical.
///
/// `Dedup` is typically created by calling [`dedup`](Deduped::dedup) on an iterator, rather than
/// constructing it directly.
pub struct Dedup<I: Iterator>
where
    I::Item: Hash + Eq + Clone,
{
    iterator: I,
    set: SmallSet<I::Item>,
}

impl<I: Iterator> Iterator for Dedup<I>
where
    I::Item: Hash + Eq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(val) => {
                if self.set.contains(&val) {
                    self.next()
                } else {
                    self.set.insert(val.clone());
                    Some(val)
                }
            }
            None => None,
        }
    }
}

/// An extension trait that provides lazy deduplication for iterators.
///
/// `Deduped` is automatically implemented for any iterator whose items implement `Hash + Eq +
/// Clone`. Importing this trait brings the [`dedup`](Deduped::dedup) method into scope, enabling
/// fluent chaining with other iterator adapters.
pub trait Deduped
where
    Self: Iterator + Sized,
    Self::Item: Hash + Eq + Clone,
{
    /// Returns an iterator that yields each unique element only once.
    ///
    /// Elements are compared using their `Hash` and `Eq` implementations. The first occurrence of
    /// each element is yielded; subsequent duplicates are silently skipped. The relative order of
    /// first occurrences is preserved.
    ///
    /// Because deduplication happens lazily, this method works well with iterators that are large,
    /// expensive to fully materialize, or even infinite (when combined with `take` or similar).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use hc_utils::iter::Deduped;
    /// let words = ["apple", "banana", "apple", "cherry", "banana"];
    /// let unique: Vec<_> = words.iter().dedup().collect();
    /// assert_eq!(unique, vec![&"apple", &"banana", &"cherry"]);
    /// ```
    fn dedup(self) -> Dedup<Self>;
}

impl<I: Iterator> Deduped for I
where
    Self::Item: Hash + Eq + Clone,
{
    fn dedup(self) -> Dedup<Self> {
        Dedup {
            iterator: self,
            set: SmallSet::new(),
        }
    }
}
