use serde::Serialize;
use std::fmt::Display;

pub type RawDOp = hpuc_langs::doplang::Operations;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DOp {
    pub raw: RawDOp,
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
