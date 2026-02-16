use std::fmt::Debug;

use crate::small::{VArray, VArrayIntoIter};

/// A stack-allocated set that maintains unique elements.
///
/// `StackSet` is built on top of `StackVec` and ensures that all elements
/// are unique by checking for duplicates before insertion. Elements must
/// implement `Eq` for equality comparison.
#[derive(Clone)]
pub struct StackSet<T: Eq, const N: usize = 10>(pub(super) VArray<T, N>);

impl<T: Eq> StackSet<T, 10> {
    /// Creates a new empty `StackSet`.
    pub fn new() -> Self {
        StackSet(VArray::new())
    }
}
impl<T: Eq, const N: usize> StackSet<T, N> {
    /// Creates a new empty `StackSet`.
    pub fn with_capacity() -> StackSet<T, N> {
        StackSet(VArray::new())
    }

    /// Returns the capacity of the `StackSet`.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Inserts a `value` into the set.
    ///
    /// Returns `true` if the value was successfully inserted, or `false` if
    /// the value already exists in the set. Duplicate values are not added.
    pub fn insert(&mut self, value: T) -> bool {
        if self.0.as_slice().contains(&value) {
            false
        } else {
            self.0.push(value);
            true
        }
    }

    /// Removes a `value` from the set.
    ///
    /// Returns `true` if the value was present and removed, or `false` if
    /// the value was not found in the set.
    pub fn remove(&mut self, value: &T) -> bool {
        match self.0.search(value) {
            Some(i) => {
                self.0.remove(i);
                true
            }
            None => false,
        }
    }

    /// Checks if the set contains the specified `value`.
    pub fn contains(&self, value: &T) -> bool {
        self.0.search(value).is_some()
    }

    /// Checks if the set has reached its maximum capacity.
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    /// Returns an iterator over the elements in the set.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the elements in the set.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }

    /// Consumes the set and returns an iterator over its elements.
    pub fn into_iter(self) -> VArrayIntoIter<T, N> {
        self.0.into_iter()
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Eq, const N: usize> std::iter::FromIterator<T> for StackSet<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = StackSet::with_capacity();
        set.extend(iter);
        set
    }
}

impl<T: Eq, const N: usize> std::iter::Extend<T> for StackSet<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
}

impl<T: Eq + Debug, const N: usize> Debug for StackSet<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .fold(&mut f.debug_set(), |acc, v| acc.entry(v))
            .finish()
    }
}

impl<T: Eq, const N: usize> PartialEq for StackSet<T, N> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().all(|item| other.contains(item))
    }
}

impl<T: Eq, const N: usize> Eq for StackSet<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestItem {
        id: u32,
    }

    impl TestItem {
        fn new(id: u32) -> Self {
            TestItem { id }
        }
    }

    #[test]
    fn test_new_empty() {
        let set: StackSet<i32> = StackSet::new();
        assert_eq!(set.0.len(), 0);
    }

    #[test]
    fn test_insert_unique() {
        let mut set = StackSet::new();
        assert!(set.insert(1));
        assert!(set.insert(2));
        assert!(set.insert(3));
        assert_eq!(set.0.len(), 3);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut set = StackSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1)); // Should return false for duplicate
        assert_eq!(set.0.len(), 1);
    }

    #[test]
    fn test_contains() {
        let mut set = StackSet::new();
        set.insert(42);
        assert!(set.contains(&42));
        assert!(!set.contains(&99));
    }

    #[test]
    fn test_remove_existing() {
        let mut set = StackSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(set.remove(&2));
        assert_eq!(set.0.len(), 2);
        assert!(!set.contains(&2));
        assert!(set.contains(&1));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut set = StackSet::new();
        set.insert(1);

        assert!(!set.remove(&99));
        assert_eq!(set.0.len(), 1);
    }

    #[test]
    fn test_is_full() {
        let mut set: StackSet<u8> = StackSet::new();

        // Fill to capacity based on StackVec's capacity for u8
        let capacity = set.0.capacity();
        for i in 0..capacity {
            assert!(!set.is_full());
            set.insert(i as u8);
        }
        assert!(set.is_full());
    }

    #[test]
    fn test_from_iterator_deduplication() {
        let vec = vec![1, 2, 3, 2, 4, 1, 5];
        let set: StackSet<i32> = vec.into_iter().collect();

        assert_eq!(set.0.len(), 5); // Should have deduplicated
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
    }

    #[test]
    fn test_extend() {
        let mut set = StackSet::new();
        set.insert(1);
        set.insert(2);

        let additional = vec![2, 3, 4, 3];
        set.extend(additional);

        assert_eq!(set.0.len(), 4); // 1, 2, 3, 4 (duplicates ignored)
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
    }

    #[test]
    fn test_custom_type() {
        let mut set = StackSet::new();
        let item1 = TestItem::new(1);
        let item2 = TestItem::new(2);
        let item1_dup = TestItem::new(1);

        assert!(set.insert(item1.clone()));
        assert!(set.insert(item2.clone()));
        assert!(!set.insert(item1_dup)); // Should be rejected as duplicate

        assert_eq!(set.0.len(), 2);
        assert!(set.contains(&item1));
        assert!(set.contains(&item2));
    }

    #[test]
    fn test_insert_remove_cycle() {
        let mut set = StackSet::new();

        // Insert, remove, insert again
        assert!(set.insert(42));
        assert!(set.remove(&42));
        assert!(set.insert(42)); // Should succeed again after removal

        assert_eq!(set.0.len(), 1);
        assert!(set.contains(&42));
    }

    #[test]
    fn test_large_type_capacity() {
        // Test with a larger type to verify capacity calculations
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct LargeType([u8; 64]);

        let set: StackSet<LargeType> = StackSet::new();
        let capacity = set.0.capacity();

        assert_eq!(capacity, 10);
    }

    #[test]
    fn test_empty_operations() {
        let mut set: StackSet<i32> = StackSet::new();

        assert!(!set.contains(&1));
        assert!(!set.remove(&1));
        assert!(!set.is_full());
    }

    #[test]
    fn test_single_element() {
        let mut set = StackSet::new();
        set.insert(99);

        assert!(set.contains(&99));
        assert_eq!(set.0.len(), 1);
        assert!(set.remove(&99));
        assert_eq!(set.0.len(), 0);
        assert!(!set.contains(&99));
    }
}
