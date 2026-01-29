use std::{fmt::Display, hash::Hash, ops::Deref};

use hc_utils::FastSet;

use crate::Printer;

use super::{Depth, Dialect, IR, OpId, Signature, State, ValId, val_ref::ValRef};

/// A reference to an operation within an IR graph.
///
/// Provides access to operation metadata, arguments, return values, and graph
/// traversal methods. The reference is tied to the lifetime of the IR it
/// references and maintains cached pointers to operation data for efficient access.
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

impl<'s, D: Dialect> Display for OpRef<'s, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let printer = Printer::from_ir(self.ir, crate::PrintWalker::Linear, true, true);
            printer.format_opref(f, self)
        } else {
            let printer = Printer::from_ir(self.ir, crate::PrintWalker::Topo, true, true);
            printer.format_opref(f, self)
        }
    }
}

impl<'s, D: Dialect> Hash for OpRef<'s, D> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<'s, D: Dialect> PartialEq for OpRef<'s, D> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.ir, other.ir) && self.id == other.id
    }
}

impl<'s, D: Dialect> Eq for OpRef<'s, D> {}

impl<'s, D: Dialect> Deref for OpRef<'s, D> {
    type Target = OpId;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<'s, D: Dialect> OpRef<'s, D> {
    /// Returns an iterator over the operation's argument values without state checking.
    pub(super) fn raw_get_args_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> + use<'s, D> {
        self.args.iter().map(|valid| self.ir.raw_get_val(*valid))
    }

    /// Returns an iterator over the operation's return values without state checking.
    pub(super) fn raw_get_returns_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> + use<'s, D> {
        self.returns.iter().map(|valid| self.ir.raw_get_val(*valid))
    }
}

impl<'s, D: Dialect> OpRef<'s, D> {
    /// Checks if the operation is active.
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Checks if the operation is inactive.
    pub fn is_inactive(&self) -> bool {
        self.state.is_inactive()
    }

    /// Checks if the operation is an input operation.
    ///
    /// An input operation is one that takes no arguments.
    pub fn is_input(&self) -> bool {
        self.signature.get_args_arity() == 0
    }

    /// Checks if the operation is an effect operation.
    ///
    /// An effect operation is one that produces no return values.
    pub fn is_effect(&self) -> bool {
        self.signature.get_returns_arity() == 0
    }

    /// Returns the unique identifier of the operation.
    pub fn get_id(&self) -> OpId {
        self.id
    }

    /// Returns a copy of the operation's dialect-specific data.
    pub fn get_operation(&self) -> D::Operations {
        self.operation.clone()
    }

    /// Returns the depth of the operation within the IR graph from the inputs.
    pub fn get_depth(&self) -> Depth {
        *self.depth
    }

    /// Returns an iterator over the operation's argument values.
    pub fn get_args_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> + use<'s, D> {
        self.args.iter().map(|valid| self.ir.get_val(*valid))
    }

    /// Returns the argument value IDs as a slice.
    pub fn get_arg_valids(&self) -> &[ValId] {
        self.args
    }

    /// Returns the number of argument vals.
    pub fn get_args_arity(&self) -> usize {
        self.signature.get_args_arity()
    }

    /// Returns an iterator over the operation's return values.
    pub fn get_returns_iter(&self) -> impl Iterator<Item = ValRef<'s, D>> + use<'s, D> {
        self.returns.iter().map(|valid| self.ir.get_val(*valid))
    }

    /// Returns the return value IDs as a slice.
    pub fn get_return_valids(&self) -> &[ValId] {
        self.returns
    }

    /// Returns the number of return vals.
    pub fn get_return_arity(&self) -> usize {
        self.signature.get_returns_arity()
    }

    /// Returns an iterator over the direct users of the current operation.
    ///
    /// Users are deduplicated, meaning that if an operation uses multiple
    /// return values from this operation, it will appear only once in the
    /// iterator.
    pub fn get_users_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D>{
        let mut raw_users = self
            .get_returns_iter()
            .flat_map(|r| r.get_users_iter().map(|a| a.get_id()))
            .collect::<Vec<OpId>>();
        raw_users.sort_unstable();
        raw_users.dedup();
        raw_users.into_iter().map(|a| self.ir.get_op(a))
    }

    /// Checks if the operation has any users.
    pub fn has_users(&self) -> bool {
        self.get_returns_iter().any(|r| r.has_users())
    }

    /// Returns an iterator over the direct predecessors of the current operation.
    ///
    /// Predecessors are deduplicated, meaning that if a predecessor produces
    /// multiple return values used by this operation, it will appear only once
    /// in the iterator.
    pub fn get_predecessors_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D>{
        let mut raw_predecessors = self
            .get_args_iter()
            .map(|r| r.get_origin().opref.get_id())
            .collect::<Vec<_>>();
        raw_predecessors.sort_unstable();
        raw_predecessors.dedup();
        raw_predecessors.into_iter().map(|a| self.ir.get_op(a))
    }

    /// Returns an iterator over all operations that can reach the current operation.
    ///
    /// Performs a backward traversal through the operation graph, collecting all
    /// operations that directly or indirectly produce values used by this operation.
    /// Operations are deduplicated in the result set.
    pub fn get_reaching_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D>{
        let mut output = FastSet::new();
        let mut worklist = vec![self.clone()];
        while let Some(val) = worklist.pop() {
            for op in val.get_args_iter().map(|a| a.get_origin().opref) {
                output.insert(op.clone());
                worklist.push(op);
            }
        }
        output.into_iter()
    }

    /// Returns an iterator over all operations that can reach the current operation, including
    /// itself.
    ///
    /// Combines the results of `get_reached_iter()` with the current operation to provide
    /// a complete set of all operations in the forward reachability cone starting from
    /// this operation.
    pub fn get_inc_reaching_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D>{
        self.get_reaching_iter()
            .chain(std::iter::once(self.to_owned()))
    }

    /// Returns an iterator over all operations that can be reached from the current operation.
    ///
    /// Performs a forward traversal through the operation graph, collecting all
    /// operations that directly or indirectly use values produced by this operation.
    /// Operations are deduplicated in the result set.
    pub fn get_reached_iter(&self) -> impl Iterator<Item = OpRef<'s, D>> + use<'s, D>{
        let mut output = FastSet::new();
        let mut worklist = vec![self.clone()];
        while let Some(val) = worklist.pop() {
            for op in val.get_returns_iter().flat_map(|a| a.get_users_iter()) {
                output.insert(op.clone());
                worklist.push(op);
            }
        }
        output.into_iter()
    }

    /// Checks if this operation can reach the specified `other` operation.
    ///
    /// Returns `true` if this operation produces values that are directly or
    /// indirectly used by `other`, or if this operation and `other` are the
    /// same operation. Uses depth information to optimize the search when possible.
    pub fn reaches<'o>(&self, other: &OpRef<'o, D>) -> bool {
        if self == other {
            return true;
        }
        // We try to leverage the depth to make the reachability analysis faster.
        if self.get_depth() >= other.get_depth() {
            // The other can not be reached for sure -> Its depth would be strictly larger.
            return false;
        }
        self.get_users_iter()
            .any(|a| a.get_id() == other.get_id() || a.reaches(other))
    }
}
