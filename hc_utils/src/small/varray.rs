use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

pub struct VArray<A, const N: usize> {
    data: [MaybeUninit<A>; N],
    len: usize,
}

impl<A, const N: usize> Default for VArray<A, N> {
    fn default() -> Self {
        Self::new()
    }
}

// Helper structure for the into_iter iterator
pub struct VArrayIntoIter<A, const N: usize> {
    array: VArray<A, N>,
    index: usize,
}

impl<A, const N: usize> Iterator for VArrayIntoIter<A, N> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len {
            let item = unsafe { self.array.data[self.index].assume_init_read() };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<A, const N: usize> Drop for VArrayIntoIter<A, N> {
    fn drop(&mut self) {
        // Drop any remaining elements
        while self.index < self.array.len {
            unsafe {
                self.array.data[self.index].assume_init_drop();
            }
            self.index += 1;
        }
        // Prevent the VArray from trying to drop elements again
        self.array.len = 0;
    }
}

impl<A, const N: usize> VArray<A, N> {
    pub fn new() -> Self {
        Self {
            data: std::array::from_fn(|_| MaybeUninit::uninit()),
            len: 0,
        }
    }

    pub fn as_slice(&self) -> &[A] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const A, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [A] {
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut A, self.len) }
    }

    pub fn push(&mut self, value: A) {
        if self.len >= N {
            panic!("VArray is full");
        }
        self.data[self.len].write(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<A> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { self.data[self.len].assume_init_read() })
        }
    }

    pub fn remove(&mut self, index: usize) -> A {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        self.as_mut_slice()[index..].rotate_left(1);
        self.pop().unwrap()
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop() {}
    }

    pub fn search(&self, query: &A) -> Option<usize>
    where
        A: PartialEq,
    {
        for i in 0..self.len {
            let element = unsafe { self.data[i].assume_init_ref() };
            if element == query {
                return Some(i);
            }
        }
        None
    }

    pub fn capacity(&self) -> usize {
        N
    }

    /// Returns `true` if the vector can accommodate one more element.
    pub fn may_push(&self) -> bool {
        self.len < N
    }

    /// Returns `true` if the vector can accommodate `other_len` additional elements.
    ///
    /// Uses saturating addition to prevent overflow when checking capacity.
    pub fn may_append(&self, other_len: usize) -> bool {
        self.len.saturating_add(other_len) <= N
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
    pub fn append<const NN: usize>(&mut self, other: &mut VArray<A, NN>) {
        if !self.may_append(other.len) {
            panic!("Insufficient capacity to append elements");
        }
        let start = self.len;
        while let Some(v) = other.pop() {
            self.push(v);
        }
        self.as_mut_slice()[start..].reverse();
    }

    /// Removes all elements from the vector and returns them as an iterator.
    ///
    /// After calling this method, the vector will be empty. The returned iterator
    /// yields owned elements and will properly drop any elements that are not
    /// consumed when the iterator is dropped.
    pub fn drain_all(&mut self) -> impl Iterator<Item = A> {
        let drained = VArray {
            data: std::mem::replace(
                &mut self.data,
                std::array::from_fn(|_| MaybeUninit::uninit()),
            ),
            len: self.len,
        };
        self.len = 0;

        VArrayIntoIter {
            array: drained,
            index: 0,
        }
    }

    /// Moves all elements from this vector into the specified `other` vector.
    ///
    /// After the operation, this vector will be empty and `other` will contain
    /// all the moved elements appended to its existing contents.
    pub fn drain_to_vec(&mut self, other: &mut Vec<A>) {
        other.extend(self.drain_all());
    }

    pub fn into_vec(self) -> Vec<A> {
        let mut vec = Vec::with_capacity(self.len);
        vec.extend(self.into_iter());
        vec
    }

    /// Returns an iterator over references to the elements.
    pub fn iter(&self) -> std::slice::Iter<'_, A> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, A> {
        self.as_mut_slice().iter_mut()
    }

    /// Converts the vector into an iterator that yields owned elements.
    ///
    /// The vector is consumed and cannot be used after calling this method.
    /// The returned iterator will properly drop any remaining elements when
    /// it is dropped.
    pub fn into_iter(self) -> VArrayIntoIter<A, N> {
        VArrayIntoIter {
            array: self,
            index: 0,
        }
    }

    /// Converts the vector into a fixed-size array.
    ///
    /// # Panics
    ///
    /// Panics if the vector is not full (length does not equal capacity).
    pub fn into_array(mut self) -> [A; N] {
        if self.len != N {
            panic!("VArray is not full");
        }
        self.len = 0;
        unsafe { std::mem::transmute_copy(&self.data) }
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<A, const N: usize> Drop for VArray<A, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<A, const N: usize> std::ops::Index<usize> for VArray<A, N> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        unsafe { self.data[index].assume_init_ref() }
    }
}

impl<A, const N: usize> std::ops::IndexMut<usize> for VArray<A, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        unsafe { self.data[index].assume_init_mut() }
    }
}

impl<A: Debug, const N: usize> Debug for VArray<A, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<A: PartialEq, const N: usize, const NN: usize> PartialEq<VArray<A, NN>> for VArray<A, N> {
    fn eq(&self, other: &VArray<A, NN>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<A: Eq, const N: usize> Eq for VArray<A, N> {}

impl<A, const N: usize> std::iter::FromIterator<A> for VArray<A, N> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut array = Self::new();
        for item in iter {
            array.push(item);
        }
        array
    }
}

impl<A, const N: usize> std::iter::Extend<A> for VArray<A, N> {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<A: Clone, const N: usize> Clone for VArray<A, N> {
    fn clone(&self) -> Self {
        let mut new_array = Self::new();
        for i in 0..self.len {
            let element = unsafe { self.data[i].assume_init_ref() };
            new_array.push(element.clone());
        }
        new_array
    }
}

impl<A, const N: usize> AsRef<[A]> for VArray<A, N> {
    fn as_ref(&self) -> &[A] {
        self.as_slice()
    }
}

impl<A: Hash, const N: usize> Hash for VArray<A, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for element in self.iter() {
            element.hash(state);
        }
    }
}

/// Creates a `VArray` containing the provided elements.
///
/// This macro provides a convenient way to create `VArray` instances with
/// known elements at compile time, similar to the `vec!` macro for `Vec`.
#[macro_export]
macro_rules! varr {
    () => {
        $crate::small::VArray::from_iter(std::iter::empty())
    };
    ($elem:expr; $n:expr) => {
        $crate::small::VArray::from_iter(std::iter::repeat($elem).take($n))
    };
    ($($x:expr),+ $(,)?) => {
        $crate::small::VArray::from_iter([$($x),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_default() {
        let arr: VArray<i32, 5> = VArray::new();
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 5);

        let arr2: VArray<i32, 5> = VArray::default();
        assert_eq!(arr2.len(), 0);
        assert_eq!(arr2.capacity(), 5);
    }

    #[test]
    fn test_push_and_pop() {
        let mut arr: VArray<i32, 3> = VArray::new();

        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.len(), 3);

        assert_eq!(arr.pop(), Some(3));
        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.pop(), None);
        assert_eq!(arr.len(), 0);
    }

    #[test]
    #[should_panic(expected = "VArray is full")]
    fn test_push_overflow() {
        let mut arr: VArray<i32, 2> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3); // Should panic
    }

    #[test]
    fn test_indexing() {
        let mut arr: VArray<i32, 3> = VArray::new();
        arr.push(10);
        arr.push(20);
        arr.push(30);

        assert_eq!(arr[0], 10);
        assert_eq!(arr[1], 20);
        assert_eq!(arr[2], 30);

        arr[1] = 25;
        assert_eq!(arr[1], 25);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_out_of_bounds() {
        let arr: VArray<i32, 3> = VArray::new();
        let _ = arr[0]; // Should panic on empty array
    }

    #[test]
    fn test_remove() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        arr.push(4);

        assert_eq!(arr.remove(1), 2);
        assert_eq!(arr.as_slice(), &[1, 3, 4]);
        assert_eq!(arr.len(), 3);

        assert_eq!(arr.remove(0), 1);
        assert_eq!(arr.as_slice(), &[3, 4]);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_remove_out_of_bounds() {
        let mut arr: VArray<i32, 3> = VArray::new();
        arr.push(1);
        arr.remove(1); // Should panic
    }

    #[test]
    fn test_clear() {
        let mut arr: VArray<i32, 3> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        arr.clear();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_search() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(10);
        arr.push(20);
        arr.push(30);

        assert_eq!(arr.search(&20), Some(1));
        assert_eq!(arr.search(&30), Some(2));
        assert_eq!(arr.search(&40), None);
    }

    #[test]
    fn test_may_push_and_may_append() {
        let mut arr: VArray<i32, 3> = VArray::new();
        assert!(arr.may_push());
        assert!(arr.may_append(3));
        assert!(!arr.may_append(4));

        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert!(!arr.may_push());
        assert!(!arr.may_append(1));
        assert!(arr.may_append(0));
    }

    #[test]
    fn test_append() {
        let mut arr1: VArray<i32, 5> = VArray::new();
        arr1.push(1);
        arr1.push(2);

        let mut arr2: VArray<i32, 3> = VArray::new();
        arr2.push(3);
        arr2.push(4);

        arr1.append(&mut arr2);

        assert_eq!(arr1.as_slice(), &[1, 2, 3, 4]);
        assert_eq!(arr2.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Insufficient capacity to append elements")]
    fn test_append_overflow() {
        let mut arr1: VArray<i32, 3> = VArray::new();
        arr1.push(1);
        arr1.push(2);

        let mut arr2: VArray<i32, 3> = VArray::new();
        arr2.push(3);
        arr2.push(4);

        arr1.append(&mut arr2); // Should panic
    }

    #[test]
    fn test_drain_all() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        let drained: Vec<i32> = arr.drain_all().collect();

        assert_eq!(drained, vec![1, 2, 3]);
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_drain_to_vec() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        let mut vec = vec![0];
        arr.drain_to_vec(&mut vec);

        assert_eq!(vec, vec![0, 1, 2, 3]);
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_into_vec() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        let vec = arr.into_vec();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_iterators() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        let collected: Vec<&i32> = arr.iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);

        for x in arr.iter_mut() {
            *x += 10;
        }
        assert_eq!(arr.as_slice(), &[11, 12, 13]);

        let collected: Vec<i32> = arr.into_iter().collect();
        assert_eq!(collected, vec![11, 12, 13]);
    }

    #[test]
    fn test_debug() {
        let mut arr: VArray<i32, 3> = VArray::new();
        arr.push(1);
        arr.push(2);

        let debug_str = format!("{:?}", arr);
        assert_eq!(debug_str, "[1, 2]");
    }

    #[test]
    fn test_partial_eq() {
        let mut arr1: VArray<i32, 5> = VArray::new();
        arr1.push(1);
        arr1.push(2);

        let mut arr2: VArray<i32, 3> = VArray::new();
        arr2.push(1);
        arr2.push(2);

        assert_eq!(arr1, arr2);

        arr2.push(3);
        assert_ne!(arr1, arr2);
    }

    #[test]
    fn test_from_iterator() {
        let arr: VArray<i32, 5> = [1, 2, 3].into_iter().collect();
        assert_eq!(arr.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_extend() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);

        arr.extend([2, 3, 4]);
        assert_eq!(arr.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_clone() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        let cloned = arr.clone();
        assert_eq!(arr, cloned);
        assert_eq!(cloned.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_as_ref() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);

        let slice: &[i32] = arr.as_ref();
        assert_eq!(slice, &[1, 2]);
    }

    #[test]
    fn test_as_slice_and_as_mut_slice() {
        let mut arr: VArray<i32, 5> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        assert_eq!(arr.as_slice(), &[1, 2, 3]);

        let slice = arr.as_mut_slice();
        slice[1] = 20;
        assert_eq!(arr.as_slice(), &[1, 20, 3]);
    }

    #[test]
    fn test_drop_behavior() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let counter = Arc::new(AtomicUsize::new(0));

        struct DropCounter(Arc<AtomicUsize>);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        {
            let mut arr: VArray<DropCounter, 5> = VArray::new();
            arr.push(DropCounter(counter.clone()));
            arr.push(DropCounter(counter.clone()));
            arr.push(DropCounter(counter.clone()));
            // arr goes out of scope here
        }

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_drain_all_drop_behavior() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let counter = Arc::new(AtomicUsize::new(0));

        #[derive(Debug)]
        struct DropCounter(Arc<AtomicUsize>);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        {
            let mut arr: VArray<DropCounter, 5> = VArray::new();
            arr.push(DropCounter(counter.clone()));
            arr.push(DropCounter(counter.clone()));
            arr.push(DropCounter(counter.clone()));

            let mut iter = arr.drain_all();
            let _first = iter.next(); // Take one element
            // iter goes out of scope here, should drop remaining elements
        }

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_empty_array() {
        let arr: VArray<i32, 0> = VArray::new();
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 0);
        assert!(!arr.may_push());
        assert!(arr.may_append(0));
        assert!(!arr.may_append(1));
    }

    #[test]
    fn test_single_element_capacity() {
        let mut arr: VArray<i32, 1> = VArray::new();
        assert!(arr.may_push());

        arr.push(42);
        assert!(!arr.may_push());
        assert_eq!(arr[0], 42);

        assert_eq!(arr.pop(), Some(42));
        assert!(arr.may_push());
    }

    #[test]
    fn test_into_array() {
        let mut arr: VArray<i32, 3> = VArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        let array = arr.into_array();
        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "VArray is not full")]
    fn test_into_array_not_full() {
        let mut arr: VArray<i32, 3> = VArray::new();
        arr.push(1);
        arr.push(2);
        // Only 2 elements, capacity is 3
        arr.into_array(); // Should panic
    }

    #[test]
    fn test_into_array_drop_behavior() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let counter = Arc::new(AtomicUsize::new(0));

        #[derive(Debug)]
        struct DropCounter(Arc<AtomicUsize>);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        {
            let mut arr: VArray<DropCounter, 3> = VArray::new();
            arr.push(DropCounter(counter.clone()));
            arr.push(DropCounter(counter.clone()));
            arr.push(DropCounter(counter.clone()));

            let _array = arr.into_array();
            // Elements are moved into the array, not dropped yet
            assert_eq!(counter.load(Ordering::SeqCst), 0);

            // array goes out of scope here, elements should be dropped
        }

        // All 3 elements should have been dropped
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_into_array_zero_size() {
        let arr: VArray<i32, 0> = VArray::new();
        let array = arr.into_array();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_into_array_single_element() {
        let mut arr: VArray<String, 1> = VArray::new();
        arr.push("hello".to_string());

        let array = arr.into_array();
        assert_eq!(array, ["hello".to_string()]);
    }
}
