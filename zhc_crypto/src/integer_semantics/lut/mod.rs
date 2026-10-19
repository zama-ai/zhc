//! Lookup table operations for ciphertext blocks.
//!
//! This module provides types and functions that emulate programmable bootstrapping (PBS)
//! lookups on ciphertext blocks. In TFHE, a PBS applies an arbitrary function to an encrypted
//! value by evaluating a lookup table — this module emulates that behavior for semantic testing.
//!
//! Four LUT types are provided for different use cases:
//!
//! - [`Lut1`] — single-output lookup table. Build with [`Lut1::from_fn`], apply with
//!   [`Lut1::lookup`].
//! - [`Lut2`] — two-output "many-LUT" table. Evaluates two functions on the same input in a single
//!   operation.
//! - [`Lut4`] — four-output "many-LUT" table. Reserves the two topmost data bits of the input.
//! - [`Lut8`] — eight-output "many-LUT" table. Reserves the three topmost data bits of the input.
//!
//! The legacy [`lookup`] function is also available for ad-hoc lookups without precomputing a
//! table.
//!
//! Assertions on the input and the outputs are controlled by a [`LookupCheck`] policy. Each flag
//! disables one assertion:
//!
//! - [`allow_input_padding`](LookupCheck::allow_input_padding) — skip the input padding check. An
//!   input with its padding bit set negates the outputs.
//! - [`allow_index_bits`](LookupCheck::allow_index_bits) — skip the many-LUT index check. An input
//!   with index bits set rotates the sub-tables. This flag has no effect on [`Lut1`].
//! - [`allow_output_padding`](LookupCheck::allow_output_padding) — skip the output padding check.
//!
//! [`Protect`](LookupCheck::Protect) sets all flags to false (strictest), and
//! [`Permissive`](LookupCheck::Permissive) sets all flags to true (no assertion).
//!
//! The module also re-exports a catalog of concrete LUT functions (e.g. [`None_0`],
//! [`MsgOnly_0`], [`CmpGt_0`], …) that implement specific TFHE operations on a single
//! ciphertext block.
//!
//! # Examples
//!
//! ```rust,no_run
//! # use zhc_crypto::integer_semantics::{SpecBlock, lut::{Lut1, LookupCheck}};
//! let spec = SpecBlock(2, 4);
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
mod constructors;
mod id;
mod lookup;
mod lut;
mod registry;

pub use builtin::*;
pub use id::*;
pub use lookup::*;
pub use lut::*;
pub use registry::*;

#[cfg(test)]
mod legacy;
#[cfg(test)]
mod test;
