use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Unit {
    #[serde(rename = "ms")]
    Milliseconds,
    #[serde(rename = "ns")]
    Nanoseconds,
}
