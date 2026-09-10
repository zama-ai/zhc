use std::fmt::{Debug, Display};
use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature};
use zhc_utils::svec;

use crate::coarselang::CoarseTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoarseInstructionSet {
    Op { inp_size: u16, oup_size: u16, work: u32 },
}

impl CoarseInstructionSet {}

impl Format for CoarseInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        match self {
            CoarseInstructionSet::Op { .. } => write!(f, "op"),
        }
    }
}

impl Display for CoarseInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DialectInstructionSet for CoarseInstructionSet {
    type TypeSystem = CoarseTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use CoarseInstructionSet::*;
        match self {
            Op { inp_size, oup_size, .. } => Signature(
                svec![CoarseTypeSystem::Val; *inp_size as usize],
                svec![CoarseTypeSystem::Val; *oup_size as usize],
            ),
        }
    }
}
