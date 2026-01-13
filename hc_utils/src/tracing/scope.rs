use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub enum Scope {
    #[serde(rename = "g")]
    #[default]
    Global,
    #[serde(rename = "p")]
    Process,
    #[serde(rename = "t")]
    Thread,
}
