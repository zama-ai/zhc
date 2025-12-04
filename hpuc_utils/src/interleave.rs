pub trait SeparateWith where Self: Iterator + Sized {
    fn separate_with<S: Iterator<Item = Self::Item>>(self, s: S) -> Separated<Self, S>;
}

impl<I: Iterator> SeparateWith for I {
    fn separate_with<S: Iterator<Item = Self::Item>>(self, s: S) -> Separated<Self, S> {
        Separated::OnMain(self, s)
    }
}

pub enum Separated<A: Iterator, S: Iterator<Item = A::Item>> {
    OnMain(A, S),
    OnSep(S, A),
    Finished
}

impl<A: Iterator, S: Iterator<Item=A::Item>> Separated<A, S> {
    fn to_sep(&mut self) {
        let Separated::OnMain(a, s) = std::mem::replace(self, Separated::Finished) else {panic!()};
        let _ = std::mem::replace(self, Separated::OnSep(s, a));
    }

    fn to_main(&mut self) {
        let Separated::OnSep(s, a) = std::mem::replace(self, Separated::Finished) else {panic!()};
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

impl<A: Iterator, S: Iterator<Item=A::Item>> Iterator for Separated<A, S> {
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
            },
            None => {
                self.finish();
            },
        };
        out
    }
}
