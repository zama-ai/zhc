use std::fmt::Display;

use serde::Serialize;
use zhc_ir::visualization::VisualAnnotation;
use zhc_utils::{Dumpable, StoreIndex, existential_enum};

pub const N_FLAGS: u8 = 64;
pub const N_RESERVED_FLAGS: u8 = 9;
pub const N_TRANSFER_FLAGS: u8 = N_FLAGS - N_RESERVED_FLAGS;
pub const FIRST_FLAG: u8 = N_RESERVED_FLAGS;
pub const LAST_FLAG: u8 = N_FLAGS - 1;

#[existential_enum]
pub enum NextTransferId {
    NewGen(TransferId),
    SameGen(TransferId),
}

impl NextTransferId {
    pub fn unwrap(self) -> TransferId {
        match self {
            NextTransferId::NewGen(transfer_id) => transfer_id,
            NextTransferId::SameGen(transfer_id) => transfer_id,
        }
    }
}

/// Identifies a inter-HPU transfer.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct TransferId(pub u16, pub u8);

impl TransferId {
    pub const FIRST: Self = Self::first_of_gen(0);

    pub const fn first_of_gen(g: u16) -> Self {
        TransferId(g, FIRST_FLAG)
    }

    pub fn inc(&self) -> NextTransferId {
        match self {
            TransferId(g, LAST_FLAG) => NextTransferId::NewGen(Self::first_of_gen(g + 1)),
            TransferId(g, f) => NextTransferId::SameGen(TransferId(*g, f + 1)),
        }
    }

    pub fn tid_sequence() -> impl Iterator<Item = TransferId> {
        std::iter::successors(Some(TransferId::FIRST), |tid| match tid.inc() {
            NextTransferId::NewGen(next) | NextTransferId::SameGen(next) => Some(next),
        })
    }

    pub fn is_first_gen(&self) -> bool {
        self.0 == 0
    }

    pub fn of_previous_gen(&self) -> Self {
        TransferId(self.0.checked_sub(1).unwrap(), self.1)
    }

    pub fn flag(&self) -> u8 {
        self.1
    }

    pub fn is_first_of_its_gen(&self) -> bool {
        self.1 == FIRST_FLAG
    }

    pub fn is_last_of_its_gen(&self) -> bool {
        self.1 == LAST_FLAG
    }

}

impl Display for TransferId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#!{}:{}", self.0, self.1)
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
/// An operation either lives on a single board (`OnHpu`), on two boards when it
/// is a transfer (`Transfer`), is replicated on a set of boards (`Shared`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HpuLocality {
    OnHpu(HpuId),
    Transfer { src: HpuId, dst: HpuId },
}

impl HpuLocality {
    /// Returns whether this locality places the value on the HPU `hid`.
    ///
    /// True when the value lives on, is being transferred to or from, or is
    /// shared with `hid`.
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

#[cfg(test)]
mod test {
    use super::{N_FLAGS, TransferId};
    use zhc_utils::iter::CollectInVec;

    #[test]
    fn tid_seq() {
        let seq = TransferId::tid_sequence()
            .take(N_FLAGS as usize * 3)
            .covec();
        assert_eq!(
            seq,
            vec![
                TransferId(0, 9),
                TransferId(0, 10),
                TransferId(0, 11),
                TransferId(0, 12),
                TransferId(0, 13),
                TransferId(0, 14),
                TransferId(0, 15),
                TransferId(0, 16),
                TransferId(0, 17),
                TransferId(0, 18),
                TransferId(0, 19),
                TransferId(0, 20),
                TransferId(0, 21),
                TransferId(0, 22),
                TransferId(0, 23),
                TransferId(0, 24),
                TransferId(0, 25),
                TransferId(0, 26),
                TransferId(0, 27),
                TransferId(0, 28),
                TransferId(0, 29),
                TransferId(0, 30),
                TransferId(0, 31),
                TransferId(0, 32),
                TransferId(0, 33),
                TransferId(0, 34),
                TransferId(0, 35),
                TransferId(0, 36),
                TransferId(0, 37),
                TransferId(0, 38),
                TransferId(0, 39),
                TransferId(0, 40),
                TransferId(0, 41),
                TransferId(0, 42),
                TransferId(0, 43),
                TransferId(0, 44),
                TransferId(0, 45),
                TransferId(0, 46),
                TransferId(0, 47),
                TransferId(0, 48),
                TransferId(0, 49),
                TransferId(0, 50),
                TransferId(0, 51),
                TransferId(0, 52),
                TransferId(0, 53),
                TransferId(0, 54),
                TransferId(0, 55),
                TransferId(0, 56),
                TransferId(0, 57),
                TransferId(0, 58),
                TransferId(0, 59),
                TransferId(0, 60),
                TransferId(0, 61),
                TransferId(0, 62),
                TransferId(0, 63),
                TransferId(1, 9),
                TransferId(1, 10),
                TransferId(1, 11),
                TransferId(1, 12),
                TransferId(1, 13),
                TransferId(1, 14),
                TransferId(1, 15),
                TransferId(1, 16),
                TransferId(1, 17),
                TransferId(1, 18),
                TransferId(1, 19),
                TransferId(1, 20),
                TransferId(1, 21),
                TransferId(1, 22),
                TransferId(1, 23),
                TransferId(1, 24),
                TransferId(1, 25),
                TransferId(1, 26),
                TransferId(1, 27),
                TransferId(1, 28),
                TransferId(1, 29),
                TransferId(1, 30),
                TransferId(1, 31),
                TransferId(1, 32),
                TransferId(1, 33),
                TransferId(1, 34),
                TransferId(1, 35),
                TransferId(1, 36),
                TransferId(1, 37),
                TransferId(1, 38),
                TransferId(1, 39),
                TransferId(1, 40),
                TransferId(1, 41),
                TransferId(1, 42),
                TransferId(1, 43),
                TransferId(1, 44),
                TransferId(1, 45),
                TransferId(1, 46),
                TransferId(1, 47),
                TransferId(1, 48),
                TransferId(1, 49),
                TransferId(1, 50),
                TransferId(1, 51),
                TransferId(1, 52),
                TransferId(1, 53),
                TransferId(1, 54),
                TransferId(1, 55),
                TransferId(1, 56),
                TransferId(1, 57),
                TransferId(1, 58),
                TransferId(1, 59),
                TransferId(1, 60),
                TransferId(1, 61),
                TransferId(1, 62),
                TransferId(1, 63),
                TransferId(2, 9),
                TransferId(2, 10),
                TransferId(2, 11),
                TransferId(2, 12),
                TransferId(2, 13),
                TransferId(2, 14),
                TransferId(2, 15),
                TransferId(2, 16),
                TransferId(2, 17),
                TransferId(2, 18),
                TransferId(2, 19),
                TransferId(2, 20),
                TransferId(2, 21),
                TransferId(2, 22),
                TransferId(2, 23),
                TransferId(2, 24),
                TransferId(2, 25),
                TransferId(2, 26),
                TransferId(2, 27),
                TransferId(2, 28),
                TransferId(2, 29),
                TransferId(2, 30),
                TransferId(2, 31),
                TransferId(2, 32),
                TransferId(2, 33),
                TransferId(2, 34),
                TransferId(2, 35),
                TransferId(2, 36),
                TransferId(2, 37),
                TransferId(2, 38),
                TransferId(2, 39),
                TransferId(2, 40),
                TransferId(2, 41),
                TransferId(2, 42),
                TransferId(2, 43),
                TransferId(2, 44),
                TransferId(2, 45),
                TransferId(2, 46),
                TransferId(2, 47),
                TransferId(2, 48),
                TransferId(2, 49),
                TransferId(2, 50),
                TransferId(2, 51),
                TransferId(2, 52),
                TransferId(2, 53),
                TransferId(2, 54),
                TransferId(2, 55),
                TransferId(2, 56),
                TransferId(2, 57),
                TransferId(2, 58),
                TransferId(2, 59),
                TransferId(2, 60),
                TransferId(2, 61),
                TransferId(2, 62),
                TransferId(2, 63),
                TransferId(3, 9),
                TransferId(3, 10),
                TransferId(3, 11),
                TransferId(3, 12),
                TransferId(3, 13),
                TransferId(3, 14),
                TransferId(3, 15),
                TransferId(3, 16),
                TransferId(3, 17),
                TransferId(3, 18),
                TransferId(3, 19),
                TransferId(3, 20),
                TransferId(3, 21),
                TransferId(3, 22),
                TransferId(3, 23),
                TransferId(3, 24),
                TransferId(3, 25),
                TransferId(3, 26),
                TransferId(3, 27),
                TransferId(3, 28),
                TransferId(3, 29),
                TransferId(3, 30),
                TransferId(3, 31),
                TransferId(3, 32),
                TransferId(3, 33),
                TransferId(3, 34),
                TransferId(3, 35)
            ]
        );
    }
}
