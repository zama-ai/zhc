use std::collections::VecDeque;
use std::fmt::Display;
use zhc_ir::{AsValId, ValId};
use zhc_utils::{SafeAs, StoreIndex, small::SmallMap};

/// A unique identifier to a slot on the heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeapSlot(pub u16);

/// A heap to spill ciphertexts on.
///
/// Dead values' slots are recycled once their release is more than
/// `isc_depth` ops old, so no DOp still in the ISC pool can reference a
/// reused slot.
#[derive(Clone, Debug)]
pub struct Heap {
    slots: SmallMap<ValId, HeapSlot>,
    last: HeapSlot,
    isc_depth: usize,
    /// Released slots, stamped with the op at which they were released.
    free: VecDeque<(HeapSlot, usize)>,
}

impl Heap {
    /// Creates a new empty heap.
    pub fn new(isc_depth: usize) -> Self {
        Heap {
            slots: SmallMap::new(),
            last: HeapSlot(0),
            isc_depth,
            free: VecDeque::new(),
        }
    }

    /// Returns a heap slot not associated with any value.
    pub fn alloc(&mut self, now: usize) -> HeapSlot {
        if let Some(&(slot, released)) = self.free.front() {
            if now - released > self.isc_depth {
                self.free.pop_front();
                return slot;
            }
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
    pub fn get(&mut self, valid: impl AsValId, now: usize) -> HeapSlot {
        let valid = valid.val_id();
        if !self.contains(valid) {
            let slot = self.alloc(now);
            self.slots.insert(valid, slot);
            slot
        } else {
            *self.slots.get(&valid).unwrap()
        }
    }

    /// Releases a dead value's slot for reuse. No-op if it was never spilled.
    pub fn release(&mut self, valid: impl AsValId, now: usize) {
        if let Some(slot) = self.slots.remove(&valid.val_id()) {
            self.free.push_back((slot, now));
        }
    }

    /// Number of distinct slots ever handed out.
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

    const DEPTH: usize = 64;

    #[test]
    fn released_slots_are_reused() {
        let mut heap = Heap::new(DEPTH);
        for now in 0..=DEPTH {
            heap.get(vid(now as u32), now);
            heap.release(vid(now as u32), now);
        }
        let warm = heap.size();
        assert_eq!(warm, DEPTH + 1);

        for i in 0..1000usize {
            let (v, now) = (vid(1000 + i as u32), DEPTH + 1 + i);
            heap.get(v, now);
            heap.release(v, now);
        }
        assert_eq!(heap.size(), warm);
    }

    #[test]
    fn slots_are_not_reused_before_aging_past_the_isc_depth() {
        let mut heap = Heap::new(DEPTH);
        let s0 = heap.get(vid(0), 0);
        heap.release(vid(0), 0);
        assert_ne!(heap.alloc(DEPTH), s0);
        assert_eq!(heap.alloc(DEPTH + 1), s0);
    }

    #[test]
    fn same_value_keeps_its_slot() {
        let mut heap = Heap::new(DEPTH);
        let s0 = heap.get(vid(0), 0);
        assert_eq!(heap.get(vid(0), 0), s0);
        let s1 = heap.get(vid(1), 0);
        assert_ne!(s0, s1);
    }

    #[test]
    fn release_is_noop_for_unspilled_values() {
        let mut heap = Heap::new(DEPTH);
        heap.release(vid(7), 0);
        assert_eq!(heap.size(), 0);
    }
}
