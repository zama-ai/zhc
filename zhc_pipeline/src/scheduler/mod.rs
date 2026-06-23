pub mod one_step;
pub mod one_step_mh;
pub mod two_step;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedPolicy {
    AsSoonAsPossible,
    AsLateAsPossible,
}
