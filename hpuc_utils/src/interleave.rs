/// Interleaves elements from this iterator with elements from a separator iterator.
pub trait SeparateWith
where
    Self: Iterator + Sized,
{
    /// Creates an iterator that alternates between elements from `self` and `s`.
    ///
    /// The resulting iterator starts with an element from `self`, then takes one
    /// from `s`, then back to `self`, and so on. If either iterator is exhausted,
    /// the combined iterator terminates.
    fn separate_with<S: Iterator<Item = Self::Item>>(self, s: S) -> Separated<Self, S>;
}

impl<I: Iterator> SeparateWith for I {
    fn separate_with<S: Iterator<Item = Self::Item>>(self, s: S) -> Separated<Self, S> {
        Separated::OnMain(self, s)
    }
}

/// An iterator that alternates between elements from two source iterators.
///
/// Created by the `separate_with` method. This iterator yields elements by
/// alternating between the main iterator and the separator iterator until
/// either one is exhausted.
pub enum Separated<A: Iterator, S: Iterator<Item = A::Item>> {
    OnMain(A, S),
    OnSep(S, A),
    Finished,
}

impl<A: Iterator, S: Iterator<Item = A::Item>> Separated<A, S> {
    fn to_sep(&mut self) {
        let Separated::OnMain(a, s) = std::mem::replace(self, Separated::Finished) else {
            panic!()
        };
        let _ = std::mem::replace(self, Separated::OnSep(s, a));
    }

    fn to_main(&mut self) {
        let Separated::OnSep(s, a) = std::mem::replace(self, Separated::Finished) else {
            panic!()
        };
        let _ = std::mem::replace(self, Separated::OnMain(a, s));
    }

    fn alternate(&mut self) {
        match self {
            Separated::OnMain(_, _) => self.to_sep(),
            Separated::OnSep(_, _) => self.to_main(),
            Separated::Finished => panic!(),
        }
    }

    fn finish(&mut self) {
        let _ = std::mem::replace(self, Separated::Finished);
    }
}

impl<A: Iterator, S: Iterator<Item = A::Item>> Iterator for Separated<A, S> {
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self {
            Separated::OnMain(a, _) => a.next(),
            Separated::OnSep(s, _) => s.next(),
            Separated::Finished => None,
        };
        match out {
            Some(_) => {
                self.alternate();
            }
            None => {
                self.finish();
            }
        };
        out
    }
}
