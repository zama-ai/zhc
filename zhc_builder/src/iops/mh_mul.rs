use std::collections::BTreeMap;

use zhc_ir::partitioning::PartitionAnnotation;

use crate::{CiphertextBlock, NU, NU_BOOL, builder::Builder};
use zhc_crypto::integer_semantics::SpecInteger;
use zhc_langs::ioplang::{Lut1, Lut2};
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
/// # use zhc_builder::{SpecInteger, mh_mul};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = mh_mul(spec, 2);
/// let ir = builder.ir();
/// ```
pub fn mh_mul(spec: SpecInteger, split_depth: usize) -> Builder {
    mh_mul_with_opt(spec, split_depth, false, None)
}

/// [`mh_mul`] with the limb products forced to the triangular (`true`) or packed (`false`)
/// multiplication. [`mh_mul`] uses the triangular one from 4-block limbs.
pub fn mh_mul_tri(spec: SpecInteger, split_depth: usize, use_tri: bool) -> Builder {
    mh_mul_with_opt(spec, split_depth, false, Some(use_tri))
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
/// # use zhc_builder::{SpecInteger, mh_overflow_mul_lsb};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = mh_overflow_mul_lsb(spec, 2);
/// let ir = builder.ir();
/// ```
pub fn mh_overflow_mul_lsb(spec: SpecInteger, split_depth: usize) -> Builder {
    mh_mul_with_opt(spec, split_depth, true, None)
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
/// # use zhc_builder::{SpecInteger, mh_overflow_mul_lsb};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = mh_overflow_mul_lsb(spec, 2);
/// let ir = builder.ir();
/// ```
fn mh_mul_with_opt(
    spec: SpecInteger,
    split_depth: usize,
    gen_overflow: bool,
    force_tri: Option<bool>,
) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());

    // Get input as array of blk
    let src_a_blocks = builder.ciphertext_integer_split(&src_a);
    let src_b_blocks = builder.ciphertext_integer_split(&src_b);
    // Only kept LSB to obtain a IxI -> I operations
    let cut_off = spec.block_count() as usize;

    // Call inner function and construct results
    // The triangular limb products do not give the carries the overflow flag is made of.
    // Otherwise they win with the hand-made partitions of `Iop::MhMul` on 2, 4 and 8 HPUs at
    // 64 bits (-18%, -10%, -5%), even with 8-block limbs, where a single HPU would prefer the
    // packed products: the limbs are spread over boards, so what counts is the work per board.
    let use_tri = !gen_overflow && force_tri.unwrap_or(cut_off / split_depth >= 4);
    let (flag_block, outputs) = builder.mh_iop_mul_raw_with(
        &src_a_blocks,
        &src_b_blocks,
        cut_off,
        split_depth,
        use_tri,
        !gen_overflow,
    );

    if gen_overflow {
        let flag = builder.ciphertext_bool_store_block(flag_block);
        builder.ciphertext_bool_output(flag);
    }
    // View output as one
    let pack_output = outputs.into_iter().flatten().collect::<Vec<_>>();
    let output = builder.ciphertext_integer_join(&pack_output, Some(spec.int_size()));
    builder.ciphertext_integer_output(output);

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
    /// # use zhc_builder::{SpecInteger, Builder};
    /// # let spec = SpecInteger::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_integer_input(spec.int_size());
    /// # let b = builder.ciphertext_integer_input(spec.int_size());
    /// # let a = builder.ciphertext_integer_split(&a);
    /// # let b = builder.ciphertext_integer_split(&b);
    /// let (flag, res) = builder.mh_iop_mul_raw(&a, &b, spec.block_count() as usize, 2);
    /// ```
    pub fn mh_iop_mul_raw(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_off_block: usize,
        split_depth: usize,
    ) -> (CiphertextBlock, Vec<Vec<CiphertextBlock>>) {
        self.mh_iop_mul_raw_with(
            src_a_blocks,
            src_b_blocks,
            cut_off_block,
            split_depth,
            false,
            false,
        )
    }

    /// Like [`mh_iop_mul_raw`](Self::mh_iop_mul_raw); with `use_tri`, the limb products use the
    /// triangular multiplication ([`iop_mul_tri_window`](Self::iop_mul_tri_window)); with
    /// `csa`, the limb results are recombined with a carry-save reduction and one final adder
    /// ([`mh_recombine_csa`](Self::mh_recombine_csa)). In both cases the overflow flag is not
    /// valid.
    fn mh_iop_mul_raw_with(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_off_block: usize,
        split_depth: usize,
        use_tri: bool,
        csa: bool,
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

                let ovf = if use_tri {
                    let zero = self.block_let_ciphertext(0);
                    if relin_cut_off > mh_blocks {
                        self.new_partition(format!("Pp[{i}::{j}]_lsb @{{{}}}", i + j));
                        let (lsb_res, _, st) = self
                            .comment(format!("SubMul[{i}][{j}]_lsb"))
                            .iop_mul_tri_window(
                                ai,
                                bj,
                                relin_cut_off,
                                (0, mh_blocks),
                                Default::default(),
                            );
                        let lsb_limb = CiphertextLimb::new(i + j, &lsb_res);
                        limb_map.entry(lsb_limb.offset).or_default().push(lsb_limb);

                        self.new_partition(format!("Pp[{i}::{j}]_msb @{{{}}}", i + j + 1));
                        let (msb_res, _, _) = self
                            .comment(format!("SubMul[{i}][{j}]_msb"))
                            .iop_mul_tri_window(
                                ai,
                                bj,
                                relin_cut_off,
                                (mh_blocks, relin_cut_off),
                                st,
                            );
                        if !msb_res.is_empty() {
                            let msb_limb = CiphertextLimb::new(i + j + 1, &msb_res);
                            limb_map.entry(msb_limb.offset).or_default().push(msb_limb);
                        }
                    } else {
                        self.new_partition(format!("Pp[{i}::{j}] @{{{}}}", i + j));
                        let (cur_res, _, _) = self
                            .comment(format!("SubMul[{i}][{j}]"))
                            .iop_mul_tri_window(
                                ai,
                                bj,
                                relin_cut_off,
                                (0, relin_cut_off),
                                Default::default(),
                            );
                        if !cur_res.is_empty() {
                            let cur_limb = CiphertextLimb::new(i + j, &cur_res);
                            limb_map.entry(cur_limb.offset).or_default().push(cur_limb);
                        }
                    }
                    zero
                } else if relin_cut_off > mh_blocks {
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

        if csa {
            let dst_limb = self.mh_recombine_csa(limb_map, cut_off_block, split_depth);
            self.new_partition("OverflowReduction");
            let ovf_flag = self.block_let_ciphertext(0);
            self.new_partition("Outputs");
            return (ovf_flag, dst_limb);
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
            // A carry gates the whole adder it enters, since it is folded into the first
            // operation of the sum, so a carry given to a leaf has to climb every remaining
            // round. A binary tree over L limbs has L-1 adders and a column always gets two more
            // adders than carries, so the carries go to the last adders instead. Taken from the
            // front, which is production order, so the earliest carry gets the shallowest adder
            // and the latest one the root.
            let n_adders = stage_limb.len().saturating_sub(1);
            let first_with_carry = n_adders.saturating_sub(carry_in.len());
            let mut adder_idx = 0;
            if stage_limb.len() > 1 {
                // Tree-like reduction
                let mut tree_iter = 0;
                while stage_limb.len() > 1 {
                    let mut current = stage_limb.into_iter();
                    let mut next = Vec::new();

                    loop {
                        match (current.next(), current.next()) {
                            (Some(a), Some(b)) => {
                                let cin =
                                    (adder_idx >= first_with_carry).then(|| carry_in.remove(0));
                                adder_idx += 1;

                                self.new_partition(format!("LimbRed[{tree_iter}] @{{{k}}}"));
                                let (sum, cout) =
                                    self.comment(format!("iter {tree_iter}")).iop_add_raw(
                                        limbs_size,
                                        a.as_blocks(),
                                        b.as_blocks(),
                                        cin.as_ref(),
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

    /// Recombines the limb products with a carry-save reduction.
    ///
    /// The tree of full adders per output limb resolves every carry before the next addition
    /// can start, and the carries ripple from one output limb to the next: about 13 PBS levels
    /// at 64 bits with split depth 4. Here every limb block is put in its absolute column; each
    /// column holding more than two blocks is summed and cleaned (message kept, carry sent to
    /// the next column), a round at a time, without propagating carries, until every column
    /// holds at most two blocks (two rounds suffice up to seven blocks per column). One
    /// Hillis-Steele addition of the two remaining numbers ends it, from the first column that
    /// holds two blocks.
    ///
    /// Partitions: one per output limb half, `Csa @{k.h}` (its rounds merged), then `FinalAdd`.
    /// Out-of-range limbs (only used for overflow) are ignored.
    fn mh_recombine_csa(
        &self,
        mut limb_map: BTreeMap<usize, Vec<CiphertextLimb>>,
        cut_off_block: usize,
        split_depth: usize,
    ) -> Vec<Vec<CiphertextBlock>> {
        let spec = *self.spec();
        let mh_blocks = cut_off_block.div_ceil(split_depth);
        let msg_max = spec.message_mask() as u64;
        let data_max = spec.data_mask() as u64;
        let lut2_cap = data_max >> 1;
        let msg_w = spec.message_size() as u32;
        let mut columns: Vec<Vec<(CiphertextBlock, u64)>> = vec![Vec::new(); cut_off_block];
        for k in 0..split_depth {
            for limb in limb_map.remove(&k).unwrap_or_default() {
                for (idx, blk) in limb.into_blocks().into_iter().enumerate() {
                    let col = k * mh_blocks + idx;
                    if col < cut_off_block {
                        columns[col].push((blk, msg_max));
                    }
                }
            }
        }
        let group = |col: usize| (col / mh_blocks, 2 * (col % mh_blocks) / mh_blocks.max(1));
        let mut group_parts: BTreeMap<(usize, usize), Vec<PartitionAnnotation>> = BTreeMap::new();
        let mut round = 0;
        while columns.iter().any(|c| c.len() > 2) {
            let mut next: Vec<Vec<(CiphertextBlock, u64)>> = vec![Vec::new(); cut_off_block];
            let mut current_group = None;
            for col in 0..cut_off_block {
                let items = std::mem::take(&mut columns[col]);
                if items.len() <= 2 {
                    next[col].extend(items);
                    continue;
                }
                let g = group(col);
                if current_group != Some(g) {
                    let p = self.new_partition(format!("Csa r{round} @{{{}.{}}}", g.0, g.1));
                    group_parts.entry(g).or_default().push(p);
                    current_group = Some(g);
                }
                self.push_comment(format!("csa r{round} col {col}"));
                let mut items = items;
                items.sort_by(|x, y| y.1.cmp(&x.1));
                let last = col + 1 == cut_off_block;
                let mut it = items.into_iter();
                let (mut acc, mut acc_max) = it.next().unwrap();
                let mut acc_nb = 1;
                let flush =
                    |acc: CiphertextBlock,
                     acc_max: u64,
                     nb: usize,
                     next: &mut Vec<Vec<(CiphertextBlock, u64)>>| {
                        if nb == 1 {
                            next[col].push((acc, acc_max));
                        } else if last {
                            let m = self.block_lookup(acc, Lut1::msg_only(spec));
                            next[col].push((m, msg_max));
                        } else if acc_max <= lut2_cap {
                            let (m, c) = self.block_lookup2(acc, Lut2::many_carry_msg(spec));
                            next[col].push((m, acc_max.min(msg_max)));
                            next[col + 1].push((c, acc_max >> msg_w));
                        } else {
                            let c = self.block_lookup(acc, Lut1::carry_in_msg(spec));
                            let m = self.block_lookup(acc, Lut1::msg_only(spec));
                            next[col].push((m, msg_max));
                            next[col + 1].push((c, acc_max >> msg_w));
                        }
                    };
                for (ct, ct_max) in it {
                    if acc_max + ct_max > data_max || acc_nb == NU_BOOL {
                        flush(acc, acc_max, acc_nb, &mut next);
                        (acc, acc_max, acc_nb) = (ct, ct_max, 1);
                        continue;
                    }
                    acc = self.block_add(acc, ct);
                    acc_max += ct_max;
                    acc_nb += 1;
                }
                flush(acc, acc_max, acc_nb, &mut next);
                self.pop_comment();
            }
            columns = next;
            round += 1;
        }
        // One partition per group: merge the rounds.
        for (_, parts) in group_parts {
            let mut it = parts.into_iter();
            let first = it.next().unwrap();
            it.fold(first, |acc, p| self.merge_partitions(acc, p));
        }

        // Final addition of the two remaining numbers.
        let base = columns
            .iter()
            .position(|c| c.len() == 2)
            .unwrap_or(cut_off_block);
        let mut digits: Vec<CiphertextBlock> = Vec::with_capacity(cut_off_block);
        for col in 0..base {
            let d = match columns[col].first() {
                Some((b, _)) => *b,
                None => self.block_let_ciphertext(0),
            };
            digits.push(d);
        }
        if base < cut_off_block {
            self.new_partition("FinalAdd");
            let zero = self.block_let_ciphertext(0);
            let x: Vec<_> = (base..cut_off_block)
                .map(|c| columns[c].first().map_or(zero, |b| b.0))
                .collect();
            let y: Vec<_> = (base..cut_off_block)
                .map(|c| columns[c].get(1).map_or(zero, |b| b.0))
                .collect();
            let (sum, _) = self.comment("final add".to_string()).iop_add_raw(
                (2 * x.len()) as u16,
                &x,
                &y,
                None,
            );
            digits.extend(sum);
        }
        digits.chunks(mh_blocks).map(|c| c.to_vec()).collect()
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
            let is_some_flag = self.block_lookup(chunk_sum, Lut1::is_some(*self.spec()));
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
                        self.block_lookup(chunk_sum, Lut1::is_some(*self.spec()))
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
    /// # use zhc_builder::{SpecInteger, Builder};
    /// # let spec = SpecInteger::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_integer_input(spec.int_size());
    /// # let b = builder.ciphertext_integer_input(spec.int_size());
    /// # let a = builder.ciphertext_integer_split(&a);
    /// # let b = builder.ciphertext_integer_split(&b);
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
                            .block_lookup(packed, Lut1::mult_carry_msg_lsb(*self.spec())),
                    );
                    // Compute Msb
                    partial_product_map.entry(i + j + 1).or_default().push(
                        self.comment(format!("pp_{i}_{j}_msb"))
                            .block_lookup(packed, Lut1::mult_carry_msg_msb(*self.spec())),
                    );
                } else {
                    // Only overflow extraction
                    let mul_is_some = self.comment(format!("ovf_{i}_{j}")).block_pack_then_lookup(
                        ai,
                        bj,
                        Lut1::mult_carry_msg_is_some(*self.spec()),
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
                        nxt_stage.push(self.block_lookup(acc_ct, Lut1::carry_in_msg(*self.spec())));
                        acc_ct = self.block_lookup(acc_ct, Lut1::msg_only(*self.spec()));
                    }
                }

                // Current stage is completly reduce. Clear block if needed
                if acc_nu != 1 {
                    // Every term is a clean digit, so the accumulator holds acc_nu of them. A
                    // many-lut gives the message and the carry in one PBS instead of two, but it
                    // halves the input space, the top data bit carrying the second table, so it
                    // only has room for two terms.
                    let many_capacity = self.spec().data_mask().sas::<usize>() >> 1;
                    if acc_nu * self.spec().message_mask().sas::<usize>() <= many_capacity {
                        let (msg, carry) =
                            self.block_lookup2(acc_ct, Lut2::many_carry_msg(*self.spec()));
                        nxt_stage.push(carry);
                        acc_ct = msg;
                    } else {
                        nxt_stage.push(self.block_lookup(acc_ct, Lut1::carry_in_msg(*self.spec())));
                        acc_ct = self.block_lookup(acc_ct, Lut1::msg_only(*self.spec()));
                    }
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
                let is_some_flag = self.block_lookup(chunk_sum, Lut1::is_some(*self.spec()));
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
                        self.block_lookup(chunk_sum, Lut1::is_some(*self.spec()))
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
    use zhc_crypto::integer_semantics::SpecInteger;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    const SPLIT_DEPTH: [usize; 2] = [2, 4];

    #[test]
    fn correctness_mh_mul() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            Some(vec![IopValue::CiphertextInteger(lhs.mul_lsb(*rhs))])
        }
        for split_depth in SPLIT_DEPTH.iter() {
            for size in (4 * *split_depth as u16..64).step_by(2 * *split_depth as usize) {
                mh_mul(SpecInteger::new(size, 2, 2), *split_depth).test_random(100, semantic);
            }
        }
    }

    #[test]
    fn correctness_mh_mul_tri() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            Some(vec![IopValue::CiphertextInteger(lhs.mul_lsb(*rhs))])
        }
        for split_depth in SPLIT_DEPTH.iter() {
            for size in (4 * *split_depth as u16..=128).step_by(2 * *split_depth as usize) {
                let b = mh_mul_tri(SpecInteger::new(size, 2, 2), *split_depth, true);
                b.check_noise();
                b.test_random(50, semantic);
            }
        }
    }

    #[test]
    fn test_mh_mul() {
        let spec = SpecInteger::new(8, 2, 2);
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
                @45    // SubMul[0][0]_msb / reduction_3      | %45, %46 = pbs2<Protect, Lut2("ManyCarryMsg")>(%44);
                @46    // SubMul[0][0]_msb / ovf / carry_in   | %47 = pbs<Protect, Lut1("IsSome")>(%46);
                @47    // SubMul[0][0]_msb / ovf / merge      | %48 = add_ct(%31, %33);
                @48    // SubMul[0][0]_msb / ovf / merge      | %49 = add_ct(%48, %35);
                @49    // SubMul[0][0]_msb / ovf / merge      | %50 = add_ct(%49, %47);
                @50    // SubMul[0][0]_msb / ovf / merge      | %51 = pbs<Protect, Lut1("IsSome")>(%50);
                @51    // SubMul[0][1] / pack_0_0             | %52 = pack_ct<4>(%2, %8);
                @52    // SubMul[0][1] / pp_0_0_lsb           | %53 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%52);
                @53    // SubMul[0][1] / pp_0_0_msb           | %54 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%52);
                @54    // SubMul[0][1] / pack_0_1             | %55 = pack_ct<4>(%2, %9);
                @55    // SubMul[0][1] / pp_0_1_lsb           | %56 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%55);
                @56    // SubMul[0][1] / pp_0_1_msb           | %57 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%55);
                @57    // SubMul[0][1] / pack_1_0             | %58 = pack_ct<4>(%3, %8);
                @58    // SubMul[0][1] / pp_1_0_lsb           | %59 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%58);
                @59    // SubMul[0][1] / pp_1_0_msb           | %60 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%58);
                @60    // SubMul[0][1] / ovf_1_1              | %61 = pack_ct<4>(%3, %9);
                @61    // SubMul[0][1] / ovf_1_1              | %62 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%61);
                @62    // SubMul[0][1] / reduction_1          | %63 = add_ct(%56, %54);
                @63    // SubMul[0][1] / reduction_1          | %64 = add_ct(%59, %63);
                @64    // SubMul[0][1] / reduction_1          | %65 = pbs<Protect, Lut1("CarryInMsg")>(%64);
                @65    // SubMul[0][1] / reduction_1          | %66 = pbs<Protect, Lut1("MsgOnly")>(%64);
                @66    // SubMul[0][1] / ovf / carry_in       | %67 = add_ct(%57, %60);
                @67    // SubMul[0][1] / ovf / carry_in       | %68 = add_ct(%67, %65);
                @68    // SubMul[0][1] / ovf / carry_in       | %69 = pbs<Protect, Lut1("IsSome")>(%68);
                @69    // SubMul[0][1] / ovf / merge          | %70 = add_ct(%62, %69);
                @70    // SubMul[0][1] / ovf / merge          | %71 = pbs<Protect, Lut1("IsSome")>(%70);
                @71    // SubMul[1][0] / pack_0_0             | %72 = pack_ct<4>(%4, %6);
                @72    // SubMul[1][0] / pp_0_0_lsb           | %73 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%72);
                @73    // SubMul[1][0] / pp_0_0_msb           | %74 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%72);
                @74    // SubMul[1][0] / pack_0_1             | %75 = pack_ct<4>(%4, %7);
                @75    // SubMul[1][0] / pp_0_1_lsb           | %76 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%75);
                @76    // SubMul[1][0] / pp_0_1_msb           | %77 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%75);
                @77    // SubMul[1][0] / pack_1_0             | %78 = pack_ct<4>(%5, %6);
                @78    // SubMul[1][0] / pp_1_0_lsb           | %79 = pbs<Protect, Lut1("MultCarryMsgLsb")>(%78);
                @79    // SubMul[1][0] / pp_1_0_msb           | %80 = pbs<Protect, Lut1("MultCarryMsgMsb")>(%78);
                @80    // SubMul[1][0] / ovf_1_1              | %81 = pack_ct<4>(%5, %7);
                @81    // SubMul[1][0] / ovf_1_1              | %82 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%81);
                @82    // SubMul[1][0] / reduction_1          | %83 = add_ct(%76, %74);
                @83    // SubMul[1][0] / reduction_1          | %84 = add_ct(%79, %83);
                @84    // SubMul[1][0] / reduction_1          | %85 = pbs<Protect, Lut1("CarryInMsg")>(%84);
                @85    // SubMul[1][0] / reduction_1          | %86 = pbs<Protect, Lut1("MsgOnly")>(%84);
                @86    // SubMul[1][0] / ovf / carry_in       | %87 = add_ct(%77, %80);
                @87    // SubMul[1][0] / ovf / carry_in       | %88 = add_ct(%87, %85);
                @88    // SubMul[1][0] / ovf / carry_in       | %89 = pbs<Protect, Lut1("IsSome")>(%88);
                @89    // SubMul[1][0] / ovf / merge          | %90 = add_ct(%82, %89);
                @90    // SubMul[1][0] / ovf / merge          | %91 = pbs<Protect, Lut1("IsSome")>(%90);
                @91    // SubMul[1][1] / ovf_0_0              | %92 = pack_ct<4>(%4, %8);
                @92    // SubMul[1][1] / ovf_0_0              | %93 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%92);
                @93    // SubMul[1][1] / ovf_0_1              | %94 = pack_ct<4>(%4, %9);
                @94    // SubMul[1][1] / ovf_0_1              | %95 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%94);
                @95    // SubMul[1][1] / ovf_1_0              | %96 = pack_ct<4>(%5, %8);
                @96    // SubMul[1][1] / ovf_1_0              | %97 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%96);
                @97    // SubMul[1][1] / ovf_1_1              | %98 = pack_ct<4>(%5, %9);
                @98    // SubMul[1][1] / ovf_1_1              | %99 = pbs<Protect, Lut1("MultCarryMsgIsSome")>(%98);
                @99    // SubMul[1][1] / ovf / merge          | %100 = add_ct(%93, %95);
                @100   // SubMul[1][1] / ovf / merge          | %101 = add_ct(%100, %97);
                @101   // SubMul[1][1] / ovf / merge          | %102 = add_ct(%101, %99);
                @102   // SubMul[1][1] / ovf / merge          | %103 = pbs<Protect, Lut1("IsSome")>(%102);
                @103   // csa r0 col 2                        | %104 = add_ct(%43, %53);
                @104   // csa r0 col 2                        | %105 = add_ct(%104, %73);
                @105   // csa r0 col 2                        | %106 = pbs<Protect, Lut1("CarryInMsg")>(%105);
                @106   // csa r0 col 2                        | %107 = pbs<Protect, Lut1("MsgOnly")>(%105);
                @107   // csa r0 col 3                        | %108 = add_ct(%45, %66);
                @108   // csa r0 col 3                        | %109 = add_ct(%108, %86);
                @109   // csa r0 col 3                        | %110 = pbs<Protect, Lut1("MsgOnly")>(%109);
                @110                                          | %111 = let_ct_block<0>();
                @111   // final add                           | %112 = let_ct_block<0>();
                @112   // final add / 0-th                    | %113 = add_ct(%106, %110);
                @113   // final add / 0-th                    | %114 = add_ct(%113, %112);
                @114   // final add / 0-th                    | %115, %116 = pbs2<Protect, Lut2("ManyCarryMsg")>(%114);
                @115                                          | %117 = let_ct_block<0>();
                @116                                          | %118 = decl_ct<8>();
                @117                                          | %119 = let_ct_block<0>();
                @118                                          | %120 = store_ct_block<0>(%119, %118);
                @119                                          | %121 = store_ct_block<1>(%119, %120);
                @120                                          | %122 = store_ct_block<2>(%119, %121);
                @121                                          | %123 = store_ct_block<3>(%119, %122);
                @122                                          | %124 = store_ct_block<0>(%11, %123);
                @123                                          | %125 = store_ct_block<1>(%24, %124);
                @124                                          | %126 = store_ct_block<2>(%107, %125);
                @125                                          | %127 = store_ct_block<3>(%115, %126);
                @126                                          | output<0>(%127);
            "#
        );
    }

    #[test]
    fn noise_mh_mul() {
        for split_depth in SPLIT_DEPTH.iter() {
            for size in (4 * *split_depth as u16..64).step_by(2 * *split_depth as usize) {
                mh_mul(SpecInteger::new(size, 2, 2), *split_depth).check_noise();
            }
        }
    }
}
