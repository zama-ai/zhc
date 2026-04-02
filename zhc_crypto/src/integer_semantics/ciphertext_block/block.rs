use zhc_utils::{Dumpable, SafeAs};

use super::{CiphertextBlockSpec, EmulatedCiphertextBlockStorage};
use std::fmt::Debug;

/// An emulated TFHE ciphertext block for semantic testing.
///
/// This structure models a single LWE ciphertext block as a fixed-precision integer with three
/// regions: `[padding_bit | carry_bits | message_bits]`. It emulates the behavior of encrypted
/// blocks without actual encryption, enabling fast validation of homomorphic operation semantics.
///
/// Blocks are created via [`CiphertextBlockSpec`] factory methods like
/// [`from_message`](CiphertextBlockSpec::from_message). Arithmetic operations are available as
/// methods with different semantics (`protect_*`, `temper_*`, `wrapping_*`) — see the
/// [module documentation](super::super) for details.
///
/// Two blocks can be compared for equality or ordering only if they share the same spec. The
/// comparison considers all bits (padding, carry, and message).
///
/// # Debug formatting
///
/// - Default format: `{padding}_{carry}_{message}_cblk` (decimal values)
/// - Alternate format (`{:#?}`): binary representation with proper bit widths
#[derive(Clone, Copy)]
pub struct EmulatedCiphertextBlock {
    pub(crate) storage: EmulatedCiphertextBlockStorage,
    pub(crate) spec: CiphertextBlockSpec,
}

impl EmulatedCiphertextBlock {
    /// Returns the message bits in their original position, with all other bits cleared.
    ///
    /// The result occupies the lowest bits of the storage; no shifting is performed.
    pub fn raw_mask_message(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.message_mask()
    }

    /// Returns the carry bits in their original position, with all other bits cleared.
    ///
    /// The result is *not* shifted down; use [`raw_carry_bits`](Self::raw_carry_bits) to obtain
    /// the carry value as a right-aligned integer.
    pub fn raw_mask_carry(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.carry_mask()
    }

    /// Returns the padding bit in its original position, with all other bits cleared.
    ///
    /// The result is *not* shifted down; use [`raw_padding_bits`](Self::raw_padding_bits) to
    /// obtain the padding value as 0 or 1.
    pub fn raw_mask_padding(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.padding_mask()
    }

    /// Returns the data bits (carry | message) in their original position.
    ///
    /// The padding bit is cleared. This is equivalent to
    /// `raw_mask_carry() | raw_mask_message()`.
    pub fn raw_mask_data(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.data_mask()
    }

    /// Returns all bits (padding | carry | message) masked to the block's valid range.
    ///
    /// Any bits beyond the block's complete size are cleared.
    pub fn raw_mask_complete(&self) -> EmulatedCiphertextBlockStorage {
        self.storage & self.spec.complete_mask()
    }

    /// Returns the message value as a right-aligned integer.
    ///
    /// Equivalent to [`raw_mask_message`](Self::raw_mask_message) since message bits are already
    /// in the lowest position.
    pub fn raw_message_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_message()
    }

    /// Returns the carry value as a right-aligned integer.
    ///
    /// The carry bits are shifted down to the lowest position. For a spec with 4 message bits
    /// and carry value 3 (binary `11`), this returns `3`, not `0b11_0000`.
    pub fn raw_carry_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_carry() >> self.spec.message_size()
    }

    /// Returns the padding bit value as 0 or 1.
    ///
    /// The padding bit is shifted down to the lowest position.
    pub fn raw_padding_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_padding() >> self.spec.data_size()
    }

    /// Returns the data value (carry | message) as a right-aligned integer.
    ///
    /// Equivalent to [`raw_mask_data`](Self::raw_mask_data) since data bits are already in
    /// the lowest position (only the padding bit is excluded).
    pub fn raw_data_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_data()
    }

    /// Returns all bits as a right-aligned integer.
    ///
    /// Equivalent to [`raw_mask_complete`](Self::raw_mask_complete) since the complete value
    /// starts at bit 0.
    pub fn raw_complete_bits(&self) -> EmulatedCiphertextBlockStorage {
        self.raw_mask_complete()
    }

    /// Checks whether bit `i` (counting from LSB) is set.
    ///
    /// Bit 0 is the least significant bit of the message region.
    ///
    /// # Panics
    ///
    /// Panics if `i >= complete_size()`.
    pub fn has_active_ith_bit(&self, i: u8) -> bool {
        assert!(i < self.spec().complete_size());
        (self.storage >> i) & 1 == 1
    }

    /// Checks whether bit `i` (counting from MSB) is set.
    ///
    /// Bit 0 is the padding bit (the most significant bit). Bit 1 is the topmost carry bit.
    ///
    /// # Panics
    ///
    /// Panics if `i >= complete_size()`.
    pub fn has_active_last_ith_bit(&self, i: u8) -> bool {
        self.has_active_ith_bit(self.spec().complete_size() - i - 1)
    }

    /// Checks whether the padding bit is set.
    ///
    /// A block with an active padding bit will trigger negacyclic folding during a PBS lookup.
    pub fn has_active_padding_bit(&self) -> bool {
        self.has_active_last_ith_bit(0)
    }

    /// Returns the specification describing this block's layout.
    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }

    /// Returns a new block containing only the message bits, with carry and padding cleared.
    ///
    /// This is useful for extracting the message portion after arithmetic operations that may
    /// have produced carries.
    pub fn mask_message(&self) -> EmulatedCiphertextBlock {
        EmulatedCiphertextBlock {
            storage: self.raw_mask_message(),
            spec: self.spec,
        }
    }

    /// Returns a new block containing only the carry bits in their original position.
    ///
    /// Message and padding bits are cleared. The carry value remains shifted; use
    /// [`move_carry_to_message`](Self::move_carry_to_message) to shift the carry into the
    /// message position.
    pub fn mask_carry(&self) -> EmulatedCiphertextBlock {
        EmulatedCiphertextBlock {
            storage: self.raw_mask_carry(),
            spec: self.spec,
        }
    }

    /// Returns a new block with the carry bits shifted down into the message position.
    ///
    /// The original message and padding bits are discarded. This is useful for propagating
    /// carries between blocks: extract the carry from one block and add it to the next.
    pub fn move_carry_to_message(&self) -> EmulatedCiphertextBlock {
        EmulatedCiphertextBlock {
            storage: self.raw_mask_carry() >> self.spec.message_size(),
            spec: self.spec,
        }
    }

    /// Checks whether this block contains only message bits.
    ///
    /// Returns true if both carry and padding bits are zero. A block must be message-only
    /// before it can be written back into an
    /// [`EmulatedCiphertext`](super::super::EmulatedCiphertext)
    /// via [`set_block`](super::super::EmulatedCiphertext::set_block).
    pub fn is_message_only(&self) -> bool {
        (self.raw_complete_bits() >> self.spec.message_size()) == 0
    }
}

impl Debug for EmulatedCiphertextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{:0padding_size$b}_{:0carry_size$b}_{:0message_size$b}_ctblock",
                self.raw_padding_bits(),
                self.raw_carry_bits(),
                self.raw_message_bits(),
                padding_size = self.spec.padding_size().sas(),
                carry_size = self.spec.carry_size().sas(),
                message_size = self.spec.message_size().sas()
            )
        } else {
            write!(
                f,
                "{}_{}_{}_ctblock",
                self.raw_padding_bits(),
                self.raw_carry_bits(),
                self.raw_message_bits()
            )
        }
    }
}

impl PartialEq for EmulatedCiphertextBlock {
    fn eq(&self, other: &Self) -> bool {
        self.raw_complete_bits() == other.raw_complete_bits() && self.spec == other.spec
    }
}

impl Eq for EmulatedCiphertextBlock {}

impl PartialOrd for EmulatedCiphertextBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.spec == other.spec {
            self.raw_complete_bits()
                .partial_cmp(&other.raw_complete_bits())
        } else {
            None
        }
    }
}

impl std::hash::Hash for EmulatedCiphertextBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_complete_bits().hash(state);
        self.spec.hash(state);
    }
}

impl Dumpable for EmulatedCiphertextBlock {
    fn dump_to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
