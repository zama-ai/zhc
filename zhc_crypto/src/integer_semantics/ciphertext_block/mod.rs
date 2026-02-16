pub type EmulatedCiphertextBlockStorage = u16;

mod block;
mod spec;

pub use block::*;
pub use spec::*;

#[cfg(test)]
mod test;
