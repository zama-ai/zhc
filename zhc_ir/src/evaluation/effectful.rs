//! Push-based evaluation of IRs whose operations can block on the context.
//!
//! Hosts [`EffectfulEvaluator`], the driver for IRs in which an operation can report
//! [`Blocked`](EvalOutcome::Blocked) until another operation has changed the context, for instance
//! a receive that waits for a matching send. The driver retries blocked operations until all
//! operations have settled, and reports a deadlock as an error instead of a panic. Use
//! [`PureEvaluator`] for IRs whose operations never block.

use super::*;
use crate::{AnnIR, AnnIRView, AsValId, Dialect, IR, OpId};

/// Push-based driver for IRs whose operations can block on the context.
///
/// Holds the evaluation state of every active operation and value of the borrowed [`IR`]. A call
/// to [`push_to_completion`](Self::push_to_completion) evaluates the IR in dependency order and
/// retries the blocked operations until all operations have settled or no progress is possible.
pub struct EffectfulEvaluator<'ir, D: Dialect, V: Evaluation>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    inner: EagerEvaluator<'ir, D, V>,
}

impl<'ir, D: Dialect, V: Evaluation> EffectfulEvaluator<'ir, D, V>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    /// Creates an evaluator over an IR, with every operation and value pending.
    ///
    /// Borrows `ir` for the evaluator's lifetime; nothing is evaluated until
    /// [`push_to_completion`](Self::push_to_completion) is called.
    pub fn from_ir(ir: &'ir IR<D>) -> Self {
        EffectfulEvaluator {
            inner: EagerEvaluator::from_ir(ir),
        }
    }

    /// Consumes the evaluator into an annotated IR holding the values themselves.
    ///
    /// Strips the state wrappers, leaving every operation annotated with `()` and every value with
    /// the value it evaluated to. Use [`into_eval_ir`](Self::into_eval_ir) instead when the run
    /// may have failed.
    ///
    /// # Panics
    ///
    /// Panics if any operation or value of the IR has not settled in its evaluated state.
    pub fn into_value_ir(self) -> AnnIR<'ir, D, (), V> {
        self.inner.into_value_ir()
    }

    /// Consumes the evaluator into an annotated IR holding the evaluation states.
    ///
    /// Preserves the complete outcome of the run, panic messages and poison markers included,
    /// making it the form to inspect after a failure.
    pub fn into_eval_ir(self) -> AnnIR<'ir, D, OpState, ValState<V>> {
        self.inner.into_eval_ir()
    }

    /// Returns a read-only annotated view of the current evaluation state.
    pub fn as_view(&self) -> AnnIRView<'ir, '_, D, OpState, ValState<V>> {
        self.inner.as_view()
    }

    /// Returns the value bound to the given value id.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::PendingValId`] if `valid` has not been evaluated yet,
    /// [`EvalError::PoisonedValId`] if the operation producing `valid` or one of its predecessors
    /// failed, and [`EvalError::UnknownValId`] if the evaluator holds no state for `valid`.
    ///
    /// # Panics
    ///
    /// Panics if `valid` is out of bounds for the IR or refers to an inactive value.
    pub fn get_val(&self, valid: impl AsValId) -> Result<&V, EvalError> {
        self.inner.get_val(valid)
    }

    /// Returns `true` if every active value of the IR has been evaluated.
    ///
    /// Reports that no value is left pending or poisoned. Operation states are not inspected, so
    /// an operation that declares no result can have failed while this still returns `true`.
    pub fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

    /// Evaluates every pending operation of the IR, retrying blocked operations.
    ///
    /// Operations are evaluated against `context`, in dependency order. An operation that reports
    /// [`Blocked`](EvalOutcome::Blocked) stays pending, together with the operations that depend
    /// on it, and is retried after the other ready operations have run. A panic raised by
    /// [`eval`](Evaluable::eval) does not stop the run: the operation settles as
    /// [`OpState::Panicked`] and the operations that depend on it are poisoned. Returns `Ok(())`
    /// when every operation has settled, which does not imply that [`is_ok`](Self::is_ok) holds.
    /// Calling it again after `Ok(())` has no effect.
    ///
    /// # Errors
    ///
    /// Returns the blocked operations, in topological order, when a full pass over the pending
    /// operations settles none of them. These operations and the operations that depend on them
    /// stay pending.
    ///
    /// # Panics
    ///
    /// Panics if an argument or a result does not inhabit the type the signature declares for it.
    pub fn push_to_completion(
        &mut self,
        context: &mut <D::InstructionSet as Evaluable<V>>::Context,
    ) -> Result<(), Vec<OpId>> {
        loop {
            let sweep = self.inner.push_ready(context);
            if sweep.blocked.is_empty() {
                return Ok(());
            }
            if sweep.settled == 0 {
                return Err(sweep.blocked);
            }
        }
    }
}
