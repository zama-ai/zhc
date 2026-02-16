//! Equality check across all elements of an iterator.
//!
//! This module provides the [`AllEq`] extension trait, which adds a method to test whether every
//! element in an iterator is equal to every other. It is automatically implemented for any
//! iterator whose items implement `PartialEq`.
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_utils::iter::AllEq;
//! let uniform = vec![7, 7, 7, 7];
//! assert_eq!(uniform.into_iter().all_eq(), Some(true));
//!
//! let mixed = vec![1, 2, 1];
//! assert_eq!(mixed.into_iter().all_eq(), Some(false));
//! ```

/// An extension trait that checks whether all elements of an iterator are equal.
///
/// `AllEq` is automatically implemented for any iterator whose items implement `PartialEq`.
/// Importing this trait brings the [`all_eq`](AllEq::all_eq) method into scope.
pub trait AllEq {
    /// Consumes the iterator and checks whether all elements are equal to each other.
    ///
    /// This method compares every element to the first using `PartialEq`. It returns `Some(true)`
    /// if all elements are equal, `Some(false)` if any element differs from the first, or `None`
    /// if the iterator is empty. A single-element iterator always returns `Some(true)`.
    ///
    /// The check short-circuits: iteration stops as soon as a non-equal element is found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::AllEq;
    /// assert_eq!([5, 5, 5].into_iter().all_eq(), Some(true));
    /// assert_eq!([5, 5, 6].into_iter().all_eq(), Some(false));
    /// assert_eq!(std::iter::empty::<i32>().all_eq(), None);
    /// ```
    fn all_eq(self) -> Option<bool>;
}

impl<I: Iterator<Item = T>, T: PartialEq> AllEq for I {
    fn all_eq(mut self) -> Option<bool> {
        let Some(first) = self.next() else {
            return None;
        };
        Some(self.all(|a| a == first))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty_iterator() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.into_iter().all_eq(), None);
    }

    #[test]
    fn test_single_element() {
        let single = vec![42];
        assert_eq!(single.into_iter().all_eq(), Some(true));
    }

    #[test]
    fn test_all_equal_elements() {
        let all_same = vec![5, 5, 5, 5];
        assert_eq!(all_same.into_iter().all_eq(), Some(true));
    }

    #[test]
    fn test_not_all_equal_elements() {
        let mixed = vec![1, 2, 3];
        assert_eq!(mixed.into_iter().all_eq(), Some(false));
    }

    #[test]
    fn test_partially_equal_elements() {
        let partial = vec![7, 7, 8, 7];
        assert_eq!(partial.into_iter().all_eq(), Some(false));
    }
}
