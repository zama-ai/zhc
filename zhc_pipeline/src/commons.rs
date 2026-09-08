use std::fmt::{Display, Formatter};

use serde::Serialize;

/// Fast, non-cryptographic hash of the pipeline's checked integer-level IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Fingerprint(pub u64);

impl Display for Fingerprint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SchedPolicy {
    AsSoonAsPossible,
    AsLateAsPossible,
}
