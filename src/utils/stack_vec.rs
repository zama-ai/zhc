use std::fmt::Debug;
use std::mem::{ManuallyDrop, MaybeUninit};

const STACK_BYTES: usize = 64;

#[repr(C)]
union AlignedStorage<A> {
    data: [MaybeUninit<u8>; STACK_BYTES],
    // The following form is never used, but it forces the alignment of the whole type to be
    // correct.
    _align: ManuallyDrop<[A; 0]>,
}

pub struct StackVec<A> {
    data: AlignedStorage<A>,
    len: usize,
}

pub struct StackVecIntoIter<A> {
    data: AlignedStorage<A>,
    start: usize,
    end: usize,
}

impl<A> Iterator for StackVecIntoIter<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            unsafe {
                let ptr = self.data.data.as_ptr() as *const A;
                let value = std::ptr::read(ptr.add(self.start));
                self.start += 1;
                Some(value)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.start;
        (remaining, Some(remaining))
    }
}

impl<A> ExactSizeIterator for StackVecIntoIter<A> {}

impl<A> Drop for StackVecIntoIter<A> {
    fn drop(&mut self) {
        // Drop any remaining elements
        while self.start < self.end {
            unsafe {
                let ptr = self.data.data.as_ptr() as *const A;
                std::ptr::drop_in_place(ptr.add(self.start) as *mut A);
                self.start += 1;
            }
        }
    }
}

impl<A> StackVec<A> {
    pub fn new() -> Self {
        let size = std::mem::size_of::<A>();
        let align = std::mem::align_of::<A>();
        if size > STACK_BYTES {
            panic!("Type is too big to fit in a stack vec.");
        }
        if size == 0 {
            panic!("ZSTs are not supported.");
        }
        if align > size {
            panic!("Types with alignment larger than their size are not supported.");
        }

        StackVec {
            data: AlignedStorage {
                data: [MaybeUninit::uninit(); STACK_BYTES],
            },
            len: 0,
        }
    }

    pub fn as_slice(&self) -> &[A] {
        unsafe { std::slice::from_raw_parts(self.data.data.as_ptr() as *const A, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [A] {
        unsafe { std::slice::from_raw_parts_mut(self.data.data.as_mut_ptr() as *mut A, self.len) }
    }

    pub fn push(&mut self, value: A) {
        if self.may_push() {
            unsafe {
                let ptr = self.data.data.as_mut_ptr() as *mut A;
                std::ptr::write(ptr.add(self.len), value);
                self.len += 1;
            }
        } else {
            panic!("StackVec overflow");
        }
    }

    pub const fn static_capacity() -> usize {
        let size = std::mem::size_of::<A>();
        let align = std::mem::align_of::<A>();
        if size > STACK_BYTES {
            panic!("Type is too big to fit in a stack vec.");
        }
        if size == 0 {
            panic!("ZSTs are not supported.");
        }
        if align > size {
            panic!("Types with alignment larger than their size are not supported.");
        }
        STACK_BYTES / size
    }

    pub fn capacity(&self) -> usize {
        STACK_BYTES / std::mem::size_of::<A>()
    }

    pub fn may_push(&self) -> bool {
        self.len < self.capacity()
    }

    pub fn may_append(&self, other_len: usize) -> bool {
        self.len.saturating_add(other_len) <= self.capacity()
    }

    pub fn append(&mut self, other: &mut StackVec<A>) {
        if !self.may_append(other.len) {
            panic!("StackVec overflow");
        }

        unsafe {
            let self_ptr = self.data.data.as_mut_ptr() as *mut A;
            let other_ptr = other.data.data.as_ptr() as *const A;

            for i in 0..other.len {
                std::ptr::write(self_ptr.add(self.len + i), std::ptr::read(other_ptr.add(i)));
            }

            self.len += other.len;
            other.len = 0;
        }
    }

    pub fn drain_to_vec(&mut self, other: &mut Vec<A>) {
        other.reserve(self.len);
        let ptr = unsafe { self.data.data.as_mut_ptr() as *const A };
        for i in 0..self.len {
            other.push(unsafe { std::ptr::read(ptr.add(i)) });
        }
        self.len = 0;
    }

    pub fn into_vec(mut self) -> Vec<A> {
        let mut vec = Vec::with_capacity(self.len);
        self.drain_to_vec(&mut vec);
        vec
    }

    pub fn iter(&self) -> std::slice::Iter<A> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<A> {
        self.as_mut_slice().iter_mut()
    }

    pub fn into_iter(self) -> StackVecIntoIter<A> {
        let len = self.len;
        let data = unsafe { std::ptr::read(&self.data) };
        std::mem::forget(self);
        StackVecIntoIter {
            data,
            start: 0,
            end: len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<A> Drop for StackVec<A> {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.data.data.as_mut_ptr() as *mut A;
            for i in 0..self.len {
                std::ptr::drop_in_place(ptr.add(i));
            }
        }
    }
}

impl<A> std::ops::Index<usize> for StackVec<A> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        }
        unsafe {
            let ptr = self.data.data.as_ptr() as *const A;
            &*ptr.add(index)
        }
    }
}

impl<A> std::ops::IndexMut<usize> for StackVec<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        }
        unsafe {
            let ptr = self.data.data.as_mut_ptr() as *mut A;
            &mut *ptr.add(index)
        }
    }
}

impl<A: Debug> Debug for StackVec<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter()
            .fold(&mut f.debug_list(), |acc, v| acc.entry(v))
            .finish()
    }
}

impl<A: PartialEq> PartialEq for StackVec<A> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(l, h)| l == h)
    }
}

impl<A> std::iter::FromIterator<A> for StackVec<A> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut output = StackVec::new();
        output.extend(iter);
        return output;
    }
}

impl<A> std::iter::Extend<A> for StackVec<A> {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        while let Some(v) = iter.next() {
            self.push(v);
        }
    }
}

impl<A: Clone> Clone for StackVec<A> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Drop-tracking type for ownership and memory safety tests
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

    #[repr(C)]
    struct Large([u64; 10]); // 80 bytes - exceeds STACK_BYTES

    #[test]
    fn test_basic_operations() {
        let mut vec = StackVec::new();
        assert_eq!(vec.len(), 0);
        assert!(vec.may_push());

        vec.push(42u32);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 42);

        vec.push(84);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.as_slice(), &[42, 84]);
    }

    #[test]
    fn test_capacity_different_sizes() {
        let small_vec: StackVec<u8> = StackVec::new();
        assert_eq!(small_vec.capacity(), 64);

        let medium_vec: StackVec<u32> = StackVec::new();
        assert_eq!(medium_vec.capacity(), 16);

        let large_vec: StackVec<u64> = StackVec::new();
        assert_eq!(large_vec.capacity(), 8);
    }

    #[test]
    #[should_panic(expected = "ZSTs are not supported")]
    fn test_zst_panic() {
        let _: StackVec<()> = StackVec::new();
    }

    #[test]
    fn test_ownership_and_drops() {
        let counter = Arc::new(AtomicUsize::new(0));

        {
            let mut vec = StackVec::new();
            vec.push(DropTracker::new(1, counter.clone()));
            vec.push(DropTracker::new(2, counter.clone()));
            assert_eq!(counter.load(Ordering::Relaxed), 2);
        } // vec drops here

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_into_iter_ownership() {
        let counter = Arc::new(AtomicUsize::new(0));

        {
            let mut vec = StackVec::new();
            vec.push(DropTracker::new(1, counter.clone()));
            vec.push(DropTracker::new(2, counter.clone()));
            assert_eq!(counter.load(Ordering::Relaxed), 2);

            let mut iter = vec.into_iter();
            let first = iter.next().unwrap();
            assert_eq!(first.id, 1);
            assert_eq!(counter.load(Ordering::Relaxed), 2);

            drop(first);
            assert_eq!(counter.load(Ordering::Relaxed), 1);

            // Iterator drops remaining elements
        }

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_into_iter_partial_consumption() {
        let counter = Arc::new(AtomicUsize::new(0));

        {
            let mut vec = StackVec::new();
            for i in 0..4 {
                vec.push(DropTracker::new(i, counter.clone()));
            }
            assert_eq!(counter.load(Ordering::Relaxed), 4);

            let mut iter = vec.into_iter();
            let _ = iter.next(); // consume first element
            let _ = iter.next(); // consume second element
            // Drop iterator with remaining elements
        }

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_append_ownership() {
        let counter = Arc::new(AtomicUsize::new(0));

        let mut vec1 = StackVec::new();
        let mut vec2 = StackVec::new();

        vec1.push(DropTracker::new(1, counter.clone()));
        vec2.push(DropTracker::new(2, counter.clone()));
        vec2.push(DropTracker::new(3, counter.clone()));

        assert_eq!(counter.load(Ordering::Relaxed), 3);
        assert_eq!(vec2.len(), 2);

        vec1.append(&mut vec2);

        assert_eq!(vec1.len(), 3);
        assert_eq!(vec2.len(), 0);
        assert_eq!(counter.load(Ordering::Relaxed), 3);

        drop(vec1);
        drop(vec2);
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_drain_to_vec() {
        let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

        let mut stack_vec = StackVec::new();
        stack_vec.push(DropTracker::new(1, counter.clone()));
        stack_vec.push(DropTracker::new(2, counter.clone()));

        let mut heap_vec = Vec::new();
        stack_vec.drain_to_vec(&mut heap_vec);

        assert_eq!(stack_vec.len(), 0);
        assert_eq!(heap_vec.len(), 2);
        assert_eq!(counter.load(Ordering::Relaxed), 2);

        drop(heap_vec);
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_clone() {
        let mut vec = StackVec::new();
        vec.push(42u32);
        vec.push(84);

        let cloned = vec.clone();
        assert_eq!(vec.as_slice(), cloned.as_slice());

        // Ensure they're independent
        vec.as_mut_slice()[0] = 999;
        assert_ne!(vec[0], cloned[0]);
    }

    #[test]
    fn test_iterators() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        vec.push(3);

        let collected: Vec<_> = vec.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);

        for item in vec.iter_mut() {
            *item *= 2;
        }
        assert_eq!(vec.as_slice(), &[2, 4, 6]);

        let into_collected: Vec<_> = vec.into_iter().collect();
        assert_eq!(into_collected, vec![2, 4, 6]);
    }

    #[test]
    fn test_from_iterator() {
        let vec: StackVec<u32> = (1..=5).collect();
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_extend() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.extend(vec![2, 3, 4]);
        assert_eq!(vec.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    #[should_panic(expected = "StackVec overflow")]
    fn test_overflow_panic() {
        let mut vec: StackVec<u64> = StackVec::new();
        for i in 0..=vec.capacity() {
            vec.push(i as u64);
        }
    }

    #[test]
    #[should_panic(expected = "StackVec overflow")]
    fn test_append_overflow() {
        let mut vec1: StackVec<u64> = StackVec::new();
        let mut vec2: StackVec<u64> = StackVec::new();

        // Fill both to near capacity
        for i in 0..vec1.capacity() {
            vec1.push(i as u64);
        }
        vec2.push(999);

        vec1.append(&mut vec2); // Should panic
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_panic() {
        let vec: StackVec<u32> = StackVec::new();
        let _ = vec[0];
    }

    #[test]
    fn test_debug_format() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        let debug_str = format!("{:?}", vec);
        assert_eq!(debug_str, "[1, 2]");
    }

    #[test]
    fn test_partial_eq() {
        let mut vec1 = StackVec::new();
        let mut vec2 = StackVec::new();

        vec1.push(1u32);
        vec1.push(2);
        vec2.push(1u32);
        vec2.push(2);

        assert_eq!(vec1, vec2);

        vec2.push(3);
        assert_ne!(vec1, vec2);
    }

    #[test]
    fn test_size_hint_iterator() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        vec.push(3);

        let iter = vec.into_iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
    }

    #[test]
    #[should_panic(expected = "Type is too big to fit in a stack vec.")]
    fn test_large_type_capacity() {
        let _vec: StackVec<Large> = StackVec::new();
    }
}
