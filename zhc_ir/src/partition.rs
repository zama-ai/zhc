use crate::{
    OpIdRaw,
    visualization::{StyleModifier, VisualAnnotation},
};
use zhc_utils::{Dumpable, graphics::ColorScale};

#[derive(Debug, Clone, Hash)]
pub struct PartitionId {
    pub id: OpIdRaw,
    pub metadata: std::sync::Arc<str>,
}

impl PartialEq for PartitionId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for PartitionId {}

impl PartialOrd for PartitionId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl Ord for PartitionId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartitionId {
    pub fn new(id: OpIdRaw, metadata: impl Into<std::sync::Arc<str>>) -> Self {
        Self {
            id,
            metadata: metadata.into(),
        }
    }
    pub fn fuse(a: &Self, b: &Self) -> Self {
        if a == b {
            a.clone()
        } else {
            let (first, second) = if a < b { (a, b) } else { (b, a) };
            Self {
                id: first.id,
                metadata: [first.metadata.as_ref(), "||", second.metadata.as_ref()]
                    .concat()
                    .into(),
            }
        }
    }
}

impl Dumpable for PartitionId {
    fn dump_to_string(&self) -> String {
        format!("Partition {:?}: {}", self.id, self.metadata)
    }
}

impl VisualAnnotation for PartitionId {
    fn style_modifier(&self) -> Option<StyleModifier> {
        Some(StyleModifier {
            fill_color: Some(
                ColorScale::RAINBOW.interpolate((self.id as f64 * 0.6180339887498949) % 1.0),
            ),
            ..Default::default()
        })
    }
}
pub struct PartitionTable(std::collections::BTreeSet<PartitionId>);
impl Dumpable for PartitionTable {
    fn dump_to_string(&self) -> String {
        self.0
            .iter()
            .map(|p| p.dump_to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<std::collections::BTreeSet<PartitionId>> for PartitionTable {
    fn from(value: std::collections::BTreeSet<PartitionId>) -> Self {
        Self(value)
    }
}
