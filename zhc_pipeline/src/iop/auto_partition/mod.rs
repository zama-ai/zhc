use zhc_ir::{IR, OpId, OpMap};
use zhc_langs::ioplang::IopLang;
use zhc_utils::{Store, StoreIndex, fsm};

#[derive(Clone, Copy, StoreIndex)]
pub struct PartitionId(u16);

pub struct PartitionSpec {
    pub parallelism: u16,
}

#[fsm]
pub enum PartitionState{
    Idle,
    Running(Vec<OpId>)
}

pub struct PartitionCluster {
    ready_ops:
    partitions: Store<PartitionId, PartitionState>,
}

pub fn partition(ir: &IR<IopLang>, partitions: Store<PartitionId, PartitionSpec>) -> OpMap<PartitionId> {
   todo!()
}
