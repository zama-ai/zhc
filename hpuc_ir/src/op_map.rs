use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use hpuc_utils::Store;

use crate::OpRef;

use super::{Dialect, IR, OpId, State};

/// A map that associates values with operation IDs, preserving the active/inactive state structure
/// from an IR.
pub struct OpMap<T> {
    store: Store<OpId, State<Option<T>>>,
    n_stored: u16,
    n_inactive: u16,
}

impl<T> OpMap<T> {
    fn may_store(&self, k: &OpId) -> bool {
        k.0 < self.store.len() && self.store[k].is_active()
    }

    /// Creates a new `OpMap` from the given `ir`, copying its operation state structure.
    ///
    /// The resulting map will have the same active and inactive operations as the source IR,
    /// but all values will be initialized to `None`.
    pub fn new_empty<D: Dialect>(ir: &IR<D>) -> Self {
        OpMap {
            store: ir
                .op_states
                .iter()
                .map(|s| match s {
                    State::Active(_) => State::Active(None),
                    State::Inactive(_) => State::Inactive(None),
                })
                .collect(),
            n_stored: 0,
            n_inactive: ir.raw_n_ops() - ir.n_ops(),
        }
    }

    /// Creates a new `OpMap` from the given `ir` with all active operations initialized to `v`.
    ///
    /// The resulting map will have the same active and inactive operations as the source IR.
    /// All active operations will contain a clone of `v`, while inactive operations remain unset.
    pub fn new_filled<D: Dialect>(ir: &IR<D>, v: T) -> Self
    where
        T: Clone,
    {
        OpMap {
            store: ir
                .op_states
                .iter()
                .map(|s| match s {
                    State::Active(_) => State::Active(Some(v.clone())),
                    State::Inactive(_) => State::Inactive(None),
                })
                .collect(),
            n_stored: ir.n_ops(),
            n_inactive: ir.raw_n_ops() - ir.n_ops(),
        }
    }

    /// Creates a new `OpMap` from the given `ir` by applying `f` to each active operation.
    ///
    /// This method allows selective population of the map, where some active operations may
    /// not receive values based on the logic in `f`.
    pub fn new_partially_mapped<D: Dialect>(
        ir: &IR<D>,
        mut f: impl FnMut(OpRef<D>) -> Option<T>,
    ) -> Self {
        OpMap {
            store: ir
                .raw_walk_ops_linear()
                .map(|op| {
                    if op.is_active() {
                        State::Active(f(op))
                    } else {
                        State::Inactive(None)
                    }
                })
                .collect(),
            n_stored: ir.n_ops(),
            n_inactive: ir.raw_n_ops() - ir.n_ops(),
        }
    }

    /// Creates a new `OpMap` from the given `ir` by applying `f` to each active operation.
    ///
    /// Unlike `new_partially_mapped`, this method guarantees that all active operations will
    /// have values in the resulting map, as `f` must return a `T` value rather than an `Option<T>`.
    pub fn new_totally_mapped<D: Dialect>(ir: &IR<D>, mut f: impl FnMut(OpRef<D>) -> T) -> Self {
        OpMap {
            store: ir
                .raw_walk_ops_linear()
                .map(|op| {
                    if op.is_active() {
                        State::Active(Some(f(op)))
                    } else {
                        State::Inactive(None)
                    }
                })
                .collect(),
            n_stored: ir.n_ops(),
            n_inactive: ir.raw_n_ops() - ir.n_ops(),
        }
    }

    /// Returns the number of stored values in the map.
    pub fn len(&self) -> u16 {
        self.n_stored
    }

    /// Returns `true` if all possible operations in the map have values stored.
    pub fn is_filled(&self) -> bool {
        self.n_stored + self.n_inactive == self.store.len()
    }

    /// Returns `true` if the map contains no stored values.
    pub fn is_empty(&self) -> bool {
        self.n_stored == 0
    }

    /// Returns `true` if the map contains a value for the specified `k`.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive operation.
    pub fn contains_key(&self, k: &OpId) -> bool {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().is_some()
    }

    /// Returns a reference to the value corresponding to `k`.
    ///
    /// Returns `None` if no value is associated with `k`.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive operation.
    pub fn get(&self, k: &OpId) -> Option<&T> {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().as_ref()
    }

    /// Returns a mutable reference to the value corresponding to `k`.
    ///
    /// Returns `None` if no value is associated with `k`.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive operation.
    pub fn get_mut(&mut self, k: &OpId) -> Option<&mut T> {
        assert!(self.may_store(k));
        self.store[k].as_mut_ref().unwrap_active().as_mut()
    }

    /// Inserts the value `v` at `k`, returning the previous value if one existed.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive operation.
    pub fn insert(&mut self, k: OpId, v: T) -> Option<T> {
        assert!(self.may_store(&k));
        let v = State::Active(Some(v));
        let out = std::mem::replace(&mut self.store[k], v).unwrap_active();
        if out.is_none() {
            self.n_stored += 1;
        }
        out
    }

    /// Removes and returns the value at `k` if one exists.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive operation.
    pub fn remove(&mut self, k: &OpId) -> Option<T> {
        assert!(self.may_store(&k));
        let v = State::Active(None);
        let out = std::mem::replace(&mut self.store[k], v).unwrap_active();
        if out.is_some() {
            self.n_stored -= 1;
        }
        out
    }

    /// Returns an iterator over the stored key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (OpId, &T)> {
        self.store.enumerate_iter().filter_map(|(i, a)| match a {
            State::Active(Some(v)) => Some((i, v)),
            _ => None,
        })
    }

    /// Returns an iterator over the stored key-value pairs with mutable references to values.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (OpId, &mut T)> {
        self.store
            .enumerate_iter_mut()
            .filter_map(|(i, a)| match a {
                State::Active(Some(v)) => Some((i, v)),
                _ => None,
            })
    }
}

impl<T> Index<OpId> for OpMap<T> {
    type Output = T;

    fn index(&self, index: OpId) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<T> IndexMut<OpId> for OpMap<T> {
    fn index_mut(&mut self, index: OpId) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}

impl<T: Debug> Debug for OpMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
