use crate::utils::SmallVec;

use super::{Depth, Dialect, OpId, Signature, State, ValId};

#[allow(dead_code)]
pub(super) struct OpMut<'s, D: Dialect> {
    pub(super) id: OpId,
    pub(super) operation: &'s mut D::Operations,
    pub(super) signature: &'s mut Signature<D::Types>,
    pub(super) args: &'s mut SmallVec<ValId>,
    pub(super) returns: &'s mut SmallVec<ValId>,
    pub(super) state: &'s mut State,
    pub(super) depth: &'s mut Depth,
}
