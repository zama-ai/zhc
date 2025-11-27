use std::fmt::Display;

use hpuc_ir::DialectTypes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Types {
    Ctx,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Ctx => write!(f, "Ctx"),
        }
    }
}

impl DialectTypes for Types {}
