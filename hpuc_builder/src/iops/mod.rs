//! Integer operations for homomorphic encryption circuits.
//!
//! This module provides specialized implementations of common integer operations
//! like comparisons that work efficiently with encrypted data blocks.

pub mod add;
pub mod cmp;
pub mod if_then_else;
pub mod if_then_zero;

// ADD
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
// BW_AND
// BW_OR
// BW_XOR
//
// ERC_20
// MEMCPY
//
// COUNT0
// COUNT1
// ILOG2
// LEAD0
// LEAD1
// TRAIL0
// TRAIL1
//
// ADD_SIMD
// ERC_20_SIMD
