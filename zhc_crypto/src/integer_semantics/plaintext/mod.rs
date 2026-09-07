pub type EmulatedPlaintextStorage = u128;

mod plaintext;
mod range;
mod spec;

pub use plaintext::*;
pub use range::*;
pub use spec::*;

#[cfg(test)]
mod test;
