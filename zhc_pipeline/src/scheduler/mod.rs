use serde::Serialize;

pub mod one_step;
pub mod two_step;
pub mod vm;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SchedPolicy {
    AsSoonAsPossible,
    AsLateAsPossible,
}
