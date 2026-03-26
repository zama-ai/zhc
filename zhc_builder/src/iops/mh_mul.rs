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
    blocks: Vec<CiphertextBlock>,
}

impl From<CiphertextLimb> for Vec<CiphertextBlock> {
    fn from(value: CiphertextLimb) -> Self {
        value.blocks
    }
}

impl CiphertextLimb {
    fn new(blks: &[CiphertextBlock]) -> Self {
        let mut blocks = Vec::with_capacity(blks.len());
        blocks.extend_from_slice(blks);

        Self { blocks }
    }

    fn as_blocks(&self) -> &[CiphertextBlock] {
        &self.blocks
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
        let mh_limbs = mh_factor as usize;

        let mh_a_blocks = src_a_blocks
            .chunks(mh_blocks)
            .map(|b| CiphertextLimb::new(b))
            .collect::<Vec<_>>();
        let mh_b_blocks = src_b_blocks
            .chunks(mh_blocks)
            .map(|b| CiphertextLimb::new(b))
            .collect::<Vec<_>>();

        println!("mh_blocks: {mh_blocks}");
        println!("mh_A: {mh_a_blocks:?}");
        println!("mh_B: {mh_b_blocks:?}");

        // Phase 1:
        // Compute each mh_block width partial product
        let mut partprod_map = BTreeMap::<usize, Vec<CiphertextLimb>>::new();
        let mut overflow_v = Vec::<CiphertextBlock>::new();

        for i in 0..mh_limbs {
            for j in 0..mh_limbs {
                // Call sub-size mul
                let blocks_ofst = (i + j) * mh_blocks;
                let relin_cut_off = cut_off_block - blocks_ofst as u8;
                println!("@{i}::{j} => cut @{relin_cut_off}");
                self.push_comment(&format!("SubMul[{i}][{j}]"));
                let (ovf, sm_res) = self.iop_mul_raw(
                    mh_a_blocks[i].as_blocks(),
                    mh_b_blocks[j].as_blocks(),
                    relin_cut_off,
                );
                self.pop_comment();

                // Spread mh_block output
                if !sm_res.is_empty() {
                    // let sm_res = sm_res
                    //     .into_iter()
                    //     .map(|b| self.block_transfer(b))
                    //     .collect::<Vec<_>>();
                    if sm_res.len() > mh_blocks {
                        let (lsb, msb) = sm_res.split_at(mh_blocks);
                        let sm_lsb = CiphertextLimb::new(lsb);
                        let sm_msb = CiphertextLimb::new(msb);
                        partprod_map.entry(i + j).or_default().push(sm_lsb);
                        partprod_map.entry(i + j + 1).or_default().push(sm_msb);
                    } else {
                        let sm_lsb = CiphertextLimb { blocks: sm_res };
                        partprod_map.entry(i + j).or_default().push(sm_lsb);
                    }
                }

                // TODO filter when we know by construct that overflow is empty
                overflow_v.push(ovf);
            }
        }
        println!("Phase2: {partprod_map:?}");

        // Phase 2 fuse
        // 2.a
        // Fuse each limb with (mh_block +1)W adder
        let mut fused_limb = vec![Default::default(); mh_limbs];
        for (k, sm_res) in partprod_map.into_iter() {
            self.push_comment(&format!("FuseHS_@{k}"));
            let sum_len = sm_res.len();
            let sum = sm_res
                .into_iter()
                .enumerate()
                .reduce(|(acc_id, acc), (limb_id, limb)| {
                    let acc = if acc_id == 0 {
                        let xfer_acc = acc
                            .blocks
                            .into_iter()
                            .map(|b| self.block_transfer(b))
                            .collect::<Vec<_>>();
                        CiphertextLimb { blocks: xfer_acc }
                    } else {
                        acc
                    };
                    let raw_acc = self.iop_add_hillis_steele(acc.as_blocks(), limb.as_blocks());
                    (limb_id, CiphertextLimb { blocks: raw_acc })

                    // // if limb_id <= sum_len/2 {
                    // if limb_id == 1 {
                    //     let xfer_acc = raw_acc
                    //         .into_iter()
                    //         .map(|b| self.block_transfer(b))
                    //         .collect::<Vec<_>>();
                    //     (limb_id, CiphertextLimb { blocks: xfer_acc })
                    // } else {
                    //     (limb_id, CiphertextLimb { blocks: raw_acc })
                    // }
                })
                .map(|(_, limb)| limb);
            self.pop_comment();

            if let Some(limb) = sum {
                fused_limb[k] = limb;
            }
        }

        // 2.b
        // Extract and propagate carry
        // TODO
        let res = fused_limb
            .into_iter()
            .map(|limb| limb.into())
            .collect::<Vec<_>>();

        // Phase 3 Reduce overflow
        let ovf_flag = self.merge_overflow_flag(Default::default(), overflow_v);
        (ovf_flag, res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::{CiphertextSpec, EmulatedCiphertext};
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    const MH_FACTOR: u8 = 2;

    #[test]
    fn correctness_mh_mul_lsb() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
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
            res_split
        }
        for size in (4 * MH_FACTOR as u16..32).step_by(2 * MH_FACTOR as usize) {
            mh_mul_lsb(CiphertextSpec::new(size, 2, 2), MH_FACTOR).test_random(1, semantic);
        }
    }

    // #[test]
    // fn correctness_mh_overflow_mul_lsb() {
    //     fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
    //         let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
    //             unreachable!()
    //         };
    //         vec![IopValue::Ciphertext(lhs.mul_lsb(*rhs))]
    //     }
    //     for size in (2..128).step_by(2) {
    //         mh_mul_lsb(CiphertextSpec::new(size, 2, 2), MH_FACTOR).test_random(100, semantic);
    //     }
    // }

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
            r#""#
        );
    }
}
