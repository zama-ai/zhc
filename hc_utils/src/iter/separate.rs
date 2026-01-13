use std::iter::RepeatWith;

use hc_utils_macro::fsm;

#[fsm]
pub enum Separated<I: Iterator, S: Iterator<Item = I::Item>> {
    OnIter { next: I::Item, iter: I, sep: S },
    OnSep { next: I::Item, iter: I, sep: S },
    Finished,
}

impl<I: Iterator, S: Iterator<Item = I::Item>> Iterator for Separated<I, S> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = None;
        self.transition(|old| match old {
            Separated::OnIter {
                next,
                mut iter,
                sep,
            } => {
                output = Some(next);
                match iter.next() {
                    Some(next) => Separated::OnSep { next, iter, sep },
                    None => Self::Finished,
                }
            }
            Separated::OnSep {
                next,
                iter,
                mut sep,
            } => {
                output = sep.next();
                Separated::OnIter { next, iter, sep }
            }
            Separated::Finished => {
                output = None;
                Separated::Finished
            }
            _ => unreachable!(),
        });
        output
    }
}

pub trait Separate where Self: Iterator + Sized {
    fn separate<S: Iterator<Item=Self::Item>>(self, sep: S) -> Separated<Self, S>;
    fn separate_with<F: FnMut()->Self::Item>(self, f: F) -> Separated<Self, RepeatWith<F>>;
}

impl<I: Iterator> Separate for I {
    fn separate<S: Iterator<Item=Self::Item>>(mut self, sep: S) -> Separated<Self, S> {
        match self.next() {
            Some(next) => Separated::OnIter { next, iter: self, sep},
            None => Separated::Finished,
        }
    }

    fn separate_with<F: FnMut()->Self::Item>(self, f: F) -> Separated<Self, RepeatWith<F>> {
        self.separate(std::iter::repeat_with(f))
    }
}
