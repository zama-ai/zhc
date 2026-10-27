//! Circuit builder for fully homomorphic encryption (FHE) programs.
//!
//! This crate exposes the [`Builder`] type, a high-level interface for constructing FHE
//! circuits as intermediate representation (IR) graphs. A circuit takes encrypted and plaintext
//! integer inputs, applies arithmetic operations and programmable bootstrapping (PBS) lookups
//! on individual blocks, and produces encrypted outputs.
//!
//! The four value types — [`CiphertextInteger`], [`CiphertextBlock`], [`PlaintextInteger`], and
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
//! The [`SpecBlock`] captures the `(carry_size, message_size)` pair and is shared
//! by every block in a circuit. Plaintext blocks follow the same radix but have no carry or
//! padding bits — only the `message_size` message bits.
//!
//! All block-level operations (`block_*` methods) work on individual blocks, while
//! multi-block integers must first be [`split`](Builder::ciphertext_integer_split) into their radix
//! digits and later [`join`](Builder::ciphertext_integer_join)ed back.
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
//! [`block_add_with`](Builder::block_add_with)). There is no dedicated negation: negating a
//! block is [`block_wrapping_plaintext_sub`](Builder::block_wrapping_plaintext_sub) from a zero
//! plaintext, which two's-complements the block on its complete width and may flip the
//! padding bit.
//!
//! # Lookup Checks
//!
//! A programmable bootstrapping evaluates a lookup table on the data bits of a block. Because
//! TFHE tables are negacyclic, an input with its padding bit set reads the negated second half
//! of the table. The [`LookupCheck`] policy attached to every PBS states which conditions the
//! interpreter asserts. It has three independent flags:
//!
//! - [`allow_input_padding`](LookupCheck::allow_input_padding) — the input may have its padding bit
//!   set, which negates the outputs.
//! - [`allow_index_bits`](LookupCheck::allow_index_bits) — the input of a many-LUT may have its
//!   index bits set, which rotates the sub-tables (see below). Single-output lookups have no index
//!   bits, so this flag has no effect on them.
//! - [`allow_output_padding`](LookupCheck::allow_output_padding) — the outputs may have their
//!   padding bit set.
//!
//! Two constants cover the common cases, and the builder offers a shortcut for each policy it uses:
//!
//! - [`Protect`](LookupCheck::Protect) — all three flags are false, so every padding bit and index
//!   bit must be clear. This is the policy of [`block_lookup`](Builder::block_lookup) and its
//!   many-LUT siblings.
//! - `LookupCheck { allow_output_padding: true, ..LookupCheck::Protect }` — only the outputs may
//!   have their padding bit set. This is the policy of
//!   [`block_padding_lookup`](Builder::block_padding_lookup).
//! - [`Permissive`](LookupCheck::Permissive) — all three flags are true, so nothing is asserted.
//!   This is the policy of [`block_wrapping_lookup`](Builder::block_wrapping_lookup).
//!
//! The `*_with` lookup methods ([`block_lookup_with`](Builder::block_lookup_with), ...) take the
//! policy as an argument, so any combination of flags can be used.
//!
//! PBS lookups come in four widths. [`block_lookup`](Builder::block_lookup) produces one output
//! block; [`block_lookup2`](Builder::block_lookup2), [`block_lookup4`](Builder::block_lookup4)
//! and [`block_lookup8`](Builder::block_lookup8) are *many-LUT* bootstrappings producing 2, 4
//! or 8 blocks from a single input. A many-LUT of `2^k` outputs reserves the `k` topmost data
//! bits of its input for the table index. When these index bits are clear, output `j` reads
//! sub-table `j`. When they hold a value `q`, the sub-tables are rotated by `q`, and the outputs
//! that wrap around are negated. Most circuits keep the index bits clear. The builder accepts
//! precomputed
//! [`Lut1`], [`Lut2`], [`Lut4`] and [`Lut8`] tables. Use named builtin constructors such as
//! [`Lut1::msg_only`], or `from_fn` to build custom tables from closures, passing the builder's
//! block specification in either case.
//!
//! # Typical Workflow
//!
//! ```rust,no_run
//! # use zhc_builder::*;
//! // 1. Create a builder for a given block spec.
//! let builder = Builder::new(SpecBlock(2, 2));
//!
//! // 2. Declare circuit inputs.
//! let a = builder.ciphertext_integer_input(8);
//! let b = builder.ciphertext_integer_input(8);
//!
//! // 3. Decompose into blocks and operate.
//! let a_blocks = builder.ciphertext_integer_split(&a);
//! let b_blocks = builder.ciphertext_integer_split(&b);
//! let sum_blocks: Vec<_> = a_blocks.iter().zip(b_blocks.iter())
//!     .map(|(ab, bb)| builder.block_add(ab, bb))
//!     .collect();
//!
//! // 4. Reassemble and declare the output.
//! let result = builder.ciphertext_integer_join(&sum_blocks, None);
//! builder.ciphertext_integer_output(&result);
//!
//! // 5. Get the IR. Optimization happens later, in the compilation pipeline.
//! let ir = builder.ir();
//! ```

const NU: usize = 5;
const NU_BOOL: usize = 8;
// A `mul_pt` by a runtime immediate is as noisy as `msg_mask` fresh blocks, the noise analysis
// having no better bound on the digit, so fewer of them may be summed before a lookup.
const NU_MUL_PT: usize = 4;

mod builder;
mod iops;

pub use builder::*;
pub use iops::*;

#[cfg(test)]
mod test;
