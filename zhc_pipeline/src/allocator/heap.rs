use std::fmt::Display;
use zhc_ir::ValId;
use zhc_utils::{SafeAs, StoreIndex, small::SmallMap};

/// A unique identifier to a slot on the heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeapSlot(pub u16);

/// A heap to spill ciphertexts on.
#[derive(Clone, Debug)]
pub struct Heap {
    slots: SmallMap<ValId, HeapSlot>,
    last: HeapSlot,
}

impl Heap {
    /// Creates a new empty heap.
    pub fn empty() -> Self {
        Heap {
            slots: SmallMap::new(),
            last: HeapSlot(0),
        }
    }

    fn push(&mut self, valid: ValId) -> HeapSlot {
        self.slots.insert(valid, self.last);
        let next = HeapSlot(self.last.0.strict_add(1));
        std::mem::replace(&mut self.last, next)
    }

    /// Check whether a value is on the heap.
    pub fn contains(&self, valid: &ValId) -> bool {
        self.slots.get(valid).is_some()
    }

    /// Get a heap slot for a value.
    ///
    /// Creates a slot if the value is not already stored, and return the slot otherwise.
    pub fn get(&mut self, valid: &ValId) -> HeapSlot {
        if !self.contains(valid) {
            self.push(*valid)
        } else {
            *self.slots.get(valid).unwrap()
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
