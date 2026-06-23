use std::fmt::Display;

use serde::Serialize;
use zhc_ir::visualization::VisualAnnotation;
use zhc_utils::small::SmallSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Copy, Hash)]
pub struct HpuId(pub u8);

impl Display for HpuId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HPU_{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct TransferId(pub u8);

impl Display for TransferId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#!{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HpuLocality {
    OnHpu(HpuId),
    Transfer { from: HpuId, to: HpuId },
    Shared(SmallSet<HpuId>),
}

impl HpuLocality {
    pub fn is_on(&self, hid: &HpuId) -> bool {
        match self {
            HpuLocality::OnHpu(hpu_id) => hpu_id == hid,
            HpuLocality::Transfer { from, to } => from == hid || to == hid,
            HpuLocality::Shared(set) => set.contains(hid),
        }
    }
}

impl VisualAnnotation for HpuLocality {}
