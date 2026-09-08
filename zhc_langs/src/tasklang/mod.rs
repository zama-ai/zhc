//! Task dialect: a graph of units of work and of the dependencies between them.
//!
//! This dialect models a program as a graph of coarse tasks. Each operation is a
//! [`Task`](TaskInstructionSet::Task) that carries a work estimate, and every value has the single
//! type [`Val`](TaskTypeSystem::Val), so the graph keeps only the dependencies between tasks.
//!
//! [`coarsen_ioplang`] builds this graph from an IOP program. The returned translation maps each
//! task back to the IOP operation it comes from, so that a result computed on tasks can be carried
//! back to the IOP program.

mod coarsen;
mod dialect;
mod instruction_set;
mod type_system;

pub use coarsen::*;
pub use dialect::*;
pub use instruction_set::*;
pub use type_system::*;
