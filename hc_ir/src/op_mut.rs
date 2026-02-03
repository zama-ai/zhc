use hc_utils::small::SmallVec;

use super::{Depth, Dialect, OpId, Signature, State, ValId};

#[allow(dead_code)]
pub struct OpMut<'s, D: Dialect> {
    pub id: OpId,
    pub operation: &'s mut D::InstructionSet,
    pub signature: &'s mut Signature<D::TypeSystem>,
    pub args: &'s mut SmallVec<ValId>,
    pub returns: &'s mut SmallVec<ValId>,
    pub state: &'s mut State,
    pub depth: &'s mut Depth,
}
