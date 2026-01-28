use crate::integer_semantics::PlaintextSpec;

use super::super::{Ciphertext, CiphertextBlockSpec, CiphertextStorage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CiphertextSpec {
    int_size: u16,
    block: CiphertextBlockSpec,
}

impl CiphertextSpec {
    pub fn new(int_size: u16, block_carry_size: u8, block_message_size: u8) -> Self {
        assert!(
            int_size <= CiphertextStorage::BITS as u16,
            "Tried to create malformed ciphertext spec."
        );
        assert_ne!(
            block_carry_size, 0,
            "Tried to create malformed ciphertext spec."
        );
        assert_ne!(
            block_message_size, 0,
            "Tried to create malformed ciphertext spec."
        );
        assert_eq!(
            int_size.rem_euclid(block_message_size as u16),
            0,
            "Tried to create malformed ciphertext spec."
        );
        Self {
            int_size,
            block: CiphertextBlockSpec(block_carry_size, block_message_size),
        }
    }

    pub fn int_size(&self) -> u16 {
        self.int_size
    }

    pub fn int_mask(&self) -> CiphertextStorage {
        (1 << (self.block_count() * self.block.message_size())) - 1
    }

    pub fn block_spec(&self) -> CiphertextBlockSpec {
        self.block
    }

    pub fn block_count(&self) -> u8 {
        self.int_size.div_euclid(self.block.1 as u16) as u8
    }

    pub fn block_mask(&self, ith: u8) -> CiphertextStorage {
        assert!(
            ith < self.block_count(),
            "Tried to get block mask for nonexistent block"
        );
        (self.block.message_mask() as CiphertextStorage) << (ith * self.block.message_size())
    }

    pub fn from_int(&self, int: CiphertextStorage) -> Ciphertext {
        let storage = int;
        if self.overflows_int(storage) {
            panic!(
                "Input value {} exceeds maximum value for int size of {} bits",
                storage,
                self.int_size()
            );
        }
        Ciphertext {
            storage,
            spec: *self,
        }
    }

    pub fn overflows_int(&self, storage: CiphertextStorage) -> bool {
        let shift = self.int_size();
        storage >= (1 << shift)
    }

    pub fn matching_plaintext_spec(&self) -> PlaintextSpec {
        PlaintextSpec::new(self.int_size(), self.block_spec().message_size())
    }
}
