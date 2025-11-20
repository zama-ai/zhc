use hpuc_utils::SmallVec;

use super::{Dialect, OpId, State, ValId};

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct ValMut<'s, D: Dialect> {
    pub(super) id: ValId,
    pub(super) users: &'s mut SmallVec<OpId>,
    pub(super) origin: &'s mut OpId,
    pub(super) typ: &'s mut D::Types,
    pub(super) state: &'s mut State,
}
