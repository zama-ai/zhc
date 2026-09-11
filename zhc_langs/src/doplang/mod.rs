//! DOP dialect — the hardware ISA of the HPU.
//!
//! Unlike the higher-level IOP and HPU dialects, DOP instructions map
//! one-to-one to hardware opcodes executed by the HPU datapath. Each
//! instruction carries its operands inline as concretely-typed fields
//! ([`CtReg`], [`CtMem`], [`PtArg`], [`LutRef`], [`UserFlag`], [`VirtId`])
//! rather than referencing SSA values, making the DOP stream a flat
//! sequence of fully-resolved machine operations. A field names exactly the
//! kind(s) of operand it accepts — e.g. `ADD`'s operands are all bare
//! [`CtReg`], so nothing else can be constructed there — rather than
//! relying on a runtime check to reject the wrong kind after the fact.
//!
//! Nothing here gathers every kind into one gathering "any operand" enum:
//! the two places that once needed that (the asm parser's token
//! classification, and `zhc_sim`'s instruction scheduler comparing operands
//! across arbitrary instructions for hazard tracking) each keep their own
//! private version, since neither is a concern of the dialect definition
//! itself.
//!
//! [`CtMem`] supports two stream modes: *unpatched* streams contain
//! symbolic variables ([`CtSrcVar`], [`PtSrcVar`]) that the microcontroller
//! patches at load time into physical addresses, while *patched* streams
//! carry resolved memory addresses ([`CtHeap`], [`CtIo`]) and constant
//! immediates ([`PtConst`]). This duality allows the same representation to
//! serve both program generation and execution trace loading.
//!
//! Instructions are classified by [`Affinity`] into four pipeline
//! lanes: ALU (register arithmetic), memory (load/store), PBS
//! (programmable bootstrapping), and control (synchronization). The
//! scheduler uses affinity to dispatch instructions to the
//! corresponding hardware functional unit.

mod assembly;
mod dialect;
mod evaluation;
mod instruction_set;
mod spills;
mod type_system;

pub use assembly::*;
pub use dialect::*;
pub use evaluation::*;
pub use instruction_set::*;
pub use spills::*;
pub use type_system::*;
