//! Lookup table operations for ciphertext blocks.
//!
//! This module provides types and functions that emulate programmable bootstrapping (PBS)
//! lookups on ciphertext blocks. In TFHE, a PBS applies an arbitrary function to an encrypted
//! value by evaluating a lookup table — this module emulates that behavior for semantic testing.
//!
//! Two LUT types are provided for different use cases:
//!
//! - [`Lut1`] — single-output lookup table. Build with [`Lut1::from_fn`], apply with
//!   [`Lut1::lookup`].
//! - [`Lut2`] — two-output "many-LUT" table. Evaluates two functions on the same input in a single
//!   operation.
//!
//! The legacy [`lookup`] function is also available for ad-hoc lookups without precomputing a
//! table.
//!
//! Padding-bit assertions on both input and output are controlled by a [`LookupCheck`] mode:
//!
//! - [`Protect`](LookupCheck::Protect) — assert both padding bits are zero (strictest).
//! - [`AllowInputPadding`](LookupCheck::AllowInputPadding) — skip the input check.
//! - [`AllowOutputPadding`](LookupCheck::AllowOutputPadding) — skip the output check.
//! - [`AllowBothPadding`](LookupCheck::AllowBothPadding) — disable all assertions.
//!
//! The module also re-exports a catalog of concrete LUT functions (e.g. [`None_0`],
//! [`MsgOnly_0`], [`CmpGt_0`], …) that implement specific TFHE operations on a single
//! ciphertext block.
//!
//! # Examples
//!
//! ```rust,no_run
//! # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut1, LookupCheck}};
//! let spec = CiphertextBlockSpec(2, 4);
//!
//! // Build a reusable LUT
//! let double = Lut1::from_fn("double", spec, |b| {
//!     spec.from_message((b.raw_message_bits() * 2) & spec.message_mask())
//! });
//!
//! let input = spec.from_message(5);
//! let output = double.lookup(input, LookupCheck::Protect);
//! assert_eq!(output.raw_message_bits(), 10);
//! ```

mod builtin;
mod lookup;
mod lut;

pub use builtin::*;
pub use lookup::*;
pub use lut::*;

#[cfg(test)]
mod legacy;
#[cfg(test)]
mod test;
