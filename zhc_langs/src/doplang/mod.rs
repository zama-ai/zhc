//! DOP dialect — the hardware ISA of the HPU.
//!
//! Unlike the higher-level IOP and HPU dialects, DOP instructions map
//! one-to-one to hardware opcodes executed by the HPU datapath. Each
//! instruction carries its operands inline as [`Argument`] values
//! rather than referencing SSA values, making the DOP stream a flat
//! sequence of fully-resolved machine operations.
//!
//! The dialect supports two stream modes through the [`Argument`] enum:
//! *unpatched* streams contain symbolic variables ([`CtVar`](Argument::CtVar),
//! [`PtVar`](Argument::PtVar)) that the microcontroller patches at
//! load time into physical addresses, while *patched* streams carry
//! resolved memory addresses ([`CtHeap`](Argument::CtHeap),
//! [`CtIo`](Argument::CtIo)) and constant immediates
//! ([`PtConst`](Argument::PtConst)). This duality allows the same
//! representation to serve both program generation and execution trace
//! loading.
//!
//! Instructions are classified by [`Affinity`] into four pipeline
//! lanes: ALU (register arithmetic), memory (load/store), PBS
//! (programmable bootstrapping), and control (synchronization). The
//! scheduler uses affinity to dispatch instructions to the
//! corresponding hardware functional unit.

mod dialect;
mod instruction_set;
mod type_system;

pub use dialect::*;
pub use instruction_set::*;
pub use type_system::*;
