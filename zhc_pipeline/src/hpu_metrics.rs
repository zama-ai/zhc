//! HPU-level performance metrics.
//!
//! This module provides metrics computed after the full compilation pipeline, including
//! latency bounds, PE idle time, and batch size distribution.

use zhc_ir::IR;
use zhc_langs::{
    doplang::DopLang,
    hpulang::{HpuLang, get_batch_statistics},
};
use zhc_sim::{MHz, hpu::HpuConfig};
use zhc_utils::{Dumpable, data_visulization::Histogram};

use crate::latency::{compute_latency, compute_lower_bound};

/// HPU execution performance metrics.
///
/// Contains timing information and batch statistics computed by simulating the
/// compiled IR on the HPU model. All timing values are in microseconds.
pub struct HpuMetrics {
    /// Theoretical lower bound assuming perfect batching and linear-op hiding (µs).
    pub lower_bound: f64,
    /// Time the PBS processing element spent idle (µs).
    pub pep_idle: f64,
    /// Total simulated execution latency (µs).
    pub latency: f64,
    /// Distribution of PBS batch sizes.
    pub batch_stats: Histogram<u16>,
}

/// Computes HPU metrics from compiled IR.
///
/// Runs latency simulation on the device-operation IR and collects batch statistics
/// from the HPU-level IR.
pub fn compute_hpu_metrics(dop_ir: &IR<DopLang>, hpu_ir: &IR<HpuLang>) -> HpuMetrics {
    let config = HpuConfig::default();
    let lower_bound = compute_lower_bound(&dop_ir, &config);
    let lower_bound = lower_bound.as_ts(MHz::default().period());
    let (latency, pep_idle) = compute_latency(&dop_ir, &config);
    let latency = latency.as_ts(MHz::default().period());
    let pep_idle = pep_idle.as_ts(MHz::default().period());
    let batch_stats = get_batch_statistics(hpu_ir);
    HpuMetrics {
        lower_bound,
        pep_idle,
        latency,
        batch_stats,
    }
}

impl Dumpable for HpuMetrics {
    fn dump_to_string(&self) -> String {
        let batch_hist_str = self.batch_stats.dump_to_string();
        let batch_hist_lines: Vec<&str> = batch_hist_str.lines().collect();

        let mut result = format!(
            "╔══════════════════════════════════════════════════════════════════════════════
║ HPU Metrics
║──────────────────────────────────────────────────────────────────────────────
║   Lower Bound : {:.2} µs
║   Latency     : {:.2} µs
║   PePbs Idle  : {:.2} µs
║   Efficiency  : {:.1}%
║──────────────────────────────────────────────────────────────────────────────
║   Batch Size Histogram:",
            self.lower_bound,
            self.latency,
            self.pep_idle,
            100.0 * self.lower_bound / self.latency
        );
        for line in batch_hist_lines {
            result.push_str(&format!("\n║     {}", line));
        }
        result.push_str(
            "\n╚══════════════════════════════════════════════════════════════════════════════",
        );
        result
    }
}
