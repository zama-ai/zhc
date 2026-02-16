//! TFHE large integer semantic model.
//!
//! Summary
//! =======
//!
//! This module contains an algebra of objects that emulates the TFHE large integer semantics. To
//! operate on large integers, TFHE decomposes encrypted values of arbitrary size in multiple LWE
//! ciphertexts using a fixed radix decomposition. The elements of this decomposition are referred
//! to as __blocks__, and are emulated by [EmulatedCiphertextBlock]. Those blocks are then assembled
//! in an __integer__ emulated by [EmulatedCiphertext].
//!
//! A __block__ can be modelled by a fixed-precision integer separated in different
//! contiguous regions `[ padding_bit | carry_bits | message_bits ]`:
//! + The `message_size` LSBs encode the message bits. A 128-bits __block__ would need
//!   `128.div_ceil(message_size)` blocks to encode values.
//! + The `carry_size` bits above encode the carry bits. Those carry bits allow to store the carries
//!   of intermediate computations.
//! + The MSB encodes the padding bit. By nature, PBSes require nega-cyclic lookup-tables. Arbitrary
//!   lookups can be performed, at the cost of an extra padding bit set to zero to ensure only the
//!   first half of the table is reached.
//!
//! An __integer__ is then, just a sequence of __blocks__, whose `message_bits` regions
//! can be aggregated in order to reconstruct the large encoded value. Of paramount importance is
//! the fact that only __clean__ blocks can be set in an __integer__. Indeed, the encoding does not
//! automatically propagate the carries: It is up to the users to propagates carries between blocks
//! before aggregating the ciphertext.
//!
//! Operations with plaintext values can be emulated as well. The [EmulatedPlaintext] and
//! [EmulatedPlaintextBlock] structures mirror the radix decomposition of ciphertexts.
//!
//! Operation flavors
//! =================
//!
//! Depending on the integer-level operation being implemented, different flavors of operations may
//! be needed at the block-level:
//! + The user may want to protect the padding bit, hence ensuring a swift lookup in PBSes.
//! + The user may want to set the padding bit, when executing an negacyclic lookup.
//! + The user may want to rely on the overflow/underflow of the whole block, to implement signed
//!   integer semantics for instance.
//!
//! To accommodate for the different use cases, we propose three flavors of linear operations:
//! + `protect_*` prefixed operations ensure that operand padding bits are zero and that the padding
//!   bit is not written during execution.
//! + `temper_*` prefixed operations allows arbitrary operand padding bits and ensure that the
//!   padding bit does not overflow/underflow during execution.
//! + `wrapping_*` prefixed operations allows arbitrary operand padding bits and overflow/underflow.
//!
//! Lookup semantics
//! ================
//!
//! Table lookups (programmable bootstrapping) also come in two flavors, reflecting how the padding
//! bit affects the lookup behavior:
//!
//! + [`lut::protect_lookup`] requires the padding bit to be zero. The lookup index is guaranteed to
//!   fall within the first half of the negacyclic table, so the LUT function is applied directly.
//!   This is the standard mode when the padding bit serves as a guard.
//!
//! + [`lut::wrapping_lookup`] handles inputs with an arbitrary padding bit. When the padding bit is
//!   zero, the LUT is applied directly. When it is set, the output is negated (two's complement) to
//!   emulate the negacyclic structure: accessing the second half of the table returns the negation
//!   of the corresponding first-half entry. Use this mode for operations that exploit negacyclic
//!   semantics or when the padding bit may be set due to prior arithmetic.

use rand::SeedableRng;
use rand::rngs::SmallRng;
use std::cell::RefCell;

thread_local! {
    static PRNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(0));
}

pub mod lut;

mod ciphertext;
mod ciphertext_block;
mod ops;
mod plaintext;
mod plaintext_block;

pub use ciphertext::*;
pub use ciphertext_block::*;
pub use plaintext::*;
pub use plaintext_block::*;

#[cfg(test)]
mod test;
