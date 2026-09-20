use crate::builder::{BoolCiphertext, Builder, IntegerCiphertext, IntegerPlaintext};
use zhc_crypto::integer_semantics::IntegerCiphertextSpec;
use zhc_langs::ioplang::Lut1;

/// Creates an IR for the subtraction of a scalar from an encrypted integer (`ct - imm`).
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_subs`].
/// See that method for algorithm details.
pub fn subs(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let res = builder.iop_subs(&src_c, &src_p);
    builder.integer_ciphertext_output(res);
    builder
}

/// Creates an IR for the subtraction of an encrypted integer from a scalar (`imm - ct`).
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_ssub`].
/// See that method for algorithm details.
///
/// The ciphertext input is declared first so that the operand slots match the ones used by
/// [`adds`](super::adds), independently of the reversed mathematical operand order.
pub fn ssub(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let res = builder.iop_ssub(&src_p, &src_c);
    builder.integer_ciphertext_output(res);
    builder
}

/// Creates an IR for `ct - imm` with overflow (borrow) detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_subs`]. Returns two outputs:
/// the wrapping difference and a single-block borrow flag.
pub fn overflow_subs(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_subs(&src_c, &src_p);
    builder.integer_ciphertext_output(res);
    builder.bool_ciphertext_output(flag);
    builder
}

/// Creates an IR for `imm - ct` with overflow (borrow) detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_ssub`]. Returns two outputs:
/// the wrapping difference and a single-block borrow flag.
pub fn overflow_ssub(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_ssub(&src_p, &src_c);
    builder.integer_ciphertext_output(res);
    builder.bool_ciphertext_output(flag);
    builder
}

impl Builder {
    /// Subtracts a scalar from an encrypted integer, automatically selecting the best algorithm.
    ///
    /// Computes `lhs - rhs` as `!(!lhs + rhs)`. The carry propagation is done by
    /// [`iop_adds`](Self::iop_adds), so the PBS count is the one of a scalar addition. The two
    /// inversions are plaintext minus ciphertext operations and need no PBS.
    ///
    /// It costs one block_plaintext_sub per block before and after the addition. In TFHE-rs we
    /// invert the constant at runtime instead, which ZHC cannot do yet. This will get better when
    /// ucore gets DOp instructions to work on an immediate on its own.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// let diff = builder.iop_subs(&a, &b);
    /// ```
    pub fn iop_subs(&self, lhs: &IntegerCiphertext, rhs: &IntegerPlaintext) -> IntegerCiphertext {
        let a_inv = self.comment("Invert Input").iop_bitwise_inv(lhs);
        let sum = self.iop_adds(&a_inv, rhs);
        self.comment("Invert Output").iop_bitwise_inv(&sum)
    }

    /// Subtracts an encrypted integer from a scalar, automatically selecting the best algorithm.
    ///
    /// Computes `lhs - rhs` as `!rhs + lhs + 1`, i.e. a scalar addition
    /// ([`iop_adds`](Self::iop_adds)) on the inverted ciphertext with a carry-in of 1. The
    /// inversion needs no PBS, so the cost is the one of a scalar addition. The result is the
    /// wrapping difference.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_plaintext_input(spec.int_size());
    /// # let b = builder.integer_ciphertext_input(spec.int_size());
    /// let diff = builder.iop_ssub(&a, &b);
    /// ```
    pub fn iop_ssub(&self, lhs: &IntegerPlaintext, rhs: &IntegerCiphertext) -> IntegerCiphertext {
        let int_size = rhs.spec().int_size();
        let one = self.block_let_ciphertext(1);
        let b_inv = self.comment("Invert Input").iop_bitwise_inv(rhs);
        match int_size {
            0..8 => self.iop_adds_ripple_carry(&b_inv, lhs, Some(&one)).0,
            8..256 => self.iop_adds_hillis_steele(&b_inv, lhs, Some(&one)).0,
            _ => todo!(),
        }
    }

    /// Subtracts a scalar from an encrypted integer with overflow (borrow) detection.
    ///
    /// Returns `(difference, overflow)` where `difference` is `lhs - rhs` (wrapping) and
    /// `overflow` is a single-block ciphertext: 1 if `rhs > lhs` (unsigned underflow
    /// occurred), 0 otherwise.
    ///
    /// The sum done inside is `!lhs + rhs`, whose carry-out is set exactly when `rhs > lhs`. It is
    /// already the borrow, so contrary to [`iop_overflow_sub`](Self::iop_overflow_sub) the flag
    /// needs no inversion PBS.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// let (diff, borrow) = builder.iop_overflow_subs(&a, &b);
    /// ```
    pub fn iop_overflow_subs(
        &self,
        lhs: &IntegerCiphertext,
        rhs: &IntegerPlaintext,
    ) -> (IntegerCiphertext, BoolCiphertext) {
        let int_size = lhs.spec().int_size();
        let a_inv = self.comment("Invert Input").iop_bitwise_inv(lhs);
        let (sum, carry_out) = match int_size {
            0..8 => self.iop_adds_ripple_carry(&a_inv, rhs, None),
            8..256 => self.iop_adds_hillis_steele(&a_inv, rhs, None),
            _ => todo!(),
        };
        (
            self.comment("Invert Output").iop_bitwise_inv(&sum),
            carry_out,
        )
    }

    /// Subtracts an encrypted integer from a scalar with overflow (borrow) detection.
    ///
    /// Returns `(difference, overflow)` where `difference` is `lhs - rhs` (wrapping) and
    /// `overflow` is a single-block ciphertext: 1 if `rhs > lhs` (unsigned underflow
    /// occurred), 0 otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_plaintext_input(spec.int_size());
    /// # let b = builder.integer_ciphertext_input(spec.int_size());
    /// let (diff, borrow) = builder.iop_overflow_ssub(&a, &b);
    /// ```
    pub fn iop_overflow_ssub(
        &self,
        lhs: &IntegerPlaintext,
        rhs: &IntegerCiphertext,
    ) -> (IntegerCiphertext, BoolCiphertext) {
        let int_size = rhs.spec().int_size();
        let one = self.block_let_ciphertext(1);
        let b_inv = self.comment("Invert Input").iop_bitwise_inv(rhs);
        let (res, carry_out) = match int_size {
            0..8 => self.iop_adds_ripple_carry(&b_inv, lhs, Some(&one)),
            8..256 => self.iop_adds_hillis_steele(&b_inv, lhs, Some(&one)),
            _ => todo!(),
        };

        // carry_out=1 means NO overflow (lhs >= rhs), carry_out=0 means overflow (lhs < rhs).
        let carry_out = self.bool_ciphertext_get_block(carry_out);
        let overflow_flag = self.block_lookup(&carry_out, Lut1::is_null(*self.spec()));
        (res, self.bool_ciphertext_from_block(overflow_flag))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;

    #[test]
    fn correctness_subs() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::IntegerCiphertext(lhs), IopValue::IntegerPlaintext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::IntegerCiphertext(lhs.subs(*rhs))])
        }
        for size in (2..128).step_by(2) {
            subs(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_ssub() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::IntegerCiphertext(lhs), IopValue::IntegerPlaintext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::IntegerCiphertext(lhs.ssub(*rhs))])
        }
        for size in (2..128).step_by(2) {
            ssub(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_subs() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::IntegerCiphertext(lhs), IopValue::IntegerPlaintext(rhs)] = inp else {
                unreachable!()
            };
            let (diff, flag) = lhs.overflow_subs(*rhs);
            Some(vec![IopValue::IntegerCiphertext(diff), IopValue::BoolCiphertext(flag)])
        }
        for size in (2..128).step_by(2) {
            overflow_subs(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_ssub() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::IntegerCiphertext(lhs), IopValue::IntegerPlaintext(rhs)] = inp else {
                unreachable!()
            };
            let (diff, flag) = lhs.overflow_ssub(*rhs);
            Some(vec![IopValue::IntegerCiphertext(diff), IopValue::BoolCiphertext(flag)])
        }
        for size in (2..128).step_by(2) {
            overflow_ssub(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    // `test_random` rarely draws the boundaries on wide integers: a null scalar, equal operands,
    // a borrow out of zero. Check them by hand, on one width per algorithm.
    #[test]
    fn correctness_edge_cases() {
        for size in [4u16, 12, 32, 64, 128] {
            let spec = IntegerCiphertextSpec::new(size, 2, 2);
            let pt_spec = spec
                .block_spec()
                .matching_plaintext_block_spec()
                .integer_plaintext_spec(size);
            let mask = spec.int_mask();

            // (ct, imm) pairs: zero scalar, equal operands, borrow out of zero, saturated.
            let cases = [
                (0, 0),
                (0, 1),
                (1, 0),
                (5 & mask, 5 & mask),
                (0, mask),
                (mask, 0),
                (mask, mask),
                (1, mask),
                (mask, 1),
            ];

            for (a, c) in cases {
                let inputs = [
                    IopValue::IntegerCiphertext(spec.from_int(a)),
                    IopValue::IntegerPlaintext(pt_spec.from_int(c)),
                ];
                let ct = spec.from_int(a);
                let pt = pt_spec.from_int(c);

                let got = subs(spec).interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::IntegerCiphertext(ct.subs(pt))],
                    "subs failed for size={size} a={a} c={c}"
                );

                let got = ssub(spec).interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::IntegerCiphertext(ct.ssub(pt))],
                    "ssub failed for size={size} a={a} c={c}"
                );

                let (diff, flag) = ct.overflow_subs(pt);
                let got = overflow_subs(spec)
                    .interpret()
                    .with_inputs(&inputs)
                    .get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::IntegerCiphertext(diff), IopValue::BoolCiphertext(flag)],
                    "overflow_subs failed for size={size} a={a} c={c}"
                );

                let (diff, flag) = ct.overflow_ssub(pt);
                let got = overflow_ssub(spec)
                    .interpret()
                    .with_inputs(&inputs)
                    .get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::IntegerCiphertext(diff), IopValue::BoolCiphertext(flag)],
                    "overflow_ssub failed for size={size} a={a} c={c}"
                );
            }
        }
    }

    #[test]
    fn noise_subs() {
        for size in (2..128).step_by(2) {
            subs(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_ssub() {
        for size in (2..128).step_by(2) {
            ssub(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_overflow_subs() {
        for size in (2..128).step_by(2) {
            overflow_subs(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_overflow_ssub() {
        for size in (2..128).step_by(2) {
            overflow_ssub(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
