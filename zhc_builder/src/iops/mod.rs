//! Integer operations on encrypted data.
//!
//! This module collects the built-in *integer operation primitives* (iops) that
//! the [`Builder`](crate::Builder) can emit into an IR. Every operation comes in
//! two flavours:
//!
//! - A **factory function** (e.g. [`add`], [`cmp_gt`], [`bitwise_xor`]) that returns a fully wired
//!   [`Builder`](crate::Builder) with declared inputs and outputs, ready to be compiled into an IR
//!   via [`optimize_ir`](crate::Builder::optimize_ir).
//!
//! - A **builder method** (e.g. [`Builder::iop_add_hillis_steele`], [`Builder::iop_cmp`]) that can
//!   be called on an existing [`Builder`](crate::Builder) to compose the operation with other
//!   logic.
//!
//! # Examples
//!
//! ```rust,no_run
//! # use zhc_builder::*;
//! # let spec = CiphertextSpec::new(16, 2, 2);
//! // Standalone: build a complete addition IR.
//! let ir = add(spec).optimize_ir();
//!
//! // Composed: add then compare inside a single builder.
//! let mut builder = Builder::new(spec.block_spec());
//! let a = builder.ciphertext_input(spec.int_size());
//! let b = builder.ciphertext_input(spec.int_size());
//! let (sum, _carry) = builder.iop_add_hillis_steele(&a, &b, None);
//! let is_gt = builder.iop_cmp(&sum, &b, CmpKind::Greater);
//! builder.ciphertext_output(&is_gt);
//! ```

/// Selects which bit value to count or propagate in bit-scanning operations.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BitType {
    /// Count or propagate one-bits.
    One,
    /// Count or propagate zero-bits.
    Zero,
}

/// Direction of bit propagation in leading/trailing bit operations.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PropagationDirection {
    /// Propagate from most-significant to least-significant bit.
    MsbToLsb,
    /// Propagate from least-significant to most-significant bit.
    LsbToMsb,
}

/// Number of parallel transfers in a SIMD batch.
pub const SIMD_N: usize = 12;

mod add;
mod adds;
mod bitwise;
mod cast;
mod cmp;
mod count;
mod div;
mod divs;
mod erc7984;
mod flip;
mod if_then_else;
mod if_then_zero;
mod lead_trail;
mod memcpy;
mod mh_mul;
mod mul;
mod muls;
mod shiftrot;
mod sub;
mod subs;
mod trivial_encrypt;

pub use add::*;
pub use adds::*;
pub use bitwise::*;
pub use cast::*;
pub use cmp::*;
pub use count::*;
pub use div::*;
pub use divs::*;
pub use erc7984::*;
pub use flip::*;
pub use if_then_else::*;
pub use if_then_zero::*;
pub use lead_trail::*;
pub use memcpy::*;
pub use mh_mul::*;
pub use mul::*;
pub use muls::*;
pub use shiftrot::*;
pub use sub::*;
pub use subs::*;
pub use trivial_encrypt::*;
