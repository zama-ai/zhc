//! PBS-level circuit analysis metrics.
//!
//! This module computes metrics on the IOP-level IR before HPU compilation, focusing
//! on PBS operation counts, critical path analysis, and scheduling slack.

use zhc_ir::{AnnIR, IR};
use zhc_langs::ioplang::IopLang;
use zhc_utils::{Dumpable, SafeAs, data_visulization::Histogram, svec};

/// PBS operation metrics computed from IOP-level IR.
///
/// These metrics characterize the circuit's parallelism and scheduling flexibility
/// before compilation to the HPU target.
pub struct PbsMetrics {
    /// Total number of PBS operations in the circuit.
    pub count: usize,
    /// Length of the longest PBS-chain (critical path depth).
    pub critical_length: usize,
    /// Distribution of scheduling slack values across PBS operations.
    pub slack_stats: Histogram<u16>,
    /// Distribution of PBS operations per depth level on the critical path.
    pub level_stats: Histogram<u16>,
}

/// Per-operation criticality annotation used during dataflow analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Criticallity {
    /// Distance from inputs (PBS-hops).
    depth: u16,
    /// Distance to outputs (PBS-hops).
    height: u16,
    /// Scheduling flexibility: critical_length - depth - height + 1.
    slack: u16,
}

/// Annotates each operation with its criticality (depth, height, slack).
fn analyze(ir: &IR<IopLang>) -> AnnIR<'_, IopLang, Criticallity, ()> {
    let a = ir.forward_dataflow_analysis(|opref| {
        let previous_depth = opref
            .get_predecessors_iter()
            .map(|p| p.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0);
        if opref.get_instruction().is_pbs() {
            (previous_depth + 1, svec![(); opref.get_return_arity()])
        } else {
            (previous_depth, svec![(); opref.get_return_arity()])
        }
    });
    let critical_path_length = a
        .walk_ops_linear()
        .map(|op| *op.get_annotation())
        .max()
        .unwrap();
    a.backward_dataflow_analysis::<Criticallity, ()>(|opref, old_opref| {
        let depth = *old_opref.get_annotation();
        let previous_height = opref
            .get_users_iter()
            .map(|p| p.get_annotation().clone().unwrap_analyzed().height)
            .max()
            .unwrap_or(0);

        if opref.get_instruction().is_pbs() {
            (
                Criticallity {
                    depth,
                    height: previous_height + 1,
                    slack: critical_path_length - depth - previous_height + 1,
                },
                svec![(); opref.get_return_arity()],
            )
        } else {
            (
                Criticallity {
                    depth,
                    height: previous_height,
                    slack: critical_path_length - depth - previous_height,
                },
                svec![(); opref.get_return_arity()],
            )
        }
    })
}

/// Computes PBS metrics from IOP-level IR.
///
/// Performs forward and backward dataflow analysis to determine each PBS operation's
/// depth (distance from inputs) and height (distance to outputs), from which slack
/// is derived.
pub fn compute_pbs_metrics(ir: &IR<IopLang>) -> PbsMetrics {
    let count = ir
        .walk_ops_linear()
        .filter(|op| op.get_instruction().is_pbs())
        .count();
    let analyzed = analyze(ir);
    let critical_length = analyzed
        .walk_ops_linear()
        .map(|op| op.get_annotation().depth)
        .max()
        .unwrap()
        .sas::<usize>();
    let mut slack_stats = Histogram::empty();
    let mut level_stats = Histogram::empty();
    for op in analyzed
        .walk_ops_linear()
        .filter(|op| op.get_instruction().is_pbs())
    {
        slack_stats.count(&op.get_annotation().slack);
        if op.get_annotation().slack == 1 {
            level_stats.count(&op.get_annotation().depth);
        }
    }

    PbsMetrics {
        count,
        critical_length,
        slack_stats,
        level_stats,
    }
}

impl Dumpable for PbsMetrics {
    fn dump_to_string(&self) -> String {
        let slack_hist_str = self.slack_stats.dump_to_string();
        let slack_hist_lines: Vec<&str> = slack_hist_str.lines().collect();

        let level_hist_str = self.level_stats.dump_to_string();
        let level_hist_lines: Vec<&str> = level_hist_str.lines().collect();

        let mut result = format!(
            "╔══════════════════════════════════════════════════════════════════════════════
║ PBS Metrics
║──────────────────────────────────────────────────────────────────────────────
║   Count : {}
║   Critical Path Length : {}
║──────────────────────────────────────────────────────────────────────────────
║   Slack Histogram:",
            self.count, self.critical_length
        );
        for line in slack_hist_lines {
            result.push_str(&format!("\n║     {}", line));
        }
        result.push_str("\n║──────────────────────────────────────────────────────────────────────────────\n║   Level Histogram:");
        for line in level_hist_lines {
            result.push_str(&format!("\n║     {}", line));
        }
        result.push_str(
            "\n╚══════════════════════════════════════════════════════════════════════════════",
        );
        result
    }
}
