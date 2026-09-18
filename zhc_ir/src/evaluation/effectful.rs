use super::*;
use crate::{AnnIR, AnnIRView, AsValId, Dialect, IR, OpId};

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
    pub fn from_ir(ir: &'ir IR<D>) -> Self {
        EffectfulEvaluator {
            inner: EagerEvaluator::from_ir(ir),
        }
    }

    pub fn into_value_ir(self) -> AnnIR<'ir, D, (), V> {
        self.inner.into_value_ir()
    }

    pub fn into_eval_ir(self) -> AnnIR<'ir, D, OpState, ValState<V>> {
        self.inner.into_eval_ir()
    }

    pub fn as_view(&self) -> AnnIRView<'ir, '_, D, OpState, ValState<V>> {
        self.inner.as_view()
    }

    pub fn get_val(&self, valid: impl AsValId) -> Result<&V, EvalError> {
        self.inner.get_val(valid)
    }

    pub fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

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
