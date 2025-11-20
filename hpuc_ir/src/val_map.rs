use std::ops::{Index, IndexMut};

use hpuc_utils::Store;

use super::{Dialect, IR, ValId, State};

/// A map that associates values with value IDs, preserving the active/inactive state structure from an IR.
pub struct ValMap<T> {
    store: Store<ValId, State<Option<T>>>,
    n_stored: u16,
    n_inactive: u16
}

impl<T> ValMap<T> {
    fn may_store(&self, k: &ValId) -> bool {
        k.0 < self.store.len() && self.store[k].is_active()
    }

    /// Creates a new `ValMap` from the given `ir`, copying its value state structure.
    ///
    /// The resulting map will have the same active and inactive values as the source IR,
    /// but all values will be initialized to `None`.
    pub fn new_empty<D: Dialect>(ir: &IR<D>) -> Self {
        ValMap {
            store: ir
                .val_states
                .iter()
                .map(|s| match s {
                    State::Active(_) => State::Active(None),
                    State::Inactive(_) => State::Inactive(None),
                })
                .collect(),
            n_stored: 0,
            n_inactive: ir.raw_n_vals() - ir.n_vals()
        }
    }

    /// Creates a new `ValMap` from the given `ir` with all active values initialized to `v`.
    ///
    /// The resulting map will have the same active and inactive values as the source IR.
    /// All active values will contain a clone of `v`, while inactive values remain unset.
    pub fn new_filled<D: Dialect>(ir: &IR<D>, v: T) -> Self
    where
        T: Clone,
    {
        ValMap {
            store: ir
                .val_states
                .iter()
                .map(|s| match s {
                    State::Active(_) => State::Active(Some(v.clone())),
                    State::Inactive(_) => State::Inactive(None),
                })
                .collect(),
            n_stored: ir.n_vals(),
            n_inactive: ir.raw_n_vals() - ir.n_vals()
        }
    }

    /// Returns the number of stored values in the map.
    pub fn len(&self) -> u16 {
        self.n_stored
    }

    /// Returns `true` if all possible values in the map have values stored.
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
    /// Panics if `k` is out of bounds or refers to an inactive value.
    pub fn contains_key(&self, k: &ValId) -> bool {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().is_some()
    }

    /// Returns a reference to the value corresponding to `k`.
    ///
    /// Returns `None` if no value is associated with `k`.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive value.
    pub fn get(&self, k: &ValId) -> Option<&T> {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().as_ref()
    }

    /// Returns a mutable reference to the value corresponding to `k`.
    ///
    /// Returns `None` if no value is associated with `k`.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive value.
    pub fn get_mut(&mut self, k: &ValId) -> Option<&mut T> {
        assert!(self.may_store(k));
        self.store[k].as_mut_ref().unwrap_active().as_mut()
    }

    /// Inserts the value `v` at `k`, returning the previous value if one existed.
    ///
    /// # Panics
    ///
    /// Panics if `k` is out of bounds or refers to an inactive value.
    pub fn insert(&mut self, k: ValId, v: T) -> Option<T> {
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
    /// Panics if `k` is out of bounds or refers to an inactive value.
    pub fn remove(&mut self, k: &ValId) -> Option<T> {
        assert!(self.may_store(&k));
        let v = State::Active(None);
        let out = std::mem::replace(&mut self.store[k], v).unwrap_active();
        if out.is_some() {
            self.n_stored -= 1;
        }
        out
    }

    /// Returns an iterator over the stored key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (ValId, &T)> {
        self.store.enumerate_iter().filter_map(|(i, a)| match a {
            State::Active(Some(v)) => Some((i, v)),
            _ => None,
        })
    }

    /// Returns an iterator over the stored key-value pairs with mutable references to values.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ValId, &mut T)> {
        self.store
            .enumerate_iter_mut()
            .filter_map(|(i, a)| match a {
                State::Active(Some(v)) => Some((i, v)),
                _ => None,
            })
    }
}

impl<T> Index<ValId> for ValMap<T> {
    type Output = T;

    fn index(&self, index: ValId) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<T> IndexMut<ValId> for ValMap<T> {
    fn index_mut(&mut self, index: ValId) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}
