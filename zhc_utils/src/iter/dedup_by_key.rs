//! Key-based lazy deduplication for iterators.
//!
//! This module provides [`DedupByKey`], an iterator adapter that filters out elements whose
//! projected key has already been seen, while preserving encounter order. The key extraction
//! function is applied to each element, and the first element producing a given key is yielded;
//! subsequent elements with the same key are skipped.
//!
//! The [`DedupedByKey`] extension trait adds the [`dedup_by_key`](DedupedByKey::dedup_by_key)
//! method to any iterator.
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_utils::iter::DedupedByKey;
//! let items = vec![(1, "a"), (2, "b"), (1, "c"), (3, "d"), (2, "e")];
//! let unique: Vec<_> = items.into_iter().dedup_by_key(|&(k, _)| k).collect();
//! assert_eq!(unique, vec![(1, "a"), (2, "b"), (3, "d")]);
//! ```

use std::hash::Hash;

use crate::small::SmallSet;

/// An iterator adapter that yields elements whose projected key has not been seen before.
///
/// Created by calling [`dedup_by_key`](DedupedByKey::dedup_by_key) on an iterator.
pub struct DedupByKey<I: Iterator, K: Hash + Eq, F> {
    iterator: I,
    key_fn: F,
    set: SmallSet<K>,
}

impl<I, K, F> Iterator for DedupByKey<I, K, F>
where
    I: Iterator,
    K: Hash + Eq + Clone,
    F: FnMut(&I::Item) -> K,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let val = self.iterator.next()?;
            let key = (self.key_fn)(&val);
            if !self.set.contains(&key) {
                self.set.insert(key);
                return Some(val);
            }
        }
    }
}

/// An extension trait that provides key-based lazy deduplication for iterators.
///
/// Importing this trait brings [`dedup_by_key`](DedupedByKey::dedup_by_key) into scope for any
/// iterator.
pub trait DedupedByKey: Iterator + Sized {
    /// Returns an iterator that yields elements whose projected key has not been seen before.
    ///
    /// The closure `f` extracts a key from each element. Keys are compared via `Hash + Eq`. The
    /// first element producing a given key is yielded; subsequent elements with the same key are
    /// skipped. Encounter order is preserved.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::DedupedByKey;
    /// let words = ["apple", "avocado", "banana", "blueberry", "cherry"];
    /// let unique: Vec<_> = words.iter().dedup_by_key(|w| w.chars().next()).collect();
    /// assert_eq!(unique, vec![&"apple", &"banana", &"cherry"]);
    /// ```
    fn dedup_by_key<K, F>(self, key_fn: F) -> DedupByKey<Self, K, F>
    where
        K: Hash + Eq + Clone,
        F: FnMut(&Self::Item) -> K;
}

impl<I: Iterator + Sized> DedupedByKey for I {
    fn dedup_by_key<K, F>(self, key_fn: F) -> DedupByKey<Self, K, F>
    where
        K: Hash + Eq + Clone,
        F: FnMut(&Self::Item) -> K,
    {
        DedupByKey {
            iterator: self,
            key_fn,
            set: SmallSet::new(),
        }
    }
}
