//! IR dialect definitions for the ZHC compiler.
//!
//! Every dialect implements the `zhc_ir` [`Dialect`](zhc_ir::Dialect)
//! trait, binding a type system and instruction set together. The
//! shared entry point is [`ioplang`] (block-level FHE operations on
//! radix ciphertexts), which then lowers along two target-specific
//! paths:
//!
//! * **HPU path** — [`hpulang`] (virtual-register operations with explicit I/O and PBS batching) →
//!   [`doplang`] (the flat hardware ISA of the HPU with inline operands and physical register
//!   assignments).
//! * **VM path** — [`vmlang`] (software-VM register operations compiled to
//!   [`VmByteCode`](vmlang::VmByteCode)).
//!
//! [`pipelinelang`] is an auxiliary dialect used for compilation pipeline
//! construction.

pub mod coarselang;
pub mod doplang;
pub mod hpulang;
pub mod ioplang;
pub mod pipelinelang;
pub mod vmlang;
pub use zhc_ir::visualization::layoutlang;
