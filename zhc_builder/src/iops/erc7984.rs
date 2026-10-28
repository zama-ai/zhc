use zhc_crypto::integer_semantics::SpecInteger;
use zhc_langs::ioplang::Lut1;
use zhc_utils::iter::CollectInSmallVec;

use crate::{CiphertextBlock, CiphertextInteger, builder::Builder};

/// Creates an IR for a homomorphic encrypted fund transfer (ERC-7984).
///
/// Convenience wrapper that calls [`Builder::iop_erc_7984_impl`]. Declares three
/// inputs (from, to, amount) and two outputs (new_from, new_to). For batched
/// transfers see [`erc7984_simd`]. See the builder method for algorithm details.
pub fn erc7984(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_from = builder.ciphertext_integer_input(spec.int_size());
    let src_to = builder.ciphertext_integer_input(spec.int_size());
    let src_amount = builder.ciphertext_integer_input(spec.int_size());
    let (new_from, new_to) = builder.iop_erc_7984_impl(&src_from, &src_to, &src_amount, spec);
    builder.ciphertext_integer_output(new_from);
    builder.ciphertext_integer_output(new_to);
    builder
}

/// Creates an IR for batched (SIMD) homomorphic fund transfers (ERC-7984).
///
/// Declares `n` independent transfer triplets as inputs and output pairs.
/// Each transfer uses [`Builder::iop_erc_7984_ripple`] — optimized for throughput.
/// For single-transfer latency see [`erc7984`].
///
/// # Panics
///
/// Panics if `n <= 1`.
pub fn erc7984_simd(spec: SpecInteger, n: u16) -> Builder {
    assert!(n > 1);
    let builder = Builder::new(spec.block_spec());
    for _ in 0..n {
        let src_from = builder.ciphertext_integer_input(spec.int_size());
        let src_to = builder.ciphertext_integer_input(spec.int_size());
        let src_amount = builder.ciphertext_integer_input(spec.int_size());
        let (new_from, new_to) = builder.iop_erc_7984_ripple(&src_from, &src_to, &src_amount);
        builder.ciphertext_integer_output(new_from);
        builder.ciphertext_integer_output(new_to);
    }

    builder
}

impl Builder {
    /// Computes a homomorphic encrypted fund transfer (latency-optimised).
    ///
    /// Selects the best carry-propagation strategy based on integer size:
    /// ripple carry for small integers, and Kogge-Stone for large.
    ///
    /// See [`iop_erc_7984_ripple`](Self::iop_erc_7984_ripple) for
    /// throughput-oriented variant.
    pub fn iop_erc_7984_impl(
        &self,
        src_from: &CiphertextInteger,
        src_to: &CiphertextInteger,
        src_amount: &CiphertextInteger,
        spec: SpecInteger,
    ) -> (CiphertextInteger, CiphertextInteger) {
        // Step 1: Check if sender has sufficient funds.
        // The result is a raw block in 0..=3, meaning "enough funds" when >= 2. It skips the
        // final PBS of the comparison: the selection below tests it directly.
        let enough_fund = self.erc_7984_enough_fund(src_from, src_amount);

        // Step 2: Compute conditional transfer amount.
        //   enough_fund >= 2 (sufficient) -> actual_amount = src_amount
        //   enough_fund <  2 (insufficient) -> actual_amount = 0
        let actual_amount = self.erc_7984_if_enough_fund(src_amount, &enough_fund);

        // Step 4: new_to = src_to + actual_amount
        let new_to = match spec.int_size() {
            0..8 => self.iop_add_ripple_carry(src_to, &actual_amount, None).0,
            8..256 => self.iop_add_hillis_steele(src_to, &actual_amount, None).0,
            _ => todo!(),
        };

        // Step 5: new_from = src_from - actual_amount (two's complement)
        let actual_amount_inv = self.iop_bitwise_inv(&actual_amount);
        let one = self.block_let_ciphertext(1);
        let new_from = match spec.int_size() {
            0..8 => {
                self.iop_add_ripple_carry(src_from, &actual_amount_inv, Some(&one))
                    .0
            }
            8..256 => {
                self.iop_add_hillis_steele(src_from, &actual_amount_inv, Some(&one))
                    .0
            }
            _ => todo!(),
        };

        (new_from, new_to)
    }

    /// Computes a homomorphic encrypted fund transfer using ripple carry.
    ///
    /// Uses sequential ripple-carry propagation for both the addition and
    /// subtraction steps. This variant has higher per-operation latency but is
    /// more area-efficient, making it suitable for SIMD batching where many
    /// independent transfers run in parallel.
    pub fn iop_erc_7984_ripple(
        &self,
        src_from: &CiphertextInteger,
        src_to: &CiphertextInteger,
        src_amount: &CiphertextInteger,
    ) -> (CiphertextInteger, CiphertextInteger) {
        // Step 1: Check if sender has sufficient funds.
        let amount_inv = self.iop_bitwise_inv(src_amount);
        let one = self.block_let_ciphertext(1);
        let (_diff, enough_fund) = self.iop_add_ripple_carry(src_from, &amount_inv, Some(&one));

        // Step 2: Compute conditional transfer amount.
        let actual_amount = self.iop_if_then_zero(src_amount, &enough_fund);

        // Step 3: new_to = src_to + actual_amount (ripple carry)
        let (new_to, _) = self.iop_add_ripple_carry(src_to, &actual_amount, None);

        // Step 4: new_from = src_from - actual_amount (two's complement, ripple carry)
        let actual_amount_inv = self.iop_bitwise_inv(&actual_amount);
        let one = self.block_let_ciphertext(1);
        let (new_from, _) = self.iop_add_ripple_carry(src_from, &actual_amount_inv, Some(&one));

        (new_from, new_to)
    }

    /// `src_from >= src_amount`, returned as a raw block instead of a boolean.
    ///
    /// The result `f` is in `0..=3` and means "enough funds" exactly when `f >= 2`. It is the
    /// sum of two PBS outputs, not fresh, which spares the final PBS of
    /// [`iop_cmp`](Self::iop_cmp). It must only be consumed by
    /// [`erc_7984_if_enough_fund`](Self::erc_7984_if_enough_fund).
    ///
    /// With the high comparison digit `hi` (0: lower, 1: equal, 2: greater) and the low part
    /// as a boolean `lo_gte`, `f = hi + lo_gte`.
    fn erc_7984_enough_fund(
        &self,
        src_from: &CiphertextInteger,
        src_amount: &CiphertextInteger,
    ) -> CiphertextBlock {
        let spec = *self.spec();
        let (digits, lo_is_gte) = self.cmp_reduce_digits(src_from, src_amount, true);
        self.push_comment("Enough fund");
        let one = self.block_let_plaintext(1);
        let raw = match (digits.len(), lo_is_gte) {
            // f = hi + lo_gte: 0, 1 -> not enough; 2, 3 -> enough.
            (2, true) => self.block_add(&digits[1], &digits[0]),
            // No reduction step ran: merge the two digits into a boolean, then f = bool + 1.
            (2, false) => {
                let p = self.vector_pack(digits.as_slice());
                let gte = self.block_lookup(&p[0], Lut1::cmp_gte_mrg(spec));
                self.block_add_plaintext(&gte, &one)
            }
            // Single digit d (0: lower, 1: equal, 2: greater): f = d + 1.
            (1, _) => self.block_wrapping_add_plaintext(&digits[0], &one),
            _ => unreachable!(),
        };
        self.pop_comment();
        raw
    }

    /// Keeps `src_amount` if the raw selector `enough_fund` is `>= 2`, zero otherwise.
    ///
    /// `enough_fund` comes from [`erc_7984_enough_fund`](Self::erc_7984_enough_fund). Each
    /// block of `src_amount` goes into the carry and the selector into the message
    /// (`4 * block + enough_fund`), so the selector needs no PBS of its own: one lookup per
    /// block does both the test and the selection.
    fn erc_7984_if_enough_fund(
        &self,
        src_amount: &CiphertextInteger,
        enough_fund: &CiphertextBlock,
    ) -> CiphertextInteger {
        let src_blocks = self.ciphertext_integer_split(src_amount);
        let lut = Lut1::from_fn("IfMsgGe2CarryElseZero", *self.spec(), |b| {
            let keep = b.raw_message_bits() >= 2;
            let data = b.move_carry_to_message().raw_message_bits();
            b.spec().from_message(if keep { data } else { 0 })
        });
        let output_blocks = src_blocks
            .iter()
            .map(|b| {
                let packed = self.block_pack(b, enough_fund);
                self.block_lookup(&packed, lut.clone())
            })
            .cosvec();
        self.ciphertext_integer_join(output_blocks, None)
    }
}

#[cfg(test)]
mod test {
    use crate::SIMD_N;

    use super::*;
    use zhc_langs::ioplang::IopValue;

    fn erc7984_semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [
            IopValue::CiphertextInteger(from),
            IopValue::CiphertextInteger(to),
            IopValue::CiphertextInteger(amount),
        ] = inp
        else {
            unreachable!()
        };
        if from >= amount {
            Some(vec![
                IopValue::CiphertextInteger(from.sub(*amount)),
                IopValue::CiphertextInteger(to.add(*amount)),
            ])
        } else {
            Some(vec![
                IopValue::CiphertextInteger(*from),
                IopValue::CiphertextInteger(*to),
            ])
        }
    }

    #[test]
    fn correctness_erc7984() {
        for size in (2..64).step_by(2) {
            erc7984(SpecInteger::new(size, 2, 2)).test_random(100, erc7984_semantic);
        }
    }

    #[test]
    fn correctness_erc7984_simd() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            inp.chunks(3)
                .flat_map(|chunk| {
                    let [
                        IopValue::CiphertextInteger(from),
                        IopValue::CiphertextInteger(to),
                        IopValue::CiphertextInteger(amount),
                    ] = chunk
                    else {
                        unreachable!()
                    };
                    if from >= amount {
                        vec![
                            IopValue::CiphertextInteger(from.sub(*amount)),
                            IopValue::CiphertextInteger(to.add(*amount)),
                        ]
                    } else {
                        vec![
                            IopValue::CiphertextInteger(*from),
                            IopValue::CiphertextInteger(*to),
                        ]
                    }
                })
                .collect::<Vec<_>>()
                .into()
        }
        for size in (2..64).step_by(2) {
            erc7984_simd(SpecInteger::new(size, 2, 2), SIMD_N).test_random(100, semantic);
        }
    }

    #[test]
    fn noise_erc7984() {
        for size in (2..64).step_by(2) {
            erc7984(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_erc7984_simd() {
        for size in (2..64).step_by(2) {
            erc7984_simd(SpecInteger::new(size, 2, 2), SIMD_N).check_noise();
        }
    }
}
