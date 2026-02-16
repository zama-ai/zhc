//! Ergonomic iterator collection methods.
//!
//! This module provides extension traits that add shorthand collection methods to iterators,
//! eliminating the need for turbofish syntax when the target collection type is known.
//!
//! Instead of writing `.collect::<Vec<_>>()`, you can simply call `.covec()`. This improves
//! readability in iterator chains where the collection type is a minor detail rather than
//! the focus of the expression.
//!
//! # Available Methods
//!
//! | Method      | Collects into     | Equivalent to                    |
//! |-------------|-------------------|----------------------------------|
//! | `.covec()`  | [`Vec<T>`]        | `.collect::<Vec<_>>()`           |
//! | `.cosvec()` | [`SmallVec<T>`]   | `.collect::<SmallVec<_>>()`      |
//! | `.codeque()`| [`VecDeque<T>`]   | `.collect::<VecDeque<_>>()`      |
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_utils::iter::CollectInVec;
//! let squares = (1..=5).map(|x| x * x).covec();
//! assert_eq!(squares, vec![1, 4, 9, 16, 25]);
//! ```
//!
//! [`SmallVec<T>`]: crate::small::SmallVec

use std::collections::VecDeque;

use crate::small::SmallVec;

/// Extension trait for collecting iterator elements into a [`Vec`].
///
/// This trait is automatically implemented for all iterators, providing the [`covec`](Self::covec)
/// method as a concise alternative to `.collect::<Vec<_>>()`.
pub trait CollectInVec<T> {
    /// Collects all elements from the iterator into a [`Vec`].
    ///
    /// This is a shorthand for `.collect::<Vec<_>>()` that improves readability in iterator
    /// chains by avoiding turbofish syntax.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::CollectInVec;
    /// let evens = (0..10).filter(|n| n % 2 == 0).covec();
    /// assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    /// ```
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

/// Extension trait for collecting iterator elements into a [`SmallVec`].
///
/// This trait is automatically implemented for all iterators, providing the
/// [`cosvec`](Self::cosvec) method as a concise alternative to `.collect::<SmallVec<_>>()`.
///
/// [`SmallVec`]: crate::small::SmallVec
pub trait CollectInSmallVec<T> {
    /// Collects all elements from the iterator into a [`SmallVec`].
    ///
    /// This is a shorthand for `.collect::<SmallVec<_>>()` that improves readability in iterator
    /// chains by avoiding turbofish syntax. Use this when you expect the result to typically
    /// contain few elements and want to avoid heap allocation in the common case.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::CollectInSmallVec;
    /// let small = [1, 2, 3].iter().copied().cosvec();
    /// ```
    ///
    /// [`SmallVec`]: crate::small::SmallVec
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

/// Extension trait for collecting iterator elements into a [`VecDeque`].
///
/// This trait is automatically implemented for all iterators, providing the
/// [`codeque`](Self::codeque) method as a concise alternative to `.collect::<VecDeque<_>>()`.
pub trait CollectInDeque<T> {
    /// Collects all elements from the iterator into a [`VecDeque`].
    ///
    /// This is a shorthand for `.collect::<VecDeque<_>>()` that improves readability in iterator
    /// chains by avoiding turbofish syntax. Use this when you need a double-ended queue for
    /// efficient push/pop operations at both ends.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::CollectInDeque;
    /// # use std::collections::VecDeque;
    /// let mut deque = (1..=3).codeque();
    /// deque.push_front(0);
    /// assert_eq!(deque, VecDeque::from([0, 1, 2, 3]));
    /// ```
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
