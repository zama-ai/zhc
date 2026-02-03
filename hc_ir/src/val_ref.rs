use std::hash::Hash;
use std::ops::Deref;

use super::{Dialect, IR, State, ValId};
use crate::val_use::ValUse;
use crate::{OpRef, ValOrigin, ValOriginRef, ValRefFormatter, ValUseRef};
use hc_utils::iter::Deduped;

/// A reference to a value within an IR graph.
///
/// Provides access to value metadata, type information, and graph
/// relationships. The reference is tied to the lifetime of the IR it
/// references and maintains cached pointers to value data for efficient access.
#[derive(Debug, Clone)]
pub struct ValRef<'ir, D: Dialect> {
    pub(super) id: ValId,
    pub(super) ir: &'ir IR<D>,
    pub(super) users: &'ir [ValUse],
    pub(super) origin: &'ir ValOrigin,
    pub(super) typ: &'ir D::TypeSystem,
    pub(super) state: &'ir State,
}

impl<'ir, D: Dialect> Hash for ValRef<'ir, D> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<'ir, D: Dialect> PartialEq for ValRef<'ir, D> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.ir, other.ir) && self.id == other.id
    }
}

impl<'ir, D: Dialect> Eq for ValRef<'ir, D> {}

impl<'ir, D: Dialect> Deref for ValRef<'ir, D> {
    type Target = ValId;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

#[allow(unused)]
impl<'ir, D: Dialect> ValRef<'ir, D> {
    pub(super) fn raw_get_uses_iter(
        &self,
    ) -> impl Iterator<Item = ValUseRef<'ir, D>> + use<'ir, D> {
        self.users.iter().map(|uze| ValUseRef {
            opref: self.ir.raw_get_op(uze.opid),
            position: uze.position,
        })
    }

    pub(super) fn raw_get_origin(&self) -> ValOriginRef<'ir, D> {
        ValOriginRef {
            opref: self.ir.raw_get_op(self.origin.opid),
            position: self.origin.position,
        }
    }
}

impl<'ir, D: Dialect> ValRef<'ir, D> {
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
    pub fn get_type(&self) -> D::TypeSystem {
        self.typ.clone()
    }

    /// Returns a reference to the operation that produces this value.
    pub fn get_origin(&self) -> ValOriginRef<'ir, D> {
        let output = self.raw_get_origin();
        assert!(output.opref.is_active());
        output
    }

    pub fn get_uses_iter(&self) -> impl Iterator<Item = ValUseRef<'ir, D>> + use<'ir, D> {
        self.raw_get_uses_iter().filter(|u| u.opref.is_active())
    }

    /// Returns an iterator over operations that use this value as an argument.
    ///
    /// Only active operations are included in the result, and operations are
    /// deduplicated even if they use this value multiple times.
    pub fn get_users_iter(&self) -> impl Iterator<Item = OpRef<'ir, D>> + use<'ir, D> {
        self.raw_get_uses_iter()
            .map(|uze| uze.opref)
            .filter(|u| u.is_active())
            .dedup()
    }

    /// Returns `true` if any active operations use this value as an argument.
    pub fn has_users(&self) -> bool {
        self.get_users_iter().next().is_some()
    }

    pub fn format(&self) -> ValRefFormatter<'_, 'ir, D> {
        ValRefFormatter::new(self)
    }
}
