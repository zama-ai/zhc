use crate::integer_semantics::PlaintextSpec;

use super::{PlaintextBlock, PlaintextBlockStorage};

/// Plaintext block specification with message bits only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaintextBlockSpec(
    /// The number of message bits.
    pub u8,
);

impl PlaintextBlockSpec {
    /// Returns the number of message bits.
    pub fn message_size(&self) -> u8 {
        self.0
    }

    /// Returns the message mask for extracting message bits.
    pub(crate) fn message_mask(&self) -> PlaintextBlockStorage {
        (1 << self.message_size()) - 1
    }

    /// Creates a plaintext block from a message value.
    pub fn from_message(&self, message: PlaintextBlockStorage) -> PlaintextBlock {
        if self.overflows_message(message) {
            panic!(
                "Input value {} exceeds maximum value for message size of {} bits",
                message,
                self.message_size()
            );
        }
        PlaintextBlock {
            storage: message,
            spec: *self,
        }
    }

    /// Checks if the given storage value overflows the message size.
    pub fn overflows_message(&self, storage: PlaintextBlockStorage) -> bool {
        let shift = self.message_size();
        storage >= (1 << shift)
    }

    pub fn plaintext_spec(&self, int_size: u16) -> PlaintextSpec {
        PlaintextSpec::new(int_size, self.message_size())
    }
}
