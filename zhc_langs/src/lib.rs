//! IR dialect definitions for the ZHC compiler.
//!
//! Provides three dialects that form a lowering chain: [`ioplang`]
//! (block-level FHE operations on radix ciphertexts), [`hpulang`]
//! (register-level virtual-register operations with explicit I/O and
//! PBS batching), and [`doplang`] (the flat hardware ISA of the HPU
//! with inline operands and physical register assignments). Each
//! dialect implements the `zhc_ir` [`Dialect`](zhc_ir::Dialect) trait,
//! binding a type system and instruction set together.

pub mod doplang;
pub mod hpulang;
pub mod ioplang;
