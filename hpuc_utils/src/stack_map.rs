use crate::{StackVec, StackVecIntoIter};
use std::fmt::Debug;

/// A map backed by a stack-allocated vector for small key-value collections.
#[derive(Clone)]
pub struct StackMap<K: Eq, V>(pub(super) StackVec<(K, V)>);

impl<K: Eq, V> StackMap<K, V> {
    /// Creates an empty map.
    pub fn new() -> Self {
        StackMap(StackVec::new())
    }

    /// Returns the maximum number of key-value pairs the map can hold.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the key already exists, replaces its value and returns the old value.
    /// Otherwise, inserts the new pair and returns `None`.
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match self.0.as_slice().iter().position(|(a, _)| *a == k) {
            Some(pos) => {
                let mut output = v;
                std::mem::swap(&mut self.0[pos].1, &mut output);
                Some(output)
            }
            None => {
                self.0.push((k, v));
                None
            }
        }
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, k: &K) -> Option<&V> {
        self.0
            .as_slice()
            .iter()
            .find_map(|(a, v)| if a == k { Some(v) } else { None })
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.0
            .as_mut_slice()
            .iter_mut()
            .find_map(|(a, v)| if a == k { Some(v) } else { None })
    }

    /// Removes a key-value pair from the map and returns the value.
    ///
    /// Returns `None` if the key was not present in the map.
    pub fn remove(&mut self, k: &K) -> Option<V> {
        match self.0.as_slice().iter().position(|(a, _)| a == k) {
            Some(pos) => Some(self.0.remove(pos).1),
            None => None,
        }
    }

    /// Returns `true` if the map contains the specified key.
    pub fn contains_key(&self, k: &K) -> bool {
        self.get(k).is_some()
    }

    // Check if the map has reached its capacity.
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    pub fn iter(&self) -> StackMapIter<'_, K, V> {
        StackMapIter {
            inner: self.0.as_slice().iter(),
        }
    }

    pub fn iter_mut(&mut self) -> StackMapMutIter<'_, K, V> {
        StackMapMutIter {
            inner: self.0.as_mut_slice().iter_mut(),
        }
    }

    pub fn into_iter(self) -> StackMapIntoIter<K, V> {
        StackMapIntoIter {
            inner: self.0.into_iter(),
        }
    }
}

pub struct StackMapMutIter<'a, K: Eq, V> {
    inner: std::slice::IterMut<'a, (K, V)>,
}

impl<'a, K: Eq, V> Iterator for StackMapMutIter<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (&*k, v))
    }
}

pub struct StackMapIntoIter<K: Eq, V> {
    inner: StackVecIntoIter<'static, (K, V)>,
}

impl<K: Eq, V> Iterator for StackMapIntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
pub struct StackMapIter<'a, K: Eq, V> {
    inner: std::slice::Iter<'a, (K, V)>,
}

impl<'a, K: Eq, V> Iterator for StackMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, v))
    }
}

impl<K: Eq + Debug, V: Debug> Debug for StackMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .fold(&mut f.debug_map(), |acc, (k, v)| acc.entry(k, v))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let map: StackMap<i32, String> = StackMap::new();
        assert!(!map.contains_key(&1));
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = StackMap::new();

        // Insert new key-value pair
        assert_eq!(map.insert("key1", 42), None);
        assert_eq!(map.get(&"key1"), Some(&42));

        // Insert another key-value pair
        assert_eq!(map.insert("key2", 24), None);
        assert_eq!(map.get(&"key2"), Some(&24));

        // Update existing key
        assert_eq!(map.insert("key1", 100), Some(42));
        assert_eq!(map.get(&"key1"), Some(&100));
    }

    #[test]
    fn test_get_nonexistent() {
        let map: StackMap<i32, String> = StackMap::new();
        assert_eq!(map.get(&1), None);
    }

    #[test]
    fn test_get_mut() {
        let mut map = StackMap::new();
        map.insert("key", 42);

        // Modify value through mutable reference
        if let Some(value) = map.get_mut(&"key") {
            *value = 100;
        }

        assert_eq!(map.get(&"key"), Some(&100));

        // Try to get mutable reference to nonexistent key
        assert_eq!(map.get_mut(&"nonexistent"), None);
    }

    #[test]
    fn test_remove() {
        let mut map = StackMap::new();
        map.insert("key1", 42);
        map.insert("key2", 24);

        // Remove existing key
        assert_eq!(map.remove(&"key1"), Some(42));
        assert_eq!(map.get(&"key1"), None);
        assert_eq!(map.get(&"key2"), Some(&24));

        // Remove nonexistent key
        assert_eq!(map.remove(&"nonexistent"), None);

        // Remove remaining key
        assert_eq!(map.remove(&"key2"), Some(24));
        assert_eq!(map.get(&"key2"), None);
    }

    #[test]
    fn test_contains_key() {
        let mut map = StackMap::new();

        assert!(!map.contains_key(&"key"));

        map.insert("key", 42);
        assert!(map.contains_key(&"key"));

        map.remove(&"key");
        assert!(!map.contains_key(&"key"));
    }

    #[test]
    fn test_map_clone() {
        let mut map1 = StackMap::new();
        map1.insert(1u8, 42);
        map1.insert(2u8, 24);

        let map2 = map1.clone();

        assert_eq!(map2.get(&1u8), Some(&42));
        assert_eq!(map2.get(&2u8), Some(&24));

        // Ensure they are independent
        map1.insert(3u8, 100);
        assert_eq!(map1.get(&3u8), Some(&100));
        assert_eq!(map2.get(&3u8), None);
    }

    #[test]
    fn test_multiple_operations() {
        let mut map = StackMap::new();

        // Test sequence of operations
        map.insert(1u8, 1u8);
        map.insert(2, 2);
        map.insert(3, 3);

        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(map.contains_key(&3));

        map.remove(&2);
        assert!(!map.contains_key(&2));

        map.insert(1, 11); // Update existing
        assert_eq!(map.get(&1), Some(&11));

        map.insert(4, 4);
        assert_eq!(map.get(&4), Some(&4));
    }

    #[test]
        fn test_iter() {
            let mut map = StackMap::new();
            map.insert(1u8, 42u8);
            map.insert(2, 24);
            map.insert(3, 100);

            let mut collected: Vec<_> = map.iter().collect();
            collected.sort_by_key(|(k, _)| *k);

            assert_eq!(collected, vec![(&1, &42), (&2, &24), (&3, &100)]);
        }

        #[test]
        fn test_iter_mut() {
            let mut map = StackMap::new();
            map.insert(1u8, 42u8);
            map.insert(2, 24);
            map.insert(3, 100);

            // Modify values through mutable iterator
            for (_, value) in map.iter_mut() {
                *value *= 2;
            }

            assert_eq!(map.get(&1), Some(&84));
            assert_eq!(map.get(&2), Some(&48));
            assert_eq!(map.get(&3), Some(&200));
        }

        #[test]
        fn test_into_iter() {
            let mut map = StackMap::new();
            map.insert(1u8, 42u8);
            map.insert(2, 24);
            map.insert(3, 100);

            let mut collected: Vec<_> = map.into_iter().collect();
            collected.sort_by_key(|(k, _)| *k);

            assert_eq!(collected, vec![(1, 42), (2, 24), (3, 100)]);
        }
}
