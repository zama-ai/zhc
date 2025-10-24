pub mod dce;
mod dialect;
mod id;
mod ir;
mod op;
mod op_mut;
mod op_ref;
mod printer;
mod signature;
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
use val::*;
use val_mut::*;

/// Error use to report IR issue
#[derive(thiserror::Error, Clone, Debug)]
pub enum IRError<D: Dialect> {
    #[error("Signature Error: {op} received {recv:?} instead of {exp:?}")]
    OpSig {
        op: D::Operations,
        recv: Vec<D::Types>,
        exp: Vec<D::Types>,
    },
    #[error("Range Error:  value could not be represented with {typ}")]
    Range { typ: D::Types },
}

#[cfg(test)]
mod tests;
