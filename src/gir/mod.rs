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

#[cfg(test)]
mod tests;
