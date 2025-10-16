use crate::utils::SmallVec;

use super::{Depth, Dialect, State, ValId};

pub(super) struct Op<D: Dialect> {
    pub(super) operation: D::Operations,
    pub(super) args: SmallVec<ValId>,
    pub(super) returns: SmallVec<ValId>,
    pub(super) state: State,
    pub(super) depth: Depth,
}
