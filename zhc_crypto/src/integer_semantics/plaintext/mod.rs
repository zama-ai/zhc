pub type EmulatedPlaintextStorage = u128;

mod plaintext;
mod spec;

pub use plaintext::*;
pub use spec::*;

#[cfg(test)]
mod test;
