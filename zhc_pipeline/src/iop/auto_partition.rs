use zhc_ir::partition::PartitionId;
use zhc_langs::ioplang::IopLang;

pub fn partition(ir: &IR<IopLang>, n_partitions: u16) -> OpMap<PartitionId>
