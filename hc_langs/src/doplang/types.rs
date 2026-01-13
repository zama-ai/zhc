use std::fmt::Display;

use hc_ir::DialectTypes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Types {
    Ctx(usize),
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Ctx(_) => write!(f, "Ctx"),
        }
    }
}

impl DialectTypes for Types {}
