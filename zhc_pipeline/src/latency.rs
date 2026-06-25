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
        Simulator::from_simulatable(config.freq, Hpu::new(&config, HpuId(0)), zhc_sim::TracingLevel::Load);
    let dops = ir
        .walk_ops_linear()
        .map(|a| DOp {
            raw: a.get_instruction(),
            id: DOpId(a.get_id().into()),
        })
        .collect();
    let event = Events::IscPushDOps(dops);
    simulator.dispatch(event);
    simulator.play_until_event(Events::IscProcessOver);
    let idle_duration = compute_pe_pbs_idle_duration(&simulator);
    (simulator.now(), idle_duration)
}

/// Diagnostic: how PEP idle time correlates with PEM activity.
///
/// Returns `(total, pep_idle, pep_idle_while_pem_busy, pem_busy)` in cycles.
/// If `pep_idle_while_pem_busy` ≈ `pep_idle`, the PEP is being starved by PEM
/// traffic (spill/unspill) — i.e. unspills are gating batches.
pub fn compute_pe_overlap(ir: &IR<DopLang>, config: &HpuConfig) -> (usize, usize, usize, usize) {
    let mut simulator =
        Simulator::from_simulatable(config.freq, Hpu::new(&config, HpuId(0)), zhc_sim::TracingLevel::Load);
    let dops = ir
        .walk_ops_linear()
        .map(|a| DOp {
            raw: a.get_instruction(),
            id: DOpId(a.get_id().into()),
        })
        .collect();
    simulator.dispatch(Events::IscPushDOps(dops));
    simulator.play_until_event(Events::IscProcessOver);
    let end = simulator.now().0;

    // Extract (cycle, state) samples for a named counter.
    let samples = |name: &str| -> Vec<(usize, f64)> {
        let mut v: Vec<(usize, f64)> = simulator
            .get_tracer()
            .trace()
            .trace_events
            .iter()
            .filter_map(|e| {
                if let Event::Counter(c) = e {
                    if c.name == name {
                        let state = c.args.as_ref()?.get("state")?.as_f64()?;
                        let cycle = (c.timestamp / MHz::default().period()).round() as usize;
                        return Some((cycle, state));
                    }
                }
                None
            })
            .collect();
        v.sort_by_key(|(t, _)| *t);
        v
    };

    let pep = samples("pe_pbs_working");
    let pem = samples("pe_mem_busy");

    // Merge the two step-functions and integrate over [0, end).
    let mut change_points: Vec<usize> = pep.iter().chain(pem.iter()).map(|(t, _)| *t).collect();
    change_points.push(end);
    change_points.sort_unstable();
    change_points.dedup();

    let state_at = |series: &[(usize, f64)], t: usize| -> f64 {
        // step function: value of the last sample at or before t (0 before first)
        let mut val = 0.0;
        for (ts, s) in series {
            if *ts <= t {
                val = *s;
            } else {
                break;
            }
        }
        val
    };

    let (mut pep_idle, mut pep_idle_pem_busy, mut pem_busy) = (0usize, 0usize, 0usize);
    for w in change_points.windows(2) {
        let (t, t_next) = (w[0], w[1]);
        let len = t_next - t;
        let pep_working = state_at(&pep, t) > 0.0;
        let pem_on = state_at(&pem, t) > 0.0;
        if pem_on {
            pem_busy += len;
        }
        if !pep_working {
            pep_idle += len;
            if pem_on {
                pep_idle_pem_busy += len;
            }
        }
    }
    (end, pep_idle, pep_idle_pem_busy, pem_busy)
}

/// Diagnostic: PEP idle / memory-gating split into `n` equal time windows.
///
/// Returns one `(pep_idle, pep_idle_while_pem_busy)` pair per window, in cycle
/// order. `gated_w = pep_idle_while_pem_busy / pep_idle` per window shows whether
/// the *late* part of the run (last windows) is starved by memory (reloads) —
/// the whole-op average hides this.
/// Each entry: `(window_cycles, pep_idle, pep_idle_while_pem_busy)`.
pub fn compute_pe_overlap_windowed(
    ir: &IR<DopLang>,
    config: &HpuConfig,
    n: usize,
) -> Vec<(usize, usize, usize)> {
    let mut simulator =
        Simulator::from_simulatable(config.freq, Hpu::new(&config, HpuId(0)), zhc_sim::TracingLevel::Load);
    let dops = ir
        .walk_ops_linear()
        .map(|a| DOp {
            raw: a.get_instruction(),
            id: DOpId(a.get_id().into()),
        })
        .collect();
    simulator.dispatch(Events::IscPushDOps(dops));
    simulator.play_until_event(Events::IscProcessOver);
    let end = simulator.now().0;

    let samples = |name: &str| -> Vec<(usize, f64)> {
        let mut v: Vec<(usize, f64)> = simulator
            .get_tracer()
            .trace()
            .trace_events
            .iter()
            .filter_map(|e| {
                if let Event::Counter(c) = e {
                    if c.name == name {
                        let state = c.args.as_ref()?.get("state")?.as_f64()?;
                        let cycle = (c.timestamp / MHz::default().period()).round() as usize;
                        return Some((cycle, state));
                    }
                }
                None
            })
            .collect();
        v.sort_by_key(|(t, _)| *t);
        v
    };

    let pep = samples("pe_pbs_working");
    let pem = samples("pe_mem_busy");

    let mut change_points: Vec<usize> = pep.iter().chain(pem.iter()).map(|(t, _)| *t).collect();
    change_points.push(end);
    change_points.sort_unstable();
    change_points.dedup();

    let state_at = |series: &[(usize, f64)], t: usize| -> f64 {
        let mut val = 0.0;
        for (ts, s) in series {
            if *ts <= t {
                val = *s;
            } else {
                break;
            }
        }
        val
    };

    let mut out = vec![(0usize, 0usize, 0usize); n];
    // Window lengths (cycles), distributing the remainder onto the last window.
    for (w, slot) in out.iter_mut().enumerate() {
        let lo = w * end / n;
        let hi = if w + 1 == n { end } else { (w + 1) * end / n };
        slot.0 = hi - lo;
    }
    for w in change_points.windows(2) {
        let (t, t_next) = (w[0], w[1]);
        let pep_working = state_at(&pep, t) > 0.0;
        let pem_on = state_at(&pem, t) > 0.0;
        if pep_working {
            continue;
        }
        // Split the [t, t_next) idle slice across the windows it spans.
        let mut cur = t;
        while cur < t_next {
            let win = (cur * n / end.max(1)).min(n - 1);
            let win_hi = ((win + 1) * end + n - 1) / n; // ceil((win+1)*end/n)
            let seg_hi = t_next.min(win_hi.max(cur + 1));
            let len = seg_hi - cur;
            out[win].1 += len;
            if pem_on {
                out[win].2 += len;
            }
            cur = seg_hi;
        }
    }
    out
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
