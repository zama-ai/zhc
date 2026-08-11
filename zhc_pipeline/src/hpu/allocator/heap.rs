use std::collections::VecDeque;
use std::fmt::Display;
use zhc_ir::{AsValId, ValId};
use zhc_utils::{SafeAs, StoreIndex, small::SmallMap};

/// A unique identifier to a slot on the heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeapSlot(pub u16);

/// Slots wait this many releases before being handed out again.
/// This numbers match ISC pool depth.
const REUSE_QUEUE: usize = 64;

/// A heap to spill ciphertexts on.
///
/// Dead values' slots are recycled, so the heap only has to hold the spills
/// that are live at the same time.
#[derive(Clone, Debug)]
pub struct Heap {
    slots: SmallMap<ValId, HeapSlot>,
    last: HeapSlot,
    free: VecDeque<HeapSlot>,
}

impl Heap {
    /// Creates a new empty heap.
    pub fn empty() -> Self {
        Heap {
            slots: SmallMap::new(),
            last: HeapSlot(0),
            free: VecDeque::new(),
        }
    }

    fn alloc(&mut self) -> HeapSlot {
        if self.free.len() > REUSE_QUEUE {
            return self.free.pop_front().unwrap();
        }
        let next = HeapSlot(self.last.0.strict_add(1));
        std::mem::replace(&mut self.last, next)
    }

    /// Check whether a value is on the heap.
    pub fn contains(&self, valid: impl AsValId) -> bool {
        self.slots.get(&valid.val_id()).is_some()
    }

    /// Get a heap slot for a value.
    ///
    /// Creates a slot if the value is not already stored, and return the slot otherwise.
    pub fn get(&mut self, valid: impl AsValId) -> HeapSlot {
        let valid = valid.val_id();
        if !self.contains(valid) {
            let slot = self.alloc();
            self.slots.insert(valid, slot);
            slot
        } else {
            *self.slots.get(&valid).unwrap()
        }
    }

    /// Returns a fresh heap slot not associated with any value.
    pub fn get_unmapped(&mut self) -> HeapSlot {
        self.alloc()
    }

    /// Releases a dead value's slot for reuse. No-op if it was never spilled.
    pub fn release(&mut self, valid: impl AsValId) {
        if let Some(slot) = self.slots.remove(&valid.val_id()) {
            self.free.push_back(slot);
        }
    }

    #[allow(unused)]
    pub fn size(&self) -> usize {
        self.last.0.sas()
    }
}

impl Display for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "((|")?;
        for (k, _) in self.slots.iter() {
            write!(f, " {}|", k.as_usize())?;
        }
        write!(f, "))")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn vid(i: u32) -> ValId {
        ValId(i)
    }

    #[test]
    fn released_slots_are_reused() {
        let mut heap = Heap::empty();
        // Warming the queue costs a fresh slot per release.
        for i in 0..=REUSE_QUEUE as u32 {
            heap.get(vid(i));
            heap.release(vid(i));
        }
        let warm = heap.size();
        assert_eq!(warm, REUSE_QUEUE + 1);

        // Then spills recycle, and the heap stops growing.
        for i in 0..1000u32 {
            let v = vid(1000 + i);
            heap.get(v);
            heap.release(v);
        }
        assert_eq!(heap.size(), warm);
    }

    #[test]
    fn same_value_keeps_its_slot() {
        let mut heap = Heap::empty();
        let s0 = heap.get(vid(0));
        assert_eq!(heap.get(vid(0)), s0);
        let s1 = heap.get(vid(1));
        assert_ne!(s0, s1);
    }

    #[test]
    fn release_is_noop_for_unspilled_values() {
        let mut heap = Heap::empty();
        heap.release(vid(7));
        assert_eq!(heap.size(), 0);
    }
}
