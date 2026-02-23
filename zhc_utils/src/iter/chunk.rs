//! Fixed-size chunking for iterators.
//!
//! This module provides an extension trait that lets you split any iterator into chunks of a
//! specified size. Unlike the standard library's `chunks` method on slices, this works on
//! arbitrary iterators and distinguishes between complete chunks and partial remainders.
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_utils::iter::{ChunkIt, Chunk};
//! let values = vec![1, 2, 3, 4, 5];
//! for chunk in values.into_iter().chunk(2) {
//!     match chunk {
//!         Chunk::Complete(items) => println!("Full chunk: {:?}", items.as_slice()),
//!         Chunk::Rest(items) => println!("Remainder: {:?}", items.as_slice()),
//!     }
//! }
//! // Output:
//! // Full chunk: [1, 2]
//! // Full chunk: [3, 4]
//! // Remainder: [5]
//! ```

use crate::small::SmallVec;

/// An extension trait that adds fixed-size chunking to any iterator.
///
/// This trait is automatically implemented for all types that implement [`Iterator`]. Import it
/// to gain access to the [`chunk`](ChunkIt::chunk) method on any iterator.
///
/// The chunking operation is lazy — elements are only consumed from the underlying iterator as
/// you iterate over the chunks.
pub trait ChunkIt
where
    Self: Iterator + Sized,
{
    /// Splits this iterator into fixed-size chunks.
    ///
    /// Returns a new iterator that yields [`Chunk`] values. Each chunk contains up to
    /// `chunk_size` elements collected into a [`SmallVec`]. The iterator distinguishes between
    /// [`Chunk::Complete`] (containing exactly `chunk_size` elements) and [`Chunk::Rest`]
    /// (containing fewer elements when the source iterator is exhausted).
    ///
    /// An empty iterator yields no chunks at all.
    ///
    /// # Examples
    ///
    /// Processing data in fixed-size batches:
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{ChunkIt, Chunk};
    /// let data = vec![1, 2, 3, 4, 5, 6, 7];
    /// let mut chunks = data.into_iter().chunk(3);
    ///
    /// assert!(matches!(chunks.next(), Some(Chunk::Complete(_)))); // [1, 2, 3]
    /// assert!(matches!(chunks.next(), Some(Chunk::Complete(_)))); // [4, 5, 6]
    /// assert!(matches!(chunks.next(), Some(Chunk::Rest(_))));     // [7]
    /// assert!(chunks.next().is_none());
    /// ```
    fn chunk(self, chunk_size: usize) -> Chunked<Self>;
}

impl<A: Iterator> ChunkIt for A {
    fn chunk(self, chunks_size: usize) -> Chunked<Self> {
        Chunked {
            original: self,
            chunks_size,
        }
    }
}

/// A group of elements yielded by a chunked iterator.
///
/// When iterating over chunks, this enum tells you whether the chunk contains the full requested
/// number of elements or just the leftover elements at the end of the source iterator. This
/// distinction is useful when your processing logic differs for complete batches versus partial
/// remainders.
///
/// Both variants wrap a [`SmallVec`], which stores small chunks inline without heap allocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Chunk<A> {
    /// A complete chunk containing exactly the requested number of elements.
    ///
    /// You will receive this variant for every chunk except possibly the last one.
    Complete(SmallVec<A>),

    /// A partial chunk containing the remaining elements.
    ///
    /// This variant appears only as the final chunk when the source iterator's length is not
    /// evenly divisible by the chunk size. It contains between 1 and `chunk_size - 1` elements.
    Rest(SmallVec<A>),
}

impl<A> Chunk<A> {
    /// Extracts the inner vector, panicking if this is a partial chunk.
    ///
    /// Use this method when you expect all chunks to be complete and want to treat a partial
    /// chunk as a programming error. For fallible extraction, match on the enum directly.
    ///
    /// # Panics
    ///
    /// Panics if called on a [`Chunk::Rest`] variant.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{ChunkIt, Chunk};
    /// let data = vec![1, 2, 3, 4, 5, 6];
    /// for chunk in data.into_iter().chunk(3) {
    ///     let items = chunk.unwrap_complete(); // Safe: 6 is divisible by 3
    ///     assert_eq!(items.len(), 3);
    /// }
    /// ```
    pub fn unwrap_complete(self) -> SmallVec<A> {
        match self {
            Chunk::Complete(small_vec) => small_vec,
            Chunk::Rest(_) => panic!(),
        }
    }

    /// Extracts the inner vector, panicking if this is a complete chunk.
    ///
    /// Use this method when you expect a partial chunk (the remainder) and want to treat a
    /// complete chunk as a programming error. For fallible extraction, match on the enum directly.
    ///
    /// # Panics
    ///
    /// Panics if called on a [`Chunk::Complete`] variant.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{ChunkIt, Chunk};
    /// let data = vec![1, 2, 3, 4, 5];
    /// let mut chunks: Vec<_> = data.into_iter().chunk(3).collect();
    /// let last = chunks.pop().unwrap();
    /// let items = last.unwrap_rest(); // Safe: 5 is not divisible by 3
    /// assert_eq!(items.len(), 2);
    /// ```
    pub fn unwrap_rest(self) -> SmallVec<A> {
        match self {
            Chunk::Rest(small_vec) => small_vec,
            Chunk::Complete(_) => panic!(),
        }
    }
}

/// An iterator that yields fixed-size chunks from an underlying iterator.
///
/// Created by calling [`chunk`](ChunkIt::chunk) on any iterator.
pub struct Chunked<A: Iterator> {
    original: A,
    chunks_size: usize,
}

impl<A: Iterator> Iterator for Chunked<A> {
    type Item = Chunk<A::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = SmallVec::new();
        match self.original.next() {
            None => return None,
            Some(v) => output.push(v),
        };
        for _ in 0..self.chunks_size - 1 {
            match self.original.next() {
                None => return Some(Chunk::Rest(output)),
                Some(v) => output.push(v),
            }
        }
        Some(Chunk::Complete(output))
    }
}

/// An extension trait that adds an unwrapping method to chunked iterators.
///
/// This trait provides a convenient way to extract the underlying [`SmallVec`] from each chunk,
/// discarding the distinction between complete and partial chunks.
pub trait UnwrapChunks<A: Iterator> {
    /// Consumes the chunked iterator and returns an iterator over the inner vectors.
    ///
    /// This method strips away the [`Chunk`] wrapper, yielding just the [`SmallVec`] contents
    /// regardless of whether each chunk was complete or partial. Use this when you need to
    /// process all chunks uniformly without caring about the complete/rest distinction.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::iter::{ChunkIt, UnwrapChunks};
    /// let data = vec![1, 2, 3, 4, 5];
    /// let vecs: Vec<_> = data.into_iter().chunk(2).unwrap_chunks().collect();
    /// assert_eq!(vecs.len(), 3);
    /// assert_eq!(vecs[0].len(), 2); // [1, 2]
    /// assert_eq!(vecs[1].len(), 2); // [3, 4]
    /// assert_eq!(vecs[2].len(), 1); // [5]
    /// ```
    fn unwrap_chunks(self) -> UnwrapChunksIter<A>;
}

impl<A: Iterator> UnwrapChunks<A> for Chunked<A> {
    fn unwrap_chunks(self) -> UnwrapChunksIter<A> {
        UnwrapChunksIter { chunked: self }
    }
}

/// An iterator that yields the inner [`SmallVec`] from each chunk.
///
/// Created by calling [`unwrap_chunks`](UnwrapChunks::unwrap_chunks) on a [`Chunked`] iterator.
pub struct UnwrapChunksIter<A: Iterator> {
    chunked: Chunked<A>,
}

impl<A: Iterator> Iterator for UnwrapChunksIter<A> {
    type Item = SmallVec<A::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunked.next().map(|chunk| match chunk {
            Chunk::Complete(vec) => vec,
            Chunk::Rest(vec) => vec,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_complete() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let mut chunked = data.into_iter().chunk(3);

        match chunked.next() {
            Some(Chunk::Complete(chunk)) => {
                assert_eq!(chunk.len(), 3);
                assert_eq!(chunk[0], 1);
                assert_eq!(chunk[1], 2);
                assert_eq!(chunk[2], 3);
            }
            _ => panic!("Expected Complete chunk"),
        }

        match chunked.next() {
            Some(Chunk::Complete(chunk)) => {
                assert_eq!(chunk.len(), 3);
                assert_eq!(chunk[0], 4);
                assert_eq!(chunk[1], 5);
                assert_eq!(chunk[2], 6);
            }
            _ => panic!("Expected Complete chunk"),
        }

        assert!(chunked.next().is_none());
    }

    #[test]
    fn test_chunk_rest() {
        let data = vec![1, 2, 3, 4, 5];
        let mut chunked = data.into_iter().chunk(3);

        match chunked.next() {
            Some(Chunk::Complete(chunk)) => {
                assert_eq!(chunk.len(), 3);
                assert_eq!(chunk[0], 1);
                assert_eq!(chunk[1], 2);
                assert_eq!(chunk[2], 3);
            }
            _ => panic!("Expected Complete chunk"),
        }

        match chunked.next() {
            Some(Chunk::Rest(chunk)) => {
                assert_eq!(chunk.len(), 2);
                assert_eq!(chunk[0], 4);
                assert_eq!(chunk[1], 5);
            }
            _ => panic!("Expected Rest chunk"),
        }

        assert!(chunked.next().is_none());
    }

    #[test]
    fn test_chunk_empty_iterator() {
        let data: Vec<i32> = vec![];
        let mut chunked = data.into_iter().chunk(3);
        assert!(chunked.next().is_none());
    }

    #[test]
    fn test_chunk_size_one() {
        let data = vec![1, 2, 3];
        let chunks: Vec<_> = data.into_iter().chunk(1).collect();

        assert_eq!(chunks.len(), 3);
        for (i, chunk) in chunks.iter().enumerate() {
            match chunk {
                Chunk::Complete(c) => {
                    assert_eq!(c.len(), 1);
                    assert_eq!(c[0], i + 1);
                }
                _ => panic!("Expected Complete chunk"),
            }
        }
    }

    #[test]
    fn test_chunk_larger_than_iterator() {
        let data = vec![1, 2];
        let mut chunked = data.into_iter().chunk(5);

        match chunked.next() {
            Some(Chunk::Rest(chunk)) => {
                assert_eq!(chunk.len(), 2);
                assert_eq!(chunk[0], 1);
                assert_eq!(chunk[1], 2);
            }
            _ => panic!("Expected Rest chunk"),
        }

        assert!(chunked.next().is_none());
    }
}
