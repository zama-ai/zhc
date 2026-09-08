//! Splitting of an integer-level circuit over several partitions.
//!
//! A partition is a group of operations that run on the same compute unit. Partitioning a circuit
//! decides which partition runs each of its operations. This module gives two ways to do it:
//!
//! - [`partition_and_materialize`] partitions the circuit automatically. It reduces the circuit to
//!   a graph of tasks with [`coarsen_ioplang`](zhc_langs::tasklang::coarsen_ioplang), distributes
//!   the tasks over the partitions described by a [`PartitionerConfig`], then places the other
//!   operations next to them.
//! - [`materialize_builder_partitioning`] starts from the partitions declared while building the
//!   circuit, and only places the operations that these partitions leave out.
//!
//! In both cases, the result is *materialized*: the circuit is rewritten so that each of its
//! operations belongs to exactly one partition. An operation placed in several partitions is
//! replicated once per partition, and each value that a partition reads from another partition
//! goes through an explicit copy operation. The rewritten circuit comes with a map that gives the
//! partition of each of its operations.
//!
//! The behavior of the automatic partitioner is set by [`PartitionerConfig`]: the capacity of each
//! partition ([`PartitionSpec`]), the direction in which the circuit is walked, the rule that
//! breaks ties between partitions ([`TiePolicy`]), and the rule that places the operations that
//! are not tasks ([`SpreadPolicy`]).
//!
//! # Examples
//!
//! ```rust,no_run
//! # use zhc_pipeline::{
//! #     PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy,
//! #     partition_and_materialize,
//! # };
//! # use zhc_utils::units::Cycle;
//! # let ir = zhc_ir::IR::<zhc_langs::ioplang::IopLang>::empty();
//! // Four identical partitions, each running up to 12 tasks per batch.
//! let config = PartitionerConfig {
//!     specs: std::iter::repeat(PartitionSpec { parallelism: 12, latency: Cycle(1) })
//!         .take(4)
//!         .collect(),
//!     sched_policy: SchedPolicy::AsSoonAsPossible,
//!     tie_policy: TiePolicy::Concentrate,
//!     spread_policy: SpreadPolicy::FavorLatency,
//! };
//!
//! let (materialized, partitions) = partition_and_materialize(&ir, &config);
//! for op in materialized.walk_ops_linear() {
//!     println!("{:?} runs in partition {:?}", op.get_id(), partitions[op.get_id()]);
//! }
//! ```
use crate::{SchedPolicy, iop::partitioning::materialize::materialize_partitioning};
use zhc_ir::{
    AnnIR, AnnOpRef, IR, OpId, OpMap, ValMap,
    height_depth::{HeightDepth, analyze_height_depth},
    partitioning::{PartitionId, PartitionPlacement},
    translation::Translation,
};
use zhc_langs::{
    ioplang::IopLang,
    tasklang::{TaskLang, coarsen_ioplang},
};
use zhc_sim::Simulator;
use zhc_utils::{Dumpable, Store, units::MHz};

mod config;
mod materialize;
mod sim;
mod spread;
#[cfg(test)]
mod tests;

pub use config::*;

/// Partitions a circuit automatically, and rewrites it so that each operation runs in one
/// partition.
///
/// The `ir` argument is the integer-level circuit to split, and `config` describes the partitions
/// and the rules of the partitioner. The partitioner distributes the tasks of `ir`, as given by
/// [`coarsen_ioplang`], over the partitions of `config`, then places the other operations with
/// the [`spread_policy`](PartitionerConfig::spread_policy) of `config`. A circuit without tasks is
/// placed entirely in partition `0`.
///
/// The returned pair holds the materialized circuit and the partition of each of its operations.
/// The materialized circuit computes the same outputs as `ir`, but an operation placed in several
/// partitions appears once per partition, and a copy operation is added for each value that a
/// partition reads from another partition. Every operation of the materialized circuit has an
/// entry in the returned map.
///
/// # Panics
///
/// Panics if `ir` holds a task and no partition of `config` can run one, that is if
/// [`specs`](PartitionerConfig::specs) is empty or all its entries have a
/// [`parallelism`](PartitionSpec::parallelism) of zero.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_pipeline::{
/// #     PartitionSpec, PartitionerConfig, SchedPolicy, SpreadPolicy, TiePolicy,
/// #     partition_and_materialize,
/// # };
/// # use zhc_utils::units::Cycle;
/// # let ir = zhc_ir::IR::<zhc_langs::ioplang::IopLang>::empty();
/// let config = PartitionerConfig {
///     specs: std::iter::repeat(PartitionSpec { parallelism: 12, latency: Cycle(1) })
///         .take(2)
///         .collect(),
///     sched_policy: SchedPolicy::AsSoonAsPossible,
///     tie_policy: TiePolicy::Balance,
///     spread_policy: SpreadPolicy::FavorBandwidth,
/// };
///
/// let (materialized, partitions) = partition_and_materialize(&ir, &config);
/// println!("{} operations after materialization", materialized.n_ops());
/// ```
pub fn partition_and_materialize(
    ir: &IR<IopLang>,
    config: &PartitionerConfig,
) -> (IR<IopLang>, OpMap<PartitionId>) {
    materialize_partitioning(ir, &partition(ir, config))
}

fn partition(ir: &IR<IopLang>, config: &PartitionerConfig) -> OpMap<PartitionPlacement> {
    let Translation {
        output: tasklang,
        provenance_map,
    } = coarsen_ioplang(ir);
    let tasklang_partitions = partition_tasklang(&tasklang, &config);
    let mut oup = ir.filled_opmap(PartitionPlacement::Irrelevant);
    for (tasklang_opid, pid) in tasklang_partitions.iter() {
        oup.insert(
            provenance_map.operations()[tasklang_opid],
            PartitionPlacement::Exclusive(*pid),
        );
    }
    spread::spread(ir, oup, config.spread_policy)
}

fn partition_tasklang(ir: &IR<TaskLang>, config: &PartitionerConfig) -> OpMap<PartitionId> {
    // See [1] for the rationale behind this priorization.
    let ann = analyze_height_depth(ir, |op, score| {
        score.strict_add(op.get_instruction().get_work())
    });
    let mut sim = Simulator::from_simulatable(
        MHz(1),
        sim::Partitioner::new(&ann, config),
        zhc_sim::TracingLevel::None,
    );
    sim.play();
    sim.into_simulatable().into_partition_map()
}

// Notes:
// ======
//
// [1]: About HEFT: Usually in Heterogeneous Earliest Time Finish, the cost of transfer is accounted for in the
// priorization scheme. This matters if: There is a large imbalance in the work of tasks, and if the
// transfer cost can make a difference (that is it is big enough compared to the cost of work). In
// our case, work (in most cast pbses) is roughly hundred time the cost of the transfer. Meaning
// that there is very little chance this makes a difference.
