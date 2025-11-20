use hpuc_utils::SmallVec;

use super::{Dialect, OpId, State};

#[derive(Debug)]
pub struct Val<D: Dialect> {
    pub users: SmallVec<OpId>,
    pub origin: OpId,
    pub typ: D::Types,
    pub state: State,
}
