use std::fmt::Display;

use serde::Serialize;
use zhc_ir::visualization::VisualAnnotation;
use zhc_utils::{Dumpable, StoreIndex};

use crate::doplang::UserFlag;

/// Total number of [`UserFlag`] values.
pub const N_FLAGS: u8 = 64;
/// Number of [`UserFlag`] values not available for transfer slots.
///
/// These are the lowest flag values. [`Slot::to_user_flag`] maps slot ids after them.
pub const N_RESERVED_FLAGS: u8 = 1;
/// Number of [`UserFlag`] values available for transfer slots.
///
/// Valid [`SlotId`]s are the values below this number.
pub const N_TRANSFER_FLAGS: u8 = N_FLAGS - N_RESERVED_FLAGS;

/// Notification that a transfer sends to an HPU on the flag of a [`Slot`].
///
/// Used with [`SyncWait`] to order the operations of two HPUs.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct SyncNotify {
    /// The HPU that receives the notification.
    pub dst: HpuId,
    /// The slot whose flag is notified.
    pub slot: Slot,
}

/// Wait of a transfer for a notification on the flag of a [`Slot`].
///
/// Matches a [`SyncNotify`] on the same slot.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct SyncWait {
    /// The HPU that sends the notification.
    pub src: HpuId,
    /// The slot whose flag is awaited.
    pub slot: Slot,
}

/// Index of a transfer slot of an HPU.
///
/// Each slot id maps to one [`UserFlag`] through [`Slot::to_user_flag`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, StoreIndex, Hash)]
pub struct SlotId(pub u8);

/// Generation of a transfer slot, which counts the uses of the slot.
///
/// The first use of a slot has generation 0. Ordering follows the numeric value.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SlotGen(pub u16);

impl SlotGen {
    /// Returns the generation that follows this one.
    ///
    /// # Panics
    ///
    /// Panics if the generation is `u16::MAX`.
    pub fn next(&self) -> Self {
        SlotGen(self.0.strict_add(1))
    }

    /// Returns `true` if this is generation 0.
    pub fn is_first_gen(&self) -> bool {
        self.0 == 0
    }

    /// Returns the generation that comes before this one.
    ///
    /// # Panics
    ///
    /// Panics if this is generation 0.
    pub fn previous(&self) -> Self {
        SlotGen(self.0.strict_sub(1))
    }
}

/// Identifies an inter-HPU transfer by its slot id and the generation of that slot.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct Slot(pub SlotGen, pub SlotId);

impl Slot {
    /// Returns the [`UserFlag`] used to synchronize on this slot.
    ///
    /// The flag value is the slot id plus [`N_RESERVED_FLAGS`]. The generation has no effect, so
    /// all generations of a slot share one flag.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the slot id plus [`N_RESERVED_FLAGS`] overflows `u8`.
    pub fn to_user_flag(&self) -> UserFlag {
        UserFlag {
            flag: self.1.0 + N_RESERVED_FLAGS,
        }
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#!{}:{}", self.0.0, self.1.0)
    }
}

/// Identifies a single HPU board within a partitioned multi-HPU program.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Copy, Hash, StoreIndex)]
pub struct HpuId(pub u8);

impl Display for HpuId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HPU_{}", self.0)
    }
}

/// Placement of a value across the HPUs of a partitioned program.
///
/// An operation either lives on a single board (`OnHpu`), or on two boards when it
/// is a transfer (`Transfer`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HpuLocality {
    OnHpu(HpuId),
    Transfer { src: HpuId, dst: HpuId },
}

impl HpuLocality {
    /// Returns whether this locality places the value on the HPU `hid`.
    ///
    /// True when the value lives on, or is being transferred to or from, `hid`.
    pub fn is_on(&self, hid: &HpuId) -> bool {
        match self {
            HpuLocality::OnHpu(hpu_id) => hpu_id == hid,
            HpuLocality::Transfer { src, dst } => src == hid || dst == hid,
        }
    }
}

impl VisualAnnotation for HpuLocality {}

impl Dumpable for HpuLocality {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}
