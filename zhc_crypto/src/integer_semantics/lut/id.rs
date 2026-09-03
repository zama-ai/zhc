use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LutId(pub usize);

impl Display for LutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lut@{}", self.0)
    }
}
