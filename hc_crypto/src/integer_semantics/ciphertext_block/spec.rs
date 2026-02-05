use crate::integer_semantics::CiphertextSpec;

use super::super::PlaintextBlockSpec;
use super::{EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage};

/// Padding, carry, message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CiphertextBlockSpec(
    /// The number of carry bits.
    pub u8,
    /// The number of message bits.
    pub u8,
);

impl CiphertextBlockSpec {
    pub fn padding_size(&self) -> u8 {
        1
    }

    pub(crate) fn padding_mask(&self) -> EmulatedCiphertextBlockStorage {
        1 << (self.carry_size() + self.message_size())
    }

    pub fn carry_size(&self) -> u8 {
        self.0
    }

    pub(crate) fn carry_mask(&self) -> EmulatedCiphertextBlockStorage {
        ((1 << self.carry_size()) - 1) << self.message_size()
    }

    pub fn message_size(&self) -> u8 {
        self.1
    }

    pub(crate) fn message_mask(&self) -> EmulatedCiphertextBlockStorage {
        (1 << self.message_size()) - 1
    }

    pub fn complete_size(&self) -> u8 {
        self.padding_size() + self.carry_size() + self.message_size()
    }

    pub(crate) fn complete_mask(&self) -> EmulatedCiphertextBlockStorage {
        self.padding_mask() | self.data_mask()
    }

    pub fn data_size(&self) -> u8 {
        self.carry_size() + self.message_size()
    }

    pub(crate) fn data_mask(&self) -> EmulatedCiphertextBlockStorage {
        self.carry_mask() | self.message_mask()
    }

    pub fn from_message(&self, message: EmulatedCiphertextBlockStorage) -> EmulatedCiphertextBlock {
        let storage = message;
        if self.overflows_message(storage) {
            panic!(
                "Input value {} exceeds maximum value for message size of {} bits",
                message,
                self.message_size()
            );
        }
        EmulatedCiphertextBlock {
            storage,
            spec: *self,
        }
    }

    pub fn from_carry(&self, carry: EmulatedCiphertextBlockStorage) -> EmulatedCiphertextBlock {
        let storage = carry << self.message_size();
        if self.overflows_carry(storage) {
            panic!(
                "Input value {} exceeds maximum value for carry size of {} bits",
                carry,
                self.carry_size()
            );
        }
        EmulatedCiphertextBlock {
            storage,
            spec: *self,
        }
    }

    pub fn from_data(&self, data: EmulatedCiphertextBlockStorage) -> EmulatedCiphertextBlock {
        let storage = data;
        if self.overflows_carry(storage) {
            panic!(
                "Input value {} exceeds maximum value for data size of {} bits",
                data,
                self.data_size()
            );
        }
        EmulatedCiphertextBlock {
            storage,
            spec: *self,
        }
    }

    pub fn from_complete(&self, data: EmulatedCiphertextBlockStorage) -> EmulatedCiphertextBlock {
        let storage = data;
        if self.overflows_padding(storage) {
            panic!(
                "Input value {} exceeds maximum value for complete size of {} bits",
                data,
                self.complete_size()
            );
        }
        EmulatedCiphertextBlock {
            storage,
            spec: *self,
        }
    }

    pub fn overflows_message(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        let shift = self.message_size();
        storage >= (1 << shift)
    }

    pub fn overflows_carry(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        let shift = self.message_size() + self.carry_size();
        storage >= (1 << shift)
    }

    pub fn overflows_padding(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        let shift = self.message_size() + self.carry_size() + self.padding_size();
        storage >= (1 << shift)
    }

    pub fn matching_plaintext_block_spec(&self) -> PlaintextBlockSpec {
        PlaintextBlockSpec(self.message_size())
    }

    pub fn ciphertext_spec(&self, int_size: u16) -> CiphertextSpec {
        CiphertextSpec::new(int_size, self.carry_size(), self.message_size())
    }
}

impl PartialEq<PlaintextBlockSpec> for CiphertextBlockSpec {
    fn eq(&self, other: &PlaintextBlockSpec) -> bool {
        self.message_size() == other.message_size()
    }
}

impl PartialEq<CiphertextBlockSpec> for PlaintextBlockSpec {
    fn eq(&self, other: &CiphertextBlockSpec) -> bool {
        self.message_size() == other.message_size()
    }
}
