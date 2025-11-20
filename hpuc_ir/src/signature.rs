use std::fmt::Display;

use hpuc_utils::SmallVec;

use super::DialectTypes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature<T: DialectTypes>(pub SmallVec<T>, pub SmallVec<T>);

impl<T: DialectTypes> Signature<T> {
    pub fn get_args(&self) -> &[T] {
        self.0.as_slice()
    }

    pub fn get_returns(&self) -> &[T] {
        self.1.as_slice()
    }

    pub fn into_args(self) -> SmallVec<T> {
        self.0
    }

    pub fn into_returns(self) -> SmallVec<T> {
        self.1
    }

    pub fn get_args_arity(&self) -> usize {
        self.0.len()
    }

    pub fn get_returns_arity(&self) -> usize {
        self.1.len()
    }
}

impl<T: DialectTypes> Display for Signature<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_args_arity() {
            0 => write!(f, "()"),
            1 => write!(f, "{:?}", self.get_args()[0]),
            _ => {
                let mut d = f.debug_tuple("");
                self.get_args().iter().for_each(|inp| {
                    d.field(inp);
                });
                d.finish()
            }
        }?;
        write!(f, " -> ")?;
        match self.get_returns_arity() {
            0 => write!(f, "()"),
            1 => write!(f, "{:?}", self.get_returns()[0]),
            _ => {
                let mut d = f.debug_tuple("");
                self.get_returns().iter().for_each(|oup| {
                    d.field(oup);
                });
                d.finish()
            }
        }
    }
}

#[macro_export]
macro_rules! sig {
    (($($arg:expr),*) -> ($($ret:expr),*)) => {
        Signature(hpuc_utils::svec![$($arg),*], hpuc_utils::svec![$($ret),*])
    };
}
