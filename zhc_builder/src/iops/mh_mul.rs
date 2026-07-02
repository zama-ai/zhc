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
    let pack_output = outputs.into_iter().flatten().collect::<Vec<_>>();
    let output = builder.ciphertext_join(&pack_output, Some(spec.int_size()));
    builder.ciphertext_output(output);

    // // View output as mh_factor sub-part
    // for out in outputs.into_iter() {
    //     let output = builder.ciphertext_join(
    //         &out,
    //         Some(out.len() as u16 * spec.block_spec().message_size() as u16),
    //     );
    //     builder.ciphertext_output(output);
    // }

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
                let cur_partition = self.new_partition();
                println!("Pp@[{i}::{j}] => {cur_partition:?}");

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
            self.push_comment(format!("Limb reduce[{k}]"));
            let mut stage_limb = limb_map.remove(&k).unwrap_or_default();
            let mut carry_in = carry_buffer.remove(&k).unwrap_or_default();

            if stage_limb.len() > 1 {
                let cur_partition = self.new_partition();
                println!("LimbRed@[{k}] => {cur_partition:?}");
            } else {
                println!("LimbRed@[{k}] => skiped");
            }

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

        let cur_partition = self.new_partition();
        println!("OvfRed => {cur_partition:?}");
        let ovf_flag = self.merge_overflow_flag(out_of_range_limb, post_carry);

        let cur_partition = self.new_partition();
        println!("Output => {cur_partition:?}");
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
    use zhc_utils::assert_display_is;

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
                @0                                         | %0 = input_ciphertext<0, 8>();
                @1                                         | %1 = input_ciphertext<1, 8>();
                @2                                         | %2 = extract_ct_block<0>(%0);
                @3                                         | %3 = extract_ct_block<1>(%0);
                @4                                         | %4 = extract_ct_block<2>(%0);
                @5                                         | %5 = extract_ct_block<3>(%0);
                @6                                         | %6 = extract_ct_block<0>(%1);
                @7                                         | %7 = extract_ct_block<1>(%1);
                @8                                         | %8 = extract_ct_block<2>(%1);
                @9                                         | %9 = extract_ct_block<3>(%1);
                @10    // SubMul[0][0] / pack_0_0          | %10 = pack_ct<4>(%2, %6);
                @11    // SubMul[0][0] / pp_0_0_lsb        | %11 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%10);
                @12    // SubMul[0][0] / pp_0_0_msb        | %12 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%10);
                @13    // SubMul[0][0] / ovf / merge       | %13 = let_ct_block<0>();
                @14    // SubMul[0][1] / pack_0_0          | %14 = pack_ct<4>(%2, %7);
                @15    // SubMul[0][1] / pp_0_0_lsb        | %15 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%14);
                @16    // SubMul[0][1] / pp_0_0_msb        | %16 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%14);
                @17    // SubMul[0][1] / ovf / merge       | %17 = let_ct_block<0>();
                @18    // SubMul[0][2] / pack_0_0          | %18 = pack_ct<4>(%2, %8);
                @19    // SubMul[0][2] / pp_0_0_lsb        | %19 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%18);
                @20    // SubMul[0][2] / pp_0_0_msb        | %20 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%18);
                @21    // SubMul[0][2] / ovf / merge       | %21 = let_ct_block<0>();
                @22    // SubMul[0][3] / pack_0_0          | %22 = pack_ct<4>(%2, %9);
                @23    // SubMul[0][3] / pp_0_0_lsb        | %23 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%22);
                @24    // SubMul[0][3] / pp_0_0_msb        | %24 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%22);
                @25    // SubMul[0][3] / ovf / carry_in    | %25 = pbs<Protect, Lut1("IsSome")>(%24);
                @26    // SubMul[1][0] / pack_0_0          | %26 = pack_ct<4>(%3, %6);
                @27    // SubMul[1][0] / pp_0_0_lsb        | %27 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%26);
                @28    // SubMul[1][0] / pp_0_0_msb        | %28 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%26);
                @29    // SubMul[1][0] / ovf / merge       | %29 = let_ct_block<0>();
                @30    // SubMul[1][1] / pack_0_0          | %30 = pack_ct<4>(%3, %7);
                @31    // SubMul[1][1] / pp_0_0_lsb        | %31 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%30);
                @32    // SubMul[1][1] / pp_0_0_msb        | %32 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%30);
                @33    // SubMul[1][1] / ovf / merge       | %33 = let_ct_block<0>();
                @34    // SubMul[1][2] / pack_0_0          | %34 = pack_ct<4>(%3, %8);
                @35    // SubMul[1][2] / pp_0_0_lsb        | %35 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%34);
                @36    // SubMul[1][2] / pp_0_0_msb        | %36 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%34);
                @37    // SubMul[1][2] / ovf / carry_in    | %37 = pbs<Protect, Lut1("IsSome")>(%36);
                @38    // SubMul[1][3] / ovf_0_0           | %38 = pack_ct<4>(%3, %9);
                @39    // SubMul[1][3] / ovf_0_0           | %39 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%38);
                @40    // SubMul[2][0] / pack_0_0          | %40 = pack_ct<4>(%4, %6);
                @41    // SubMul[2][0] / pp_0_0_lsb        | %41 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%40);
                @42    // SubMul[2][0] / pp_0_0_msb        | %42 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%40);
                @43    // SubMul[2][0] / ovf / merge       | %43 = let_ct_block<0>();
                @44    // SubMul[2][1] / pack_0_0          | %44 = pack_ct<4>(%4, %7);
                @45    // SubMul[2][1] / pp_0_0_lsb        | %45 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%44);
                @46    // SubMul[2][1] / pp_0_0_msb        | %46 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%44);
                @47    // SubMul[2][1] / ovf / carry_in    | %47 = pbs<Protect, Lut1("IsSome")>(%46);
                @48    // SubMul[2][2] / ovf_0_0           | %48 = pack_ct<4>(%4, %8);
                @49    // SubMul[2][2] / ovf_0_0           | %49 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%48);
                @50    // SubMul[2][3] / ovf_0_0           | %50 = pack_ct<4>(%4, %9);
                @51    // SubMul[2][3] / ovf_0_0           | %51 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%50);
                @52    // SubMul[3][0] / pack_0_0          | %52 = pack_ct<4>(%5, %6);
                @53    // SubMul[3][0] / pp_0_0_lsb        | %53 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%52);
                @54    // SubMul[3][0] / pp_0_0_msb        | %54 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%52);
                @55    // SubMul[3][0] / ovf / carry_in    | %55 = pbs<Protect, Lut1("IsSome")>(%54);
                @56    // SubMul[3][1] / ovf_0_0           | %56 = pack_ct<4>(%5, %7);
                @57    // SubMul[3][1] / ovf_0_0           | %57 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%56);
                @58    // SubMul[3][2] / ovf_0_0           | %58 = pack_ct<4>(%5, %8);
                @59    // SubMul[3][2] / ovf_0_0           | %59 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%58);
                @60    // SubMul[3][3] / ovf_0_0           | %60 = pack_ct<4>(%5, %9);
                @61    // SubMul[3][3] / ovf_0_0           | %61 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%60);
                @62    // Limb reduce[1] / iter 0          | %62 = let_ct_block<0>();
                @63    // Limb reduce[1] / iter 0 / 0-th   | %63 = add_ct(%12, %15);
                @64    // Limb reduce[1] / iter 0 / 0-th   | %64 = add_ct(%63, %62);
                @65    // Limb reduce[1] / iter 0 / 0-th   | %65, %66 = pbs2<Protect, Lut2("ManyCarryMsg")>(%64);
                @66    // Limb reduce[1] / iter 1          | %67 = let_ct_block<0>();
                @67    // Limb reduce[1] / iter 1 / 0-th   | %68 = add_ct(%65, %27);
                @68    // Limb reduce[1] / iter 1 / 0-th   | %69 = add_ct(%68, %67);
                @69    // Limb reduce[1] / iter 1 / 0-th   | %70, %71 = pbs2<Protect, Lut2("ManyCarryMsg")>(%69);
                @70    // Limb reduce[2] / iter 0 / 0-th   | %72 = add_ct(%16, %19);
                @71    // Limb reduce[2] / iter 0 / 0-th   | %73 = add_ct(%72, %71);
                @72    // Limb reduce[2] / iter 0 / 0-th   | %74, %75 = pbs2<Protect, Lut2("ManyCarryMsg")>(%73);
                @73    // Limb reduce[2] / iter 1 / 0-th   | %76 = add_ct(%28, %31);
                @74    // Limb reduce[2] / iter 1 / 0-th   | %77 = add_ct(%76, %66);
                @75    // Limb reduce[2] / iter 1 / 0-th   | %78, %79 = pbs2<Protect, Lut2("ManyCarryMsg")>(%77);
                @76    // Limb reduce[2] / iter 2          | %80 = let_ct_block<0>();
                @77    // Limb reduce[2] / iter 2 / 0-th   | %81 = add_ct(%74, %78);
                @78    // Limb reduce[2] / iter 2 / 0-th   | %82 = add_ct(%81, %80);
                @79    // Limb reduce[2] / iter 2 / 0-th   | %83, %84 = pbs2<Protect, Lut2("ManyCarryMsg")>(%82);
                @80    // Limb reduce[2] / iter 3          | %85 = let_ct_block<0>();
                @81    // Limb reduce[2] / iter 3 / 0-th   | %86 = add_ct(%83, %41);
                @82    // Limb reduce[2] / iter 3 / 0-th   | %87 = add_ct(%86, %85);
                @83    // Limb reduce[2] / iter 3 / 0-th   | %88, %89 = pbs2<Protect, Lut2("ManyCarryMsg")>(%87);
                @84    // Limb reduce[3] / iter 0 / 0-th   | %90 = add_ct(%20, %23);
                @85    // Limb reduce[3] / iter 0 / 0-th   | %91 = add_ct(%90, %89);
                @86    // Limb reduce[3] / iter 0 / 0-th   | %92, %93 = pbs2<Protect, Lut2("ManyCarryMsg")>(%91);
                @87    // Limb reduce[3] / iter 1 / 0-th   | %94 = add_ct(%32, %35);
                @88    // Limb reduce[3] / iter 1 / 0-th   | %95 = add_ct(%94, %84);
                @89    // Limb reduce[3] / iter 1 / 0-th   | %96, %97 = pbs2<Protect, Lut2("ManyCarryMsg")>(%95);
                @90    // Limb reduce[3] / iter 2 / 0-th   | %98 = add_ct(%42, %45);
                @91    // Limb reduce[3] / iter 2 / 0-th   | %99 = add_ct(%98, %79);
                @92    // Limb reduce[3] / iter 2 / 0-th   | %100, %101 = pbs2<Protect, Lut2("ManyCarryMsg")>(%99);
                @93    // Limb reduce[3] / iter 3 / 0-th   | %102 = add_ct(%92, %96);
                @94    // Limb reduce[3] / iter 3 / 0-th   | %103 = add_ct(%102, %75);
                @95    // Limb reduce[3] / iter 3 / 0-th   | %104, %105 = pbs2<Protect, Lut2("ManyCarryMsg")>(%103);
                @96    // Limb reduce[3] / iter 4          | %106 = let_ct_block<0>();
                @97    // Limb reduce[3] / iter 4 / 0-th   | %107 = add_ct(%100, %53);
                @98    // Limb reduce[3] / iter 4 / 0-th   | %108 = add_ct(%107, %106);
                @99    // Limb reduce[3] / iter 4 / 0-th   | %109, %110 = pbs2<Protect, Lut2("ManyCarryMsg")>(%108);
                @100   // Limb reduce[3] / iter 5          | %111 = let_ct_block<0>();
                @101   // Limb reduce[3] / iter 5 / 0-th   | %112 = add_ct(%104, %109);
                @102   // Limb reduce[3] / iter 5 / 0-th   | %113 = add_ct(%112, %111);
                @103   // Limb reduce[3] / iter 5 / 0-th   | %114, %115 = pbs2<Protect, Lut2("ManyCarryMsg")>(%113);
                @104   // Limb_ovf / merge                 | %116 = add_ct(%93, %97);
                @105   // Limb_ovf / merge                 | %117 = add_ct(%116, %101);
                @106   // Limb_ovf / merge                 | %118 = add_ct(%117, %105);
                @107   // Limb_ovf / merge                 | %119 = add_ct(%118, %110);
                @108   // Limb_ovf / merge                 | %120 = add_ct(%119, %115);
                @109   // Limb_ovf / merge                 | %121 = pbs<Protect, Lut1("IsSome")>(%120);
                @110                                       | %122 = decl_ct<2>();
                @111                                       | %123 = let_ct_block<0>();
                @112                                       | %124 = store_ct_block<0>(%123, %122);
                @113                                       | %125 = store_ct_block<0>(%11, %124);
                @114                                       | output<0>(%125);
                @115                                       | %126 = decl_ct<2>();
                @116                                       | %127 = let_ct_block<0>();
                @117                                       | %128 = store_ct_block<0>(%127, %126);
                @118                                       | %129 = store_ct_block<0>(%70, %128);
                @119                                       | output<1>(%129);
                @120                                       | %130 = decl_ct<2>();
                @121                                       | %131 = let_ct_block<0>();
                @122                                       | %132 = store_ct_block<0>(%131, %130);
                @123                                       | %133 = store_ct_block<0>(%88, %132);
                @124                                       | output<2>(%133);
                @125                                       | %134 = decl_ct<2>();
                @126                                       | %135 = let_ct_block<0>();
                @127                                       | %136 = store_ct_block<0>(%135, %134);
                @128                                       | %137 = store_ct_block<0>(%114, %136);
                @129                                       | output<3>(%137);
            "#
        );
    }
}
