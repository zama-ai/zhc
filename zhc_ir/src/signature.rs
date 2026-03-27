use std::fmt::{Debug, Display};
use zhc_utils::small::SmallVec;

/// A function signature specifying argument and return types.
///
/// The signature describes the type interface of an operation, listing the
/// types of input arguments and output values. This information is used for
/// type checking during IR construction and optimization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Signature<T>(pub SmallVec<T>, pub SmallVec<T>);

impl<T> Signature<T> {
    /// Creates a signature with no argument and no return types.
    pub fn empty() -> Self {
        Signature(SmallVec::new(), SmallVec::new())
    }

    /// Appends an argument type to the signature.
    pub fn push_arg(&mut self, typ: T) {
        self.0.push(typ);
    }

    /// Appends a return type to the signature.
    pub fn push_ret(&mut self, typ: T) {
        self.1.push(typ);
    }

    /// Returns the argument types as a slice.
    pub fn get_args(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Returns the return types as a slice.
    pub fn get_returns(&self) -> &[T] {
        self.1.as_slice()
    }

    /// Consumes the signature and returns the argument types.
    pub fn into_args(self) -> SmallVec<T> {
        self.0
    }

    /// Consumes the signature and returns the return types.
    pub fn into_returns(self) -> SmallVec<T> {
        self.1
    }

    /// Returns the number of argument types.
    pub fn get_args_arity(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of return types.
    pub fn get_returns_arity(&self) -> usize {
        self.1.len()
    }
}

impl<T: Debug> Display for Signature<T> {
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

/// Creates a signature with the specified argument and return types.
///
/// This macro provides a convenient syntax for creating function signatures,
/// using arrow notation to separate arguments from return values.
#[macro_export]
macro_rules! sig {
    (($($arg:expr),*) -> ($($ret:expr),*)) => {
        Signature(zhc_utils::svec![$($arg),*], zhc_utils::svec![$($ret),*])
    };
}
