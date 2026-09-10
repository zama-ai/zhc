use std::fmt::{Debug, Display};
use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature};
use zhc_utils::svec;

use crate::tasklang::TaskTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskInstructionSet {
    Task { inp_size: u16, oup_size: u16, work: u32 },
}

impl TaskInstructionSet {
    pub fn get_work(&self) -> u32 {
        match self {
            TaskInstructionSet::Task { work, .. } => *work,
        }
    }
}

impl Format for TaskInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        match self {
            TaskInstructionSet::Task { .. } => write!(f, "task"),
        }
    }
}

impl Display for TaskInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DialectInstructionSet for TaskInstructionSet {
    type TypeSystem = TaskTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use TaskInstructionSet::*;
        match self {
            Task { inp_size, oup_size, .. } => Signature(
                svec![TaskTypeSystem::Val; *inp_size as usize],
                svec![TaskTypeSystem::Val; *oup_size as usize],
            ),
        }
    }
}
