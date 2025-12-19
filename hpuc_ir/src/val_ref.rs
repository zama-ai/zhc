use std::fmt::Display;
use std::hash::Hash;

use crate::Printer;

use super::{Dialect, IR, OpId, OpRef, State, ValId};

/// A reference to a value within an IR graph.
///
/// Provides access to value metadata, type information, and graph
/// relationships. The reference is tied to the lifetime of the IR it
/// references and maintains cached pointers to value data for efficient access.
#[derive(Debug, Clone)]
pub struct ValRef<'s, D: Dialect> {
    pub(super) id: ValId,
    pub(super) ir: &'s IR<D>,
    pub(super) users: &'s [OpId],
    pub(super) origin: &'s OpId,
    pub(super) typ: &'s D::Types,
    pub(super) state: &'s State,
}

impl<'s, D: Dialect> Display for ValRef<'s, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let printer = Printer::from_ir(self.ir, crate::PrintWalker::Linear, true, true);
            printer.format_arg(f, self.to_owned())
        } else {
            let printer = Printer::from_ir(self.ir, crate::PrintWalker::Topo, true, true);
            printer.format_arg(f, self.to_owned())
        }
    }
}

impl<'s, D: Dialect> Hash for ValRef<'s, D> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<'s, D: Dialect> PartialEq for ValRef<'s, D> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.ir, other.ir) && self.id == other.id
    }
}

impl<'s, D: Dialect> Eq for ValRef<'s, D> {}


#[allow(unused)]
impl<'s, D: Dialect> ValRef<'s, D> {
    pub(super) fn raw_get_users_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D> {
        self.users.iter().map(|opid| self.ir.raw_get_op(*opid))
    }

    pub(super) fn raw_get_origin(&self) -> OpRef<'s, D> {
        self.ir.raw_get_op(*self.origin)
    }
}

impl<'s, D: Dialect> ValRef<'s, D> {
    /// Checks if the value is active.
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Checks if the value is inactive.
    pub fn is_inactive(&self) -> bool {
        self.state.is_inactive()
    }

    /// Returns the unique identifier of the value.
    pub fn get_id(&self) -> ValId {
        self.id
    }

    /// Returns the type of the value according to the dialect's type system.
    pub fn get_type(&self) -> D::Types {
        self.typ.clone()
    }

    /// Returns a reference to the operation that produces this value.
    pub fn get_origin(&self) -> OpRef<'s, D> {
        self.ir.get_op(*self.origin)
    }

    /// Returns an iterator over operations that use this value as an argument.
    ///
    /// Only active operations are included in the result, and operations are
    /// deduplicated even if they use this value multiple times.
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

    /// Returns `true` if any active operations use this value as an argument.
    pub fn has_users(&self) -> bool {
        self.get_users_iter().next().is_some()
    }
}
