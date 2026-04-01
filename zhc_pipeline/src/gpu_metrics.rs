//! GPU-level performance metrics.
//!
//! This module provides metrics computed after the GPU compilation pipeline, including
//! batch size distribution and the compiled IR.

use zhc_ir::IR;
use zhc_langs::hpulang::HpuLang;
use zhc_utils::{Dumpable, data_visulization::Histogram};

/// GPU execution performance metrics.
///
/// Contains batch statistics and the compiled IR for the GPU target.
pub struct GpuMetrics {
    /// Distribution of PBS batch sizes.
    pub batch_stats: Histogram<u16>,
    /// Compiled HPU-level IR.
    pub ir: IR<HpuLang>,
}

impl Dumpable for GpuMetrics {
    fn dump_to_string(&self) -> String {
        let batch_hist_str = self.batch_stats.dump_to_string();
        let batch_hist_lines: Vec<&str> = batch_hist_str.lines().collect();
        let ir_str = self.ir.dump_to_string();
        let ir_lines: Vec<&str> = ir_str.lines().collect();

        let mut result = String::from(
            "╔══════════════════════════════════════════════════════════════════════════════
║ GPU Metrics
║──────────────────────────────────────────────────────────────────────────────
║   Batch Size Histogram:",
        );
        for line in batch_hist_lines {
            result.push_str(&format!("\n║     {}", line));
        }
        result.push_str(
            "\n║──────────────────────────────────────────────────────────────────────────────
║   IR:",
        );
        for line in ir_lines {
            result.push_str(&format!("\n║     {}", line));
        }
        result.push_str(
            "\n╚══════════════════════════════════════════════════════════════════════════════",
        );
        result
    }
}
