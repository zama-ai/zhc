//! Integer operations on encrypted data.
//!
//! This module collects the built-in *integer operation primitives* (iops) that
//! the [`Builder`](crate::Builder) can emit into an IR. Every operation comes in
//! two flavours:
//!
//! - A **factory function** (e.g. [`add`], [`cmp_gt`], [`bitwise_xor`]) that returns a fully wired
//!   [`Builder`](crate::Builder) with declared inputs and outputs, ready to be compiled into an IR
//!   via [`into_ir`](crate::Builder::into_ir).
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
//! let ir = add(spec).into_ir();
//!
//! // Composed: add then compare inside a single builder.
//! let mut builder = Builder::new(spec.block_spec());
//! let a = builder.input_ciphertext(spec.int_size());
//! let b = builder.input_ciphertext(spec.int_size());
//! let sum = builder.iop_add_hillis_steele(&a, &b);
//! let is_gt = builder.iop_cmp(&sum, &b, CmpKind::Greater);
//! builder.output_ciphertext(is_gt);
//! ```

mod add;
mod bitwise;
mod cmp;
mod count;
mod if_then_else;
mod if_then_zero;
mod mh_mul;
mod mul;

pub use add::*;
pub use bitwise::*;
pub use cmp::*;
pub use count::*;
pub use if_then_else::*;
pub use if_then_zero::*;
pub use mh_mul::*;
pub use mul::*;

// SUB
// MUL
// DIV
// MOD
//
// OVF_ADD
// OVF_SUB
// OVF_MUL
//
// ROT_R
// ROT_L
// SHIFT_R
// SHIFT_L
//
// ADDS
// SUBS
// SSUB
// MULS
// DIVS
// MODS
//
// ROTS_R
// ROTS_L
// SHIFTS_R
// SHIFTS_L
//
// OVF_ADDS
// OVF_SUBS
// OVF_SSUB
// OVF_MULS
//
// ERC_20
// MEMCPY
//
// ILOG2
// LEAD0
// LEAD1
// TRAIL0
// TRAIL1
//
// ADD_SIMD
// ERC_20_SIMD
