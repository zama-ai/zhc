use std::fmt::Debug;
use std::ops::Index;

use zhc_utils::{ChangeGuard, Store};

use crate::OpRef;

use super::{Dialect, IR, OpId, State};

/// A map that associates values with operation IDs.
///
/// Maintains the same active/inactive structure as the source IR, allowing
/// efficient mapping of operations to analysis results or other metadata.
/// Only active operations can store values, and the map tracks how many
/// values are currently stored.
#[derive(Clone)]
pub struct OpMap<T> {
    store: Store<OpId, State<Option<T>>>,
    n_stored: u16,
    n_inactive: u16,
    changed: bool,
}

impl<T> OpMap<T> {
    fn may_store(&self, k: &OpId) -> bool {
        k.0 < self.store.len() && self.store[k].is_active()
    }

    /// Creates an empty operation map with the same structure as the IR.
    ///
    /// The resulting map preserves the active/inactive state of operations
    /// from the source IR but contains no stored values.
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
            changed: false,
        }
    }

    /// Creates an operation map filled with the specified value for all active operations.
    ///
    /// Every active operation in the source IR will be associated with a clone
    /// of `v`. Inactive operations remain unmapped.
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
            changed: false,
        }
    }

    /// Creates an operation map by selectively applying a function to active operations.
    ///
    /// The function returns `None` for operations that should not have entries
    /// in the resulting map. Only operations for which the function returns
    /// `Some(value)` will be stored.
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
            changed: false,
        }
    }

    /// Creates an operation map by applying a function to all active operations.
    ///
    /// Every active operation will have an entry in the resulting map,
    /// as the function must return a value rather than an option.
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
            changed: false,
        }
    }

    /// Returns the number of operations that have stored values.
    pub fn len(&self) -> u16 {
        self.n_stored
    }

    /// Returns `true` if all active operations have stored values.
    pub fn is_filled(&self) -> bool {
        self.n_stored + self.n_inactive == self.store.len()
    }

    /// Returns `true` if no operations have stored values.
    pub fn is_empty(&self) -> bool {
        self.n_stored == 0
    }

    /// Returns `true` if the specified operation has a stored value.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID is out of bounds or refers to an inactive operation.
    pub fn contains_key(&self, k: &OpId) -> bool {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().is_some()
    }

    /// Returns a reference to the value for the specified operation.
    ///
    /// Returns `None` if no value is stored for the operation.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID is out of bounds or refers to an inactive operation.
    pub fn get(&self, k: &OpId) -> Option<&T> {
        assert!(self.may_store(k));
        self.store[k].as_ref().unwrap_active().as_ref()
    }

    /// Returns a mutable guard for the data at the specified operation.
    ///
    /// Returns `None` if no data is stored for the operation. The guard
    /// automatically tracks changes when dropped.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID is out of bounds or refers to an inactive operation.
    pub fn get_mut(&mut self, k: &OpId) -> Option<ChangeGuard<'_, T>>
    where
        T: Clone + PartialEq,
    {
        assert!(self.may_store(k));
        if let Some(value) = self.store[k].as_mut_ref().unwrap_active().as_mut() {
            Some(ChangeGuard::new(value, &mut self.changed))
        } else {
            None
        }
    }

    /// Stores a value for the specified operation.
    ///
    /// Returns the previous value if one existed, otherwise `None`.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID is out of bounds or refers to an inactive operation.
    pub fn insert(&mut self, k: OpId, v: T) -> Option<T>
    where
        T: PartialEq,
    {
        assert!(self.may_store(&k));
        let v = State::Active(Some(v));
        if v == self.store[k] {
            self.changed = true;
        }
        let out = std::mem::replace(&mut self.store[k], v).unwrap_active();
        if out.is_none() {
            self.n_stored += 1;
        }
        out
    }

    /// Removes and returns the value for the specified operation.
    ///
    /// Returns `None` if no value was stored for the operation.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID is out of bounds or refers to an inactive operation.
    pub fn remove(&mut self, k: &OpId) -> Option<T> {
        assert!(self.may_store(&k));
        let v = State::Active(None);
        let out = std::mem::replace(&mut self.store[k], v).unwrap_active();
        if out.is_some() {
            self.n_stored -= 1;
        }
        self.changed = true;
        out
    }

    /// Returns an iterator over operation IDs and their stored values.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (OpId, &T)> {
        self.store.enumerate_iter().filter_map(|(i, a)| match a {
            State::Active(Some(v)) => Some((i, v)),
            _ => None,
        })
    }

    /// Consumes the map and returns an iterator over operation IDs and their stored values.
    pub fn into_iter(self) -> impl DoubleEndedIterator<Item = (OpId, T)> {
        self.store
            .enumerate_into_iter()
            .filter_map(|(i, a)| match a {
                State::Active(Some(v)) => Some((i, v)),
                _ => None,
            })
    }

    /// Acknowledge eventual changes
    pub fn ack_changes(&mut self) -> bool {
        std::mem::replace(&mut self.changed, false)
    }

    /// Transforms stored values by applying `f`, preserving map structure.
    ///
    /// Active slots with a value are mapped through `f`; empty active slots
    /// and inactive slots remain unchanged. Counters and change flag are
    /// carried over as-is.
    pub fn map<TN>(self, mut f: impl FnMut(T) -> TN) -> OpMap<TN> {
        let OpMap {
            store,
            n_stored,
            n_inactive,
            changed,
        } = self;
        let store = store
            .into_iter()
            .map(|a| match a {
                State::Active(o) => State::Active(o.map(&mut f)),
                State::Inactive(_) => State::Inactive(None),
            })
            .collect();
        OpMap {
            store,
            n_stored,
            n_inactive,
            changed,
        }
    }
}

impl<T> Index<OpId> for OpMap<T> {
    type Output = T;

    fn index(&self, index: OpId) -> &Self::Output {
        match self.get(&index) {
            Some(a) => a,
            None => panic!("Tried to get unmapped index {:?}", index),
        }
    }
}

impl<T: Debug> Debug for OpMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
