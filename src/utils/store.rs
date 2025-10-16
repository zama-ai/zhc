use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub struct Store<I: StoreIndex, V>(Vec<V>, PhantomData<I>);

impl<I: StoreIndex, V> Store<I, V> {
    pub fn empty() -> Self {
        Store(Vec::new(), PhantomData)
    }

    pub fn len(&self) -> I::Raw {
        I::raw_from_usize(self.0.len())
    }

    pub fn push(&mut self, val: V) -> I {
        let opid = I::from_usize(self.0.len());
        self.0.push(val);
        opid
    }

    pub fn get_disjoint_mut<const N: usize>(&mut self, indices: [I; N]) -> [&mut V; N] {
        let usize_indices = indices.map(|a| a.as_usize());
        self.0.get_disjoint_mut(usize_indices).unwrap()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.0.iter()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<V> {
        self.0.into_iter()
    }

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

pub trait StoreIndex {
    type Raw;
    fn as_raw(&self) -> Self::Raw;
    fn as_usize(&self) -> usize;
    fn raw_from_usize(val: usize) -> Self::Raw;
    fn from_usize(val: usize) -> Self;
}
