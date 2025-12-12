use crate::SmallVec;

/// Splits an iterator into fixed-size chunks.
pub trait ChunkIt
where
    Self: Iterator + Sized,
{
    /// Creates an iterator that yields chunks of elements from this iterator.
    ///
    /// Each chunk contains up to `chunks_size` elements. The last chunk may
    /// contain fewer elements if the iterator length is not evenly divisible
    /// by `chunks_size`.
    fn chunk(self, chunks_size: usize) -> Chunked<Self>;
}

impl<A: Iterator> ChunkIt for A {
    fn chunk(self, chunks_size: usize) -> Chunked<Self> {
        Chunked {
            original: self,
            chunks_size,
        }
    }
}

/// A chunk of elements from an iterator.
///
/// Chunks can be either complete (containing the requested number of elements)
/// or partial (containing fewer elements when the iterator is exhausted).
pub enum Chunk<A> {
    /// A complete chunk containing exactly the requested number of elements.
    Complete(SmallVec<A>),
    /// A partial chunk containing the remaining elements when the iterator ends.
    Rest(SmallVec<A>),
}

/// An iterator that yields chunks of elements from another iterator.
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
