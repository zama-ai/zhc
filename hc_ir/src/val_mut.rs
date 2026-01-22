use hc_utils::small::SmallVec;

use crate::{ValOrigin, val_use::ValUse};

use super::{Dialect, State, ValId};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ValMut<'s, D: Dialect> {
    pub id: ValId,
    pub users: &'s mut SmallVec<ValUse>,
    pub origin: &'s mut ValOrigin,
    pub typ: &'s mut D::Types,
    pub state: &'s mut State,
}
