use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use hpuc_utils::Store;

use crate::val_ref::ValRef;

use super::{Dialect, IR, State, ValId};

/// A map that associates values with value IDs.
///
/// Maintains the same active/inactive structure as the source IR, allowing
/// efficient mapping of values to analysis results or other metadata.
/// Only active values can store data, and the map tracks how many
/// entries are currently stored.
pub struct ValMap<T> {
    store: Store<ValId, State<Option<T>>>,
    n_stored: u16,
    n_inactive: u16,
}

impl<T> ValMap<T> {
    fn may_store(&self, k: &ValId) -> bool {
        k.0 < self.store.len() && self.store[k].is_active()
    }

    /// Creates an empty value map with the same structure as the IR.
    ///
    /// The resulting map preserves the active/inactive state of values
    /// from the source IR but contains no stored data.
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
            n_inactive: ir.raw_n_vals() - ir.n_vals(),
        }
    }

    /// Creates a value map filled with the specified value for all active values.
    ///
    /// Every active value in the source IR will be associated with a clone
    /// of `v`. Inactive values remain unmapped.
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
            n_inactive: ir.raw_n_vals() - ir.n_vals(),
        }
    }

    /// Creates a value map by selectively applying a function to active values.
    ///
    /// The function returns `None` for values that should not have entries
    /// in the resulting map. Only values for which the function returns
    /// `Some(value)` will be stored.
    pub fn new_partially_mapped<D: Dialect>(
        ir: &IR<D>,
        mut f: impl FnMut(ValRef<D>) -> Option<T>,
    ) -> Self {
        ValMap {
            store: ir
                .raw_walk_vals_linear()
                .map(|val| {
                    if val.is_active() {
                        State::Active(f(val))
                    } else {
                        State::Inactive(None)
                    }
                })
                .collect(),
            n_stored: ir.n_vals(),
            n_inactive: ir.raw_n_vals() - ir.n_vals(),
        }
    }

    /// Creates a value map by applying a function to all active values.
    ///
    /// Every active value will have an entry in the resulting map,
    /// as the function must return a value rather than an option.
    pub fn new_totally_mapped<D: Dialect>(ir: &IR<D>, mut f: impl FnMut(ValRef<D>) -> T) -> Self {
        ValMap {
            store: ir
                .raw_walk_vals_linear()
                .map(|val| {
                    if val.is_active() {
                        State::Active(Some(f(val)))
                    } else {
                        State::Inactive(None)
                    }
                })
                .collect(),
            n_stored: ir.n_vals(),
            n_inactive: ir.raw_n_vals() - ir.n_vals(),
        }
    }

    /// Returns the number of values that have stored data.
    pub fn len(&self) -> u16 {
        self.n_stored
    }

    /// Returns `true` if all active values have stored data.
    pub fn is_filled(&self) -> bool {
        self.n_stored + self.n_inactive == self.store.len()
    }

    /// Returns `true` if no values have stored data.
    pub fn is_empty(&self) -> bool {
        self.n_stored == 0
    }

    /// Returns `true` if the specified value has stored data.
    ///
    /// # Panics
    ///
    /// Panics if the value ID is out of bounds or refers to an inactive value.
    pub fn contains_key(&self, k: &ValId) -> bool {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().is_some()
    }

    /// Returns a reference to the data for the specified value.
    ///
    /// Returns `None` if no data is stored for the value.
    ///
    /// # Panics
    ///
    /// Panics if the value ID is out of bounds or refers to an inactive value.
    pub fn get(&self, k: &ValId) -> Option<&T> {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().as_ref()
    }

    /// Returns a mutable reference to the data for the specified value.
    ///
    /// Returns `None` if no data is stored for the value.
    ///
    /// # Panics
    ///
    /// Panics if the value ID is out of bounds or refers to an inactive value.
    pub fn get_mut(&mut self, k: &ValId) -> Option<&mut T> {
        assert!(self.may_store(k));
        self.store[k].as_mut_ref().unwrap_active().as_mut()
    }

    /// Stores data for the specified value.
    ///
    /// Returns the previous data if it existed, otherwise `None`.
    ///
    /// # Panics
    ///
    /// Panics if the value ID is out of bounds or refers to an inactive value.
    pub fn insert(&mut self, k: ValId, v: T) -> Option<T> {
        assert!(self.may_store(&k));
        let v = State::Active(Some(v));
        let out = std::mem::replace(&mut self.store[k], v).unwrap_active();
        if out.is_none() {
            self.n_stored += 1;
        }
        out
    }

    /// Removes and returns the data for the specified value.
    ///
    /// Returns `None` if no data was stored for the value.
    ///
    /// # Panics
    ///
    /// Panics if the value ID is out of bounds or refers to an inactive value.
    pub fn remove(&mut self, k: &ValId) -> Option<T> {
        assert!(self.may_store(&k));
        let v = State::Active(None);
        let out = std::mem::replace(&mut self.store[k], v).unwrap_active();
        if out.is_some() {
            self.n_stored -= 1;
        }
        out
    }

    /// Returns an iterator over value IDs and their stored data.
    pub fn iter(&self) -> impl Iterator<Item = (ValId, &T)> {
        self.store.enumerate_iter().filter_map(|(i, a)| match a {
            State::Active(Some(v)) => Some((i, v)),
            _ => None,
        })
    }

    /// Returns an iterator over value IDs and mutable references to their data.
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
        match self.get(&index) {
            Some(a) => a,
            None => panic!("Tried to get unmapped index {:?}", index),
        }
    }
}

impl<T> IndexMut<ValId> for ValMap<T> {
    fn index_mut(&mut self, index: ValId) -> &mut Self::Output {
        match self.get_mut(&index) {
            Some(a) => a,
            None => panic!("Tried to get unmapped index {:?}", index),
        }
    }
}

impl<T: Debug> Debug for ValMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
