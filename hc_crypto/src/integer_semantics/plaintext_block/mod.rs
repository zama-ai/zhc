pub type EmulatedPlaintextBlockStorage = u16;

mod block;
mod spec;

pub use block::*;
pub use spec::*;

#[cfg(test)]
mod test;
