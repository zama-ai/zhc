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
