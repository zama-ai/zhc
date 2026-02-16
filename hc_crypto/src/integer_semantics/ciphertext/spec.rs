use rand::RngExt;

use crate::integer_semantics::PlaintextSpec;

use super::super::{CiphertextBlockSpec, EmulatedCiphertext, EmulatedCiphertextStorage};

/// Specification for a multi-block radix ciphertext representing a large integer.
///
/// TFHE operates on large integers by decomposing them into multiple LWE ciphertext blocks using
/// a fixed radix. Each block holds a portion of the integer's bits according to a shared
/// [`CiphertextBlockSpec`]. This specification defines both the total integer size and the
/// per-block layout.
///
/// The total `int_size` must be a multiple of the block's message size — this ensures the integer
/// can be evenly partitioned across blocks. For example, a 64-bit integer with 4-bit message
/// blocks requires exactly 16 blocks.
///
/// Use [`from_int`](Self::from_int) to create an [`EmulatedCiphertext`] from a raw integer value,
/// or [`random`](Self::random) to generate a random ciphertext for testing.
///
/// # Examples
///
/// ```
/// use hc_crypto::integer_semantics::CiphertextSpec;
///
/// // Create a spec for 16-bit integers using blocks with 2 carry bits and 4 message bits
/// let spec = CiphertextSpec::new(16, 2, 4);
/// assert_eq!(spec.block_count(), 4); // 16 / 4 = 4 blocks
///
/// // Create a ciphertext from an integer value
/// let ct = spec.from_int(0x1234);
///
/// // Access individual blocks
/// let block_0 = ct.get_block(0); // least significant block
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CiphertextSpec {
    int_size: u16,
    block: CiphertextBlockSpec,
}

impl CiphertextSpec {
    /// Creates a new ciphertext specification with the given parameters.
    ///
    /// The `int_size` defines the total number of message bits in the integer. The
    /// `block_carry_size` and `block_message_size` define the per-block layout. The integer
    /// size must be divisible by the block message size so blocks partition the integer evenly.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `int_size` exceeds 128 bits (the underlying storage capacity)
    /// - `block_carry_size` is zero
    /// - `block_message_size` is zero
    /// - `int_size` is not divisible by `block_message_size`
    pub fn new(int_size: u16, block_carry_size: u8, block_message_size: u8) -> Self {
        assert!(
            int_size <= EmulatedCiphertextStorage::BITS as u16,
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

    /// Returns the total size of the integer in bits.
    ///
    /// This is the sum of all message bits across all blocks, representing the maximum value
    /// range `[0, 2^int_size)`.
    pub fn int_size(&self) -> u16 {
        self.int_size
    }

    pub fn int_mask(&self) -> EmulatedCiphertextStorage {
        (1 << (self.block_count() * self.block.message_size())) - 1
    }

    /// Returns the block specification shared by all blocks in this integer.
    pub fn block_spec(&self) -> CiphertextBlockSpec {
        self.block
    }

    /// Returns the number of blocks in this integer.
    ///
    /// Computed as `int_size / block_message_size`.
    pub fn block_count(&self) -> u8 {
        self.int_size.div_euclid(self.block.1 as u16) as u8
    }

    /// Returns a bitmask selecting the message bits of the `ith` block within the integer.
    ///
    /// Block 0 is the least significant block. The returned mask can be used to extract or
    /// clear a specific block's contribution to the integer value.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= block_count()`.
    pub fn block_mask(&self, ith: u8) -> EmulatedCiphertextStorage {
        assert!(
            ith < self.block_count(),
            "Tried to get block mask for nonexistent block"
        );
        (self.block.message_mask() as EmulatedCiphertextStorage)
            << (ith * self.block.message_size())
    }

    /// Generates a random ciphertext with uniformly distributed message bits.
    ///
    /// Uses a thread-local PRNG seeded deterministically. Useful for testing and fuzzing.
    pub fn random(&self) -> EmulatedCiphertext {
        super::super::PRNG.with_borrow_mut(|prng| {
            let a = prng.random::<u128>() & self.int_mask();
            self.from_int(a)
        })
    }

    /// Creates a ciphertext from a raw integer value.
    ///
    /// The integer is stored directly; individual blocks can then be accessed via
    /// [`EmulatedCiphertext::get_block`]. All blocks will have zero carry and padding bits.
    ///
    /// # Panics
    ///
    /// Panics if `int >= 2^int_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hc_crypto::integer_semantics::CiphertextSpec;
    ///
    /// let spec = CiphertextSpec::new(8, 2, 2);
    /// let ct = spec.from_int(0b1011_0110);
    /// assert_eq!(ct.get_block(0).spec().from_message(0b10), ct.get_block(0)); // bits [1:0]
    /// assert_eq!(ct.get_block(1).spec().from_message(0b01), ct.get_block(1)); // bits [3:2]
    /// ```
    pub fn from_int(&self, int: EmulatedCiphertextStorage) -> EmulatedCiphertext {
        let storage = int;
        if self.overflows_int(storage) {
            panic!(
                "Input value {} exceeds maximum value for int size of {} bits",
                storage,
                self.int_size()
            );
        }
        EmulatedCiphertext {
            storage,
            spec: *self,
        }
    }

    /// Checks whether a value exceeds the integer's capacity.
    ///
    /// Returns true if `storage >= 2^int_size`.
    pub fn overflows_int(&self, storage: EmulatedCiphertextStorage) -> bool {
        let shift = self.int_size();
        storage >= (1 << shift)
    }

    /// Returns the corresponding plaintext specification.
    ///
    /// The returned [`PlaintextSpec`] has the same integer size and block message size,
    /// allowing plaintext integers to be used in mixed ciphertext-plaintext operations.
    pub fn matching_plaintext_spec(&self) -> PlaintextSpec {
        PlaintextSpec::new(self.int_size(), self.block.message_size())
    }
}
