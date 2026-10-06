//! Multi-HPU performance metrics.
//!
//! This module provides metrics computed after the full multi-HPU compilation pipeline, by
//! simulating the per-board DOP streams on the multi-HPU model.

use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_sim::{
    Simulator, TracingLevel,
    hpu::{DOp, DOpId},
    multi_hpu::{Events, MultiHpu},
};
use zhc_utils::{Dumpable, units::Microseconds};

/// Multi-HPU execution performance metrics.
///
/// Computed by simulating the compiled per-board DOP streams on the multi-HPU model. All
/// timing values are in microseconds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiHpuMetrics {
    /// Total simulated execution latency of the whole system (µs).
    pub latency: Microseconds,
}

impl Dumpable for MultiHpuMetrics {
    fn dump_to_string(&self) -> String {
        format!(
            "╔══════════════════════════════════════════════════════════════════════════════
║ Multi-HPU Metrics
║──────────────────────────────────────────────────────────────────────────────
║   Latency          : {}
╚══════════════════════════════════════════════════════════════════════════════",
            self.latency,
        )
    }
}

pub(crate) fn compute_multi_hpu_metrics(
    irs: &[IR<DopLang>],
    config: &MultiHpuConfig,
) -> MultiHpuMetrics {
    let latency = simulate(irs, config);
    MultiHpuMetrics { latency }
}

fn simulate(irs: &[IR<DopLang>], config: &MultiHpuConfig) -> Microseconds {
    let streams: Vec<Vec<DOp>> = irs
        .iter()
        .map(|ir| {
            ir.walk_ops_linear()
                .map(|a| DOp {
                    raw: a.get_instruction().clone(),
                    id: DOpId(a.get_id().into()),
                })
                .collect()
        })
        .collect();
    let mut simulator = Simulator::from_simulatable(
        config.hpu_config.freq,
        MultiHpu::new(config),
        TracingLevel::None,
    );
    simulator.dispatch(Events::PushDOps(streams));
    simulator.play_until_event(Events::ProcessOver);
    simulator.now_us()
}
