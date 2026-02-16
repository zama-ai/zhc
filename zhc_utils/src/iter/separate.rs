//! Iterator adapter for inserting separators between elements.
//!
//! This module provides the [`Separate`] trait, which extends any iterator with methods to
//! intersperse separator elements between its items. Unlike strict interleaving (which alternates
//! elements and stops when either iterator ends), separation inserts separators *between* elements
//! only — no leading or trailing separators — and always exhausts the main iterator.
//!
//! This is particularly useful for formatting output, where you want to join elements with a
//! delimiter (like underscores or commas) without a trailing separator.
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_utils::iter::Separate;
//! let parts = ["foo", "bar", "baz"];
//! let result: Vec<_> = parts.into_iter().separate_with(|| "_").collect();
//! assert_eq!(result, vec!["foo", "_", "bar", "_", "baz"]);
//! ```

use std::iter::RepeatWith;

use zhc_utils_macro::fsm;

/// An iterator that yields elements from a main iterator with separator elements interspersed.
///
/// `Separated` is created by the [`Separate::separate`] or [`Separate::separate_with`] methods.
/// It alternates between yielding elements from the main iterator and elements from the separator
/// iterator, but only inserts separators *between* main elements — never before the first or after
/// the last.
///
/// The separator iterator is advanced once for each gap between main elements. If the main
/// iterator yields `n` elements, the separator iterator will be polled `n - 1` times.
///
/// # Type Parameters
///
/// - `I`: The main iterator type.
/// - `S`: The separator iterator type, which must yield the same item type as `I`.
#[fsm]
pub enum Separated<I: Iterator, S: Iterator<Item = I::Item>> {
    /// The iterator will yield an element from the main iterator on the next call to `next`.
    OnIter {
        /// The buffered element to yield.
        next: I::Item,
        /// The remaining main iterator.
        iter: I,
        /// The separator iterator.
        sep: S,
    },
    /// The iterator will yield a separator element on the next call to `next`.
    OnSep {
        /// The buffered main element to yield after the separator.
        next: I::Item,
        /// The remaining main iterator.
        iter: I,
        /// The separator iterator.
        sep: S,
    },
    /// The iterator has been exhausted.
    Finished,
}

impl<I: Iterator, S: Iterator<Item = I::Item>> Iterator for Separated<I, S> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = None;
        self.transition(|old| match old {
            Separated::OnIter {
                next,
                mut iter,
                sep,
            } => {
                output = Some(next);
                match iter.next() {
                    Some(next) => Separated::OnSep { next, iter, sep },
                    None => Self::Finished,
                }
            }
            Separated::OnSep {
                next,
                iter,
                mut sep,
            } => {
                output = sep.next();
                Separated::OnIter { next, iter, sep }
            }
            Separated::Finished => {
                output = None;
                Separated::Finished
            }
            _ => unreachable!(),
        });
        output
    }
}

/// Extension trait for inserting separator elements between iterator items.
///
/// `Separate` is implemented for all iterators and provides two methods for interspersing
/// separator elements. This is useful when you need to join elements with delimiters, such as
/// formatting a sequence with underscores or commas between items.
///
/// Unlike [`Interleave`], which strictly alternates between two iterators and stops when either
/// is exhausted, `Separate` guarantees that all elements from the main iterator are yielded,
/// with separators appearing only *between* them.
///
/// [`Interleave`]: crate::iter::Interleave
pub trait Separate
where
    Self: Iterator + Sized,
{
    /// Intersperses elements from a separator iterator between elements of this iterator.
    ///
    /// Returns a [`Separated`] iterator that yields elements from `self` with elements from `sep`
    /// inserted between each pair of consecutive main elements. The separator iterator is polled
    /// once per gap, so if `self` yields `n` elements, `sep` will be polled `n - 1` times.
    ///
    /// If this iterator is empty, the returned iterator is also empty and `sep` is never polled.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::Separate;
    /// let items = [1, 2, 3];
    /// let seps = [10, 20];
    /// let result: Vec<_> = items.into_iter().separate(seps.into_iter()).collect();
    /// assert_eq!(result, vec![1, 10, 2, 20, 3]);
    /// ```
    fn separate<S: Iterator<Item = Self::Item>>(self, sep: S) -> Separated<Self, S>;

    /// Intersperses separator values produced by a closure between elements of this iterator.
    ///
    /// This is a convenience method equivalent to `self.separate(std::iter::repeat_with(f))`.
    /// The closure `f` is called once for each gap between consecutive elements.
    ///
    /// This method is particularly useful when the separator is a constant or easily computed
    /// value, such as a delimiter string or a default instance.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::Separate;
    /// let words = ["hello", "world"];
    /// let result: Vec<_> = words.into_iter().separate_with(|| ", ").collect();
    /// assert_eq!(result, vec!["hello", ", ", "world"]);
    /// ```
    fn separate_with<F: FnMut() -> Self::Item>(self, f: F) -> Separated<Self, RepeatWith<F>>;
}

impl<I: Iterator> Separate for I {
    fn separate<S: Iterator<Item = Self::Item>>(mut self, sep: S) -> Separated<Self, S> {
        match self.next() {
            Some(next) => Separated::OnIter {
                next,
                iter: self,
                sep,
            },
            None => Separated::Finished,
        }
    }

    fn separate_with<F: FnMut() -> Self::Item>(self, f: F) -> Separated<Self, RepeatWith<F>> {
        self.separate(std::iter::repeat_with(f))
    }
}
