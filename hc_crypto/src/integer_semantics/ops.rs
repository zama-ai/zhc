use std::ops::{Add, Mul, Shl, Shr, Sub};

use super::{CiphertextBlock, PlaintextBlock};

impl CiphertextBlock {
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
    pub fn protect_add_pt(self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-add, but lhs has active padding bit."
        );
        let storage = self.raw_complete_bits().add(rhs.raw_message_bits());
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
    pub fn temper_add_pt(self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self.raw_complete_bits().add(rhs.raw_message_bits());
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
    pub fn wrapping_add_pt(self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_add(rhs.raw_message_bits())
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
    pub fn protect_sub_pt(self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-sub, but lhs has active padding bit."
        );
        assert!(
            self.raw_complete_bits() >= rhs.raw_message_bits(),
            "Underflow occured while performing protect-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_message_bits());
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts a plaintext block from a ciphertext block while preventing underflow.
    pub fn temper_sub_pt(self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_complete_bits() >= rhs.raw_message_bits(),
            "Underflow occured while performing temper-sub."
        );
        let storage = self.raw_complete_bits().sub(rhs.raw_message_bits());
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Subtracts a plaintext block from a ciphertext block with underflow wrapping.
    pub fn wrapping_sub_pt(self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_sub(rhs.raw_message_bits())
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

    /// Shifts a ciphertext block left with overflow wrapping.
    pub fn wrapping_shl(&self, rhs: u8) -> Self {
        let storage = self.raw_complete_bits().shl(rhs) & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Shifts a ciphertext block right while protecting the padding bit from writes.
    pub fn protect_shr(&self, rhs: u8) -> Self {
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-shr, but lhs has active padding bit."
        );
        let storage = self.raw_complete_bits().shr(rhs);
        assert!(
            !self.spec.overflows_carry(storage),
            "Overflow occured while performing protect-shr."
        );
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Shifts a ciphertext block right with underflow wrapping.
    pub fn wrapping_shr(&self, rhs: u8) -> Self {
        let storage = self.raw_complete_bits().shr(rhs);
        Self {
            storage,
            spec: self.spec,
        }
    }

    /// Multiplies a ciphertext block by a plaintext block while protecting the padding bit from
    /// writes.
    pub fn protect_mul_pt(&self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            self.raw_padding_bits() == 0,
            "Tried to protect-mul, but lhs has active padding bit."
        );
        let storage = self.raw_complete_bits().mul(rhs.raw_message_bits());
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
    pub fn temper_mul_pt(&self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self.raw_complete_bits().mul(rhs.raw_message_bits());
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
    pub fn wrapping_mul(&self, rhs: PlaintextBlock) -> Self {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_complete_bits()
            .wrapping_mul(rhs.raw_message_bits())
            & self.spec.complete_mask();
        Self {
            storage,
            spec: self.spec,
        }
    }
}
impl PlaintextBlock {
    /// Subtracts a ciphertext block from this plaintext block while protecting the padding bit.
    pub fn protect_sub_ct(self, rhs: CiphertextBlock) -> CiphertextBlock {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        assert!(
            rhs.raw_padding_bits() == 0,
            "Tried to protect-sub, but rhs has active padding bit."
        );
        assert!(
            self.raw_message_bits() >= rhs.raw_complete_bits(),
            "Underflow occured while performing protect-sub."
        );
        let storage = self.raw_message_bits().sub(rhs.raw_complete_bits());
        CiphertextBlock {
            storage,
            spec: rhs.spec,
        }
    }

    /// Subtracts a ciphertext block from this plaintext block with underflow wrapping.
    pub fn wrapping_sub_ct(self, rhs: CiphertextBlock) -> CiphertextBlock {
        assert_eq!(self.spec, rhs.spec, "Spec mismatch.");
        let storage = self
            .raw_message_bits()
            .wrapping_sub(rhs.raw_complete_bits())
            & rhs.spec.complete_mask();
        CiphertextBlock {
            storage,
            spec: rhs.spec,
        }
    }
}
