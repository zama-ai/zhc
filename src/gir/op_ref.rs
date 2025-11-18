use super::{Depth, Dialect, IR, OpId, Signature, State, ValId, val_ref::ValRef};

#[derive(Debug, Clone)]
pub struct OpRef<'s, D: Dialect> {
    pub(super) id: OpId,
    pub(super) ir: &'s IR<D>,
    pub(super) operation: &'s D::Operations,
    pub(super) signature: &'s Signature<D::Types>,
    pub(super) args: &'s [ValId],
    pub(super) returns: &'s [ValId],
    pub(super) state: &'s State,
    pub(super) depth: &'s Depth,
}

impl<'s, D: Dialect> PartialEq for OpRef<'s, D> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.ir, other.ir) && self.id == other.id
    }
}

impl<'s, D: Dialect> OpRef<'s, D> {
    pub(super) fn raw_get_args_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> {
        self.args.iter().map(|valid| self.ir.raw_get_val(*valid))
    }

    pub(super) fn raw_get_returns_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> + use<'s, D> {
        self.returns.iter().map(|valid| self.ir.raw_get_val(*valid))
    }
}

impl<'s, D: Dialect> OpRef<'s, D> {
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn is_inactive(&self) -> bool {
        self.state.is_inactive()
    }

    pub fn is_input(&self) -> bool {
        self.signature.get_args_arity() == 0
    }

    pub fn is_effect(&self) -> bool {
        self.signature.get_returns_arity() == 0
    }

    pub fn get_id(&self) -> OpId {
        self.id
    }

    pub fn get_operation(&self) -> D::Operations {
        self.operation.clone()
    }

    pub fn get_depth(&self) -> Depth {
        *self.depth
    }

    pub fn get_args_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> {
        self.args.iter().map(|valid| self.ir.get_val(*valid))
    }

    pub fn get_returns_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> + use<'s, D> {
        self.returns.iter().map(|valid| self.ir.get_val(*valid))
    }

    /// Returns an iterator over the direct users of the current operation.
    ///
    /// Note:
    /// =====
    ///
    /// Users are deduplicated. This means that if a using op takes multiple args defined by the
    /// same op, it will appear only once in the iterator.
    pub fn get_users_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> {
        let mut raw_users = self
            .get_returns_iter()
            .flat_map(|r| r.get_users_iter().map(|a| a.get_id()))
            .collect::<Vec<OpId>>();
        raw_users.sort_unstable();
        raw_users.dedup();
        raw_users.into_iter().map(|a| self.ir.get_op(a))
    }

    pub fn has_users(&self) -> bool {
        self.get_returns_iter().any(|r| r.has_users())
    }

    /// Returns an iterator over the direct predecessors of the current operation.
    ///
    /// Note:
    /// =====
    ///
    /// Predecessors are deduplicated. This means that if a predecessor produces multiple return
    /// values used by the current op, it will appear only once in the iterator.
    pub fn get_predecessors_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> {
        let mut raw_predecessors = self
            .get_args_iter()
            .map(|r| r.get_origin().get_id())
            .collect::<Vec<_>>();
        raw_predecessors.sort_unstable();
        raw_predecessors.dedup();
        raw_predecessors.into_iter().map(|a| self.ir.get_op(a))
    }

    /// Test if an other operation is reachable starting from the current operation.
    pub fn reaches<'o>(&self, other: OpRef<'o, D>) -> bool {
        if *self == other {
            return true;
        }
        // We try to leverage the depth to make the reachability analysis faster.
        if self.get_depth() >= other.get_depth() {
            // The other can not be reached for sure -> Its depth would be strictly larger.
            return false;
        }
        self.get_users_iter()
            .any(|a| a.get_id() == other.get_id() || a.reaches(other.clone()))
    }
}
