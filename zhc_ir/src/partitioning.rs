use std::rc::Rc;

use serde::Serialize;
use zhc_utils::{
    Dumpable, StoreIndex, fsm, graphics::ColorScale, iter::ReconcilerOf3, small::SmallSet,
};

use crate::visualization::{
    DynamicElement, Fill, NoClass, StyleModifier, TextBox, VisualAnnotation,
};

#[derive(Serialize, Debug, Clone, Copy, StoreIndex, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionId(pub u16);

impl VisualAnnotation for PartitionId {
    fn style_modifier(&self) -> Option<StyleModifier> {
        let fill = Fill::Solid(
            ColorScale::RAINBOW.interpolate((self.0 as f64 * 0.6180339887498949) % 1.0),
        );
        Some(StyleModifier {
            fill: Some(fill),
            ..Default::default()
        })
    }
}

#[fsm]
#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartitionPlacement {
    Irrelevant,
    Exclusive(PartitionId),
    Shared(SmallSet<PartitionId>),
}

impl VisualAnnotation for PartitionPlacement {
    fn style_modifier(&self) -> Option<StyleModifier> {
        let fill = match self {
            PartitionPlacement::Irrelevant => return None,
            PartitionPlacement::Exclusive(pid) => Fill::Solid(
                ColorScale::RAINBOW.interpolate((pid.0 as f64 * 0.6180339887498949) % 1.0),
            ),
            PartitionPlacement::Shared(pids) => Fill::stripes_with_width(
                pids.clone().into_iter().map(|pid| {
                    ColorScale::RAINBOW.interpolate((pid.0 as f64 * 0.6180339887498949) % 1.0)
                }),
                20.,
            ),
            _ => unreachable!(),
        };

        Some(StyleModifier {
            fill: Some(fill),
            ..Default::default()
        })
    }
}

impl PartitionPlacement {
    pub fn iter_pids(&self) -> impl Iterator<Item = PartitionId> + use<> {
        match self {
            PartitionPlacement::Irrelevant => std::iter::empty().reconcile_1_of_3(),
            PartitionPlacement::Exclusive(pid) => std::iter::once(*pid).reconcile_2_of_3(),
            PartitionPlacement::Shared(pids) => pids.clone().into_iter().reconcile_3_of_3(),
            _ => unreachable!(),
        }
    }

    pub fn add_pid(&mut self, pid: &PartitionId) {
        self.transition(|old| match old {
            PartitionPlacement::Irrelevant => PartitionPlacement::Exclusive(*pid),
            PartitionPlacement::Exclusive(opid) => {
                PartitionPlacement::Shared([opid, *pid].into_iter().collect())
            }
            PartitionPlacement::Shared(mut pids) => PartitionPlacement::Shared({
                pids.insert(*pid);
                pids
            }),
            _ => unreachable!(),
        });
    }

    pub fn is_relevant(&self) -> bool {
        match self {
            PartitionPlacement::Irrelevant => false,
            PartitionPlacement::Exclusive(_) => true,
            PartitionPlacement::Shared(_) => true,
            _ => unreachable!(),
        }
    }
}

impl FromIterator<PartitionId> for PartitionPlacement {
    fn from_iter<T: IntoIterator<Item = PartitionId>>(iter: T) -> Self {
        let mut placement = PartitionPlacement::Irrelevant;
        for pid in iter {
            placement.add_pid(&pid);
        }
        placement
    }
}

/// A labelled cluster of IR operations forming a single unit of computation.
///
/// A partition groups operations that are close in the graph into one task, in the sense of
/// parallel compilation. The `id` field carries the partition's identity — a sequence number
/// assigned when the partition is created — while `metadata` holds a human-readable label
/// describing what the group represents.
///
/// Identity rests on `id` alone: two partitions with the same `id` compare equal and order
/// identically regardless of their `metadata`, and ordering follows the numeric `id`. This lets
/// partitions be de-duplicated and sorted purely by identity while retaining a descriptive label
/// for display.
#[derive(Debug, Clone, Hash)]
pub struct PartitionAnnotation {
    /// The partition's identity, a sequence number assigned at creation.
    ///
    /// Equality, ordering, and hashing of a [`PartitionId`] derive from this field alone.
    pub placement: PartitionPlacement,

    /// A human-readable label describing what the partition represents.
    pub metadata: Rc<str>,
}

impl PartialEq for PartitionAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.placement == other.placement
    }
}
impl Eq for PartitionAnnotation {}

impl PartialOrd for PartitionAnnotation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use PartitionPlacement::*;
        match (&self.placement, &other.placement) {
            (Irrelevant, Irrelevant) => Some(std::cmp::Ordering::Equal),
            (Irrelevant, Exclusive(_)) => Some(std::cmp::Ordering::Less),
            (Exclusive(s), Exclusive(o)) => s.partial_cmp(&o),
            (Exclusive(_), Irrelevant) => Some(std::cmp::Ordering::Greater),
            _ => unreachable!(),
        }
    }
}

impl Ord for PartitionAnnotation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartitionAnnotation {
    pub fn new_irrelevant() -> Self {
        Self {
            placement: PartitionPlacement::Irrelevant,
            metadata: Rc::from("Irrelevant"),
        }
    }

    /// Creates a partition with the given identity and label.
    ///
    /// The `id` becomes the partition's identity, and `metadata` — anything convertible into a
    /// shared string, such as a `&str` or `String` — its human-readable label.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_ir::partition::PartitionId;
    /// let partition = PartitionId::new(0, "Inputs");
    /// assert_eq!(partition.id, 0);
    /// ```
    pub fn new_exclusive(id: PartitionId, metadata: impl AsRef<str>) -> Self {
        Self {
            placement: PartitionPlacement::Exclusive(id),
            metadata: Rc::from(metadata.as_ref()),
        }
    }

    /// Merges two partitions into a single coarser unit.
    ///
    /// The fused partition keeps the lower of the two identities, so merging is stable with
    /// respect to partition ordering. Its label combines the two source labels, ordered by
    /// identity and separated by `||`, so the result records everything it subsumes. When both
    /// arguments already denote the same partition the label is left untouched and a clone is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_ir::partition::PartitionId;
    /// let a = PartitionId::new(0, "Inputs");
    /// let b = PartitionId::new(1, "Stage 1");
    ///
    /// let fused = PartitionId::fuse(&a, &b);
    /// assert_eq!(fused.id, 0);
    /// assert_eq!(&*fused.metadata, "Inputs||Stage 1");
    /// ```
    pub fn merge(a: &Self, b: &Self) -> Self {
        if a == b {
            a.clone()
        } else {
            let (first, second) = if a < b { (a, b) } else { (b, a) };
            Self {
                placement: first.placement.clone(),
                metadata: [first.metadata.as_ref(), "||", second.metadata.as_ref()]
                    .concat()
                    .into(),
            }
        }
    }
}

impl Dumpable for PartitionAnnotation {
    fn dump_to_string(&self) -> String {
        format!("Partition {:?}: {}", self.placement, self.metadata)
    }
}

impl VisualAnnotation for PartitionAnnotation {
    fn style_modifier(&self) -> Option<StyleModifier> {
        self.placement.style_modifier()
    }

    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        Some(Box::new(TextBox::<NoClass>::new(
            None,
            format!("{:?}", self.metadata),
        )))
    }
}

/// A sorted, de-duplicated view of the partitions present in a graph.
///
/// A partition table gathers the distinct [`PartitionId`]s of a program into a set ordered by
/// identity, giving a concise overview of the units of computation a graph has been split into.
/// It is primarily an inspection aid: its [`Dumpable`] rendering lists one partition per line.
/// Construct one from a `BTreeSet<PartitionId>` via the [`From`] implementation.
pub struct PartitionTable(std::collections::BTreeSet<PartitionAnnotation>);

impl Dumpable for PartitionTable {
    fn dump_to_string(&self) -> String {
        self.0
            .iter()
            .map(|p| p.dump_to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<std::collections::BTreeSet<PartitionAnnotation>> for PartitionTable {
    fn from(value: std::collections::BTreeSet<PartitionAnnotation>) -> Self {
        Self(value)
    }
}
