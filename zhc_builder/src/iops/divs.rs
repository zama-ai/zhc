use zhc_crypto::integer_semantics::IntegerCiphertextSpec;

use crate::builder::{Builder, IntegerCiphertext, IntegerPlaintext};

/// Creates an IR for the unsigned division of an encrypted integer by a scalar (`ct / imm`).
///
/// Convenience wrapper that calls [`Builder::iop_divsx`] and outputs both quotient and remainder,
/// as `div` does. See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{IntegerCiphertextSpec, divs};
/// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
/// let builder = divs(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn divs(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let (quotient, remainder) = builder.iop_divsx(&src_c, &src_p);
    builder.integer_ciphertext_output(quotient);
    builder.integer_ciphertext_output(remainder);
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
/// # use zhc_builder::{IntegerCiphertextSpec, mods};
/// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
/// let builder = mods(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn mods(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let remainder = builder.iop_mods(&src_c, &src_p);
    builder.integer_ciphertext_output(remainder);
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
    /// The divisor is lifted to the ciphertext domain with
    /// [`iop_trivial_encrypt`](Self::iop_trivial_encrypt), which is one linear DOp per digit and no
    /// PBS, then the `ct x ct` [`iop_divx`](Self::iop_divx) does the work. A scalar divisor saves
    /// nothing here: the cost of a division is its carry propagations, which cost the same with an
    /// immediate as with a ciphertext.
    ///
    /// # Panics
    ///
    /// Panics if the operands do not share the same integer width and block count. The division
    /// core only looks at block counts, so a mismatch would silently extend the shorter operand
    /// instead of failing.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// let (quotient, remainder) = builder.iop_divsx(&a, &b);
    /// ```
    pub fn iop_divsx(
        &self,
        lhs: &IntegerCiphertext,
        rhs: &IntegerPlaintext,
    ) -> (IntegerCiphertext, IntegerCiphertext) {
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
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// let quotient = builder.iop_divs(&a, &b);
    /// ```
    pub fn iop_divs(&self, lhs: &IntegerCiphertext, rhs: &IntegerPlaintext) -> IntegerCiphertext {
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
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// let remainder = builder.iop_mods(&a, &b);
    /// ```
    pub fn iop_mods(&self, lhs: &IntegerCiphertext, rhs: &IntegerPlaintext) -> IntegerCiphertext {
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
            let [IopValue::IntegerCiphertext(lhs), IopValue::IntegerPlaintext(rhs)] = inp else {
                unreachable!()
            };
            // A null divisor leaves the remainder unspecified: skip the draw.
            if rhs.as_storage() == 0 {
                return None;
            }
            let quotient = lhs.as_storage().div_euclid(rhs.as_storage());
            let remainder = lhs.as_storage().rem_euclid(rhs.as_storage());
            Some(vec![
                IopValue::IntegerCiphertext(lhs.spec().from_int(quotient)),
                IopValue::IntegerCiphertext(lhs.spec().from_int(remainder)),
            ])
        }
        for size in (2..128).step_by(2) {
            divs(IntegerCiphertextSpec::new(size, 2, 2)).test_random(10, semantic);
        }
        for size in [16, 32, 64, 128] {
            divs(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    // `mods` keeps one output only, so nothing else checks that it really is the remainder.
    #[test]
    fn correctness_mods() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::IntegerCiphertext(lhs), IopValue::IntegerPlaintext(rhs)] = inp else {
                unreachable!()
            };
            if rhs.as_storage() == 0 {
                return None;
            }
            let remainder = lhs.as_storage().rem_euclid(rhs.as_storage());
            Some(vec![IopValue::IntegerCiphertext(lhs.spec().from_int(remainder))])
        }
        for size in (2..128).step_by(2) {
            mods(IntegerCiphertextSpec::new(size, 2, 2)).test_random(10, semantic);
        }
        for size in [16, 32, 64, 128] {
            mods(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn noise_divs() {
        for size in (2..128).step_by(2) {
            divs(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_mods() {
        for size in (2..128).step_by(2) {
            mods(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
