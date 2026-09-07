pub type EmulatedCiphertextStorage = u128;

mod ciphertext;
mod range;
mod spec;

pub use ciphertext::*;
pub use range::*;
pub use spec::*;

#[cfg(test)]
mod test;
