use super::super::{EmulatedPlaintextBlock, EmulatedPlaintextBlockStorage, SpecInteger};
use super::EmulatedPlaintextIntegerStorage;
use std::fmt::Debug;
use zhc_utils::{Dumpable, SafeAs, iter::Separate};

/// An emulated plaintext integer for use in ciphertext-plaintext operations.
///
/// This structure models a multi-block plaintext where a large integer is decomposed into
/// multiple [`EmulatedPlaintextBlock`] values using a fixed radix. Each block holds a portion
/// of the integer's bits according to a shared [`SpecInteger`].
///
/// Plaintexts are created via [`from_int`](Self::from_int) or [`random`](Self::random).
/// Individual blocks can be accessed with [`get_block`](Self::get_block). Block 0 is the
/// least significant.
///
/// Integer plaintexts are used as scalar operands in mixed operations with ciphertexts. A
/// plaintext is compatible with a ciphertext when their specs are equal.
///
/// Two plaintexts can be compared for equality or ordering only if they share the same spec.
///
/// # Debug formatting
///
/// - Default format: `{block_n}_.._{block_0}_ptint` (decimal block values, MSB first)
/// - Alternate format (`{:#?}`): binary representation with proper bit widths
#[derive(Clone, Copy)]
pub struct EmulatedPlaintextInteger {
    pub(crate) storage: EmulatedPlaintextIntegerStorage,
    pub(crate) spec: SpecInteger,
}

impl EmulatedPlaintextInteger {
    /// Creates a plaintext integer from a raw integer value.
    ///
    /// # Panics
    ///
    /// Panics if `int >= 2^int_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zhc_crypto::integer_semantics::{EmulatedPlaintextBlock, EmulatedPlaintextInteger, SpecInteger};
    ///
    /// let spec = SpecInteger::new(8, 2, 2);
    /// let pt = EmulatedPlaintextInteger::from_int(spec, 0b1011_0110);
    /// let block = spec.block_spec();
    /// assert_eq!(pt.get_block(0), EmulatedPlaintextBlock::from_message(block, 0b10)); // bits [1:0]
    /// assert_eq!(pt.get_block(1), EmulatedPlaintextBlock::from_message(block, 0b01)); // bits [3:2]
    /// ```
    pub fn from_int(spec: SpecInteger, int: EmulatedPlaintextIntegerStorage) -> Self {
        if spec.overflows_int(int) {
            panic!(
                "Input value {} exceeds maximum value for int size of {} bits",
                int,
                spec.int_size()
            );
        }
        Self { storage: int, spec }
    }

    /// Generates a random plaintext integer with uniformly distributed bits.
    pub fn random(spec: SpecInteger) -> Self {
        Self::from_int(spec, spec.random_storage())
    }

    /// Returns the number of blocks in this plaintext.
    pub fn len(&self) -> u8 {
        self.spec.block_count()
    }

    /// Returns the block at the given index.
    ///
    /// Block 0 is the least significant block.
    ///
    /// # Panics
    ///
    /// Panics if `ith >= len()`.
    pub fn get_block(&self, ith: u8) -> EmulatedPlaintextBlock {
        assert!(ith < self.len(), "Tried to get nonexistent block.");
        let storage = (self.storage >> (ith * self.spec.block_spec().message_size()))
            as EmulatedPlaintextBlockStorage
            & self.spec.block_spec().message_mask();
        EmulatedPlaintextBlock {
            storage,
            spec: self.spec.block_spec(),
        }
    }

    /// Returns the specification describing this plaintext's layout.
    pub fn spec(&self) -> SpecInteger {
        self.spec
    }

    /// Returns the integer value reconstructed from all blocks' message bits.
    pub fn as_storage(&self) -> EmulatedPlaintextIntegerStorage {
        self.storage & self.spec.int_mask()
    }
}

impl Debug for EmulatedPlaintextInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alternate = f.alternate();
        (0..self.len())
            .rev()
            .map(|i| self.get_block(i))
            .map(|block| {
                if alternate {
                    format!(
                        "{:0width$b}",
                        block.storage,
                        width = self.spec.block_spec().message_size().sas::<usize>()
                    )
                } else {
                    format!("{}", block.storage,)
                }
            })
            .separate_with(|| format!("_"))
            .for_each(|string| write!(f, "{}", string).unwrap());
        write!(f, "_ptint")
    }
}

impl PartialEq for EmulatedPlaintextInteger {
    fn eq(&self, other: &Self) -> bool {
        self.as_storage() == other.as_storage() && self.spec == other.spec
    }
}

impl Eq for EmulatedPlaintextInteger {}

impl PartialOrd for EmulatedPlaintextInteger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec != other.spec {
            None
        } else {
            self.as_storage().partial_cmp(&other.as_storage())
        }
    }
}

impl std::hash::Hash for EmulatedPlaintextInteger {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_storage().hash(state);
        self.spec.hash(state);
    }
}

impl Dumpable for EmulatedPlaintextInteger {
    fn dump_to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
