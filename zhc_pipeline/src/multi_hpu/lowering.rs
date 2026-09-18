use zhc_ir::{IR, OpMap, partitioning::PartitionId, translation::Translation};
use zhc_langs::{
    hpulang::{HpuId, HpuInstructionSet, HpuLang, HpuLocality},
    ioplang::IopLang,
};
use zhc_utils::SafeAs;

use crate::hpu::lowering::lower_iop_to_hpu;

pub fn lower_iop_to_multi_hpu<'a>(
    ir: &IR<IopLang>,
    partitions: &OpMap<PartitionId>,
) -> (IR<HpuLang>, OpMap<HpuLocality>) {
    let Translation {
        output: mut ir,
        provenance_map,
    } = lower_iop_to_hpu(ir);
    let hid_map: OpMap<HpuId> = partitions.clone().map(|a| HpuId(a.0.sas()));
    let hid_map = provenance_map.project_opmap(&hid_map);
    ir.replace_ops_instr_linear(|opref| match opref.get_instruction() {
        HpuInstructionSet::Transfer { .. } => {
            let from = *hid_map
                .get(opref.get_predecessors_iter().next().unwrap())
                .unwrap();
            let to = *hid_map.get(opref).unwrap();
            HpuInstructionSet::Transfer { from, to }
        }
        i => i.clone(),
    });

    let localities = ir.totally_mapped_opmap(|opref| {
        use HpuInstructionSet::*;
        match opref.get_instruction() {
            Transfer { from, to } => HpuLocality::Transfer {
                from: *from,
                to: *to,
            },
            _ => HpuLocality::OnHpu(*hid_map.get(opref).unwrap()),
        }
    });
    (ir, localities)
}
