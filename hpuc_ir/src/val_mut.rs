use hpuc_utils::SmallVec;

use super::{Dialect, OpId, State, ValId};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ValMut<'s, D: Dialect> {
    pub id: ValId,
    pub users: &'s mut SmallVec<OpId>,
    pub origin: &'s mut OpId,
    pub typ: &'s mut D::Types,
    pub state: &'s mut State,
}
