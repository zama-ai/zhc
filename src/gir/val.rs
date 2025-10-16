use crate::utils::SmallVec;

use super::{Dialect, OpId, State};

#[derive(Debug)]
pub(super) struct Val<D: Dialect> {
    pub(super) users: SmallVec<OpId>,
    pub(super) origin: OpId,
    pub(super) typ: D::Types,
    pub(super) state: State,
}
