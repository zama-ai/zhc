use zhc_ir::partitioning::PartitionId;
use zhc_utils::{Store, units::Cycle};

use crate::SchedPolicy;

/// The processing capacity of one partition, as seen by the automatic partitioner.
///
/// The partitioner models each partition as a unit that runs tasks in batches: a batch holds at
/// most `parallelism` tasks, and takes `latency` cycles to complete.
#[derive(Clone, Debug, PartialEq, Eq)]
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

/// The rule that breaks ties between partitions that are equally good for a task.
///
/// The partitioner places each task in the partition with the lowest
/// [`latency`](PartitionSpec::latency), then in the one that already holds the most operands of
/// the task. When several partitions are still equal after these two criteria, the tie policy
/// selects one of them, based on the number of tasks each one already received in the batch being
/// filled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// The rule that places the operations that are not tasks.
///
/// The automatic partitioner only distributes the tasks of the circuit. Every other operation is
/// then placed relative to them, and an operation can be placed in more than one
/// partition, in which case each of these partitions computes it. The spread policy selects the
/// trade-off between computing such operations several times and copying their results from one
/// partition to another.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// The parameters of the automatic partitioner.
///
/// The partitioner walks the circuit in the direction given by `sched_policy`, and distributes
/// its tasks over the partitions described by `specs`, batch after batch, the most critical tasks
/// first. The `tie_policy` breaks the ties between equally good partitions, and the
/// `spread_policy` then places the operations that are not tasks.
#[derive(Clone, Debug, PartialEq, Eq)]
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
