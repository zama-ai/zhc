/// Checks if all elements in an iterator are equal to each other.
pub trait AllEq {
    /// Returns `Some(true)` if all elements are equal, `Some(false)` if not, or `None` for empty iterators.
    fn all_eq(self) -> Option<bool>;
}

impl<I: Iterator<Item = T>, T: PartialEq> AllEq for I {
    fn all_eq(mut self) -> Option<bool> {
        let Some(first) = self.next() else {
            return None;
        };
        Some(self.all(|a| a == first))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty_iterator() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.into_iter().all_eq(), None);
    }

    #[test]
    fn test_single_element() {
        let single = vec![42];
        assert_eq!(single.into_iter().all_eq(), Some(true));
    }

    #[test]
    fn test_all_equal_elements() {
        let all_same = vec![5, 5, 5, 5];
        assert_eq!(all_same.into_iter().all_eq(), Some(true));
    }

    #[test]
    fn test_not_all_equal_elements() {
        let mixed = vec![1, 2, 3];
        assert_eq!(mixed.into_iter().all_eq(), Some(false));
    }

    #[test]
    fn test_partially_equal_elements() {
        let partial = vec![7, 7, 8, 7];
        assert_eq!(partial.into_iter().all_eq(), Some(false));
    }
}
