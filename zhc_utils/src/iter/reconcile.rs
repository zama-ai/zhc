//! Iterator type reconciliation for conditional branching.
//!
//! When a function returns an iterator whose concrete type depends on a runtime condition, Rust
//! requires all branches to produce the same type. This module provides sum types that unify
//! multiple iterator types sharing the same [`Item`](Iterator::Item), along with extension traits
//! to wrap iterators into the appropriate variant.
//!
//! The pattern scales to any number of branches: [`Reconciled2`] handles two-way branching,
//! [`Reconciled3`] handles three-way branching, and additional arities can be added as needed.
//!
//! # Examples
//!
//! Returning different iterator types from conditional branches:
//!
//! ```rust,no_run
//! # use std::ops::Range;
//! # use std::iter::Rev;
//! # use zhc_utils::iter::{Reconciled2, ReconcilerOf2};
//! fn numbers(ascending: bool) -> Reconciled2<Range<i32>, Rev<Range<i32>>> {
//!     if ascending {
//!         (0..10).reconcile_1_of_2()
//!     } else {
//!         (0..10).rev().reconcile_2_of_2()
//!     }
//! }
//!
//! // Both branches yield the same Item type, so callers see a unified iterator:
//! for n in numbers(true) {
//!     println!("{n}");
//! }
//! ```
//!
//! Three-way branching with [`Reconciled3`]:
//!
//! ```rust,no_run
//! # use std::iter::{once, Empty};
//! # use zhc_utils::iter::{Reconciled3, ReconcilerOf3};
//! enum Mode { None, Single, Many }
//!
//! fn values(mode: Mode) -> Reconciled3<Empty<u8>, std::iter::Once<u8>, std::vec::IntoIter<u8>> {
//!     match mode {
//!         Mode::None => std::iter::empty().reconcile_1_of_3(),
//!         Mode::Single => once(42).reconcile_2_of_3(),
//!         Mode::Many => vec![1, 2, 3].into_iter().reconcile_3_of_3(),
//!     }
//! }
//! ```

/// A sum type unifying two iterator types with the same item type.
///
/// This enum wraps one of two possible iterator types, delegating iteration to whichever variant
/// is active. Use the [`ReconcilerOf2`] extension trait to construct instances from any iterator.
///
/// The type parameters `I1` and `I2` correspond to the first and second branches respectively.
/// When calling [`reconcile_1_of_2`](ReconcilerOf2::reconcile_1_of_2), the iterator becomes the
/// `I1` slot; when calling [`reconcile_2_of_2`](ReconcilerOf2::reconcile_2_of_2), it becomes `I2`.
pub enum Reconciled2<I1, I2>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
{
    /// The first branch iterator.
    Iter1(I1),
    /// The second branch iterator.
    Iter2(I2),
}

impl<I1, I2> Iterator for Reconciled2<I1, I2>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Reconciled2::Iter1(i) => i.next(),
            Reconciled2::Iter2(i) => i.next(),
        }
    }
}

/// Extension trait for wrapping an iterator into a two-branch [`Reconciled2`].
///
/// This trait is blanket-implemented for all iterators. The type parameter `I1` represents the
/// *other* iterator type in the reconciled sum — the one not being wrapped by the current call.
/// Rust infers this from the function's return type annotation.
pub trait ReconcilerOf2<I1>
where
    Self: Iterator + Sized,
    I1: Iterator<Item = Self::Item>,
{
    /// Wraps this iterator as the first branch of a two-branch reconciliation.
    ///
    /// The resulting [`Reconciled2`] places this iterator in the `I1` position (first type
    /// parameter). The `I2` slot remains available for the other branch.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::ops::Range;
    /// # use zhc_utils::iter::{Reconciled2, ReconcilerOf2};
    /// let iter: Reconciled2<Range<i32>, std::vec::IntoIter<i32>> = (0..5).reconcile_1_of_2();
    /// ```
    fn reconcile_1_of_2(self) -> Reconciled2<Self, I1>;

    /// Wraps this iterator as the second branch of a two-branch reconciliation.
    ///
    /// The resulting [`Reconciled2`] places this iterator in the `I2` position (second type
    /// parameter). The `I1` slot remains available for the other branch.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::ops::Range;
    /// # use zhc_utils::iter::{Reconciled2, ReconcilerOf2};
    /// let iter: Reconciled2<Range<i32>, std::vec::IntoIter<i32>> = vec![1, 2].into_iter().reconcile_2_of_2();
    /// ```
    fn reconcile_2_of_2(self) -> Reconciled2<I1, Self>;
}

impl<T, I1> ReconcilerOf2<I1> for T
where
    T: Iterator,
    I1: Iterator<Item = T::Item>,
{
    fn reconcile_1_of_2(self) -> Reconciled2<Self, I1> {
        Reconciled2::Iter1(self)
    }

    fn reconcile_2_of_2(self) -> Reconciled2<I1, Self> {
        Reconciled2::Iter2(self)
    }
}

/// A sum type unifying three iterator types with the same item type.
///
/// This enum wraps one of three possible iterator types, delegating iteration to whichever variant
/// is active. Use the [`ReconcilerOf3`] extension trait to construct instances from any iterator.
///
/// The type parameters `I1`, `I2`, and `I3` correspond to the first, second, and third branches
/// respectively. The position in the type parameter list matches the numeric suffix of the
/// reconcile method used to construct it.
pub enum Reconciled3<I1, I2, I3>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I3: Iterator<Item = I1::Item>,
{
    /// The first branch iterator.
    Iter1(I1),
    /// The second branch iterator.
    Iter2(I2),
    /// The third branch iterator.
    Iter3(I3),
}

impl<I1, I2, I3> Iterator for Reconciled3<I1, I2, I3>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I3: Iterator<Item = I1::Item>,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Reconciled3::Iter1(i) => i.next(),
            Reconciled3::Iter2(i) => i.next(),
            Reconciled3::Iter3(i) => i.next(),
        }
    }
}

/// Extension trait for wrapping an iterator into a three-branch [`Reconciled3`].
///
/// This trait is blanket-implemented for all iterators. The type parameters `I1` and `I2`
/// represent the *other* iterator types in the reconciled sum — those not being wrapped by the
/// current call. Rust infers these from the function's return type annotation.
pub trait ReconcilerOf3<I1, I2>
where
    Self: Iterator + Sized,
    I1: Iterator<Item = Self::Item>,
    I2: Iterator<Item = Self::Item>,
{
    /// Wraps this iterator as the first branch of a three-branch reconciliation.
    ///
    /// The resulting [`Reconciled3`] places this iterator in the `I1` position (first type
    /// parameter). The `I2` and `I3` slots remain available for the other branches.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::ops::Range;
    /// # use std::iter::Empty;
    /// # use zhc_utils::iter::{Reconciled3, ReconcilerOf3};
    /// let iter: Reconciled3<Range<i32>, Empty<i32>, std::vec::IntoIter<i32>> =
    ///     (0..5).reconcile_1_of_3();
    /// ```
    fn reconcile_1_of_3(self) -> Reconciled3<Self, I1, I2>;

    /// Wraps this iterator as the second branch of a three-branch reconciliation.
    ///
    /// The resulting [`Reconciled3`] places this iterator in the `I2` position (second type
    /// parameter). The `I1` and `I3` slots remain available for the other branches.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::ops::Range;
    /// # use std::iter::Empty;
    /// # use zhc_utils::iter::{Reconciled3, ReconcilerOf3};
    /// let iter: Reconciled3<Range<i32>, Empty<i32>, std::vec::IntoIter<i32>> =
    ///     std::iter::empty().reconcile_2_of_3();
    /// ```
    fn reconcile_2_of_3(self) -> Reconciled3<I1, Self, I2>;

    /// Wraps this iterator as the third branch of a three-branch reconciliation.
    ///
    /// The resulting [`Reconciled3`] places this iterator in the `I3` position (third type
    /// parameter). The `I1` and `I2` slots remain available for the other branches.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::ops::Range;
    /// # use std::iter::Empty;
    /// # use zhc_utils::iter::{Reconciled3, ReconcilerOf3};
    /// let iter: Reconciled3<Range<i32>, Empty<i32>, std::vec::IntoIter<i32>> =
    ///     vec![1, 2, 3].into_iter().reconcile_3_of_3();
    /// ```
    fn reconcile_3_of_3(self) -> Reconciled3<I1, I2, Self>;
}

impl<T, I1, I2> ReconcilerOf3<I1, I2> for T
where
    T: Iterator,
    I1: Iterator<Item = T::Item>,
    I2: Iterator<Item = T::Item>,
{
    fn reconcile_1_of_3(self) -> Reconciled3<Self, I1, I2> {
        Reconciled3::Iter1(self)
    }

    fn reconcile_2_of_3(self) -> Reconciled3<I1, Self, I2> {
        Reconciled3::Iter2(self)
    }

    fn reconcile_3_of_3(self) -> Reconciled3<I1, I2, Self> {
        Reconciled3::Iter3(self)
    }
}
