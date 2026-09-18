//! Pull-based IR evaluation.
//!
//! Hosts [`LazyEvaluator`], the driver that evaluates on demand whatever a requested value depends
//! on, and is the scheduling counterpart of [`EagerEvaluator`].

use std::panic::{AssertUnwindSafe, catch_unwind};

use zhc_utils::iter::{CollectInSmallVec, MultiZip};

use super::*;
use crate::{AnnIR, AnnIRView, AsOpId, AsValId, Dialect, IR, OpId, OpMap, ValMap};

const PROFILE: bool = true;

/// Pull-based driver evaluating only the operations a request depends on.
///
/// Holds the evaluation state of every active operation and value of the borrowed [`IR`], but
/// evaluates nothing until a value or an operation is pulled, at which point the dependencies
/// still pending are evaluated first. Operations that have already settled are never re-evaluated,
/// so pulls are idempotent and overlapping requests share the work already done.
pub struct LazyEvaluator<'ir, D: Dialect, V: Evaluation>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    valmap: ValMap<ValState<V>>,
    opmap: OpMap<OpState>,
    ir: &'ir IR<D>,
}

impl<'ir, D: Dialect, V: Evaluation> LazyEvaluator<'ir, D, V>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    /// Consumes the evaluator into an annotated IR holding the evaluation states.
    ///
    /// Preserves the complete outcome of the run, panic messages and poison markers included.
    /// Values never reached by a pull are still pending, which is the normal outcome of a lazy run
    /// and the reason no value-only counterpart to this method exists.
    pub fn into_eval_ir(self) -> AnnIR<'ir, D, OpState, ValState<V>> {
        AnnIR::new(self.ir, self.opmap, self.valmap)
    }

    /// Returns a read-only annotated view of the current evaluation state.
    ///
    /// Borrows the evaluator rather than consuming it, so the states can be inspected in between
    /// pulls.
    pub fn as_view(&self) -> AnnIRView<'ir, '_, D, OpState, ValState<V>> {
        AnnIRView::new(self.ir, &self.opmap, &self.valmap)
    }

    pub fn is_ok(&self) -> bool {
        !self
            .as_view()
            .walk_ops_linear()
            .any(|a| a.get_annotation().is_panic())
    }

    /// Creates an evaluator over an IR, with every operation and value pending.
    ///
    /// Borrows `ir` for the evaluator's lifetime; nothing is evaluated until a pull requests it.
    pub fn from_ir(ir: &'ir IR<D>) -> Self {
        LazyEvaluator {
            valmap: ir.filled_valmap(ValState::Pending),
            opmap: ir.filled_opmap(OpState::Pending),
            ir,
        }
    }

    /// Returns the value bound to the given value id.
    ///
    /// Never triggers evaluation: a value that has not been pulled is reported as pending rather
    /// than computed on the spot.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::PendingValId`] if the operation producing `valid` has not been pulled
    /// yet, [`EvalError::PoisonedValId`] if that operation or one of its predecessors failed, and
    /// [`EvalError::UnknownValId`] if the evaluator holds no state for `valid`.
    ///
    /// # Panics
    ///
    /// Panics if `valid` is out of bounds for the IR or refers to an inactive value.
    pub fn get_val(&self, valid: impl AsValId) -> Result<&V, EvalError> {
        match self.valmap.get(valid) {
            None => Err(EvalError::UnknownValId),
            Some(ValState::Pending) => Err(EvalError::PendingValId),
            Some(ValState::PoisonedBy(opid)) => Err(EvalError::PoisonedValId(*opid)),
            Some(ValState::Evaluated(v)) => Ok(v),
        }
    }

    pub fn into_val(mut self, valid: impl AsValId) -> Result<V, EvalError> {
        match self.valmap.remove(valid) {
            None => Err(EvalError::UnknownValId),
            Some(ValState::Pending) => Err(EvalError::PendingValId),
            Some(ValState::PoisonedBy(opid)) => Err(EvalError::PoisonedValId(opid)),
            Some(ValState::Evaluated(v)) => Ok(v),
        }
    }

    /// Evaluates whatever is needed to produce the given value.
    ///
    /// Resolves `valid` to its producing operation and pulls that operation against `context`. The
    /// value can then be read with [`get_val`](Self::get_val), unless its producer or one of its
    /// predecessors failed.
    ///
    /// # Panics
    ///
    /// Panics if `valid` is out of bounds for the IR, refers to an inactive value, or is produced
    /// by an inactive operation, and for the reasons listed on [`pull_op`](Self::pull_op).
    pub fn pull_val(
        &mut self,
        context: &mut <D::InstructionSet as Evaluable<V>>::Context,
        valid: impl AsValId,
    ) {
        let opid = self.as_view().get_val(valid).get_origin().opref.op_id();
        self.pull_op(context, opid);
    }

    /// Resets the operation that produces `valid`, and every operation transitively downstream
    /// of it, back to [pending](ValState::Pending)/[pending](OpState::Pending).
    ///
    /// Meant to be called after mutating whatever external state an `eval` implementation reads
    /// out-of-band (i.e. from its `Context` rather than its arguments) for the operation that
    /// produces `valid` — most commonly a leaf input operation. Once evaluated, an operation is
    /// never re-run on its own: a settled [`OpState::Evaluated`]/[`OpState::Panicked`]/
    /// [`OpState::PoisonedBy`] is returned as-is by [`pull_op`](Self::pull_op), even if the
    /// context it reads from has since changed. This makes that state stale, and everything
    /// computed from it (transitively, through however many operations) stale as well; this
    /// method un-settles exactly that operation and its downstream, and nothing else, so a
    /// subsequent `pull_val`/`pull_op` recomputes them while operations unrelated to `valid`
    /// keep whatever result they already settled on.
    ///
    /// # Panics
    ///
    /// Panics if `valid` is out of bounds for the IR or refers to an inactive value.
    pub fn invalidate_downstream(&mut self, valid: impl AsValId) {
        let origin = self.ir.get_val(valid).get_origin().opref;
        for op in std::iter::once(origin.clone()).chain(origin.get_reached_iter()) {
            *self.opmap.get_mut(op.op_id()).unwrap() = OpState::Pending;
            for ret_valid in op.get_return_valids() {
                *self.valmap.get_mut(*ret_valid).unwrap() = ValState::Pending;
            }
        }
    }

    /// Evaluates an operation, pulling its pending dependencies first.
    ///
    /// Returns without doing anything if `opid` has already settled. Otherwise each argument still
    /// pending is produced by pulling its own origin, after which the arguments are checked against
    /// the operation's signature, [`eval`](Evaluable::eval) is called with `context`, and the
    /// results are checked and bound to the operation's return values. A panic raised by `eval` is
    /// caught: the operation settles as [`OpState::Panicked`] carrying the message and its results
    /// are poisoned by the operation's own id, whereas an argument that is already poisoned skips
    /// the call entirely and propagates the id carried by the first poisoned argument in signature
    /// order.
    ///
    /// # Panics
    ///
    /// Panics if `opid` does not refer to an active operation of the IR, or if an argument or a
    /// result does not inhabit the type the signature declares for it; unlike an `eval` panic, such
    /// a mismatch propagates out of the driver rather than being recorded, as it indicates a faulty
    /// [`Evaluable`] implementation.
    pub fn pull_op(
        &mut self,
        context: &mut <D::InstructionSet as Evaluable<V>>::Context,
        opid: impl AsOpId,
    ) {
        let ir = self.ir;
        let opid = opid.op_id();

        // First we match the steady states.
        if matches!(
            self.opmap.get(opid).unwrap(),
            OpState::Panicked(_) | OpState::Evaluated(_) | OpState::PoisonedBy(_)
        ) {
            return;
        }

        // Now we proceed with the pending branch.
        let arg_valids = ir.get_op(opid).get_arg_valids().iter().cloned().cosvec();
        let return_valids = ir.get_op(opid).get_return_valids().iter().cloned().cosvec();

        let mut poison: Option<OpId> = None;
        for arg_valid in arg_valids.iter() {
            if matches!(self.valmap.get(arg_valid).unwrap(), ValState::Pending) {
                let origin_opid = ir.get_val(arg_valid).get_origin().opref.get_id();
                self.pull_op(context, origin_opid);
            }
            match self.valmap.get(arg_valid).unwrap() {
                ValState::PoisonedBy(poison_opid) => poison = poison.or(Some(*poison_opid)),
                ValState::Pending => unreachable!(),
                ValState::Evaluated(_) => {
                    // Good to go
                }
            }
        }

        if let Some(poison_opid) = poison {
            *(self.opmap.get_mut(opid).unwrap()) = OpState::PoisonedBy(poison_opid);
            for ret_valid in return_valids.iter() {
                *(self.valmap.get_mut(ret_valid).unwrap()) = ValState::PoisonedBy(poison_opid)
            }
            return;
        }

        // All predecessors are evaluated. We can evaluate the current op.
        let arg_evals = arg_valids
            .iter()
            .map(|a| self.valmap.get(a).unwrap().as_evaluated().unwrap())
            .cosvec();
        let sig = ir.get_op(opid).get_instruction().get_signature();

        // Typecheck the arguments
        for (i, (arg, expected_type)) in
            (arg_evals.iter(), sig.get_args().iter()).mzip().enumerate()
        {
            if !expected_type.is_inhabited_by(arg) {
                panic!(
                    "Unexpected argument type encountered while evaluating {}. \
                     At position {i}, expected type {expected_type}, but encountered {}.",
                    self.ir.get_op(opid).format(),
                    D::TypeSystem::type_of(arg)
                )
            }
        }

        // Eval with panic catching
        let (evaluation, duration) = if PROFILE {
            let tic = std::time::Instant::now();
            let evaluation = catch_unwind(AssertUnwindSafe(|| {
                ir.get_op(opid).get_instruction().eval(context, arg_evals)
            }));
            (evaluation, Some(tic.elapsed()))
        } else {
            let evaluation = catch_unwind(AssertUnwindSafe(|| {
                ir.get_op(opid).get_instruction().eval(context, arg_evals)
            }));
            (evaluation, None)
        };

        match evaluation {
            Err(payload) => {
                let msg = extract_panic_message(payload);
                *(self.opmap.get_mut(opid).unwrap()) = OpState::Panicked(msg);
                for ret_valid in return_valids.iter() {
                    *(self.valmap.get_mut(ret_valid).unwrap()) = ValState::PoisonedBy(opid)
                }
            }
            Ok(ret_evals) => {
                // Typechecks the returns
                for (i, (ret, expected_type)) in (ret_evals.iter(), sig.get_returns().iter())
                    .mzip()
                    .enumerate()
                {
                    if !expected_type.is_inhabited_by(ret) {
                        panic!(
                            "Unexpected return type encountered while evaluating {}. \
                             At position {i}, expected type {expected_type}, but encountered {}.",
                            ir.get_op(opid).format(),
                            D::TypeSystem::type_of(ret)
                        )
                    }
                }
                *(self.opmap.get_mut(opid).unwrap()) = OpState::Evaluated(duration);
                for (ret_valid, ret_eval) in (return_valids.iter(), ret_evals.into_iter()).mzip() {
                    *(self.valmap.get_mut(ret_valid).unwrap()) = ValState::Evaluated(ret_eval)
                }
            }
        }
    }
}
