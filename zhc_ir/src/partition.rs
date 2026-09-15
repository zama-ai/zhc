//! Grouping of IR operations into labelled units of computation.
//!
//! A *partition* clusters operations that sit close together in the graph into a single unit
//! of work — a task in the parallel-compilation sense. At this stage the notion stays abstract:
//! a partition is simply a named group of related operations that forms a natural unit of
//! computation. Later stages may use these units to split a program across several parallel
//! devices, but partitioning imposes no placement or scheduling decision on its own.
//!
//! Each partition is identified by a [`PartitionId`], which pairs a numeric identity with a
//! human-readable label. Partitions can be [fused](PartitionId::fuse) into coarser units, and
//! the set of partitions present in a graph can be summarised as a [`PartitionTable`] for
//! inspection.
//!
//! # Examples
//!
//! ```rust,no_run
//! # use zhc_ir::partition::{PartitionId, PartitionTable};
//! # use std::collections::BTreeSet;
//! let inputs = PartitionId::new(0, "Inputs");
//! let stage_1 = PartitionId::new(1, "Stage 1");
//!
//! // Fusing two partitions yields a single unit that keeps the lower identity.
//! let merged = PartitionId::fuse(&inputs, &stage_1);
//! assert_eq!(merged.id, 0);
//!
//! // Collect the distinct partitions of a graph into an inspectable table.
//! let table = PartitionTable::from(BTreeSet::from([inputs, merged]));
//! ```

use std::rc::Rc;

use crate::{
    OpIdRaw,
    visualization::{StyleModifier, VisualAnnotation},
};
use zhc_utils::{Dumpable, graphics::ColorScale};

pub type PartitionIdRaw = OpIdRaw;

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
pub struct PartitionId {
    /// The partition's identity, a sequence number assigned at creation.
    ///
    /// Equality, ordering, and hashing of a [`PartitionId`] derive from this field alone.
    pub id: PartitionIdRaw,

    /// A human-readable label describing what the partition represents.
    pub metadata: Rc<str>,
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
    pub fn new(id: PartitionIdRaw, metadata: impl AsRef<str>) -> Self {
        Self {
            id,
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
            fill: Some(
                ColorScale::RAINBOW
                    .interpolate((self.id as f64 * 0.6180339887498949) % 1.0)
                    .into(),
            ),
            ..Default::default()
        })
    }
}

/// A sorted, de-duplicated view of the partitions present in a graph.
///
/// A partition table gathers the distinct [`PartitionId`]s of a program into a set ordered by
/// identity, giving a concise overview of the units of computation a graph has been split into.
/// It is primarily an inspection aid: its [`Dumpable`] rendering lists one partition per line.
/// Construct one from a `BTreeSet<PartitionId>` via the [`From`] implementation.
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
