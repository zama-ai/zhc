pub type CiphertextBlockStorage = u16;

mod block;
mod spec;

pub use block::*;
pub use spec::*;

#[cfg(test)]
mod test;
