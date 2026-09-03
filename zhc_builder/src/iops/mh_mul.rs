use std::collections::BTreeMap;

use crate::{CiphertextBlock, NU, NU_BOOL, builder::Builder};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::SafeAs;

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
/// # use zhc_builder::{CiphertextSpec, mh_mul};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = mh_mul(spec, 2);
/// let ir = builder.optimize_ir();
/// ```
pub fn mh_mul(spec: CiphertextSpec, split_depth: usize) -> Builder {
    mh_mul_with_opt(spec, split_depth, false)
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
/// # use zhc_builder::{CiphertextSpec, mh_overflow_mul_lsb};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = mh_overflow_mul_lsb(spec, 2);
/// let ir = builder.optimize_ir();
/// ```
pub fn mh_overflow_mul_lsb(spec: CiphertextSpec, split_depth: usize) -> Builder {
    mh_mul_with_opt(spec, split_depth, true)
}

/// Creates an IR for a multiplication of two encrypted integers split into mh_factor sub-graph.
///
/// The returned [`Builder`] declares two ciphertext inputs and two ciphertext outputs.
/// First output is an *Optional* overflow flag, second one is the LSB part of the input product
///
/// Internally delegates to [`Builder::limb_mul_chain`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// This function is private; reach it through [`mh_mul`] (`gen_overflow = false`) or
/// [`mh_overflow_mul_lsb`] (`gen_overflow = true`):
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, mh_overflow_mul_lsb};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = mh_overflow_mul_lsb(spec, 2);
/// let ir = builder.optimize_ir();
/// ```
fn mh_mul_with_opt(spec: CiphertextSpec, split_depth: usize, gen_overflow: bool) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());

    // Get input as array of blk
    let src_a_blocks = builder.ciphertext_split(&src_a);
    let src_b_blocks = builder.ciphertext_split(&src_b);
    // Only kept LSB to obtain a IxI -> I operations
    let cut_off = spec.block_count() as usize;

    // Call inner function and construct results
    let (flag_block, outputs) =
        builder.mh_iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off, split_depth);

    if gen_overflow {
        let flag = builder.ciphertext_join(&[flag_block], Some(1)); // NB: This is a boolean flag
        builder.ciphertext_output(flag);
    }
    // View output as one
    let pack_output = outputs.into_iter().flatten().collect::<Vec<_>>();
    let output = builder.ciphertext_join(&pack_output, Some(spec.int_size()));
    builder.ciphertext_output(output);

    builder
}

// Describe limb of ciphertext
// Used to describe N-size arithmetic in p limb of (N/p)-size
// ALso contain some metadata to ease explicit xfer addition
#[derive(Debug, Default, Clone)]
struct CiphertextLimb {
    offset: usize,
    blocks: Vec<CiphertextBlock>,
}

impl From<CiphertextLimb> for Vec<CiphertextBlock> {
    fn from(value: CiphertextLimb) -> Self {
        value.blocks
    }
}
impl CiphertextLimb {
    fn new(offset: usize, blks: &[CiphertextBlock]) -> Self {
        let mut blocks = Vec::with_capacity(blks.len());
        blocks.extend_from_slice(blks);
        Self { offset, blocks }
    }

    fn chunks(blks: &[CiphertextBlock], chunk_size: usize) -> Vec<Self> {
        Self::chunks_at(0, blks, chunk_size)
    }

    fn chunks_at(offset: usize, blks: &[CiphertextBlock], chunk_size: usize) -> Vec<Self> {
        blks.chunks(chunk_size)
            .enumerate()
            .map(|(ofst, b)| CiphertextLimb::new(offset + ofst, b))
            .collect::<Vec<_>>()
    }

    fn as_blocks(&self) -> &[CiphertextBlock] {
        &self.blocks
    }
    fn into_blocks(self) -> Vec<CiphertextBlock> {
        self.blocks
    }
}

impl Builder {
    /// Multiply two ciphertext in a raw fashion.
    ///
    /// Use schoolbook implementation based on smaller size sequential mul.
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
    /// let (flag, res) = builder.mh_iop_mul_raw(&a, &b, spec.block_count() as usize, 2);
    /// ```
    pub fn mh_iop_mul_raw(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_off_block: usize,
        split_depth: usize,
    ) -> (CiphertextBlock, Vec<Vec<CiphertextBlock>>) {
        // Compute split structure
        let blocks = if src_a_blocks.len() == src_b_blocks.len() {
            src_a_blocks.len()
        } else {
            panic!("Error: current split only work with symetrics operands");
        };

        let mh_blocks = if 0 == (blocks % split_depth) {
            blocks / split_depth
        } else {
            panic!("Error: current split only work when blocks is a muliple of mh_factor");
        };
        let limbs_size = (mh_blocks * src_a_blocks[0].spec().message_size() as usize) as u16;

        let mh_a_blocks = CiphertextLimb::chunks(src_a_blocks, mh_blocks);
        let mh_b_blocks = CiphertextLimb::chunks(src_b_blocks, mh_blocks);
        // Phase 1:
        // Compute each limb sub-mul and dispatch output
        let mut limb_map = BTreeMap::<usize, Vec<CiphertextLimb>>::new();
        let mut overflow_v = Vec::<CiphertextBlock>::new();

        for (i, ai) in mh_a_blocks
            .iter()
            .map(|CiphertextLimb { offset, blocks }| (offset, blocks))
        {
            for (j, bj) in mh_b_blocks
                .iter()
                .map(|CiphertextLimb { offset, blocks }| (offset, blocks))
            {
                // Compute cut_range point based on input and current limb offset
                let blocks_ofst = (i + j) * mh_blocks;
                let relin_cut_off = std::cmp::min(
                    2 * mh_blocks, // mul generate at most 2x input_width,
                    cut_off_block.saturating_sub(blocks_ofst),
                );

                let ovf = if relin_cut_off > mh_blocks {
                    // Compute in two half
                    self.new_partition(format!("Pp[{i}::{j}]_lsb @{{{}}}", i + j));

                    let (lsb_res, _ovf, lsb_cout) = self
                        .comment(format!("SubMul[{i}][{j}]_lsb"))
                        .limb_mul_chain(ai, bj, (0, mh_blocks), vec![]);
                    let lsb_limb = CiphertextLimb::new(i + j, &lsb_res);
                    limb_map.entry(lsb_limb.offset).or_default().push(lsb_limb);

                    self.new_partition(format!("Pp[{i}::{j}]_msb @{{{}}}", i + j + 1));

                    let (msb_res, ovf, _cout) = self
                        .comment(format!("SubMul[{i}][{j}]_msb"))
                        .limb_mul_chain(ai, bj, (mh_blocks, relin_cut_off), lsb_cout);
                    if !msb_res.is_empty() {
                        let msb_limb = CiphertextLimb::new(i + j + 1, &msb_res);
                        limb_map.entry(msb_limb.offset).or_default().push(msb_limb);
                    }
                    ovf
                } else {
                    self.new_partition(format!("Pp[{i}::{j}] @{{{}}}", i + j));

                    let (cur_res, ovf, _cout) = self
                        .comment(format!("SubMul[{i}][{j}]"))
                        .limb_mul_chain(ai, bj, (0, relin_cut_off), vec![]);
                    if !cur_res.is_empty() {
                        let cur_limb = CiphertextLimb::new(i + j, &cur_res);
                        limb_map.entry(cur_limb.offset).or_default().push(cur_limb);
                    }
                    ovf
                };
                overflow_v.push(ovf);
            }
        }

        // Phase 2 Reduce/Merge:
        // Fuse each limb with (mh_block +1)W adder
        let first_limb_id = limb_map.keys().min().copied().unwrap_or_default();
        let last_limb_id = std::cmp::min(
            cut_off_block as usize,
            limb_map.keys().max().copied().unwrap_or_default(),
        );

        // At each step there is a list of limb to sum and a list of input_carry
        // Gather value through add-tree and consume one input_carry at each stage
        // Each stage output one value and a vector of carry_out
        let mut dst_limb = vec![Default::default(); split_depth];
        let mut carry_buffer = BTreeMap::<usize, Vec<CiphertextBlock>>::new();
        for k in first_limb_id..=last_limb_id {
            let mut stage_limb = limb_map.remove(&k).unwrap_or_default();
            let mut carry_in = carry_buffer.remove(&k).unwrap_or_default();

            self.push_comment(format!("Limb reduce[{k}]"));
            if stage_limb.len() > 1 {
                // Tree-like reduction
                let mut tree_iter = 0;
                while stage_limb.len() > 1 {
                    let mut current = stage_limb.into_iter();
                    let mut next = Vec::new();

                    loop {
                        match (current.next(), current.next()) {
                            (Some(a), Some(b)) => {
                                self.new_partition(format!("LimbRed[{tree_iter}] @{{{k}}}"));
                                let (sum, cout) =
                                    self.comment(format!("iter {tree_iter}")).iop_add_raw(
                                        limbs_size,
                                        a.as_blocks(),
                                        b.as_blocks(),
                                        carry_in.pop().as_ref(),
                                    );
                                next.push(CiphertextLimb::new(k, &sum));
                                carry_buffer.entry(k + 1).or_default().push(cout)
                            }
                            (Some(a), None) => {
                                // odd element passes through unchanged
                                next.push(a);
                                break;
                            }
                            _ => break,
                        }
                    }
                    tree_iter += 1;
                    stage_limb = next;
                }
            }
            dst_limb[k] = stage_limb
                .pop()
                .expect("A stage must contain at least 1 limb")
                .into_blocks();
            self.pop_comment();
        }

        // Phase 3
        // Reduce overflow
        let out_of_range_limb = limb_map
            .into_values()
            .flatten()
            .flat_map(|CiphertextLimb { offset: _, blocks }| blocks)
            .collect::<Vec<_>>();
        let post_carry = carry_buffer.into_values().flatten().collect::<Vec<_>>();

        self.new_partition("OverflowReduction");
        let ovf_flag = self.merge_overflow_flag(out_of_range_limb, post_carry);

        self.new_partition("Outputs");
        (ovf_flag, dst_limb)
    }

    /// Overflow Reduction
    /// Extract overflow flag of post_map carry and overflow_v
    fn merge_overflow_flag(
        &self,
        out_of_range: Vec<CiphertextBlock>,
        carry_v: Vec<CiphertextBlock>,
    ) -> CiphertextBlock {
        let mut overflow_v = carry_v;
        self.push_comment(format!("Limb_ovf"));

        // Start by handling out_of_scope value
        // NB: they are full-fledge (i.e. could take all message space)
        self.push_comment(format!("oor"));
        for chunk in out_of_range.chunks(NU) {
            let mut chunk_iter = chunk.iter();
            let init = *chunk_iter.next().unwrap();
            let chunk_sum = chunk_iter.fold(init, |acc, v| self.block_add(&acc, v));
            let is_some_flag = self.block_lookup(chunk_sum, Lut1Def::IsSome);
            overflow_v.push(is_some_flag);
        }
        self.pop_comment();

        // Continue with overflow merge
        // NB: overflow value are boolean only
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
        overflow_flag
    }

    /// Multiply two ciphertext in a raw fashion.
    /// I.e. Compute all output between cut-in a cut-off point. After cut-off point only
    /// overflow flag status is computed.
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
    /// This implementation also return a vector of direct carry,
    /// and support of list of input carry.
    /// The aims is to chain them while keeping a fine control over partition
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
    pub fn limb_mul_chain(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_range: (usize, usize), //(in,out)
        carry_in: Vec<CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, CiphertextBlock, Vec<CiphertextBlock>) {
        // Phase 1 expand:
        // It's a cartesien product of a and b for each terms we sort them by degree
        // (i.e. ai +bi) and kept assocatied nu for the later reduction
        // NB: nu encode range of data. nu*(1<<msg_w) = Max Ct value
        // After the cut-off block only NonNull flag is computed instead of the complete partial
        // product with carry extract
        let mut partial_product_map = BTreeMap::<usize, Vec<CiphertextBlock>>::new();

        let mut overflow_v = Vec::<CiphertextBlock>::new();
        let (cut_in, cut_off) = cut_range;
        // Inject carry in if any
        for cin in carry_in.into_iter() {
            partial_product_map.entry(cut_in).or_default().push(cin);
        }
        assert!(
            cut_off >= cut_in,
            "Invalid cut_range definition [cut_in, cut_out] -> {cut_range:?}, check bounds definition"
        );

        for (i, ai) in src_a_blocks.iter().enumerate() {
            for (j, bj) in src_b_blocks.iter().enumerate() {
                if ((i + j) >= cut_in) && ((i + j) < cut_off) {
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
        for k in cut_in..cut_off {
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
        let carry_out = if let Some(in_carry_v) = partial_product_map.remove(&(cut_off.sas())) {
            for chunk in in_carry_v.chunks(NU) {
                let mut chunk_iter = chunk.iter();
                let init = *chunk_iter.next().unwrap();
                let chunk_sum = chunk_iter.fold(init, |acc, v| self.block_add(&acc, v));
                let is_some_flag = self.block_lookup(chunk_sum, Lut1Def::IsSome);
                overflow_v.push(is_some_flag);
            }
            in_carry_v
        } else {
            vec![]
        };
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

        (dst_blk, overflow_flag, carry_out)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::CiphertextSpec;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    const SPLIT_DEPTH: [usize; 2] = [2, 4];

    #[test]
    fn correctness_mh_mul() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.mul_lsb(*rhs))])
        }
        for split_depth in SPLIT_DEPTH.iter() {
            for size in (4 * *split_depth as u16..64).step_by(2 * *split_depth as usize) {
                mh_mul(CiphertextSpec::new(size, 2, 2), *split_depth).test_random(100, semantic);
            }
        }
    }

    #[test]
    fn test_mh_mul() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let ir = mh_mul(spec, 2);
        assert_display_is!(
            ir.ir()
                .format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_opid(true),
            r#"
                @0                                            | %0 = input_ciphertext<0, 8>();
                @1                                            | %1 = input_ciphertext<1, 8>();
                @2                                            | %2 = extract_ct_block<0>(%0);
                @3                                            | %3 = extract_ct_block<1>(%0);
                @4                                            | %4 = extract_ct_block<2>(%0);
                @5                                            | %5 = extract_ct_block<3>(%0);
                @6                                            | %6 = extract_ct_block<0>(%1);
                @7                                            | %7 = extract_ct_block<1>(%1);
                @8                                            | %8 = extract_ct_block<2>(%1);
                @9                                            | %9 = extract_ct_block<3>(%1);
                @10    // SubMul[0][0]_lsb / pack_0_0         | %10 = pack_ct<4>(%2, %6);
                @11    // SubMul[0][0]_lsb / pp_0_0_lsb       | %11 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%10);
                @12    // SubMul[0][0]_lsb / pp_0_0_msb       | %12 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%10);
                @13    // SubMul[0][0]_lsb / pack_0_1         | %13 = pack_ct<4>(%2, %7);
                @14    // SubMul[0][0]_lsb / pp_0_1_lsb       | %14 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%13);
                @15    // SubMul[0][0]_lsb / pp_0_1_msb       | %15 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%13);
                @16    // SubMul[0][0]_lsb / pack_1_0         | %16 = pack_ct<4>(%3, %6);
                @17    // SubMul[0][0]_lsb / pp_1_0_lsb       | %17 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%16);
                @18    // SubMul[0][0]_lsb / pp_1_0_msb       | %18 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%16);
                @19    // SubMul[0][0]_lsb / ovf_1_1          | %19 = pack_ct<4>(%3, %7);
                @20    // SubMul[0][0]_lsb / ovf_1_1          | %20 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%19);
                @21    // SubMul[0][0]_lsb / reduction_1      | %21 = add_ct(%14, %12);
                @22    // SubMul[0][0]_lsb / reduction_1      | %22 = add_ct(%17, %21);
                @23    // SubMul[0][0]_lsb / reduction_1      | %23 = pbs<Protect, Lut1("CarryInMsg")>(%22);
                @24    // SubMul[0][0]_lsb / reduction_1      | %24 = pbs<Protect, Lut1("MsgOnly")>(%22);
                @25    // SubMul[0][0]_lsb / ovf / carry_in   | %25 = add_ct(%15, %18);
                @26    // SubMul[0][0]_lsb / ovf / carry_in   | %26 = add_ct(%25, %23);
                @27    // SubMul[0][0]_lsb / ovf / carry_in   | %27 = pbs<Protect, Lut1("IsSome")>(%26);
                @28    // SubMul[0][0]_lsb / ovf / merge      | %28 = add_ct(%20, %27);
                @29    // SubMul[0][0]_lsb / ovf / merge      | %29 = pbs<Protect, Lut1("IsSome")>(%28);
                @30    // SubMul[0][0]_msb / ovf_0_0          | %30 = pack_ct<4>(%2, %6);
                @31    // SubMul[0][0]_msb / ovf_0_0          | %31 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%30);
                @32    // SubMul[0][0]_msb / ovf_0_1          | %32 = pack_ct<4>(%2, %7);
                @33    // SubMul[0][0]_msb / ovf_0_1          | %33 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%32);
                @34    // SubMul[0][0]_msb / ovf_1_0          | %34 = pack_ct<4>(%3, %6);
                @35    // SubMul[0][0]_msb / ovf_1_0          | %35 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%34);
                @36    // SubMul[0][0]_msb / pack_1_1         | %36 = pack_ct<4>(%3, %7);
                @37    // SubMul[0][0]_msb / pp_1_1_lsb       | %37 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%36);
                @38    // SubMul[0][0]_msb / pp_1_1_msb       | %38 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%36);
                @39    // SubMul[0][0]_msb / reduction_2      | %39 = add_ct(%18, %15);
                @40    // SubMul[0][0]_msb / reduction_2      | %40 = add_ct(%23, %39);
                @41    // SubMul[0][0]_msb / reduction_2      | %41 = add_ct(%37, %40);
                @42    // SubMul[0][0]_msb / reduction_2      | %42 = pbs<Protect, Lut1("CarryInMsg")>(%41);
                @43    // SubMul[0][0]_msb / reduction_2      | %43 = pbs<Protect, Lut1("MsgOnly")>(%41);
                @44    // SubMul[0][0]_msb / reduction_3      | %44 = add_ct(%42, %38);
                @45    // SubMul[0][0]_msb / reduction_3      | %45 = pbs<Protect, Lut1("CarryInMsg")>(%44);
                @46    // SubMul[0][0]_msb / reduction_3      | %46 = pbs<Protect, Lut1("MsgOnly")>(%44);
                @47    // SubMul[0][0]_msb / ovf / carry_in   | %47 = pbs<Protect, Lut1("IsSome")>(%45);
                @48    // SubMul[0][0]_msb / ovf / merge      | %48 = add_ct(%31, %33);
                @49    // SubMul[0][0]_msb / ovf / merge      | %49 = add_ct(%48, %35);
                @50    // SubMul[0][0]_msb / ovf / merge      | %50 = add_ct(%49, %47);
                @51    // SubMul[0][0]_msb / ovf / merge      | %51 = pbs<Protect, Lut1("IsSome")>(%50);
                @52    // SubMul[0][1] / pack_0_0             | %52 = pack_ct<4>(%2, %8);
                @53    // SubMul[0][1] / pp_0_0_lsb           | %53 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%52);
                @54    // SubMul[0][1] / pp_0_0_msb           | %54 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%52);
                @55    // SubMul[0][1] / pack_0_1             | %55 = pack_ct<4>(%2, %9);
                @56    // SubMul[0][1] / pp_0_1_lsb           | %56 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%55);
                @57    // SubMul[0][1] / pp_0_1_msb           | %57 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%55);
                @58    // SubMul[0][1] / pack_1_0             | %58 = pack_ct<4>(%3, %8);
                @59    // SubMul[0][1] / pp_1_0_lsb           | %59 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%58);
                @60    // SubMul[0][1] / pp_1_0_msb           | %60 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%58);
                @61    // SubMul[0][1] / ovf_1_1              | %61 = pack_ct<4>(%3, %9);
                @62    // SubMul[0][1] / ovf_1_1              | %62 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%61);
                @63    // SubMul[0][1] / reduction_1          | %63 = add_ct(%56, %54);
                @64    // SubMul[0][1] / reduction_1          | %64 = add_ct(%59, %63);
                @65    // SubMul[0][1] / reduction_1          | %65 = pbs<Protect, Lut1("CarryInMsg")>(%64);
                @66    // SubMul[0][1] / reduction_1          | %66 = pbs<Protect, Lut1("MsgOnly")>(%64);
                @67    // SubMul[0][1] / ovf / carry_in       | %67 = add_ct(%57, %60);
                @68    // SubMul[0][1] / ovf / carry_in       | %68 = add_ct(%67, %65);
                @69    // SubMul[0][1] / ovf / carry_in       | %69 = pbs<Protect, Lut1("IsSome")>(%68);
                @70    // SubMul[0][1] / ovf / merge          | %70 = add_ct(%62, %69);
                @71    // SubMul[0][1] / ovf / merge          | %71 = pbs<Protect, Lut1("IsSome")>(%70);
                @72    // SubMul[1][0] / pack_0_0             | %72 = pack_ct<4>(%4, %6);
                @73    // SubMul[1][0] / pp_0_0_lsb           | %73 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%72);
                @74    // SubMul[1][0] / pp_0_0_msb           | %74 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%72);
                @75    // SubMul[1][0] / pack_0_1             | %75 = pack_ct<4>(%4, %7);
                @76    // SubMul[1][0] / pp_0_1_lsb           | %76 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%75);
                @77    // SubMul[1][0] / pp_0_1_msb           | %77 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%75);
                @78    // SubMul[1][0] / pack_1_0             | %78 = pack_ct<4>(%5, %6);
                @79    // SubMul[1][0] / pp_1_0_lsb           | %79 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%78);
                @80    // SubMul[1][0] / pp_1_0_msb           | %80 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%78);
                @81    // SubMul[1][0] / ovf_1_1              | %81 = pack_ct<4>(%5, %7);
                @82    // SubMul[1][0] / ovf_1_1              | %82 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%81);
                @83    // SubMul[1][0] / reduction_1          | %83 = add_ct(%76, %74);
                @84    // SubMul[1][0] / reduction_1          | %84 = add_ct(%79, %83);
                @85    // SubMul[1][0] / reduction_1          | %85 = pbs<Protect, Lut1("CarryInMsg")>(%84);
                @86    // SubMul[1][0] / reduction_1          | %86 = pbs<Protect, Lut1("MsgOnly")>(%84);
                @87    // SubMul[1][0] / ovf / carry_in       | %87 = add_ct(%77, %80);
                @88    // SubMul[1][0] / ovf / carry_in       | %88 = add_ct(%87, %85);
                @89    // SubMul[1][0] / ovf / carry_in       | %89 = pbs<Protect, Lut1("IsSome")>(%88);
                @90    // SubMul[1][0] / ovf / merge          | %90 = add_ct(%82, %89);
                @91    // SubMul[1][0] / ovf / merge          | %91 = pbs<Protect, Lut1("IsSome")>(%90);
                @92    // SubMul[1][1] / ovf_0_0              | %92 = pack_ct<4>(%4, %8);
                @93    // SubMul[1][1] / ovf_0_0              | %93 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%92);
                @94    // SubMul[1][1] / ovf_0_1              | %94 = pack_ct<4>(%4, %9);
                @95    // SubMul[1][1] / ovf_0_1              | %95 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%94);
                @96    // SubMul[1][1] / ovf_1_0              | %96 = pack_ct<4>(%5, %8);
                @97    // SubMul[1][1] / ovf_1_0              | %97 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%96);
                @98    // SubMul[1][1] / ovf_1_1              | %98 = pack_ct<4>(%5, %9);
                @99    // SubMul[1][1] / ovf_1_1              | %99 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%98);
                @100   // SubMul[1][1] / ovf / merge          | %100 = add_ct(%93, %95);
                @101   // SubMul[1][1] / ovf / merge          | %101 = add_ct(%100, %97);
                @102   // SubMul[1][1] / ovf / merge          | %102 = add_ct(%101, %99);
                @103   // SubMul[1][1] / ovf / merge          | %103 = pbs<Protect, Lut1("IsSome")>(%102);
                @104   // Limb reduce[1] / iter 0             | %104 = let_ct_block<0>();
                @105   // Limb reduce[1] / iter 0 / 0-th      | %105 = add_ct(%43, %53);
                @106   // Limb reduce[1] / iter 0 / 0-th      | %106 = add_ct(%105, %104);
                @107   // Limb reduce[1] / iter 0 / 0-th      | %107, %108 = pbs2<Protect, Lut2("ManyCarryMsg")>(%106);
                @108   // Limb reduce[1] / iter 0 / 1-th      | %109 = add_ct(%46, %66);
                @109   // Limb reduce[1] / iter 0 / 1-th      | %110 = add_ct(%109, %108);
                @110   // Limb reduce[1] / iter 0 / 1-th      | %111, %112 = pbs2<Protect, Lut2("ManyCarryMsg")>(%110);
                @111   // Limb reduce[1] / iter 1             | %113 = let_ct_block<0>();
                @112   // Limb reduce[1] / iter 1 / 0-th      | %114 = add_ct(%107, %73);
                @113   // Limb reduce[1] / iter 1 / 0-th      | %115 = add_ct(%114, %113);
                @114   // Limb reduce[1] / iter 1 / 0-th      | %116, %117 = pbs2<Protect, Lut2("ManyCarryMsg")>(%115);
                @115   // Limb reduce[1] / iter 1 / 1-th      | %118 = add_ct(%111, %86);
                @116   // Limb reduce[1] / iter 1 / 1-th      | %119 = add_ct(%118, %117);
                @117   // Limb reduce[1] / iter 1 / 1-th      | %120, %121 = pbs2<Protect, Lut2("ManyCarryMsg")>(%119);
                @118   // Limb_ovf / merge                    | %122 = add_ct(%112, %121);
                @119   // Limb_ovf / merge                    | %123 = pbs<Protect, Lut1("IsSome")>(%122);
                @120                                          | %124 = decl_ct<8>();
                @121                                          | %125 = let_ct_block<0>();
                @122                                          | %126 = store_ct_block<0>(%125, %124);
                @123                                          | %127 = store_ct_block<1>(%125, %126);
                @124                                          | %128 = store_ct_block<2>(%125, %127);
                @125                                          | %129 = store_ct_block<3>(%125, %128);
                @126                                          | %130 = store_ct_block<0>(%11, %129);
                @127                                          | %131 = store_ct_block<1>(%24, %130);
                @128                                          | %132 = store_ct_block<2>(%116, %131);
                @129                                          | %133 = store_ct_block<3>(%120, %132);
                @130                                          | output<0>(%133);
            "#
        );
    }

    #[test]
    fn noise_mh_mul() {
        for split_depth in SPLIT_DEPTH.iter() {
            for size in (4 * *split_depth as u16..64).step_by(2 * *split_depth as usize) {
                mh_mul(CiphertextSpec::new(size, 2, 2), *split_depth).check_noise();
            }
        }
    }
}
