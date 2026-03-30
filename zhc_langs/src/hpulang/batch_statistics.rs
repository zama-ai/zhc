use zhc_ir::IR;
use zhc_utils::data_visulization::Histogram;

use super::{HpuInstructionSet, HpuLang};

pub type BatchStatistics = Histogram<u16>;

/// Walks the IR and records the PBS count of each `Batch` block.
pub fn get_batch_statistics(ir: &IR<HpuLang>) -> BatchStatistics {
    let mut stats = BatchStatistics::empty();
    for op in ir.walk_ops_linear() {
        if let HpuInstructionSet::Batch { block } = op.get_instruction() {
            let pbs_count = block
                .walk_ops_linear()
                .filter(|op| op.get_instruction().is_pbs())
                .count();
            stats.count(&(pbs_count as u16));
        }
    }
    stats
}
