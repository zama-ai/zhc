pub struct FewMapped<I: Iterator, M: Iterator>{
    iter: I,
    mapped: M
}

impl<I: Iterator, M: Iterator> FewMapped<I, M> {

    pub fn map_next(mut self, f: impl FnOnce(I::Item) -> M::Item) -> FewMapped<I, std::iter::Chain<M, std::option::IntoIter<<M as Iterator>::Item>>>{
        FewMapped{
            mapped: self.mapped.chain(self.iter.next().map(f).into_iter()),
            iter: self.iter
        }
    }

    pub fn map_rest(self, f: impl FnMut(I::Item) -> M::Item) -> impl Iterator<Item=M::Item> {
        self.mapped.into_iter().chain(self.iter.map(f))
    }
}

pub trait MapFew<A> where Self: Iterator + Sized {
    fn map_first(self, f: impl FnOnce(Self::Item) -> A) -> FewMapped<Self, std::option::IntoIter<A>>;
}

impl<I: Iterator, A> MapFew<A> for I {
    fn map_first(mut self, f: impl FnOnce(Self::Item) -> A) -> FewMapped<Self, std::option::IntoIter<A>> {
        FewMapped {
            mapped: self.next().map(f).into_iter(),
            iter: self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_first_with_empty_iterator() {
        let iter = std::iter::empty::<i32>();
        let mapped = iter.map_first(|x| x * 2);
        let result: Vec<i32> = mapped.map_rest(|x| x * 2).collect();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_map_first_with_single_element() {
        let iter = vec![5].into_iter();
        let mapped = iter.map_first(|x| x * 2);
        let result: Vec<i32> = mapped.map_rest(|x| x * 3).collect();
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_map_first_with_multiple_elements() {
        let iter = vec![1, 2, 3, 4].into_iter();
        let mapped = iter.map_first(|x| x * 10);
        let result: Vec<i32> = mapped.map_rest(|x| x * 2).collect();
        assert_eq!(result, vec![10, 4, 6, 8]);
    }

    #[test]
    fn test_map_next_multiple_times() {
        let iter = vec![1, 2, 3, 4, 5].into_iter();
        let mapped = iter
            .map_first(|x| x * 10)
            .map_next(|x| x * 100)
            .map_next(|x| x * 1000);
        let result: Vec<i32> = mapped.map_rest(|x| x).collect();
        assert_eq!(result, vec![10, 200, 3000, 4, 5]);
    }

    #[test]
    fn test_different_types() {
        let iter = vec!["hello", "world"].into_iter();
        let mapped = iter.map_first(|s| s.len());
        let result: Vec<usize> = mapped.map_rest(|s| s.len() * 2).collect();
        assert_eq!(result, vec![5, 10]);
    }

    #[test]
    fn test_empty_then_map_next() {
        let iter = vec![42].into_iter();
        let mapped = iter.map_first(|x| x.to_string());
        let result: Vec<String> = mapped.map_rest(|x| format!("rest_{}", x)).collect();
        assert_eq!(result, vec!["42".to_string()]);
    }
}
