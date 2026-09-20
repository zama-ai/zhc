use crate::{
    CiphertextBlock, NU_BOOL, NU_MUL_PT, PlaintextBlock,
    builder::{BoolCiphertext, Builder, IntegerCiphertext, IntegerPlaintext},
};
use zhc_crypto::integer_semantics::IntegerCiphertextSpec;
use zhc_langs::ioplang::{Lut1, Lut2};
use zhc_utils::SafeAs;

/// Order in which the carries coming out of a column enter the reduction of the next one.
///
/// A digit product has degree `msg_mask^2` while a carry only has degree `msg_mask`, so the
/// carries are what lets an extraction fill a block up to its capacity. Taking them early costs
/// the fewest PBS, but it also makes the n-th extraction of a column wait on the n-th extraction
/// of the previous one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryOrder {
    /// Reduce the digit products first, the incoming carries after. An extraction wastes some
    /// capacity, but a column no longer waits on the whole reduction of the previous one. Better
    /// for narrow integers, which are latency bound.
    Late,
    /// Mix the incoming carries with the digit products, so that every extraction is filled up to
    /// capacity. Better for wide integers, which are throughput bound.
    Interleaved,
}

/// Carry ordering used by [`Builder::iop_muls`], see [`CarryOrder`].
fn carry_order(int_size: u16) -> CarryOrder {
    match int_size {
        0..40 => CarryOrder::Late,
        _ => CarryOrder::Interleaved,
    }
}

/// Creates an IR for the multiplication of an encrypted integer by a scalar (`ct * imm`).
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_muls`].
/// Returns the low bits of the product (wrapping multiplication). See that method for
/// algorithm details.
pub fn muls(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let res = builder.iop_muls(&src_c, &src_p);
    builder.integer_ciphertext_output(res);
    builder
}

/// Creates an IR for `ct * imm` with overflow detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_muls`]. Returns two outputs:
/// the wrapping product and a single-block overflow flag.
pub fn overflow_muls(spec: IntegerCiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.integer_ciphertext_input(spec.int_size());
    let src_p = builder.integer_plaintext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_muls(&src_c, &src_p);
    builder.integer_ciphertext_output(res);
    builder.bool_ciphertext_output(flag);
    builder
}

impl Builder {
    /// Multiplies an encrypted integer by a scalar, automatically selecting the best algorithm.
    ///
    /// Computes `(lhs * rhs) mod 2^n` where `n` is the integer bit-width, i.e. wrapping
    /// multiplication that discards the overflowing MSBs.
    ///
    /// A partial product is a ciphertext digit times a plaintext digit, which needs no PBS. The
    /// whole cost is the carry reduction of the columns, see
    /// [`iop_muls_raw`](Self::iop_muls_raw). The carry order is picked on the operand width.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// let product = builder.iop_muls(&a, &b);
    /// ```
    pub fn iop_muls(&self, lhs: &IntegerCiphertext, rhs: &IntegerPlaintext) -> IntegerCiphertext {
        let src_c_blocks = self.integer_ciphertext_split(lhs);
        let src_p_blocks = self.integer_plaintext_split(rhs);
        let cut_off = lhs.spec().block_count();
        let (output, _flag) = self.iop_muls_raw(
            &src_c_blocks,
            &src_p_blocks,
            cut_off,
            carry_order(lhs.spec().int_size()),
        );
        self.integer_ciphertext_join(&output, Some(lhs.spec().int_size()))
    }

    /// Multiplies an encrypted integer by a scalar with overflow detection.
    ///
    /// Returns `(product, overflow)` where `product` is the low bits of the multiplication
    /// (wrapping) and `overflow` is a single-block ciphertext: 1 if the full product exceeds the
    /// representable range, 0 otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// let (product, overflow) = builder.iop_overflow_muls(&a, &b);
    /// ```
    pub fn iop_overflow_muls(
        &self,
        lhs: &IntegerCiphertext,
        rhs: &IntegerPlaintext,
    ) -> (IntegerCiphertext, BoolCiphertext) {
        let src_c_blocks = self.integer_ciphertext_split(lhs);
        let src_p_blocks = self.integer_plaintext_split(rhs);
        let cut_off = lhs.spec().block_count();
        let (output, flag_block) = self.iop_muls_raw(
            &src_c_blocks,
            &src_p_blocks,
            cut_off,
            carry_order(lhs.spec().int_size()),
        );
        (
            self.integer_ciphertext_join(&output, Some(lhs.spec().int_size())),
            self.bool_ciphertext_from_block(flag_block),
        )
    }

    /// Multiplies a ciphertext by a plaintext in a raw fashion.
    /// I.e. compute all the output blocks up to the cut-off point, dropping the MSBs.
    /// This function should be wrapped by specialized instances that select the desired
    /// output information and use the deadcode analysis to remove the useless parts.
    ///
    /// The multiplication is done in two phases:
    ///  * Expansion: generate all the partial products (linear, no PBS)
    ///  * Reduction: sum the partial products of a column and propagate the carry
    ///
    /// Overflow computation also uses the same phases, with slight differences:
    ///  * Expansion: only compute the NonNull flag of a whole row (linear, one PBS per digit)
    ///  * Reduction: merge the NonNull flags together with the carry leaving the cut-off column
    ///
    /// NB: the terms are not clean digits here, so the `NU` counting of
    /// [`iop_mul_raw`](Self::iop_mul_raw) does not apply. Every term and the accumulator carry
    /// their degree, and a carry is extracted only when the next term would not fit.
    ///
    /// # Panics
    ///
    /// Panics if `carry_size < message_size`, since a digit product would then not fit in the
    /// `carry + message` capacity of a block.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CarryOrder, IntegerCiphertextSpec, Builder};
    /// # let spec = IntegerCiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.integer_ciphertext_input(spec.int_size());
    /// # let b = builder.integer_plaintext_input(spec.int_size());
    /// # let a = builder.integer_ciphertext_split(&a);
    /// # let b = builder.integer_plaintext_split(&b);
    /// let (res, flag) = builder.iop_muls_raw(&a, &b, spec.block_count(), CarryOrder::Interleaved);
    /// ```
    pub fn iop_muls_raw(
        &self,
        src_c_blocks: &[CiphertextBlock],
        src_p_blocks: &[PlaintextBlock],
        cut_off_block: u8,
        carry_order: CarryOrder,
    ) -> (Vec<CiphertextBlock>, CiphertextBlock) {
        let msg_mod = 1_usize << self.spec().message_size();
        let msg_mask = msg_mod - 1;
        // Largest value a block may hold without overflowing into the padding bit.
        let capacity = self.spec().data_mask().sas::<usize>();
        assert!(
            msg_mask * msg_mask <= capacity,
            "MULS needs carry_size >= message_size, a digit product does not fit in a block."
        );
        let cut_off = cut_off_block.sas::<usize>();

        // Phase 1 expand: all the products of a ciphertext digit by a plaintext digit, gathered by
        // output column (i.e. i+j) with the degree of the term.
        // NB: a degree is the largest value a block may hold. Contrary to the ct x ct case the
        // terms are not clean digits, a digit product spans [0, msg_mask^2].
        // Columns at or above the cut-off only feed the dropped MSBs: only their non-nullity
        // matters, which the overflow terms below track.
        let mut partial_product = vec![Vec::<(CiphertextBlock, usize)>::new(); cut_off];
        let mut overflow_terms = Vec::<(CiphertextBlock, usize)>::new();
        for (i, ci) in src_c_blocks.iter().enumerate() {
            // `IsSome` of a ciphertext digit is shared by the whole dropped part of its row.
            let first_dropped = cut_off.saturating_sub(i);
            let is_some = (first_dropped < src_p_blocks.len()).then(|| {
                self.comment(format!("ovf_is_some_{i}"))
                    .block_lookup(ci, Lut1::is_some(*self.spec()))
            });
            for (j, pj) in src_p_blocks.iter().enumerate() {
                if (i + j) < cut_off {
                    let pp = self
                        .comment(format!("pp_{i}_{j}"))
                        .block_mul_plaintext(ci, pj);
                    partial_product[i + j].push((pp, msg_mask * msg_mask));
                } else {
                    // `a_i * c_j` is not null only if both digits are not null, and multiplying
                    // the flag of the ciphertext digit by the plaintext one is linear: no PBS.
                    let ovf = self
                        .comment(format!("ovf_{i}_{j}"))
                        .block_mul_plaintext(is_some.expect("row has a dropped part"), pj);
                    overflow_terms.push((ovf, msg_mask));
                }
            }
        }

        // Phase 2 reduce/merge: sum the terms of a column while they fit in a block. When the next
        // one would go over the capacity, extract the carry, which becomes a term of the next
        // column, and go on with the message left. A column ends as a single clean digit.
        // The carries leaving the cut-off column are dropped MSBs too, so they join the overflow
        // terms instead of a column: hence the extra slot.
        let mut carry_in = vec![Vec::<(CiphertextBlock, usize)>::new(); cut_off + 1];
        let mut dst_blk = Vec::with_capacity(cut_off);
        for k in 0..cut_off {
            self.push_comment(format!("reduction_{k}"));
            let want_carry = true;
            let stage = order_terms(
                std::mem::take(&mut partial_product[k]),
                std::mem::take(&mut carry_in[k]),
                carry_order,
            );

            // Every column below the cut-off holds at least the (k, 0) partial product, unless
            // the caller asked for more blocks than the operands can feed.
            let mut stage_iter = stage.into_iter();
            let (mut acc_ct, mut acc_deg) = stage_iter
                .next()
                .unwrap_or_else(|| (self.block_let_ciphertext(0), 0));

            for (ct, deg) in stage_iter {
                if acc_deg + deg > capacity {
                    // Room exhausted: canonicalize the accumulator before going on.
                    let (msg, carry) = self.block_extract_carry(acc_ct, acc_deg, want_carry);
                    if let Some(carry) = carry {
                        carry_in[k + 1].push((carry, acc_deg / msg_mod));
                    }
                    (acc_ct, acc_deg) = (msg, acc_deg.min(msg_mask));
                }
                acc_ct = self.block_add(ct, acc_ct);
                acc_deg += deg;
            }

            // Column reduced, but the block may not be a clean digit yet.
            if acc_deg > msg_mask {
                let (msg, carry) = self.block_extract_carry(acc_ct, acc_deg, want_carry);
                if let Some(carry) = carry {
                    carry_in[k + 1].push((carry, acc_deg / msg_mod));
                }
                acc_ct = msg;
            }
            dst_blk.push(acc_ct);
            self.pop_comment();
        }

        // Phase 2.b overflow merge: every dropped term is non-negative, so a group of them is not
        // null only if one of its members is not null.
        self.push_comment("ovf");
        overflow_terms.extend(std::mem::take(&mut carry_in[cut_off]));
        let overflow_flag = self.block_merge_is_some(overflow_terms);
        self.pop_comment();

        (dst_blk, overflow_flag)
    }

    /// Splits an accumulator of degree `acc_deg` into its message digit and the carry to
    /// forward to the next column, the carry being computed only when `want_carry` is set.
    ///
    /// Uses a single many-lut PBS when the accumulator fits in the many-lut input space (its
    /// topmost data bit must be clear), and falls back to a `MsgOnly`/`CarryInMsg` PBS pair
    /// otherwise.
    fn block_extract_carry(
        &self,
        acc_ct: CiphertextBlock,
        acc_deg: usize,
        want_carry: bool,
    ) -> (CiphertextBlock, Option<CiphertextBlock>) {
        let many_capacity = self.spec().data_mask().sas::<usize>() >> 1;
        match (want_carry, acc_deg <= many_capacity) {
            (true, true) => {
                let (msg, carry) = self.block_lookup2(acc_ct, Lut2::many_carry_msg(*self.spec()));
                (msg, Some(carry))
            }
            (true, false) => (
                self.block_lookup(acc_ct, Lut1::msg_only(*self.spec())),
                Some(self.block_lookup(acc_ct, Lut1::carry_in_msg(*self.spec()))),
            ),
            (false, _) => (
                self.block_lookup(acc_ct, Lut1::msg_only(*self.spec())),
                None,
            ),
        }
    }

    /// Reduces a set of non-negative terms to a single boolean telling whether any is non-null.
    ///
    /// A sum of non-negative terms is not null only if one of them is not null. So terms are summed
    /// while they fit in a block, then every group is turned into a flag by an `IsSome` lookup,
    /// until one is left. Contrary to [`iop_mul_raw`](Self::iop_mul_raw), which merges by
    /// `NU`/`NU_BOOL`, the grouping follows the degrees, so the first round already takes terms
    /// that are not booleans.
    ///
    /// A group is also limited in number of terms: the degrees alone would fill a block up to its
    /// capacity, but a term out of `mul_pt` is as noisy as `msg_mask` fresh blocks, so the noise
    /// budget runs out first.
    fn block_merge_is_some(&self, terms: Vec<(CiphertextBlock, usize)>) -> CiphertextBlock {
        let capacity = self.spec().data_mask().sas::<usize>();
        // `mul_pt` terms in the first round, plain flags after it.
        let mut max_terms = NU_MUL_PT;
        let mut terms = terms;
        if terms.is_empty() {
            return self.block_let_ciphertext(0);
        }
        loop {
            // A lone flag is the answer, a lone wider term still needs a lookup.
            if terms.len() == 1 && terms[0].1 <= 1 {
                return terms[0].0;
            }
            let mut merged = Vec::with_capacity(terms.len());
            let mut term_iter = terms.into_iter();
            let (mut acc_ct, mut acc_deg) = term_iter.next().expect("non empty");
            let mut acc_len = 1;
            for (ct, deg) in term_iter {
                if acc_deg + deg > capacity || acc_len == max_terms {
                    merged.push((self.block_lookup(acc_ct, Lut1::is_some(*self.spec())), 1));
                    (acc_ct, acc_deg, acc_len) = (ct, deg, 1);
                } else {
                    acc_ct = self.block_add(ct, acc_ct);
                    acc_deg += deg;
                    acc_len += 1;
                }
            }
            merged.push((self.block_lookup(acc_ct, Lut1::is_some(*self.spec())), 1));
            terms = merged;
            max_terms = NU_BOOL;
        }
    }
}

/// Lays out the terms of a column reduction according to `carry_order`.
fn order_terms(
    partial_product: Vec<(CiphertextBlock, usize)>,
    carry_in: Vec<(CiphertextBlock, usize)>,
    carry_order: CarryOrder,
) -> Vec<(CiphertextBlock, usize)> {
    match carry_order {
        CarryOrder::Late => partial_product.into_iter().chain(carry_in).collect(),
        CarryOrder::Interleaved => {
            // The n-th carry is ready at the depth of the n-th extraction of the previous
            // column, hence the pairwise interleaving.
            let mut pp_iter = partial_product.into_iter();
            let mut cin_iter = carry_in.into_iter();
            let mut stage = Vec::new();
            loop {
                let (pp, cin) = (pp_iter.next(), cin_iter.next());
                if pp.is_none() && cin.is_none() {
                    return stage;
                }
                stage.extend(pp);
                stage.extend(cin);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    fn muls_with(spec: IntegerCiphertextSpec, carry_order: CarryOrder) -> Builder {
        let builder = Builder::new(spec.block_spec());
        let src_c = builder.integer_ciphertext_input(spec.int_size());
        let src_p = builder.integer_plaintext_input(spec.int_size());
        let src_c_blocks = builder.integer_ciphertext_split(&src_c);
        let src_p_blocks = builder.integer_plaintext_split(&src_p);
        let (res, _flag) = builder.iop_muls_raw(
            &src_c_blocks,
            &src_p_blocks,
            spec.block_count(),
            carry_order,
        );
        let res = builder.integer_ciphertext_join(&res, Some(spec.int_size()));
        builder.integer_ciphertext_output(res);
        builder
    }

    fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [
            IopValue::IntegerCiphertext(lhs),
            IopValue::IntegerPlaintext(rhs),
        ] = inp
        else {
            unreachable!()
        };
        Some(vec![IopValue::IntegerCiphertext(lhs.muls_lsb(*rhs))])
    }

    fn overflow_semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [
            IopValue::IntegerCiphertext(lhs),
            IopValue::IntegerPlaintext(rhs),
        ] = inp
        else {
            unreachable!()
        };
        let (product, flag) = lhs.overflow_muls_lsb(*rhs);
        Some(vec![
            IopValue::IntegerCiphertext(product),
            IopValue::BoolCiphertext(flag),
        ])
    }

    #[test]
    fn correctness_overflow_muls_lsb() {
        for size in (2..128).step_by(2) {
            overflow_muls(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, overflow_semantic);
        }
    }

    #[test]
    fn correctness_muls_lsb() {
        for size in (2..128).step_by(2) {
            muls(IntegerCiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn test_muls_lsb() {
        let spec = IntegerCiphertextSpec::new(8, 2, 2);
        let ir = muls(spec).optimize_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true),
            r#"
                                 | %0 = input_ciphertext<0, 8>();
                                 | %1 = input_plaintext<1, 8>();
                                 | %2 = extract_ct_block<0>(%0);
                                 | %3 = extract_ct_block<1>(%0);
                                 | %4 = extract_ct_block<2>(%0);
                                 | %5 = extract_ct_block<3>(%0);
                                 | %6 = extract_pt_block<0>(%1);
                                 | %7 = extract_pt_block<1>(%1);
                                 | %8 = extract_pt_block<2>(%1);
                                 | %9 = extract_pt_block<3>(%1);
                // pp_0_0        | %10 = mul_pt(%2, %6);
                // pp_0_1        | %11 = mul_pt(%2, %7);
                // pp_0_2        | %12 = mul_pt(%2, %8);
                // pp_0_3        | %13 = mul_pt(%2, %9);
                // pp_1_0        | %15 = mul_pt(%3, %6);
                // pp_1_1        | %16 = mul_pt(%3, %7);
                // pp_1_2        | %17 = mul_pt(%3, %8);
                // pp_2_0        | %20 = mul_pt(%4, %6);
                // pp_2_1        | %21 = mul_pt(%4, %7);
                // pp_3_0        | %25 = mul_pt(%5, %6);
                // reduction_0   | %29 = pbs<Protect, Lut1("MsgOnly")>(%10);
                // reduction_0   | %30 = pbs<Protect, Lut1("CarryInMsg")>(%10);
                // reduction_1   | %31 = pbs<Protect, Lut1("MsgOnly")>(%11);
                // reduction_1   | %32 = pbs<Protect, Lut1("CarryInMsg")>(%11);
                // reduction_1   | %33 = add_ct(%15, %31);
                // reduction_1   | %34 = add_ct(%30, %33);
                // reduction_1   | %35 = pbs<Protect, Lut1("MsgOnly")>(%34);
                // reduction_1   | %36 = pbs<Protect, Lut1("CarryInMsg")>(%34);
                // reduction_2   | %37 = pbs<Protect, Lut1("MsgOnly")>(%12);
                // reduction_2   | %38 = pbs<Protect, Lut1("CarryInMsg")>(%12);
                // reduction_2   | %39 = add_ct(%16, %37);
                // reduction_2   | %40 = pbs<Protect, Lut1("MsgOnly")>(%39);
                // reduction_2   | %41 = pbs<Protect, Lut1("CarryInMsg")>(%39);
                // reduction_2   | %42 = add_ct(%20, %40);
                // reduction_2   | %43 = add_ct(%32, %42);
                // reduction_2   | %44 = pbs<Protect, Lut1("MsgOnly")>(%43);
                // reduction_2   | %45 = pbs<Protect, Lut1("CarryInMsg")>(%43);
                // reduction_2   | %46 = add_ct(%36, %44);
                // reduction_2   | %47, %48 = pbs2<Protect, Lut2("ManyCarryMsg")>(%46);
                // reduction_3   | %49 = pbs<Protect, Lut1("MsgOnly")>(%13);
                // reduction_3   | %51 = add_ct(%17, %49);
                // reduction_3   | %52 = pbs<Protect, Lut1("MsgOnly")>(%51);
                // reduction_3   | %54 = add_ct(%21, %52);
                // reduction_3   | %55 = pbs<Protect, Lut1("MsgOnly")>(%54);
                // reduction_3   | %57 = add_ct(%25, %55);
                // reduction_3   | %58 = add_ct(%38, %57);
                // reduction_3   | %59 = pbs<Protect, Lut1("MsgOnly")>(%58);
                // reduction_3   | %61 = add_ct(%41, %59);
                // reduction_3   | %62 = add_ct(%45, %61);
                // reduction_3   | %63 = add_ct(%48, %62);
                // reduction_3   | %64 = pbs<Protect, Lut1("MsgOnly")>(%63);
                                 | %80 = decl_ct<8>();
                                 | %86 = store_ct_block<0>(%29, %80);
                                 | %87 = store_ct_block<1>(%35, %86);
                                 | %88 = store_ct_block<2>(%47, %87);
                                 | %89 = store_ct_block<3>(%64, %88);
                                 | output<0>(%89);
            "#
        );
    }

    #[test]
    fn correctness_muls_lsb_carry_order() {
        // Both orderings must hold over the whole width range, not only where `carry_order`
        // selects them.
        for size in (2..64).step_by(2) {
            for order in [CarryOrder::Late, CarryOrder::Interleaved] {
                muls_with(IntegerCiphertextSpec::new(size, 2, 2), order).test_random(50, semantic);
            }
        }
    }

    #[test]
    fn noise_muls_lsb() {
        for size in (2..128).step_by(2) {
            muls(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_overflow_muls_lsb() {
        for size in (2..128).step_by(2) {
            overflow_muls(IntegerCiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
