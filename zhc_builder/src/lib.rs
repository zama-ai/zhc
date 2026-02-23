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
//! multi-block integers must first be [`split`](Builder::split_ciphertext) into their radix
//! digits and later [`join`](Builder::join_ciphertext)ed back.
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
//! To accommodate these use cases, block-level operations come in three flavors:
//!
//! - **`protect`** — operand padding bits must be zero, and the result must not overflow into the
//!   padding bit. This is the default and most common flavor.
//! - **`temper`** — operand padding bits may be arbitrary, but the result must not
//!   overflow/underflow *past* the padding bit.
//! - **`wrapping`** — operand padding bits may be arbitrary, and overflow/underflow is
//!   unrestricted. Similar to Rust's `wrapping_add` / `wrapping_sub` on integers.
//!
//! Unless explicited in their name, [`Builder`] arithmetic methods use the **protect** flavor.
//! Methods that use a different flavor are explicitly marked (e.g.
//! [`block_wrapping_add_plaintext`](Builder::block_wrapping_add_plaintext)).
//!
//! # Typical Workflow
//!
//! ```rust,no_run
//! # use zhc_builder::*;
//! // 1. Create a builder for a given block spec.
//! let builder = Builder::new(CiphertextBlockSpec(2, 2));
//!
//! // 2. Declare circuit inputs.
//! let a = builder.input_ciphertext(8);
//! let b = builder.input_ciphertext(8);
//!
//! // 3. Decompose into blocks and operate.
//! let a_blocks = builder.split_ciphertext(&a);
//! let b_blocks = builder.split_ciphertext(&b);
//! let sum_blocks: Vec<_> = a_blocks.iter().zip(b_blocks.iter())
//!     .map(|(ab, bb)| builder.block_add(ab, bb))
//!     .collect();
//!
//! // 4. Reassemble and declare the output.
//! let result = builder.join_ciphertext(&sum_blocks, None);
//! builder.output_ciphertext(&result);
//!
//! // 5. Finalize — this runs dead-code elimination and CSE.
//! let ir = builder.into_ir();
//! ```

const NU: usize = 5;
const NU_BOOL: usize = 8;

mod builder;
mod iops;

pub use builder::*;
pub use iops::*;
