pub type EmulatedPlaintextIntegerStorage = u128;

mod integer;

pub use integer::*;

#[cfg(test)]
mod test;
