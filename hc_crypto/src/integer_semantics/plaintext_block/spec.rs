use crate::integer_semantics::PlaintextSpec;

use super::{EmulatedPlaintextBlock, EmulatedPlaintextBlockStorage};

/// Specification for the bit layout of a single plaintext block.
///
/// Unlike ciphertext blocks, plaintext blocks contain only message bits — there are no carry or
/// padding regions. This simpler layout mirrors the message portion of a
/// [`CiphertextBlockSpec`](crate::integer_semantics::CiphertextBlockSpec), enabling mixed
/// plaintext-ciphertext operations.
///
/// A [`PlaintextBlockSpec`] is considered equal to a
/// [`CiphertextBlockSpec`](crate::integer_semantics::CiphertextBlockSpec) when their message
/// sizes match. This cross-type equality determines compatibility for operations like
/// [`protect_add_pt`](crate::integer_semantics::EmulatedCiphertextBlock::protect_add_pt).
///
/// # Examples
///
/// ```
/// use hc_crypto::integer_semantics::{PlaintextBlockSpec, CiphertextBlockSpec};
///
/// // Create a spec for 4-bit plaintext blocks
/// let spec = PlaintextBlockSpec(4);
///
/// // Create a block from a message value
/// let block = spec.from_message(0b1010);
///
/// // Check compatibility with a ciphertext spec
/// let ct_spec = CiphertextBlockSpec(2, 4);
/// assert!(spec == ct_spec); // equal because message sizes match
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaintextBlockSpec(
    /// The number of message bits in this block layout.
    pub u8,
);

impl PlaintextBlockSpec {
    /// Returns the size of the message region in bits.
    ///
    /// For plaintext blocks, this is also the total block size since there are no carry or
    /// padding regions.
    pub fn message_size(&self) -> u8 {
        self.0
    }

    pub(crate) fn message_mask(&self) -> EmulatedPlaintextBlockStorage {
        (1 << self.message_size()) - 1
    }

    /// Creates a plaintext block with the given message value.
    ///
    /// The provided `message` value must fit within the message region; use
    /// [`overflows_message`](Self::overflows_message) to check beforehand if needed.
    ///
    /// # Panics
    ///
    /// Panics if `message` exceeds the maximum value representable in the message region.
    ///
    /// # Examples
    ///
    /// ```
    /// use hc_crypto::integer_semantics::PlaintextBlockSpec;
    ///
    /// let spec = PlaintextBlockSpec(4);
    /// let block = spec.from_message(0b1010); // message = 10
    /// ```
    pub fn from_message(&self, message: EmulatedPlaintextBlockStorage) -> EmulatedPlaintextBlock {
        if self.overflows_message(message) {
            panic!(
                "Input value {} exceeds maximum value for message size of {} bits",
                message,
                self.message_size()
            );
        }
        EmulatedPlaintextBlock {
            storage: message,
            spec: *self,
        }
    }

    /// Checks whether a value exceeds the message region capacity.
    ///
    /// Returns true if `storage >= 2^message_size`.
    pub fn overflows_message(&self, storage: EmulatedPlaintextBlockStorage) -> bool {
        let shift = self.message_size();
        storage >= (1 << shift)
    }

    /// Creates a multi-block plaintext specification using this block layout.
    ///
    /// The `int_size` parameter specifies the total number of message bits across all blocks
    /// in the resulting integer. It must be a multiple of this spec's message size.
    ///
    /// # Panics
    ///
    /// Panics if `int_size` is not divisible by the message size.
    pub fn plaintext_spec(&self, int_size: u16) -> PlaintextSpec {
        PlaintextSpec::new(int_size, self.message_size())
    }
}
