use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};
use zhc_utils::{Store, StoreIndex, fsm};

/// A unique identifier to a slot on the heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, StoreIndex)]
pub struct HeapSlot(pub u16);

#[fsm]
#[derive(Debug, Clone, PartialEq, Eq)]
enum SlotState {
    /// The slot holds a spilled value.
    Busy,
    /// The slot is reserved for the whole program and is never recycled.
    Exclusive,
    /// The slot is free, but may still be referenced by in-flight DOps.
    Quarantined,
}

/// A heap to spill ciphertexts on.
///
/// Released slots are quarantined for `quarantine` ops before they are handed
/// out again, so no DOp still in the ISC window can reference a reused slot.
#[derive(Clone, Debug)]
pub struct Heap {
    slots: Store<HeapSlot, SlotState>,
    quarantined: BinaryHeap<Reverse<(usize, HeapSlot)>>,
    quarantine: u16,
    heap_size: usize,
}

impl Heap {
    /// Creates a new empty heap.
    pub fn new(quarantine: u16, heap_size: usize) -> Self {
        Heap {
            slots: Store::empty(),
            quarantined: BinaryHeap::new(),
            quarantine,
            heap_size,
        }
    }

    /// Returns a heap slot to spill a value on.
    pub fn alloc(&mut self, now: usize) -> HeapSlot {
        let maybe_slot = {
            if let Some(Reverse((t, _))) = self.quarantined.peek()
                && *t < now
            {
                self.quarantined.pop().map(|Reverse((_, hs))| hs)
            } else {
                None
            }
        };

        match maybe_slot {
            Some(hs) => {
                self.slots[hs].transition(|ss| match ss {
                    SlotState::Quarantined => SlotState::Busy,
                    _ => unreachable!(),
                });
                hs
            }
            None => self.grow(SlotState::Busy),
        }
    }

    /// Returns a fresh heap slot reserved for the whole program.
    pub fn alloc_exclusive(&mut self) -> HeapSlot {
        self.grow(SlotState::Exclusive)
    }

    fn grow(&mut self, state: SlotState) -> HeapSlot {
        let used = self.slots.len() as usize;
        assert!(
            used < self.heap_size,
            "Heap overflow: the program needs more than {used} slots, the device reserves {}.",
            self.heap_size
        );
        let oup = HeapSlot(self.slots.len());
        self.slots.push(state);
        oup
    }

    /// Releases a busy slot after its last access at `now`.
    pub fn release(&mut self, slot: HeapSlot, now: usize) {
        self.slots[slot].transition(|s| match s {
            SlotState::Busy => {
                self.quarantined
                    .push(Reverse((now + self.quarantine as usize, slot)));
                SlotState::Quarantined
            }
            SlotState::Exclusive => panic!("Exclusive slot {slot:?} can not be released."),
            _ => unreachable!("Slot {slot:?} released twice."),
        });
    }

    /// Number of distinct slots handed out so far.
    pub fn size(&self) -> u16 {
        self.slots.len()
    }
}

impl Display for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "((|")?;
        for k in self.slots.iter() {
            match k {
                SlotState::Busy => write!(f, " B|")?,
                SlotState::Exclusive => write!(f, " X|")?,
                SlotState::Quarantined => write!(f, " Q|")?,
                _ => unreachable!(),
            }
        }
        write!(f, "))")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slot_is_reused_after_quarantine() {
        let mut heap = Heap::new(64, 1024);
        let s0 = heap.alloc(0);
        heap.release(s0, 10);
        assert_eq!(heap.alloc(75), s0);
        assert_eq!(heap.size(), 1);
    }

    #[test]
    fn slot_is_not_reused_during_quarantine() {
        let mut heap = Heap::new(64, 1024);
        let s0 = heap.alloc(0);
        heap.release(s0, 10);
        assert_ne!(heap.alloc(74), s0);
        assert_eq!(heap.size(), 2);
    }

    #[test]
    fn earliest_released_slot_is_reused_first() {
        let mut heap = Heap::new(64, 1024);
        let s0 = heap.alloc(0);
        let s1 = heap.alloc(0);
        heap.release(s1, 5);
        heap.release(s0, 20);
        assert_eq!(heap.alloc(70), s1);
        assert_ne!(heap.alloc(70), s0);
    }

    #[test]
    fn heap_stays_bounded_under_steady_churn() {
        let mut heap = Heap::new(64, 1024);
        for now in 0..10_000usize {
            let s = heap.alloc(now);
            heap.release(s, now);
        }
        assert_eq!(heap.size(), 65);
    }

    #[test]
    fn exclusive_slot_is_fresh_and_never_reused() {
        let mut heap = Heap::new(64, 1024);
        let s0 = heap.alloc(0);
        heap.release(s0, 0);
        let x = heap.alloc_exclusive();
        assert_ne!(x, s0);
        for now in 65..200usize {
            assert_ne!(heap.alloc(now), x);
        }
    }

    #[test]
    #[should_panic(expected = "can not be released")]
    fn releasing_exclusive_slot_panics() {
        let mut heap = Heap::new(64, 1024);
        let x = heap.alloc_exclusive();
        heap.release(x, 0);
    }

    #[test]
    #[should_panic(expected = "released twice")]
    fn releasing_slot_twice_panics() {
        let mut heap = Heap::new(64, 1024);
        let s = heap.alloc(0);
        heap.release(s, 0);
        heap.release(s, 1);
    }

    #[test]
    #[should_panic(expected = "Heap overflow")]
    fn growing_past_heap_size_panics() {
        let mut heap = Heap::new(64, 2);
        heap.alloc(0);
        heap.alloc_exclusive();
        heap.alloc(0);
    }
}
