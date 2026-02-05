//! A module containing tools to generate traces in the TraceEvent json format.
//!
//! See <https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview> for a specification of the format.

mod categories;
mod events;
mod phtypes;
mod scope;
mod stack;
mod trace;
mod unit;

pub use categories::*;
pub use events::*;
pub use phtypes::*;
pub use scope::*;
pub use stack::*;
pub use trace::*;
pub use unit::*;

pub type Microseconds = f64;
pub type Pid = usize;
pub type Tid = usize;
