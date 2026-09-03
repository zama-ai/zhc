//! Circuit builder for fully homomorphic encryption (FHE) programs.
//!
//! This crate exposes the [`Builder`] type, a high-level interface for constructing FHE
//! circuits as intermediate representation (IR) graphs. A circuit takes encrypted and plaintext
//! integer inputs, applies arithmetic operations and programmable bootstrapping (PBS) lookups
//! on individual blocks, and produces encrypted outputs.
//!
//! The four value types — [`Ciphertext`], [`CiphertextBlock`], [`Plaintext`], and
//! [`PlaintextBlock`] — are opaque handles into the IR graph. They cannot be inspected
//! directly; instead, they are passed to [`Builder`] methods that emit the corresponding IR
//! instructions.
//!
//! # Radix Decomposition
//!
//! Large encrypted integers are represented using a **radix decomposition**: an integer of
//! `int_size` message bits is split into `int_size / message_size` blocks, each carrying
//! `message_size` bits of payload. For example, with
//! `message_size = 2`, an 8-bit integer is decomposed into 4 blocks, each encoding a
//! base-4 digit.
//!
//! Each [`CiphertextBlock`] also reserves `carry_size` extra bits above the message to
//! absorb carries from arithmetic operations. A programmable bootstrapping (PBS) lookup
//! can then be used to propagate carries and extract the message, restoring the block to a
//! canonical state. The bit layout of a block, from MSB to LSB, is:
//!
//! ```text
//!  ┌─────────┬────────────┬─────────┐
//!  │ padding │   carry    │ message │
//!  │ (1 bit) │  (c bits)  │ (m bits)│
//!  └─────────┴────────────┴─────────┘
//!   MSB                          LSB
//! ```
//!
//! The [`CiphertextBlockSpec`] captures the `(carry_size, message_size)` pair and is shared
//! by every block in a circuit. Plaintext blocks follow the same radix but have no carry or
//! padding bits — only the `message_size` message bits.
//!
//! All block-level operations (`block_*` methods) work on individual blocks, while
//! multi-block integers must first be [`split`](Builder::ciphertext_split) into their radix
//! digits and later [`join`](Builder::ciphertext_join)ed back.
//!
//! # Operation Flavors
//!
//! Depending on the integer-level operation being implemented, different flavors of
//! block-level arithmetic may be needed:
//!
//! - The user may want to **protect** the padding bit, ensuring a swift (non-negacyclic) lookup in
//!   PBSes.
//! - The user may want to **set** the padding bit, when executing a negacyclic lookup.
//! - The user may want to rely on the **overflow/underflow** of the whole block, to implement
//!   signed integer semantics for instance.
//!
//! To accommodate these use cases, block-level operations come in three flavors, modelled by
//! [`Flavor`]:
//!
//! - **`protect`** — operand padding bits must be zero, and the result must not overflow into the
//!   padding bit. This is the default and most common flavor.
//! - **`temper`** — operand padding bits may be arbitrary, but the result must not
//!   overflow/underflow *past* the padding bit.
//! - **`wrapping`** — operand padding bits may be arbitrary, and overflow/underflow is
//!   unrestricted. Similar to Rust's `wrapping_add` / `wrapping_sub` on integers.
//!
//! Every linear block operation (`add`, `sub`, `shl`, `mac`, `pack`, `add_plaintext`,
//! `sub_plaintext`, `plaintext_sub`, `mul_plaintext`) exists in all three flavors. Unless
//! explicited in their name, [`Builder`] arithmetic methods use the **protect** flavor. Methods
//! that use a different flavor are explicitly marked (e.g.
//! [`block_wrapping_add_plaintext`](Builder::block_wrapping_add_plaintext)), and every operation
//! also has a `*_with` form taking the [`Flavor`] as a runtime argument (e.g.
//! [`block_add_with`](Builder::block_add_with)). [`block_neg`](Builder::block_neg) is the only
//! linear operation without a flavor: negation is inherently wrapping.
//!
//! # Lookup Checks
//!
//! A programmable bootstrapping evaluates a lookup table on the data bits of a block. Because
//! TFHE tables are negacyclic, an input with its padding bit set reads the negated second half
//! of the table. The [`LookupCheck`] policy attached to every PBS states which padding bits
//! the interpreter asserts clear:
//!
//! - [`Protect`](LookupCheck::Protect) — input and output padding bits must be clear. This is the
//!   default of [`block_lookup`](Builder::block_lookup) and its many-LUT siblings.
//! - [`AllowOutputPadding`](LookupCheck::AllowOutputPadding) — the table may write the padding bit.
//!   Used by [`block_padding_lookup`](Builder::block_padding_lookup).
//! - [`AllowInputPadding`](LookupCheck::AllowInputPadding) — the input may have its padding bit
//!   set, triggering negacyclic negation, but the output must be clean.
//! - [`AllowBothPadding`](LookupCheck::AllowBothPadding) — no assertion at all. Used by
//!   [`block_wrapping_lookup`](Builder::block_wrapping_lookup).
//!
//! The `*_with` lookup methods ([`block_lookup_with`](Builder::block_lookup_with), ...) take the
//! policy as an argument.
//!
//! PBS lookups come in four widths. [`block_lookup`](Builder::block_lookup) produces one output
//! block; [`block_lookup2`](Builder::block_lookup2), [`block_lookup4`](Builder::block_lookup4)
//! and [`block_lookup8`](Builder::block_lookup8) are *many-LUT* bootstrappings producing 2, 4
//! or 8 blocks from a single input. A many-LUT of `2^k` outputs reserves the `k` topmost data
//! bits of its input for the table index, so the input must be small enough, and only the
//! `Protect` and `AllowOutputPadding` checks are meaningful. Tables are named by the
//! [`Lut1Def`], [`Lut2Def`], [`Lut4Def`] and [`Lut8Def`] enums, which also accept custom
//! functions through their `custom` constructor.
//!
//! # Typical Workflow
//!
//! ```rust,no_run
//! # use zhc_builder::*;
//! // 1. Create a builder for a given block spec.
//! let builder = Builder::new(CiphertextBlockSpec(2, 2));
//!
//! // 2. Declare circuit inputs.
//! let a = builder.ciphertext_input(8);
//! let b = builder.ciphertext_input(8);
//!
//! // 3. Decompose into blocks and operate.
//! let a_blocks = builder.ciphertext_split(&a);
//! let b_blocks = builder.ciphertext_split(&b);
//! let sum_blocks: Vec<_> = a_blocks.iter().zip(b_blocks.iter())
//!     .map(|(ab, bb)| builder.block_add(ab, bb))
//!     .collect();
//!
//! // 4. Reassemble and declare the output.
//! let result = builder.ciphertext_join(&sum_blocks, None);
//! builder.ciphertext_output(&result);
//!
//! // 5. Finalize — this runs dead-code elimination and CSE.
//! let ir = builder.optimize_ir();
//! ```

const NU: usize = 5;
const NU_BOOL: usize = 8;

mod builder;
mod iops;

pub use builder::*;
pub use iops::*;

#[cfg(test)]
mod test;
