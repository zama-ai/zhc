use std::ops::{Add, Mul, Shl, Sub};

use zhc_utils::SafeAs;

use crate::integer_semantics::{
    EmulatedCiphertextBool, EmulatedCiphertextInteger, EmulatedPlaintextInteger,
};

use super::{
    EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage, EmulatedPlaintextBlock, Flavor,
};

/// Flavor-dispatching operations.
///
/// Each method selects the `protect_*`, `temper_*` or `wrapping_*` implementation matching the
/// given [`Flavor`]. See the [module documentation](super) for the semantics of each flavor.
impl EmulatedCiphertextBlock {
    /// Adds two ciphertext blocks with the given flavor.
    pub fn add(self, rhs: Self, flavor: Flavor) -> Self {
        match flavor {
            Flavor::Protect => self.protect_add(rhs),
            Flavor::Temper => self.temper_add(rhs),
            Flavor::Wrapping => self.wrapping_add(rhs),
        }
    }

    /// Subtracts two ciphertext blocks with the given flavor.
    pub fn sub(self, rhs: Self, flavor: Flavor) -> Self {
        match flavor {
            Flavor::Protect => self.protect_sub(rhs),
            Flavor::Temper => self.temper_sub(rhs),
            Flavor::Wrapping => self.wrapping_sub(rhs),
        }
    }

    /// Adds a plaintext block to a ciphertext block with the given flavor.
    pub fn add_pt(self, rhs: EmulatedPlaintextBlock, flavor: Flavor) -> Self {
        match flavor {
            Flavor::Protect => self.protect_add_pt(rhs),
            Flavor::Temper => self.temper_add_pt(rhs),
            Flavor::Wrapping => self.wrapping_add_pt(rhs),
        }
    }

    /// Subtracts a plaintext block from a ciphertext block with the given flavor.
    pub fn sub_pt(self, rhs: EmulatedPlaintextBlock, flavor: Flavor) -> Self {
        match flavor {
            Flavor::Protect => self.protect_sub_pt(rhs),
            Flavor::Temper => self.temper_sub_pt(rhs),
            Flavor::Wrapping => self.wrapping_sub_pt(rhs),
        }
    }

    /// Multiplies a ciphertext block by a plaintext block with the given flavor.
    pub fn mul_pt(self, rhs: EmulatedPlaintextBlock, flavor: Flavor) -> Self {
        match flavor {
            Flavor::Protect => self.protect_mul_pt(rhs),
            Flavor::Temper => self.temper_mul_pt(rhs),
            Flavor::Wrapping => self.wrapping_mul_pt(rhs),
        }
    }

    /// Shifts a ciphertext block left by `amount` bits with the given flavor.
    pub fn shl(self, amount: u8, flavor: Flavor) -> Self {
        match flavor {
            Flavor::Protect => self.protect_shl(amount),
            Flavor::Temper => self.temper_shl(amount),
            Flavor::Wrapping => self.wrapping_shl(amount),
        }
    }

    /// Computes `self * mul + rhs` (multiply-accumulate) with the given flavor.
    ///
    /// With `mul = 2^message_size` this packs `self` in the carry region and `rhs` in the
    /// message region of a single block.
    pub fn mac(self, mul: u8, rhs: Self, flavor: Flavor) -> Self {
        match flavor {
            Flavor::Protect => self.protect_mac(mul, rhs),
            Flavor::Temper => self.temper_mac(mul, rhs),
            Flavor::Wrapping => self.wrapping_mac(mul, rhs),
        }
    }
}

impl EmulatedPlaintextBlock {
    /// Subtracts a ciphertext block from this plaintext block with the given flavor.
    pub fn sub_ct(self, rhs: EmulatedCiphertextBlock, flavor: Flavor) -> EmulatedCiphertextBlock {
        match flavor {
            Flavor::Protect => self.protect_sub_ct(rhs),
            Flavor::Temper => self.temper_sub_ct(rhs),
            Flavor::Wrapping => self.wrapping_sub_ct(rhs),
        }
    }
}

impl EmulatedCiphertextBlock {
    /// Adds two ciphertext blocks while protecting the padding bit from writes.
    pub fn protect_add(self, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-add, but lhs has active padding bit."
        );
        assert!(
            rhs.raw_padding_bits() == 0,
            "Tried to protect-add, but rhs has active padding bit."
        );
        let storage = self.raw_complete_bits().add(rhs.raw_complete_bits());
        assert!(
            !self.spec.overflows_carry(storage),
            "Overflow occured while performing protect-add."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Adds two ciphertext blocks while preventing padding bit overflow.
    pub fn temper_add(self, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self.raw_complete_bits().add(rhs.raw_complete_bits());
        assert!(
            !self.spec.overflows_padding(storage),
            "Overflow occured while performing temper-add."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Adds two ciphertext blocks with modular arithmetic and overflow wrapping.
    pub fn wrapping_add(self, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_add(rhs.raw_complete_bits())
            & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Adds a plaintext block to a ciphertext block while protecting the padding bit.
    pub fn protect_add_pt(self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-add, but lhs has active padding bit."
        );
        let storage = self.raw_complete_bits().add(rhs.raw_complete_bits());
        assert!(
            !self.spec.overflows_carry(storage),
            "Overflow occured while performing protect-add."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Adds a plaintext block to a ciphertext block while preventing padding overflow.
    pub fn temper_add_pt(self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self.raw_complete_bits().add(rhs.raw_complete_bits());
        assert!(
            !self.spec.overflows_padding(storage),
            "Overflow occured while performing temper-add."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Adds a plaintext block to a ciphertext block with overflow wrapping.
    pub fn wrapping_add_pt(self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_add(rhs.raw_complete_bits())
            & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts two ciphertext blocks while protecting the padding bit from writes.
    pub fn protect_sub(self, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-sub, but lhs has active padding bit."
        );
        assert!(
            rhs.raw_padding_bits() == 0,
            "Tried to protect-sub, but rhs has active padding bit."
        );
        assert!(
            self.raw_complete_bits() >= rhs.raw_complete_bits(),
            "Underflow occured while performing protect-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_complete_bits());
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts two ciphertext blocks while preventing underflow.
    pub fn temper_sub(self, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_complete_bits() >= rhs.raw_complete_bits(),
            "Underflow occured while performing temper-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_complete_bits());
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts two ciphertext blocks with modular arithmetic and underflow wrapping.
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_sub(rhs.raw_complete_bits())
            & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts a plaintext block from a ciphertext block while protecting the padding bit.
    pub fn protect_sub_pt(self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-sub, but lhs has active padding bit."
        );
        assert!(
            self.raw_complete_bits() >= rhs.raw_complete_bits(),
            "Underflow occured while performing protect-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_complete_bits());
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts a plaintext block from a ciphertext block while preventing underflow.
    pub fn temper_sub_pt(self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_complete_bits() >= rhs.raw_complete_bits(),
            "Underflow occured while performing temper-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_complete_bits());
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts a plaintext block from a ciphertext block with underflow wrapping.
    pub fn wrapping_sub_pt(self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_sub(rhs.raw_complete_bits())
            & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Shifts a ciphertext block left while protecting the padding bit from writes.
    pub fn protect_shl(&self, rhs: u8) -> Self {
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-shl, but lhs has active padding bit."
        );
        let storage = self.raw_complete_bits().shl(rhs);
        assert!(
            !self.spec.overflows_carry(storage),
            "Overflow occured while performing protect-shl."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    pub fn protect_shr(&self, rhs: u8) -> Self {
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-shr, but lhs has active padding bit."
        );
        Self {
            storage: self.raw_complete_bits() >> rhs,
            spec: self.spec,
        }
    }

    /// Shifts a ciphertext block left while preventing padding bit overflow.
    pub fn temper_shl(&self, rhs: u8) -> Self {
        let storage = self.raw_complete_bits().shl(rhs);
        assert!(
            !self.spec.overflows_padding(storage),
            "Overflow occured while performing temper-shl."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Shifts a ciphertext block left with overflow wrapping.
    pub fn wrapping_shl(&self, rhs: u8) -> Self {
        let storage = self.raw_complete_bits().shl(rhs) & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Multiplies a ciphertext block by a plaintext block while protecting the padding bit from
    /// writes.
    pub fn protect_mul_pt(&self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-mul, but lhs has active padding bit."
        );
        let storage = self.raw_complete_bits().mul(rhs.raw_complete_bits());
        assert!(
            !self.spec.overflows_carry(storage),
            "Overflow occured while performing protect-mul."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Multiplies a ciphertext block by a plaintext block while preventing padding bit overflow.
    pub fn temper_mul_pt(&self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self.raw_complete_bits().mul(rhs.raw_complete_bits());
        assert!(
            !self.spec.overflows_padding(storage),
            "Overflow occured while performing temper-mul."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Multiplies a ciphertext block by a plaintext block with modular arithmetic and overflow
    /// wrapping.
    pub fn wrapping_mul_pt(&self, rhs: EmulatedPlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_mul(rhs.raw_complete_bits())
            & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Computes `self * mul + rhs` while protecting the padding bit from writes.
    pub fn protect_mac(self, mul: u8, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-mac, but lhs has active padding bit."
        );
        assert!(
            rhs.raw_padding_bits() == 0,
            "Tried to protect-mac, but rhs has active padding bit."
        );
        let storage = self.raw_complete_bits() * mul.sas::<EmulatedCiphertextBlockStorage>()
            + rhs.raw_complete_bits();
        assert!(
            !self.spec.overflows_carry(storage),
            "Overflow occured while performing protect-mac."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Computes `self * mul + rhs` while preventing padding bit overflow.
    pub fn temper_mac(self, mul: u8, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self.raw_complete_bits() * mul.sas::<EmulatedCiphertextBlockStorage>()
            + rhs.raw_complete_bits();
        assert!(
            !self.spec.overflows_padding(storage),
            "Overflow occured while performing temper-mac."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Computes `self * mul + rhs` with modular arithmetic and overflow wrapping.
    pub fn wrapping_mac(self, mul: u8, rhs: Self) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_mul(mul.sas::<EmulatedCiphertextBlockStorage>())
            .wrapping_add(rhs.raw_complete_bits())
            & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }
}

impl EmulatedPlaintextBlock {
    /// Subtracts a ciphertext block from this plaintext block while protecting the padding bit.
    pub fn protect_sub_ct(self, rhs: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            rhs.raw_padding_bits() == 0,
            "Tried to protect-sub, but rhs has active padding bit."
        );
        assert!(
            self.raw_complete_bits() >= rhs.raw_complete_bits(),
            "Underflow occured while performing protect-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_complete_bits());
        EmulatedCiphertextBlock {
            storage,
            spec: rhs.spec,
        }
    }

    /// Subtracts a ciphertext block from this plaintext block while preventing underflow.
    pub fn temper_sub_ct(self, rhs: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_complete_bits() >= rhs.raw_complete_bits(),
            "Underflow occured while performing temper-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_complete_bits());
        EmulatedCiphertextBlock {
            storage,
            spec: rhs.spec,
        }
    }

    /// Subtracts a ciphertext block from this plaintext block with underflow wrapping.
    pub fn wrapping_sub_ct(self, rhs: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_sub(rhs.raw_complete_bits())
            & rhs.spec.complete_mask();
        EmulatedCiphertextBlock {
            storage,
            spec: rhs.spec,
        }
    }
}

impl EmulatedCiphertextInteger {
    pub fn cgt(self, other: Self) -> EmulatedCiphertextBool {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        EmulatedCiphertextBool::from_bool(self.storage > other.storage, self.spec.block_spec())
    }

    pub fn cgte(self, other: Self) -> EmulatedCiphertextBool {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        EmulatedCiphertextBool::from_bool(self.storage >= other.storage, self.spec.block_spec())
    }

    pub fn clt(self, other: Self) -> EmulatedCiphertextBool {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        EmulatedCiphertextBool::from_bool(self.storage < other.storage, self.spec.block_spec())
    }

    pub fn clte(self, other: Self) -> EmulatedCiphertextBool {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        EmulatedCiphertextBool::from_bool(self.storage <= other.storage, self.spec.block_spec())
    }

    pub fn equal(self, other: Self) -> EmulatedCiphertextBool {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        EmulatedCiphertextBool::from_bool(self.storage == other.storage, self.spec.block_spec())
    }

    pub fn not_equal(self, other: Self) -> EmulatedCiphertextBool {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        EmulatedCiphertextBool::from_bool(self.storage != other.storage, self.spec.block_spec())
    }

    pub fn add(self, other: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = (self.storage + other.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn adds(self, other: EmulatedPlaintextInteger) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = (self.storage + other.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts a scalar from an encrypted integer: `self - other` (wrapping).
    pub fn subs(self, other: EmulatedPlaintextInteger) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = self.storage.wrapping_sub(other.as_storage()) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts an encrypted integer from a scalar: `other - self` (wrapping).
    pub fn ssub(self, other: EmulatedPlaintextInteger) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = other.as_storage().wrapping_sub(self.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn sub(self, other: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = self.storage.wrapping_sub(other.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn bitwise_and(self, other: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = (self.storage & other.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn bitwise_or(self, other: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = (self.storage | other.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn bitwise_xor(self, other: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = (self.storage ^ other.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn bitwise_not(self) -> EmulatedCiphertextInteger {
        let storage = !self.storage & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn shift_right(self, amount: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, amount.spec(), "Spec mismatch.");
        let w = self.spec.int_size() as u128;
        let n = amount.storage as u128;
        let storage = if n >= w {
            0
        } else {
            (self.storage >> n) & self.spec.int_mask()
        };
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn shift_left(self, amount: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, amount.spec(), "Spec mismatch.");
        let w = self.spec.int_size() as u128;
        let n = amount.storage as u128;
        let storage = if n >= w {
            0
        } else {
            (self.storage << n) & self.spec.int_mask()
        };
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn rotate_right(self, amount: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, amount.spec(), "Spec mismatch.");
        let w = self.spec.int_size() as u32;
        let n = (amount.storage as u32) % w;
        let storage = if n == 0 {
            self.storage
        } else {
            ((self.storage >> n) | (self.storage << (w - n))) & self.spec.int_mask()
        };
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn rotate_left(self, amount: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, amount.spec(), "Spec mismatch.");
        let w = self.spec.int_size() as u32;
        let n = (amount.storage as u32) % w;
        let storage = if n == 0 {
            self.storage
        } else {
            ((self.storage << n) | (self.storage >> (w - n))) & self.spec.int_mask()
        };
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    pub fn overflow_add(self, other: Self) -> (EmulatedCiphertextInteger, EmulatedCiphertextBool) {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let full = (self.storage as u128) + (other.storage as u128);
        let sum = full & self.spec.int_mask();
        let overflow: u128 = if full > self.spec.int_mask() { 1 } else { 0 };
        (
            EmulatedCiphertextInteger {
                storage: sum,
                spec: self.spec,
            },
            EmulatedCiphertextBool::from_bool(overflow != 0, self.spec.block_spec()),
        )
    }

    /// `self + other` with unsigned-overflow (carry) detection, `other` being a scalar.
    pub fn overflow_adds(
        self,
        other: EmulatedPlaintextInteger,
    ) -> (EmulatedCiphertextInteger, EmulatedCiphertextBool) {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let (raw_sum, wrapped) = self.storage.overflowing_add(other.as_storage());
        let overflow: u128 = (wrapped || raw_sum > self.spec.int_mask()).sas();
        (
            EmulatedCiphertextInteger {
                storage: raw_sum & self.spec.int_mask(),
                spec: self.spec,
            },
            EmulatedCiphertextBool::from_bool(overflow != 0, self.spec.block_spec()),
        )
    }

    /// `self - other` with unsigned-underflow (borrow) detection, `other` being a scalar.
    pub fn overflow_subs(
        self,
        other: EmulatedPlaintextInteger,
    ) -> (EmulatedCiphertextInteger, EmulatedCiphertextBool) {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let rhs = other.as_storage();
        let diff = self.storage.wrapping_sub(rhs) & self.spec.int_mask();
        let overflow: u128 = if rhs > self.storage { 1 } else { 0 };
        (
            EmulatedCiphertextInteger {
                storage: diff,
                spec: self.spec,
            },
            EmulatedCiphertextBool::from_bool(overflow != 0, self.spec.block_spec()),
        )
    }

    /// `other - self` with unsigned-underflow (borrow) detection, `other` being a scalar.
    pub fn overflow_ssub(
        self,
        other: EmulatedPlaintextInteger,
    ) -> (EmulatedCiphertextInteger, EmulatedCiphertextBool) {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let lhs = other.as_storage();
        let diff = lhs.wrapping_sub(self.storage) & self.spec.int_mask();
        let overflow: u128 = if self.storage > lhs { 1 } else { 0 };
        (
            EmulatedCiphertextInteger {
                storage: diff,
                spec: self.spec,
            },
            EmulatedCiphertextBool::from_bool(overflow != 0, self.spec.block_spec()),
        )
    }

    pub fn overflow_sub(self, other: Self) -> (EmulatedCiphertextInteger, EmulatedCiphertextBool) {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let diff = self.storage.wrapping_sub(other.storage) & self.spec.int_mask();
        let overflow: u128 = if other.storage > self.storage { 1 } else { 0 };
        (
            EmulatedCiphertextInteger {
                storage: diff,
                spec: self.spec,
            },
            EmulatedCiphertextBool::from_bool(overflow != 0, self.spec.block_spec()),
        )
    }

    /// Describe multiplication behavior when MSB are dropped
    pub fn mul_lsb(self, other: Self) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = self.storage.wrapping_mul(other.storage) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }
    /// Describe scalar multiplication behavior when MSB are dropped
    pub fn muls_lsb(self, other: EmulatedPlaintextInteger) -> EmulatedCiphertextInteger {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let storage = self.storage.wrapping_mul(other.as_storage()) & self.spec.int_mask();
        EmulatedCiphertextInteger {
            storage,
            spec: self.spec,
        }
    }

    /// Describe scalar multiplication behavior with overflow detection when MSB are dropped
    pub fn overflow_muls_lsb(
        self,
        other: EmulatedPlaintextInteger,
    ) -> (EmulatedCiphertextInteger, EmulatedCiphertextBool) {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let (raw_mul, overflow_u128) = self.storage.overflowing_mul(other.as_storage());
        let int_size = self.spec.int_size();
        let overflow_flag = overflow_u128 || (int_size < 128 && (raw_mul >> int_size) != 0);
        (
            EmulatedCiphertextInteger {
                storage: raw_mul & self.spec.int_mask(),
                spec: self.spec,
            },
            EmulatedCiphertextBool::from_bool(overflow_flag, self.spec.block_spec()),
        )
    }

    /// Describe multiplication behavior with overflow detection when MSB are dropped
    pub fn overflow_mul_lsb(
        self,
        other: Self,
    ) -> (EmulatedCiphertextInteger, EmulatedCiphertextBool) {
        assert_eq!(self.spec, other.spec(), "Spec mismatch.");
        let (raw_mul, overflow_u128) = self.storage.overflowing_mul(other.storage);
        let mul_lsb = raw_mul & self.spec.int_mask();
        let int_size = self.spec.int_size();
        let overflow_flag = overflow_u128 || (int_size < 128 && (raw_mul >> int_size) != 0);
        (
            EmulatedCiphertextInteger {
                storage: mul_lsb,
                spec: self.spec,
            },
            EmulatedCiphertextBool::from_bool(overflow_flag, self.spec.block_spec()),
        )
    }
}
