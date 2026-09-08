//! Partitioning of an IR into units of computation.
//!
//! A partitioning assigns the operations of an [`IR`](crate::IR) to partitions, each named by a
//! [`PartitionId`]. The placement of one operation is a [`PartitionPlacement`]: no partition yet,
//! exactly one partition, or several partitions when the operation is replicated in each of them.
//! Placements are typically stored per operation in an [`OpMap`](crate::OpMap).
//!
//! [`PartitionAnnotation`] attaches a human-readable label to a placement, so that partitions can
//! be merged and inspected. [`PartitionTable`] gathers the distinct partitions of a graph for
//! display. All annotation types implement [`VisualAnnotation`], giving each partition a stable
//! color in the rendered graph.

use std::rc::Rc;

use serde::Serialize;
use zhc_utils::{
    Dumpable, StoreIndex, fsm, graphics::ColorScale, iter::ReconcilerOf3, small::SmallSet,
};

use crate::visualization::{
    DynamicElement, Fill, NoClass, StyleModifier, TextBox, VisualAnnotation,
};

/// Identifier of a partition of an IR.
///
/// Ordering follows the numeric value. Two operations with the same identifier belong to the same
/// partition.
#[derive(Serialize, Debug, Clone, Copy, StoreIndex, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionId(pub u16);

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
    /// Returns an iterator over the partitions of the placement.
    ///
    /// Yields nothing for `Irrelevant`, the single partition for `Exclusive`, and every partition
    /// of the set for `Shared`, each once and in unspecified order. The iterator owns its data and
    /// does not borrow `self`.
    pub fn iter_pids(&self) -> impl Iterator<Item = PartitionId> + use<> {
        match self {
            PartitionPlacement::Irrelevant => std::iter::empty().reconcile_1_of_3(),
            PartitionPlacement::Exclusive(pid) => std::iter::once(*pid).reconcile_2_of_3(),
            PartitionPlacement::Shared(pids) => pids.clone().into_iter().reconcile_3_of_3(),
            _ => unreachable!(),
        }
    }

    /// Adds a partition to the placement.
    ///
    /// An `Irrelevant` placement becomes `Exclusive` of `pid`. An `Exclusive` placement becomes
    /// `Shared` of its partition and `pid`; this happens also when `pid` is the partition already
    /// present, which gives a `Shared` placement with one partition. A `Shared` placement gets
    /// `pid` inserted in its set, with no effect if `pid` is already in it.
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

    /// Returns `true` if the placement is not `Irrelevant`.
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
/// parallel compilation. The `placement` field carries the partition's identity — the
/// [`PartitionPlacement`] it denotes — while `metadata` holds a human-readable label describing
/// what the group represents.
///
/// Equality rests on `placement` alone: two partitions with the same `placement` compare equal
/// regardless of their `metadata`. Ordering puts [`PartitionPlacement::Irrelevant`] first, then
/// [`PartitionPlacement::Exclusive`] partitions by their [`PartitionId`]. Comparing the order of a
/// [`PartitionPlacement::Shared`] partition panics.
#[derive(Debug, Clone, Hash)]
pub struct PartitionAnnotation {
    /// The partition's identity, the placement it denotes.
    ///
    /// Equality and ordering of a [`PartitionAnnotation`] derive from this field alone.
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
    /// Creates a partition with the [`PartitionPlacement::Irrelevant`] placement.
    ///
    /// The label is `"Irrelevant"`.
    pub fn new_irrelevant() -> Self {
        Self {
            placement: PartitionPlacement::Irrelevant,
            metadata: Rc::from("Irrelevant"),
        }
    }

    /// Creates an exclusive partition with the given identity and label.
    ///
    /// The placement becomes [`PartitionPlacement::Exclusive`] of `id`, and `metadata` — anything
    /// that gives a `&str`, such as a `&str` or `String` — its human-readable label.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zhc_ir::partitioning::{PartitionAnnotation, PartitionId, PartitionPlacement};
    /// let partition = PartitionAnnotation::new_exclusive(PartitionId(0), "Inputs");
    /// assert_eq!(partition.placement, PartitionPlacement::Exclusive(PartitionId(0)));
    /// ```
    pub fn new_exclusive(id: PartitionId, metadata: impl AsRef<str>) -> Self {
        Self {
            placement: PartitionPlacement::Exclusive(id),
            metadata: Rc::from(metadata.as_ref()),
        }
    }

    /// Merges two partitions into a single coarser unit.
    ///
    /// The fused partition keeps the placement of the lower of the two partitions, so merging is
    /// stable with respect to partition ordering. Its label combines the two source labels,
    /// ordered by placement and separated by `||`, so the result records everything it subsumes.
    /// When both arguments already denote the same placement the label is left untouched and a
    /// clone of `a` is returned.
    ///
    /// # Panics
    ///
    /// Panics if the placements differ and one of them is [`PartitionPlacement::Shared`], because
    /// such partitions cannot be ordered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zhc_ir::partitioning::{PartitionAnnotation, PartitionId, PartitionPlacement};
    /// let a = PartitionAnnotation::new_exclusive(PartitionId(0), "Inputs");
    /// let b = PartitionAnnotation::new_exclusive(PartitionId(1), "Stage 1");
    ///
    /// let fused = PartitionAnnotation::merge(&a, &b);
    /// assert_eq!(fused.placement, PartitionPlacement::Exclusive(PartitionId(0)));
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
/// A partition table gathers the distinct [`PartitionAnnotation`]s of a program into a set ordered
/// by placement, giving a concise overview of the units of computation a graph has been split
/// into. It is primarily an inspection aid: its [`Dumpable`] rendering lists one partition per
/// line. Construct one from a `BTreeSet<PartitionAnnotation>` via the [`From`] implementation.
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
