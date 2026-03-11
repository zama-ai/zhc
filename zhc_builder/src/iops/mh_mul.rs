use std::collections::BTreeMap;

use crate::{CiphertextBlock, PartProd, builder::Builder};
use zhc_crypto::integer_semantics::CiphertextSpec;

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
    // View output as one
    // // let pack_output = outputs.into_iter().flatten().collect::<Vec<_>>();
    // // let output = builder.join_ciphertext(&pack_output, Some(spec.int_size()));
    // builder.output_ciphertext(output);

    // View output as mh_factor sub-part
    for out in outputs.into_iter() {
        let output = builder.join_ciphertext(
            &out,
            Some(out.len() as u16 * spec.block_spec().message_size() as u16),
        );
        builder.output_ciphertext(output);
    }

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
        println!("mh_partprod_map {mh_partprod_map:?}");

        // Phase 2  Reduce data
        let mut mh_data_blk = Vec::with_capacity(mh_factor as usize);
        let mut prv_post_map = BTreeMap::<usize, Vec<CiphertextBlock>>::new();
        for mut partprod_map in mh_partprod_map.into_iter() {
            // fuse post_map of previous chunk
            // Insert explicit block transfer here
            for (k, v) in prv_post_map {
                partprod_map.entry(k).or_default().extend(
                    v.into_iter()
                        .map(|b| PartProd::FromSum(self.block_transfer(b))),
                );
            }
            let (data_blk, post_map) = self.merge_partprod(partprod_map, cut_off_block);

            // Stort data result
            mh_data_blk.push(data_blk);

            // Store post_map for next iter
            prv_post_map = post_map;
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
            // Distribute the remainder across last chunks
            let chunk_size = base + if (n - i) < remainder { 1 } else { 0 };
            println!("@{i} => {chunk_size}");
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
                @000                       | %0 : Ct = input_ciphertext<0, 8>();
                @001                       | %1 : Ct = input_ciphertext<1, 8>();
                @002                       | %2 : CtBlock = extract_ct_block<0>(%0 : Ct);
                @003                       | %3 : CtBlock = extract_ct_block<1>(%0 : Ct);
                @004                       | %4 : CtBlock = extract_ct_block<2>(%0 : Ct);
                @005                       | %5 : CtBlock = extract_ct_block<3>(%0 : Ct);
                @006                       | %6 : CtBlock = extract_ct_block<0>(%1 : Ct);
                @007                       | %7 : CtBlock = extract_ct_block<1>(%1 : Ct);
                @008                       | %8 : CtBlock = extract_ct_block<2>(%1 : Ct);
                @009                       | %9 : CtBlock = extract_ct_block<3>(%1 : Ct);
                @010   // pack_0_0         | %10 : CtBlock = pack_ct<4>(%2 : CtBlock, %6 : CtBlock);
                @011   // pp_0_0_lsb       | %11 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%10 : CtBlock);
                @012   // pp_0_0_msb       | %12 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%10 : CtBlock);
                @013   // pack_0_1         | %13 : CtBlock = pack_ct<4>(%2 : CtBlock, %7 : CtBlock);
                @014   // pp_0_1_lsb       | %14 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%13 : CtBlock);
                @015   // pp_0_1_msb       | %15 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%13 : CtBlock);
                @016   // pack_0_2         | %16 : CtBlock = pack_ct<4>(%2 : CtBlock, %8 : CtBlock);
                @017   // pp_0_2_lsb       | %17 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%16 : CtBlock);
                @018   // pp_0_2_msb       | %18 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%16 : CtBlock);
                @019   // pack_0_3         | %19 : CtBlock = pack_ct<4>(%2 : CtBlock, %9 : CtBlock);
                @020   // pp_0_3_lsb       | %20 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%19 : CtBlock);
                @021   // pp_0_3_msb       | %21 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%19 : CtBlock);
                @022   // pack_1_0         | %22 : CtBlock = pack_ct<4>(%3 : CtBlock, %6 : CtBlock);
                @023   // pp_1_0_lsb       | %23 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%22 : CtBlock);
                @024   // pp_1_0_msb       | %24 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%22 : CtBlock);
                @025   // pack_1_1         | %25 : CtBlock = pack_ct<4>(%3 : CtBlock, %7 : CtBlock);
                @026   // pp_1_1_lsb       | %26 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%25 : CtBlock);
                @027   // pp_1_1_msb       | %27 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%25 : CtBlock);
                @028   // pack_1_2         | %28 : CtBlock = pack_ct<4>(%3 : CtBlock, %8 : CtBlock);
                @029   // pp_1_2_lsb       | %29 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%28 : CtBlock);
                @030   // pp_1_2_msb       | %30 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%28 : CtBlock);
                @031   // ovf_1_3          | %31 : CtBlock = pack_ct<4>(%3 : CtBlock, %9 : CtBlock);
                @032   // ovf_1_3          | %32 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%31 : CtBlock);
                @033   // pack_2_0         | %33 : CtBlock = pack_ct<4>(%4 : CtBlock, %6 : CtBlock);
                @034   // pp_2_0_lsb       | %34 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%33 : CtBlock);
                @035   // pp_2_0_msb       | %35 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%33 : CtBlock);
                @036   // pack_2_1         | %36 : CtBlock = pack_ct<4>(%4 : CtBlock, %7 : CtBlock);
                @037   // pp_2_1_lsb       | %37 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%36 : CtBlock);
                @038   // pp_2_1_msb       | %38 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%36 : CtBlock);
                @039   // ovf_2_2          | %39 : CtBlock = pack_ct<4>(%4 : CtBlock, %8 : CtBlock);
                @040   // ovf_2_2          | %40 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%39 : CtBlock);
                @041   // ovf_2_3          | %41 : CtBlock = pack_ct<4>(%4 : CtBlock, %9 : CtBlock);
                @042   // ovf_2_3          | %42 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%41 : CtBlock);
                @043   // pack_3_0         | %43 : CtBlock = pack_ct<4>(%5 : CtBlock, %6 : CtBlock);
                @044   // pp_3_0_lsb       | %44 : CtBlock = pbs<Protect, MultCarryMsgLsb>(%43 : CtBlock);
                @045   // pp_3_0_msb       | %45 : CtBlock = pbs<Protect, MultCarryMsgMsb>(%43 : CtBlock);
                @046   // ovf_3_1          | %46 : CtBlock = pack_ct<4>(%5 : CtBlock, %7 : CtBlock);
                @047   // ovf_3_1          | %47 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%46 : CtBlock);
                @048   // ovf_3_2          | %48 : CtBlock = pack_ct<4>(%5 : CtBlock, %8 : CtBlock);
                @049   // ovf_3_2          | %49 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%48 : CtBlock);
                @050   // ovf_3_3          | %50 : CtBlock = pack_ct<4>(%5 : CtBlock, %9 : CtBlock);
                @051   // ovf_3_3          | %51 : CtBlock = pbs<Protect, MultCarryMsgIsSome>(%50 : CtBlock);
                @052   // reduction_1      | %52 : CtBlock = add_ct(%14 : CtBlock, %12 : CtBlock);
                @053   // reduction_1      | %53 : CtBlock = add_ct(%23 : CtBlock, %52 : CtBlock);
                @054   // reduction_1      | %54 : CtBlock = pbs<Protect, CarryInMsg>(%53 : CtBlock);
                @055   // reduction_1      | %55 : CtBlock = pbs<Protect, MsgOnly>(%53 : CtBlock);
                @056   // reduction_2      | %56 : CtBlock = add_ct(%17 : CtBlock, %15 : CtBlock);
                @057   // reduction_2      | %57 : CtBlock = add_ct(%24 : CtBlock, %56 : CtBlock);
                @058   // reduction_2      | %58 : CtBlock = add_ct(%26 : CtBlock, %57 : CtBlock);
                @059   // reduction_2      | %59 : CtBlock = add_ct(%34 : CtBlock, %58 : CtBlock);
                @060   // reduction_2      | %60 : CtBlock = pbs<Protect, CarryInMsg>(%59 : CtBlock);
                @061   // reduction_2      | %61 : CtBlock = pbs<Protect, MsgOnly>(%59 : CtBlock);
                @062   // reduction_2      | %62 : CtBlock = add_ct(%54 : CtBlock, %61 : CtBlock);
                @063   // reduction_2      | %63 : CtBlock = pbs<Protect, CarryInMsg>(%62 : CtBlock);
                @064   // reduction_2      | %64 : CtBlock = pbs<Protect, MsgOnly>(%62 : CtBlock);
                @065                       | %65 : CtBlock = transfer(%60 : CtBlock);
                @066                       | %66 : CtBlock = transfer(%63 : CtBlock);
                @067   // reduction_3      | %67 : CtBlock = transfer(%18 : CtBlock);
                @068   // reduction_3      | %68 : CtBlock = transfer(%27 : CtBlock);
                @069   // reduction_3      | %69 : CtBlock = transfer(%35 : CtBlock);
                @070   // reduction_3      | %70 : CtBlock = add_ct(%20 : CtBlock, %67 : CtBlock);
                @071   // reduction_3      | %71 : CtBlock = add_ct(%68 : CtBlock, %70 : CtBlock);
                @072   // reduction_3      | %72 : CtBlock = add_ct(%29 : CtBlock, %71 : CtBlock);
                @073   // reduction_3      | %73 : CtBlock = add_ct(%69 : CtBlock, %72 : CtBlock);
                @074   // reduction_3      | %74 : CtBlock = pbs<Protect, CarryInMsg>(%73 : CtBlock);
                @075   // reduction_3      | %75 : CtBlock = pbs<Protect, MsgOnly>(%73 : CtBlock);
                @076   // reduction_3      | %76 : CtBlock = add_ct(%37 : CtBlock, %75 : CtBlock);
                @077   // reduction_3      | %77 : CtBlock = add_ct(%44 : CtBlock, %76 : CtBlock);
                @078   // reduction_3      | %78 : CtBlock = add_ct(%65 : CtBlock, %77 : CtBlock);
                @079   // reduction_3      | %79 : CtBlock = add_ct(%66 : CtBlock, %78 : CtBlock);
                @080   // reduction_3      | %80 : CtBlock = pbs<Protect, CarryInMsg>(%79 : CtBlock);
                @081   // reduction_3      | %81 : CtBlock = pbs<Protect, MsgOnly>(%79 : CtBlock);
                @082   // ovf / post_map   | %82 : CtBlock = add_ct(%74 : CtBlock, %80 : CtBlock);
                @083   // ovf / post_map   | %83 : CtBlock = add_ct(%82 : CtBlock, %21 : CtBlock);
                @084   // ovf / post_map   | %84 : CtBlock = add_ct(%83 : CtBlock, %30 : CtBlock);
                @085   // ovf / post_map   | %85 : CtBlock = add_ct(%84 : CtBlock, %38 : CtBlock);
                @086   // ovf / post_map   | %86 : CtBlock = pbs<Protect, IsSome>(%85 : CtBlock);
                @087   // ovf / post_map   | %87 : CtBlock = pbs<Protect, IsSome>(%45 : CtBlock);
                @088   // ovf / merge      | %88 : CtBlock = add_ct(%32 : CtBlock, %40 : CtBlock);
                @089   // ovf / merge      | %89 : CtBlock = add_ct(%88 : CtBlock, %42 : CtBlock);
                @090   // ovf / merge      | %90 : CtBlock = add_ct(%89 : CtBlock, %86 : CtBlock);
                @091   // ovf / merge      | %91 : CtBlock = add_ct(%90 : CtBlock, %87 : CtBlock);
                @092   // ovf / merge      | %92 : CtBlock = pbs<Protect, IsSome>(%91 : CtBlock);
                @093   // ovf / merge      | %93 : CtBlock = add_ct(%47 : CtBlock, %49 : CtBlock);
                @094   // ovf / merge      | %94 : CtBlock = add_ct(%93 : CtBlock, %51 : CtBlock);
                @095   // ovf / merge      | %95 : CtBlock = pbs<Protect, IsSome>(%94 : CtBlock);
                @096                       | %96 : CtBlock = transfer(%95 : CtBlock);
                @097   // ovf / merge      | %97 : CtBlock = add_ct(%92 : CtBlock, %96 : CtBlock);
                @098   // ovf / merge      | %98 : CtBlock = pbs<Protect, IsSome>(%97 : CtBlock);
                @099                       | %99 : Ct = decl_ct<8>();
                @100                       | %100 : Ct = store_ct_block<0>(%11 : CtBlock, %99 : Ct);
                @101                       | %101 : Ct = store_ct_block<1>(%55 : CtBlock, %100 : Ct);
                @102                       | %102 : Ct = store_ct_block<2>(%64 : CtBlock, %101 : Ct);
                @103                       | %103 : Ct = store_ct_block<3>(%81 : CtBlock, %102 : Ct);
                @104                       | output<0>(%103 : Ct);
            "#
        );
    }
}
