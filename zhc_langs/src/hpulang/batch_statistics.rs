use std::fmt;

use zhc_ir::IR;

use super::{HpuInstructionSet, HpuLang};

/// Accumulates batch-size counts and renders them as a horizontal histogram.
///
/// Call [`record`](Self::record) once per batch with its fill level,
/// then print via `Display` (or `Debug`, which delegates to `Display`).
#[derive(Clone, Default)]
pub struct BatchStatistics {
    /// (batch_size, count) — kept sorted on display, not on insert.
    entries: Vec<(u16, u16)>,
}

impl BatchStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one batch with the given fill level.
    pub fn record(&mut self, batch_size: u16) {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| *k == batch_size) {
            entry.1 += 1;
        } else {
            self.entries.push((batch_size, 1));
        }
    }

    pub fn total_batches(&self) -> u16 {
        self.entries.iter().map(|(_, c)| c).sum()
    }
}

impl fmt::Display for BatchStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.entries.is_empty() {
            return write!(f, "Batch statistics: (none)");
        }
        let mut sorted: Vec<(u16, u16)> = self.entries.clone();
        sorted.sort_unstable_by_key(|&(size, _)| size);
        let total: u16 = sorted.iter().map(|(_, c)| c).sum();
        let max_count = sorted.iter().map(|(_, c)| *c).max().unwrap().max(1);
        // 80 cols - "  NNN │ " (8) - " (NNNNN)" (8) = 64 max bar width
        const MAX_BAR: usize = 64;
        writeln!(f, "Batch statistics ({total} batches):")?;
        for (size, count) in &sorted {
            let w = (*count as usize * MAX_BAR) / max_count as usize;
            let bar: String = "█".repeat(w.max(1));
            writeln!(f, "  {size:>3} │ {bar} ({count})")?;
        }
        Ok(())
    }
}

impl fmt::Debug for BatchStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Walks the IR and records the PBS count of each `Batch` block.
pub fn get_batch_statistics(ir: &IR<HpuLang>) -> BatchStatistics {
    let mut stats = BatchStatistics::new();
    for op in ir.walk_ops_linear() {
        if let HpuInstructionSet::Batch { block } = op.get_instruction() {
            let pbs_count = block
                .walk_ops_linear()
                .filter(|op| op.get_instruction().is_pbs())
                .count();
            stats.record(pbs_count as u16);
        }
    }
    stats
}
