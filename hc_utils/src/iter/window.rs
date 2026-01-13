use crate::{small::VArray, varr};
use std::mem::MaybeUninit;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Slider<A, const N: usize> {
    Prelude(VArray<A, N>),
    Complete(VArray<A, N>),
    Postlude(VArray<A, N>),
}

impl<A, const N: usize> Slider<A, N> {
    pub fn unwrap_prelude(self) -> VArray<A, N> {
        match self {
            Slider::Prelude(sv) => sv,
            _ => panic!()
        }
    }
    pub fn unwrap_complete(self) -> VArray<A, N> {
        match self {
            Slider::Complete(sv) => sv,
            _ => panic!()
        }
    }
    pub fn unwrap_postlude(self) -> VArray<A, N> {
        match self {
            Slider::Postlude(sv) => sv,
            _ => panic!()
        }
    }
}

pub struct Slided<I: Iterator, const N: usize> {
    iter: I,
    buffer: MaybeUninit<Slider<I::Item, N>>,
}

impl<I: Iterator, const N: usize> Iterator for Slided<I, N>
where
    I::Item: Clone,
{
    type Item = Slider<I::Item, N>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => {
                let current_buffer = unsafe { self.buffer.assume_init_read() };
                let output = current_buffer.clone();
                match current_buffer {
                    Slider::Prelude(mut sv) | Slider::Complete(mut sv) => {
                        if sv.len() == N {
                            sv.as_mut_slice().rotate_left(1);
                            sv.as_mut_slice()[N - 1] = v;
                            self.buffer.write(Slider::Complete(sv))
                        } else if sv.len() == N - 1 {
                            sv.push(v);
                            self.buffer.write(Slider::Complete(sv))
                        } else {
                            sv.push(v);
                            self.buffer.write(Slider::Prelude(sv))
                        }
                    }
                    _ => unreachable!(),
                };
                Some(output)
            }
            None => {
                let current_buffer = unsafe { self.buffer.assume_init_read() };
                let output = current_buffer.clone();
                match current_buffer {
                    Slider::Prelude(mut sv) | Slider::Complete(mut sv) | Slider::Postlude(mut sv) => {
                        if !sv.is_empty() {
                            sv.as_mut_slice().rotate_left(1);
                            sv.pop();
                        }
                        self.buffer.write(Slider::Postlude(sv))
                    }
                };
                if let Slider::Postlude(sv) = &output && sv.is_empty() {
                    None
                } else {
                   Some(output)
                }
            },
        }
    }
}

pub trait Slide
where
    Self: Iterator + Sized,
{
    fn slide<const N: usize>(self) -> Slided<Self, N>;
}

impl<I: Iterator> Slide for I {
    fn slide<const N: usize>(mut self) -> Slided<Self, N> {
        assert!(N > 1);
        let buffer = match self.next() {
            Some(v) => MaybeUninit::new(Slider::Prelude(varr![v])),
            None => MaybeUninit::new(Slider::Postlude(varr![])),
        };
        Slided {
            iter: self,
            buffer,
        }
    }
}

pub fn filter_out_postludes<A, const N: usize>(inp: &Slider<A, N>) -> bool {
    !matches!(inp, Slider::Postlude(_))
}

pub fn filter_out_preludes<A, const N: usize>(inp: &Slider<A, N>) -> bool {
    !matches!(inp, Slider::Prelude(_))
}

pub fn filter_out_noncompletes<A, const N: usize>(inp: &Slider<A, N>) -> bool {
    matches!(inp, Slider::Complete(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_empty_iterator() {
        let vec: Vec<i32> = vec![];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_single_element() {
        let vec = vec![1];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1])));
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_exact_size() {
        let vec = vec![1, 2, 3];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1])));
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1, 2])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![1, 2, 3])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![2, 3])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![3])));
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_larger_than_window() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut slided = vec.into_iter().slide::<3>();
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1])));
        assert_eq!(slided.next(), Some(Slider::Prelude(varr![1, 2])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![1, 2, 3])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![2, 3, 4])));
        assert_eq!(slided.next(), Some(Slider::Complete(varr![3, 4, 5])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![4, 5])));
        assert_eq!(slided.next(), Some(Slider::Postlude(varr![5])));
        assert_eq!(slided.next(), None);
    }

    #[test]
    fn test_slide_collect() {
        let vec = vec![1, 2, 3, 4];
        let result: Vec<_> = vec.into_iter().slide::<2>().collect();
        assert_eq!(result, vec![
            Slider::Prelude(varr![1]),
            Slider::Complete(varr![1, 2]),
            Slider::Complete(varr![2, 3]),
            Slider::Complete(varr![3, 4]),
            Slider::Postlude(varr![4]),
        ]);
    }

    #[test]
    fn test_smaller_than_window() {
        let vec = vec![1, 2, 3, 4];
        let result: Vec<_> = vec.into_iter().slide::<5>().collect();
        assert_eq!(result, vec![
            Slider::Prelude(varr![1]),
            Slider::Prelude(varr![1, 2]),
            Slider::Prelude(varr![1, 2, 3]),
            Slider::Prelude(varr![1, 2, 3, 4]),
            Slider::Postlude(varr![2, 3, 4]),
            Slider::Postlude(varr![3, 4]),
            Slider::Postlude(varr![4]),
        ]);
    }

}
