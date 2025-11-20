use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use super::*;

#[derive(Serialize, Debug, Clone, Default)]
pub struct Trace {
    #[serde(rename = "traceEvents")]
    pub trace_events: Vec<Event>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayTimeUnit")]
    pub display_time_unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackFrames")]
    pub stack_frames: Option<HashMap<String, Value>>,
}

impl Trace {
    pub fn new_instant<S: AsRef<str>>(
        &mut self,
        timestamp: Microseconds,
        pid: Pid,
        tid: Tid,
        name: S,
        args: Option<Value>,
        scope: Scope,
    ) {
        self.trace_events.push(Event::Instant(InstantEvent {
            name: name.as_ref().into(),
            timestamp,
            pid,
            tid,
            scope,
            args,
            ..InstantEvent::default()
        }));
    }

    pub fn new_complete<S: AsRef<str>>(
        &mut self,
        timestamp: Microseconds,
        pid: Pid,
        tid: Tid,
        name: S,
        args: Option<Value>,
        duration: Microseconds,
    ) {
        self.trace_events.push(Event::Complete(CompleteEvent {
            name: name.as_ref().into(),
            timestamp,
            duration,
            pid,
            tid,
            args,
            ..CompleteEvent::default()
        }));
    }

    pub fn set_thread_name<S: AsRef<str>>(&mut self, pid: Pid, tid: Tid, name: S) {
        self.trace_events.push(Event::Metadata(MetadataEvent {
            name: Metadata::ThreadName,
            pid,
            tid,
            args: MetadataArgs::ThreadName {
                name: name.as_ref().into(),
            },
            ..MetadataEvent::default()
        }));
    }

    pub fn set_process_name<S: AsRef<str>>(&mut self, pid: Pid, name: S) {
        self.trace_events.push(Event::Metadata(MetadataEvent {
            name: Metadata::ProcessName,
            pid,
            args: MetadataArgs::ProcessName {
                name: name.as_ref().into(),
            },
            ..MetadataEvent::default()
        }));
    }

    pub fn new_counter<S: AsRef<str>>(
        &mut self,
        timestamp: Microseconds,
        pid: Pid,
        tid: Tid,
        name: S,
        args: Option<Value>,
    ) {
        self.trace_events.push(Event::Counter(CounterEvent {
            name: name.as_ref().into(),
            ph: PhC,
            timestamp,
            pid,
            tid,
            args,
            ..CounterEvent::default()
        }));
    }
}
