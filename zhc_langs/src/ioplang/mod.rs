//! Intermediate Operation (IOP) dialect for the ZHC compiler IR.
//!
//! This dialect models FHE computations at the block-operation level, sitting
//! between the high-level builder API and the hardware-oriented HPU/DOP
//! dialects. Programs in this dialect operate on multi-block ciphertexts and
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
//! PBS operations apply lookup tables defined by the [`Lut1Def`] and [`Lut2Def`] enums, which
//! enumerate all available single- and two-output table functions. The dialect supports
//! CSE via the [`AllowCse`](zhc_ir::cse::AllowCse) trait, normalizing
//! commutative addition operand order.
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
mod instruction_set;
mod interpretation;
mod lut;
mod skip_store_load;
mod type_system;

pub use dialect::*;
pub use eliminate_aliases::*;
pub use instruction_set::*;
pub use interpretation::*;
pub use lut::*;
pub use skip_store_load::*;
pub use type_system::*;
