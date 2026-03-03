use std::ops::Index;

use zhc_ir::{OpRef, ValId};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang};
use zhc_utils::{
    iter::{CollectInVec, MultiZip},
    small::SmallMap,
};

/// Mapping between values in the batch and outside the batch.
pub struct BatchMap(SmallMap<ValId, ValId>);

impl BatchMap {
    /// Built the batch map from a batch op.
    pub fn from_op(op: &OpRef<HpuLang>) -> Self {
        let args = op.get_arg_valids();
        let rets = op.get_return_valids();
        let mut map = SmallMap::<ValId, ValId>::new();
        let HpuInstructionSet::Batch { block } = op.get_instruction() else {
            unreachable!()
        };
        let mut ordered_batch_arg = block
            .walk_ops_linear()
            .filter(|op| matches!(op.get_instruction(), HpuInstructionSet::BatchArg { .. }))
            .covec();
        ordered_batch_arg.sort_unstable_by_key(|op| {
            let HpuInstructionSet::BatchArg { pos, .. } = op.get_instruction() else {
                unreachable!()
            };
            pos
        });
        let mut ordered_batch_ret = block
            .walk_ops_linear()
            .filter(|op| matches!(op.get_instruction(), HpuInstructionSet::BatchRet { .. }))
            .covec();
        ordered_batch_ret.sort_unstable_by_key(|op| {
            let HpuInstructionSet::BatchRet { pos, .. } = op.get_instruction() else {
                unreachable!()
            };
            pos
        });
        for (outer_valid, inner_valid) in (
            args.iter(),
            ordered_batch_arg
                .into_iter()
                .map(|a| a.get_return_valids()[0]),
        )
            .mzip()
        {
            map.insert(inner_valid, *outer_valid);
        }
        for (outer_valid, inner_valid) in (
            rets.iter(),
            ordered_batch_ret.into_iter().map(|a| a.get_arg_valids()[0]),
        )
            .mzip()
        {
            map.insert(inner_valid, *outer_valid);
        }
        BatchMap(map)
    }
}

impl Index<ValId> for BatchMap {
    type Output = ValId;

    fn index(&self, index: ValId) -> &Self::Output {
        self.0.get(&index).unwrap()
    }
}
