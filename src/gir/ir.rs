use crate::{
    gir::val_ref::ValRef,
    svec,
    utils::{SmallVec, Store, StoreIndex},
};
use std::{
    cmp::max,
    fmt::{Debug, Display},
    mem::MaybeUninit,
    sync::Arc,
};

use super::{
    Dialect, DialectOperations, IRError, Op, OpId, OpIdRaw, OpMut, OpRef, Printer, Signature, Val,
    ValId, ValIdRaw, ValMut,
};

#[derive(Debug, Clone, Copy)]
pub(super) enum State {
    Active,
    Inactive,
}

impl State {
    pub(super) fn is_active(&self) -> bool {
        matches!(self, State::Active)
    }

    pub(super) fn is_inactive(&self) -> bool {
        matches!(self, State::Inactive)
    }

    pub(super) fn shutdown(&mut self) {
        assert!(
            self.is_active(),
            "Tried to shut an already inactive element"
        );
        *self = State::Inactive
    }
}

pub(super) type Depth = u8;

pub struct IR<D: Dialect> {
    pub(super) op_operations: Store<OpId, D::Operations>,
    pub(super) op_signatures: Store<OpId, Signature<D::Types>>,
    pub(super) op_arguments: Store<OpId, SmallVec<ValId>>,
    pub(super) op_returns: Store<OpId, SmallVec<ValId>>,
    pub(super) op_states: Store<OpId, State>,
    pub(super) op_depth: Store<OpId, Depth>,
    pub(super) op_count: OpIdRaw,
    pub(super) val_users: Store<ValId, SmallVec<OpId>>,
    pub(super) val_origins: Store<ValId, OpId>,
    pub(super) val_types: Store<ValId, D::Types>,
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

// This impl block contains the private implementations
impl<D: Dialect> IR<D> {
    pub(super) fn raw_n_ops(&self) -> OpIdRaw {
        self.op_states.len()
    }

    pub(super) fn raw_has_opid(&self, opid: OpId) -> bool {
        opid.0 < self.raw_n_ops()
    }

    pub(super) fn raw_get_op(&self, opid: OpId) -> OpRef<'_, D> {
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
        }
    }

    pub(super) fn raw_get_op_mut(&mut self, opid: OpId) -> OpMut<'_, D> {
        assert!(self.raw_has_opid(opid), "Unknown opid");
        OpMut {
            operation: &mut self.op_operations[opid],
            signature: &mut self.op_signatures[opid],
            args: &mut self.op_arguments[opid],
            returns: &mut self.op_returns[opid],
            id: opid,
            state: &mut self.op_states[opid],
            depth: &mut self.op_depth[opid],
        }
    }

    pub(super) fn raw_n_vals(&self) -> ValIdRaw {
        self.val_states.len()
    }

    pub(super) fn raw_has_valid(&self, valid: ValId) -> bool {
        valid.0 < self.raw_n_vals()
    }

    pub(super) fn raw_get_val(&self, valid: ValId) -> ValRef<'_, D> {
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

    pub(super) fn raw_get_val_mut(&mut self, valid: ValId) -> ValMut<'_, D> {
        assert!(self.raw_has_valid(valid), "Unkown valid");
        ValMut {
            id: valid,
            users: &mut self.val_users[valid],
            origin: &mut self.val_origins[valid],
            typ: &mut self.val_types[valid],
            state: &mut self.val_states[valid],
        }
    }

    pub(super) fn raw_ops_iter(&self) -> impl Iterator<Item = OpRef<'_, D>> {
        OpId::range(0, self.raw_n_ops()).map(|opid| self.raw_get_op(opid))
    }

    pub(super) fn raw_vals_iter(&self) -> impl Iterator<Item = ValRef<'_, D>> {
        ValId::range(0, self.raw_n_vals()).map(|valid| self.raw_get_val(valid))
    }

    pub(super) fn raw_insert_op(&mut self, op: Op<D>) -> OpId {
        let opid = OpId(self.raw_n_ops());
        let Op {
            operation,
            signature,
            args,
            returns,
            state,
            depth,
        } = op;
        self.op_operations.push(operation);
        self.op_signatures.push(signature);
        self.op_arguments.push(args);
        self.op_returns.push(returns);
        self.op_states.push(state);
        self.op_depth.push(depth);
        self.op_count += 1;
        opid
    }

    pub(super) fn raw_insert_val(&mut self, val: Val<D>) -> ValId {
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

    pub(super) fn raw_get_topological_order(&self) -> impl Iterator<Item = OpId> {
        let max_depth = *self.op_depth.iter().max().unwrap_or(&0);
        let mut depth_buckets = vec![svec![]; (max_depth + 1) as usize];
        for op in self.raw_ops_iter() {
            depth_buckets[op.get_depth() as usize].push(op.get_id());
        }
        depth_buckets.into_iter().flat_map(|b| b.into_iter())
    }

    pub(super) fn raw_topological_ops_iter(&self) -> impl Iterator<Item = OpRef<'_, D>> {
        self.raw_get_topological_order()
            .map(|opid| self.raw_get_op(opid))
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
        val_origin: &Store<ValId, OpId>,
        val_users: &Store<ValId, SmallVec<OpId>>,
        opid: OpId,
    ) {
        let current_depth = op_depth[opid];
        let mut new_depth = 1;
        for arg in op_args[opid].iter() {
            new_depth = max(op_depth[val_origin[arg]] + 1, new_depth);
        }
        if current_depth != new_depth {
            op_depth[opid] = new_depth;
            for valid in op_returns[opid].iter() {
                for user in val_users[valid].iter() {
                    Self::raw_update_depths(
                        op_depth, op_args, op_returns, val_origin, val_users, *user,
                    );
                }
            }
        }
    }
}

// Public API
impl<D: Dialect> IR<D> {
    pub fn empty() -> Self {
        IR {
            op_operations: Store::empty(),
            op_signatures: Store::empty(),
            op_arguments: Store::empty(),
            op_returns: Store::empty(),
            op_states: Store::empty(),
            op_depth: Store::empty(),
            op_count: 0,
            val_users: Store::empty(),
            val_origins: Store::empty(),
            val_types: Store::empty(),
            val_states: Store::empty(),
            val_count: 0,
        }
    }

    pub fn n_ops(&self) -> OpIdRaw {
        self.op_count
    }

    pub fn has_opid(&self, opid: OpId) -> bool {
        self.raw_has_opid(opid) && self.raw_get_op(opid).is_active()
    }

    pub fn n_vals(&self) -> ValIdRaw {
        self.val_count
    }

    pub fn has_valid(&self, valid: ValId) -> bool {
        self.raw_has_valid(valid) && self.raw_get_val(valid).is_active()
    }

    pub fn get_op(&self, opid: OpId) -> OpRef<'_, D> {
        let op = self.raw_get_op(opid);
        assert!(op.is_active(), "Tried to get a dead op");
        op
    }

    pub fn get_val(&self, valid: ValId) -> ValRef<'_, D> {
        let val = self.raw_get_val(valid);
        assert!(val.is_active(), "Tried to get a dead val");
        val
    }

    pub fn ops_iter(&self) -> impl Iterator<Item = OpRef<'_, D>> {
        self.raw_ops_iter().filter(|o| o.is_active())
    }

    pub fn topological_ops_iter(&self) -> impl Iterator<Item = OpRef<'_, D>> {
        self.raw_topological_ops_iter().filter(|o| o.is_active())
    }

    pub fn add_op(
        &mut self,
        op: D::Operations,
        args: SmallVec<ValId>,
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
            .map(|a| self.get_val(*a).get_origin().get_depth())
            .max()
            .unwrap_or(0);
        let (depth, overflow) = arg_depth.overflowing_add(1);
        if overflow {
            panic!("Overflow occured while computing the depth of a new operation.");
        }

        // Now we are ready to mutate the various stores.

        // We begin by adding the op. Note that for now the return values do not exist, and will be
        // added once created.
        let op = Op {
            operation: op,
            signature: sig.clone(),
            args: args.clone(),
            returns: svec![],
            state: State::Active,
            depth,
        };
        let opid = self.raw_insert_op(op);

        // We update the arg users list to add the newly created operation
        for arg in args.iter() {
            let arg = self.raw_get_val_mut(*arg);
            arg.users.push(opid);
        }

        // Now we can add new values for each return value of the operation.
        let valids = sig
            .into_returns()
            .into_iter()
            .map(|ty| {
                let ret = Val {
                    users: svec![],
                    origin: opid,
                    typ: ty,
                    state: State::Active,
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
            if user.reaches(new.get_origin()) {
                panic!("Tried to replace a value with one it reaches.");
            }
        }

        // The replace is valid. We can now proceed with the mutation.
        let old = old.get_id();
        let new = new.get_id();

        // Update the arguments of the users of old, with new instead.
        for user in self.val_users[old].iter() {
            self.op_arguments[user].iter_mut().for_each(|a| {
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
                *user,
            );
        }

        // Drain the old users into the new users.
        let [old_mut, new_mut] = self.val_users.get_disjoint_mut([old, new]);
        new_mut.append(old_mut);
    }

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
    pub fn dump(&self) {
        println!("{}", self);
        panic!();
    }

    #[cfg(test)]
    pub fn check_ir(&self, expected: &str) {
        let clean = |inp: &str| inp.replace(' ', "").replace('\n', "");
        let repr = format!("{}", self);
        if clean(&repr) != clean(expected) {
            println!(
                "Failed to check ir.\nExpected:\n{}\nActual:\n{}",
                expected, repr
            );
            panic!("Failed to check ir");
        }
    }

    pub fn to_contextual_graph(
        self: Arc<Self>,
    ) -> petgraph::stable_graph::StableGraph<(OpId, Arc<Self>), (ValId, Arc<Self>)> {
        use petgraph::stable_graph::*;
        let mut output = StableGraph::new();
        let mut idmap: Vec<MaybeUninit<NodeIndex>> =
            vec![MaybeUninit::uninit(); self.raw_n_ops() as usize];
        self.raw_ops_iter().for_each(|op| {
            idmap[op.get_id().as_usize()] =
                MaybeUninit::new(output.add_node((op.get_id(), self.clone())));
        });
        use std::iter::repeat;
        self.raw_vals_iter()
            .flat_map(|val| {
                repeat(val.get_id())
                    .zip(repeat(val.get_origin()))
                    .zip(val.get_users_iter())
            })
            .for_each(|((valid, from), to)| {
                let from_nix = unsafe { idmap[from.get_id().as_usize()].assume_init() };
                let to_nix = unsafe { idmap[to.get_id().as_usize()].assume_init() };
                output.add_edge(from_nix, to_nix, (valid, self.clone()));
            });
        output
    }
}

impl<D: Dialect> Display for IR<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printer = Printer::from_ir(self, true, true);
        printer.format_ir(f, self)
    }
}
