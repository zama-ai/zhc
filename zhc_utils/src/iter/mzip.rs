//! Multi-iterator zipping for tuples of up to five iterators.
//!
//! This module extends the standard library's [`Iterator::zip`], which only handles pairs, to
//! support zipping three, four, or five iterators at once. The [`MultiZip`] trait is implemented
//! for tuples of iterators, allowing you to call [`mzip`](MultiZip::mzip) directly on the tuple.
//!
//! The resulting iterator yields tuples of items from each input iterator and stops as soon as
//! the shortest iterator is exhausted — remaining elements from longer iterators are discarded.
//!
//! # Examples
//!
//! Zipping three iterators together:
//!
//! ```rust,no_run
//! # use zhc_utils::iter::MultiZip;
//! let names = ["Alice", "Bob", "Carol"].into_iter();
//! let ages = [30, 25, 35].into_iter();
//! let scores = [95.0, 88.5, 92.0].into_iter();
//!
//! for (name, age, score) in (names, ages, scores).mzip() {
//!     println!("{name} (age {age}): {score}");
//! }
//! ```
//!
//! When iterators have different lengths, iteration stops at the shortest:
//!
//! ```rust,no_run
//! # use zhc_utils::iter::MultiZip;
//! let a = [1, 2, 3, 4, 5].into_iter();
//! let b = ['x', 'y'].into_iter();
//! let c = [true, false, true].into_iter();
//!
//! let result: Vec<_> = (a, b, c).mzip().collect();
//! assert_eq!(result, vec![(1, 'x', true), (2, 'y', false)]);
//! ```

/// An extension trait for zipping multiple iterators into a single iterator of tuples.
///
/// This trait is implemented for tuples of two to five iterators, enabling ergonomic multi-way
/// zipping without nested calls to [`Iterator::zip`]. The trait consumes the tuple of iterators
/// and produces a new iterator that yields tuples of their items in lockstep.
///
/// Iteration terminates when any of the input iterators is exhausted. Elements remaining in
/// longer iterators are silently discarded.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_utils::iter::MultiZip;
/// let xs = [1, 2, 3].into_iter();
/// let ys = [4, 5, 6].into_iter();
/// let zs = [7, 8, 9].into_iter();
///
/// let sums: Vec<_> = (xs, ys, zs).mzip().map(|(x, y, z)| x + y + z).collect();
/// assert_eq!(sums, vec![12, 15, 18]);
/// ```
pub trait MultiZip {
    /// The iterator type returned by [`mzip`](Self::mzip).
    ///
    /// This is a concrete struct ([`Zip2`], [`Zip3`], etc.) that implements [`Iterator`] and
    /// yields tuples matching the arity of the input.
    type Zipped;

    /// Combines the iterators in this tuple into a single iterator yielding tuples.
    ///
    /// This method consumes the tuple of iterators and returns a new iterator that advances all
    /// inputs in parallel. Each call to `next()` on the returned iterator pulls one element from
    /// each source; if any source is exhausted, iteration ends immediately.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::MultiZip;
    /// let letters = ['a', 'b', 'c'].into_iter();
    /// let numbers = [1, 2, 3].into_iter();
    ///
    /// let pairs: Vec<_> = (letters, numbers).mzip().collect();
    /// assert_eq!(pairs, vec![('a', 1), ('b', 2), ('c', 3)]);
    /// ```
    fn mzip(self) -> Self::Zipped;
}

macro_rules! impl_multizip {
    ($n:literal, $zip_struct:ident, ($($generic:ident),+), ($($field:tt),+), ($($param:ident),+)) => {
        #[doc = concat!("An iterator that zips ", $n, " iterators together.")]
        ///
        /// This struct is created by calling [`mzip`](MultiZip::mzip) on a tuple of iterators.
        /// See [`MultiZip`] for usage examples.
        pub struct $zip_struct<$($generic: Iterator),+>($($generic),+);

        impl<$($generic: Iterator),+> Iterator for $zip_struct<$($generic),+> {
            type Item = ($($generic::Item),+);

            fn next(&mut self) -> Option<Self::Item> {
                match ($(self.$field.next()),+) {
                    ($(Some($param)),+) => Some(($($param),+)),
                    _ => None
                }
            }
        }

        impl<$($generic: Iterator),+> MultiZip for ($($generic),+) {
            type Zipped = $zip_struct<$($generic),+>;

            fn mzip(self) -> Self::Zipped {
                $zip_struct($(self.$field),+)
            }
        }
    };
}

impl_multizip!(2, Zip2, (A, B), (0, 1), (a, b));
impl_multizip!(3, Zip3, (A, B, C), (0, 1, 2), (a, b, c));
impl_multizip!(4, Zip4, (A, B, C, D), (0, 1, 2, 3), (a, b, c, d));
impl_multizip!(5, Zip5, (A, B, C, D, E), (0, 1, 2, 3, 4), (a, b, c, d, e));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip3_basic() {
        let iter1 = vec![1, 2, 3].into_iter();
        let iter2 = vec!['a', 'b', 'c'].into_iter();
        let iter3 = vec![true, false, true].into_iter();

        let zipped: Vec<_> = (iter1, iter2, iter3).mzip().collect();

        assert_eq!(
            zipped,
            vec![(1, 'a', true), (2, 'b', false), (3, 'c', true)]
        );
    }

    #[test]
    fn test_zip3_different_lengths() {
        let iter1 = vec![1, 2].into_iter();
        let iter2 = vec!['a', 'b', 'c'].into_iter();
        let iter3 = vec![true].into_iter();

        let zipped: Vec<_> = (iter1, iter2, iter3).mzip().collect();

        // Should stop at the shortest iterator
        assert_eq!(zipped, vec![(1, 'a', true)]);
    }

    #[test]
    fn test_zip3_empty() {
        let iter1 = Vec::<usize>::new().into_iter();
        let iter2 = vec!['a', 'b'].into_iter();
        let iter3 = vec![true, false].into_iter();

        let zipped: Vec<_> = (iter1, iter2, iter3).mzip().collect();

        assert_eq!(zipped, vec![]);
    }
}
