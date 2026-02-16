use std::hash::Hash;

use crate::{
    FastSet,
    small::{VArrayIntoIter, stack_set::StackSet},
};

/// A set optimized for small collections with automatic storage strategy.
///
/// Starts with stack-based storage for better performance with few elements,
/// then transitions to heap-based storage when capacity is exceeded.
#[derive(Clone, Debug)]
pub enum SmallSet<T: Eq + Hash, const N: usize = 10> {
    Heap(FastSet<T>),
    Stack(StackSet<T, N>),
}

/// Iterator over references to elements in a SmallSet.
pub enum SmallSetIter<'a, T> {
    Heap(std::collections::hash_set::Iter<'a, T>),
    Stack(std::slice::Iter<'a, T>),
}

impl<'a, T> Iterator for SmallSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SmallSetIter::Heap(iter) => iter.next(),
            SmallSetIter::Stack(iter) => iter.next(),
        }
    }
}

/// Iterator that takes ownership of elements in a SmallSet.
pub enum SmallSetIntoIter<T, const N: usize> {
    Heap(std::collections::hash_set::IntoIter<T>),
    Stack(VArrayIntoIter<T, N>),
}

impl<T, const N: usize> Iterator for SmallSetIntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SmallSetIntoIter::Heap(iter) => iter.next(),
            SmallSetIntoIter::Stack(iter) => iter.next(),
        }
    }
}

impl<T: Eq + Hash> SmallSet<T> {
    /// Creates a new empty set.
    pub fn new() -> Self {
        SmallSet::Stack(StackSet::with_capacity())
    }
}

impl<T: Eq + Hash, const N: usize> SmallSet<T, N> {
    /// Creates a new empty set.
    pub fn with_capacity() -> Self {
        SmallSet::Stack(StackSet::with_capacity())
    }

    /// Adds a value to the set.
    ///
    /// Returns `true` if the `value` was newly inserted, or `false` if
    /// the value was already present. When the stack storage becomes full,
    /// the set automatically transitions to heap storage to accommodate
    /// the new element.
    pub fn insert(&mut self, value: T) -> bool {
        match self {
            SmallSet::Stack(stack_set) => {
                if !stack_set.is_full() {
                    stack_set.insert(value)
                } else if stack_set.contains(&value) {
                    false
                } else {
                    *self = SmallSet::Heap(
                        stack_set
                            .0
                            .drain_all()
                            .chain(std::iter::once(value))
                            .collect(),
                    );
                    true
                }
            }
            SmallSet::Heap(fast_set) => fast_set.insert(value),
        }
    }

    /// Removes a value from the set.
    ///
    /// Returns `true` if the `value` was present and removed, or `false`
    /// if the value was not found in the set.
    pub fn remove(&mut self, value: &T) -> bool {
        match self {
            SmallSet::Stack(stack_set) => stack_set.remove(value),
            SmallSet::Heap(fast_set) => fast_set.remove(value),
        }
    }

    /// Checks if the set contains a value.
    ///
    /// Returns `true` if the set contains the specified `value`, `false`
    /// otherwise.
    pub fn contains(&self, value: &T) -> bool {
        match self {
            SmallSet::Stack(stack_set) => stack_set.contains(value),
            SmallSet::Heap(fast_set) => fast_set.contains(value),
        }
    }

    /// Returns an iterator over references to the elements.
    pub fn iter(&self) -> SmallSetIter<'_, T> {
        match self {
            SmallSet::Heap(h) => SmallSetIter::Heap(h.iter()),
            SmallSet::Stack(s) => SmallSetIter::Stack(s.iter()),
        }
    }

    /// Returns an iterator that takes ownership of the elements.
    pub fn into_iter(self) -> SmallSetIntoIter<T, N> {
        match self {
            SmallSet::Heap(h) => SmallSetIntoIter::Heap(h.into_iter()),
            SmallSet::Stack(s) => SmallSetIntoIter::Stack(s.into_iter()),
        }
    }
}

impl<T: Eq + Hash> std::iter::FromIterator<T> for SmallSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = SmallSet::new();
        set.extend(iter);
        set
    }
}

impl<T: Eq + Hash> std::iter::Extend<T> for SmallSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
}

impl<T: Eq + Hash, const N: usize> IntoIterator for SmallSet<T, N> {
    type Item = T;
    type IntoIter = SmallSetIntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a SmallSet<T> {
    type Item = &'a T;
    type IntoIter = SmallSetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Eq + Hash, const N: usize> PartialEq for SmallSet<T, N> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SmallSet::Stack(s1), SmallSet::Stack(s2)) => s1 == s2,
            (SmallSet::Heap(h1), SmallSet::Heap(h2)) => h1 == h2,
            (SmallSet::Stack(s), SmallSet::Heap(h)) | (SmallSet::Heap(h), SmallSet::Stack(s)) => {
                if s.len() != h.len() {
                    return false;
                }
                s.iter().all(|item| h.contains(item))
            }
        }
    }
}

impl<T: Eq + Hash, const N: usize> Eq for SmallSet<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let set: SmallSet<i32> = SmallSet::new();
        match set {
            SmallSet::Stack(_) => (),
            SmallSet::Heap(_) => panic!("Expected Stack variant"),
        }
    }

    #[test]
    fn test_insert_stack_phase() {
        let mut set = SmallSet::new();
        assert!(set.insert(1));
        assert!(set.insert(2));
        assert!(set.insert(3));

        // Should still be in stack phase
        match set {
            SmallSet::Stack(_) => (),
            SmallSet::Heap(_) => panic!("Expected to remain in Stack phase"),
        }
    }

    #[test]
    fn test_insert_duplicate_stack() {
        let mut set = SmallSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1)); // Should return false for duplicate

        match set {
            SmallSet::Stack(_) => (),
            SmallSet::Heap(_) => panic!("Expected to remain in Stack phase"),
        }
    }

    #[test]
    fn test_transition_to_heap() {
        let mut set: SmallSet<u8> = SmallSet::new();

        // Fill stack to capacity
        let mut capacity = 0;
        if let SmallSet::Stack(ref stack_set) = set {
            capacity = stack_set.0.capacity();
        }

        for i in 0..capacity {
            assert!(set.insert(i as u8));
        }

        // Should still be stack
        match set {
            SmallSet::Stack(_) => (),
            SmallSet::Heap(_) => panic!("Expected to remain in Stack phase when full"),
        }

        // This should trigger transition to heap
        assert!(set.insert(capacity as u8));

        match set {
            SmallSet::Heap(_) => (),
            SmallSet::Stack(_) => panic!("Expected transition to Heap phase"),
        }
    }

    #[test]
    fn test_insert_duplicate_when_full() {
        let mut set: SmallSet<u8> = SmallSet::new();

        // Fill to capacity
        let mut capacity = 0;
        if let SmallSet::Stack(ref stack_set) = set {
            capacity = stack_set.0.capacity();
        }

        for i in 0..capacity {
            set.insert(i as u8);
        }

        // Try to insert duplicate - should return false and not transition
        assert!(!set.insert(0));

        match set {
            SmallSet::Stack(_) => (),
            SmallSet::Heap(_) => panic!("Should not transition on duplicate insert"),
        }
    }

    #[test]
    fn test_contains_stack() {
        let mut set = SmallSet::new();
        set.insert(42);
        assert!(set.contains(&42));
        assert!(!set.contains(&99));
    }

    #[test]
    fn test_contains_heap() {
        let mut set: SmallSet<u8> = SmallSet::new();

        // Force transition to heap
        let SmallSet::Stack(ref stack_set) = set else {
            unreachable!()
        };
        let capacity = stack_set.capacity();

        for i in 0..=capacity {
            set.insert(i as u8);
        }

        assert!(matches!(set, SmallSet::Heap(_)));

        assert!(set.contains(&0));
        assert!(set.contains(&(capacity as u8)));
        assert!(!set.contains(&99));
    }

    #[test]
    fn test_remove_stack() {
        let mut set = SmallSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(set.remove(&2));
        assert!(!set.contains(&2));
        assert!(set.contains(&1));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_remove_heap() {
        let mut set: SmallSet<u8> = SmallSet::new();

        // Force transition to heap
        let mut capacity = 0;
        if let SmallSet::Stack(ref stack_set) = set {
            capacity = stack_set.0.capacity();
        }

        for i in 0..=capacity {
            set.insert(i as u8);
        }

        assert!(set.remove(&1));
        assert!(!set.contains(&1));
        assert!(set.contains(&0));
        assert!(set.contains(&(capacity as u8)));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut set = SmallSet::new();
        set.insert(1);
        assert!(!set.remove(&99));

        // Test in heap phase too
        let mut heap_set: SmallSet<u8> = SmallSet::new();
        let mut capacity = 0;
        if let SmallSet::Stack(ref stack_set) = heap_set {
            capacity = stack_set.0.capacity();
        }

        for i in 0..=capacity {
            heap_set.insert(i as u8);
        }

        assert!(!heap_set.remove(&99));
    }

    #[test]
    fn test_from_iterator_small() {
        let vec = vec![1, 2, 3];
        let set: SmallSet<i32> = vec.into_iter().collect();

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));

        // Should still be in stack phase for small collections
        match set {
            SmallSet::Stack(_) => (),
            SmallSet::Heap(_) => (), // Could be either depending on capacity
        }
    }

    #[test]
    fn test_from_iterator_with_duplicates() {
        let vec = vec![1, 2, 3, 2, 4, 1, 5];
        let set: SmallSet<i32> = vec.into_iter().collect();

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
    }

    #[test]
    fn test_extend() {
        let mut set = SmallSet::new();
        set.insert(1);
        set.insert(2);

        let additional = vec![2, 3, 4, 3];
        set.extend(additional);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
    }

    #[test]
    fn test_custom_type() {
        let mut set = SmallSet::new();
        let item1 = TestItem::new(1);
        let item2 = TestItem::new(2);
        let item1_dup = TestItem::new(1);

        assert!(set.insert(item1.clone()));
        assert!(set.insert(item2.clone()));
        assert!(!set.insert(item1_dup)); // Should be rejected as duplicate

        assert!(set.contains(&item1));
        assert!(set.contains(&item2));
    }

    #[test]
    fn test_insert_remove_cycle() {
        let mut set = SmallSet::new();

        // Test in stack phase
        assert!(set.insert(42));
        assert!(set.remove(&42));
        assert!(set.insert(42)); // Should succeed again after removal

        assert!(set.contains(&42));
    }

    #[test]
    fn test_heap_operations() {
        let mut set: SmallSet<u8> = SmallSet::new();

        // Force transition to heap by filling beyond stack capacity
        let mut capacity = 0;
        if let SmallSet::Stack(ref stack_set) = set {
            capacity = stack_set.0.capacity();
        }

        for i in 0..=capacity {
            set.insert(i as u8);
        }

        // Verify we're in heap mode
        match set {
            SmallSet::Heap(_) => (),
            SmallSet::Stack(_) => panic!("Expected to be in Heap phase"),
        }

        // Test heap operations
        assert!(set.insert((capacity + 1) as u8));
        assert!(!set.insert(0)); // Duplicate
        assert!(set.remove(&1));
        assert!(!set.contains(&1));
        assert!(set.contains(&0));
    }

    #[test]
    fn test_empty_operations() {
        let mut set: SmallSet<i32> = SmallSet::new();

        assert!(!set.contains(&1));
        assert!(!set.remove(&1));
    }

    #[test]
    fn test_large_type() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct LargeType([u8; 64]);

        let mut set: SmallSet<LargeType> = SmallSet::new();
        let large_val1 = LargeType([1; 64]);
        let large_val2 = LargeType([2; 64]);

        assert!(set.insert(large_val1.clone()));
        assert!(set.insert(large_val2.clone()));
        assert!(set.contains(&large_val1));
        assert!(set.contains(&large_val2));
    }
}
