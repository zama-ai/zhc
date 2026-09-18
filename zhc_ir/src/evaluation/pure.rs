//! Push-based evaluation of IRs whose operations never block.
//!
//! Hosts [`PureEvaluator`], the driver for IRs in which every operation evaluates as soon as its
//! arguments are available. It rejects an operation that reports
//! [`Blocked`](EvalOutcome::Blocked) with a panic. Use [`EffectfulEvaluator`] for IRs whose
//! operations can block on the context.

use super::*;
use crate::{AnnIR, AnnIRView, AsValId, Dialect, IR};

/// Push-based driver for IRs whose operations never block.
///
/// Holds the evaluation state of every active operation and value of the borrowed [`IR`]. A single
/// call to [`push_to_completion`](Self::push_to_completion) evaluates the whole IR in dependency
/// order.
pub struct PureEvaluator<'ir, D: Dialect, V: Evaluation>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    inner: EagerEvaluator<'ir, D, V>,
}

impl<'ir, D: Dialect, V: Evaluation> PureEvaluator<'ir, D, V>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    /// Creates an evaluator over an IR, with every operation and value pending.
    ///
    /// Borrows `ir` for the evaluator's lifetime; nothing is evaluated until
    /// [`push_to_completion`](Self::push_to_completion) is called.
    pub fn from_ir(ir: &'ir IR<D>) -> Self {
        PureEvaluator {
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

    /// Evaluates every pending operation of the IR, in dependency order.
    ///
    /// Each operation is evaluated against `context`. A panic raised by
    /// [`eval`](Evaluable::eval) does not stop the run: the operation settles as
    /// [`OpState::Panicked`] and the operations that depend on it are poisoned. After the call,
    /// every operation has settled. A second call has no effect.
    ///
    /// # Panics
    ///
    /// Panics if an operation reports [`Blocked`](EvalOutcome::Blocked). The other operations that
    /// do not depend on it are evaluated before the panic, so `context` can already be modified.
    /// Also panics if an argument or a result does not inhabit the type the signature declares for
    /// it.
    pub fn push_to_completion(
        &mut self,
        context: &mut <D::InstructionSet as Evaluable<V>>::Context,
    ) {
        let sweep = self.inner.push_ready(context);
        if let Some(opid) = sweep.blocked.first() {
            panic!(
                "Operation {} blocked during a pure run.",
                self.inner.ir().get_op(*opid).format()
            );
        }
    }
}
