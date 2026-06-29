//! Latency computation for HPU operations.
//!
//! This module provides functionality to compute execution latency for device
//! operations by simulating their execution on the target HPU hardware. The
//! latency computation takes into account hardware characteristics, operation
//! dependencies, and execution pipeline behavior.

use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_sim::{
    Cycle, MHz, Simulator,
    hpu::{DOp, DOpId, Events, FlatLinLatency, Hpu, HpuConfig, HpuId},
};
use zhc_utils::tracing::Event;

/// Computes the lower bound on the execution latency.
///
/// This lower bound is computed assuming we have a perfect batching, and a hiding of every linear
/// operations behind pbs batches..
pub fn compute_lower_bound(ir: &IR<DopLang>, config: &HpuConfig) -> Cycle {
    let pbses_count = ir
        .walk_ops_linear()
        .filter(|op| op.get_instruction().is_pbs())
        .count();
    let n_full = pbses_count.div_euclid(config.pbs_max_batch_size);
    let last_batch_length = pbses_count.rem_euclid(config.pbs_max_batch_size);
    let model = FlatLinLatency::new(
        config.pbs_processing_latency_a,
        config.pbs_processing_latency_b,
        config.pbs_processing_latency_m,
    );
    model.compute_latency(config.pbs_max_batch_size) * n_full
        + model.compute_latency(last_batch_length)
}

/// Computes the execution latency and PE idle time for the given device operation IR.
///
/// Takes an intermediate representation `ir` containing device operations and
/// the hardware configuration `config` to simulate execution. Returns a tuple of
/// `(total_latency, pbs_pe_idle_time)` in cycles.
pub fn compute_latency(ir: &IR<DopLang>, config: &HpuConfig) -> (Cycle, Cycle) {
    let mut simulator =
        Simulator::from_simulatable(config.freq, Hpu::new(&config, HpuId(0)), zhc_sim::TracingLevel::None);
    let dops = ir
        .walk_ops_linear()
        .map(|a| DOp {
            raw: a.get_instruction(),
            id: DOpId(a.get_id().into()),
        })
        .collect();
    let event = Events::UCorePushDOps(dops);
    simulator.dispatch(event);
    simulator.play_until_event(Events::IscProcessOver);
    let idle_duration = compute_pe_pbs_idle_duration(&simulator);
    (simulator.now(), idle_duration)
}

fn compute_pe_pbs_idle_duration(simulator: &Simulator<Hpu>) -> Cycle {
    let end_time = simulator.now().0;

    let mut events: Vec<(usize, f64)> = simulator
        .get_tracer()
        .trace()
        .trace_events
        .iter()
        .filter_map(|e| {
            if let Event::Counter(c) = e {
                if c.name == "pe_pbs_working" {
                    let state = c.args.as_ref()?.get("state")?.as_f64()?;
                    // Timestamp is stored as cycle * MHz(400).period(), convert back to cycles
                    let cycle = (c.timestamp / MHz::default().period()).round() as usize;
                    return Some((cycle, state));
                }
            }
            None
        })
        .collect();

    // Sort by timestamp
    events.sort_by_key(|(ts, _)| *ts);

    // Integrate idle time (state = 0.0)
    let mut idle_duration: usize = 0;
    let mut last_ts: usize = 0;
    let mut last_state = 0.0; // Assume idle at start

    for (ts, state) in events {
        if last_state == 0.0 {
            idle_duration += ts - last_ts;
        }
        last_ts = ts;
        last_state = state;
    }

    // Account for final period up to end_time
    if last_state == 0.0 {
        idle_duration += end_time - last_ts;
    }

    Cycle(idle_duration)
}
