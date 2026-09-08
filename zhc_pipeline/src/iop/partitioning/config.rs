use zhc_ir::partitioning::PartitionId;
use zhc_utils::{Store, units::Cycle};

use crate::SchedPolicy;

#[derive(Clone, Debug)]
pub struct PartitionSpec {
    /// The maximum number of tasks the partition runs in one batch.
    ///
    /// A partition with a `parallelism` of zero never receives a task.
    pub parallelism: u16,

    /// The number of cycles the partition takes to complete one batch of tasks.
    ///
    /// When several partitions have room for a task, the partitioner prefers the ones with the
    /// lowest `latency`.
    pub latency: Cycle,
}

#[derive(Clone, Copy, Debug)]
pub enum TiePolicy {
    /// Prefers the partition that already received the most tasks in the current batch.
    ///
    /// Tasks gather in as few partitions as possible, which fills the batches of a partition
    /// before the next partition is used.
    Concentrate,

    /// Prefers the partition that already received the fewest tasks in the current batch.
    ///
    /// Tasks spread evenly over the partitions, which keeps the batches of all the partitions at
    /// similar sizes.
    Balance,
}

#[derive(Clone, Copy, Debug)]
pub enum SpreadPolicy {
    /// Places an operation first in every partition that uses its result.
    ///
    /// The operation is computed in each of these partitions, so its result does not have to be
    /// copied between partitions. This costs duplicated computations, and reduces the number of
    /// copies.
    FavorLatency,

    /// Places an operation first in the partition that produces most of its operands.
    ///
    /// The operation is computed once, and its result is copied to the other partitions that use
    /// it. This avoids duplicated computations, and increases the number of copies. An operation
    /// whose operands come from no partition is still placed in the partitions that use its
    /// result.
    FavorBandwidth,
}

#[derive(Clone, Debug)]
pub struct PartitionerConfig {
    /// The capacity of each partition, indexed by partition id.
    ///
    /// The number of entries is the number of partitions to distribute the circuit over.
    pub specs: Store<PartitionId, PartitionSpec>,

    /// The direction in which the partitioner walks the circuit.
    ///
    /// [`SchedPolicy::AsSoonAsPossible`] distributes the tasks from the inputs to the outputs, and
    /// [`SchedPolicy::AsLateAsPossible`] from the outputs back to the inputs.
    pub sched_policy: SchedPolicy,

    /// The rule that breaks ties between partitions that are equally good for a task.
    pub tie_policy: TiePolicy,

    /// The rule that places the operations that are not tasks.
    pub spread_policy: SpreadPolicy,
}
