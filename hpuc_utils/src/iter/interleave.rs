/// Interleaves elements from this iterator with elements from a separator iterator.
pub trait Interleave
where
    Self: Iterator + Sized,
{
    /// Creates an iterator that alternates between elements from `self` and `s`.
    ///
    /// The resulting iterator starts with an element from `self`, then takes one
    /// from `s`, then back to `self`, and so on. If either iterator is exhausted,
    /// the combined iterator terminates.
    fn interleave_with<S: Iterator<Item = Self::Item>>(self, s: S) -> Interleaved<Self, S>;
}

impl<I: Iterator> Interleave for I {
    fn interleave_with<S: Iterator<Item = Self::Item>>(self, s: S) -> Interleaved<Self, S> {
        Interleaved::OnMain(self, s)
    }
}

/// An iterator that alternates between elements from two source iterators.
///
/// Created by the `separate_with` method. This iterator yields elements by
/// alternating between the main iterator and the separator iterator until
/// either one is exhausted.
pub enum Interleaved<A: Iterator, S: Iterator<Item = A::Item>> {
    OnMain(A, S),
    OnSep(S, A),
    Finished,
}

impl<A: Iterator, S: Iterator<Item = A::Item>> Interleaved<A, S> {
    fn to_sep(&mut self) {
        let Interleaved::OnMain(a, s) = std::mem::replace(self, Interleaved::Finished) else {
            panic!()
        };
        let _ = std::mem::replace(self, Interleaved::OnSep(s, a));
    }

    fn to_main(&mut self) {
        let Interleaved::OnSep(s, a) = std::mem::replace(self, Interleaved::Finished) else {
            panic!()
        };
        let _ = std::mem::replace(self, Interleaved::OnMain(a, s));
    }

    fn alternate(&mut self) {
        match self {
            Interleaved::OnMain(_, _) => self.to_sep(),
            Interleaved::OnSep(_, _) => self.to_main(),
            Interleaved::Finished => panic!(),
        }
    }

    fn finish(&mut self) {
        let _ = std::mem::replace(self, Interleaved::Finished);
    }
}

impl<A: Iterator, S: Iterator<Item = A::Item>> Iterator for Interleaved<A, S> {
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self {
            Interleaved::OnMain(a, _) => a.next(),
            Interleaved::OnSep(s, _) => s.next(),
            Interleaved::Finished => None,
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
