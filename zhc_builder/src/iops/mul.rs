use std::collections::HashMap;

use crate::{CiphertextBlock, NU, NU_BOOL, builder::Builder};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

/// Creates an IR for a multiplication of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one ciphertext output encoding LSB
/// result of the product Internally delegates to [`Builder::iop_mul_raw`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, mul_lsb};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = mul_lsb(spec);
/// let ir = builder.into_ir();
/// ```
pub fn mul_lsb(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());

    // Get input as array of blk
    let src_a_blocks = builder.ciphertext_split(&src_a);
    let src_b_blocks = builder.ciphertext_split(&src_b);
    // Only kept LSB to obtain a IxI -> I operations
    let cut_off = spec.block_count();

    // Call inner function and construct results
    let (_flag, output) = builder.iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off);
    let lsb_output = builder.ciphertext_join(&output, Some(spec.int_size()));
    builder.ciphertext_output(lsb_output);
    builder
}

/// Creates an IR for a multiplication of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and two ciphertext outputs.
/// First output is an overflow flag, second one is the LSB part of the input product
///
/// Internally delegates to [`Builder::iop_mul_raw`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, overflow_mul_lsb};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = overflow_mul_lsb(spec);
/// let ir = builder.into_ir();
/// ```
pub fn overflow_mul_lsb(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());

    // Get input as array of blk
    let src_a_blocks = builder.ciphertext_split(&src_a);
    let src_b_blocks = builder.ciphertext_split(&src_b);
    // Only kept LSB to obtain a IxI -> I operations
    let cut_off = spec.block_count();

    // Call inner function and construct results
    let (flag_block, output) = builder.iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off);
    let flag = builder.ciphertext_join(&[flag_block], Some(1)); // NB: This is a boolean flag
    let lsb_output = builder.ciphertext_join(&output, Some(spec.int_size()));

    builder.ciphertext_output(flag);
    builder.ciphertext_output(lsb_output);
    builder
}

impl Builder {
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
    /// let (flag, res) = builder.iop_mul_raw(&a, &b, spec.block_count());
    /// ```
    pub fn iop_mul_raw(
        &self,
        src_a_blocks: &Vec<CiphertextBlock>,
        src_b_blocks: &Vec<CiphertextBlock>,
        cut_off_block: u8,
    ) -> (CiphertextBlock, Vec<CiphertextBlock>) {
        // Phase 1 expand:
        // It's a cartesien product of a and b for each terms we sort them by degree
        // (i.e. ai +bi) and kept assocatied nu for the later reduction
        // NB: nu encode range of data. nu*(1<<msg_w) = Max Ct value
        // After the cut-off block only NonNull flag is computed instead of the complete partial
        // product with carry extract
        let mut partial_product_map = HashMap::<usize, Vec<CiphertextBlock>>::new();
        let mut overflow_v = Vec::<CiphertextBlock>::new();

        for (i, ai) in src_a_blocks.iter().enumerate() {
            for (j, bj) in src_b_blocks.iter().enumerate() {
                if (i + j) < cut_off_block as usize {
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
        for k in 0..cut_off_block as usize {
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
        if let Some(in_carry_v) = partial_product_map.remove(&(cut_off_block as usize)) {
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

        (overflow_flag, dst_blk)
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
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            vec![IopValue::Ciphertext(lhs.mul_lsb(*rhs))]
        }
        for size in (2..128).step_by(2) {
            mul_lsb(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_mul_lsb() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            vec![IopValue::Ciphertext(lhs.mul_lsb(*rhs))]
        }
        for size in (2..128).step_by(2) {
            mul_lsb(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn test_mul_lsb() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let ir = mul_lsb(spec);
        assert_display_is!(
            ir.ir()
                .format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_opid(true),
            r#"
                @0                        | %0 : Ct = input_ciphertext<0, 8>();
                @1                        | %1 : Ct = input_ciphertext<1, 8>();
                @2                        | %2 : CtBlock = extract_ct_block<0>(%0 : Ct);
                @3                        | %3 : CtBlock = extract_ct_block<1>(%0 : Ct);
                @4                        | %4 : CtBlock = extract_ct_block<2>(%0 : Ct);
                @5                        | %5 : CtBlock = extract_ct_block<3>(%0 : Ct);
                @6                        | %6 : CtBlock = extract_ct_block<0>(%1 : Ct);
                @7                        | %7 : CtBlock = extract_ct_block<1>(%1 : Ct);
                @8                        | %8 : CtBlock = extract_ct_block<2>(%1 : Ct);
                @9                        | %9 : CtBlock = extract_ct_block<3>(%1 : Ct);
                @10   // pack_0_0         | %10 : CtBlock = pack_ct<4>(%2 : CtBlock, %6 : CtBlock);
                @11   // pp_0_0_lsb       | %11 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%10 : CtBlock);
                @12   // pp_0_0_msb       | %12 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%10 : CtBlock);
                @13   // pack_0_1         | %13 : CtBlock = pack_ct<4>(%2 : CtBlock, %7 : CtBlock);
                @14   // pp_0_1_lsb       | %14 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%13 : CtBlock);
                @15   // pp_0_1_msb       | %15 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%13 : CtBlock);
                @16   // pack_0_2         | %16 : CtBlock = pack_ct<4>(%2 : CtBlock, %8 : CtBlock);
                @17   // pp_0_2_lsb       | %17 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%16 : CtBlock);
                @18   // pp_0_2_msb       | %18 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%16 : CtBlock);
                @19   // pack_0_3         | %19 : CtBlock = pack_ct<4>(%2 : CtBlock, %9 : CtBlock);
                @20   // pp_0_3_lsb       | %20 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%19 : CtBlock);
                @21   // pp_0_3_msb       | %21 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%19 : CtBlock);
                @22   // pack_1_0         | %22 : CtBlock = pack_ct<4>(%3 : CtBlock, %6 : CtBlock);
                @23   // pp_1_0_lsb       | %23 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%22 : CtBlock);
                @24   // pp_1_0_msb       | %24 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%22 : CtBlock);
                @25   // pack_1_1         | %25 : CtBlock = pack_ct<4>(%3 : CtBlock, %7 : CtBlock);
                @26   // pp_1_1_lsb       | %26 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%25 : CtBlock);
                @27   // pp_1_1_msb       | %27 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%25 : CtBlock);
                @28   // pack_1_2         | %28 : CtBlock = pack_ct<4>(%3 : CtBlock, %8 : CtBlock);
                @29   // pp_1_2_lsb       | %29 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%28 : CtBlock);
                @30   // pp_1_2_msb       | %30 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%28 : CtBlock);
                @31   // ovf_1_3          | %31 : CtBlock = pack_ct<4>(%3 : CtBlock, %9 : CtBlock);
                @32   // ovf_1_3          | %32 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%31 : CtBlock);
                @33   // pack_2_0         | %33 : CtBlock = pack_ct<4>(%4 : CtBlock, %6 : CtBlock);
                @34   // pp_2_0_lsb       | %34 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%33 : CtBlock);
                @35   // pp_2_0_msb       | %35 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%33 : CtBlock);
                @36   // pack_2_1         | %36 : CtBlock = pack_ct<4>(%4 : CtBlock, %7 : CtBlock);
                @37   // pp_2_1_lsb       | %37 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%36 : CtBlock);
                @38   // pp_2_1_msb       | %38 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%36 : CtBlock);
                @39   // ovf_2_2          | %39 : CtBlock = pack_ct<4>(%4 : CtBlock, %8 : CtBlock);
                @40   // ovf_2_2          | %40 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%39 : CtBlock);
                @41   // ovf_2_3          | %41 : CtBlock = pack_ct<4>(%4 : CtBlock, %9 : CtBlock);
                @42   // ovf_2_3          | %42 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%41 : CtBlock);
                @43   // pack_3_0         | %43 : CtBlock = pack_ct<4>(%5 : CtBlock, %6 : CtBlock);
                @44   // pp_3_0_lsb       | %44 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%43 : CtBlock);
                @45   // pp_3_0_msb       | %45 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%43 : CtBlock);
                @46   // ovf_3_1          | %46 : CtBlock = pack_ct<4>(%5 : CtBlock, %7 : CtBlock);
                @47   // ovf_3_1          | %47 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%46 : CtBlock);
                @48   // ovf_3_2          | %48 : CtBlock = pack_ct<4>(%5 : CtBlock, %8 : CtBlock);
                @49   // ovf_3_2          | %49 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%48 : CtBlock);
                @50   // ovf_3_3          | %50 : CtBlock = pack_ct<4>(%5 : CtBlock, %9 : CtBlock);
                @51   // ovf_3_3          | %51 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%50 : CtBlock);
                @52   // reduction_1      | %52 : CtBlock = add_ct(%14 : CtBlock, %12 : CtBlock);
                @53   // reduction_1      | %53 : CtBlock = add_ct(%23 : CtBlock, %52 : CtBlock);
                @54   // reduction_1      | %54 : CtBlock = pbs<Protect, CarryInMsg>(%53 : CtBlock);
                @55   // reduction_1      | %55 : CtBlock = pbs<Protect, MsgOnly>(%53 : CtBlock);
                @56   // reduction_2      | %56 : CtBlock = add_ct(%17 : CtBlock, %15 : CtBlock);
                @57   // reduction_2      | %57 : CtBlock = add_ct(%24 : CtBlock, %56 : CtBlock);
                @58   // reduction_2      | %58 : CtBlock = add_ct(%26 : CtBlock, %57 : CtBlock);
                @59   // reduction_2      | %59 : CtBlock = add_ct(%34 : CtBlock, %58 : CtBlock);
                @60   // reduction_2      | %60 : CtBlock = pbs<Protect, CarryInMsg>(%59 : CtBlock);
                @61   // reduction_2      | %61 : CtBlock = pbs<Protect, MsgOnly>(%59 : CtBlock);
                @62   // reduction_2      | %62 : CtBlock = add_ct(%54 : CtBlock, %61 : CtBlock);
                @63   // reduction_2      | %63 : CtBlock = pbs<Protect, CarryInMsg>(%62 : CtBlock);
                @64   // reduction_2      | %64 : CtBlock = pbs<Protect, MsgOnly>(%62 : CtBlock);
                @65   // reduction_3      | %65 : CtBlock = add_ct(%20 : CtBlock, %18 : CtBlock);
                @66   // reduction_3      | %66 : CtBlock = add_ct(%27 : CtBlock, %65 : CtBlock);
                @67   // reduction_3      | %67 : CtBlock = add_ct(%29 : CtBlock, %66 : CtBlock);
                @68   // reduction_3      | %68 : CtBlock = add_ct(%35 : CtBlock, %67 : CtBlock);
                @69   // reduction_3      | %69 : CtBlock = pbs<Protect, CarryInMsg>(%68 : CtBlock);
                @70   // reduction_3      | %70 : CtBlock = pbs<Protect, MsgOnly>(%68 : CtBlock);
                @71   // reduction_3      | %71 : CtBlock = add_ct(%37 : CtBlock, %70 : CtBlock);
                @72   // reduction_3      | %72 : CtBlock = add_ct(%44 : CtBlock, %71 : CtBlock);
                @73   // reduction_3      | %73 : CtBlock = add_ct(%60 : CtBlock, %72 : CtBlock);
                @74   // reduction_3      | %74 : CtBlock = add_ct(%63 : CtBlock, %73 : CtBlock);
                @75   // reduction_3      | %75 : CtBlock = pbs<Protect, CarryInMsg>(%74 : CtBlock);
                @76   // reduction_3      | %76 : CtBlock = pbs<Protect, MsgOnly>(%74 : CtBlock);
                @77   // ovf / carry_in   | %77 : CtBlock = add_ct(%21 : CtBlock, %30 : CtBlock);
                @78   // ovf / carry_in   | %78 : CtBlock = add_ct(%77 : CtBlock, %38 : CtBlock);
                @79   // ovf / carry_in   | %79 : CtBlock = add_ct(%78 : CtBlock, %45 : CtBlock);
                @80   // ovf / carry_in   | %80 : CtBlock = add_ct(%79 : CtBlock, %69 : CtBlock);
                @81   // ovf / carry_in   | %81 : CtBlock = pbs<Protect, IsSome>(%80 : CtBlock);
                @82   // ovf / carry_in   | %82 : CtBlock = pbs<Protect, IsSome>(%75 : CtBlock);
                @83   // ovf / merge      | %83 : CtBlock = add_ct(%32 : CtBlock, %40 : CtBlock);
                @84   // ovf / merge      | %84 : CtBlock = add_ct(%83 : CtBlock, %42 : CtBlock);
                @85   // ovf / merge      | %85 : CtBlock = add_ct(%84 : CtBlock, %47 : CtBlock);
                @86   // ovf / merge      | %86 : CtBlock = add_ct(%85 : CtBlock, %49 : CtBlock);
                @87   // ovf / merge      | %87 : CtBlock = add_ct(%86 : CtBlock, %51 : CtBlock);
                @88   // ovf / merge      | %88 : CtBlock = add_ct(%87 : CtBlock, %81 : CtBlock);
                @89   // ovf / merge      | %89 : CtBlock = add_ct(%88 : CtBlock, %82 : CtBlock);
                @90   // ovf / merge      | %90 : CtBlock = pbs<Protect, IsSome>(%89 : CtBlock);
                @91                       | %91 : Ct = decl_ct<8>();
                @92                       | %92 : Ct = store_ct_block<0>(%11 : CtBlock, %91 : Ct);
                @93                       | %93 : Ct = store_ct_block<1>(%55 : CtBlock, %92 : Ct);
                @94                       | %94 : Ct = store_ct_block<2>(%64 : CtBlock, %93 : Ct);
                @95                       | %95 : Ct = store_ct_block<3>(%76 : CtBlock, %94 : Ct);
                @96                       | output<0>(%95 : Ct);
            "#
        );
    }
}
