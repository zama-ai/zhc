use std::path::Path;

use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_sim::{
    Cycle, MHz, Simulator,
    hpu::{DOp, DOpId, Events, Hpu, HpuConfig},
};
use zhc_utils::tracing::Event;

pub fn trace_execution(ir: &IR<DopLang>, config: &HpuConfig, path: impl AsRef<Path>) {
    let mut simulator =
        Simulator::from_simulatable(config.freq, Hpu::new(&config), zhc_sim::TracingLevel::Load);
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
    simulator.dump_trace(path.as_ref());
}

pub fn compute_pe_pbs_idle_duration(simulator: &Simulator<Hpu>) -> Cycle {
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
                    let cycle = (c.timestamp / MHz(400).period()).round() as usize;
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
