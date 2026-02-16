//! Statistical aggregations for floating-point iterators.
//!
//! This module provides extension traits that add common statistical operations to iterators
//! yielding `f64` values. Both [`Average`] and [`Median`] are automatically implemented for any
//! `Iterator<Item = f64>`, so importing the traits is all that's needed.
//!
//! # Example
//!
//! ```rust,no_run
//! # use hc_utils::iter::{Average, Median};
//! let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//!
//! let mean = samples.iter().copied().average();
//! assert_eq!(mean, Some(3.0));
//!
//! let mid = samples.into_iter().median();
//! assert_eq!(mid, Some(3.0));
//! ```

/// An extension trait that computes the arithmetic mean of an iterator.
///
/// `Average` is automatically implemented for any iterator yielding `f64` values. Importing this
/// trait brings the [`average`](Average::average) method into scope.
pub trait Average
where
    Self: Iterator<Item = f64>,
{
    /// Consumes the iterator and returns the arithmetic mean of all elements.
    ///
    /// The mean is computed by summing all values and dividing by the count. If the iterator is
    /// empty, this method returns `None`; otherwise it returns `Some(mean)`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use hc_utils::iter::Average;
    /// let values = [10.0, 20.0, 30.0];
    /// assert_eq!(values.into_iter().average(), Some(20.0));
    ///
    /// let empty: [f64; 0] = [];
    /// assert_eq!(empty.into_iter().average(), None);
    /// ```
    fn average(self) -> Option<f64>;
}

impl<I: Iterator<Item = f64>> Average for I {
    fn average(mut self) -> Option<f64> {
        let mut val = self.next()?;
        let mut n = 1;
        for v in self {
            val = val + v;
            n += 1;
        }
        Some(val / (n as f64))
    }
}

/// An extension trait that computes the median of an iterator.
///
/// `Median` is automatically implemented for any iterator yielding `f64` values. Importing this
/// trait brings the [`median`](Median::median) method into scope.
pub trait Median
where
    Self: Iterator<Item = f64>,
{
    /// Consumes the iterator and returns the median of all elements.
    ///
    /// The median is the middle value of a sorted sequence. For sequences with an odd number of
    /// elements, this is the single middle element. For sequences with an even number of elements,
    /// this is the average of the two middle elements. If the iterator is empty, this method
    /// returns `None`.
    ///
    /// Note that this method collects all elements into a vector and sorts them, so it requires
    /// O(n) memory and O(n log n) time.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use hc_utils::iter::Median;
    /// // Odd count: middle element
    /// let odd = [3.0, 1.0, 2.0];
    /// assert_eq!(odd.into_iter().median(), Some(2.0));
    ///
    /// // Even count: average of two middle elements
    /// let even = [4.0, 1.0, 3.0, 2.0];
    /// assert_eq!(even.into_iter().median(), Some(2.5));
    /// ```
    fn median(self) -> Option<f64>;
}

impl<I: Iterator<Item = f64>> Median for I {
    fn median(self) -> Option<f64> {
        let mut values: Vec<f64> = self.collect();
        if values.is_empty() {
            return None;
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let len = values.len();
        if len % 2 == 1 {
            Some(values[len / 2])
        } else {
            Some((values[len / 2 - 1] + values[len / 2]) / 2.0)
        }
    }
}
