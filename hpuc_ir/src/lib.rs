pub mod cse;
pub mod dce;
mod dialect;
mod id;
mod ir;
mod op;
mod op_map;
mod op_mut;
mod op_ref;
mod printer;
pub mod scheduling;
mod signature;
mod state;
mod val;
mod val_mut;
mod val_ref;

pub use dialect::*;
pub use id::*;
pub use ir::*;
use op::*;
use op_mut::*;
pub use op_ref::*;
pub use printer::*;
pub use signature::*;
pub use state::*;
use val::*;
use val_mut::*;

/// Error use to report IR issue
#[derive(Clone, Debug)]
pub enum IRError<D: Dialect> {
    OpSig {
        op: D::Operations,
        recv: Vec<D::Types>,
        exp: Vec<D::Types>,
    },
    Range {
        typ: D::Types,
    },
}

impl<D: Dialect> std::fmt::Display for IRError<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRError::OpSig { op, recv, exp } => {
                write!(
                    f,
                    "Signature Error: {op} received {recv:?} instead of {exp:?}"
                )
            }
            IRError::Range { typ } => {
                write!(f, "Range Error: value could not be represented with {typ}")
            }
        }
    }
}

impl<D: Dialect> std::error::Error for IRError<D> {}

#[cfg(test)]
mod tests;
