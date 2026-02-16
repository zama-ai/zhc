use rand::RngExt;

use super::super::{EmulatedPlaintext, EmulatedPlaintextStorage, PlaintextBlockSpec};

/// Specification for a multi-block plaintext representing a large integer.
///
/// This is the plaintext counterpart to [`CiphertextSpec`](super::super::CiphertextSpec). It
/// defines a large integer decomposed into multiple plaintext blocks, where each block holds a
/// portion of the integer according to a shared [`PlaintextBlockSpec`].
///
/// The total `int_size` must be a multiple of the block's message size — this ensures the integer
/// can be evenly partitioned across blocks.
///
/// Plaintext integers are used for scalar operations with ciphertexts, such as adding a constant
/// to an encrypted value.
///
/// # Examples
///
/// ```
/// use hc_crypto::integer_semantics::PlaintextSpec;
///
/// // Create a spec for 16-bit plaintext integers with 4-bit blocks
/// let spec = PlaintextSpec::new(16, 4);
/// assert_eq!(spec.block_count(), 4); // 16 / 4 = 4 blocks
///
/// // Create a plaintext from an integer value
/// let pt = spec.from_int(0x1234);
///
/// // Access individual blocks
/// let block_0 = pt.get_block(0); // least significant block
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaintextSpec {
    int_size: u16,
    block: PlaintextBlockSpec,
}

impl PlaintextSpec {
    /// Creates a new plaintext specification with the given parameters.
    ///
    /// The `int_size` defines the total number of bits in the integer. The `block_message_size`
    /// defines the per-block layout. The integer size must be divisible by the block message
    /// size so blocks partition the integer evenly.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `int_size` exceeds 128 bits (the underlying storage capacity)
    /// - `block_message_size` is zero
    /// - `int_size` is not divisible by `block_message_size`
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

    /// Returns the total size of the integer in bits.
    ///
    /// This represents the maximum value range `[0, 2^int_size)`.
    pub fn int_size(&self) -> u16 {
        self.int_size
    }

    pub fn int_mask(&self) -> EmulatedPlaintextStorage {
        (1 << (self.block_count() * self.block.message_size())) - 1
    }

    /// Returns the block specification shared by all blocks in this integer.
    pub fn block_spec(&self) -> PlaintextBlockSpec {
        self.block
    }

    /// Returns a bitmask selecting the message bits of the `ith` block within the integer.
    ///
    /// Block 0 is the least significant block. The returned mask can be used to extract or
    /// clear a specific block's contribution to the integer value.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= block_count()`.
    pub fn block_mask(&self, ith: u8) -> EmulatedPlaintextStorage {
        assert!(
            ith < self.block_count(),
            "Tried to get block mask for nonexistent block"
        );
        (self.block.message_mask() as EmulatedPlaintextStorage) << (ith * self.block.message_size())
    }

    /// Returns the number of blocks in this integer.
    ///
    /// Computed as `int_size / block_message_size`.
    pub fn block_count(&self) -> u8 {
        self.int_size.div_euclid(self.block.0 as u16) as u8
    }

    /// Generates a random plaintext with uniformly distributed bits.
    ///
    /// Uses a thread-local PRNG seeded deterministically. Useful for testing and fuzzing.
    pub fn random(&self) -> EmulatedPlaintext {
        super::super::PRNG.with_borrow_mut(|prng| {
            let a = prng.random::<u128>() & self.int_mask();
            self.from_int(a)
        })
    }

    /// Creates a plaintext from a raw integer value.
    ///
    /// The integer is stored directly; individual blocks can then be accessed via
    /// [`EmulatedPlaintext::get_block`].
    ///
    /// # Panics
    ///
    /// Panics if `int >= 2^int_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hc_crypto::integer_semantics::PlaintextSpec;
    ///
    /// let spec = PlaintextSpec::new(8, 2);
    /// let pt = spec.from_int(0b1011_0110);
    /// assert_eq!(pt.get_block(0), spec.block_spec().from_message(0b10)); // bits [1:0]
    /// assert_eq!(pt.get_block(1), spec.block_spec().from_message(0b01)); // bits [3:2]
    /// ```
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

    /// Checks whether a value exceeds the integer's capacity.
    ///
    /// Returns true if `storage >= 2^int_size`.
    pub fn overflows_int(&self, storage: EmulatedPlaintextStorage) -> bool {
        let shift = self.int_size();
        storage >= (1 << shift)
    }
}
