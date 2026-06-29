use serde::Serialize;
use std::fmt::Display;

/// Raw operation type from the DOp language specification.
pub type RawDOp = zhc_langs::doplang::DopInstructionSet;

/// Unique identifier for a DOp operation within the simulation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash, serde::Deserialize)]
pub struct DOpId(pub usize);

impl Display for DOpId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl Serialize for DOpId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Discrete operation combining raw operation data with a unique identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DOp {
    /// The underlying operation specification from the DOp language.
    pub raw: RawDOp,
    /// Unique identifier for tracking this operation through the simulation.
    pub id: DOpId,
}

impl Display for DOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.raw)
    }
}

impl Serialize for DOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl PartialOrd for DOp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for DOp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
