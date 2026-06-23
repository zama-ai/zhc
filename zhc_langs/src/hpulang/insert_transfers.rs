use zhc_ir::{IR, OpMap, ValId, ValUse};
use zhc_utils::{iter::CollectInSmallVec, svec};

use crate::{
    hpulang::{HpuId, HpuInstructionSet, HpuLang},
};

pub fn insert_transfers(ir: &mut IR<HpuLang>, partitions: &OpMap<HpuId>) {
    struct TransferToInsert {
        valid: ValId,
        uze: ValUse,
        from: HpuId,
        to: HpuId,
    }
    let val_uses_to_transfer = ir
        .walk_vals_linear()
        .flat_map(|val| val.get_uses_iter().map(move |uze| (val.clone(), uze)))
        .filter(|(val, uze)| {
            !val.get_origin().opref.get_instruction().is_replicable()
                && partitions[val.get_origin().opref] != partitions[*uze.opref]
        })
        .map(|(val, uze)| TransferToInsert {
            valid: val.get_id(),
            uze: ValUse {
                opid: uze.opref.get_id(),
                position: uze.position,
            },
            from: partitions[val.get_origin().opref],
            to: partitions[*uze.opref],
        })
        .cosvec();

    for transfer in val_uses_to_transfer.into_iter() {
        let TransferToInsert {
            valid,
            uze,
            from,
            to,
        } = transfer;
        let (_, valids) = ir.add_op(HpuInstructionSet::Transfer { from, to }, svec![valid]);
        ir.replace_val_use_at(uze, valids[0]);
    }
}

#[cfg(test)]
mod test {

}
