//! HPU dialect for the ZHC compiler IR.
//!
//! This dialect models computation at the register level of the HPU,
//! sitting between the block-level IOP dialect and the fully-allocated
//! DOP instruction stream. Operations consume and produce virtual
//! registers ([`CtRegister`](HpuTypeSystem::CtRegister)) and plaintext
//! immediates ([`PtImmediate`](HpuTypeSystem::PtImmediate)), with
//! explicit load/store operations for I/O memory transfer.
//!
//! Operand identifiers ([`TSrcId`], [`TDstId`], [`TImmId`]) encode the
//! positional slot and block index of ciphertext inputs, ciphertext
//! outputs, and plaintext inputs respectively, bridging the IOP-level
//! positional naming to the hardware memory layout. [`LutId`] provides
//! a numeric handle for lookup tables, mapped from the symbolic
//! [`Lut1Def`](crate::ioplang::Lut1Def)/[`Lut2Def`](crate::ioplang::Lut2Def)
//! names during IOP-to-HPU translation. [`Immediate`] wraps a `u8`
//! plaintext constant inlined into instructions.
//!
//! PBS operations come in regular and flush variants (`Pbs`/`PbsF`,
//! `Pbs2`/`Pbs2F`, etc.). The flush variants mark the end of a PBS
//! batch; the batcher groups consecutive PBS operations between flush
//! markers into [`Batch`](HpuInstructionSet::Batch) blocks that carry
//! a nested `IR<HpuLang>` sub-program. [`BatchArg`](HpuInstructionSet::BatchArg)
//! and [`BatchRet`](HpuInstructionSet::BatchRet) define the batch
//! boundary interface inside the nested IR.

mod dialect;
mod instruction_set;
mod type_system;

pub use dialect::*;
pub use instruction_set::*;
pub use type_system::*;
