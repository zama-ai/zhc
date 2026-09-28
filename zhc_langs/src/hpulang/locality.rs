use std::fmt::Display;

use serde::Serialize;
use zhc_ir::visualization::VisualAnnotation;
use zhc_utils::{Dumpable, StoreIndex};

/// Identifies a single HPU board within a partitioned multi-HPU program.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Copy, Hash, StoreIndex)]
pub struct HpuId(pub u8);

impl Display for HpuId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HPU_{}", self.0)
    }
}

/// Maximal number of transfer ids currently supported at runtime.
pub const MAX_TRANSFER_ID: u16 = 64;

/// Identifies a single inter-HPU transfer, pairing its two split halves.
///
/// The `TransferOut` on the source board and the `TransferIn` on the
/// destination board that implement one transfer carry the same
/// `TransferId`, and it doubles as the handshake flag exchanged between
/// the boards at runtime.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct TransferId(pub u16);

impl Display for TransferId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#!{}", self.0)
    }
}

/// Placement of a value across the HPUs of a partitioned program.
///
/// An operation either lives on a single board (`OnHpu`), on two boards when it
/// is a transfer (`Transfer`), is replicated on a set of boards (`Shared`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HpuLocality {
    OnHpu(HpuId),
    Transfer { from: HpuId, to: HpuId },
}

impl HpuLocality {
    /// Returns whether this locality places the value on the HPU `hid`.
    ///
    /// True when the value lives on, is being transferred to or from, or is
    /// shared with `hid`.
    pub fn is_on(&self, hid: &HpuId) -> bool {
        match self {
            HpuLocality::OnHpu(hpu_id) => hpu_id == hid,
            HpuLocality::Transfer { from, to } => from == hid || to == hid,
        }
    }
}

impl VisualAnnotation for HpuLocality {}

impl Dumpable for HpuLocality {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}
