//! Intermediate Operation (IOP) dialect for the ZHC compiler IR.
//!
//! This dialect models FHE computations at the block-operation level, sitting
//! between the high-level builder API and the target-specific backend
//! dialects (HPU/DOP for hardware, VM for software). Programs in this
//! dialect operate on multi-block ciphertexts and
//! plaintexts, with explicit block extraction, storage, arithmetic, and
//! programmable bootstrapping (PBS) instructions.
//!
//! [`IopLang`] is the dialect tag binding [`IopTypeSystem`] to
//! [`IopInstructionSet`]. The type system distinguishes composite values
//! ([`Ciphertext`](IopTypeSystem::Ciphertext),
//! [`Plaintext`](IopTypeSystem::Plaintext)) from their individual blocks
//! ([`CiphertextBlock`](IopTypeSystem::CiphertextBlock),
//! [`PlaintextBlock`](IopTypeSystem::PlaintextBlock)). Arithmetic and PBS
//! instructions operate exclusively on blocks; composite values are
//! disassembled and reassembled via extract/store operations.
//!
//! Linear block instructions carry a
//! [`Flavor`](zhc_crypto::integer_semantics::Flavor) selecting their overflow
//! policy (protect, temper, wrapping), mirroring the `zhc_crypto` operation
//! flavors one-to-one. PBS instructions exist in 1-, 2-, 4- and 8-output
//! variants, each carrying a
//! [`LookupCheck`](zhc_crypto::integer_semantics::lut::LookupCheck) policy and a
//! table built from the [`Lut1Def`], [`Lut2Def`], [`Lut4Def`] or [`Lut8Def`]
//! enums. The dialect supports CSE via the [`AllowCse`](zhc_ir::cse::AllowCse)
//! trait, normalizing commutative addition operand order.
//!
//! Two dialect-specific optimization passes are provided:
//! [`eliminate_aliases`] removes identity-forwarding [`Inspect`](IopInstructionSet::Inspect)
//! operations, and [`skip_store_load`] eliminates redundant
//! store-then-extract round-trips on ciphertext blocks.
//!
//! [`IopValue`] and [`IopInterepreterContext`] support emulated execution
//! of IOP programs via the `zhc_ir` interpretation framework, enabling
//! semantic validation against the `zhc_crypto` emulation layer.

mod dialect;
mod eliminate_aliases;
mod evaluation;
mod instruction_set;
mod lut;
mod skip_redundant_stores;
mod skip_store_load;
mod type_system;

pub use dialect::*;
pub use eliminate_aliases::*;
pub use evaluation::*;
pub use instruction_set::*;
pub use lut::*;
pub use skip_redundant_stores::*;
pub use skip_store_load::*;
pub use type_system::*;
