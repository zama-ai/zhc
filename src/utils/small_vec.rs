use super::StackVec;
use crate::utils::StackVecIntoIter;
use std::usize;

pub enum SmallVecIntoIter<A> {
    Heap(std::vec::IntoIter<A>),
    Stack(StackVecIntoIter<A>),
}

impl<A> Iterator for SmallVecIntoIter<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SmallVecIntoIter::Heap(h) => h.next(),
            SmallVecIntoIter::Stack(s) => s.next(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SmallVec<A> {
    Heap(Vec<A>),
    Stack(StackVec<A>),
}

impl<A> SmallVec<A> {
    pub fn as_slice(&self) -> &[A] {
        match self {
            SmallVec::Heap(h) => h.as_slice(),
            SmallVec::Stack(s) => s.as_slice(),
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [A] {
        match self {
            SmallVec::Heap(h) => h.as_mut_slice(),
            SmallVec::Stack(s) => s.as_mut_slice(),
        }
    }

    pub fn push(&mut self, value: A) {
        if let SmallVec::Stack(stack_vec) = self {
            if !stack_vec.may_push() {
                let mut output = Vec::with_capacity(stack_vec.len() + 1);
                stack_vec.drain_to_vec(&mut output);
                *self = SmallVec::Heap(output);
            }
        }
        match self {
            SmallVec::Heap(h) => h.push(value),
            SmallVec::Stack(s) => s.push(value),
        }
    }

    pub fn append(&mut self, other: &mut SmallVec<A>) {
        if let SmallVec::Stack(l) = self
            && let SmallVec::Stack(r) = other
            && l.may_append(r.len())
        {
            // Happy path.
            l.append(r);
            return;
        }

        if let SmallVec::Stack(l) = self {
            // Must grow, and hence need an alloc.
            let mut output = Vec::with_capacity(l.len());
            l.drain_to_vec(&mut output);
            *self = SmallVec::Heap(output);
        }

        let SmallVec::Heap(l) = self else {
            unreachable!()
        };

        match other {
            SmallVec::Heap(r) => {
                l.append(r);
                return;
            }
            SmallVec::Stack(r) => {
                r.drain_to_vec(l);
            }
        }
    }

    pub fn iter(&self) -> std::slice::Iter<A> {
        match self {
            SmallVec::Heap(h) => h.iter(),
            SmallVec::Stack(s) => s.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<A> {
        match self {
            SmallVec::Heap(h) => h.iter_mut(),
            SmallVec::Stack(s) => s.iter_mut(),
        }
    }

    pub fn into_iter(self) -> SmallVecIntoIter<A> {
        match self {
            SmallVec::Heap(h) => SmallVecIntoIter::Heap(h.into_iter()),
            SmallVec::Stack(s) => SmallVecIntoIter::Stack(s.into_iter()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SmallVec::Heap(h) => h.len(),
            SmallVec::Stack(s) => s.len(),
        }
    }
}

impl<A> std::ops::Index<usize> for SmallVec<A> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            SmallVec::Heap(h) => h.index(index),
            SmallVec::Stack(s) => s.index(index),
        }
    }
}

impl<A> std::ops::IndexMut<usize> for SmallVec<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            SmallVec::Heap(h) => h.index_mut(index),
            SmallVec::Stack(s) => s.index_mut(index),
        }
    }
}

impl<A> std::iter::FromIterator<A> for SmallVec<A> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let iter = iter.into_iter();
        if let (_, Some(max)) = iter.size_hint()
            && max <= StackVec::<A>::static_capacity()
        {
            SmallVec::Stack(StackVec::from_iter(iter))
        } else {
            SmallVec::Heap(Vec::from_iter(iter))
        }
    }
}

impl<A> std::iter::Extend<A> for SmallVec<A> {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (min, maybe_max) = iter.size_hint();
        let max = maybe_max.unwrap_or(usize::MAX);
        if let SmallVec::Stack(stack_vec) = self
            && stack_vec.may_append(max)
        {
            stack_vec.extend(iter);
        } else {
            if let SmallVec::Stack(stack_vec) = self {
                let mut output = Vec::with_capacity(stack_vec.len() + min);
                stack_vec.drain_to_vec(&mut output);
                *self = SmallVec::Heap(output);
            }
            let SmallVec::Heap(vec) = self else {
                unreachable!()
            };
            vec.extend(iter);
        }
    }
}

impl<A: PartialEq> PartialEq for SmallVec<A> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        } else {
            self.as_slice().eq(other.as_slice())
        }
    }
}

impl<A: Eq> Eq for SmallVec<A> {}

#[macro_export]
macro_rules! svec {
    () => {
        $crate::utils::SmallVec::from_iter(std::iter::empty())
    };
    ($elem:expr; $n:expr) => {
        $crate::utils::SmallVec::from_iter(std::iter::repeat($elem).take($n))
    };
    ($($x:expr),+ $(,)?) => {
        $crate::utils::SmallVec::from_iter([$($x),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Drop tracker for memory safety tests
    #[derive(Debug, Clone)]
    struct DropTracker {
        id: usize,
        counter: Arc<AtomicUsize>,
    }

    impl DropTracker {
        fn new(id: usize, counter: Arc<AtomicUsize>) -> Self {
            counter.fetch_add(1, Ordering::Relaxed);
            DropTracker { id, counter }
        }
    }

    impl Drop for DropTracker {
        fn drop(&mut self) {
            self.counter.fetch_sub(1, Ordering::Relaxed);
        }
    }

    impl PartialEq for DropTracker {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for DropTracker {}

    #[test]
    fn test_empty_creation() {
        let vec: SmallVec<u32> = SmallVec::from_iter(std::iter::empty());
        assert!(matches!(vec, SmallVec::Stack(_)));
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.as_slice(), &[]);
    }

    #[test]
    fn test_small_creation_stays_stack() {
        let vec: SmallVec<u32> = SmallVec::from_iter(1..=5);
        assert!(matches!(vec, SmallVec::Stack(_)));
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_large_creation_uses_heap() {
        let large_iter = 1..=100;
        let vec: SmallVec<u32> = SmallVec::from_iter(large_iter);
        assert!(matches!(vec, SmallVec::Heap(_)));
        assert_eq!(vec.len(), 100);
    }

    #[test]
    fn test_push_stack_to_heap_transition() {
        let mut vec: SmallVec<u64> = SmallVec::from_iter(std::iter::empty());
        assert!(matches!(vec, SmallVec::Stack(_)));

        // Fill to capacity (8 u64s in 64 bytes)
        for i in 0..8 {
            vec.push(i);
            assert!(matches!(vec, SmallVec::Stack(_)));
        }
        assert_eq!(vec.len(), 8);

        // This push should trigger transition to heap
        vec.push(8);
        assert!(matches!(vec, SmallVec::Heap(_)));
        assert_eq!(vec.len(), 9);
        assert_eq!(vec.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_push_preserves_data_across_transition() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut vec: SmallVec<DropTracker> = SmallVec::from_iter(std::iter::empty());

        // Fill with drop-tracked items
        for i in 0..4 {
            vec.push(DropTracker::new(i, counter.clone()));
        }
        assert_eq!(counter.load(Ordering::Relaxed), 4);
        assert!(matches!(vec, SmallVec::Stack(_)));

        // Trigger transition
        vec.push(DropTracker::new(4, counter.clone()));
        assert_eq!(counter.load(Ordering::Relaxed), 5);
        assert!(matches!(vec, SmallVec::Heap(_)));

        // Verify data integrity
        for (i, item) in vec.iter().enumerate() {
            assert_eq!(item.id, i);
        }

        drop(vec);
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_append_stack_stack_fits() {
        let mut vec1: SmallVec<u32> = SmallVec::from_iter(1..=3);
        let mut vec2: SmallVec<u32> = SmallVec::from_iter(4..=6);

        assert!(matches!(vec1, SmallVec::Stack(_)));
        assert!(matches!(vec2, SmallVec::Stack(_)));

        vec1.append(&mut vec2);

        assert!(matches!(vec1, SmallVec::Stack(_)));
        assert_eq!(vec1.len(), 6);
        assert_eq!(vec2.len(), 0);
        assert_eq!(vec1.as_slice(), &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_append_stack_stack_overflow() {
        let mut vec1: SmallVec<u64> = SmallVec::from_iter(0..5);
        let mut vec2: SmallVec<u64> = SmallVec::from_iter(5..10);

        assert!(matches!(vec1, SmallVec::Stack(_)));
        assert!(matches!(vec2, SmallVec::Stack(_)));

        vec1.append(&mut vec2);

        assert!(matches!(vec1, SmallVec::Heap(_)));
        assert_eq!(vec1.len(), 10);
        assert_eq!(vec2.len(), 0);
        assert_eq!(vec1.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_append_heap_stack() {
        let mut vec1: SmallVec<u32> = SmallVec::from_iter(1..=100);
        let mut vec2: SmallVec<u32> = SmallVec::from_iter(101..=105);

        assert!(matches!(vec1, SmallVec::Heap(_)));
        assert!(matches!(vec2, SmallVec::Stack(_)));

        let original_len = vec1.len();
        vec1.append(&mut vec2);

        assert!(matches!(vec1, SmallVec::Heap(_)));
        assert_eq!(vec1.len(), original_len + 5);
        assert_eq!(vec2.len(), 0);
    }

    #[test]
    fn test_append_stack_heap() {
        let mut vec1: SmallVec<u32> = SmallVec::from_iter(1..=5);
        let mut vec2: SmallVec<u32> = SmallVec::from_iter(6..=100);

        assert!(matches!(vec1, SmallVec::Stack(_)));
        assert!(matches!(vec2, SmallVec::Heap(_)));

        vec1.append(&mut vec2);

        assert!(matches!(vec1, SmallVec::Heap(_)));
        assert_eq!(vec1.len(), 100);
        assert_eq!(vec2.len(), 0);
    }

    #[test]
    fn test_append_heap_heap() {
        let mut vec1: SmallVec<u32> = SmallVec::from_iter(1..=50);
        let mut vec2: SmallVec<u32> = SmallVec::from_iter(51..=100);

        assert!(matches!(vec1, SmallVec::Heap(_)));
        assert!(matches!(vec2, SmallVec::Heap(_)));

        vec1.append(&mut vec2);

        assert!(matches!(vec1, SmallVec::Heap(_)));
        assert_eq!(vec1.len(), 100);
        assert_eq!(vec2.len(), 0);
    }

    #[test]
    fn test_extend_within_stack_capacity() {
        let mut vec: SmallVec<u32> = SmallVec::from_iter(1..=3);
        assert!(matches!(vec, SmallVec::Stack(_)));

        vec.extend(4..=6);
        assert!(matches!(vec, SmallVec::Stack(_)));
        assert_eq!(vec.as_slice(), &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_extend_forces_heap_transition() {
        let mut vec: SmallVec<u64> = SmallVec::from_iter(0..5);
        assert!(matches!(vec, SmallVec::Stack(_)));

        vec.extend(5..15);
        assert!(matches!(vec, SmallVec::Heap(_)));
        assert_eq!(vec.len(), 15);
    }

    #[test]
    fn test_extend_unknown_size_hint() {
        let mut vec: SmallVec<u32> = SmallVec::from_iter(1..=3);
        assert!(matches!(vec, SmallVec::Stack(_)));

        // Iterator with no upper bound in size hint
        let iter = (4..).take_while(|a| *a <= 13);
        vec.extend(iter);

        // Should transition to heap due to unknown size
        assert!(matches!(vec, SmallVec::Heap(_)));
        assert_eq!(vec.len(), 13);
    }

    #[test]
    fn test_indexing() {
        let vec: SmallVec<u32> = SmallVec::from_iter(10..15);

        assert_eq!(vec[0], 10);
        assert_eq!(vec[4], 14);
    }

    #[test]
    fn test_mutable_indexing() {
        let mut vec: SmallVec<u32> = SmallVec::from_iter(1..=5);

        vec[2] = 99;
        assert_eq!(vec[2], 99);
        assert_eq!(vec.as_slice(), &[1, 2, 99, 4, 5]);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let vec: SmallVec<u32> = SmallVec::from_iter(1..=3);
        let _ = vec[5];
    }

    #[test]
    fn test_iterators_stack() {
        let vec: SmallVec<u32> = SmallVec::from_iter(1..=5);
        assert!(matches!(vec, SmallVec::Stack(_)));

        let collected: Vec<_> = vec.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iterators_heap() {
        let vec: SmallVec<u32> = SmallVec::from_iter(1..=100);
        assert!(matches!(vec, SmallVec::Heap(_)));

        let sum: u32 = vec.iter().sum();
        assert_eq!(sum, 5050); // sum of 1..=100
    }

    #[test]
    fn test_mutable_iterator() {
        let mut vec: SmallVec<u32> = SmallVec::from_iter(1..=5);

        for item in vec.iter_mut() {
            *item *= 2;
        }

        assert_eq!(vec.as_slice(), &[2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_into_iter_stack() {
        let counter = Arc::new(AtomicUsize::new(0));
        let vec: SmallVec<DropTracker> =
            SmallVec::from_iter((0..3).map(|i| DropTracker::new(i, counter.clone())));
        assert_eq!(counter.load(Ordering::Relaxed), 3);

        let collected: Vec<_> = vec.into_iter().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(counter.load(Ordering::Relaxed), 3);

        drop(collected);
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_into_iter_heap() {
        let counter = Arc::new(AtomicUsize::new(0));
        let vec: SmallVec<DropTracker> =
            SmallVec::from_iter((0..50).map(|i| DropTracker::new(i, counter.clone())));
        assert_eq!(counter.load(Ordering::Relaxed), 50);
        assert!(matches!(vec, SmallVec::Heap(_)));

        let collected: Vec<_> = vec.into_iter().collect();
        assert_eq!(collected.len(), 50);
        assert_eq!(counter.load(Ordering::Relaxed), 50);

        drop(collected);
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_equality() {
        let vec1: SmallVec<u32> = SmallVec::from_iter(1..=5);
        let vec2: SmallVec<u32> = SmallVec::from_iter(1..=5);
        let vec3: SmallVec<u32> = SmallVec::from_iter(1..=100);
        let vec4: SmallVec<u32> = SmallVec::from_iter(2..=6);

        assert_eq!(vec1, vec2);
        assert_ne!(vec1, vec3);
        assert_ne!(vec1, vec4);

        // Cross-variant equality (stack vs heap with same content)
        let small: SmallVec<u32> = SmallVec::from_iter(1..=5);
        let mut large = SmallVec::from_iter(1..=5);
        // Force to heap by extending beyond capacity
        large.extend(6..20);
        large.as_mut_slice()[5..].fill(0); // Reset extra elements
        // Manually adjust length - this is just for testing
        match &mut large {
            SmallVec::Heap(v) => v.truncate(5),
            _ => unreachable!(),
        }
        assert_eq!(small, large);
    }

    #[test]
    fn test_clone() {
        let original: SmallVec<u32> = SmallVec::from_iter(1..=10);
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Ensure independence
        let mut mutable_clone = cloned.clone();
        mutable_clone.push(999);
        assert_ne!(original, mutable_clone);
    }

    #[test]
    fn test_svec_macro_empty() {
        let vec: SmallVec<u32> = svec![];
        assert_eq!(vec.len(), 0);
        assert!(matches!(vec, SmallVec::Stack(_)));
    }

    #[test]
    fn test_svec_macro_repeat() {
        let vec: SmallVec<u32> = svec![42; 5];
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.as_slice(), &[42, 42, 42, 42, 42]);
        assert!(matches!(vec, SmallVec::Stack(_)));
    }

    #[test]
    fn test_svec_macro_list() {
        let vec: SmallVec<u32> = svec![1, 2, 3, 4, 5];
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.as_slice(), &[1, 2, 3, 4, 5]);
        assert!(matches!(vec, SmallVec::Stack(_)));
    }

    #[test]
    fn test_svec_macro_large_repeat() {
        let vec: SmallVec<u32> = svec![1; 100];
        assert_eq!(vec.len(), 100);
        assert!(matches!(vec, SmallVec::Heap(_)));
        assert!(vec.iter().all(|&x| x == 1));
    }
}
