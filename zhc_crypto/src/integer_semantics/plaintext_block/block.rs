use rand::RngExt;
use zhc_utils::{Dumpable, SafeAs};

use super::super::SpecBlock;
use super::EmulatedPlaintextBlockStorage;
use std::fmt::Debug;

/// An emulated plaintext block for use in ciphertext-plaintext operations.
///
/// This structure models a single plaintext block with the same layout as a ciphertext block:
/// `[padding_bit | carry_bits | message_bits]`. Plaintext blocks are used as scalar operands in
/// mixed operations with ciphertext blocks, such as
/// [`protect_add_pt`](super::super::EmulatedCiphertextBlock::protect_add_pt). A plaintext block
/// is compatible with a ciphertext block when their specs are equal.
///
/// Blocks are created via [`from_message`](Self::from_message) or
/// [`from_complete`](Self::from_complete).
///
/// Two blocks can be compared for equality or ordering only if they share the same spec.
///
/// # Debug formatting
///
/// - Default format: `{padding}_{carry}_{message}_ptblock` (decimal values)
/// - Alternate format (`{:#?}`): binary representation with proper bit widths
#[derive(Clone, Copy)]
pub struct EmulatedPlaintextBlock {
    pub(crate) storage: EmulatedPlaintextBlockStorage,
    pub(crate) spec: SpecBlock,
}

impl EmulatedPlaintextBlock {
    /// Creates a plaintext block with the given value in the message region.
    ///
    /// # Panics
    ///
    /// Panics if `message` exceeds the maximum value representable in the message region.
    pub fn from_message(spec: SpecBlock, message: EmulatedPlaintextBlockStorage) -> Self {
        if spec.overflows_message(message) {
            panic!(
                "Input value {} exceeds maximum value for message size of {} bits",
                message,
                spec.message_size()
            );
        }
        Self {
            storage: message,
            spec,
        }
    }

    /// Creates a plaintext block with the given value spanning all regions including padding.
    ///
    /// # Panics
    ///
    /// Panics if `data` exceeds the maximum value representable in the complete block.
    pub fn from_complete(spec: SpecBlock, data: EmulatedPlaintextBlockStorage) -> Self {
        if spec.overflows_padding(data) {
            panic!(
                "Input value {} exceeds maximum value for complete size of {} bits",
                data,
                spec.complete_size()
            );
        }
        Self {
            storage: data,
            spec,
        }
    }

    /// Generates a random plaintext block spanning the message range.
    pub fn random(spec: SpecBlock) -> Self {
        super::super::PRNG.with_borrow_mut(|prng| {
            let a = prng.random::<EmulatedPlaintextBlockStorage>() & spec.message_mask();
            Self::from_message(spec, a)
        })
    }

    /// Returns the message bits in their original position, with all other bits cleared.
    pub fn raw_mask_message(&self) -> EmulatedPlaintextBlockStorage {
        self.storage & self.spec.message_mask()
    }

    /// Returns all bits (padding | carry | message) as a right-aligned integer.
    pub fn raw_complete_bits(&self) -> EmulatedPlaintextBlockStorage {
        self.storage & self.spec.complete_mask()
    }

    /// Returns the message value as a right-aligned integer.
    pub fn raw_message_bits(&self) -> EmulatedPlaintextBlockStorage {
        self.raw_mask_message()
    }

    /// Returns the carry value as a right-aligned integer.
    pub fn raw_carry_bits(&self) -> EmulatedPlaintextBlockStorage {
        (self.storage & self.spec.carry_mask()) >> self.spec.message_size()
    }

    /// Checks whether this block contains only message bits.
    pub fn is_message_only(&self) -> bool {
        (self.raw_complete_bits() >> self.spec.message_size()) == 0
    }

    /// Checks whether this block contains only carry bits.
    pub fn is_carry_only(&self) -> bool {
        self.raw_message_bits() == 0 && (self.raw_complete_bits() >> self.spec.data_size()) == 0
    }

    /// Returns the specification describing this block's layout.
    pub fn spec(&self) -> SpecBlock {
        self.spec
    }
}

impl Debug for EmulatedPlaintextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = self.raw_complete_bits() >> self.spec.data_size();
        if f.alternate() {
            write!(
                f,
                "{:0padding_size$b}_{:0carry_size$b}_{:0message_size$b}_ptblock",
                padding,
                self.raw_carry_bits(),
                self.raw_message_bits(),
                padding_size = self.spec.padding_size().sas(),
                carry_size = self.spec.carry_size().sas(),
                message_size = self.spec.message_size().sas()
            )
        } else {
            write!(
                f,
                "{}_{}_{}_ptblock",
                padding,
                self.raw_carry_bits(),
                self.raw_message_bits()
            )
        }
    }
}

impl PartialEq for EmulatedPlaintextBlock {
    fn eq(&self, other: &Self) -> bool {
        self.raw_complete_bits() == other.raw_complete_bits() && self.spec == other.spec
    }
}

impl Eq for EmulatedPlaintextBlock {}

impl PartialOrd for EmulatedPlaintextBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec != other.spec {
            return None;
        }
        self.raw_complete_bits()
            .partial_cmp(&other.raw_complete_bits())
    }
}

impl std::hash::Hash for EmulatedPlaintextBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_complete_bits().hash(state);
        self.spec.hash(state);
    }
}

impl Dumpable for EmulatedPlaintextBlock {
    fn dump_to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
