use std::{
    iter::FromIterator,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

/// A vector-like container that uses typed indices for element access.
///
/// The `Store` provides safe, typed access to elements using custom index types
/// that implement `StoreIndex`. This prevents mixing up indices between different
/// stores and provides better type safety than raw `usize` indices.
#[derive(Clone)]
pub struct Store<I: StoreIndex, V>(Vec<V>, PhantomData<I>);

impl<I: StoreIndex, V> Store<I, V> {
    /// Creates an empty store.
    pub fn empty() -> Self {
        Store(Vec::new(), PhantomData)
    }

    /// Creates an empty store with the specified capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Store(Vec::with_capacity(cap), PhantomData)
    }

    /// Creates a store with `len` copies of `value`.
    pub fn with_value(value: V, len: usize) -> Self
    where
        V: Clone,
    {
        Store(vec![value; len], PhantomData)
    }

    /// Returns the number of elements in the store as the index type's raw representation.
    pub fn len(&self) -> I::Raw {
        I::raw_from_usize(self.0.len())
    }

    /// Appends an element to the store and returns its typed index.
    pub fn push(&mut self, val: V) -> I {
        let opid = I::from_usize(self.0.len());
        self.0.push(val);
        opid
    }

    /// Returns mutable references to multiple elements at different indices.
    ///
    /// # Panics
    ///
    /// Panics if any index is out of bounds or if any indices are duplicated.
    pub fn get_disjoint_mut<const N: usize>(&mut self, indices: [I; N]) -> [&mut V; N] {
        let usize_indices = indices.map(|a| a.as_usize());
        self.0.get_disjoint_mut(usize_indices).unwrap()
    }

    /// Returns an iterator over typed indices and element references.
    pub fn enumerate_iter(&self) -> impl Iterator<Item = (I, &V)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, v)| (I::from_usize(i), v))
    }

    /// Returns an iterator over typed indices and mutable element references.
    pub fn enumerate_iter_mut(&mut self) -> impl Iterator<Item = (I, &mut V)> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(i, v)| (I::from_usize(i), v))
    }

    /// Returns an iterator over element references.
    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.0.iter()
    }

    /// Consumes the store and returns an iterator over owned elements.
    pub fn into_iter(self) -> std::vec::IntoIter<V> {
        self.0.into_iter()
    }

    /// Returns an iterator over mutable element references.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, V> {
        self.0.iter_mut()
    }
}

impl<I: StoreIndex, V> Index<I> for Store<I, V> {
    type Output = V;
    fn index(&self, index: I) -> &Self::Output {
        &self.0[index.as_usize()]
    }
}
impl<I: StoreIndex, V> Index<&I> for Store<I, V> {
    type Output = V;
    fn index(&self, index: &I) -> &Self::Output {
        &self.0[index.as_usize()]
    }
}
impl<I: StoreIndex, V> IndexMut<I> for Store<I, V> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index.as_usize()]
    }
}
impl<I: StoreIndex, V> IndexMut<&I> for Store<I, V> {
    fn index_mut(&mut self, index: &I) -> &mut Self::Output {
        &mut self.0[index.as_usize()]
    }
}

impl<I: StoreIndex, V> FromIterator<V> for Store<I, V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Store(Vec::from_iter(iter), PhantomData)
    }
}

/// A trait for types that can be used as typed indices into a `Store`.
///
/// This trait provides conversion methods between the index type and raw numeric
/// representations, enabling type-safe indexing while maintaining compatibility
/// with underlying storage mechanisms.
pub trait StoreIndex: Copy {
    /// The raw numeric type used to represent this index internally.
    type Raw;

    /// Converts this index to its raw numeric representation.
    fn as_raw(&self) -> Self::Raw;

    /// Converts this index to a `usize` for array/vector indexing.
    fn as_usize(&self) -> usize;

    /// Creates a raw representation from a `usize`.
    fn raw_from_usize(val: usize) -> Self::Raw;

    /// Creates an index from a `usize`.
    fn from_usize(val: usize) -> Self;
}
