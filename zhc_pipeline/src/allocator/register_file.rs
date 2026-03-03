use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};
use zhc_utils::{iter::ChunkIt, small::SmallVec};

use crate::allocator::register_state::RegState;

/// A unique identifier of a register in a register file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegId(pub u8);

/// A unique identifier of a range of register of a given size, in a register file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegRangeId(pub RegId, pub u8);

impl RegRangeId {
    /// Returns an iterator to the reg identifiers within this range.
    pub fn rids_iter(&self) -> impl Iterator<Item = RegId> {
        (0..self.1).map(|a| RegId(self.0.0 + a))
    }
}

/// A register file.
///
/// In our case, a regfile is a map from registers identifiers to register states stored at a
/// certain point in time. See [1] for more informations.
#[derive(Debug)]
pub struct RegFile(Vec<RegState>);

impl RegFile {
    /// Creates a new register file of a given size.
    pub fn empty(size: usize) -> Self {
        RegFile(vec![RegState::Empty; size])
    }

    /// Returns an iterator over the registers and register states.
    pub fn iter_registers(&self) -> impl Iterator<Item = (RegId, &RegState)> {
        self.0.iter().enumerate().map(|(i, r)| (RegId(i as u8), r))
    }

    /// Returns a mutable iterator over the registers and register states.
    pub fn iter_registers_mut(&mut self) -> impl Iterator<Item = (RegId, &mut RegState)> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(i, r)| (RegId(i as u8), r))
    }

    /// Returns an iterator over the register ranges and the registers states.
    pub fn iter_register_ranges(
        &self,
        range_size: u8,
    ) -> impl Iterator<Item = (RegRangeId, SmallVec<&RegState>)> {
        self.0
            .iter()
            .chunk(range_size as usize)
            .enumerate()
            .map(move |(i, a)| {
                let a = a.unwrap_complete();
                (RegRangeId(RegId(i as u8 * range_size), range_size), a)
            })
    }

    /// Returns a mutable iterator over the register ranges and the registers states.
    #[allow(unused)]
    pub fn iter_register_ranges_mut(
        &mut self,
        range_size: u8,
    ) -> impl Iterator<Item = (RegRangeId, SmallVec<&mut RegState>)> {
        self.0
            .iter_mut()
            .chunk(range_size as usize)
            .enumerate()
            .map(move |(i, a)| {
                let a = a.unwrap_complete();
                (RegRangeId(RegId(i as u8 * range_size), range_size), a)
            })
    }
}

impl Index<RegId> for RegFile {
    type Output = RegState;

    fn index(&self, index: RegId) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<RegId> for RegFile {
    fn index_mut(&mut self, index: RegId) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl Display for RegFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<[|")?;
        for i in self.0.iter() {
            write!(f, " {}|", i)?;
        }
        write!(f, "]>")
    }
}

// Notes
// =====
//
// [1]: The register file structure encodes different types of information depending on the stage of the allocation
// process to assist the allocator:
// - Stable states: The Empty/Storing states are the only possible states at the start of an
//   allocation iteration. They
// represent the physical register file's state during execution.
// - Transitive states: The other states only occur within an allocation iteration. They provide
//   additional information
// to help the allocator make optimal decisions.
// In this sense, the transitive states do not represent any meaningful informatino about a physical
// regfile, but rather support the allocator in its work.
