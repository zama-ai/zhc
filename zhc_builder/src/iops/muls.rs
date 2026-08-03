//! Scalar multiplication IOps, i.e. `ct * imm`.
//!
//! The algorithm is the same schoolbook expand/reduce as [`mul`](super::mul), but every partial
//! product involves a *plaintext* digit, so it becomes a linear operation:
//!
//! ```text
//! A * C == sum_{i,j} (a_i * c_j) * (2^msg_w)^(i+j)
//! ```
//!
//! `a_i * c_j` is a ciphertext-block times plaintext-block product
//! ([`Builder::block_mul_plaintext`]) — one DOp and **no PBS**, where the `ct x ct` case needs a
//! pack plus two PBS (`MultCarryMsgLsb`/`MultCarryMsgMsb`) per partial product. The whole PBS
//! budget of `MULS` is therefore spent on the reduction alone: a digit product ranges over
//! `[0, msg_mask^2]`, so a column of them saturates the `carry + message` capacity of a block
//! quickly and has to be canonicalized with a carry extraction.
//!
//! | op | partial product | reduction |
//! |---|---|---|
//! | [`Builder::iop_mul`] | 1 pack + 2 PBS | `NU` clean terms per extraction |
//! | [`Builder::iop_muls`] | 1 linear DOp | degree driven, see below |
//!
//! Because the terms entering the reduction are not clean digits, the `NU` counting used by
//! [`Builder::iop_mul_raw`] does not apply: this implementation tracks the *degree* (the largest
//! value a block may hold) of every term and of the accumulator, and only pays a carry extraction
//! when the next term would not fit. Extractions use the many-lut flavor
//! ([`Lut2Def::ManyCarryMsg`], one PBS for both halves) when the accumulator is small enough for
//! it, and the `MsgOnly`/`CarryInMsg` pair otherwise.
//!
//! The order in which the carries of the previous column are consumed trades PBS count against
//! critical path, see [`CarryOrder`]; [`Builder::iop_muls`] picks it per bit-width.
//!
//! Overflow detection benefits from the plaintext operand too. A dropped partial product is
//! non-null iff *both* its digits are, and `a_i` being non-null is a property of the ciphertext
//! alone: one `IsSome` per ciphertext digit turns the whole row into the linear
//! `IsSome(a_i) * c_j`, whose degree is a mere `msg_mask` -- so several of them fit in one block
//! before a single PBS decides the row. [`Builder::iop_mul_raw`] instead needs one
//! `MultCarryMsgIsSome` PBS for every dropped *pair*.

use crate::{
    CiphertextBlock, PlaintextBlock,
    builder::{Builder, Ciphertext, Plaintext},
};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::SafeAs;

/// Order in which the carries coming out of a column enter the reduction of the next one.
///
/// A digit product has degree `msg_mask^2` while a carry only has degree `msg_mask`, so the
/// carries are what allows an extraction to fill a block right up to its capacity. Feeding them
/// early is therefore the cheapest option, but it also makes the n-th extraction of a column wait
/// on the n-th extraction of the previous one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryOrder {
    /// Reduce the digit products first and only then the incoming carries. An extraction has
    /// only the message left over by the previous one to fill its spare room, which wastes some
    /// capacity, but a column no longer waits on the whole reduction of the previous one.
    /// Prefer this for narrow integers, which are latency bound.
    Late,
    /// Interleave the incoming carries with the digit products, filling every extraction up to
    /// capacity. Prefer this for wide integers, which are throughput bound.
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
pub fn muls(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let res = builder.iop_muls(&src_c, &src_p);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for `ct * imm` with overflow detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_muls`]. Returns two outputs:
/// the wrapping product and a single-block overflow flag.
pub fn overflow_muls(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_muls(&src_c, &src_p);
    builder.ciphertext_output(res);
    builder.ciphertext_output(flag);
    builder
}

impl Builder {
    /// Multiplies an encrypted integer by a scalar, automatically selecting the best algorithm.
    ///
    /// Computes `(lhs * rhs) mod 2^n` where `n` is the integer bit-width, i.e. wrapping
    /// multiplication that discards the overflowing MSBs.
    ///
    /// Partial products are plaintext-times-ciphertext products, which need no PBS; the cost of
    /// the operation is the carry reduction of the partial product columns, whose flavor is
    /// selected on the operand bit-width. See the `muls` module documentation for the details.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let product = builder.iop_muls(&a, &b);
    /// ```
    pub fn iop_muls(&self, lhs: &Ciphertext, rhs: &Plaintext) -> Ciphertext {
        let src_c_blocks = self.ciphertext_split(lhs);
        let src_p_blocks = self.plaintext_split(rhs);
        // Only keep the LSBs to obtain an IxP -> I operation
        let cut_off = lhs.spec().block_count();
        let (output, _flag) = self.iop_muls_raw(
            &src_c_blocks,
            &src_p_blocks,
            cut_off,
            carry_order(lhs.spec().int_size()),
        );
        self.ciphertext_join(&output, Some(lhs.spec().int_size()))
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
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let (product, overflow) = builder.iop_overflow_muls(&a, &b);
    /// ```
    pub fn iop_overflow_muls(&self, lhs: &Ciphertext, rhs: &Plaintext) -> (Ciphertext, Ciphertext) {
        let src_c_blocks = self.ciphertext_split(lhs);
        let src_p_blocks = self.plaintext_split(rhs);
        // Only keep the LSBs to obtain an IxP -> I operation
        let cut_off = lhs.spec().block_count();
        let (output, flag_block) = self.iop_muls_raw(
            &src_c_blocks,
            &src_p_blocks,
            cut_off,
            carry_order(lhs.spec().int_size()),
        );
        (
            self.ciphertext_join(&output, Some(lhs.spec().int_size())),
            self.ciphertext_join([flag_block], None),
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
    /// # Panics
    ///
    /// Panics if `carry_size < message_size`, since a digit product would then not fit in the
    /// `carry + message` capacity of a block.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CarryOrder, CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// # let a = builder.ciphertext_split(&a);
    /// # let b = builder.plaintext_split(&b);
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

        // Phase 1 expand:
        // Cartesian product of the ciphertext and plaintext digits, gathered by output column
        // (i.e. i+j) together with the degree of the term, for the later reduction.
        // NB: a degree is the largest value a block may hold. Contrary to the ct x ct case the
        // terms are not clean digits, a digit product spans [0, msg_mask^2].
        // Columns at or above the cut-off only feed the dropped MSBs: their exact value is
        // irrelevant, only whether they are non-null, which the overflow terms below track.
        let mut partial_product = vec![Vec::<(CiphertextBlock, usize)>::new(); cut_off];
        let mut overflow_terms = Vec::<(CiphertextBlock, usize)>::new();
        for (i, ci) in src_c_blocks.iter().enumerate() {
            // `IsSome` of a ciphertext digit is shared by the whole dropped part of its row.
            let first_dropped = cut_off.saturating_sub(i);
            let is_some = (first_dropped < src_p_blocks.len()).then(|| {
                self.comment(format!("ovf_is_some_{i}"))
                    .block_lookup(ci, Lut1Def::IsSome)
            });
            for (j, pj) in src_p_blocks.iter().enumerate() {
                if (i + j) < cut_off {
                    let pp = self
                        .comment(format!("pp_{i}_{j}"))
                        .block_mul_plaintext(ci, pj);
                    partial_product[i + j].push((pp, msg_mask * msg_mask));
                } else {
                    // `a_i * c_j != 0` iff both digits are non-null, and multiplying the flag of
                    // the ciphertext digit by the plaintext one is linear: no PBS here.
                    let ovf = self
                        .comment(format!("ovf_{i}_{j}"))
                        .block_mul_plaintext(is_some.expect("row has a dropped part"), pj);
                    overflow_terms.push((ovf, msg_mask));
                }
            }
        }

        // Phase 2 reduce/merge:
        // Sum the terms of a column while they fit in a block; when the next one would overflow
        // the capacity, extract the carry -- which becomes a term of the next column -- and keep
        // going with the message left over. A column ends up as a single clean digit.
        // The carries leaving the cut-off column are dropped MSBs as well, so they join the
        // overflow terms instead of a column: hence the extra slot.
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

            // Column completely reduced. Clear the block if it is not a clean digit yet.
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

        // Phase 2.b
        // Overflow merge: every dropped contribution is now a small non-negative term, so a
        // group of them is non-null iff one of its members is.
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
                let (msg, carry) = self.block_lookup2(acc_ct, Lut2Def::ManyCarryMsg);
                (msg, Some(carry))
            }
            (true, false) => (
                self.block_lookup(acc_ct, Lut1Def::MsgOnly),
                Some(self.block_lookup(acc_ct, Lut1Def::CarryInMsg)),
            ),
            (false, _) => (self.block_lookup(acc_ct, Lut1Def::MsgOnly), None),
        }
    }

    /// Reduces a set of non-negative terms to a single boolean telling whether any is non-null.
    ///
    /// Terms are summed while they fit in a block -- a sum of non-negative terms is non-null iff
    /// one of them is -- then every group is turned into a flag by an `IsSome` lookup, until a
    /// single one is left. Contrary to [`iop_mul_raw`](Self::iop_mul_raw), which merges by
    /// `NU`/`NU_BOOL`, the grouping follows the degrees so that the first round can already
    /// absorb non-boolean terms.
    fn block_merge_is_some(&self, terms: Vec<(CiphertextBlock, usize)>) -> CiphertextBlock {
        let capacity = self.spec().data_mask().sas::<usize>();
        let mut terms = terms;
        if terms.is_empty() {
            return self.block_let_ciphertext(0);
        }
        loop {
            // A lone flag is the answer; a lone wider term still needs to be normalized.
            if terms.len() == 1 && terms[0].1 <= 1 {
                return terms[0].0;
            }
            let mut merged = Vec::with_capacity(terms.len());
            let mut term_iter = terms.into_iter();
            let (mut acc_ct, mut acc_deg) = term_iter.next().expect("non empty");
            for (ct, deg) in term_iter {
                if acc_deg + deg > capacity {
                    merged.push((self.block_lookup(acc_ct, Lut1Def::IsSome), 1));
                    (acc_ct, acc_deg) = (ct, deg);
                } else {
                    acc_ct = self.block_add(ct, acc_ct);
                    acc_deg += deg;
                }
            }
            merged.push((self.block_lookup(acc_ct, Lut1Def::IsSome), 1));
            terms = merged;
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

    fn muls_with(spec: CiphertextSpec, carry_order: CarryOrder) -> Builder {
        let builder = Builder::new(spec.block_spec());
        let src_c = builder.ciphertext_input(spec.int_size());
        let src_p = builder.plaintext_input(spec.int_size());
        let src_c_blocks = builder.ciphertext_split(&src_c);
        let src_p_blocks = builder.plaintext_split(&src_p);
        let (res, _flag) = builder.iop_muls_raw(
            &src_c_blocks,
            &src_p_blocks,
            spec.block_count(),
            carry_order,
        );
        let res = builder.ciphertext_join(&res, Some(spec.int_size()));
        builder.ciphertext_output(res);
        builder
    }

    fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
            unreachable!()
        };
        Some(vec![IopValue::Ciphertext(lhs.muls_lsb(*rhs))])
    }

    fn overflow_semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
            unreachable!()
        };
        let (product, flag) = lhs.overflow_muls_lsb(*rhs);
        Some(vec![
            IopValue::Ciphertext(product),
            IopValue::Ciphertext(flag),
        ])
    }

    #[test]
    fn correctness_overflow_muls_lsb() {
        for size in (2..128).step_by(2) {
            overflow_muls(CiphertextSpec::new(size, 2, 2)).test_random(100, overflow_semantic);
        }
    }

    #[test]
    fn correctness_muls_lsb() {
        for size in (2..128).step_by(2) {
            muls(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn test_muls_lsb() {
        let spec = CiphertextSpec::new(8, 2, 2);
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
                muls_with(CiphertextSpec::new(size, 2, 2), order).test_random(50, semantic);
            }
        }
    }
}
