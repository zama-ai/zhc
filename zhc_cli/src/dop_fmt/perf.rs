//! `--perf`: runs a loaded `DopLang` program through `zhc_sim`'s real discrete-event HPU
//! simulator (not the interpreter behind `--sim`) to estimate hardware latency and PBS batching
//! behavior.
//!
//! Mirrors `zhc_pipeline::hpu::metrics::compute_hpu_metrics`'s decomposition
//! (`latency = lower_bound + batching_overhead + starvation`), minus the one field
//! (`batch_stats`, a histogram of the *compiler's own* declared batch groupings) that needs a
//! pre-lowering `IR<HpuLang>` — something a hand-written or hex-loaded `IR<DopLang>` never has,
//! since that intermediate only exists inside `Pipeline`'s IOp-to-DOp compilation. Everything
//! else here only needs `(IR<DopLang>, HpuConfig)`, so it's reproduced directly rather than
//! depending on that (crate-private) helper.

use zhc_config::hpu::HpuConfig;
use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_langs::hpulang::HpuId;
use zhc_sim::hpu::{DOp, DOpId, Events, Hpu, Statistics};
use zhc_sim::{Simulator, TracingLevel};
use zhc_utils::units::{Cycle, Microseconds};

/// A `--perf-rpt` report category. Each is printed independently; passing `--perf-rpt` more than
/// once toggles on more than one.
#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfReport {
    /// Total simulated execution latency.
    Latency,
    /// Theoretical lower bound (perfect batching, linear-op hiding) vs. the actual overhead paid
    /// for under-filled batches and PBS-engine starvation.
    LowerBound,
    /// PBS batch count, slot occupancy, and timeout-triggered (rather than full-batch) launches.
    PbsBatch,
}

impl PerfReport {
    /// Every report category, in the order they print by default when `--perf` is given without
    /// an explicit `--perf-rpt`.
    pub const ALL: &[PerfReport] = &[PerfReport::Latency, PerfReport::LowerBound, PerfReport::PbsBatch];
}

/// Performance metrics for one simulated run of a `DopLang` program.
pub struct PerfMetrics {
    pub latency: Microseconds,
    pub lower_bound: Microseconds,
    pub batching_overhead: Microseconds,
    pub starvation: Microseconds,
    pub batch_count: usize,
    pub slots_filled: usize,
    pub slots_total: usize,
    pub timeout_launches: u16,
}

impl PerfMetrics {
    /// Prints one line per requested `reports` category.
    pub fn print(&self, reports: &[PerfReport]) {
        for report in reports {
            match report {
                PerfReport::Latency => println!("[perf:latency] {}", self.latency),
                PerfReport::LowerBound => println!(
                    "[perf:lower_bound] lower_bound={} batching_overhead={} starvation={}",
                    self.lower_bound, self.batching_overhead, self.starvation
                ),
                PerfReport::PbsBatch => {
                    let occupancy = if self.slots_total > 0 {
                        100.0 * self.slots_filled as f64 / self.slots_total as f64
                    } else {
                        0.0
                    };
                    println!(
                        "[perf:pbs_batch] batches={} slots={}/{} ({occupancy:.1}%) timeout_launches={}",
                        self.batch_count, self.slots_filled, self.slots_total, self.timeout_launches
                    );
                }
            }
        }
    }
}

/// Runs `ir` through the real HPU discrete-event simulator, using `HpuConfig::default()`
/// (matching `zhc_pipeline`'s own metrics computation), and returns its performance metrics.
///
/// # Panics
///
/// Panics if the program contains an instruction the simulator can't schedule (mirrors the
/// simulator's own behavior — unlike `--sim`, this isn't caught, since a scheduling failure here
/// reflects a program the real hardware couldn't run either).
pub fn run(ir: &IR<DopLang>) -> PerfMetrics {
    let config = HpuConfig::default();
    let period = config.freq.period();

    let lower_bound = compute_lower_bound(ir, &config).as_ts(period);
    let (latency, stats) = simulate(ir, &config);
    let latency = latency.as_ts(period);
    let pbs_busy = stats.pbs_busy.as_ts(period);

    PerfMetrics {
        latency,
        lower_bound,
        batching_overhead: Microseconds(pbs_busy.0 - lower_bound.0),
        starvation: Microseconds(latency.0 - pbs_busy.0),
        batch_count: stats.pbs_batches,
        slots_filled: stats.pbs_slots_filled,
        slots_total: stats.pbs_batches * config.pbs_max_batch_size,
        timeout_launches: stats.timeouts,
    }
}

fn compute_lower_bound(ir: &IR<DopLang>, config: &HpuConfig) -> Cycle {
    let pbses_count = ir
        .walk_ops_linear()
        .filter(|op| op.get_instruction().is_pbs())
        .count();
    if pbses_count == 0 {
        return Cycle(0);
    }
    let n_batches = pbses_count.div_ceil(config.pbs_max_batch_size);
    let padded_slots = pbses_count.max(n_batches * config.pbs_processing_latency_m);
    Cycle(
        config.pbs_processing_latency_a * padded_slots
            + config.pbs_processing_latency_b * n_batches,
    )
}

fn simulate(ir: &IR<DopLang>, config: &HpuConfig) -> (Cycle, Statistics) {
    let mut simulator = Simulator::from_simulatable(
        config.freq,
        Hpu::new(config, HpuId(0)),
        TracingLevel::None,
    );
    let dops = ir
        .walk_ops_linear()
        .map(|op| DOp {
            raw: op.get_instruction().clone(),
            id: DOpId(op.get_id().into()),
        })
        .collect();
    simulator.dispatch(Events::UCorePushDOps(dops));
    simulator.play_until_event(Events::UCoreStarved);
    let latency = simulator.now();
    let stats = std::mem::take(&mut simulator.simulatable_mut().statistics);
    (latency, stats)
}
