pub type EmulatedPlaintextBlockStorage = u16;

mod block;
mod range;
mod spec;

pub use block::*;
pub use range::*;
pub use spec::*;

#[cfg(test)]
mod test;
