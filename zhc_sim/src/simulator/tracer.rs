use serde_json::{Value, json};
use std::{f64, fs::File, io::Write, mem::Discriminant, path::Path};

use super::*;
use zhc_utils::{
    FastMap,
    tracing::{Scope, Trace},
};

static NS_IN_US: f64 = 0.001;
static EVENTS_PID: usize = 0;
static SIMULATABLES_PID: usize = 1;
static COUNTERS_PID: usize = 2;

/// Tracks simulation state changes for a specific simulatable component.
pub struct SimulatableTracker {
    tid: usize,
    name: String,
    state_change: Option<Cycle>,
    state: Option<Value>,
}

/// Tracks event occurrences for a specific event type.
pub struct EventTracker {
    tid: usize,
    name: String,
}

/// Tracks numeric counter values over time.
pub struct CounterTracker {
    tid: usize,
    state: Option<f64>,
}

/// Records simulation events, component states, and counters for analysis and visualization.
pub struct Tracer<E: Event> {
    trace: Trace,
    // Events are added to the profile under pid 0
    event_trackers: FastMap<Discriminant<E>, EventTracker>,
    // Simulatables are added to the profile under pid 1
    simulatable_trackers: FastMap<usize, SimulatableTracker>,
    // Counters are added to the profile under pid 2
    counter_trackers: FastMap<String, CounterTracker>,
}

impl<E: Event> Tracer<E> {
    /// Creates a new tracer for recording simulation data.
    pub fn new() -> Self {
        let mut trace = Trace::default();
        trace.display_time_unit = Some(zhc_utils::tracing::Unit::Nanoseconds);
        trace.set_process_name(EVENTS_PID, "Events");
        trace.set_process_name(SIMULATABLES_PID, "Simulatables");
        trace.set_process_name(COUNTERS_PID, "Counters");
        let simulatable_trackers = FastMap::new();
        let event_trackers = FastMap::new();
        let counter_trackers = FastMap::new();
        Tracer {
            trace,
            simulatable_trackers,
            event_trackers,
            counter_trackers,
        }
    }

    /// Writes the complete trace data to a JSON file at the specified `path`.
    pub fn dump<P: AsRef<Path>>(&self, at: Cycle, path: P) {
        // We add the last states that were not flushed yet to the dumped trace
        let mut trace = self.trace.clone();
        for (_, tracker) in self.simulatable_trackers.iter() {
            trace.new_complete(
                tracker.state_change.as_ref().unwrap().as_ts(NS_IN_US),
                SIMULATABLES_PID,
                tracker.tid,
                &tracker.name,
                Some(json!({"val": tracker.state.as_ref().unwrap()})),
                (at - *tracker.state_change.as_ref().unwrap()).as_ts(NS_IN_US) - 5. * f64::EPSILON,
            );
        }
        let json = serde_json::to_string_pretty(&trace).expect("Failed to serialize trace.");
        let mut file = File::create(path.as_ref()).expect("Failed to create file");
        file.write_all(json.as_bytes())
            .expect("Failed to write to file");
    }

    /// Records a numeric counter `value` with the given `name` at the specified cycle.
    pub fn add_counter<S: AsRef<str>>(&mut self, at: Cycle, name: S, value: f64) {
        if !self.counter_trackers.contains_key(name.as_ref()) {
            let tid = self.counter_trackers.len() + 1;
            self.counter_trackers
                .insert(name.as_ref().into(), CounterTracker { tid, state: None });
            self.trace.set_thread_name(COUNTERS_PID, tid, name.as_ref());
        }

        let tracker = self.counter_trackers.get_mut(name.as_ref()).unwrap();

        if tracker.state != Some(value) {
            self.trace.new_counter(
                at.as_ts(NS_IN_US),
                COUNTERS_PID,
                tracker.tid,
                name,
                Some(json!({"state": value})),
            );
            tracker.state = Some(value);
        }
    }

    /// Records an event occurrence at the specified cycle.
    pub fn add_event(&mut self, at: Cycle, event: &E) {
        if !self
            .event_trackers
            .contains_key(&std::mem::discriminant(event))
        {
            let tid = self.event_trackers.len() + 1;
            let name = format!("{}", event);
            self.trace.set_thread_name(EVENTS_PID, tid, &name);
            self.event_trackers
                .insert(std::mem::discriminant(event), EventTracker { tid, name });
        }
        let tracker = self
            .event_trackers
            .get(&std::mem::discriminant(event))
            .unwrap();
        let state = serde_json::to_value(event).unwrap();
        self.trace.new_instant(
            at.as_ts(NS_IN_US),
            EVENTS_PID,
            tracker.tid,
            &tracker.name,
            Some(json!({"state": state})),
            Scope::Thread,
        );
    }

    /// Records the state of a simulatable component at the specified cycle.
    pub fn add_simulatable<S: Simulatable>(&mut self, at: Cycle, simulatable: &S) {
        let address = simulatable as *const S as usize;
        if !self.simulatable_trackers.contains_key(&address) {
            let tid = self.simulatable_trackers.len() + 1;
            let name = simulatable.name();
            self.trace.set_thread_name(SIMULATABLES_PID, tid, &name);
            self.simulatable_trackers.insert(
                address,
                SimulatableTracker {
                    tid,
                    state: None,
                    state_change: None,
                    name,
                },
            );
        }

        let tracker = self.simulatable_trackers.get_mut(&address).unwrap();
        let state = serde_json::to_value(simulatable).unwrap();
        if tracker.state.is_none() {
            tracker.state_change = Some(at);
            tracker.state = Some(state);
        } else if tracker.state.as_ref().unwrap() != &state {
            self.trace.new_complete(
                tracker.state_change.as_ref().unwrap().as_ts(NS_IN_US),
                SIMULATABLES_PID,
                tracker.tid,
                &tracker.name,
                Some(json!({"val": tracker.state.as_ref().unwrap()})),
                (at - *tracker.state_change.as_ref().unwrap()).as_ts(NS_IN_US) - 5. * f64::EPSILON,
            );
            tracker.state_change = Some(at);
            tracker.state = Some(state);
        }
    }
}
