use zhc_ir::{IR, OpMap, partition::PartitionId, translation::Translation};
use zhc_langs::{
    hpulang::{HpuId, HpuInstructionSet, HpuLang, HpuLocality, insert_transfers},
    ioplang::IopLang,
};
use zhc_utils::{
    SafeAs,
    iter::{Deduped, DedupedByKey},
    small::SmallMap,
};

use crate::hpu::lowering::lower_iop_to_hpu;

pub fn lower_iop_to_multi_hpu<'a>(
    ir: &IR<IopLang>,
    partitions: &OpMap<PartitionId>,
) -> (IR<HpuLang>, OpMap<HpuLocality>) {
    let lowered = lower_iop_to_hpu(ir);
    // TODO: thread the LUT payload through the multi-HPU artifacts.
    assert!(
        lowered.lut_payload.is_empty(),
        "LUT payload is not plumbed through the multi-HPU pipeline yet."
    );
    let Translation {
        output: mut ir,
        provenance_map,
    } = lowered.translation;
    let partition_to_hid: SmallMap<PartitionId, HpuId> = partitions
        .iter()
        .map(|a| a.1.clone())
        .dedup_by_key(|a| a.clone())
        .enumerate()
        .map(|(i, p)| (p, HpuId(i.sas())))
        .collect();
    let hid_map: OpMap<HpuId> = partitions
        .clone()
        .map(|a| partition_to_hid.get(&a).unwrap().clone());
    let mut hid_map = provenance_map.project_opmap(&hid_map);
    ir.walk_ops_linear()
        .filter(|a| matches!(a.get_instruction(), HpuInstructionSet::DstSt { .. }))
        .for_each(|op| {
            *hid_map.get_mut(op).unwrap() = *hid_map
                .get(op.get_predecessors_iter().next().unwrap())
                .unwrap();
        });
    insert_transfers(&mut ir, &hid_map);
    let localities = ir.totally_mapped_opmap(|opref| {
        use HpuInstructionSet::*;
        match opref.get_instruction() {
            Transfer { from, to } => HpuLocality::Transfer { from, to },
            CstCt { .. } | ImmLd { .. } | SrcLd { .. } => HpuLocality::Shared(
                opref
                    .get_users_iter()
                    .map(|u| *hid_map.get(u).unwrap())
                    .dedup()
                    .collect(),
            ),
            _ => HpuLocality::OnHpu(*hid_map.get(opref).unwrap()),
        }
    });
    (ir, localities)
}
