//! Lookup table operations for ciphertext blocks.
//!
//! This module provides functions that emulate programmable bootstrapping (PBS) lookups on
//! ciphertext blocks. In TFHE, a PBS applies an arbitrary function to an encrypted value by
//! evaluating a lookup table — this module emulates that behavior for semantic testing.
//!
//! Two lookup modes are provided, corresponding to different treatments of the padding bit:
//!
//! - [`protect_lookup`] requires the padding bit to be zero and applies the LUT directly. Use this
//!   when the padding bit serves as a guard ensuring the lookup index stays in the table's first
//!   half.
//!
//! - [`wrapping_lookup`] handles inputs with an active padding bit by negating the output,
//!   emulating the negacyclic structure of TFHE lookup tables.
//!
//! # Examples
//!
//! ```
//! use hc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{protect_lookup, wrapping_lookup}};
//!
//! let spec = CiphertextBlockSpec(2, 4);
//! let block = spec.from_message(5);
//!
//! // Define a simple identity LUT
//! let identity = |b| b;
//!
//! // Protected lookup — padding bit must be zero
//! let result = protect_lookup(identity, block);
//!
//! // Wrapping lookup — handles negacyclic semantics
//! let result = wrapping_lookup(identity, block);
//! ```

mod lookup;
mod lut;

pub use lookup::*;
pub use lut::*;

#[cfg(test)]
mod legacy;
#[cfg(test)]
mod test;
