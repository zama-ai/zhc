use super::super::{EmulatedPlaintext, EmulatedPlaintextStorage, PlaintextBlockSpec};

/// Specification for plaintext structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaintextSpec {
    int_size: u16,
    block: PlaintextBlockSpec,
}

impl PlaintextSpec {
    pub fn new(int_size: u16, block_message_size: u8) -> Self {
        assert!(
            int_size <= EmulatedPlaintextStorage::BITS as u16,
            "Tried to create malformed plaintext spec."
        );
        assert_ne!(
            block_message_size, 0,
            "Tried to create malformed plaintext spec."
        );
        assert_eq!(
            int_size.rem_euclid(block_message_size as u16),
            0,
            "Tried to create malformed plaintext spec."
        );
        Self {
            int_size,
            block: PlaintextBlockSpec(block_message_size),
        }
    }

    pub fn int_size(&self) -> u16 {
        self.int_size
    }

    pub fn int_mask(&self) -> EmulatedPlaintextStorage {
        (1 << (self.block_count() * self.block.message_size())) - 1
    }

    pub fn block_spec(&self) -> PlaintextBlockSpec {
        self.block
    }

    pub fn block_mask(&self, ith: u8) -> EmulatedPlaintextStorage {
        assert!(
            ith < self.block_count(),
            "Tried to get block mask for nonexistent block"
        );
        (self.block.message_mask() as EmulatedPlaintextStorage) << (ith * self.block.message_size())
    }

    pub fn block_count(&self) -> u8 {
        self.int_size.div_euclid(self.block.0 as u16) as u8
    }

    pub fn from_int(&self, int: EmulatedPlaintextStorage) -> EmulatedPlaintext {
        let storage = int;
        if self.overflows_int(storage) {
            panic!(
                "Input value {} exceeds maximum value for int size of {} bits",
                storage,
                self.int_size()
            );
        }
        EmulatedPlaintext {
            storage,
            spec: *self,
        }
    }

    pub fn overflows_int(&self, storage: EmulatedPlaintextStorage) -> bool {
        let shift = self.int_size();
        storage >= (1 << shift)
    }
}
