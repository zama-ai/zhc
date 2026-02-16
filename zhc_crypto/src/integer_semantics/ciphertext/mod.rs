pub type EmulatedCiphertextStorage = u128;

mod ciphertext;
mod spec;

pub use ciphertext::*;
pub use spec::*;

#[cfg(test)]
mod test;
