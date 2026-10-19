use rand::RngExt;

use super::{EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage, SpecInteger};

/// Specification for the bit layout of a single block.
///
/// A block is a fixed-precision integer partitioned into three contiguous regions, from MSB to
/// LSB: `[padding_bit | carry_bits | message_bits]`. The padding bit (always 1 bit) ensures
/// correct behavior with negacyclic lookup tables. The carry bits store intermediate results
/// during homomorphic arithmetic. The message bits hold the actual data.
///
/// The same specification describes ciphertext blocks and plaintext blocks. Use the factory
/// methods [`from_message`](Self::from_message), [`from_carry`](Self::from_carry),
/// [`from_data`](Self::from_data), and [`from_complete`](Self::from_complete) to create
/// [`EmulatedCiphertextBlock`] values. Plaintext blocks are created through
/// [`EmulatedPlaintextBlock`](super::EmulatedPlaintextBlock) constructors.
///
/// # Examples
///
/// ```
/// use zhc_crypto::integer_semantics::SpecBlock;
///
/// // Create a spec with 2 carry bits and 4 message bits (7 bits total with padding)
/// let spec = SpecBlock(2, 4);
///
/// // Create blocks from different regions
/// let msg_block = spec.from_message(0b1010);
/// let carry_block = spec.from_carry(0b11);
/// let data_block = spec.from_data(0b11_1010); // carry | message
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SpecBlock(
    /// The number of carry bits in this block layout.
    pub u8,
    /// The number of message bits in this block layout.
    pub u8,
);

impl SpecBlock {
    /// Returns the size of the padding region in bits.
    ///
    /// The padding bit is always exactly 1 bit. It ensures that negacyclic PBS lookups access
    /// only the first half of the lookup table when the padding bit is zero.
    pub fn padding_size(&self) -> u8 {
        1
    }

    /// Returns the bitmask isolating the padding bit.
    pub fn padding_mask(&self) -> EmulatedCiphertextBlockStorage {
        1 << (self.carry_size() + self.message_size())
    }

    /// Returns the size of the carry region in bits.
    pub fn carry_size(&self) -> u8 {
        self.0
    }

    /// Returns the bitmask isolating the carry bits, in their original position.
    pub fn carry_mask(&self) -> EmulatedCiphertextBlockStorage {
        ((1 << self.carry_size()) - 1) << self.message_size()
    }

    /// Returns the size of the message region in bits.
    pub fn message_size(&self) -> u8 {
        self.1
    }

    /// Returns the bitmask isolating the message bits.
    pub fn message_mask(&self) -> EmulatedCiphertextBlockStorage {
        (1 << self.message_size()) - 1
    }

    /// Returns the total size of the block in bits: `1 + carry_size + message_size`.
    pub fn complete_size(&self) -> u8 {
        self.padding_size() + self.carry_size() + self.message_size()
    }

    /// Returns the bitmask covering all bits in the block.
    pub fn complete_mask(&self) -> EmulatedCiphertextBlockStorage {
        self.padding_mask() | self.data_mask()
    }

    /// Returns the size of the data region in bits: `carry_size + message_size`.
    pub fn data_size(&self) -> u8 {
        self.carry_size() + self.message_size()
    }

    /// Returns the bitmask covering carry and message bits (excluding padding).
    pub fn data_mask(&self) -> EmulatedCiphertextBlockStorage {
        self.carry_mask() | self.message_mask()
    }

    /// Creates a ciphertext block with the given value in the message region.
    ///
    /// # Panics
    ///
    /// Panics if `message` exceeds the maximum value representable in the message region.
    pub fn from_message(&self, message: EmulatedCiphertextBlockStorage) -> EmulatedCiphertextBlock {
        if self.overflows_message(message) {
            panic!(
                "Input value {} exceeds maximum value for message size of {} bits",
                message,
                self.message_size()
            );
        }
        EmulatedCiphertextBlock {
            storage: message,
            spec: *self,
        }
    }

    /// Creates a ciphertext block with the given value in the carry region.
    ///
    /// # Panics
    ///
    /// Panics if `carry` exceeds the maximum value representable in the carry region.
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
    /// # Panics
    ///
    /// Panics if `data` exceeds the maximum value representable in the data region.
    pub fn from_data(&self, data: EmulatedCiphertextBlockStorage) -> EmulatedCiphertextBlock {
        if self.overflows_carry(data) {
            panic!(
                "Input value {} exceeds maximum value for data size of {} bits",
                data,
                self.data_size()
            );
        }
        EmulatedCiphertextBlock {
            storage: data,
            spec: *self,
        }
    }

    /// Creates a ciphertext block with the given value spanning all regions including padding.
    ///
    /// # Panics
    ///
    /// Panics if `data` exceeds the maximum value representable in the complete block.
    pub fn from_complete(&self, data: EmulatedCiphertextBlockStorage) -> EmulatedCiphertextBlock {
        if self.overflows_padding(data) {
            panic!(
                "Input value {} exceeds maximum value for complete size of {} bits",
                data,
                self.complete_size()
            );
        }
        EmulatedCiphertextBlock {
            storage: data,
            spec: *self,
        }
    }

    /// Checks whether a value exceeds the message region capacity.
    pub fn overflows_message(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        storage >= (1 << self.message_size())
    }

    /// Checks whether a value exceeds the data region capacity.
    pub fn overflows_carry(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        storage >= (1 << self.data_size())
    }

    /// Checks whether a value exceeds the complete block capacity.
    pub fn overflows_padding(&self, storage: EmulatedCiphertextBlockStorage) -> bool {
        storage >= (1 << self.complete_size())
    }

    /// Generates a random ciphertext block spanning the full complete range.
    pub fn random(&self) -> EmulatedCiphertextBlock {
        super::PRNG.with_borrow_mut(|prng| {
            let a = prng.random::<EmulatedCiphertextBlockStorage>() & self.complete_mask();
            self.from_complete(a)
        })
    }

    /// Creates a multi-block integer specification using this block layout.
    ///
    /// # Panics
    ///
    /// Panics if `int_size` is not divisible by the message size.
    pub fn integer_spec(&self, int_size: u16) -> SpecInteger {
        SpecInteger::new(int_size, self.carry_size(), self.message_size())
    }

    /// Iterates over all ciphertext blocks in the data space, in ascending order.
    pub fn iter_data_space(&self) -> impl Iterator<Item = EmulatedCiphertextBlock> {
        (0..=self.data_mask()).map(|a| self.from_data(a))
    }

    /// Iterates over all ciphertext blocks in the message space, in ascending order.
    pub fn iter_message_space(&self) -> impl Iterator<Item = EmulatedCiphertextBlock> {
        (0..=self.message_mask()).map(|a| self.from_message(a))
    }

    /// Iterates over all ciphertext blocks in the complete space, in ascending order.
    pub fn iter_complete_space(&self) -> impl Iterator<Item = EmulatedCiphertextBlock> {
        (0..=self.complete_mask()).map(|a| self.from_complete(a))
    }
}
