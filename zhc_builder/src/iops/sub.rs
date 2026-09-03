use crate::builder::{Builder, Ciphertext};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

/// Creates an IR for subtraction of two encrypted integers.
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_sub`].
/// See that method for algorithm details.
pub fn sub(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_sub(&src_a, &src_b);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for subtraction with overflow (borrow) detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_sub`]. Returns two outputs:
/// the wrapping difference and a single-block borrow flag. See the builder method for details.
pub fn overflow_sub(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_sub(&src_a, &src_b);
    builder.ciphertext_output(res);
    builder.ciphertext_output(flag);
    builder
}

impl Builder {
    /// Subtracts two encrypted integers, automatically selecting the best algorithm.
    ///
    /// Computes `lhs - rhs` using two's complement: internally `lhs + (!rhs) + 1`.
    /// The algorithm is chosen based on bit-width, matching [`iop_add`](Self::iop_add).
    /// The result is the wrapping difference.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let diff = builder.iop_sub(&a, &b);
    /// ```
    pub fn iop_sub(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        let one = self.block_let_ciphertext(1);
        let b_inv = self.iop_bitwise_inv(&rhs);
        match lhs.spec().int_size() {
            0..8 => self.iop_add_ripple_carry(&lhs, &b_inv, Some(&one)).0,
            8..256 => self.iop_add_hillis_steele(&lhs, &b_inv, Some(&one)).0,
            _ => todo!(),
        }
    }

    /// Subtracts two encrypted integers with overflow (borrow) detection.
    ///
    /// Returns `(difference, overflow)` where `difference` is `lhs - rhs` (wrapping)
    /// and `overflow` is a single-block ciphertext: 1 if `rhs > lhs` (unsigned
    /// underflow occurred), 0 otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (diff, borrow) = builder.iop_overflow_sub(&a, &b);
    /// ```
    pub fn iop_overflow_sub(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> (Ciphertext, Ciphertext) {
        let one = self.block_let_ciphertext(1);
        let b_inv = self.iop_bitwise_inv(&rhs);
        let (res, carry_out) = match lhs.spec().int_size() {
            0..8 => self.iop_add_ripple_carry(&lhs, &b_inv, Some(&one)),
            8..256 => self.iop_add_hillis_steele(&lhs, &b_inv, Some(&one)),
            _ => todo!(),
        };

        // For sub: carry_out=1 means NO overflow (a >= b), carry_out=0 means overflow (a < b).
        let carry_out = self.ciphertext_split(carry_out);
        let overflow_flag = self.block_lookup(&carry_out[0], Lut1Def::IsNull);
        (res, self.ciphertext_join(&[overflow_flag], None))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;

    #[test]
    fn correctness_sub() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.sub(*rhs))])
        }
        for size in (2..128).step_by(2) {
            sub(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_sub() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            let (diff, flag) = lhs.overflow_sub(*rhs);
            Some(vec![IopValue::Ciphertext(diff), IopValue::Ciphertext(flag)])
        }
        for size in (2..128).step_by(2) {
            overflow_sub(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn noise_sub() {
        for size in (2..128).step_by(2) {
            sub(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_overflow_sub() {
        for size in (2..128).step_by(2) {
            overflow_sub(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
