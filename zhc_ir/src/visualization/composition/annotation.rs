use zhc_utils::small::SmallVec;

use crate::{OpMap, ValMap};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionVariable {
    Op {
        sol: VariableCell,
        args: SmallVec<VariableCell>,
        rets: SmallVec<VariableCell>,
        body: VariableCell,
        comment: Option<VariableCell>,
    },
    Dummy {
        sol: VariableCell,
    },
    GroupInput {
        sol: VariableCell,
    },
    GroupOutput {
        sol: VariableCell,
    },
    Group {
        sol: VariableCell,
        inputs: SmallVec<VariableCell>,
        outputs: SmallVec<VariableCell>,
        maps: (OpMap<CompositionVariable>, ValMap<()>),
    },
}
