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
/// let builder = mh_mul(spec, 2);
/// let ir = builder.into_ir();
/// ```
pub fn mh_mul(spec: CiphertextSpec, mh_factor: u8) -> Builder {
    mh_mul_with_opt(spec, mh_factor, false)
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
    mh_mul_with_opt(spec, mh_factor, true)
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
/// let builder = mh_mul_with_opt(spec, 2, true);
/// let ir = builder.into_ir();
/// ```
fn mh_mul_with_opt(spec: CiphertextSpec, mh_factor: u8, gen_overflow: bool) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());

    // Get input as array of blk
    let src_a_blocks = builder.ciphertext_split(&src_a);
    let src_b_blocks = builder.ciphertext_split(&src_b);
    // Only kept LSB to obtain a IxI -> I operations
    let cut_off = spec.block_count();

    // Call inner function and construct results
    let (flag_block, outputs) =
        builder.mh_iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off, mh_factor);

    if gen_overflow {
        let flag = builder.ciphertext_join(&[flag_block], Some(1)); // NB: This is a boolean flag
        builder.ciphertext_output(flag);
    }
    // View output as one
    // // let pack_output = outputs.into_iter().flatten().collect::<Vec<_>>();
    // // let output = builder.join_ciphertext(&pack_output, Some(spec.int_size()));
    // builder.output_ciphertext(output);

    // View output as mh_factor sub-part
    for out in outputs.into_iter() {
        let output = builder.ciphertext_join(
            &out,
            Some(out.len() as u16 * spec.block_spec().message_size() as u16),
        );
        builder.ciphertext_output(output);
    }

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
    /// # let a = builder.input_ciphertext(spec.int_size());
    /// # let b = builder.input_ciphertext(spec.int_size());
    /// # let a = builder.split_ciphertext(&a);
    /// # let b = builder.split_ciphertext(&b);
    /// let (flag, res) = builder.iop_mul_raw(&a, &b, spec.block_count());
    /// ```
    pub fn mh_iop_mul_raw(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_off_block: u8,
        mh_factor: u8,
    ) -> (CiphertextBlock, Vec<Vec<CiphertextBlock>>) {
        // Compute split structure
        let blocks = if src_a_blocks.len() == src_b_blocks.len() {
            src_a_blocks.len()
        } else {
            panic!("Error: current split only work with symetrics operands");
        };

        let mh_blocks = if 0 == (blocks % mh_factor as usize) {
            blocks / mh_factor as usize
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
                self.new_partition();

                // Compute cut_off point based on input and current limb offset
                let blocks_ofst = (i + j) * mh_blocks;
                let relin_cut_off = cut_off_block.saturating_sub(blocks_ofst as u8);

                // Call sub-size mul
                let (sm_res, ovf) =
                    self.comment(format!("SubMul[{i}][{j}]"))
                        .iop_mul_raw(ai, bj, relin_cut_off);

                // Spread output
                for limb in CiphertextLimb::chunks_at(i + j, &sm_res, mh_blocks) {
                    limb_map.entry(limb.offset).or_default().push(limb);
                }
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
        let mut dst_limb = vec![Default::default(); mh_factor as usize];
        let mut carry_buffer = BTreeMap::<usize, Vec<CiphertextBlock>>::new();
        for k in first_limb_id..=last_limb_id {
            self.new_partition();
            self.push_comment(format!("Limb reduce[{k}]"));
            let mut stage_limb = limb_map.remove(&k).unwrap_or_default();
            let mut carry_in = carry_buffer.remove(&k).unwrap_or_default();

            // TODO add explicit cut at correct place
            // let xfer_first = {
            //     let xfer = first
            //         .blocks
            //         .into_iter()
            //         .map(|b| self.block_transfer(b))
            //         .collect::<Vec<_>>();
            //     CiphertextLimb::new(k, &xfer)

            // Tree-like reduction
            let mut tree_iter = 0;
            while stage_limb.len() > 1 {
                let mut current = stage_limb.into_iter();
                let mut next = Vec::new();

                loop {
                    match (current.next(), current.next()) {
                        (Some(a), Some(b)) => {
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
                    tree_iter += 1;
                }
                stage_limb = next;
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

        self.new_partition();
        let ovf_flag = self.merge_overflow_flag(out_of_range_limb, post_carry);
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
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::{CiphertextSpec, EmulatedCiphertext};
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::{Dumpable, assert_display_is};

    const MH_FACTOR: u8 = 4;

    #[test]
    fn correctness_mh_mul() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            let res = lhs.mul_lsb(*rhs);
            let res_raw = res.as_storage();
            let int_size = res.spec().int_size();
            let mh_bits = int_size / MH_FACTOR as u16;
            let mh_mask = !(0x1 << mh_bits);
            let mh_spec = CiphertextSpec::new(
                mh_bits,
                res.spec().block_spec().carry_size(),
                res.spec().block_spec().message_size(),
            );
            let mut res_split = Vec::with_capacity(MH_FACTOR as usize);

            for i in 0..MH_FACTOR {
                let split_raw = (res_raw >> (i as u16 * mh_bits)) & mh_mask;
                let split_emu = EmulatedCiphertext::new(split_raw, mh_spec);

                res_split.push(IopValue::Ciphertext(split_emu));
            }
            Some(res_split)
        }
        for size in (4 * MH_FACTOR as u16..64).step_by(2 * MH_FACTOR as usize) {
            mh_mul(CiphertextSpec::new(size, 2, 2), MH_FACTOR).test_random(100, semantic);
        }
    }

    // #[test]
    // fn correctness_mh_overflow_mul() {
    //     fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
    //         let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
    //             unreachable!()
    //         };
    //         vec![IopValue::Ciphertext(lhs.mul(*rhs))]
    //     }
    //     for size in (2..128).step_by(2) {
    //         mh_mul(CiphertextSpec::new(size, 2, 2), MH_FACTOR).test_random(100, semantic);
    //     }
    // }

    #[test]
    fn test_mh_mul() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let ir = mh_mul(spec, MH_FACTOR);
        assert_display_is!(
            ir.ir()
                .format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_opid(true),
            r#"
                @0                                                | %0 = input_ciphertext<0, 8>();
                @1                                                | %1 = input_ciphertext<1, 8>();
                @2                                                | %2 = extract_ct_block<0>(%0);
                @3                                                | %3 = extract_ct_block<1>(%0);
                @4                                                | %4 = extract_ct_block<2>(%0);
                @5                                                | %5 = extract_ct_block<3>(%0);
                @6                                                | %6 = extract_ct_block<0>(%1);
                @7                                                | %7 = extract_ct_block<1>(%1);
                @8                                                | %8 = extract_ct_block<2>(%1);
                @9                                                | %9 = extract_ct_block<3>(%1);
                @10    // SubMul[0][0] / pack_0_0                 | %10 = pack_ct<4>(%2, %6);
                @11    // SubMul[0][0] / pp_0_0_lsb               | %11 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%10);
                @12    // SubMul[0][0] / pp_0_0_msb               | %12 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%10);
                @13    // SubMul[0][0] / ovf / merge              | %13 = let_ct_block<0>();
                @14    // SubMul[0][1] / pack_0_0                 | %14 = pack_ct<4>(%2, %7);
                @15    // SubMul[0][1] / pp_0_0_lsb               | %15 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%14);
                @16    // SubMul[0][1] / pp_0_0_msb               | %16 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%14);
                @17    // SubMul[0][1] / ovf / merge              | %17 = let_ct_block<0>();
                @18    // SubMul[0][2] / pack_0_0                 | %18 = pack_ct<4>(%2, %8);
                @19    // SubMul[0][2] / pp_0_0_lsb               | %19 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%18);
                @20    // SubMul[0][2] / pp_0_0_msb               | %20 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%18);
                @21    // SubMul[0][2] / ovf / merge              | %21 = let_ct_block<0>();
                @22    // SubMul[0][3] / pack_0_0                 | %22 = pack_ct<4>(%2, %9);
                @23    // SubMul[0][3] / pp_0_0_lsb               | %23 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%22);
                @24    // SubMul[0][3] / pp_0_0_msb               | %24 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%22);
                @25    // SubMul[0][3] / ovf / carry_in           | %25 = pbs<Protect, Lut1("IsSome")>(%24);
                @26    // SubMul[1][0] / pack_0_0                 | %26 = pack_ct<4>(%3, %6);
                @27    // SubMul[1][0] / pp_0_0_lsb               | %27 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%26);
                @28    // SubMul[1][0] / pp_0_0_msb               | %28 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%26);
                @29    // SubMul[1][0] / ovf / merge              | %29 = let_ct_block<0>();
                @30    // SubMul[1][1] / pack_0_0                 | %30 = pack_ct<4>(%3, %7);
                @31    // SubMul[1][1] / pp_0_0_lsb               | %31 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%30);
                @32    // SubMul[1][1] / pp_0_0_msb               | %32 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%30);
                @33    // SubMul[1][1] / ovf / merge              | %33 = let_ct_block<0>();
                @34    // SubMul[1][2] / pack_0_0                 | %34 = pack_ct<4>(%3, %8);
                @35    // SubMul[1][2] / pp_0_0_lsb               | %35 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%34);
                @36    // SubMul[1][2] / pp_0_0_msb               | %36 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%34);
                @37    // SubMul[1][2] / ovf / carry_in           | %37 = pbs<Protect, Lut1("IsSome")>(%36);
                @38    // SubMul[1][3] / ovf_0_0                  | %38 = pack_ct<4>(%3, %9);
                @39    // SubMul[1][3] / ovf_0_0                  | %39 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%38);
                @40    // SubMul[2][0] / pack_0_0                 | %40 = pack_ct<4>(%4, %6);
                @41    // SubMul[2][0] / pp_0_0_lsb               | %41 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%40);
                @42    // SubMul[2][0] / pp_0_0_msb               | %42 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%40);
                @43    // SubMul[2][0] / ovf / merge              | %43 = let_ct_block<0>();
                @44    // SubMul[2][1] / pack_0_0                 | %44 = pack_ct<4>(%4, %7);
                @45    // SubMul[2][1] / pp_0_0_lsb               | %45 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%44);
                @46    // SubMul[2][1] / pp_0_0_msb               | %46 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%44);
                @47    // SubMul[2][1] / ovf / carry_in           | %47 = pbs<Protect, Lut1("IsSome")>(%46);
                @48    // SubMul[2][2] / ovf_0_0                  | %48 = pack_ct<4>(%4, %8);
                @49    // SubMul[2][2] / ovf_0_0                  | %49 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%48);
                @50    // SubMul[2][3] / ovf_0_0                  | %50 = pack_ct<4>(%4, %9);
                @51    // SubMul[2][3] / ovf_0_0                  | %51 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%50);
                @52    // SubMul[3][0] / pack_0_0                 | %52 = pack_ct<4>(%5, %6);
                @53    // SubMul[3][0] / pp_0_0_lsb               | %53 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%52);
                @54    // SubMul[3][0] / pp_0_0_msb               | %54 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%52);
                @55    // SubMul[3][0] / ovf / carry_in           | %55 = pbs<Protect, Lut1("IsSome")>(%54);
                @56    // SubMul[3][1] / ovf_0_0                  | %56 = pack_ct<4>(%5, %7);
                @57    // SubMul[3][1] / ovf_0_0                  | %57 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%56);
                @58    // SubMul[3][2] / ovf_0_0                  | %58 = pack_ct<4>(%5, %8);
                @59    // SubMul[3][2] / ovf_0_0                  | %59 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%58);
                @60    // SubMul[3][3] / ovf_0_0                  | %60 = pack_ct<4>(%5, %9);
                @61    // SubMul[3][3] / ovf_0_0                  | %61 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%60);
                @62    // Limb reduce[1]                          | %62 = decl_ct<2>();
                @63    // Limb reduce[1]                          | %63 = let_ct_block<0>();
                @64    // Limb reduce[1]                          | %64 = store_ct_block<0>(%63, %62);
                @65    // Limb reduce[1]                          | %65 = store_ct_block<0>(%12, %64);
                @66    // Limb reduce[1]                          | %66 = decl_ct<2>();
                @67    // Limb reduce[1]                          | %67 = let_ct_block<0>();
                @68    // Limb reduce[1]                          | %68 = store_ct_block<0>(%67, %66);
                @69    // Limb reduce[1]                          | %69 = store_ct_block<0>(%15, %68);
                @70    // Limb reduce[1] / iter 0                 | %70 = extract_ct_block<0>(%65);
                @71    // Limb reduce[1] / iter 0                 | %71 = extract_ct_block<0>(%69);
                @72    // Limb reduce[1] / iter 0                 | %72 = let_ct_block<0>();
                @73    // Limb reduce[1] / iter 0 / 0-th          | %73 = add_ct(%70, %71);
                @74    // Limb reduce[1] / iter 0 / 0-th          | %74 = add_ct(%73, %72);
                @75    // Limb reduce[1] / iter 0 / 0-th          | %75, %76 = pbs2<Protect, Lut2("ManyCarryMsg")>(%74);
                @76    // Limb reduce[1] / iter 0 / Join Output   | %77 = decl_ct<2>();
                @77    // Limb reduce[1] / iter 0 / Join Output   | %78 = let_ct_block<0>();
                @78    // Limb reduce[1] / iter 0 / Join Output   | %79 = store_ct_block<0>(%78, %77);
                @79    // Limb reduce[1] / iter 0 / Join Output   | %80 = store_ct_block<0>(%75, %79);
                @80    // Limb reduce[1] / iter 0 / Join Carry    | %81 = decl_ct<2>();
                @81    // Limb reduce[1] / iter 0 / Join Carry    | %82 = let_ct_block<0>();
                @82    // Limb reduce[1] / iter 0 / Join Carry    | %83 = store_ct_block<0>(%82, %81);
                @83    // Limb reduce[1] / iter 0 / Join Carry    | %84 = store_ct_block<0>(%76, %83);
                @84    // Limb reduce[1]                          | %85 = extract_ct_block<0>(%80);
                @85    // Limb reduce[1]                          | %86 = extract_ct_block<0>(%84);
                @86    // Limb reduce[1]                          | %87 = decl_ct<2>();
                @87    // Limb reduce[1]                          | %88 = let_ct_block<0>();
                @88    // Limb reduce[1]                          | %89 = store_ct_block<0>(%88, %87);
                @89    // Limb reduce[1]                          | %90 = store_ct_block<0>(%85, %89);
                @90    // Limb reduce[1]                          | %91 = decl_ct<2>();
                @91    // Limb reduce[1]                          | %92 = let_ct_block<0>();
                @92    // Limb reduce[1]                          | %93 = store_ct_block<0>(%92, %91);
                @93    // Limb reduce[1]                          | %94 = store_ct_block<0>(%27, %93);
                @94    // Limb reduce[1] / iter 1                 | %95 = extract_ct_block<0>(%90);
                @95    // Limb reduce[1] / iter 1                 | %96 = extract_ct_block<0>(%94);
                @96    // Limb reduce[1] / iter 1                 | %97 = let_ct_block<0>();
                @97    // Limb reduce[1] / iter 1 / 0-th          | %98 = add_ct(%95, %96);
                @98    // Limb reduce[1] / iter 1 / 0-th          | %99 = add_ct(%98, %97);
                @99    // Limb reduce[1] / iter 1 / 0-th          | %100, %101 = pbs2<Protect, Lut2("ManyCarryMsg")>(%99);
                @100   // Limb reduce[1] / iter 1 / Join Output   | %102 = decl_ct<2>();
                @101   // Limb reduce[1] / iter 1 / Join Output   | %103 = let_ct_block<0>();
                @102   // Limb reduce[1] / iter 1 / Join Output   | %104 = store_ct_block<0>(%103, %102);
                @103   // Limb reduce[1] / iter 1 / Join Output   | %105 = store_ct_block<0>(%100, %104);
                @104   // Limb reduce[1] / iter 1 / Join Carry    | %106 = decl_ct<2>();
                @105   // Limb reduce[1] / iter 1 / Join Carry    | %107 = let_ct_block<0>();
                @106   // Limb reduce[1] / iter 1 / Join Carry    | %108 = store_ct_block<0>(%107, %106);
                @107   // Limb reduce[1] / iter 1 / Join Carry    | %109 = store_ct_block<0>(%101, %108);
                @108   // Limb reduce[1]                          | %110 = extract_ct_block<0>(%105);
                @109   // Limb reduce[1]                          | %111 = extract_ct_block<0>(%109);
                @110   // Limb reduce[2]                          | %112 = decl_ct<2>();
                @111   // Limb reduce[2]                          | %113 = let_ct_block<0>();
                @112   // Limb reduce[2]                          | %114 = store_ct_block<0>(%113, %112);
                @113   // Limb reduce[2]                          | %115 = store_ct_block<0>(%16, %114);
                @114   // Limb reduce[2]                          | %116 = decl_ct<2>();
                @115   // Limb reduce[2]                          | %117 = let_ct_block<0>();
                @116   // Limb reduce[2]                          | %118 = store_ct_block<0>(%117, %116);
                @117   // Limb reduce[2]                          | %119 = store_ct_block<0>(%19, %118);
                @118   // Limb reduce[2] / iter 0                 | %120 = extract_ct_block<0>(%115);
                @119   // Limb reduce[2] / iter 0                 | %121 = extract_ct_block<0>(%119);
                @120   // Limb reduce[2] / iter 0 / 0-th          | %122 = add_ct(%120, %121);
                @121   // Limb reduce[2] / iter 0 / 0-th          | %123 = add_ct(%122, %111);
                @122   // Limb reduce[2] / iter 0 / 0-th          | %124, %125 = pbs2<Protect, Lut2("ManyCarryMsg")>(%123);
                @123   // Limb reduce[2] / iter 0 / Join Output   | %126 = decl_ct<2>();
                @124   // Limb reduce[2] / iter 0 / Join Output   | %127 = let_ct_block<0>();
                @125   // Limb reduce[2] / iter 0 / Join Output   | %128 = store_ct_block<0>(%127, %126);
                @126   // Limb reduce[2] / iter 0 / Join Output   | %129 = store_ct_block<0>(%124, %128);
                @127   // Limb reduce[2] / iter 0 / Join Carry    | %130 = decl_ct<2>();
                @128   // Limb reduce[2] / iter 0 / Join Carry    | %131 = let_ct_block<0>();
                @129   // Limb reduce[2] / iter 0 / Join Carry    | %132 = store_ct_block<0>(%131, %130);
                @130   // Limb reduce[2] / iter 0 / Join Carry    | %133 = store_ct_block<0>(%125, %132);
                @131   // Limb reduce[2]                          | %134 = extract_ct_block<0>(%129);
                @132   // Limb reduce[2]                          | %135 = extract_ct_block<0>(%133);
                @133   // Limb reduce[2]                          | %136 = decl_ct<2>();
                @134   // Limb reduce[2]                          | %137 = let_ct_block<0>();
                @135   // Limb reduce[2]                          | %138 = store_ct_block<0>(%137, %136);
                @136   // Limb reduce[2]                          | %139 = store_ct_block<0>(%28, %138);
                @137   // Limb reduce[2]                          | %140 = decl_ct<2>();
                @138   // Limb reduce[2]                          | %141 = let_ct_block<0>();
                @139   // Limb reduce[2]                          | %142 = store_ct_block<0>(%141, %140);
                @140   // Limb reduce[2]                          | %143 = store_ct_block<0>(%31, %142);
                @141   // Limb reduce[2] / iter 1                 | %144 = extract_ct_block<0>(%139);
                @142   // Limb reduce[2] / iter 1                 | %145 = extract_ct_block<0>(%143);
                @143   // Limb reduce[2] / iter 1 / 0-th          | %146 = add_ct(%144, %145);
                @144   // Limb reduce[2] / iter 1 / 0-th          | %147 = add_ct(%146, %86);
                @145   // Limb reduce[2] / iter 1 / 0-th          | %148, %149 = pbs2<Protect, Lut2("ManyCarryMsg")>(%147);
                @146   // Limb reduce[2] / iter 1 / Join Output   | %150 = decl_ct<2>();
                @147   // Limb reduce[2] / iter 1 / Join Output   | %151 = let_ct_block<0>();
                @148   // Limb reduce[2] / iter 1 / Join Output   | %152 = store_ct_block<0>(%151, %150);
                @149   // Limb reduce[2] / iter 1 / Join Output   | %153 = store_ct_block<0>(%148, %152);
                @150   // Limb reduce[2] / iter 1 / Join Carry    | %154 = decl_ct<2>();
                @151   // Limb reduce[2] / iter 1 / Join Carry    | %155 = let_ct_block<0>();
                @152   // Limb reduce[2] / iter 1 / Join Carry    | %156 = store_ct_block<0>(%155, %154);
                @153   // Limb reduce[2] / iter 1 / Join Carry    | %157 = store_ct_block<0>(%149, %156);
                @154   // Limb reduce[2]                          | %158 = extract_ct_block<0>(%153);
                @155   // Limb reduce[2]                          | %159 = extract_ct_block<0>(%157);
                @156   // Limb reduce[2]                          | %160 = decl_ct<2>();
                @157   // Limb reduce[2]                          | %161 = let_ct_block<0>();
                @158   // Limb reduce[2]                          | %162 = store_ct_block<0>(%161, %160);
                @159   // Limb reduce[2]                          | %163 = store_ct_block<0>(%134, %162);
                @160   // Limb reduce[2]                          | %164 = decl_ct<2>();
                @161   // Limb reduce[2]                          | %165 = let_ct_block<0>();
                @162   // Limb reduce[2]                          | %166 = store_ct_block<0>(%165, %164);
                @163   // Limb reduce[2]                          | %167 = store_ct_block<0>(%158, %166);
                @164   // Limb reduce[2] / iter 2                 | %168 = extract_ct_block<0>(%163);
                @165   // Limb reduce[2] / iter 2                 | %169 = extract_ct_block<0>(%167);
                @166   // Limb reduce[2] / iter 2                 | %170 = let_ct_block<0>();
                @167   // Limb reduce[2] / iter 2 / 0-th          | %171 = add_ct(%168, %169);
                @168   // Limb reduce[2] / iter 2 / 0-th          | %172 = add_ct(%171, %170);
                @169   // Limb reduce[2] / iter 2 / 0-th          | %173, %174 = pbs2<Protect, Lut2("ManyCarryMsg")>(%172);
                @170   // Limb reduce[2] / iter 2 / Join Output   | %175 = decl_ct<2>();
                @171   // Limb reduce[2] / iter 2 / Join Output   | %176 = let_ct_block<0>();
                @172   // Limb reduce[2] / iter 2 / Join Output   | %177 = store_ct_block<0>(%176, %175);
                @173   // Limb reduce[2] / iter 2 / Join Output   | %178 = store_ct_block<0>(%173, %177);
                @174   // Limb reduce[2] / iter 2 / Join Carry    | %179 = decl_ct<2>();
                @175   // Limb reduce[2] / iter 2 / Join Carry    | %180 = let_ct_block<0>();
                @176   // Limb reduce[2] / iter 2 / Join Carry    | %181 = store_ct_block<0>(%180, %179);
                @177   // Limb reduce[2] / iter 2 / Join Carry    | %182 = store_ct_block<0>(%174, %181);
                @178   // Limb reduce[2]                          | %183 = extract_ct_block<0>(%178);
                @179   // Limb reduce[2]                          | %184 = extract_ct_block<0>(%182);
                @180   // Limb reduce[2]                          | %185 = decl_ct<2>();
                @181   // Limb reduce[2]                          | %186 = let_ct_block<0>();
                @182   // Limb reduce[2]                          | %187 = store_ct_block<0>(%186, %185);
                @183   // Limb reduce[2]                          | %188 = store_ct_block<0>(%183, %187);
                @184   // Limb reduce[2]                          | %189 = decl_ct<2>();
                @185   // Limb reduce[2]                          | %190 = let_ct_block<0>();
                @186   // Limb reduce[2]                          | %191 = store_ct_block<0>(%190, %189);
                @187   // Limb reduce[2]                          | %192 = store_ct_block<0>(%41, %191);
                @188   // Limb reduce[2] / iter 3                 | %193 = extract_ct_block<0>(%188);
                @189   // Limb reduce[2] / iter 3                 | %194 = extract_ct_block<0>(%192);
                @190   // Limb reduce[2] / iter 3                 | %195 = let_ct_block<0>();
                @191   // Limb reduce[2] / iter 3 / 0-th          | %196 = add_ct(%193, %194);
                @192   // Limb reduce[2] / iter 3 / 0-th          | %197 = add_ct(%196, %195);
                @193   // Limb reduce[2] / iter 3 / 0-th          | %198, %199 = pbs2<Protect, Lut2("ManyCarryMsg")>(%197);
                @194   // Limb reduce[2] / iter 3 / Join Output   | %200 = decl_ct<2>();
                @195   // Limb reduce[2] / iter 3 / Join Output   | %201 = let_ct_block<0>();
                @196   // Limb reduce[2] / iter 3 / Join Output   | %202 = store_ct_block<0>(%201, %200);
                @197   // Limb reduce[2] / iter 3 / Join Output   | %203 = store_ct_block<0>(%198, %202);
                @198   // Limb reduce[2] / iter 3 / Join Carry    | %204 = decl_ct<2>();
                @199   // Limb reduce[2] / iter 3 / Join Carry    | %205 = let_ct_block<0>();
                @200   // Limb reduce[2] / iter 3 / Join Carry    | %206 = store_ct_block<0>(%205, %204);
                @201   // Limb reduce[2] / iter 3 / Join Carry    | %207 = store_ct_block<0>(%199, %206);
                @202   // Limb reduce[2]                          | %208 = extract_ct_block<0>(%203);
                @203   // Limb reduce[2]                          | %209 = extract_ct_block<0>(%207);
                @204   // Limb reduce[3]                          | %210 = decl_ct<2>();
                @205   // Limb reduce[3]                          | %211 = let_ct_block<0>();
                @206   // Limb reduce[3]                          | %212 = store_ct_block<0>(%211, %210);
                @207   // Limb reduce[3]                          | %213 = store_ct_block<0>(%20, %212);
                @208   // Limb reduce[3]                          | %214 = decl_ct<2>();
                @209   // Limb reduce[3]                          | %215 = let_ct_block<0>();
                @210   // Limb reduce[3]                          | %216 = store_ct_block<0>(%215, %214);
                @211   // Limb reduce[3]                          | %217 = store_ct_block<0>(%23, %216);
                @212   // Limb reduce[3] / iter 0                 | %218 = extract_ct_block<0>(%213);
                @213   // Limb reduce[3] / iter 0                 | %219 = extract_ct_block<0>(%217);
                @214   // Limb reduce[3] / iter 0 / 0-th          | %220 = add_ct(%218, %219);
                @215   // Limb reduce[3] / iter 0 / 0-th          | %221 = add_ct(%220, %209);
                @216   // Limb reduce[3] / iter 0 / 0-th          | %222, %223 = pbs2<Protect, Lut2("ManyCarryMsg")>(%221);
                @217   // Limb reduce[3] / iter 0 / Join Output   | %224 = decl_ct<2>();
                @218   // Limb reduce[3] / iter 0 / Join Output   | %225 = let_ct_block<0>();
                @219   // Limb reduce[3] / iter 0 / Join Output   | %226 = store_ct_block<0>(%225, %224);
                @220   // Limb reduce[3] / iter 0 / Join Output   | %227 = store_ct_block<0>(%222, %226);
                @221   // Limb reduce[3] / iter 0 / Join Carry    | %228 = decl_ct<2>();
                @222   // Limb reduce[3] / iter 0 / Join Carry    | %229 = let_ct_block<0>();
                @223   // Limb reduce[3] / iter 0 / Join Carry    | %230 = store_ct_block<0>(%229, %228);
                @224   // Limb reduce[3] / iter 0 / Join Carry    | %231 = store_ct_block<0>(%223, %230);
                @225   // Limb reduce[3]                          | %232 = extract_ct_block<0>(%227);
                @226   // Limb reduce[3]                          | %233 = extract_ct_block<0>(%231);
                @227   // Limb reduce[3]                          | %234 = decl_ct<2>();
                @228   // Limb reduce[3]                          | %235 = let_ct_block<0>();
                @229   // Limb reduce[3]                          | %236 = store_ct_block<0>(%235, %234);
                @230   // Limb reduce[3]                          | %237 = store_ct_block<0>(%32, %236);
                @231   // Limb reduce[3]                          | %238 = decl_ct<2>();
                @232   // Limb reduce[3]                          | %239 = let_ct_block<0>();
                @233   // Limb reduce[3]                          | %240 = store_ct_block<0>(%239, %238);
                @234   // Limb reduce[3]                          | %241 = store_ct_block<0>(%35, %240);
                @235   // Limb reduce[3] / iter 1                 | %242 = extract_ct_block<0>(%237);
                @236   // Limb reduce[3] / iter 1                 | %243 = extract_ct_block<0>(%241);
                @237   // Limb reduce[3] / iter 1 / 0-th          | %244 = add_ct(%242, %243);
                @238   // Limb reduce[3] / iter 1 / 0-th          | %245 = add_ct(%244, %184);
                @239   // Limb reduce[3] / iter 1 / 0-th          | %246, %247 = pbs2<Protect, Lut2("ManyCarryMsg")>(%245);
                @240   // Limb reduce[3] / iter 1 / Join Output   | %248 = decl_ct<2>();
                @241   // Limb reduce[3] / iter 1 / Join Output   | %249 = let_ct_block<0>();
                @242   // Limb reduce[3] / iter 1 / Join Output   | %250 = store_ct_block<0>(%249, %248);
                @243   // Limb reduce[3] / iter 1 / Join Output   | %251 = store_ct_block<0>(%246, %250);
                @244   // Limb reduce[3] / iter 1 / Join Carry    | %252 = decl_ct<2>();
                @245   // Limb reduce[3] / iter 1 / Join Carry    | %253 = let_ct_block<0>();
                @246   // Limb reduce[3] / iter 1 / Join Carry    | %254 = store_ct_block<0>(%253, %252);
                @247   // Limb reduce[3] / iter 1 / Join Carry    | %255 = store_ct_block<0>(%247, %254);
                @248   // Limb reduce[3]                          | %256 = extract_ct_block<0>(%251);
                @249   // Limb reduce[3]                          | %257 = extract_ct_block<0>(%255);
                @250   // Limb reduce[3]                          | %258 = decl_ct<2>();
                @251   // Limb reduce[3]                          | %259 = let_ct_block<0>();
                @252   // Limb reduce[3]                          | %260 = store_ct_block<0>(%259, %258);
                @253   // Limb reduce[3]                          | %261 = store_ct_block<0>(%42, %260);
                @254   // Limb reduce[3]                          | %262 = decl_ct<2>();
                @255   // Limb reduce[3]                          | %263 = let_ct_block<0>();
                @256   // Limb reduce[3]                          | %264 = store_ct_block<0>(%263, %262);
                @257   // Limb reduce[3]                          | %265 = store_ct_block<0>(%45, %264);
                @258   // Limb reduce[3] / iter 2                 | %266 = extract_ct_block<0>(%261);
                @259   // Limb reduce[3] / iter 2                 | %267 = extract_ct_block<0>(%265);
                @260   // Limb reduce[3] / iter 2 / 0-th          | %268 = add_ct(%266, %267);
                @261   // Limb reduce[3] / iter 2 / 0-th          | %269 = add_ct(%268, %159);
                @262   // Limb reduce[3] / iter 2 / 0-th          | %270, %271 = pbs2<Protect, Lut2("ManyCarryMsg")>(%269);
                @263   // Limb reduce[3] / iter 2 / Join Output   | %272 = decl_ct<2>();
                @264   // Limb reduce[3] / iter 2 / Join Output   | %273 = let_ct_block<0>();
                @265   // Limb reduce[3] / iter 2 / Join Output   | %274 = store_ct_block<0>(%273, %272);
                @266   // Limb reduce[3] / iter 2 / Join Output   | %275 = store_ct_block<0>(%270, %274);
                @267   // Limb reduce[3] / iter 2 / Join Carry    | %276 = decl_ct<2>();
                @268   // Limb reduce[3] / iter 2 / Join Carry    | %277 = let_ct_block<0>();
                @269   // Limb reduce[3] / iter 2 / Join Carry    | %278 = store_ct_block<0>(%277, %276);
                @270   // Limb reduce[3] / iter 2 / Join Carry    | %279 = store_ct_block<0>(%271, %278);
                @271   // Limb reduce[3]                          | %280 = extract_ct_block<0>(%275);
                @272   // Limb reduce[3]                          | %281 = extract_ct_block<0>(%279);
                @273   // Limb reduce[3]                          | %282 = decl_ct<2>();
                @274   // Limb reduce[3]                          | %283 = let_ct_block<0>();
                @275   // Limb reduce[3]                          | %284 = store_ct_block<0>(%283, %282);
                @276   // Limb reduce[3]                          | %285 = store_ct_block<0>(%232, %284);
                @277   // Limb reduce[3]                          | %286 = decl_ct<2>();
                @278   // Limb reduce[3]                          | %287 = let_ct_block<0>();
                @279   // Limb reduce[3]                          | %288 = store_ct_block<0>(%287, %286);
                @280   // Limb reduce[3]                          | %289 = store_ct_block<0>(%256, %288);
                @281   // Limb reduce[3] / iter 3                 | %290 = extract_ct_block<0>(%285);
                @282   // Limb reduce[3] / iter 3                 | %291 = extract_ct_block<0>(%289);
                @283   // Limb reduce[3] / iter 3 / 0-th          | %292 = add_ct(%290, %291);
                @284   // Limb reduce[3] / iter 3 / 0-th          | %293 = add_ct(%292, %135);
                @285   // Limb reduce[3] / iter 3 / 0-th          | %294, %295 = pbs2<Protect, Lut2("ManyCarryMsg")>(%293);
                @286   // Limb reduce[3] / iter 3 / Join Output   | %296 = decl_ct<2>();
                @287   // Limb reduce[3] / iter 3 / Join Output   | %297 = let_ct_block<0>();
                @288   // Limb reduce[3] / iter 3 / Join Output   | %298 = store_ct_block<0>(%297, %296);
                @289   // Limb reduce[3] / iter 3 / Join Output   | %299 = store_ct_block<0>(%294, %298);
                @290   // Limb reduce[3] / iter 3 / Join Carry    | %300 = decl_ct<2>();
                @291   // Limb reduce[3] / iter 3 / Join Carry    | %301 = let_ct_block<0>();
                @292   // Limb reduce[3] / iter 3 / Join Carry    | %302 = store_ct_block<0>(%301, %300);
                @293   // Limb reduce[3] / iter 3 / Join Carry    | %303 = store_ct_block<0>(%295, %302);
                @294   // Limb reduce[3]                          | %304 = extract_ct_block<0>(%299);
                @295   // Limb reduce[3]                          | %305 = extract_ct_block<0>(%303);
                @296   // Limb reduce[3]                          | %306 = decl_ct<2>();
                @297   // Limb reduce[3]                          | %307 = let_ct_block<0>();
                @298   // Limb reduce[3]                          | %308 = store_ct_block<0>(%307, %306);
                @299   // Limb reduce[3]                          | %309 = store_ct_block<0>(%280, %308);
                @300   // Limb reduce[3]                          | %310 = decl_ct<2>();
                @301   // Limb reduce[3]                          | %311 = let_ct_block<0>();
                @302   // Limb reduce[3]                          | %312 = store_ct_block<0>(%311, %310);
                @303   // Limb reduce[3]                          | %313 = store_ct_block<0>(%53, %312);
                @304   // Limb reduce[3] / iter 4                 | %314 = extract_ct_block<0>(%309);
                @305   // Limb reduce[3] / iter 4                 | %315 = extract_ct_block<0>(%313);
                @306   // Limb reduce[3] / iter 4                 | %316 = let_ct_block<0>();
                @307   // Limb reduce[3] / iter 4 / 0-th          | %317 = add_ct(%314, %315);
                @308   // Limb reduce[3] / iter 4 / 0-th          | %318 = add_ct(%317, %316);
                @309   // Limb reduce[3] / iter 4 / 0-th          | %319, %320 = pbs2<Protect, Lut2("ManyCarryMsg")>(%318);
                @310   // Limb reduce[3] / iter 4 / Join Output   | %321 = decl_ct<2>();
                @311   // Limb reduce[3] / iter 4 / Join Output   | %322 = let_ct_block<0>();
                @312   // Limb reduce[3] / iter 4 / Join Output   | %323 = store_ct_block<0>(%322, %321);
                @313   // Limb reduce[3] / iter 4 / Join Output   | %324 = store_ct_block<0>(%319, %323);
                @314   // Limb reduce[3] / iter 4 / Join Carry    | %325 = decl_ct<2>();
                @315   // Limb reduce[3] / iter 4 / Join Carry    | %326 = let_ct_block<0>();
                @316   // Limb reduce[3] / iter 4 / Join Carry    | %327 = store_ct_block<0>(%326, %325);
                @317   // Limb reduce[3] / iter 4 / Join Carry    | %328 = store_ct_block<0>(%320, %327);
                @318   // Limb reduce[3]                          | %329 = extract_ct_block<0>(%324);
                @319   // Limb reduce[3]                          | %330 = extract_ct_block<0>(%328);
                @320   // Limb reduce[3]                          | %331 = decl_ct<2>();
                @321   // Limb reduce[3]                          | %332 = let_ct_block<0>();
                @322   // Limb reduce[3]                          | %333 = store_ct_block<0>(%332, %331);
                @323   // Limb reduce[3]                          | %334 = store_ct_block<0>(%304, %333);
                @324   // Limb reduce[3]                          | %335 = decl_ct<2>();
                @325   // Limb reduce[3]                          | %336 = let_ct_block<0>();
                @326   // Limb reduce[3]                          | %337 = store_ct_block<0>(%336, %335);
                @327   // Limb reduce[3]                          | %338 = store_ct_block<0>(%329, %337);
                @328   // Limb reduce[3] / iter 5                 | %339 = extract_ct_block<0>(%334);
                @329   // Limb reduce[3] / iter 5                 | %340 = extract_ct_block<0>(%338);
                @330   // Limb reduce[3] / iter 5                 | %341 = let_ct_block<0>();
                @331   // Limb reduce[3] / iter 5 / 0-th          | %342 = add_ct(%339, %340);
                @332   // Limb reduce[3] / iter 5 / 0-th          | %343 = add_ct(%342, %341);
                @333   // Limb reduce[3] / iter 5 / 0-th          | %344, %345 = pbs2<Protect, Lut2("ManyCarryMsg")>(%343);
                @334   // Limb reduce[3] / iter 5 / Join Output   | %346 = decl_ct<2>();
                @335   // Limb reduce[3] / iter 5 / Join Output   | %347 = let_ct_block<0>();
                @336   // Limb reduce[3] / iter 5 / Join Output   | %348 = store_ct_block<0>(%347, %346);
                @337   // Limb reduce[3] / iter 5 / Join Output   | %349 = store_ct_block<0>(%344, %348);
                @338   // Limb reduce[3] / iter 5 / Join Carry    | %350 = decl_ct<2>();
                @339   // Limb reduce[3] / iter 5 / Join Carry    | %351 = let_ct_block<0>();
                @340   // Limb reduce[3] / iter 5 / Join Carry    | %352 = store_ct_block<0>(%351, %350);
                @341   // Limb reduce[3] / iter 5 / Join Carry    | %353 = store_ct_block<0>(%345, %352);
                @342   // Limb reduce[3]                          | %354 = extract_ct_block<0>(%349);
                @343   // Limb reduce[3]                          | %355 = extract_ct_block<0>(%353);
                @344   // Limb_ovf / merge                        | %356 = add_ct(%233, %257);
                @345   // Limb_ovf / merge                        | %357 = add_ct(%356, %281);
                @346   // Limb_ovf / merge                        | %358 = add_ct(%357, %305);
                @347   // Limb_ovf / merge                        | %359 = add_ct(%358, %330);
                @348   // Limb_ovf / merge                        | %360 = add_ct(%359, %355);
                @349   // Limb_ovf / merge                        | %361 = pbs<Protect, Lut1("IsSome")>(%360);
                @350                                              | %362 = decl_ct<2>();
                @351                                              | %363 = let_ct_block<0>();
                @352                                              | %364 = store_ct_block<0>(%363, %362);
                @353                                              | %365 = store_ct_block<0>(%11, %364);
                @354                                              | output<0>(%365);
                @355                                              | %366 = decl_ct<2>();
                @356                                              | %367 = let_ct_block<0>();
                @357                                              | %368 = store_ct_block<0>(%367, %366);
                @358                                              | %369 = store_ct_block<0>(%110, %368);
                @359                                              | output<1>(%369);
                @360                                              | %370 = decl_ct<2>();
                @361                                              | %371 = let_ct_block<0>();
                @362                                              | %372 = store_ct_block<0>(%371, %370);
                @363                                              | %373 = store_ct_block<0>(%208, %372);
                @364                                              | output<2>(%373);
                @365                                              | %374 = decl_ct<2>();
                @366                                              | %375 = let_ct_block<0>();
                @367                                              | %376 = store_ct_block<0>(%375, %374);
                @368                                              | %377 = store_ct_block<0>(%354, %376);
                @369                                              | output<3>(%377);
            "#
        );
    }
}
