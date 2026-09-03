use std::collections::BTreeMap;

use crate::{Ciphertext, CiphertextBlock, NU, NU_BOOL, builder::Builder};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::SafeAs;

/// Creates an IR for multiplication of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_mul`]. Returns the low bits of
/// the product (wrapping multiplication). See the builder method for algorithm details.
pub fn mul(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_mul(&src_a, &src_b);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for multiplication with overflow detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_mul`]. Returns the product
/// and a single-block overflow flag. See the builder method for algorithm details.
pub fn overflow_mul(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_mul(&src_a, &src_b);
    builder.ciphertext_output(res);
    builder.ciphertext_output(flag);
    builder
}

impl Builder {
    /// Multiplies two encrypted integers, returning the low bits of the product.
    ///
    /// Computes `(lhs * rhs) mod 2^n` where n is the integer bit-width. This is
    /// wrapping multiplication that discards overflow. For overflow detection, use
    /// [`iop_overflow_mul`](Self::iop_overflow_mul).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let product = builder.iop_mul(&a, &b);
    /// ```
    pub fn iop_mul(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        let src_a_blocks = self.ciphertext_split(&lhs);
        let src_b_blocks = self.ciphertext_split(&rhs);
        // Only kept LSB to obtain a IxI -> I operations
        let cut_off = lhs.spec().block_count();
        // Call inner function and construct results
        let (output, _flag) = self.iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off);
        self.ciphertext_join(&output, Some(lhs.spec().int_size()))
    }

    /// Multiplies two encrypted integers with overflow detection.
    ///
    /// Returns `(product, overflow)` where `product` is the low bits of the
    /// multiplication (wrapping) and `overflow` is a single-block ciphertext: 1 if
    /// the full product exceeds the representable range, 0 otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (product, overflow) = builder.iop_overflow_mul(&a, &b);
    /// ```
    pub fn iop_overflow_mul(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> (Ciphertext, Ciphertext) {
        // Get input as array of blk
        let src_a_blocks = self.ciphertext_split(&lhs);
        let src_b_blocks = self.ciphertext_split(&rhs);
        // Only kept LSB to obtain a IxI -> I operations
        let cut_off = lhs.spec().block_count();
        // Call inner function and construct results
        let (output, flag_block) = self.iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off);
        (
            self.ciphertext_join(&output, Some(lhs.spec().int_size())),
            self.ciphertext_join(&[flag_block], None),
        )
    }

    /// Multiply two ciphertext in a raw fashion.
    /// I.e. Compute all output up to cut-off point then only overflow flag status.
    /// This function should be wrapped specialized instances that select the desired
    /// output information and use the deadcode analysis to remove useless part
    ///
    /// The muliplication is done in two phases:
    ///  * Expansion: generate all the partial product
    ///  * Reduction: sum partial product and propagate the carry
    ///
    /// Overflow computation also uses same phases, whith slight differences:
    ///  * Expansion: only compute NonNull flag of the product
    ///  * Reduction: sum NonNull flag (no carry propagation)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// # let a = builder.ciphertext_split(&a);
    /// # let b = builder.ciphertext_split(&b);
    /// let (res, flag) = builder.iop_mul_raw(&a, &b, spec.block_count());
    /// ```
    pub fn iop_mul_raw(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_off_block: u8,
    ) -> (Vec<CiphertextBlock>, CiphertextBlock) {
        // Phase 1 expand:
        // It's a cartesien product of a and b for each terms we sort them by degree
        // (i.e. ai +bi) and kept assocatied nu for the later reduction
        // NB: nu encode range of data. nu*(1<<msg_w) = Max Ct value
        // After the cut-off block only NonNull flag is computed instead of the complete partial
        // product with carry extract
        let mut partial_product_map = BTreeMap::<usize, Vec<CiphertextBlock>>::new();
        let mut overflow_v = Vec::<CiphertextBlock>::new();

        for (i, ai) in src_a_blocks.iter().enumerate() {
            for (j, bj) in src_b_blocks.iter().enumerate() {
                if (i + j) < cut_off_block.sas::<usize>() {
                    // Full partial product compution
                    // Pack
                    let packed = self.comment(format!("pack_{i}_{j}")).block_pack(ai, bj);
                    // Compute Lsb
                    partial_product_map.entry(i + j).or_default().push(
                        self.comment(format!("pp_{i}_{j}_lsb"))
                            .block_lookup(packed, Lut1Def::MultCarryMsgLsb),
                    );
                    // Compute Msb
                    partial_product_map.entry(i + j + 1).or_default().push(
                        self.comment(format!("pp_{i}_{j}_msb"))
                            .block_lookup(packed, Lut1Def::MultCarryMsgMsb),
                    );
                } else {
                    // Only overflow extraction
                    let mul_is_some = self.comment(format!("ovf_{i}_{j}")).block_pack_then_lookup(
                        ai,
                        bj,
                        Lut1Def::MultCarryMsgIsSome,
                    );
                    overflow_v.push(mul_is_some);
                }
            }
        }

        // Phase 2  Reduce/Merge:
        //
        // Phase 2.a
        // Gather partial products together at each level.
        // Partial product are sum until nu threshold is reach, then carry is extracted
        // and injected in the next stages
        // NB: Reduce up to cut_off_block
        let mut dst_blk = Vec::new();
        for k in 0..cut_off_block.sas::<usize>() {
            self.push_comment(format!("reduction_{k}"));
            let stage_sum = partial_product_map.remove(&k).unwrap_or_default();
            if !stage_sum.is_empty() {
                let mut nxt_stage = Vec::new();
                // Fold them two by two while storing optional carry
                let mut stg_iter = stage_sum.into_iter();
                let mut acc_nu = 1;
                let mut acc_ct = stg_iter.next().unwrap();

                // NB: only fresh ciphertext is push in partial_product_map
                for ct in stg_iter {
                    acc_nu = acc_nu + 1;
                    acc_ct = self.block_add(ct, acc_ct);

                    // Extract carry if required
                    if acc_nu == NU {
                        acc_nu = 1;
                        nxt_stage.push(self.block_lookup(acc_ct, Lut1Def::CarryInMsg));
                        acc_ct = self.block_lookup(acc_ct, Lut1Def::MsgOnly);
                    }
                }

                // Current stage is completly reduce. Clear block if needed
                if acc_nu != 1 {
                    nxt_stage.push(self.block_lookup(acc_ct, Lut1Def::CarryInMsg));
                    acc_ct = self.block_lookup(acc_ct, Lut1Def::MsgOnly);
                }
                dst_blk.push(acc_ct);

                // insert current stage carry in next stage
                if !nxt_stage.is_empty() {
                    partial_product_map
                        .entry(k + 1)
                        .or_default()
                        .extend(nxt_stage);
                }
            }
            self.pop_comment();
        }

        // Phase 2.b
        // Overflow extraction: Only check if a block upper than cut-off is some
        // Here we could be more aggressive on merge since we manipulate only boolean values
        self.push_comment(format!("ovf"));

        // Start by handling last carry of 2.a
        self.push_comment(format!("carry_in"));
        if let Some(in_carry_v) = partial_product_map.remove(&(cut_off_block.sas())) {
            for chunk in in_carry_v.chunks(NU) {
                let mut chunk_iter = chunk.iter();
                let init = *chunk_iter.next().unwrap();
                let chunk_sum = chunk_iter.fold(init, |acc, v| self.block_add(&acc, v));
                let is_some_flag = self.block_lookup(chunk_sum, Lut1Def::IsSome);
                overflow_v.push(is_some_flag);
            }
        }
        self.pop_comment();

        self.push_comment(format!("merge"));
        let overflow_flag = if !overflow_v.is_empty() {
            // All overflow ct entry is a boolean => Merge by grp of max_nu_bool
            while overflow_v.len() > 1 {
                overflow_v = overflow_v
                    .chunks(NU_BOOL)
                    .map(|chunk| {
                        let mut chunk_iter = chunk.iter();
                        let init = *chunk_iter.next().unwrap();
                        let chunk_sum = chunk_iter.fold(init, |acc, v| self.block_add(&acc, v));
                        self.block_lookup(chunk_sum, Lut1Def::IsSome)
                    })
                    .collect();
            }

            overflow_v.pop().unwrap()
        } else {
            self.block_let_ciphertext(0)
        };
        self.pop_comment();
        self.pop_comment();

        (dst_blk, overflow_flag)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::CiphertextSpec;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn correctness_mul_lsb() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.mul_lsb(*rhs))])
        }
        for size in (2..128).step_by(2) {
            mul(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_mul_lsb() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            let (product, flag) = lhs.overflow_mul_lsb(*rhs);
            Some(vec![
                IopValue::Ciphertext(product),
                IopValue::Ciphertext(flag),
            ])
        }
        for size in (2..128).step_by(2) {
            overflow_mul(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn test_mul_lsb() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let ir = mul(spec).optimize_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true),
            r#"
                                 | %0 = input_ciphertext<0, 8>();
                                 | %1 = input_ciphertext<1, 8>();
                                 | %2 = extract_ct_block<0>(%0);
                                 | %3 = extract_ct_block<1>(%0);
                                 | %4 = extract_ct_block<2>(%0);
                                 | %5 = extract_ct_block<3>(%0);
                                 | %6 = extract_ct_block<0>(%1);
                                 | %7 = extract_ct_block<1>(%1);
                                 | %8 = extract_ct_block<2>(%1);
                                 | %9 = extract_ct_block<3>(%1);
                // pack_0_0      | %10 = pack_ct<4>(%2, %6);
                // pp_0_0_lsb    | %11 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%10);
                // pp_0_0_msb    | %12 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%10);
                // pack_0_1      | %13 = pack_ct<4>(%2, %7);
                // pp_0_1_lsb    | %14 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%13);
                // pp_0_1_msb    | %15 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%13);
                // pack_0_2      | %16 = pack_ct<4>(%2, %8);
                // pp_0_2_lsb    | %17 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%16);
                // pp_0_2_msb    | %18 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%16);
                // pack_0_3      | %19 = pack_ct<4>(%2, %9);
                // pp_0_3_lsb    | %20 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%19);
                // pack_1_0      | %22 = pack_ct<4>(%3, %6);
                // pp_1_0_lsb    | %23 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%22);
                // pp_1_0_msb    | %24 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%22);
                // pack_1_1      | %25 = pack_ct<4>(%3, %7);
                // pp_1_1_lsb    | %26 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%25);
                // pp_1_1_msb    | %27 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%25);
                // pack_1_2      | %28 = pack_ct<4>(%3, %8);
                // pp_1_2_lsb    | %29 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%28);
                // pack_2_0      | %33 = pack_ct<4>(%4, %6);
                // pp_2_0_lsb    | %34 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%33);
                // pp_2_0_msb    | %35 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%33);
                // pack_2_1      | %36 = pack_ct<4>(%4, %7);
                // pp_2_1_lsb    | %37 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%36);
                // pack_3_0      | %43 = pack_ct<4>(%5, %6);
                // pp_3_0_lsb    | %44 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%43);
                // reduction_1   | %52 = add_ct(%14, %12);
                // reduction_1   | %53 = add_ct(%23, %52);
                // reduction_1   | %54 = pbs<Protect, Lut1("CarryInMsg")>(%53);
                // reduction_1   | %55 = pbs<Protect, Lut1("MsgOnly")>(%53);
                // reduction_2   | %56 = add_ct(%17, %15);
                // reduction_2   | %57 = add_ct(%24, %56);
                // reduction_2   | %58 = add_ct(%26, %57);
                // reduction_2   | %59 = add_ct(%34, %58);
                // reduction_2   | %60 = pbs<Protect, Lut1("CarryInMsg")>(%59);
                // reduction_2   | %61 = pbs<Protect, Lut1("MsgOnly")>(%59);
                // reduction_2   | %62 = add_ct(%54, %61);
                // reduction_2   | %63 = pbs<Protect, Lut1("CarryInMsg")>(%62);
                // reduction_2   | %64 = pbs<Protect, Lut1("MsgOnly")>(%62);
                // reduction_3   | %65 = add_ct(%20, %18);
                // reduction_3   | %66 = add_ct(%27, %65);
                // reduction_3   | %67 = add_ct(%29, %66);
                // reduction_3   | %68 = add_ct(%35, %67);
                // reduction_3   | %70 = pbs<Protect, Lut1("MsgOnly")>(%68);
                // reduction_3   | %71 = add_ct(%37, %70);
                // reduction_3   | %72 = add_ct(%44, %71);
                // reduction_3   | %73 = add_ct(%60, %72);
                // reduction_3   | %74 = add_ct(%63, %73);
                // reduction_3   | %76 = pbs<Protect, Lut1("MsgOnly")>(%74);
                                 | %91 = decl_ct<8>();
                                 | %97 = store_ct_block<0>(%11, %91);
                                 | %98 = store_ct_block<1>(%55, %97);
                                 | %99 = store_ct_block<2>(%64, %98);
                                 | %100 = store_ct_block<3>(%76, %99);
                                 | output<0>(%100);
            "#
        );
    }

    #[test]
    fn noise_mul_lsb() {
        for size in (2..128).step_by(2) {
            mul(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_overflow_mul_lsb() {
        for size in (2..128).step_by(2) {
            overflow_mul(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
