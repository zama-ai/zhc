use std::fmt::Display;

use crate::DialectTypeSystem;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayoutTypeSystem {
    Value,
}

impl Display for LayoutTypeSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutTypeSystem::Value => write!(f, "Value"),
        }
    }
}

impl DialectTypeSystem for LayoutTypeSystem {}
