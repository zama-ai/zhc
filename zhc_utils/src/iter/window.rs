//! Sliding window iteration with boundary awareness.
//!
//! This module provides a sliding window iterator that, unlike standard windowing approaches,
//! distinguishes between three phases of iteration: the *prelude* (window filling), the
//! *complete* phase (full windows), and the *postlude* (window draining). This makes it
//! suitable for algorithms that need to handle boundary conditions explicitly, such as
//! convolutions, moving averages with edge handling, or any operation where partial windows
//! at the start and end require different treatment.
//!
//! The core abstraction is the [`Slide`] trait, which extends all iterators with a
//! [`slide`](Slide::slide) method. Each element yielded is wrapped in a [`Slider`] enum
//! that indicates the current phase.
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_utils::iter::{Slide, Slider};
//! let windows: Vec<_> = [1, 2, 3, 4].into_iter().slide::<3>().collect();
//!
//! // Prelude: window is filling
//! // [1]       -> Prelude
//! // [1, 2]    -> Prelude
//!
//! // Complete: full window
//! // [1, 2, 3] -> Complete
//! // [2, 3, 4] -> Complete
//!
//! // Postlude: window is draining
//! // [3, 4]    -> Postlude
//! // [4]       -> Postlude
//! ```
//!
//! To work with only specific phases, use the provided filter predicates:
//!
//! ```rust,no_run
//! # use zhc_utils::iter::{Slide, Slider, filter_out_noncompletes};
//! let complete_only: Vec<_> = [1, 2, 3, 4, 5]
//!     .into_iter()
//!     .slide::<3>()
//!     .filter(filter_out_noncompletes)
//!     .map(|s| s.unwrap_complete())
//!     .collect();
//! // [[1,2,3], [2,3,4], [3,4,5]]
//! ```

use crate::{small::VArray, varr};
use std::mem::MaybeUninit;

/// A sliding window element tagged with its phase within the iteration.
///
/// As the sliding window traverses a sequence, it passes through three phases:
///
/// - **Prelude**: The window is still accumulating elements and contains fewer than `N` items. This
///   occurs at the beginning of iteration before enough elements have been seen.
///
/// - **Complete**: The window contains exactly `N` elements. This is the "steady state" that occurs
///   for most elements in sequences longer than the window size.
///
/// - **Postlude**: The input is exhausted and the window is draining. The window shrinks by one
///   element per iteration until empty.
///
/// All three variants contain a [`VArray`] holding the current window contents. The array
/// length varies by phase: preludes grow from 1 to N-1, complete windows have exactly N
/// elements, and postludes shrink from N-1 to 1.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Slider<A, const N: usize> {
    /// The window is filling up and contains fewer than `N` elements.
    Prelude(VArray<A, N>),
    /// The window is full with exactly `N` elements.
    Complete(VArray<A, N>),
    /// The input is exhausted and the window is draining.
    Postlude(VArray<A, N>),
}

impl<A, const N: usize> Slider<A, N> {
    /// Extracts the inner array from a [`Prelude`](Slider::Prelude) variant.
    ///
    /// This is a convenience method for cases where you have already verified or filtered
    /// to ensure only prelude values are present.
    ///
    /// # Panics
    ///
    /// Panics if called on a [`Complete`](Slider::Complete) or [`Postlude`](Slider::Postlude)
    /// variant.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{Slide, Slider};
    /// let preludes: Vec<_> = [1, 2, 3]
    ///     .into_iter()
    ///     .slide::<3>()
    ///     .filter(|s| matches!(s, Slider::Prelude(_)))
    ///     .map(|s| s.unwrap_prelude())
    ///     .collect();
    /// ```
    pub fn unwrap_prelude(self) -> VArray<A, N> {
        match self {
            Slider::Prelude(sv) => sv,
            _ => panic!(),
        }
    }

    /// Extracts the inner array from a [`Complete`](Slider::Complete) variant.
    ///
    /// This is a convenience method for cases where you have already verified or filtered
    /// to ensure only complete windows are present. The returned array is guaranteed to
    /// contain exactly `N` elements.
    ///
    /// # Panics
    ///
    /// Panics if called on a [`Prelude`](Slider::Prelude) or [`Postlude`](Slider::Postlude)
    /// variant.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{Slide, Slider, filter_out_noncompletes};
    /// let windows: Vec<_> = [1, 2, 3, 4]
    ///     .into_iter()
    ///     .slide::<2>()
    ///     .filter(filter_out_noncompletes)
    ///     .map(|s| s.unwrap_complete())
    ///     .collect();
    /// ```
    pub fn unwrap_complete(self) -> VArray<A, N> {
        match self {
            Slider::Complete(sv) => sv,
            _ => panic!(),
        }
    }

    /// Extracts the inner array from a [`Postlude`](Slider::Postlude) variant.
    ///
    /// This is a convenience method for cases where you have already verified or filtered
    /// to ensure only postlude values are present.
    ///
    /// # Panics
    ///
    /// Panics if called on a [`Prelude`](Slider::Prelude) or [`Complete`](Slider::Complete)
    /// variant.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{Slide, Slider};
    /// let postludes: Vec<_> = [1, 2, 3]
    ///     .into_iter()
    ///     .slide::<3>()
    ///     .filter(|s| matches!(s, Slider::Postlude(_)))
    ///     .map(|s| s.unwrap_postlude())
    ///     .collect();
    /// ```
    pub fn unwrap_postlude(self) -> VArray<A, N> {
        match self {
            Slider::Postlude(sv) => sv,
            _ => panic!(),
        }
    }
}

/// The iterator adapter returned by [`Slide::slide`].
///
/// This struct is created by calling [`slide`](Slide::slide) on any iterator. It wraps the
/// source iterator and maintains an internal buffer to produce sliding windows tagged with
/// their phase information.
pub struct Slided<I: Iterator, const N: usize> {
    iter: I,
    buffer: MaybeUninit<Slider<I::Item, N>>,
}

impl<I: Iterator, const N: usize> Iterator for Slided<I, N>
where
    I::Item: Clone,
{
    type Item = Slider<I::Item, N>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => {
                let current_buffer = unsafe { self.buffer.assume_init_read() };
                let output = current_buffer.clone();
                match current_buffer {
                    Slider::Prelude(mut sv) | Slider::Complete(mut sv) => {
                        if sv.len() == N {
                            sv.as_mut_slice().rotate_left(1);
                            sv.as_mut_slice()[N - 1] = v;
                            self.buffer.write(Slider::Complete(sv))
                        } else if sv.len() == N - 1 {
                            sv.push(v);
                            self.buffer.write(Slider::Complete(sv))
                        } else {
                            sv.push(v);
                            self.buffer.write(Slider::Prelude(sv))
                        }
                    }
                    _ => unreachable!(),
                };
                Some(output)
            }
            None => {
                let current_buffer = unsafe { self.buffer.assume_init_read() };
                let output = current_buffer.clone();
                match current_buffer {
                    Slider::Prelude(mut sv)
                    | Slider::Complete(mut sv)
                    | Slider::Postlude(mut sv) => {
                        if !sv.is_empty() {
                            sv.as_mut_slice().rotate_left(1);
                            sv.pop();
                        }
                        self.buffer.write(Slider::Postlude(sv))
                    }
                };
                if let Slider::Postlude(sv) = &output
                    && sv.is_empty()
                {
                    None
                } else {
                    Some(output)
                }
            }
        }
    }
}

/// Extension trait providing sliding window iteration on any iterator.
///
/// This trait is implemented for all types that implement [`Iterator`], enabling the
/// [`slide`](Slide::slide) method to be called on any iterator.
pub trait Slide
where
    Self: Iterator + Sized,
{
    /// Creates a sliding window iterator of size `N`.
    ///
    /// The returned iterator yields [`Slider`] values that indicate whether each window
    /// is in the prelude (filling), complete (full), or postlude (draining) phase. This
    /// allows callers to handle boundary conditions explicitly rather than silently
    /// truncating or padding.
    ///
    /// The window size `N` must be greater than 1. For a sequence of length L with window
    /// size N, the iterator yields:
    /// - min(L, N-1) prelude elements (growing windows from size 1 to min(L, N-1))
    /// - max(0, L-N+1) complete elements (full windows of size N)
    /// - min(L, N-1) postlude elements (shrinking windows from size min(L, N)-1 to 1)
    ///
    /// An empty input iterator yields no elements.
    ///
    /// # Panics
    ///
    /// Panics if `N <= 1`. A window size of 1 would be degenerate (always complete), and
    /// a window size of 0 is meaningless.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{Slide, Slider};
    /// # use zhc_utils::varr;
    /// let mut iter = [1, 2, 3].into_iter().slide::<2>();
    ///
    /// assert_eq!(iter.next(), Some(Slider::Prelude(varr![1])));
    /// assert_eq!(iter.next(), Some(Slider::Complete(varr![1, 2])));
    /// assert_eq!(iter.next(), Some(Slider::Complete(varr![2, 3])));
    /// assert_eq!(iter.next(), Some(Slider::Postlude(varr![3])));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn slide<const N: usize>(self) -> Slided<Self, N>;
}

impl<I: Iterator> Slide for I {
    fn slide<const N: usize>(mut self) -> Slided<Self, N> {
        assert!(N > 1);
        let buffer = match self.next() {
            Some(v) => MaybeUninit::new(Slider::Prelude(varr![v])),
            None => MaybeUninit::new(Slider::Postlude(varr![])),
        };
        Slided { iter: self, buffer }
    }
}

/// Filter predicate that keeps prelude and complete windows, removing postludes.
///
/// Returns true for [`Slider::Prelude`] and [`Slider::Complete`] variants, false for
/// [`Slider::Postlude`]. Intended for use with [`Iterator::filter`].
///
/// # Example
///
/// ```rust,no_run
/// # use zhc_utils::iter::{Slide, filter_out_postludes};
/// let without_postludes: Vec<_> = [1, 2, 3]
///     .into_iter()
///     .slide::<2>()
///     .filter(filter_out_postludes)
///     .collect();
/// // Contains only Prelude([1]) and Complete([1,2]), Complete([2,3])
/// ```
pub fn filter_out_postludes<A, const N: usize>(inp: &Slider<A, N>) -> bool {
    !matches!(inp, Slider::Postlude(_))
}

/// Filter predicate that keeps complete and postlude windows, removing preludes.
///
/// Returns true for [`Slider::Complete`] and [`Slider::Postlude`] variants, false for
/// [`Slider::Prelude`]. Intended for use with [`Iterator::filter`].
///
/// # Example
///
/// ```rust,no_run
/// # use zhc_utils::iter::{Slide, filter_out_preludes};
/// let without_preludes: Vec<_> = [1, 2, 3]
///     .into_iter()
///     .slide::<2>()
///     .filter(filter_out_preludes)
///     .collect();
/// // Contains only Complete([1,2]), Complete([2,3]), and Postlude([3])
/// ```
pub fn filter_out_preludes<A, const N: usize>(inp: &Slider<A, N>) -> bool {
    !matches!(inp, Slider::Prelude(_))
}

/// Filter predicate that keeps only complete windows, removing preludes and postludes.
///
/// Returns true only for [`Slider::Complete`] variants. Intended for use with
/// [`Iterator::filter`]. This is the most common filter when you want standard sliding
/// window behavior without partial windows at the boundaries.
///
/// # Example
///
/// ```rust,no_run
/// # use zhc_utils::iter::{Slide, Slider, filter_out_noncompletes};
/// let complete_only: Vec<_> = [1, 2, 3, 4]
///     .into_iter()
///     .slide::<2>()
///     .filter(filter_out_noncompletes)
///     .map(|s| s.unwrap_complete())
///     .collect();
/// // [[1,2], [2,3], [3,4]]
/// ```
pub fn filter_out_noncompletes<A, const N: usize>(inp: &Slider<A, N>) -> bool {
    matches!(inp, Slider::Complete(_))
}

/// Extension trait for iterators yielding [`Slider`] values, providing convenience methods
/// to filter out specific phases.
///
/// This trait is automatically implemented for any iterator whose items are [`Slider`] values.
/// It provides methods that are more ergonomic than manually calling [`Iterator::filter`] with
/// the filter predicates.
pub trait SliderExt: Iterator + Sized {
    /// Filters out prelude windows, keeping only complete and postlude windows.
    ///
    /// This is equivalent to calling `.filter(filter_out_preludes)` but is more concise
    /// and self-documenting.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{Slide, SliderExt};
    /// let without_preludes: Vec<_> = [1, 2, 3]
    ///     .into_iter()
    ///     .slide::<2>()
    ///     .skip_preludes()
    ///     .collect();
    /// // Contains only Complete([1,2]), Complete([2,3]), and Postlude([3])
    /// ```
    fn skip_preludes(self) -> impl Iterator<Item = Self::Item>;

    /// Filters out postlude windows, keeping only prelude and complete windows.
    ///
    /// This is equivalent to calling `.filter(filter_out_postludes)` but is more concise
    /// and self-documenting.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{Slide, SliderExt};
    /// let without_postludes: Vec<_> = [1, 2, 3]
    ///     .into_iter()
    ///     .slide::<2>()
    ///     .skip_postludes()
    ///     .collect();
    /// // Contains only Prelude([1]), Complete([1,2]), and Complete([2,3])
    /// ```
    fn skip_postludes(self) -> impl Iterator<Item = Self::Item>;

    /// Filters out prelude and postlude windows, keeping only complete windows.
    ///
    /// This is equivalent to calling `.filter(filter_out_noncompletes)` but is more concise
    /// and self-documenting. This is the most common filtering operation when you want
    /// standard sliding window behavior without partial windows at the boundaries.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{Slide, SliderExt};
    /// let complete_only: Vec<_> = [1, 2, 3, 4]
    ///     .into_iter()
    ///     .slide::<2>()
    ///     .skip_noncompletes()
    ///     .map(|s| s.unwrap_complete())
    ///     .collect();
    /// // [[1,2], [2,3], [3,4]]
    /// ```
    fn skip_noncompletes(self) -> impl Iterator<Item = Self::Item>;
}

impl<I, A, const N: usize> SliderExt for I
where
    I: Iterator<Item = Slider<A, N>> + Sized,
{
    fn skip_preludes(self) -> impl Iterator<Item = Self::Item> {
        self.filter(filter_out_preludes)
    }

    fn skip_postludes(self) -> impl Iterator<Item = Self::Item> {
        self.filter(filter_out_postludes)
    }

    fn skip_noncompletes(self) -> impl Iterator<Item = Self::Item> {
        self.filter(filter_out_noncompletes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_empty_iterator() {
        let vec: Vec<i32> = vec![];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_single_element() {
        let vec = vec![1];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1])));
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_exact_size() {
        let vec = vec![1, 2, 3];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1])));
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1, 2])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![1, 2, 3])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![2, 3])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![3])));
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_larger_than_window() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1])));
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1, 2])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![1, 2, 3])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![2, 3, 4])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![3, 4, 5])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![4, 5])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![5])));
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_collect() {
        let vec = vec![1, 2, 3, 4];
        let result: Vec<_> = vec.into_iter().slide::<2>().collect();
        assert_eq!(
            result,
            vec![
                Slider::Prelude(varr![1]),
                Slider::Complete(varr![1, 2]),
                Slider::Complete(varr![2, 3]),
                Slider::Complete(varr![3, 4]),
                Slider::Postlude(varr![4]),
            ]
        );
    }

    #[test]
    fn test_smaller_than_window() {
        let vec = vec![1, 2, 3, 4];
        let result: Vec<_> = vec.into_iter().slide::<5>().collect();
        assert_eq!(
            result,
            vec![
                Slider::Prelude(varr![1]),
                Slider::Prelude(varr![1, 2]),
                Slider::Prelude(varr![1, 2, 3]),
                Slider::Prelude(varr![1, 2, 3, 4]),
                Slider::Postlude(varr![2, 3, 4]),
                Slider::Postlude(varr![3, 4]),
                Slider::Postlude(varr![4]),
            ]
        );
    }
}
