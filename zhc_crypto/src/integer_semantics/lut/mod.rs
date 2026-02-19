//! Lookup table operations for ciphertext blocks.
//!
//! This module provides functions that emulate programmable bootstrapping (PBS) lookups on
//! ciphertext blocks. In TFHE, a PBS applies an arbitrary function to an encrypted value by
//! evaluating a lookup table — this module emulates that behavior for semantic testing.
//!
//! The central entry point is [`lookup`], which applies a LUT function to an
//! [`EmulatedCiphertextBlock`](super::EmulatedCiphertextBlock) with negacyclic wraparound
//! semantics. When the input padding bit is set, the raw output is two's-complement negated to
//! reproduce the negacyclic table folding of a real TFHE bootstrap.
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
//! # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{lookup, LookupCheck}};
//! let spec = CiphertextBlockSpec(2, 4);
//! let block = spec.from_message(5);
//! let identity = |b| b;
//!
//! // Protected lookup — both padding bits must be zero
//! let result = lookup(identity, block, LookupCheck::Protect);
//!
//! // Permissive lookup — skip all padding-bit checks
//! let result = lookup(identity, block, LookupCheck::AllowBothPadding);
//! ```

mod lookup;
mod lut;

pub use lookup::*;
pub use lut::*;

#[cfg(test)]
mod legacy;
#[cfg(test)]
mod test;
