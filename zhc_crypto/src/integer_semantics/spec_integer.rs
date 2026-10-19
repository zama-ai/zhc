use rand::RngExt;
use zhc_utils::SafeAs;

use super::{EmulatedCiphertextInteger, EmulatedCiphertextIntegerStorage, SpecBlock};

/// Specification for a multi-block radix integer.
///
/// TFHE operates on large integers by decomposing them into multiple blocks using a fixed radix.
/// Each block holds a portion of the integer's bits according to a shared [`SpecBlock`]. This
/// specification defines both the total integer size and the per-block layout.
///
/// The same specification describes ciphertext integers and plaintext integers. The total
/// `int_size` must be a multiple of the block's message size, so the integer can be evenly
/// partitioned across blocks.
///
/// Use [`from_int`](Self::from_int) to create an [`EmulatedCiphertextInteger`] from a raw integer
/// value, or [`random`](Self::random) to generate a random ciphertext for testing. Plaintext
/// integers are created through [`EmulatedPlaintextInteger`](super::EmulatedPlaintextInteger)
/// constructors.
///
/// # Examples
///
/// ```
/// use zhc_crypto::integer_semantics::SpecInteger;
///
/// // Create a spec for 16-bit integers using blocks with 2 carry bits and 4 message bits
/// let spec = SpecInteger::new(16, 2, 4);
/// assert_eq!(spec.block_count(), 4); // 16 / 4 = 4 blocks
///
/// // Create a ciphertext from an integer value
/// let ct = spec.from_int(0x1234);
///
/// // Access individual blocks
/// let block_0 = ct.get_block(0); // least significant block
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecInteger {
    int_size: u16,
    block: SpecBlock,
}

impl SpecInteger {
    /// Creates a new integer specification with the given parameters.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `int_size` is zero or exceeds 128 bits (the underlying storage capacity)
    /// - `block_carry_size` is zero
    /// - `block_message_size` is zero
    /// - `int_size` is not divisible by `block_message_size`
    pub fn new(int_size: u16, block_carry_size: u8, block_message_size: u8) -> Self {
        assert_ne!(
            int_size, 0,
            "Tried to create malformed integer spec: precision must be non-zero."
        );
        assert!(
            int_size <= EmulatedCiphertextIntegerStorage::BITS.sas::<u16>(),
            "Tried to create malformed integer spec. Int size: {int_size}."
        );
        assert_ne!(
            block_carry_size, 0,
            "Tried to create malformed integer spec: carry size must be non-zero."
        );
        assert_ne!(
            block_message_size, 0,
            "Tried to create malformed integer spec: message size must be non-zero."
        );
        assert!(
            int_size.is_multiple_of(block_message_size.sas()),
            "Tried to create malformed integer spec: precision {int_size} is not a multiple of message size {block_message_size}."
        );
        Self {
            int_size,
            block: SpecBlock(block_carry_size, block_message_size),
        }
    }

    /// Returns the total size of the integer in bits.
    pub fn int_size(&self) -> u16 {
        self.int_size
    }

    /// Returns the bitmask covering the `int_size` bits of the integer.
    pub fn int_mask(&self) -> EmulatedCiphertextIntegerStorage {
        EmulatedCiphertextIntegerStorage::MAX
            >> (EmulatedCiphertextIntegerStorage::BITS as u16 - self.int_size())
    }

    /// Returns the block specification shared by all blocks in this integer.
    pub fn block_spec(&self) -> SpecBlock {
        self.block
    }

    /// Returns the number of blocks in this integer: `int_size / block_message_size`.
    pub fn block_count(&self) -> u8 {
        (self.int_size / self.block.message_size().sas::<u16>()).sas::<u8>()
    }

    /// Returns a bitmask selecting the message bits of the `ith` block within the integer.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= block_count()`.
    pub fn block_mask(&self, ith: u8) -> EmulatedCiphertextIntegerStorage {
        assert!(
            ith < self.block_count(),
            "Tried to get block mask for nonexistent block"
        );
        (self
            .block
            .message_mask()
            .sas::<EmulatedCiphertextIntegerStorage>())
            << (ith * self.block.message_size())
    }

    /// Generates a random ciphertext integer with uniformly distributed message bits.
    pub fn random(&self) -> EmulatedCiphertextInteger {
        self.from_int(self.random_storage())
    }

    pub(crate) fn random_storage(&self) -> EmulatedCiphertextIntegerStorage {
        super::PRNG.with_borrow_mut(|prng| {
            let shr = prng.random_range(0..self.int_size());
            let shl = prng.random_range(0..self.int_size());
            let value = prng.random::<u128>() & self.int_mask();
            ((value >> shr) << shl) & self.int_mask()
        })
    }

    /// Creates a ciphertext integer from a raw integer value.
    ///
    /// # Panics
    ///
    /// Panics if `int >= 2^int_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zhc_crypto::integer_semantics::SpecInteger;
    ///
    /// let spec = SpecInteger::new(8, 2, 2);
    /// let ct = spec.from_int(0b1011_0110);
    /// assert_eq!(ct.get_block(0).spec().from_message(0b10), ct.get_block(0)); // bits [1:0]
    /// assert_eq!(ct.get_block(1).spec().from_message(0b01), ct.get_block(1)); // bits [3:2]
    /// ```
    pub fn from_int(&self, int: EmulatedCiphertextIntegerStorage) -> EmulatedCiphertextInteger {
        if self.overflows_int(int) {
            panic!(
                "Input value {} exceeds maximum value for int size of {} bits",
                int,
                self.int_size()
            );
        }
        EmulatedCiphertextInteger {
            storage: int,
            spec: *self,
        }
    }

    /// Checks whether a value exceeds the integer's capacity.
    pub fn overflows_int(&self, storage: EmulatedCiphertextIntegerStorage) -> bool {
        storage > self.int_mask()
    }
}
