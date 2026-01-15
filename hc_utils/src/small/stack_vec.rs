use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};

pub const STACK_BYTES: usize = 64;

#[repr(C)]
union AlignedStorage<A> {
    data: [MaybeUninit<u8>; STACK_BYTES],
    // The following form is never used, but it forces the alignment of the whole type to be
    // correct.
    _align: ManuallyDrop<[A; 0]>,
}

/// A stack-allocated vector with a fixed memory footprint.
///
/// `StackVec` stores elements in a fixed-size byte-buffer allocated on the stack.
/// It provides vector-like functionality without heap allocation. Since the memory
/// footprint is statically fixed, its capacity varies depending on the size of the
/// objects to store.
pub struct StackVec<A> {
    data: AlignedStorage<A>,
    len: usize,
}

/// An iterator that moves elements out of a `StackVec`.
///
/// This iterator is created by calling `drain_all` or `into_iter` on a `StackVec`.
/// It yields owned elements and properly handles cleanup of any remaining
/// elements when dropped.
pub struct StackVecIntoIter<'a, A> {
    data: AlignedStorage<A>,
    start: usize,
    end: usize,
    lifetime: PhantomData<&'a u8>,
}

impl<'a, A> Iterator for StackVecIntoIter<'a, A> {
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

impl<'a, A> DoubleEndedIterator for StackVecIntoIter<'a, A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            self.end -= 1;
            unsafe {
                let ptr = self.data.data.as_ptr() as *const A;
                let value = std::ptr::read(ptr.add(self.end));
                Some(value)
            }
        }
    }
}

impl<'a, A> ExactSizeIterator for StackVecIntoIter<'a, A> {}

impl<'a, A> Drop for StackVecIntoIter<'a, A> {
    fn drop(&mut self) {
        while self.start < self.end {
            unsafe {
                let ptr = self.data.data.as_ptr() as *const A;
                std::ptr::drop_in_place(ptr.add(self.start) as *mut A);
                self.start += 1;
            }
        }
    }
}

impl<A> Default for StackVec<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> StackVec<A> {
    /// Creates a new empty `StackVec`.
    ///
    /// The vector will have a capacity determined by the size of type `A`
    /// and the available stack buffer space.
    ///
    /// # Panics
    ///
    /// Panics if the type `A` is too large to fit in the stack buffer,
    /// if `A` is a zero-sized type, or if `A` has alignment requirements
    /// larger than its size.
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

    /// Returns a slice containing all elements in the vector.
    pub fn as_slice(&self) -> &[A] {
        unsafe { std::slice::from_raw_parts(self.data.data.as_ptr() as *const A, self.len) }
    }

    /// Returns a mutable slice containing all elements in the vector.
    pub fn as_mut_slice(&mut self) -> &mut [A] {
        unsafe { std::slice::from_raw_parts_mut(self.data.data.as_mut_ptr() as *mut A, self.len) }
    }

    /// Appends an element to the end of the vector.
    ///
    /// The `value` is moved into the vector and stored at the current length position.
    ///
    /// # Panics
    ///
    /// Panics if the vector is at capacity and cannot accommodate another element.
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

    /// Removes and returns the last element from the vector.
    ///
    /// Returns `None` if the vector is empty, otherwise returns `Some(element)`
    /// containing the removed element.
    pub fn pop(&mut self) -> Option<A> {
        if self.len() != 0 {
            self.len -= 1;
            unsafe {
                let ptr = self.data.data.as_ptr() as *const A;
                Some(std::ptr::read(ptr.add(self.len)))
            }
        } else {
            None
        }
    }

    /// Removes and returns the element at the specified `index`.
    ///
    /// All elements after the removed element are shifted left to fill the gap.
    /// The vector's length is decreased by one.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> A {
        if index >= self.len {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        }

        unsafe {
            let ptr = self.data.data.as_mut_ptr() as *mut A;
            let value = std::ptr::read(ptr.add(index));
            for i in index..self.len - 1 {
                std::ptr::write(ptr.add(i), std::ptr::read(ptr.add(i + 1)));
            }
            self.len -= 1;
            value
        }
    }

    /// Removes all elements from the vector.
    ///
    /// After calling this method, the vector will be empty (length 0).
    /// All elements are properly dropped during the clearing process.
    /// The vector's capacity remains unchanged.
    pub fn clear(&mut self) {
        unsafe {
            let ptr = self.data.data.as_mut_ptr() as *mut A;
            for i in 0..self.len {
                std::ptr::drop_in_place(ptr.add(i));
            }
            self.len = 0;
        }
    }

    /// Searches for an element equal to `query` and returns its index.
    ///
    /// Returns `Some(index)` if an element equal to `query` is found,
    /// or `None` if no such element exists. If multiple equal elements
    /// exist, returns the index of the first occurrence.
    pub fn search(&self, query: &A) -> Option<usize>
    where
        A: PartialEq,
    {
        for i in 0..self.len {
            if &self[i] == query {
                return Some(i);
            }
        }
        None
    }

    /// Returns the maximum number of elements the vector can hold for type `A`.
    ///
    /// This is a compile-time constant that depends only on the size of type `A`
    /// and the stack buffer size.
    ///
    /// # Panics
    ///
    /// Panics if the type `A` is too large to fit in the stack buffer,
    /// if `A` is a zero-sized type, or if `A` has alignment requirements
    /// larger than its size.
    pub const fn static_capacity() -> usize {
        let size = std::mem::size_of::<A>();
        let align = std::mem::align_of::<A>();
        if size > STACK_BYTES {
            return 0;
        }
        if size == 0 {
            panic!("ZSTs are not supported.");
        }
        if align > size {
            panic!("Types with alignment larger than their size are not supported.");
        }
        STACK_BYTES / size
    }

    /// Returns the maximum number of elements this vector can hold.
    pub fn capacity(&self) -> usize {
        STACK_BYTES / std::mem::size_of::<A>()
    }

    /// Returns `true` if the vector can accommodate one more element.
    pub fn may_push(&self) -> bool {
        self.len < self.capacity()
    }

    /// Returns `true` if the vector can accommodate `other_len` additional elements.
    ///
    /// Uses saturating addition to prevent overflow when checking capacity.
    pub fn may_append(&self, other_len: usize) -> bool {
        self.len.saturating_add(other_len) <= self.capacity()
    }

    /// Moves all elements from `other` into this vector.
    ///
    /// After the operation, `other` will be empty and this vector will contain
    /// all elements from both vectors. The elements from `other` are appended
    /// to the end of this vector in their original order.
    ///
    /// # Panics
    ///
    /// Panics if there is insufficient capacity to hold all elements from both vectors.
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

    /// Removes all elements from the vector and returns them as an iterator.
    ///
    /// After calling this method, the vector will be empty. The returned iterator
    /// yields owned elements and will properly drop any elements that are not
    /// consumed when the iterator is dropped.
    pub fn drain_all(&mut self) -> StackVecIntoIter<'_, A> {
        let len = self.len;
        self.len = 0;
        let data = unsafe { std::ptr::read(&self.data) };
        StackVecIntoIter {
            data,
            start: 0,
            end: len,
            lifetime: PhantomData,
        }
    }

    /// Moves all elements from this vector into the specified `other` vector.
    ///
    /// After the operation, this vector will be empty and `other` will contain
    /// all the moved elements appended to its existing contents.
    pub fn drain_to_vec(&mut self, other: &mut Vec<A>) {
        other.reserve(self.len);
        let ptr = unsafe { self.data.data.as_mut_ptr() as *const A };
        for i in 0..self.len {
            other.push(unsafe { std::ptr::read(ptr.add(i)) });
        }
        self.len = 0;
    }

    /// Converts the `StackVec` into a `Vec`.
    ///
    /// All elements are moved from the stack vector to a new heap-allocated
    /// vector. The returned `Vec` will have the same length and contain the
    /// same elements in the same order.
    pub fn into_vec(mut self) -> Vec<A> {
        let mut vec = Vec::with_capacity(self.len);
        self.drain_to_vec(&mut vec);
        vec
    }

    /// Returns an iterator over references to the elements.
    pub fn iter(&self) -> std::slice::Iter<'_, A> {
        self.as_slice().iter()
    }

    /// Returns an iterator over mutable references to the elements.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, A> {
        self.as_mut_slice().iter_mut()
    }

    /// Converts the vector into an iterator that yields owned elements.
    ///
    /// The vector is consumed and cannot be used after calling this method.
    /// The returned iterator will properly drop any remaining elements when
    /// it is dropped.
    pub fn into_iter(self) -> StackVecIntoIter<'static, A> {
        let len = self.len;
        let data = unsafe { std::ptr::read(&self.data) };
        std::mem::forget(self);
        StackVecIntoIter {
            data,
            start: 0,
            end: len,
            lifetime: PhantomData,
        }
    }

    /// Returns the number of elements in the vector.
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
        output
    }
}

impl<A> std::iter::Extend<A> for StackVec<A> {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        for v in iter {
            self.push(v);
        }
    }
}

impl<A: Clone> Clone for StackVec<A> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<A> AsRef<[A]> for StackVec<A> {
    fn as_ref(&self) -> &[A] {
        self.as_slice()
    }
}

impl<A, const N: usize> TryInto<[A; N]> for StackVec<A> {
    type Error = StackVec<A>;

    fn try_into(mut self) -> Result<[A; N], Self::Error> {
        if self.len() != N {
            return Err(self);
        }

        // Create an uninitialized array
        let mut result: [MaybeUninit<A>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        // Move elements from the StackVec into the array
        unsafe {
            let src_ptr = self.data.data.as_ptr() as *const A;
            for i in 0..N {
                result[i] = MaybeUninit::new(std::ptr::read(src_ptr.add(i)));
            }

            // Prevent StackVec from dropping the elements we just moved
            self.len = 0;
        }

        // Transmute the MaybeUninit array to the final array type
        let result = unsafe { std::mem::transmute_copy::<[MaybeUninit<A>; N], [A; N]>(&result) };
        std::mem::forget(self);

        Ok(result)
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
    fn test_pop_basic() {
        let mut vec = StackVec::new();

        // Pop from empty vec should return None
        assert_eq!(vec.pop(), None::<u32>);
        assert_eq!(vec.len(), 0);

        // Push some values and pop them
        vec.push(1u32);
        vec.push(2u32);
        vec.push(3u32);
        assert_eq!(vec.len(), 3);

        // Pop returns last element
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);

        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.len(), 0);

        // Pop from empty vec again
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_pop_ownership() {
        let counter = Arc::new(AtomicUsize::new(0));

        let mut vec = StackVec::new();
        vec.push(DropTracker::new(1, counter.clone()));
        vec.push(DropTracker::new(2, counter.clone()));
        vec.push(DropTracker::new(3, counter.clone()));

        assert_eq!(counter.load(Ordering::Relaxed), 3);

        // Pop and immediately drop
        {
            let popped = vec.pop().unwrap();
            assert_eq!(popped.id, 3);
            assert_eq!(counter.load(Ordering::Relaxed), 3); // Still 3 because value moved out
        } // popped drops here
        assert_eq!(counter.load(Ordering::Relaxed), 2);

        // Pop and keep alive
        let popped2 = vec.pop().unwrap();
        assert_eq!(popped2.id, 2);
        assert_eq!(counter.load(Ordering::Relaxed), 2);

        drop(vec); // This should drop the remaining element
        assert_eq!(counter.load(Ordering::Relaxed), 1);

        drop(popped2); // Drop the popped element
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_push_pop_sequence() {
        let mut vec = StackVec::new();

        // Test alternating push/pop
        vec.push(10u32);
        assert_eq!(vec.pop(), Some(10));

        vec.push(20);
        vec.push(30);
        assert_eq!(vec.pop(), Some(30));
        vec.push(40);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.as_slice(), &[20, 40]);

        assert_eq!(vec.pop(), Some(40));
        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_pop_lifo_order() {
        let mut vec = StackVec::new();

        // Push multiple elements
        for i in 1..=5 {
            vec.push(i * 10);
        }

        // Pop should return in LIFO order
        assert_eq!(vec.pop(), Some(50));
        assert_eq!(vec.pop(), Some(40));
        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
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
    fn test_remove_basic() {
        let mut vec = StackVec::new();
        vec.push(10u32);
        vec.push(20);
        vec.push(30);
        vec.push(40);

        // Remove from middle
        let removed = vec.remove(1);
        assert_eq!(removed, 20);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.as_slice(), &[10, 30, 40]);

        // Remove from end
        let removed = vec.remove(2);
        assert_eq!(removed, 40);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.as_slice(), &[10, 30]);

        // Remove from beginning
        let removed = vec.remove(0);
        assert_eq!(removed, 10);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.as_slice(), &[30]);

        // Remove last element
        let removed = vec.remove(0);
        assert_eq!(removed, 30);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_remove_out_of_bounds() {
        let mut vec = StackVec::new();
        vec.push(10u32);
        vec.remove(1); // Should panic
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_remove_empty() {
        let mut vec: StackVec<u32> = StackVec::new();
        vec.remove(0); // Should panic
    }

    #[test]
    fn test_remove_ownership() {
        let counter = Arc::new(AtomicUsize::new(0));

        let mut vec = StackVec::new();
        vec.push(DropTracker::new(1, counter.clone()));
        vec.push(DropTracker::new(2, counter.clone()));
        vec.push(DropTracker::new(3, counter.clone()));

        assert_eq!(counter.load(Ordering::Relaxed), 3);

        // Remove middle element
        {
            let removed = vec.remove(1);
            assert_eq!(removed.id, 2);
            assert_eq!(counter.load(Ordering::Relaxed), 3); // Still 3 because value moved out
        } // removed drops here
        assert_eq!(counter.load(Ordering::Relaxed), 2);

        // Verify remaining elements are correct
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0].id, 1);
        assert_eq!(vec[1].id, 3);

        drop(vec);
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_search_basic() {
        let mut vec = StackVec::new();
        vec.push(10u32);
        vec.push(20);
        vec.push(30);
        vec.push(20); // Duplicate

        // Search for existing elements
        assert_eq!(vec.search(&10), Some(0));
        assert_eq!(vec.search(&20), Some(1)); // Returns first occurrence
        assert_eq!(vec.search(&30), Some(2));

        // Search for non-existing element
        assert_eq!(vec.search(&40), None);
    }

    #[test]
    fn test_search_empty() {
        let vec: StackVec<u32> = StackVec::new();
        assert_eq!(vec.search(&10), None);
    }

    #[test]
    fn test_search_single_element() {
        let mut vec = StackVec::new();
        vec.push(42u32);

        assert_eq!(vec.search(&42), Some(0));
        assert_eq!(vec.search(&41), None);
    }

    #[test]
    fn test_search_custom_type() {
        #[derive(PartialEq, Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut vec = StackVec::new();
        vec.push(Point { x: 1, y: 2 });
        vec.push(Point { x: 3, y: 4 });
        vec.push(Point { x: 5, y: 6 });

        assert_eq!(vec.search(&Point { x: 3, y: 4 }), Some(1));
        assert_eq!(vec.search(&Point { x: 0, y: 0 }), None);
    }

    #[test]
    fn test_search_after_remove() {
        let mut vec = StackVec::new();
        vec.push(10u32);
        vec.push(20);
        vec.push(30);

        // Initially find element at index 2
        assert_eq!(vec.search(&30), Some(2));

        // Remove middle element
        vec.remove(1);

        // Element should now be at index 1
        assert_eq!(vec.search(&30), Some(1));
        assert_eq!(vec.search(&20), None); // Should no longer exist
    }

    #[test]
    fn test_remove_search_integration() {
        let mut vec = StackVec::new();
        vec.push(100u32);
        vec.push(200);
        vec.push(300);
        vec.push(400);
        vec.push(500);

        // Find and remove element
        if let Some(index) = vec.search(&300) {
            let removed = vec.remove(index);
            assert_eq!(removed, 300);
        }

        assert_eq!(vec.len(), 4);
        assert_eq!(vec.as_slice(), &[100, 200, 400, 500]);
        assert_eq!(vec.search(&300), None);

        // Verify other elements are still findable at correct indices
        assert_eq!(vec.search(&100), Some(0));
        assert_eq!(vec.search(&200), Some(1));
        assert_eq!(vec.search(&400), Some(2));
        assert_eq!(vec.search(&500), Some(3));
    }
    #[test]
    fn test_drain_all_basic() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        vec.push(3);

        let iter = vec.drain_all();

        // Collect all elements from iterator
        let collected: Vec<u32> = iter.collect();
        assert_eq!(collected, vec![1, 2, 3]);

        assert_eq!(vec.len(), 0); // Vector should be empty after drain
    }

    #[test]
    fn test_drain_all_empty() {
        let mut vec: StackVec<u32> = StackVec::new();
        let mut iter = vec.drain_all();
        assert_eq!(iter.next(), None);
        drop(iter);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_drain_all_ownership() {
        let counter = Arc::new(AtomicUsize::new(0));

        let mut vec = StackVec::new();
        vec.push(DropTracker::new(1, counter.clone()));
        vec.push(DropTracker::new(2, counter.clone()));
        vec.push(DropTracker::new(3, counter.clone()));

        assert_eq!(counter.load(Ordering::Relaxed), 3);

        {
            let mut iter = vec.drain_all();

            assert_eq!(counter.load(Ordering::Relaxed), 3); // Elements still alive in iterator

            // Consume first element
            let first = iter.next().unwrap();
            assert_eq!(first.id, 1);
            drop(first);
            assert_eq!(counter.load(Ordering::Relaxed), 2);

            drop(iter);
            assert_eq!(vec.len(), 0); // Vector is empty after drain
            // Iterator drops remaining elements when it goes out of scope
        }

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_drain_all_partial_consumption() {
        let counter = Arc::new(AtomicUsize::new(0));

        let mut vec = StackVec::new();
        for i in 0..4 {
            vec.push(DropTracker::new(i, counter.clone()));
        }
        assert_eq!(counter.load(Ordering::Relaxed), 4);

        {
            let mut iter = vec.drain_all();

            // Only consume some elements
            let _ = iter.next(); // consume first
            let _ = iter.next(); // consume second
            drop(iter);
            assert_eq!(vec.len(), 0);
            // Drop iterator with remaining elements
        }

        // All elements should be dropped
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_drain_all_size_hint() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        vec.push(3);

        let iter = vec.drain_all();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.len(), 3); // ExactSizeIterator
    }

    #[test]
    fn test_drain_all_iterator_properties() {
        let mut vec = StackVec::new();
        for i in 1..=5 {
            vec.push(i);
        }

        let mut iter = vec.drain_all();

        // Test that iterator returns elements in order
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.size_hint(), (3, Some(3)));

        let remaining: Vec<i32> = iter.collect();
        assert_eq!(remaining, vec![3, 4, 5]);
    }

    #[test]
    fn test_drain_all_single_element() {
        let mut vec = StackVec::new();
        vec.push(42u32);

        let mut iter = vec.drain_all();

        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), None);
        drop(iter);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_drain_all_after_operations() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        vec.push(3);

        // Remove an element first
        vec.remove(1); // Remove element 2
        assert_eq!(vec.as_slice(), &[1, 3]);

        // Drain remaining elements
        let drained: Vec<u32> = vec.drain_all().collect();
        assert_eq!(drained, vec![1, 3]);
        assert_eq!(vec.len(), 0);
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
    fn test_double_ended_iterator() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        let mut iter = vec.into_iter();

        // Test alternating next() and next_back()
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next(), Some(3));

        // Iterator should be exhausted
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_double_ended_iterator_drain_all() {
        let mut vec = StackVec::new();
        vec.push(10u32);
        vec.push(20);
        vec.push(30);

        let mut iter = vec.drain_all();

        // Test next_back() with drain_all
        assert_eq!(iter.next_back(), Some(30));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next_back(), Some(20));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_double_ended_iterator_ownership() {
        let counter = Arc::new(AtomicUsize::new(0));

        let mut vec = StackVec::new();
        vec.push(DropTracker::new(1, counter.clone()));
        vec.push(DropTracker::new(2, counter.clone()));
        vec.push(DropTracker::new(3, counter.clone()));
        vec.push(DropTracker::new(4, counter.clone()));

        assert_eq!(counter.load(Ordering::Relaxed), 4);

        {
            let mut iter = vec.into_iter();

            // Consume from both ends
            let first = iter.next().unwrap();
            assert_eq!(first.id, 1);
            let last = iter.next_back().unwrap();
            assert_eq!(last.id, 4);

            drop(first);
            drop(last);
            assert_eq!(counter.load(Ordering::Relaxed), 2);

            // Iterator drops remaining elements
        }

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_double_ended_iterator_single_element() {
        let mut vec = StackVec::new();
        vec.push(42u32);

        let mut iter = vec.into_iter();

        // Single element should be returned by either next() or next_back()
        assert_eq!(iter.next_back(), Some(42));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_double_ended_iterator_empty() {
        let vec: StackVec<u32> = StackVec::new();
        let mut iter = vec.into_iter();

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_double_ended_iterator_size_hint() {
        let mut vec = StackVec::new();
        vec.push(1u32);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        let mut iter = vec.into_iter();
        assert_eq!(iter.size_hint(), (4, Some(4)));

        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(3)));

        iter.next_back();
        assert_eq!(iter.size_hint(), (2, Some(2)));

        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
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
