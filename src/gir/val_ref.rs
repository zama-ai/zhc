use super::{Dialect, IR, OpId, OpRef, State, ValId};

#[derive(Debug, Clone)]
pub struct ValRef<'s, D: Dialect> {
    pub(super) id: ValId,
    pub(super) ir: &'s IR<D>,
    pub(super) users: &'s [OpId],
    pub(super) origin: &'s OpId,
    pub(super) typ: &'s D::Types,
    pub(super) state: &'s State,
}

impl<'s, D: Dialect> PartialEq for ValRef<'s, D> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.ir, other.ir) && self.id == other.id
    }
}

#[allow(unused)]
impl<'s, D: Dialect> ValRef<'s, D> {
    pub(super) fn raw_get_users_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D> {
        self.users.into_iter().map(|opid| self.ir.raw_get_op(*opid))
    }

    pub(super) fn raw_get_origin(&self) -> OpRef<'s, D> {
        self.ir.raw_get_op(*self.origin)
    }
}

impl<'s, D: Dialect> ValRef<'s, D> {
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn is_inactive(&self) -> bool {
        self.state.is_inactive()
    }

    pub fn get_id(&self) -> ValId {
        self.id
    }

    pub fn get_type(&self) -> D::Types {
        self.typ.clone()
    }

    pub fn get_origin(&self) -> OpRef<'s, D> {
        self.ir.get_op(*self.origin)
    }

    pub fn get_users_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D> {
        let mut raw_users = self
            .raw_get_users_iter()
            .filter(|u| u.is_active())
            .map(|o| o.get_id())
            .collect::<Vec<OpId>>();
        raw_users.sort_unstable();
        raw_users.dedup();
        raw_users.into_iter().map(|a| self.ir.get_op(a))
    }

    pub fn has_users(&self) -> bool {
        self.get_users_iter().next().is_some()
    }
}
