//! Push-based evaluation of IRs whose operations can block on the context.
//!
//! Hosts [`EffectfulEvaluator`], the driver for IRs in which an operation can report
//! [`Blocked`](EvalOutcome::Blocked) until another operation has changed the context, for instance
//! a receive that waits for a matching send. The driver can run several IRs against one shared
//! context, so that a send in one IR unblocks a receive in another. It retries blocked operations
//! until all operations have settled, and reports a deadlock as an error instead of a panic. Use
//! [`PureEvaluator`] for IRs whose operations never block.

use super::*;
use crate::{AnnIR, AnnIRView, AsValId, Dialect, IR, OpId};

/// Push-based driver for one or more IRs whose operations can block on the context.
///
/// Holds the evaluation state of every active operation and value of each borrowed [`IR`]. The
/// IRs are independent graphs that share only the context. They are addressed by their index, in
/// the order given at construction. A call to [`push_to_completion`](Self::push_to_completion)
/// sweeps the IRs in turn, each in dependency order, and retries the blocked operations until all
/// operations have settled or no progress is possible.
pub struct EffectfulEvaluator<'ir, D: Dialect, V: Evaluation>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    inner: Vec<EagerEvaluator<'ir, D, V>>,
}

impl<'ir, D: Dialect, V: Evaluation> EffectfulEvaluator<'ir, D, V>
where
    D::InstructionSet: Evaluable<V>,
    D::TypeSystem: EvaluatesTo<V>,
{
    /// Creates an evaluator over a single IR, with every operation and value pending.
    ///
    /// Equivalent to [`from_irs`](Self::from_irs) with one IR, which gets index `0`.
    pub fn from_ir(ir: &'ir IR<D>) -> Self {
        Self::from_irs([ir])
    }

    /// Creates an evaluator over several IRs, with every operation and value pending.
    ///
    /// Each IR gets the index of its position in `irs`. The IRs are borrowed for the evaluator's
    /// lifetime; nothing is evaluated until [`push_to_completion`](Self::push_to_completion) is
    /// called.
    pub fn from_irs(irs: impl IntoIterator<Item = &'ir IR<D>>) -> Self {
        EffectfulEvaluator {
            inner: irs.into_iter().map(EagerEvaluator::from_ir).collect(),
        }
    }

    /// Consumes the evaluator into one annotated IR per IR, holding the values themselves.
    ///
    /// Strips the state wrappers, leaving every operation annotated with `()` and every value with
    /// the value it evaluated to. The annotated IRs come in index order. Use
    /// [`into_eval_irs`](Self::into_eval_irs) instead when the run may have failed.
    ///
    /// # Panics
    ///
    /// Panics if any operation or value of any IR has not settled in its evaluated state.
    pub fn into_value_irs(self) -> Vec<AnnIR<'ir, D, (), V>> {
        self.inner
            .into_iter()
            .map(EagerEvaluator::into_value_ir)
            .collect()
    }

    /// Consumes the evaluator into one annotated IR per IR, holding the evaluation states.
    ///
    /// Preserves the complete outcome of the run, panic messages and poison markers included,
    /// making it the form to inspect after a failure. The annotated IRs come in index order.
    pub fn into_eval_irs(self) -> Vec<AnnIR<'ir, D, OpState, ValState<V>>> {
        self.inner
            .into_iter()
            .map(EagerEvaluator::into_eval_ir)
            .collect()
    }

    /// Returns one read-only annotated view of the current evaluation state per IR, in index
    /// order.
    pub fn as_views(&self) -> Vec<AnnIRView<'ir, '_, D, OpState, ValState<V>>> {
        self.inner.iter().map(EagerEvaluator::as_view).collect()
    }

    /// Returns the value bound to the given value id of the IR at `ir_index`.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::PendingValId`] if `valid` has not been evaluated yet,
    /// [`EvalError::PoisonedValId`] if the operation producing `valid` or one of its predecessors
    /// failed, and [`EvalError::UnknownValId`] if the evaluator holds no state for `valid`.
    ///
    /// # Panics
    ///
    /// Panics if `ir_index` is out of bounds, or if `valid` is out of bounds for the IR at
    /// `ir_index` or refers to an inactive value.
    pub fn get_val(&self, ir_index: usize, valid: impl AsValId) -> Result<&V, EvalError> {
        self.inner[ir_index].get_val(valid)
    }

    /// Returns `true` if every active operation of every IR has been evaluated.
    ///
    /// Reports that no operation is left pending, panicked or poisoned, including operations that
    /// declare no result. Since every value is produced by an operation, every active value is
    /// then evaluated too.
    pub fn is_ok(&self) -> bool {
        self.inner.iter().all(EagerEvaluator::is_ok)
    }

    /// Evaluates every pending operation of the IRs, retrying blocked operations.
    ///
    /// Operations are evaluated against `context`, which all IRs share. A round sweeps each IR
    /// once, in index order, and each sweep follows the dependency order of its IR. An operation
    /// that reports [`Blocked`](EvalOutcome::Blocked) stays pending, together with the operations
    /// that depend on it, and is retried in the next round. A panic raised by
    /// [`eval`](Evaluable::eval) does not stop the run: the operation settles as
    /// [`OpState::Panicked`] and the operations that depend on it are poisoned. Returns `Ok(())`
    /// when every operation of every IR has settled, which does not imply that
    /// [`is_ok`](Self::is_ok) holds. Calling it again after `Ok(())` has no effect.
    ///
    /// # Errors
    ///
    /// Returns the blocked operations when a full round settles no operation in any IR. Each
    /// operation comes with the index of its IR. The operations are grouped by IR in index order,
    /// and in topological order within an IR. These operations and the operations that depend on
    /// them stay pending.
    ///
    /// # Panics
    ///
    /// Panics if an argument or a result does not inhabit the type the signature declares for it.
    pub fn push_to_completion(
        &mut self,
        context: &mut <D::InstructionSet as Evaluable<V>>::Context,
    ) -> Result<(), Vec<(usize, OpId)>> {
        self.push_to_completion_with(context, |_, _| {})
    }

    /// Evaluates every pending operation of the IRs, telling the context which IR is swept.
    ///
    /// Same as [`push_to_completion`](Self::push_to_completion), except that `enter` is called
    /// with the context and the index of the IR before each sweep of that IR. Use it when the
    /// context keeps a state per IR, for instance one register file per board, and an operation
    /// cannot tell from its own fields which IR it belongs to.
    ///
    /// # Errors
    ///
    /// Same as [`push_to_completion`](Self::push_to_completion).
    ///
    /// # Panics
    ///
    /// Same as [`push_to_completion`](Self::push_to_completion).
    pub fn push_to_completion_with(
        &mut self,
        context: &mut <D::InstructionSet as Evaluable<V>>::Context,
        mut enter: impl FnMut(&mut <D::InstructionSet as Evaluable<V>>::Context, usize),
    ) -> Result<(), Vec<(usize, OpId)>> {
        loop {
            let mut settled = 0;
            let mut blocked = Vec::new();
            for (ir_index, inner) in self.inner.iter_mut().enumerate() {
                enter(context, ir_index);
                let sweep = inner.push_ready(context);
                settled += sweep.settled;
                blocked.extend(sweep.blocked.into_iter().map(|opid| (ir_index, opid)));
            }
            if blocked.is_empty() {
                return Ok(());
            }
            if settled == 0 {
                return Err(blocked);
            }
        }
    }
}
