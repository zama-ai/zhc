use crate::interpretation::{
    InterpState, Interpretable, Interpretation, InterpretsTo, interpret_ir,
};
use crate::val_ref::ValRef;
use crate::{AnnIR, Annotation, IRFormatter, ValMap, ValOrigin, ValUse};
use std::{cmp::max, fmt::Debug};
use zhc_utils::iter::MultiZip;
use zhc_utils::svec;
use zhc_utils::{Store, small::SmallVec};

use super::{
    Dialect, DialectInstructionSet, IRError, Op, OpId, OpIdRaw, OpMut, OpRef, Signature, State,
    Val, ValId, ValIdRaw, ValMut, op_map::OpMap,
};

pub type Depth = u16;

fn val_active<'a, D: Dialect>(val: &ValRef<'a, D>) -> bool {
    val.is_active()
}

fn op_active<'a, D: Dialect>(op: &OpRef<'a, D>) -> bool {
    op.is_active()
}

/// The main intermediate representation structure for a dataflow graph.
///
/// Maintains the complete graph of operations and values for a program, providing
/// efficient access to operation metadata, value usage information, and graph
/// traversal capabilities. The IR enforces type safety through the dialect system
/// and maintains structural integrity through automatic bookkeeping.
#[derive(Clone)]
pub struct IR<D: Dialect> {
    pub(super) op_operations: Store<OpId, D::InstructionSet>,
    pub(super) op_signatures: Store<OpId, Signature<D::TypeSystem>>,
    pub(super) op_arguments: Store<OpId, SmallVec<ValId>>,
    pub(super) op_returns: Store<OpId, SmallVec<ValId>>,
    pub(super) op_states: Store<OpId, State>,
    pub(super) op_depth: Store<OpId, Depth>,
    pub(super) op_comments: Store<OpId, Option<String>>,
    pub(super) op_count: OpIdRaw,
    pub(super) val_users: Store<ValId, SmallVec<ValUse>>,
    pub(super) val_origins: Store<ValId, ValOrigin>,
    pub(super) val_types: Store<ValId, D::TypeSystem>,
    pub(super) val_states: Store<ValId, State>,
    pub(super) val_count: ValIdRaw,
}

impl<D: Dialect> Debug for IR<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct(&format!("IR<{}>", std::any::type_name::<D>()))
                .field("n_ops", &self.n_ops())
                .field("n_vals", &self.n_vals())
                .finish()
        } else {
            write!(f, "IR<{}>", std::any::type_name::<D>())
        }
    }
}

impl<D: Dialect> PartialEq for IR<D> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl<D: Dialect> Eq for IR<D> {}

impl<D: Dialect> std::hash::Hash for IR<D> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}

// This impl block contains the private implementations
#[allow(unused)]
impl<D: Dialect> IR<D> {
    pub(crate) fn raw_n_ops(&self) -> OpIdRaw {
        self.op_states.len()
    }

    pub(crate) fn raw_has_opid(&self, opid: OpId) -> bool {
        opid.0 < self.raw_n_ops()
    }

    pub(crate) fn raw_get_op(&self, opid: OpId) -> OpRef<'_, D> {
        assert!(self.raw_has_opid(opid), "Unknown opid");
        OpRef {
            ir: self,
            operation: &self.op_operations[opid],
            signature: &self.op_signatures[opid],
            args: self.op_arguments[opid].as_slice(),
            returns: self.op_returns[opid].as_slice(),
            id: opid,
            state: &self.op_states[opid],
            depth: &self.op_depth[opid],
            comment: &self.op_comments[opid],
        }
    }

    pub(crate) fn raw_get_op_mut(&mut self, opid: OpId) -> OpMut<'_, D> {
        assert!(self.raw_has_opid(opid), "Unknown opid");
        OpMut {
            instruction: &mut self.op_operations[opid],
            signature: &mut self.op_signatures[opid],
            args: &mut self.op_arguments[opid],
            returns: &mut self.op_returns[opid],
            id: opid,
            state: &mut self.op_states[opid],
            depth: &mut self.op_depth[opid],
            comment: &mut self.op_comments[opid],
        }
    }

    pub(crate) fn raw_n_vals(&self) -> ValIdRaw {
        self.val_states.len()
    }

    pub(crate) fn raw_has_valid(&self, valid: ValId) -> bool {
        valid.0 < self.raw_n_vals()
    }

    pub(crate) fn raw_get_val(&self, valid: ValId) -> ValRef<'_, D> {
        assert!(self.raw_has_valid(valid), "Unkown valid");
        ValRef {
            id: valid,
            ir: self,
            users: self.val_users[valid].as_slice(),
            origin: &self.val_origins[valid],
            typ: &self.val_types[valid],
            state: &self.val_states[valid],
        }
    }

    pub(crate) fn raw_get_val_mut(&mut self, valid: ValId) -> ValMut<'_, D> {
        assert!(self.raw_has_valid(valid), "Unkown valid");
        ValMut {
            id: valid,
            users: &mut self.val_users[valid],
            origin: &mut self.val_origins[valid],
            typ: &mut self.val_types[valid],
            state: &mut self.val_states[valid],
        }
    }

    pub(crate) fn depth(&self) -> Depth {
        *self.op_depth.iter().max().unwrap_or(&0)
    }

    pub(crate) fn raw_linear_opwalker(&self) -> impl DoubleEndedIterator<Item = OpId> {
        OpId::range(0, self.raw_n_ops())
    }

    pub(crate) fn raw_topological_opwalker(&self) -> impl DoubleEndedIterator<Item = OpId> {
        let max_depth = *self.op_depth.iter().max().unwrap_or(&0);
        let mut depth_buckets = vec![svec![]; (max_depth + 1) as usize];
        for op in self.raw_walk_ops_linear() {
            depth_buckets[op.get_depth() as usize].push(op.get_id());
        }
        depth_buckets.into_iter().flat_map(|b| b.into_iter())
    }

    pub(crate) fn raw_walk_ops(
        &self,
        walker: impl Iterator<Item = OpId>,
    ) -> impl Iterator<Item = OpRef<'_, D>> {
        walker.map(|opid| self.raw_get_op(opid))
    }

    pub(crate) fn raw_walk_ops_linear(&self) -> impl DoubleEndedIterator<Item = OpRef<'_, D>> {
        self.raw_linear_opwalker().map(|opid| self.raw_get_op(opid))
    }

    pub(crate) fn raw_walk_ops_topo(&self) -> impl DoubleEndedIterator<Item = OpRef<'_, D>> {
        self.raw_topological_opwalker()
            .map(|opid| self.raw_get_op(opid))
    }

    pub(crate) fn raw_linear_valwalker(&self) -> impl DoubleEndedIterator<Item = ValId> {
        ValId::range(0, self.raw_n_vals())
    }

    pub(crate) fn raw_walk_vals(
        &self,
        walker: impl Iterator<Item = ValId>,
    ) -> impl Iterator<Item = ValRef<'_, D>> {
        walker.map(|valid| self.raw_get_val(valid))
    }

    pub(crate) fn raw_walk_vals_linear(&self) -> impl DoubleEndedIterator<Item = ValRef<'_, D>> {
        self.raw_linear_valwalker()
            .map(|valid| self.raw_get_val(valid))
    }

    pub(crate) fn raw_insert_op(&mut self, op: Op<D>) -> OpId {
        let opid = OpId(self.raw_n_ops());
        let Op {
            instruction: operation,
            signature,
            args,
            returns,
            state,
            depth,
            comment,
        } = op;
        self.op_operations.push(operation);
        self.op_signatures.push(signature);
        self.op_arguments.push(args);
        self.op_returns.push(returns);
        self.op_states.push(state);
        self.op_depth.push(depth);
        self.op_comments.push(comment);
        self.op_count += 1;
        opid
    }

    pub(crate) fn raw_insert_val(&mut self, val: Val<D>) -> ValId {
        let valid = ValId(self.raw_n_vals());
        let Val {
            users,
            origin,
            typ,
            state,
        } = val;
        self.val_users.push(users);
        self.val_origins.push(origin);
        self.val_types.push(typ);
        self.val_states.push(state);
        self.val_count += 1;
        valid
    }

    // This static method allows to recursively update the depths of operations on mutation. In
    // theory, it _could_ be implemented as a mutable method, but the bck does not manage to prove
    // the code is valid on recursion. Which it is since we borrow different fields with different
    // mutability. For this reason, we have to rely on a static method that works on the different
    // fields directly.
    fn raw_update_depths(
        op_depth: &mut Store<OpId, Depth>,
        op_args: &Store<OpId, SmallVec<ValId>>,
        op_returns: &Store<OpId, SmallVec<ValId>>,
        val_origin: &Store<ValId, ValOrigin>,
        val_users: &Store<ValId, SmallVec<ValUse>>,
        opid: OpId,
    ) {
        let current_depth = op_depth[opid];
        let mut new_depth = 1;
        for arg in op_args[opid].iter() {
            new_depth = max(op_depth[val_origin[arg].opid] + 1, new_depth);
        }
        if current_depth != new_depth {
            op_depth[opid] = new_depth;
            for valid in op_returns[opid].iter() {
                for user in val_users[valid].iter() {
                    Self::raw_update_depths(
                        op_depth, op_args, op_returns, val_origin, val_users, user.opid,
                    );
                }
            }
        }
    }
}

// Public API
impl<D: Dialect> IR<D> {
    /// Creates a new empty IR with no operations or values.
    pub fn empty() -> Self {
        IR {
            op_operations: Store::empty(),
            op_signatures: Store::empty(),
            op_arguments: Store::empty(),
            op_returns: Store::empty(),
            op_states: Store::empty(),
            op_depth: Store::empty(),
            op_comments: Store::empty(),
            op_count: 0,
            val_users: Store::empty(),
            val_origins: Store::empty(),
            val_types: Store::empty(),
            val_states: Store::empty(),
            val_count: 0,
        }
    }

    /// Returns the total number of active operations in the IR.
    pub fn n_ops(&self) -> OpIdRaw {
        self.op_count
    }

    /// Returns `true` if the specified operation ID exists and is active.
    pub fn has_opid(&self, opid: OpId) -> bool {
        self.raw_has_opid(opid) && self.raw_get_op(opid).is_active()
    }

    /// Returns the total number of active values in the IR.
    pub fn n_vals(&self) -> ValIdRaw {
        self.val_count
    }

    /// Returns `true` if the specified value ID exists and is active.
    pub fn has_valid(&self, valid: ValId) -> bool {
        self.raw_has_valid(valid) && self.raw_get_val(valid).is_active()
    }

    /// Returns a reference to the specified active operation.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID does not exist or refers to an inactive operation.
    pub fn get_op(&self, opid: OpId) -> OpRef<'_, D> {
        let op = self.raw_get_op(opid);
        assert!(op.is_active(), "Tried to get a dead op");
        op
    }

    /// Returns a reference to the specified active value.
    ///
    /// # Panics
    ///
    /// Panics if the value ID does not exist or refers to an inactive value.
    pub fn get_val(&self, valid: ValId) -> ValRef<'_, D> {
        let val = self.raw_get_val(valid);
        assert!(val.is_active(), "Tried to get a dead val");
        val
    }

    /// Returns an iterator over all active operations in linear order.
    ///
    /// Operations are yielded in the order they were added to the IR.
    pub fn walk_ops_linear(&self) -> impl DoubleEndedIterator<Item = OpRef<'_, D>> {
        self.raw_walk_ops_linear().filter(op_active)
    }

    /// Returns an iterator over all active operations in topological order.
    ///
    /// Operations are yielded such that all dependencies of an operation
    /// are visited before the operation itself.
    pub fn walk_ops_topological(&self) -> impl DoubleEndedIterator<Item = OpRef<'_, D>> {
        self.raw_walk_ops_topo().filter(op_active)
    }

    /// Returns an iterator over operations using a custom walker.
    ///
    /// The `walker` provides the order in which operation IDs are visited,
    /// and this method maps those IDs to their corresponding operation references.
    pub fn walk_ops_with(
        &self,
        walker: impl Iterator<Item = OpId>,
    ) -> impl Iterator<Item = OpRef<'_, D>> {
        walker.map(|opid| self.get_op(opid))
    }

    /// Returns an iterator over all active values in linear order.
    ///
    /// Values are yielded in the order they were added to the IR.
    pub fn walk_vals_linear(&self) -> impl DoubleEndedIterator<Item = ValRef<'_, D>> {
        self.raw_walk_vals_linear().filter(val_active)
    }

    /// Returns an iterator over values using a custom walker.
    ///
    /// The `walker` provides the order in which value IDs are visited,
    /// and this method maps those IDs to their corresponding value references.
    pub fn walk_vals_with(
        &self,
        walker: impl Iterator<Item = ValId>,
    ) -> impl Iterator<Item = ValRef<'_, D>> {
        walker.map(|valid| self.get_val(valid))
    }

    /// Applies a mutation function to all active operations in linear order.
    pub fn mutate_ops(&mut self, f: impl FnMut(&mut D::InstructionSet)) {
        self.mutate_ops_with_walker(
            self.raw_linear_opwalker()
                .collect::<SmallVec<_>>()
                .into_iter(),
            f,
        );
    }

    /// Applies a mutation function to operations visited by the specified walker.
    ///
    /// Only active operations are mutated; inactive operations are skipped.
    pub fn mutate_ops_with_walker(
        &mut self,
        walker: impl Iterator<Item = OpId>,
        mut f: impl FnMut(&mut D::InstructionSet),
    ) {
        walker.for_each(|opid| {
            let opmut = self.raw_get_op_mut(opid);
            if opmut.state.is_active() {
                f(opmut.instruction);
            };
        });
    }

    /// Adds a new operation to the IR with the specified arguments.
    ///
    /// Returns the ID of the created operation and the IDs of its return values.
    /// The operation's signature is validated against the argument types, and
    /// use-def chains are automatically updated.
    ///
    /// # Panics
    ///
    /// Panics if any argument value ID is invalid or inactive, or if depth
    /// computation overflows.
    pub fn add_op(
        &mut self,
        op: D::InstructionSet,
        args: SmallVec<ValId>,
    ) -> Result<(OpId, SmallVec<ValId>), IRError<D>> {
        self.add_op_impl(op, args, None)
    }

    /// Adds a new operation to the IR with the specified arguments and a comment.
    ///
    /// Same as `add_op`, but attaches a comment to the operation for debugging.
    pub fn add_op_with_comment(
        &mut self,
        op: D::InstructionSet,
        args: SmallVec<ValId>,
        comment: String,
    ) -> Result<(OpId, SmallVec<ValId>), IRError<D>> {
        self.add_op_impl(op, args, Some(comment))
    }

    fn add_op_impl(
        &mut self,
        op: D::InstructionSet,
        args: SmallVec<ValId>,
        comment: Option<String>,
    ) -> Result<(OpId, SmallVec<ValId>), IRError<D>> {
        // Check that the args are live.
        args.iter().for_each(|valid| {
            assert!(self.has_valid(*valid), "Unknown valid");
        });
        // Check that the signature matches the arguments types.
        let sig = op.get_signature();
        let actual = args
            .iter()
            .map(|a| self.get_val(*a).get_type())
            .collect::<Vec<_>>();
        if sig.get_args() != actual {
            return Err(IRError::OpSig {
                op,
                recv: actual,
                exp: sig.get_args().into(),
            });
        }

        // We compute the depth from the inputs.
        let arg_depth = args
            .iter()
            .map(|a| self.get_val(*a).get_origin().opref.get_depth())
            .max();
        let depth = if arg_depth.is_none() {
            0
        } else {
            let (d, overflow) = arg_depth.unwrap().overflowing_add(1);
            if overflow {
                panic!("Overflow occured while computing the depth of a new operation.");
            }
            d
        };

        // Now we are ready to mutate the various stores.

        // We begin by adding the op. Note that for now the return values do not exist, and will be
        // added once created.
        let op = Op {
            instruction: op,
            signature: sig.clone(),
            args: args.clone(),
            returns: svec![],
            state: State::Active(()),
            depth,
            comment,
        };
        let opid = self.raw_insert_op(op);

        // We update the arg users list to add the newly created operation
        for (i, arg) in args.iter().enumerate() {
            let arg = self.raw_get_val_mut(*arg);
            arg.users.push(ValUse {
                opid,
                position: i as u8,
            });
        }

        // Now we can add new values for each return value of the operation.
        let valids = sig
            .into_returns()
            .into_iter()
            .enumerate()
            .map(|(i, ty)| {
                let ret = Val {
                    users: svec![],
                    origin: ValOrigin {
                        opid,
                        position: i as u8,
                    },
                    typ: ty,
                    state: State::Active(()),
                };
                self.raw_insert_val(ret)
            })
            .collect::<SmallVec<_>>();

        // We update the op returns according to our newly created values
        self.raw_get_op_mut(opid)
            .returns
            .extend(valids.as_slice().iter().cloned());

        // All good
        Ok((opid, valids))
    }

    /// Replace all the uses of a value by another one.
    ///
    /// Panics:
    /// =======
    ///
    /// If the new value has a different type.
    /// If the new value is reachable from one of the users.
    pub fn replace_val_use(&mut self, old: ValId, new: ValId) {
        assert!(self.has_valid(old), "Unknown valid.");
        assert!(self.has_valid(new), "Unknown valid.");
        if old == new {
            return;
        };
        let old = self.raw_get_val(old);
        let new = self.raw_get_val(new);

        // We check that the two values have compatible types.
        assert_eq!(
            old.get_type(),
            new.get_type(),
            "Tried to replace a value with one of different type."
        );

        // Now we are going to check that the new value is not reachable by any user. That would
        // mean a cycle is introduced by the mutation.
        for user in old.get_users_iter() {
            if user.reaches(&new.get_origin().opref) {
                panic!("Tried to replace a value with one it reaches.");
            }
        }

        // The replace is valid. We can now proceed with the mutation.
        let old = old.get_id();
        let new = new.get_id();

        // Update the arguments of the users of old, with new instead.
        for user in self.val_users[old].iter() {
            self.op_arguments[user.opid].iter_mut().for_each(|a| {
                if *a == old {
                    *a = new
                }
            });
        }

        // We update the depths of the reached operations.
        for user in self.val_users[old].iter() {
            Self::raw_update_depths(
                &mut self.op_depth,
                &self.op_arguments,
                &self.op_returns,
                &self.val_origins,
                &self.val_users,
                user.opid,
            );
        }

        // Drain the old users into the new users.
        let [old_mut, new_mut] = self.val_users.get_disjoint_mut([old, new]);
        new_mut.append(old_mut);
    }

    /// Deletes multiple operations in dependency-safe order.
    ///
    /// Operations are deleted in reverse topological order to ensure that
    /// dependencies are deleted before their users.
    pub fn batch_delete_op(&mut self, opids: impl Iterator<Item = OpId>) {
        let mut batch: Vec<_> = opids
            .map(|opid| (opid, self.get_op(opid).get_depth()))
            .collect();
        batch.sort_unstable_by_key(|(_, depth)| *depth);
        batch
            .into_iter()
            .rev()
            .for_each(|(opid, _)| self.delete_op(opid));
    }

    /// Deletes an operation from the IR.
    ///
    /// Panics:
    /// =======
    ///
    /// If the operation has active users, the operation will panic.
    pub fn delete_op(&mut self, opid: OpId) {
        assert!(
            !self.raw_get_op(opid).is_inactive(),
            "Tried to delete an already inactive operation"
        );
        assert!(
            !self.get_op(opid).has_users(),
            "Tried to delete an operation whose return values are still in use."
        );
        for valid in self.op_returns[opid].iter().cloned() {
            self.val_states[valid].shutdown();
            // Decrementing the val count is valid since `shutdown` panics if the value is already
            // shutdown.
            self.val_count -= 1;
        }
        self.op_states[opid].shutdown();
        // Decrementing the op count is valid since `shutdown` panics if the value is already
        // shutdown.
        self.op_count -= 1;
    }

    /// Dump the IR and stop the program.
    /// Prints the IR to stdout and panics.
    ///
    /// This is a debugging utility that displays the current IR state
    /// before terminating the program.
    pub fn dump(&self) {
        println!("{}", self.format());
        panic!();
    }

    /// Creates an empty operation map for this IR.
    pub fn empty_opmap<V>(&self) -> OpMap<V> {
        OpMap::new_empty(self)
    }

    /// Creates an operation map filled with the specified value for all operations.
    pub fn filled_opmap<V: Clone>(&self, v: V) -> OpMap<V> {
        OpMap::new_filled(self, v)
    }

    /// Creates an operation map by applying a function to each operation.
    ///
    /// The function returns `None` for operations that should not have entries.
    pub fn partially_mapped_opmap<V>(&self, f: impl FnMut(OpRef<D>) -> Option<V>) -> OpMap<V> {
        OpMap::new_partially_mapped(self, f)
    }

    /// Creates an operation map by applying a function to each operation.
    ///
    /// All operations will have entries in the resulting map.
    pub fn totally_mapped_opmap<V>(&self, f: impl FnMut(OpRef<D>) -> V) -> OpMap<V> {
        OpMap::new_totally_mapped(self, f)
    }

    /// Creates an empty value map for this IR.
    pub fn empty_valmap<V>(&self) -> ValMap<V> {
        ValMap::new_empty(self)
    }

    /// Creates a value map filled with the specified value for all values.
    pub fn filled_valmap<V: Clone>(&self, v: V) -> ValMap<V> {
        ValMap::new_filled(self, v)
    }

    /// Creates a value map by applying a function to each value.
    ///
    /// The function returns `None` for values that should not have entries.
    pub fn partially_mapped_valmap<V>(&self, f: impl FnMut(ValRef<D>) -> Option<V>) -> ValMap<V> {
        ValMap::new_partially_mapped(self, f)
    }

    /// Creates a value map by applying a function to each value.
    ///
    /// All values will have entries in the resulting map.
    pub fn totally_mapped_valmap<V>(&self, f: impl FnMut(ValRef<D>) -> V) -> ValMap<V> {
        ValMap::new_totally_mapped(self, f)
    }

    /// Performs backward dataflow analysis on the IR operations.
    pub fn backward_dataflow_analysis<OpAnn: Annotation, ValAnn: Annotation>(
        &self,
        mut f: impl FnMut(&OpMap<OpAnn>, &ValMap<ValAnn>, &OpRef<D>) -> (OpAnn, SmallVec<ValAnn>),
    ) -> AnnIR<'_, D, OpAnn, ValAnn> {
        let mut opmap = self.empty_opmap();
        let mut valmap = self.empty_valmap();
        for opref in self.walk_ops_topological().rev() {
            assert!(opref.get_users_iter().all(|k| opmap.contains_key(&k)));
            let (opann, valanns) = f(&opmap, &valmap, &opref);
            assert_eq!(valanns.len(), opref.get_return_valids().len());
            assert!(opmap.insert(*opref, opann).is_none());
            for (valann, valref) in (valanns.into_iter(), opref.get_return_valids().iter()).mzip() {
                assert!(valmap.insert(*valref, valann).is_none());
            }
        }
        AnnIR::new(self, opmap, valmap)
    }

    /// Performs forward dataflow analysis on the IR operations.
    pub fn forward_dataflow_analysis<OpAnn: Annotation, ValAnn: Annotation>(
        &self,
        mut f: impl FnMut(&OpMap<OpAnn>, &ValMap<ValAnn>, &OpRef<D>) -> (OpAnn, SmallVec<ValAnn>),
    ) -> AnnIR<'_, D, OpAnn, ValAnn> {
        let mut opmap = self.empty_opmap();
        let mut valmap = self.empty_valmap();
        for opref in self.walk_ops_topological() {
            assert!(
                opref
                    .get_predecessors_iter()
                    .all(|k| opmap.contains_key(&k))
            );
            let (opann, valanns) = f(&opmap, &valmap, &opref);
            assert_eq!(valanns.len(), opref.get_return_valids().len());
            assert!(opmap.insert(*opref, opann).is_none());
            for (valann, valref) in (valanns.into_iter(), opref.get_return_valids().iter()).mzip() {
                assert!(valmap.insert(*valref, valann).is_none());
            }
        }
        AnnIR::new(self, opmap, valmap)
    }

    /// Interprets the IR with the given context and returns the annotated result.
    ///
    /// Returns `Ok` with the fully interpreted IR and context on success, or `Err`
    /// with partial interpretation state (containing `Panicked` and `Poisoned` markers)
    /// and context on failure.
    pub fn interpret<I: Interpretation>(
        &self,
        mut context: <D::InstructionSet as Interpretable<I>>::Context,
    ) -> Result<
        (
            AnnIR<'_, D, (), I>,
            <D::InstructionSet as Interpretable<I>>::Context,
        ),
        (
            AnnIR<'_, D, (), InterpState<I>>,
            <D::InstructionSet as Interpretable<I>>::Context,
        ),
    >
    where
        D::InstructionSet: Interpretable<I>,
        D::TypeSystem: InterpretsTo<I>,
    {
        match interpret_ir(self, &mut context) {
            Ok(interpreted) => Ok((interpreted, context)),
            Err(partial) => Err((partial, context)),
        }
    }

    pub fn format(&self) -> IRFormatter<'_, D> {
        IRFormatter::new(self)
    }
}
