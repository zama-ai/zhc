pub type EmulatedPlaintextBlockStorage = u16;

mod block;

pub use block::*;

#[cfg(test)]
mod test;
