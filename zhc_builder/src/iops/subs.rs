//! Scalar subtraction IOps, all built on top of the [`adds`](super::adds) datapath.
//!
//! Every operation here is a rewrite of a subtraction into the scalar *addition* already
//! implemented in [`Builder::iop_adds`](Builder::iop_adds), using the identity
//! `A - B == A + !B + 1` (two's complement). Writing `mask` for `2^W - 1`, `!X == mask - X`,
//! which on a radix decomposition is just the per-digit `msg_mask - digit` — a *linear*
//! plaintext-minus-ciphertext operation ([`Builder::iop_bitwise_inv`]), so it costs one DOp
//! per block and **no PBS**.
//!
//! | op | identity | carry-in | carry-out is |
//! |---|---|---|---|
//! | [`Builder::iop_subs`] | `A - C == !(!A + C)` | none | the borrow, directly |
//! | [`Builder::iop_ssub`] | `C - A == !A + C + 1` | 1 | the *inverse* of the borrow |
//!
//! Both overflow flavours reuse the very same sums, so the whole family costs `ADDS` plus a
//! handful of carry-free DOps.

use crate::builder::{Builder, Ciphertext, Plaintext};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

/// Kogge-Stone chunk width, matching the table used by [`super::add`]/[`super::adds`].
fn par_w(int_size: u16) -> usize {
    match int_size {
        8..16 => 1,
        16..24 => 7,
        24..256 => 12,
        _ => 1,
    }
}

/// Creates an IR for the subtraction of a scalar from an encrypted integer (`ct - imm`).
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_subs`].
/// See that method for algorithm details.
pub fn subs(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let res = builder.iop_subs(&src_c, &src_p);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for the subtraction of an encrypted integer from a scalar (`imm - ct`).
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_ssub`].
/// See that method for algorithm details.
///
/// The ciphertext input is declared first so that the operand slots match the ones used by
/// [`adds`](super::adds), independently of the reversed mathematical operand order.
pub fn ssub(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let res = builder.iop_ssub(&src_p, &src_c);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for `ct - imm` with overflow (borrow) detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_subs`]. Returns two outputs:
/// the wrapping difference and a single-block borrow flag.
pub fn overflow_subs(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_subs(&src_c, &src_p);
    builder.ciphertext_output(res);
    builder.ciphertext_output(flag);
    builder
}

/// Creates an IR for `imm - ct` with overflow (borrow) detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_ssub`]. Returns two outputs:
/// the wrapping difference and a single-block borrow flag.
pub fn overflow_ssub(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_ssub(&src_p, &src_c);
    builder.ciphertext_output(res);
    builder.ciphertext_output(flag);
    builder
}

impl Builder {
    /// Subtracts a scalar from an encrypted integer, automatically selecting the best algorithm.
    ///
    /// Computes `lhs - rhs` as `!(!lhs + rhs)`, delegating the carry propagation to
    /// [`iop_adds`](Self::iop_adds) — so the algorithm (ripple-carry, Hillis-Steele or
    /// Kogge-Stone) and the PBS count are exactly those of a scalar addition. The two
    /// complements are plaintext-minus-ciphertext operations, which need no PBS.
    ///
    /// It is costing 1 block_plaintext_sub per block before & after adds so in TFHE-rs we are
    /// not using it and perfer inverting constant at runtime which we cannot do in ZHC.
    /// It will have to be improved when we give ucore a few DOp instructions to manipulate
    /// immediate on its own.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let diff = builder.iop_subs(&a, &b);
    /// ```
    pub fn iop_subs(&self, lhs: &Ciphertext, rhs: &Plaintext) -> Ciphertext {
        // lhs - rhs == !(!lhs + rhs)
        let a_inv = self.comment("Invert Input").iop_bitwise_inv(lhs);
        let sum = self.iop_adds(&a_inv, rhs);
        self.comment("Invert Output").iop_bitwise_inv(&sum)
    }

    /// Subtracts an encrypted integer from a scalar, automatically selecting the best algorithm.
    ///
    /// Computes `lhs - rhs` as `!rhs + lhs + 1`, i.e. a scalar addition
    /// ([`iop_adds`](Self::iop_adds)) on the complemented ciphertext with an injected
    /// carry-in of 1. The complement needs no PBS, so the cost is that of a scalar addition.
    ///
    /// The result is the wrapping difference.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.plaintext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let diff = builder.iop_ssub(&a, &b);
    /// ```
    pub fn iop_ssub(&self, lhs: &Plaintext, rhs: &Ciphertext) -> Ciphertext {
        // lhs - rhs == !rhs + lhs + 1
        let int_size = rhs.spec().int_size();
        let one = self.block_let_ciphertext(1);
        let b_inv = self.comment("Invert Input").iop_bitwise_inv(rhs);
        match int_size {
            0..8 => self.iop_adds_ripple_carry(&b_inv, lhs, Some(&one)).0,
            8..17 => self.iop_adds_hillis_steele(&b_inv, lhs, Some(&one)).0,
            17..256 => self
                .iop_adds_kogge_stone(&b_inv, lhs, Some(&one), par_w(int_size))
                .0,
            _ => todo!(),
        }
    }

    /// Subtracts a scalar from an encrypted integer with overflow (borrow) detection.
    ///
    /// Returns `(difference, overflow)` where `difference` is `lhs - rhs` (wrapping) and
    /// `overflow` is a single-block ciphertext: 1 if `rhs > lhs` (unsigned underflow
    /// occurred), 0 otherwise.
    ///
    /// Because the sum computed under the hood is `!lhs + rhs`, its carry-out is set exactly
    /// when `rhs > lhs` — it *is* the borrow, so unlike [`iop_overflow_sub`](Self::iop_overflow_sub)
    /// no inversion PBS is needed on the flag.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let (diff, borrow) = builder.iop_overflow_subs(&a, &b);
    /// ```
    pub fn iop_overflow_subs(
        &self,
        lhs: &Ciphertext,
        rhs: &Plaintext,
    ) -> (Ciphertext, Ciphertext) {
        // lhs - rhs == !(!lhs + rhs), and carry_out(!lhs + rhs) == 1 iff rhs > lhs.
        let int_size = lhs.spec().int_size();
        let a_inv = self.comment("Invert Input").iop_bitwise_inv(lhs);
        let (sum, carry_out) = match int_size {
            0..8 => self.iop_adds_ripple_carry(&a_inv, rhs, None),
            8..17 => self.iop_adds_hillis_steele(&a_inv, rhs, None),
            17..256 => self.iop_adds_kogge_stone(&a_inv, rhs, None, par_w(int_size)),
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
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.plaintext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (diff, borrow) = builder.iop_overflow_ssub(&a, &b);
    /// ```
    pub fn iop_overflow_ssub(
        &self,
        lhs: &Plaintext,
        rhs: &Ciphertext,
    ) -> (Ciphertext, Ciphertext) {
        // lhs - rhs == !rhs + lhs + 1, whose carry-out is set iff lhs >= rhs.
        let int_size = rhs.spec().int_size();
        let one = self.block_let_ciphertext(1);
        let b_inv = self.comment("Invert Input").iop_bitwise_inv(rhs);
        let (res, carry_out) = match int_size {
            0..8 => self.iop_adds_ripple_carry(&b_inv, lhs, Some(&one)),
            8..17 => self.iop_adds_hillis_steele(&b_inv, lhs, Some(&one)),
            17..256 => {
                // Take the raw form so that `IsNull` can be applied straight to the
                // PG-encoded carry, saving the `IsSome` normalization the joined form does.
                let b_inv = self.ciphertext_split(&b_inv);
                let lhs = self.plaintext_split(lhs);
                let (blocks, co) =
                    self.iop_adds_kogge_stone_raw(b_inv, lhs, Some(&one), par_w(int_size), false);
                (
                    self.comment("Join Output").ciphertext_join(blocks, None),
                    self.comment("Join Carry").ciphertext_join([co], None),
                )
            }
            _ => todo!(),
        };

        // carry_out=1 means NO overflow (lhs >= rhs), carry_out=0 means overflow (lhs < rhs).
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
    fn correctness_subs() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.subs(*rhs))])
        }
        for size in (2..128).step_by(2) {
            subs(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_ssub() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.ssub(*rhs))])
        }
        for size in (2..128).step_by(2) {
            ssub(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_subs() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            let (diff, flag) = lhs.overflow_subs(*rhs);
            Some(vec![IopValue::Ciphertext(diff), IopValue::Ciphertext(flag)])
        }
        for size in (2..128).step_by(2) {
            overflow_subs(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_ssub() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            let (diff, flag) = lhs.overflow_ssub(*rhs);
            Some(vec![IopValue::Ciphertext(diff), IopValue::Ciphertext(flag)])
        }
        for size in (2..128).step_by(2) {
            overflow_ssub(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    /// The identities are most fragile at the boundaries (a zero scalar, equal operands,
    /// a borrow out of zero), which `test_random` is unlikely to hit on wide integers.
    /// Check them explicitly, on one width per algorithm band.
    #[test]
    fn correctness_edge_cases() {
        for size in [4u16, 12, 32, 64, 128] {
            let spec = CiphertextSpec::new(size, 2, 2);
            let pt_spec = spec.matching_plaintext_spec();
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
                    IopValue::Ciphertext(spec.from_int(a)),
                    IopValue::Plaintext(pt_spec.from_int(c)),
                ];
                let ct = spec.from_int(a);
                let pt = pt_spec.from_int(c);

                let got = subs(spec).interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::Ciphertext(ct.subs(pt))],
                    "subs failed for size={size} a={a} c={c}"
                );

                let got = ssub(spec).interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::Ciphertext(ct.ssub(pt))],
                    "ssub failed for size={size} a={a} c={c}"
                );

                let (diff, flag) = ct.overflow_subs(pt);
                let got = overflow_subs(spec).interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::Ciphertext(diff), IopValue::Ciphertext(flag)],
                    "overflow_subs failed for size={size} a={a} c={c}"
                );

                let (diff, flag) = ct.overflow_ssub(pt);
                let got = overflow_ssub(spec).interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(
                    got,
                    vec![IopValue::Ciphertext(diff), IopValue::Ciphertext(flag)],
                    "overflow_ssub failed for size={size} a={a} c={c}"
                );
            }
        }
    }
}
