use zhc_crypto::integer_semantics::CiphertextSpec;

use crate::{Ciphertext, CmpKind, builder::Builder};

/// Creates an IR for a homomorphic encrypted fund transfer (ERC-7984).
///
/// Convenience wrapper that calls [`Builder::iop_erc_7984_impl`]. Declares three
/// inputs (from, to, amount) and two outputs (new_from, new_to). For batched
/// transfers see [`erc7984_simd`]. See the builder method for algorithm details.
pub fn erc7984(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_from = builder.ciphertext_input(spec.int_size());
    let src_to = builder.ciphertext_input(spec.int_size());
    let src_amount = builder.ciphertext_input(spec.int_size());
    let (new_from, new_to) = builder.iop_erc_7984_impl(&src_from, &src_to, &src_amount, spec);
    builder.ciphertext_output(new_from);
    builder.ciphertext_output(new_to);
    builder
}

/// Creates an IR for batched (SIMD) homomorphic fund transfers (ERC-7984).
///
/// Declares `SIMD_N` independent transfer triplets as inputs and output pairs.
/// Each transfer uses [`Builder::iop_erc_7984_ripple`] — optimized for throughput.
/// For single-transfer latency see [`erc7984`].
pub fn erc7984_simd(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());

    for _ in 0..crate::SIMD_N {
        let src_from = builder.ciphertext_input(spec.int_size());
        let src_to = builder.ciphertext_input(spec.int_size());
        let src_amount = builder.ciphertext_input(spec.int_size());
        let (new_from, new_to) = builder.iop_erc_7984_ripple(&src_from, &src_to, &src_amount);
        builder.ciphertext_output(new_from);
        builder.ciphertext_output(new_to);
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
        src_from: &Ciphertext,
        src_to: &Ciphertext,
        src_amount: &Ciphertext,
        spec: CiphertextSpec,
    ) -> (Ciphertext, Ciphertext) {
        // Step 1: Check if sender has sufficient funds.
        let enough_fund = self.iop_cmp(src_from, src_amount, CmpKind::GreaterOrEqual);

        // Step 2: Compute conditional transfer amount.
        // iop_if_then_zero uses IfFalseZeroed internally:
        //   enough_fund=1 (sufficient) -> actual_amount = src_amount
        //   enough_fund=0 (insufficient) -> actual_amount = 0
        let actual_amount = self.iop_if_then_zero(src_amount, &enough_fund);

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
        src_from: &Ciphertext,
        src_to: &Ciphertext,
        src_amount: &Ciphertext,
    ) -> (Ciphertext, Ciphertext) {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;

    fn erc7984_semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [
            IopValue::Ciphertext(from),
            IopValue::Ciphertext(to),
            IopValue::Ciphertext(amount),
        ] = inp
        else {
            unreachable!()
        };
        if from >= amount {
            Some(vec![
                IopValue::Ciphertext(from.sub(*amount)),
                IopValue::Ciphertext(to.add(*amount)),
            ])
        } else {
            Some(vec![IopValue::Ciphertext(*from), IopValue::Ciphertext(*to)])
        }
    }

    #[test]
    fn correctness_erc7984() {
        for size in (2..64).step_by(2) {
            erc7984(CiphertextSpec::new(size, 2, 2)).test_random(100, erc7984_semantic);
        }
    }

    #[test]
    fn correctness_erc7984_simd() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            inp.chunks(3)
                .flat_map(|chunk| {
                    let [
                        IopValue::Ciphertext(from),
                        IopValue::Ciphertext(to),
                        IopValue::Ciphertext(amount),
                    ] = chunk
                    else {
                        unreachable!()
                    };
                    if from >= amount {
                        vec![
                            IopValue::Ciphertext(from.sub(*amount)),
                            IopValue::Ciphertext(to.add(*amount)),
                        ]
                    } else {
                        vec![IopValue::Ciphertext(*from), IopValue::Ciphertext(*to)]
                    }
                })
                .collect::<Vec<_>>()
                .into()
        }
        for size in (2..64).step_by(2) {
            erc7984_simd(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn noise_erc7984() {
        for size in (2..64).step_by(2) {
            erc7984(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_erc7984_simd() {
        for size in (2..64).step_by(2) {
            erc7984_simd(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
