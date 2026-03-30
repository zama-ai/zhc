use rand::RngExt;

use crate::integer_semantics::CiphertextSpec;

use super::super::PlaintextBlockSpec;
use super::{EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage};

/// Specification for the bit layout of a single ciphertext block.
///
/// A ciphertext block is a fixed-precision integer partitioned into three contiguous regions,
/// from MSB to LSB: `[padding_bit | carry_bits | message_bits]`. The padding bit (always 1 bit)
/// ensures correct behavior with negacyclic lookup tables. The carry bits store intermediate
/// results during homomorphic arithmetic. The message bits hold the actual encrypted data.
///
/// Use this specification to create [`EmulatedCiphertextBlock`] values via the factory methods
/// [`from_message`](Self::from_message), [`from_carry`](Self::from_carry),
/// [`from_data`](Self::from_data), and [`from_complete`](Self::from_complete).
///
/// Two specs are compatible for arithmetic operations when their message and carry sizes match.
/// A [`CiphertextBlockSpec`] can also be compared with a [`PlaintextBlockSpec`] for equality —
/// they are considered equal when their message sizes match, which determines whether plaintext
/// and ciphertext blocks can be combined in mixed operations.
///
/// # Examples
///
/// ```
/// use zhc_crypto::integer_semantics::CiphertextBlockSpec;
///
/// // Create a spec with 2 carry bits and 4 message bits (7 bits total with padding)
/// let spec = CiphertextBlockSpec(2, 4);
///
/// // Create blocks from different regions
/// let msg_block = spec.from_message(0b1010);
/// let carry_block = spec.from_carry(0b11);
/// let data_block = spec.from_data(0b11_1010); // carry | message
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CiphertextBlockSpec(
    /// The number of carry bits in this block layout.
    pub u8,
    /// The number of message bits in this block layout.
    pub u8,
);

impl CiphertextBlockSpec {
    /// Returns the size of the padding region in bits.
    ///
    /// The padding bit is always exactly 1 bit. It ensures that negacyclic PBS lookups access
    /// only the first half of the lookup table when the padding bit is zero.
    pub fn padding_size(&self) -> u8 {
        1
    }

    pub fn padding_mask(&self) -> EmulatedCiphertextBlockStorage {
        1 << (self.carry_size() + self.message_size())
    }

    /// Returns the size of the carry region in bits.
    ///
    /// The carry region stores intermediate results during homomorphic arithmetic operations,
    /// allowing multiple additions before a carry propagation is required.
    pub fn carry_size(&self) -> u8 {
        self.0
    }

    pub fn carry_mask(&self) -> EmulatedCiphertextBlockStorage {
        ((1 << self.carry_size()) - 1) << self.message_size()
    }

    /// Returns the size of the message region in bits.
    ///
    /// The message region holds the actual encrypted payload. For a radix integer decomposition,
    /// this determines the radix base: a block with `n` message bits represents values in
    /// `[0, 2^n)`.
    pub fn message_size(&self) -> u8 {
        self.1
    }

    pub fn message_mask(&self) -> EmulatedCiphertextBlockStorage {
        (1 << self.message_size()) - 1
    }

    /// Returns the total size of the block in bits.
    ///
    /// This is the sum of padding, carry, and message sizes: `1 + carry_size + message_size`.
    pub fn complete_size(&self) -> u8 {
        self.padding_size() + self.carry_size() + self.message_size()
    }

    pub fn complete_mask(&self) -> EmulatedCiphertextBlockStorage {
        self.padding_mask() | self.data_mask()
    }

    /// Returns the size of the data region in bits.
    ///
    /// The data region comprises both carry and message bits, excluding the padding bit:
    /// `carry_size + message_size`.
    pub fn data_size(&self) -> u8 {
        self.carry_size() + self.message_size()
    }

    pub fn data_mask(&self) -> EmulatedCiphertextBlockStorage {
        self.carry_mask() | self.message_mask()
    }

    /// Creates a ciphertext block with the given value in the message region.
    ///
    /// The carry and padding regions are set to zero. The provided `message` value must fit
    /// within the message region; use [`overflows_message`](Self::overflows_message) to check
    /// beforehand if needed.
    ///
    /// # Panics
    ///
    /// Panics if `message` exceeds the maximum value representable in the message region.
    ///
    /// # Examples
    ///
    /// ```
    /// use zhc_crypto::integer_semantics::CiphertextBlockSpec;
    ///
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let block = spec.from_message(0b1010); // message = 10, carry = 0, padding = 0
    /// ```
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

    /// Creates a ciphertext block with the given value in the carry region.
    ///
    /// The `carry` value is shifted into the carry position; message and padding regions are
    /// set to zero. The value must fit within the carry region.
    ///
    /// # Panics
    ///
    /// Panics if `carry` exceeds the maximum value representable in the carry region.
    ///
    /// # Examples
    ///
    /// ```
    /// use zhc_crypto::integer_semantics::CiphertextBlockSpec;
    ///
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let block = spec.from_carry(0b11); // message = 0, carry = 3, padding = 0
    /// ```
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

    /// Creates a ciphertext block with the given value spanning both carry and message regions.
    ///
    /// The `data` value occupies the lower `data_size()` bits (carry | message), with the
    /// padding bit set to zero. The value must fit within the data region.
    ///
    /// # Panics
    ///
    /// Panics if `data` exceeds the maximum value representable in the data region.
    ///
    /// # Examples
    ///
    /// ```
    /// use zhc_crypto::integer_semantics::CiphertextBlockSpec;
    ///
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let block = spec.from_data(0b11_1010); // message = 10, carry = 3, padding = 0
    /// ```
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

    /// Creates a ciphertext block with the given value spanning all regions including padding.
    ///
    /// The `data` value occupies all `complete_size()` bits (padding | carry | message). This
    /// is the only factory method that can set the padding bit. The value must fit within the
    /// complete block size.
    ///
    /// # Panics
    ///
    /// Panics if `data` exceeds the maximum value representable in the complete block.
    ///
    /// # Examples
    ///
    /// ```
    /// use zhc_crypto::integer_semantics::CiphertextBlockSpec;
    ///
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let block = spec.from_complete(0b1_11_1010); // message = 10, carry = 3, padding = 1
    /// ```
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

    /// Checks whether a value exceeds the message region capacity.
    ///
    /// Returns true if `storage >= 2^message_size`.
    pub fn overflows_message(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        let shift = self.message_size();
        storage >= (1 << shift)
    }

    /// Checks whether a value exceeds the data region capacity.
    ///
    /// Returns true if `storage >= 2^(message_size + carry_size)`. This checks overflow for
    /// values intended to span both message and carry regions.
    pub fn overflows_carry(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        let shift = self.message_size() + self.carry_size();
        storage >= (1 << shift)
    }

    /// Checks whether a value exceeds the complete block capacity.
    ///
    /// Returns true if `storage >= 2^(message_size + carry_size + 1)`. This checks overflow
    /// for values intended to span all regions including the padding bit.
    pub fn overflows_padding(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        let shift = self.message_size() + self.carry_size() + self.padding_size();
        storage >= (1 << shift)
    }

    /// Returns the corresponding plaintext block specification.
    ///
    /// The returned [`PlaintextBlockSpec`] has the same message size as this ciphertext spec,
    /// allowing plaintext blocks to be used in mixed ciphertext-plaintext operations.
    pub fn matching_plaintext_block_spec(&self) -> PlaintextBlockSpec {
        PlaintextBlockSpec(self.message_size())
    }

    /// Returns a plaintext block spec matching the complete (message + carry) size.
    ///
    /// Unlike [`matching_plaintext_block_spec`](Self::matching_plaintext_block_spec) which
    /// matches only the message bits, this includes carry bits for operations that need
    /// access to the full block value.
    pub fn complete_plaintext_block_spec(&self) -> PlaintextBlockSpec {
        PlaintextBlockSpec(self.complete_size())
    }

    /// Generates a random ciphertext block using a thread-local PRNG.
    ///
    /// Useful for testing and fuzzing. The generated value spans the full complete range.
    pub fn random(&self) -> EmulatedCiphertextBlock {
        super::super::PRNG.with_borrow_mut(|prng| {
            let a = prng.random::<EmulatedCiphertextBlockStorage>() & self.complete_mask();
            self.from_complete(a)
        })
    }

    /// Creates a multi-block ciphertext specification using this block layout.
    ///
    /// The `int_size` parameter specifies the total number of message bits across all blocks
    /// in the resulting integer. It must be a multiple of this spec's message size.
    ///
    /// # Panics
    ///
    /// Panics if `int_size` is not divisible by the message size.
    pub fn ciphertext_spec(&self, int_size: u16) -> CiphertextSpec {
        CiphertextSpec::new(int_size, self.carry_size(), self.message_size())
    }
}
