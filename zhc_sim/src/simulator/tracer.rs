use serde::Serialize;
use serde_json::{Value, json};
use std::{f64, fs::File, io::Write, path::Path};

use super::*;
use zhc_utils::{
    FastMap,
    tracing::{Scope, Trace},
};

pub static PERIOD_IN_US: f64 = MHz(400).period();
static DEFAULT_EVENTS_PID: usize = 0;
static DEFAULT_STATES_PID: usize = 1;
static DEFAULT_COUNTERS_PID: usize = 2;

/// Controls the verbosity and overhead of simulation tracing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TracingLevel {
    /// Zero tracing. Zero overhead.
    None,
    /// Trace load counters only. Low overhead.
    Load,
    /// Trace load counters, component states, and events. High overhead.
    Events,
}

impl TracingLevel {
    /// Returns true if component states should be recorded.
    pub fn trace_simulatables(&self) -> bool {
        match self {
            TracingLevel::None => false,
            TracingLevel::Load => false,
            TracingLevel::Events => true,
        }
    }

    pub fn trace_states(&self) -> bool {
        match self {
            TracingLevel::None => false,
            TracingLevel::Load => true,
            TracingLevel::Events => true,
        }
    }

    /// Returns true if load counters should be recorded.
    pub fn trace_counters(&self) -> bool {
        match self {
            TracingLevel::None => false,
            TracingLevel::Load => true,
            TracingLevel::Events => true,
        }
    }

    /// Returns true if events should be recorded.
    pub fn trace_events(&self) -> bool {
        match self {
            TracingLevel::None => false,
            TracingLevel::Load => false,
            TracingLevel::Events => true,
        }
    }
}

/// Tracks simulation state changes for a specific simulatable component.
pub struct StateTracker {
    pid: usize,
    tid: usize,
    name: String,
    state_change: Option<Cycle>,
    state: Option<Value>,
}

/// Tracks event occurrences for a specific event type.
pub struct EventTracker {
    #[allow(unused)]
    pid: usize,
    tid: usize,
    name: String,
}

/// Tracks numeric counter values over time.
pub struct CounterTracker {
    #[allow(unused)]
    pid: usize,
    tid: usize,
    state: Option<f64>,
}

/// Records simulation events, component states, and counters for analysis and visualization.
pub struct Tracer {
    trace: Trace,
    event_trackers: FastMap<String, EventTracker>,
    state_trackers: FastMap<(usize, String), StateTracker>,
    counter_trackers: FastMap<(usize, String), CounterTracker>,
    groups_tids: FastMap<usize, usize>,
}

impl Tracer {
    /// Creates a new tracer for recording simulation data.
    pub fn new() -> Self {
        let mut trace = Trace::default();
        trace.display_time_unit = Some(zhc_utils::tracing::Unit::Nanoseconds);
        trace.set_process_name(DEFAULT_EVENTS_PID, "Events");
        trace.set_process_name(DEFAULT_STATES_PID, "States");
        trace.set_process_name(DEFAULT_COUNTERS_PID, "Counters");
        let state_trackers = FastMap::new();
        let event_trackers = FastMap::new();
        let counter_trackers = FastMap::new();
        let groups_tids = FastMap::new();
        Tracer {
            trace,
            state_trackers,
            event_trackers,
            counter_trackers,
            groups_tids,
        }
    }

    /// Writes the complete trace data to a JSON file at the specified `path`.
    pub fn dump<P: AsRef<Path>>(&self, at: Cycle, path: P) {
        // We add the last states that were not flushed yet to the dumped trace
        let mut trace = self.trace.clone();
        for (_, tracker) in self.state_trackers.iter() {
            trace.new_complete(
                tracker.state_change.as_ref().unwrap().as_ts(PERIOD_IN_US),
                tracker.pid,
                tracker.tid,
                &tracker.name,
                Some(json!({"val": tracker.state.as_ref().unwrap()})),
                // Shrink the slice by a few ULP *at the end's magnitude* so that, after the
                // trace viewer recomputes `ts + dur`, it stays strictly below the next slice's
                // `ts` and the two render as siblings rather than nested (see `add_state`).
                (at - *tracker.state_change.as_ref().unwrap()).as_ts(PERIOD_IN_US)
                    - at.as_ts(PERIOD_IN_US).abs() * 4. * f64::EPSILON,
            );
        }
        let json = serde_json::to_string_pretty(&trace).expect("Failed to serialize trace.");
        let mut file = File::create(path.as_ref()).expect("Failed to create file");
        file.write_all(json.as_bytes())
            .expect("Failed to write to file");
    }

    /// Records a numeric counter `value` with the given `name` at the specified cycle.
    ///
    /// Recording occurs only if `tracing_level` enables counters.
    pub fn add_counter<S: AsRef<str>>(
        &mut self,
        tracing_level: TracingLevel,
        at: Cycle,
        group: Option<usize>,
        name: S,
        value: f64,
    ) {
        if tracing_level.trace_counters() {
            let pid = group.map(|a| a + 3).unwrap_or(DEFAULT_COUNTERS_PID);
            let name = name.as_ref().to_string();
            let key = (pid, name);
            if !self.counter_trackers.contains_key(&key) {
                let tid = self.groups_tids.get(&pid).cloned().unwrap_or(0) + 1;
                self.groups_tids.insert(pid, tid);
                self.counter_trackers.insert(
                    key.clone(),
                    CounterTracker {
                        pid,
                        tid,
                        state: None,
                    },
                );
                self.trace.set_thread_name(pid, tid, key.1.as_str());
            }

            let tracker = self.counter_trackers.get_mut(&key).unwrap();
            if tracker.state != Some(value) {
                self.trace.new_counter(
                    at.as_ts(PERIOD_IN_US),
                    pid,
                    tracker.tid,
                    key.1,
                    Some(json!({"state": value})),
                );
                tracker.state = Some(value);
            }
        }
    }

    /// Records an event occurrence at the specified cycle.
    ///
    /// Recording occurs only if `tracing_level` enables events.
    pub fn add_event<E: Event>(
        &mut self,
        tracing_level: TracingLevel,
        at: Cycle,
        event: &E,
    ) {
        if tracing_level.trace_events() {
            let pid = DEFAULT_EVENTS_PID;
            let event_name = format!("{}", event);
            if !self.event_trackers.contains_key(&event_name) {
                let tid = self.event_trackers.len() + 1;
                let name = format!("{}", event);
                self.trace.set_thread_name(pid, tid, &name);
                self.event_trackers
                    .insert(event_name.clone(), EventTracker { pid, tid, name });
            }
            let tracker = self.event_trackers.get(&event_name).unwrap();
            let state = serde_json::to_value(event).unwrap();
            self.trace.new_instant(
                at.as_ts(PERIOD_IN_US),
                pid,
                tracker.tid,
                &tracker.name,
                Some(json!({"state": state})),
                Scope::Thread,
            );
        }
    }

    /// Records the state of a simulatable component at the specified cycle.
    ///
    /// Recording occurs only if `tracing_level` enables simulatables.
    pub fn add_state<S: Serialize, St: AsRef<str>>(
        &mut self,
        tracing_level: TracingLevel,
        at: Cycle,
        group: Option<usize>,
        name: St,
        state: &S,
    ) {
        if tracing_level.trace_states() {
            let pid = group.map(|a| a + 3).unwrap_or(DEFAULT_STATES_PID);
            let name = name.as_ref().to_string();
            let key = (pid, name);
            if !self.state_trackers.contains_key(&key) {
                let tid = self.groups_tids.get(&pid).cloned().unwrap_or(0) + 1;
                self.groups_tids.insert(pid, tid);
                self.trace.set_thread_name(pid, tid, &key.1);
                let name = key.1.clone();
                self.state_trackers.insert(
                    key.clone(),
                    StateTracker {
                        pid,
                        tid,
                        state: None,
                        state_change: None,
                        name,
                    },
                );
            }

            let tracker = self.state_trackers.get_mut(&key).unwrap();
            let state = serde_json::to_value(state).unwrap();
            if tracker.state.is_none() {
                tracker.state_change = Some(at);
                tracker.state = Some(state);
            } else if tracker.state.as_ref().unwrap() != &state {
                self.trace.new_complete(
                    tracker.state_change.as_ref().unwrap().as_ts(PERIOD_IN_US),
                    pid,
                    tracker.tid,
                    &tracker.name,
                    Some(json!({"val": tracker.state.as_ref().unwrap()})),
                    (at - *tracker.state_change.as_ref().unwrap()).as_ts(PERIOD_IN_US)
                        - at.as_ts(PERIOD_IN_US).abs() * 4. * f64::EPSILON,
                );
                tracker.state_change = Some(at);
                tracker.state = Some(state);
            }
        }
    }

    /// Returns a reference to the underlying trace.
    pub fn trace(&self) -> &Trace {
        &self.trace
    }
}
