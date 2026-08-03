//! Scalar division IOps, i.e. `ct / imm` and `ct % imm`.
//!
//! Both are the `ct x ct` [`div`](super::div::div) datapath with the divisor lifted out of the plaintext
//! domain by [`Builder::iop_trivial_encrypt`] — one `let_ct<0>` shared by every digit plus one
//! `add_pt` per digit, so a linear DOp per block and **no PBS**.
//!
//! Contrary to [`subs`](super::subs) and [`muls`](super::muls), the scalar operand buys no
//! algorithmic saving here, hence the plain lift:
//!
//! * The IOp language has no plaintext-domain arithmetic — a plaintext can only come from
//!   `LetPlaintextBlock` (a compile-time constant) or `ExtractPtBlock`, and every op consuming one
//!   lands straight in the ciphertext domain. So `2*C`, `3*C` or `!C` cannot be formed from a
//!   *runtime* immediate, which is precisely what the division init phase needs.
//! * A division costs `block_count` iterations of three carry propagations, and a propagation
//!   costs the same whether its second operand is a ciphertext or an immediate. The dominant term
//!   is therefore untouched by a scalar divisor.
//!
//! The legacy reference streams agree: `DIVS` weighs exactly as many PBS as `DIV` (182 + 17
//! many-luts, `MODS` and `MOD` 174 + 17) and differs only by a score of linear DOps — see
//! `zhc_sim/src/hpu/test/streams/DIVS.rs`.
//!
//! A division by zero behaves as in [`div`](super::div): the quotient is forced to zero, the
//! remainder is unspecified. The immediate is in the clear on the host side, which can reject or
//! shortcut that case before submitting the IOp.

use zhc_crypto::integer_semantics::CiphertextSpec;

use crate::builder::{Builder, Ciphertext, Plaintext};

/// Creates an IR for the unsigned division of an encrypted integer by a scalar (`ct / imm`).
///
/// Convenience wrapper that calls [`Builder::iop_divsx`] and outputs both quotient and remainder,
/// as `div` does. See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, divs};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = divs(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn divs(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let (quotient, remainder) = builder.iop_divsx(&src_c, &src_p);
    builder.ciphertext_output(quotient);
    builder.ciphertext_output(remainder);
    builder
}

/// Creates an IR for the unsigned remainder of an encrypted integer by a scalar (`ct % imm`).
///
/// Convenience wrapper that calls [`Builder::iop_mods`], as `rem` does. See that method for
/// details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, mods};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = mods(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn mods(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let remainder = builder.iop_mods(&src_c, &src_p);
    builder.ciphertext_output(remainder);
    builder
}

impl Builder {
    /// Computes both quotient and remainder of an encrypted integer by a scalar.
    ///
    /// Returns `(quotient, remainder)` for the unsigned division `lhs / rhs`. This is the scalar
    /// entry point of the division datapath; use [`iop_divs`](Self::iop_divs) or
    /// [`iop_mods`](Self::iop_mods) if only one result is needed (dead-code elimination removes
    /// the unused output). Division by zero produces an unspecified result without trapping.
    ///
    /// # Panics
    ///
    /// Panics if the operands do not share the same integer width and block count — the division
    /// core is driven by block counts alone, so a mismatch would silently extend the shorter
    /// operand instead of failing.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let (quotient, remainder) = builder.iop_divsx(&a, &b);
    /// ```
    pub fn iop_divsx(&self, lhs: &Ciphertext, rhs: &Plaintext) -> (Ciphertext, Ciphertext) {
        assert_eq!(
            lhs.spec().int_size(),
            rhs.spec().int_size(),
            "Spec mismatch."
        );
        assert_eq!(
            lhs.spec().block_count(),
            rhs.spec().block_count(),
            "Spec mismatch."
        );
        // The lift is a linear DOp per digit and no PBS, so `DIVS` costs what `DIV` costs.
        let divisor = self.comment("Lift Divisor").iop_trivial_encrypt(rhs);
        self.iop_divx(lhs, &divisor)
    }

    /// Computes the unsigned quotient of an encrypted integer by a scalar.
    ///
    /// Returns `lhs / rhs` (integer division). Division by zero produces a zero quotient without
    /// trapping, as [`iop_div`](Self::iop_div) does. Internally delegates to
    /// [`iop_divsx`](Self::iop_divsx) and discards the remainder.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let quotient = builder.iop_divs(&a, &b);
    /// ```
    pub fn iop_divs(&self, lhs: &Ciphertext, rhs: &Plaintext) -> Ciphertext {
        self.iop_divsx(lhs, rhs).0
    }

    /// Computes the unsigned remainder of an encrypted integer by a scalar.
    ///
    /// Returns `lhs % rhs` (Euclidean remainder). Remainder by zero produces an unspecified result
    /// without trapping, as [`iop_rem`](Self::iop_rem) does. Internally delegates to
    /// [`iop_divsx`](Self::iop_divsx) and discards the quotient.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let remainder = builder.iop_mods(&a, &b);
    /// ```
    pub fn iop_mods(&self, lhs: &Ciphertext, rhs: &Plaintext) -> Ciphertext {
        self.iop_divsx(lhs, rhs).1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;

    #[test]
    fn correctness_divs() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            // A null divisor leaves the remainder unspecified: skip the draw.
            if rhs.as_storage() == 0 {
                return None;
            }
            let quotient = lhs.as_storage().div_euclid(rhs.as_storage());
            let remainder = lhs.as_storage().rem_euclid(rhs.as_storage());
            Some(vec![
                IopValue::Ciphertext(lhs.spec().from_int(quotient)),
                IopValue::Ciphertext(lhs.spec().from_int(remainder)),
            ])
        }
        for size in (2..128).step_by(2) {
            divs(CiphertextSpec::new(size, 2, 2)).test_random(10, semantic);
        }
        for size in [16, 32, 64, 128] {
            divs(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    /// `mods` keeps a single output, so nothing else checks that it really is the *remainder*:
    /// the specialization is pure dead-code elimination on top of `iop_divsx`.
    #[test]
    fn correctness_mods() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            if rhs.as_storage() == 0 {
                return None;
            }
            let remainder = lhs.as_storage().rem_euclid(rhs.as_storage());
            Some(vec![IopValue::Ciphertext(lhs.spec().from_int(remainder))])
        }
        for size in (2..128).step_by(2) {
            mods(CiphertextSpec::new(size, 2, 2)).test_random(10, semantic);
        }
        for size in [16, 32, 64, 128] {
            mods(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }
}
