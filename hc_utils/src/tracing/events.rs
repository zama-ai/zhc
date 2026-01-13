use super::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Event {
    DurationBegin(DurationBeginEvent),
    DurationEnd(DurationEndEvent),
    Complete(CompleteEvent),
    Instant(InstantEvent),
    Metadata(MetadataEvent),
    Counter(CounterEvent),
}

#[derive(Serialize, Debug, Clone)]
pub struct DurationBeginEvent {
    pub name: String,
    #[serde(rename = "cat")]
    pub categories: Categories,
    pub ph: PhB,
    #[serde(rename = "ts")]
    pub timestamp: Microseconds,
    pub pid: Pid,
    pub tid: Tid,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tts")]
    pub thread_timestamp: Option<Microseconds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
    #[serde(flatten)]
    pub stack_trace: MaybeStackTrace,
}

#[derive(Serialize, Debug, Clone)]
pub struct DurationEndEvent {
    pub name: String,
    #[serde(rename = "cat")]
    pub categories: Categories,
    pub ph: PhE,
    #[serde(rename = "ts")]
    pub timestamp: Microseconds,
    pub pid: Pid,
    pub tid: Tid,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tts")]
    pub thread_timestamp: Option<Microseconds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
    #[serde(flatten)]
    pub stack_trace: MaybeStackTrace,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct CompleteEvent {
    pub name: String,
    #[serde(rename = "cat")]
    pub categories: Categories,
    pub ph: PhX,
    #[serde(rename = "ts")]
    pub timestamp: Microseconds,
    #[serde(rename = "dur")]
    pub duration: Microseconds,
    pub pid: Pid,
    pub tid: Tid,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tdur")]
    pub thread_duration: Option<Microseconds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct InstantEvent {
    pub name: String,
    #[serde(rename = "cat")]
    pub categories: Categories,
    pub ph: Phi,
    #[serde(rename = "ts")]
    pub timestamp: Microseconds,
    pub pid: Pid,
    pub tid: Tid,
    #[serde(rename = "s")]
    pub scope: Scope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub enum Metadata {
    #[serde(rename = "process_name")]
    #[default]
    ProcessName,
    #[serde(rename = "process_labels")]
    ProcessLabels,
    #[serde(rename = "process_sort_index")]
    ProcessSortIndex,
    #[serde(rename = "thread_name")]
    ThreadName,
    #[serde(rename = "thread_sort_index")]
    ThreadSortIndex,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(untagged)]
pub enum MetadataArgs {
    ProcessName {
        name: String,
    },
    ProcessLabels {
        labels: String,
    },
    ProcessSortIndex {
        sort_index: usize,
    },
    ThreadName {
        name: String,
    },
    ThreadSortIndex {
        sort_index: usize,
    },
    #[default]
    None,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct MetadataEvent {
    pub name: Metadata,
    pub ph: PhM,
    pub pid: Pid,
    pub tid: Tid,
    pub args: MetadataArgs,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct CounterEvent {
    pub name: String,
    #[serde(rename = "cat")]
    pub categories: Categories,
    pub ph: PhC,
    #[serde(rename = "ts")]
    pub timestamp: Microseconds,
    pub pid: Pid,
    pub tid: Tid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
}
