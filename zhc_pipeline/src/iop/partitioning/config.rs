use zhc_ir::partitioning::PartitionId;
use zhc_utils::{Store, units::Cycle};

use crate::SchedPolicy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartitionSpec {
    pub parallelism: u16,
    pub latency: Cycle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiePolicy {
    Concentrate,
    Balance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpreadPolicy {
    FavorLatency,
    FavorBandwidth,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartitionerConfig {
    pub specs: Store<PartitionId, PartitionSpec>,
    pub sched_policy: SchedPolicy,
    pub tie_policy: TiePolicy,
    pub spread_policy: SpreadPolicy,
}
