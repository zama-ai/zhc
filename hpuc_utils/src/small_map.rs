use std::hash::Hash;

use crate::{FastMap, StackMap};

/// A map that starts stack-allocated and spills to heap when capacity is exceeded.
#[derive(Clone, Debug)]
pub enum SmallMap<K: Eq, V> {
    Heap(FastMap<K, V>),
    Stack(StackMap<K, V>),
}

impl<K: Eq, V> SmallMap<K, V> {
    /// Creates an empty stack-allocated map.
    pub fn new() -> Self {
        SmallMap::Stack(StackMap::new())
    }

    /// Returns the current capacity of the map.
    ///
    /// For stack-allocated maps, this is the fixed stack capacity.
    /// For heap-allocated maps, this delegates to the underlying `HashMap`.
    pub fn capacity(&self) -> usize {
        match self {
            SmallMap::Heap(m) => m.capacity(),
            SmallMap::Stack(m) => m.capacity(),
        }
    }
}

impl<K: Eq + Hash, V> SmallMap<K, V> {
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, k: &K) -> Option<&V> {
        match self {
            SmallMap::Heap(m) => m.get(k),
            SmallMap::Stack(m) => m.get(k),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        match self {
            SmallMap::Heap(m) => m.get_mut(k),
            SmallMap::Stack(m) => m.get_mut(k),
        }
    }

    /// Returns `true` if the map contains the specified key.
    pub fn contains_key(&self, k: &K) -> bool {
        match self {
            SmallMap::Heap(m) => m.contains_key(k),
            SmallMap::Stack(m) => m.contains_key(k),
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the key already exists, replaces its value and returns the old value.
    /// If the stack map is full and a new key is inserted, the map spills to heap.
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match self {
            SmallMap::Heap(m) => m.insert(k, v),
            SmallMap::Stack(m) => {
                if m.contains_key(&k) {
                    return m.insert(k, v);
                } else if !m.is_full() {
                    m.insert(k, v)
                } else {
                    *self = SmallMap::Heap(
                        m
                            .0
                            .drain_all()
                            .chain(std::iter::once((k, v)))
                            .collect(),
                    );
                    None
                }
            }
        }
    }

    /// Removes a key-value pair from the map and returns the value.
    pub fn remove(&mut self, k: &K) -> Option<V> {
        match self {
            SmallMap::Heap(m) => m.remove(k),
            SmallMap::Stack(m) => m.remove(k),
        }
    }

}

impl<K: Eq, V> Default for SmallMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for SmallMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = SmallMap::new();
        map.extend(iter);
        map
    }
}

impl<K: Eq + Hash, V> Extend<(K, V)> for SmallMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_stack() {
        let map: SmallMap<i32, i32> = SmallMap::new();
        assert!(matches!(map, SmallMap::Stack(_)));
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = SmallMap::new();
        assert_eq!(map.insert("a", 1), None);
        assert_eq!(map.insert("b", 2), None);
        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), None);
    }

    #[test]
    fn test_insert_update() {
        let mut map = SmallMap::new();
        assert_eq!(map.insert("a", 1), None);
        assert_eq!(map.insert("a", 2), Some(1));
        assert_eq!(map.get(&"a"), Some(&2));
    }

    #[test]
    fn test_get_mut() {
        let mut map = SmallMap::new();
        map.insert("a", 1);
        if let Some(v) = map.get_mut(&"a") {
            *v = 42;
        }
        assert_eq!(map.get(&"a"), Some(&42));
    }

    #[test]
    fn test_remove() {
        let mut map = SmallMap::new();
        map.insert("a", 1);
        assert_eq!(map.remove(&"a"), Some(1));
        assert_eq!(map.remove(&"a"), None);
        assert_eq!(map.get(&"a"), None);
    }

    #[test]
    fn test_contains_key() {
        let mut map = SmallMap::new();
        assert!(!map.contains_key(&"a"));
        map.insert("a", 1);
        assert!(map.contains_key(&"a"));
    }

    #[test]
    fn test_spill_to_heap() {
        let mut map: SmallMap<u64, u64> = SmallMap::new();
        let capacity = map.capacity();

        // Fill to capacity
        for i in 0..capacity as u64 {
            map.insert(i, i * 10);
        }
        assert!(matches!(map, SmallMap::Stack(_)));

        // One more should spill
        map.insert(capacity as u64, 999);
        assert!(matches!(map, SmallMap::Heap(_)));

        // Verify all data is preserved
        for i in 0..capacity as u64 {
            assert_eq!(map.get(&i), Some(&(i * 10)));
        }
        assert_eq!(map.get(&(capacity as u64)), Some(&999));
    }

    #[test]
    fn test_update_at_capacity_no_spill() {
        let mut map: SmallMap<u64, u64> = SmallMap::new();
        let capacity = map.capacity();

        // Fill to capacity
        for i in 0..capacity as u64 {
            map.insert(i, i);
        }
        assert!(matches!(map, SmallMap::Stack(_)));

        // Update existing key should not spill
        map.insert(0, 100);
        assert!(matches!(map, SmallMap::Stack(_)));
        assert_eq!(map.get(&0), Some(&100));
    }

    #[test]
    fn test_from_iterator() {
        let map: SmallMap<i32, i32> = vec![(1, 10), (2, 20), (3, 30)].into_iter().collect();
        assert_eq!(map.get(&1), Some(&10));
        assert_eq!(map.get(&2), Some(&20));
        assert_eq!(map.get(&3), Some(&30));
    }

    #[test]
    fn test_extend() {
        let mut map = SmallMap::new();
        map.insert("a", 1);
        map.extend(vec![("b", 2), ("c", 3)]);
        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), Some(&3));
    }

    #[test]
    fn test_default() {
        let map: SmallMap<i32, i32> = SmallMap::default();
        assert!(matches!(map, SmallMap::Stack(_)));
    }
}
