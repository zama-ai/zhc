use serde::Serialize;
use std::{
    fmt::Debug,
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub struct Fifo<T> {
    container: Vec<MaybeUninit<T>>,
    len: usize,
    ptr: usize,
}

impl<T> Fifo<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let container = (0..capacity).map(|_| MaybeUninit::uninit()).collect();
        Self {
            container,
            len: 0,
            ptr: 0,
        }
    }

    pub fn pop_front(&mut self) -> T {
        if self.len == 0 {
            panic!("Tried to pop from an empty fifo")
        }
        let output = unsafe { self.container[self.ptr].assume_init_read() };
        self.len -= 1;
        self.ptr = (self.ptr + 1) % self.capacity();
        output
    }

    pub fn push_back(&mut self, val: T) {
        assert_ne!(
            self.len(),
            self.capacity(),
            "Tried to insert in a full fifo"
        );
        let insert_ptr = (self.ptr + self.len) % self.capacity();
        self.container[insert_ptr].write(val);
        self.len += 1;
    }

    pub fn capacity(&self) -> usize {
        self.container.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn has_elements(&self) -> bool {
        self.len() > 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len).map(move |i| {
            let actual_index = (self.ptr + i) % self.capacity();
            unsafe { self.container[actual_index].assume_init_ref() }
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let ptr = self.ptr;
        let len = self.len;
        let capacity = self.capacity();
        (0..len).map(move |i| {
            let actual_index = (ptr + i) % capacity;
            unsafe { &mut *self.container[actual_index].as_mut_ptr() }
        })
    }

    pub fn into_iter(mut self) -> impl Iterator<Item = T> {
        std::iter::from_fn(move || {
            if self.len > 0 {
                Some(self.pop_front())
            } else {
                None
            }
        })
    }
}

impl<T> Index<usize> for Fifo<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Index {} out of bounds for fifo of length {}",
                index, self.len
            );
        }
        let actual_index = (self.ptr + index) % self.capacity();
        unsafe { self.container[actual_index].assume_init_ref() }
    }
}

impl<T> IndexMut<usize> for Fifo<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "Index {} out of bounds for fifo of length {}",
                index, self.len
            );
        }
        let actual_index = (self.ptr + index) % self.capacity();
        unsafe { &mut *self.container[actual_index].as_mut_ptr() }
    }
}

impl<T: Clone> Clone for Fifo<T> {
    fn clone(&self) -> Self {
        let mut cloned = Fifo::with_capacity(self.capacity());
        for item in self.iter() {
            cloned.push_back(item.clone());
        }
        cloned
    }
}

impl<T> Drop for Fifo<T> {
    fn drop(&mut self) {
        while self.len > 0 {
            self.pop_front();
        }
    }
}

impl<T: Serialize> Serialize for Fifo<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<T: Debug> Debug for Fifo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fifo_creation() {
        let fifo: Fifo<i32> = Fifo::with_capacity(5);
        assert_eq!(fifo.capacity(), 5);
        assert_eq!(fifo.len(), 0);
        assert!(fifo.is_empty());
    }

    #[test]
    fn test_insert_and_pop_front() {
        let mut fifo = Fifo::with_capacity(3);

        // Insert elements
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        assert_eq!(fifo.len(), 3);
        assert!(!fifo.is_empty());

        // Pop elements in FIFO order
        assert_eq!(fifo.pop_front(), 1);
        assert_eq!(fifo.pop_front(), 2);
        assert_eq!(fifo.pop_front(), 3);

        assert_eq!(fifo.len(), 0);
        assert!(fifo.is_empty());
    }

    #[test]
    fn test_circular_buffer_behavior() {
        let mut fifo = Fifo::with_capacity(3);

        // Fill the buffer
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        // Pop one element and insert another (tests wrap-around)
        assert_eq!(fifo.pop_front(), 1);
        fifo.push_back(4);

        assert_eq!(fifo.len(), 3);
        assert_eq!(fifo.pop_front(), 2);
        assert_eq!(fifo.pop_front(), 3);
        assert_eq!(fifo.pop_front(), 4);
    }

    #[test]
    fn test_indexing() {
        let mut fifo = Fifo::with_capacity(3);
        fifo.push_back(10);
        fifo.push_back(20);
        fifo.push_back(30);

        assert_eq!(fifo[0], 10);
        assert_eq!(fifo[1], 20);
        assert_eq!(fifo[2], 30);

        // Test mutable indexing
        fifo[1] = 25;
        assert_eq!(fifo[1], 25);
    }

    #[test]
    #[should_panic(expected = "Index 3 out of bounds for fifo of length 3")]
    fn test_index_out_of_bounds() {
        let mut fifo = Fifo::with_capacity(5);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        let _ = fifo[3]; // Should panic
    }

    #[test]
    #[should_panic(expected = "Tried to pop from an empty fifo")]
    fn test_pop_from_empty() {
        let mut fifo: Fifo<i32> = Fifo::with_capacity(3);
        fifo.pop_front(); // Should panic
    }

    #[test]
    #[should_panic(expected = "Tried to insert in a full fifo")]
    fn test_insert_into_full() {
        let mut fifo = Fifo::with_capacity(2);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3); // Should panic
    }

    #[test]
    fn test_iter() {
        let mut fifo = Fifo::with_capacity(4);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        let values: Vec<&i32> = fifo.iter().collect();
        assert_eq!(values, vec![&1, &2, &3]);
    }

    #[test]
    fn test_iter_mut() {
        let mut fifo = Fifo::with_capacity(3);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        for val in fifo.iter_mut() {
            *val *= 2;
        }

        assert_eq!(fifo[0], 2);
        assert_eq!(fifo[1], 4);
        assert_eq!(fifo[2], 6);
    }

    #[test]
    fn test_into_iter() {
        let mut fifo = Fifo::with_capacity(3);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        let values: Vec<i32> = fifo.into_iter().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_debug_format() {
        let mut fifo = Fifo::with_capacity(3);
        fifo.push_back(1);
        fifo.push_back(2);

        let debug_str = format!("{:?}", fifo);
        assert_eq!(debug_str, "[1, 2]");
    }

    #[test]
    fn test_serialize() {
        let mut fifo = Fifo::with_capacity(3);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        let serialized = serde_json::to_string(&fifo).unwrap();
        assert_eq!(serialized, "[1,2,3]");
    }

    #[test]
    fn test_iter_with_wrapped_buffer() {
        let mut fifo = Fifo::with_capacity(3);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);

        // Pop one and add one to test wrap-around in iteration
        fifo.pop_front();
        fifo.push_back(4);

        let values: Vec<&i32> = fifo.iter().collect();
        assert_eq!(values, vec![&2, &3, &4]);
    }
}
