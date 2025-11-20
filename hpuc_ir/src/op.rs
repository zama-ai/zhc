use hpuc_utils::SmallVec;

use super::{Depth, Dialect, Signature, State, ValId};

pub struct Op<D: Dialect> {
    pub operation: D::Operations,
    pub signature: Signature<D::Types>,
    pub args: SmallVec<ValId>,
    pub returns: SmallVec<ValId>,
    pub state: State,
    pub depth: Depth,
}
