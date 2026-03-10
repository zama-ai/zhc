use std::collections::BTreeMap;

use crate::{CiphertextBlock, NU, NU_BOOL, builder::Builder};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

/// Creates an IR for a multiplication of two encrypted integers split into mh_factor sub-graph.
///
/// The returned [`Builder`] declares two ciphertext inputs and one ciphertext output encoding LSB
/// result of the product
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
/// let builder = mh_mul_lsb(spec, 2);
/// let ir = builder.into_ir();
/// ```
pub fn mh_mul_lsb(spec: CiphertextSpec, mh_factor: u8) -> Builder {
    mh_mul_lsb_with_opt(spec, mh_factor, false)
}

/// Creates an IR for a multiplication of two encrypted integers split into mh_factor sub-graph.
///
/// The returned [`Builder`] declares two ciphertext inputs and two ciphertext outputs.
/// First output is an overflow flag, second one is the LSB part of the input product
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
/// let builder = mh_overflow_mul_lsb(spec, 2);
/// let ir = builder.into_ir();
/// ```
pub fn mh_overflow_mul_lsb(spec: CiphertextSpec, mh_factor: u8) -> Builder {
    mh_mul_lsb_with_opt(spec, mh_factor, true)
}

/// Creates an IR for a multiplication of two encrypted integers split into mh_factor sub-graph.
///
/// The returned [`Builder`] declares two ciphertext inputs and two ciphertext outputs.
/// First output is an *Optional* overflow flag, second one is the LSB part of the input product
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
/// let builder = mh_mul_lsb_with_opt(spec, 2, true);
/// let ir = builder.into_ir();
/// ```
fn mh_mul_lsb_with_opt(spec: CiphertextSpec, mh_factor: u8, gen_overflow: bool) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());

    // Get input as array of blk
    let src_a_blocks = builder.split_ciphertext(&src_a);
    let src_b_blocks = builder.split_ciphertext(&src_b);
    // Only kept LSB to obtain a IxI -> I operations
    let cut_off = spec.block_count();

    // Call inner function and construct results
    let (flag_block, outputs) =
        builder.mh_iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off, mh_factor);

    if gen_overflow {
        let flag = builder.join_ciphertext(&[flag_block], Some(1)); // NB: This is a boolean flag
        builder.output_ciphertext(flag);
    }
    let pack_output = outputs.into_iter().flatten().collect::<Vec<_>>();
    let output = builder.join_ciphertext(&pack_output, Some(spec.int_size()));
    builder.output_ciphertext(output);

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
    /// # let a = builder.input_ciphertext(spec.int_size());
    /// # let b = builder.input_ciphertext(spec.int_size());
    /// # let a = builder.split_ciphertext(&a);
    /// # let b = builder.split_ciphertext(&b);
    /// let (flag, res) = builder.iop_mul_raw(&a, &b, spec.block_count());
    /// ```
    pub fn mh_iop_mul_raw(
        &self,
        src_a_blocks: &Vec<CiphertextBlock>,
        src_b_blocks: &Vec<CiphertextBlock>,
        cut_off_block: u8,
        mh_factor: u8,
    ) -> (CiphertextBlock, Vec<Vec<CiphertextBlock>>) {
        // Phase 1 expand:
        let (overflow_v, partprod_map) =
            self.expand_partprod(src_a_blocks, src_b_blocks, cut_off_block);

        // Split in mh_factor chunk of work
        let mh_overflow_v = Self::split_vec(mh_factor as usize, overflow_v);
        let mh_partprod_map = Self::split_btmap(mh_factor as usize, partprod_map);

        // Phase 2  Reduce data
        let mut mh_data_blk = Vec::with_capacity(mh_factor as usize);
        let mut prv_post_map = BTreeMap::<usize, Vec<CiphertextBlock>>::new();
        for mut partprod_map in mh_partprod_map {
            // fuse post_map of previous chunk
            // Only fuse item below our scope point
            let scope_block = partprod_map.keys().max().copied().unwrap_or_default();
            partprod_map.extend(
                prv_post_map
                    .keys()
                    .copied()
                    .filter(|k| *k <= scope_block as usize)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .filter_map(|k| prv_post_map.remove(&k).map(|v| (k, v))),
            );
            let (data_blk, post_map) = self.merge_partprod(partprod_map);

            // Stort data result
            mh_data_blk.push(data_blk);

            // Insert explicit block transfer on post_map
            // and store for next iter
            prv_post_map.extend(
                post_map
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().map(|b| self.block_transfer(b)).collect())),
            );
        }

        // Phase 3 Reduce overflow
        // 3.a merge each chunk independently
        // Last flag is transfered
        let mut mh_ovf_iter = mh_overflow_v.into_iter();
        let mh_ovf_flags: Vec<_> =
            std::iter::once(self.merge_overflow_flag(prv_post_map, mh_ovf_iter.next().unwrap()))
                .chain(mh_ovf_iter.map(|v| {
                    let flag = self.merge_overflow_flag(Default::default(), v);
                    self.block_transfer(flag)
                }))
                .collect();

        // 3.b merge all remaining flag together
        let ovf_flag = self.merge_overflow_flag(Default::default(), mh_ovf_flags);
        (ovf_flag, mh_data_blk)
    }

    /// Helper function to split a vector in n parts
    /// with homogeneous repartion while preserving contiguousness
    fn split_vec<T: Clone>(n: usize, v: Vec<T>) -> Vec<Vec<T>> {
        let len = v.len();
        let base = len / n;
        let remainder = len % n;

        let mut splitted = Vec::with_capacity(n);
        let mut cons_idx = 0;

        for i in 0..n {
            // Distribute the remainder across first chunks
            let chunk_size = base + if i < remainder { 1 } else { 0 };
            splitted.push(v[cons_idx..cons_idx + chunk_size].to_vec());
            cons_idx += chunk_size;
        }
        splitted
    }

    /// Helper function to split a BTreeMap in n parts
    /// with homogeneous repartion while preserving contiguousness
    fn split_btmap<K: Ord, V>(n: usize, map: BTreeMap<K, V>) -> Vec<BTreeMap<K, V>> {
        let len = map.len();
        let base = len / n;
        let remainder = len % n;

        let mut splitted: Vec<BTreeMap<K, V>> = (0..n).map(|_| BTreeMap::new()).collect();
        let mut iter = map.into_iter();

        for i in 0..n {
            let chunk_size = base + if i < remainder { 1 } else { 0 };
            for (k, v) in iter.by_ref().take(chunk_size) {
                splitted[i].insert(k, v);
            }
        }
        splitted
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::CiphertextSpec;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    const MH_FACTOR: u8 = 2;

    #[test]
    fn correctness_mh_mul_lsb() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            vec![IopValue::Ciphertext(lhs.mul_lsb(*rhs))]
        }
        for size in (2..128).step_by(2) {
            mh_mul_lsb(CiphertextSpec::new(size, 2, 2), MH_FACTOR).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_mh_overflow_mul_lsb() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            vec![IopValue::Ciphertext(lhs.mul_lsb(*rhs))]
        }
        for size in (2..128).step_by(2) {
            mh_mul_lsb(CiphertextSpec::new(size, 2, 2), MH_FACTOR).test_random(100, semantic);
        }
    }

    #[test]
    fn test_mh_mul_lsb() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let ir = mh_mul_lsb(spec, MH_FACTOR);
        assert_display_is!(
            ir.ir()
                .format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_opid(true),
            r#"
                @00                                                              | %0 : Ct = input_ciphertext<0, 8>();
                @01                                                              | %1 : Ct = input_ciphertext<1, 8>();
                @02                                                              | %2 : CtBlock = extract_ct_block<0>(%0 : Ct);
                @03                                                              | %3 : CtBlock = extract_ct_block<1>(%0 : Ct);
                @04                                                              | %4 : CtBlock = extract_ct_block<2>(%0 : Ct);
                @05                                                              | %5 : CtBlock = extract_ct_block<3>(%0 : Ct);
                @06                                                              | %6 : CtBlock = extract_ct_block<0>(%1 : Ct);
                @07                                                              | %7 : CtBlock = extract_ct_block<1>(%1 : Ct);
                @08                                                              | %8 : CtBlock = extract_ct_block<2>(%1 : Ct);
                @09                                                              | %9 : CtBlock = extract_ct_block<3>(%1 : Ct);
                @10   // pack_0_0                                                | %10 : CtBlock = pack_ct<4>(%2 : CtBlock, %6 : CtBlock);
                @11   // pp_0_0_lsb                                              | %11 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%10 : CtBlock);
                @12   // pp_0_0_msb                                              | %12 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%10 : CtBlock);
                @13   // pack_0_1                                                | %13 : CtBlock = pack_ct<4>(%2 : CtBlock, %7 : CtBlock);
                @14   // pp_0_1_lsb                                              | %14 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%13 : CtBlock);
                @15   // pp_0_1_msb                                              | %15 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%13 : CtBlock);
                @16   // pack_0_2                                                | %16 : CtBlock = pack_ct<4>(%2 : CtBlock, %8 : CtBlock);
                @17   // pp_0_2_lsb                                              | %17 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%16 : CtBlock);
                @18   // pp_0_2_msb                                              | %18 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%16 : CtBlock);
                @19   // pack_0_3                                                | %19 : CtBlock = pack_ct<4>(%2 : CtBlock, %9 : CtBlock);
                @20   // pp_0_3_lsb                                              | %20 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%19 : CtBlock);
                @21   // pp_0_3_msb                                              | %21 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%19 : CtBlock);
                @22   // pack_1_0                                                | %22 : CtBlock = pack_ct<4>(%3 : CtBlock, %6 : CtBlock);
                @23   // pp_1_0_lsb                                              | %23 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%22 : CtBlock);
                @24   // pp_1_0_msb                                              | %24 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%22 : CtBlock);
                @25   // pack_1_1                                                | %25 : CtBlock = pack_ct<4>(%3 : CtBlock, %7 : CtBlock);
                @26   // pp_1_1_lsb                                              | %26 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%25 : CtBlock);
                @27   // pp_1_1_msb                                              | %27 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%25 : CtBlock);
                @28   // pack_1_2                                                | %28 : CtBlock = pack_ct<4>(%3 : CtBlock, %8 : CtBlock);
                @29   // pp_1_2_lsb                                              | %29 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%28 : CtBlock);
                @30   // pp_1_2_msb                                              | %30 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%28 : CtBlock);
                @31   // ovf_1_3                                                 | %31 : CtBlock = pack_ct<4>(%3 : CtBlock, %9 : CtBlock);
                @32   // ovf_1_3                                                 | %32 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%31 : CtBlock);
                @33   // pack_2_0                                                | %33 : CtBlock = pack_ct<4>(%4 : CtBlock, %6 : CtBlock);
                @34   // pp_2_0_lsb                                              | %34 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%33 : CtBlock);
                @35   // pp_2_0_msb                                              | %35 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%33 : CtBlock);
                @36   // pack_2_1                                                | %36 : CtBlock = pack_ct<4>(%4 : CtBlock, %7 : CtBlock);
                @37   // pp_2_1_lsb                                              | %37 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%36 : CtBlock);
                @38   // pp_2_1_msb                                              | %38 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%36 : CtBlock);
                @39   // ovf_2_2                                                 | %39 : CtBlock = pack_ct<4>(%4 : CtBlock, %8 : CtBlock);
                @40   // ovf_2_2                                                 | %40 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%39 : CtBlock);
                @41   // ovf_2_3                                                 | %41 : CtBlock = pack_ct<4>(%4 : CtBlock, %9 : CtBlock);
                @42   // ovf_2_3                                                 | %42 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%41 : CtBlock);
                @43   // pack_3_0                                                | %43 : CtBlock = pack_ct<4>(%5 : CtBlock, %6 : CtBlock);
                @44   // pp_3_0_lsb                                              | %44 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%43 : CtBlock);
                @45   // pp_3_0_msb                                              | %45 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%43 : CtBlock);
                @46   // ovf_3_1                                                 | %46 : CtBlock = pack_ct<4>(%5 : CtBlock, %7 : CtBlock);
                @47   // ovf_3_1                                                 | %47 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%46 : CtBlock);
                @48   // ovf_3_2                                                 | %48 : CtBlock = pack_ct<4>(%5 : CtBlock, %8 : CtBlock);
                @49   // ovf_3_2                                                 | %49 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%48 : CtBlock);
                @50   // ovf_3_3                                                 | %50 : CtBlock = pack_ct<4>(%5 : CtBlock, %9 : CtBlock);
                @51   // ovf_3_3                                                 | %51 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%50 : CtBlock);
                @52   // reduction_1                                             | %52 : CtBlock = add_ct(%14 : CtBlock, %12 : CtBlock);
                @53   // reduction_1                                             | %53 : CtBlock = add_ct(%23 : CtBlock, %52 : CtBlock);
                @54   // reduction_1                                             | %54 : CtBlock = pbs<Protect, CarryInMsg>(%53 : CtBlock);
                @55   // reduction_1                                             | %55 : CtBlock = pbs<Protect, MsgOnly>(%53 : CtBlock);
                @56   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %56 : CtBlock = add_ct(%20 : CtBlock, %18 : CtBlock);
                @57   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %57 : CtBlock = add_ct(%27 : CtBlock, %56 : CtBlock);
                @58   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %58 : CtBlock = add_ct(%29 : CtBlock, %57 : CtBlock);
                @59   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %59 : CtBlock = add_ct(%35 : CtBlock, %58 : CtBlock);
                @60   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %60 : CtBlock = pbs<Protect, CarryInMsg>(%59 : CtBlock);
                @61   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %61 : CtBlock = pbs<Protect, MsgOnly>(%59 : CtBlock);
                @62   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %62 : CtBlock = add_ct(%37 : CtBlock, %61 : CtBlock);
                @63   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %63 : CtBlock = add_ct(%44 : CtBlock, %62 : CtBlock);
                @64   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %64 : CtBlock = pbs<Protect, CarryInMsg>(%63 : CtBlock);
                @65   // reduction_0 / reduction_1 / reduction_2 / reduction_3   | %65 : CtBlock = pbs<Protect, MsgOnly>(%63 : CtBlock);
                @66   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %66 : CtBlock = add_ct(%32 : CtBlock, %40 : CtBlock);
                @67   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %67 : CtBlock = add_ct(%66 : CtBlock, %42 : CtBlock);
                @68   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %68 : CtBlock = pbs<Protect, IsSome>(%67 : CtBlock);
                @69   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %69 : CtBlock = add_ct(%47 : CtBlock, %49 : CtBlock);
                @70   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %70 : CtBlock = add_ct(%69 : CtBlock, %51 : CtBlock);
                @71   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %71 : CtBlock = pbs<Protect, IsSome>(%70 : CtBlock);
                @72   // reduction_0 / reduction_1 / reduction_2                 | %72 : CtBlock = transfer(%71 : CtBlock);
                @73   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %73 : CtBlock = add_ct(%68 : CtBlock, %72 : CtBlock);
                @74   // reduction_0 / reduction_1 / reduction_2 / ovf / merge   | %74 : CtBlock = pbs<Protect, IsSome>(%73 : CtBlock);
                @75   // reduction_0 / reduction_1 / reduction_2                 | %75 : Ct = decl_ct<4>();
                @76   // reduction_0 / reduction_1 / reduction_2                 | %76 : Ct = store_ct_block<0>(%11 : CtBlock, %75 : Ct);
                @77   // reduction_0 / reduction_1 / reduction_2                 | %77 : Ct = store_ct_block<1>(%55 : CtBlock, %76 : Ct);
                @78   // reduction_0 / reduction_1 / reduction_2                 | output<0>(%77 : Ct);
                @79   // reduction_0 / reduction_1 / reduction_2                 | %78 : Ct = decl_ct<2>();
                @80   // reduction_0 / reduction_1 / reduction_2                 | %79 : Ct = store_ct_block<0>(%65 : CtBlock, %78 : Ct);
                @81   // reduction_0 / reduction_1 / reduction_2                 | output<1>(%79 : Ct);
            "#
        );
    }
}
