use std::collections::{BTreeMap, HashMap};

use zhc_ir::ValId;

use crate::{CiphertextBlock, CiphertextBool, CiphertextInteger, NU, NU_BOOL, builder::Builder};
use zhc_crypto::integer_semantics::{SpecBlock, SpecInteger};
use zhc_langs::ioplang::{Lut1, Lut2};
use zhc_utils::SafeAs;

/// Creates an IR for multiplication of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_mul`]. Returns the low bits of
/// the product (wrapping multiplication). See the builder method for algorithm details.
pub fn mul(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    let res = builder.iop_mul(&src_a, &src_b);
    builder.ciphertext_integer_output(res);
    builder
}

/// Creates an IR for multiplication with overflow detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_mul`]. Returns the product
/// and a single-block overflow flag. See the builder method for algorithm details.
pub fn overflow_mul(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_mul(&src_a, &src_b);
    builder.ciphertext_integer_output(res);
    builder.ciphertext_bool_output(flag);
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
    /// # use zhc_builder::{SpecInteger, Builder};
    /// # let spec = SpecInteger::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_integer_input(spec.int_size());
    /// # let b = builder.ciphertext_integer_input(spec.int_size());
    /// let product = builder.iop_mul(&a, &b);
    /// ```
    pub fn iop_mul(&self, lhs: &CiphertextInteger, rhs: &CiphertextInteger) -> CiphertextInteger {
        let src_a_blocks = self.ciphertext_integer_split(&lhs);
        let src_b_blocks = self.ciphertext_integer_split(&rhs);
        // Only kept LSB to obtain a IxI -> I operations
        let cut_off = lhs.spec().block_count();
        // Call inner function and construct results
        let output = self.iop_mul_lsb_auto(&src_a_blocks, &src_b_blocks, cut_off);
        self.ciphertext_integer_join(&output, Some(lhs.spec().int_size()))
    }

    /// Whether the triangular multiplication is expected to beat the packed one.
    ///
    /// It saves about one PBS per partial product and pays about 2 PBS per output column for the
    /// division by 3. Measured on one HPU, it wins from 4.5 partial products per output column
    /// (64x10 and 128x10 bits win, 32x10 bits is even, 64x8 and 128x8 bits lose), once the
    /// circuit is large enough not to be latency-bound (from 9 output blocks: 18x18 bits wins,
    /// 16x16 loses).
    pub fn mul_tri_is_better(la: usize, lb: usize, cut: usize) -> bool {
        let pairs: usize = (0..la).map(|i| lb.min(cut.saturating_sub(i))).sum();
        cut >= 9 && 2 * pairs >= 9 * cut
    }

    /// Low `cut_off_block` blocks of `a * b`, with whichever algorithm is faster for these sizes.
    pub fn iop_mul_lsb_auto(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_off_block: u8,
    ) -> Vec<CiphertextBlock> {
        let cut = cut_off_block.sas::<usize>();
        if Self::mul_tri_is_better(src_a_blocks.len(), src_b_blocks.len(), cut) {
            self.iop_mul_lsb_raw(src_a_blocks, src_b_blocks, cut_off_block)
        } else {
            self.iop_mul_raw(src_a_blocks, src_b_blocks, cut_off_block)
                .0
        }
    }

    /// Computes the `cut_off_block` low blocks of `a * b` (wrapping), without overflow flag.
    ///
    /// The operands may have different lengths (asymmetric multiplication). See
    /// [`iop_mul_tri_window`](Self::iop_mul_tri_window) for the algorithm.
    pub fn iop_mul_lsb_raw(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        cut_off_block: u8,
    ) -> Vec<CiphertextBlock> {
        let cut = cut_off_block.sas::<usize>();
        self.iop_mul_tri_window(
            src_a_blocks,
            src_b_blocks,
            cut,
            (0, cut),
            TriMulState::default(),
        )
        .0
    }

    /// Computes the columns `range.0..range.1` of `a * b mod 4^total_cut`.
    ///
    /// Uses `x * y = f(x + y) - g(x) - g(y)` with `f(s) = s (s - 3) / 2` and
    /// `g(x) = x (x - 3) / 2` (any `f(s) = s^2 / 2 + c s`, `g(x) = x^2 / 2 + c x` works; `c = -3/2`
    /// makes `g` the smallest: `g(0..=3) = 0, -1, -1, 0`).
    ///
    /// * Expansion: for every pair `(i, j)` with `i + j` in the range, `s = a_i + b_j <= 6 < 8`, so
    ///   one many-LUT PBS gives both base-4 digits of `f(s) + 1` in `0..=10`: low (at most 3) in
    ///   column `i + j`, high (at most 2) in column `i + j + 1`. The `+1` per pair adds the known
    ///   constant `N = sum 4^(i + j)`, subtracted as plaintext digits during the reduction.
    /// * Corrections: over the pairs `i < La, j < Lb, i + j < C`, the subtracted `g` terms sum to
    ///   `+(U - V) / 3 mod 4^C`, with `U = sum g(a_k) 4^k + sum g(b_k) 4^k` and `V = sum g(a_i)
    ///   4^(i + Lb) + sum g(b_j) 4^(j + La)` (terms at or above `C` vanish). Digit `d_k` of `U - V`
    ///   involves at most two input digits (one PBS) and lies in `-2..=2`. The exact 2-adic
    ///   division by 3 is a carry chain: `v_k = d_k + rho_(k-1)`, `m_k = 3 v_k mod 4`, `rho_k =
    ///   (v_k - 3 m_k) / 4`. With offsets from interval bounds, its input stays below 8, so one
    ///   many-LUT PBS gives both `m_k` and the next carry.
    /// * Reduction: columns are summed with bound tracking, then cleaned (carry and message).
    ///
    /// The state carries the columns at or above `range.1` and the chain, so that a later call
    /// with the next range continues the product (used by the multi-HPU split). Anything left
    /// unused (top carries, chain after the last column) is removed as dead code.
    ///
    /// Also returns the correction digit blocks (`d_k` minus its lower bound), used by the
    /// overflow check.
    pub fn iop_mul_tri_window(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
        total_cut: usize,
        range: (usize, usize),
        mut state: TriMulState,
    ) -> (
        Vec<CiphertextBlock>,
        Vec<Option<CiphertextBlock>>,
        TriMulState,
    ) {
        let spec = *self.spec();
        assert_eq!(
            spec.message_size(),
            2,
            "triangular multiplication assumes 2-bit digits"
        );
        assert_eq!(
            spec.carry_size(),
            2,
            "triangular multiplication assumes 2-bit carries"
        );
        assert!(
            range.0 <= range.1 && range.1 <= total_cut,
            "invalid range {range:?}"
        );
        let (la, lb) = (src_a_blocks.len(), src_b_blocks.len());
        let msg_max = spec.message_mask() as u64;
        let msg_w = spec.message_size() as u32;
        let data_max = (1u64 << (spec.message_size() + spec.carry_size())) - 1;
        // Digits of -N mod 4^(total_cut (+1 when the overflow check needs column C)).
        let n_const = total_cut + state.keep_top as usize;
        let neg_n = tri_neg_pair_count_digits(la, lb, total_cut, n_const);

        // The division chain is the critical path of the product (one PBS level per column); the
        // reduction below orders the column items by arrival so that it adds as little as
        // possible after it (35 levels at 64 bits, the chain being 33).
        // Correction chain
        self.push_comment("tri_div3".to_string());
        let mut corr_blocks = Vec::new();
        let mut const_done = vec![false; range.1 - range.0];
        for k in range.0..range.1 {
            let terms = tri_corr_terms(la, lb, total_cut, k);
            // g in -1..=0: a positive term spans -1..=0, a negative one 0..=1.
            let lo: i64 = -(terms.iter().filter(|t| t.2 > 0).count() as i64);
            let hi: i64 = terms.iter().filter(|t| t.2 < 0).count() as i64;
            let digit = |op: char, idx: usize| {
                if op == 'a' {
                    &src_a_blocks[idx]
                } else {
                    &src_b_blocks[idx]
                }
            };
            let e = match terms.as_slice() {
                [] => None,
                [(o, i, s)] => Some(self.block_lookup(digit(*o, *i), tri_lut_corr1(spec, *s))),
                [(o1, i1, s1), (o2, i2, s2)] => Some(self.block_pack_then_lookup(
                    digit(*o1, *i1),
                    digit(*o2, *i2),
                    tri_lut_corr2(spec, *s1, *s2),
                )),
                _ => unreachable!(),
            };
            corr_blocks.push(e);
            let e_lvl = e.map_or(0, |_| 1);
            let (r, rmin, rmax) = match state.chain.take() {
                None => (None, 0, 0),
                Some(c) => (Some(c.r), c.rmin, c.rmax),
            };
            let w = match (e, r) {
                (Some(e), Some(r)) => self.block_add(e, r),
                (Some(e), None) => e,
                (None, Some(r)) => r,
                // d_k = 0 and rho = 0: digit 0, chain stays at 0.
                (None, None) => continue,
            };
            let vmin = lo + rmin;
            let vmax = hi + rmax;
            assert!(
                vmax - vmin < 8,
                "division chain input does not fit a many-LUT"
            );
            let (nrmin, nrmax) = rho_range(vmin, vmax);
            // After the last column, the overflow check wants the next digit, 3 rho mod 4, plus
            // the constant digit C of column C (folded the same way, mod 4).
            let top = state.keep_top && k + 1 == total_cut;
            let top_cst = if top {
                (neg_n[total_cut] as i64 + 6 * total_cut as i64) % 4
            } else {
                0
            };
            // Digit k of -N rides with m_k: no plaintext operation, no extra block.
            let cst = neg_n[k] as i64;
            let r_lvl = r.map_or(0, |r| *state.levels.get(&r.valid).unwrap_or(&0));
            let (m, nxt) =
                self.block_lookup2(w, tri_lut_div3_step(spec, vmin, nrmin, top, cst, top_cst));
            let step_lvl = e_lvl.max(r_lvl) + 1;
            state.levels.insert(m.valid, step_lvl);
            state.levels.insert(nxt.valid, step_lvl);
            state
                .columns
                .entry(k)
                .or_default()
                .push((m, msg_max + cst as u64));
            const_done[k - range.0] = true;
            if top {
                state.top_digit = Some(nxt);
            } else {
                state.chain = Some(TriChain {
                    r: nxt,
                    rmin: nrmin,
                    rmax: nrmax,
                });
            }
        }
        self.pop_comment();

        // Expansion
        self.push_comment("tri_expand".to_string());
        let lo_hi = tri_lut_lo_hi(spec);
        for (i, ai) in src_a_blocks.iter().enumerate() {
            for (j, bj) in src_b_blocks.iter().enumerate() {
                let k = i + j;
                if k < range.0 || k >= range.1 {
                    continue;
                }
                let sum = self.block_add(ai, bj);
                if k + 1 >= total_cut && !state.keep_top {
                    let lo = self.block_lookup(sum, tri_lut_lo(spec));
                    state.columns.entry(k).or_default().push((lo, msg_max));
                    continue;
                }
                let (lo, hi) = self.block_lookup2(sum, lo_hi.clone());
                state.columns.entry(k).or_default().push((lo, msg_max));
                if k + 1 == total_cut {
                    // Only the overflow check reads them; kept apart from the late carries.
                    state.top_hi.push((hi, 2));
                } else {
                    state.columns.entry(k + 1).or_default().push((hi, 2));
                }
            }
        }
        self.pop_comment();

        // Reduction
        let mut dst_blk = Vec::new();
        for k in range.0..range.1 {
            self.push_comment(format!("reduction_{k}"));
            let mut stage = state.columns.remove(&k).unwrap_or_default();
            let cst = neg_n[k] as u64;
            if !const_done[k - range.0] && cst > 0 {
                // No division digit in this column to carry the constant.
                stage.push((self.block_let_ciphertext(cst as u8), cst));
            }
            // Items in order of arrival (estimated PBS level), largest bounds first among those
            // ready together. The late ones (division digit, carries of the previous column's last
            // cleaning) then end up in the last segment, with a single cleaning after them,
            // instead of forcing a cleaning in the middle of the critical path.
            let lv = |b: &CiphertextBlock| *state.levels.get(&b.valid).unwrap_or(&1);
            stage.sort_by(|x, y| lv(&x.0).cmp(&lv(&y.0)).then(y.1.cmp(&x.1)));
            if stage.is_empty() {
                dst_blk.push(self.block_let_ciphertext(0));
                self.pop_comment();
                continue;
            }
            let mut nxt_stage = Vec::new();
            // Many-LUT input bound: below it, one PBS gives both message and carry.
            let lut2_cap = data_max >> 1;
            let last = k + 1 == total_cut && !state.keep_top;
            // Sums are filled up to the data bound (15) before a two-PBS cleaning: a 7-bound
            // with many-LUT cleanings only would take 2 or 3 partial products per PBS instead of
            // 5 or 6 per two PBS (+39% PBS at 64 bits). But a cleaning that happens to have a sum
            // of at most 7 (often the last one of a column) takes one many-LUT PBS.
            let item_lvls: Vec<u32> = stage.iter().map(|x| lv(&x.0)).collect();
            let mut new_levels: Vec<(ValId, u32)> = Vec::new();
            let mut clean = |acc_ct: CiphertextBlock, acc_max: u64, acc_lvl: u32| {
                if acc_max <= lut2_cap {
                    let (msg, carry) = self.block_lookup2(acc_ct, Lut2::many_carry_msg(spec));
                    nxt_stage.push((carry, acc_max >> msg_w));
                    new_levels.push((carry.valid, acc_lvl + 1));
                    (msg, acc_max.min(msg_max))
                } else {
                    let carry = self.block_lookup(acc_ct, Lut1::carry_in_msg(spec));
                    nxt_stage.push((carry, acc_max >> msg_w));
                    new_levels.push((carry.valid, acc_lvl + 1));
                    let msg = self.block_lookup(acc_ct, Lut1::msg_only(spec));
                    (msg, acc_max.min(msg_max))
                }
            };
            let mut it = stage.into_iter().zip(item_lvls);
            let ((mut acc_ct, mut acc_max), mut acc_lvl) = it.next().unwrap();
            let mut acc_nb = 1;
            for ((ct, ct_max), ct_lvl) in it {
                if acc_max + ct_max > data_max || acc_nb == NU_BOOL {
                    (acc_ct, acc_max) = clean(acc_ct, acc_max, acc_lvl);
                    acc_lvl += 1;
                    acc_nb = 1;
                }
                acc_ct = self.block_add(ct, acc_ct);
                acc_max += ct_max;
                acc_lvl = acc_lvl.max(ct_lvl);
                acc_nb += 1;
            }
            if acc_nb != 1 || acc_max > msg_max {
                if last {
                    // Carry unused: one PBS whatever the bound.
                    acc_ct = self.block_lookup(acc_ct, Lut1::msg_only(spec));
                } else {
                    (acc_ct, _) = clean(acc_ct, acc_max, acc_lvl);
                }
            }
            dst_blk.push(acc_ct);
            state.levels.extend(new_levels);
            let nxt_stage: Vec<_> = nxt_stage.into_iter().filter(|(_, m)| *m > 0).collect();
            if !nxt_stage.is_empty() {
                state.columns.entry(k + 1).or_default().extend(nxt_stage);
            }
            self.pop_comment();
        }
        if state.keep_top && range.1 == total_cut && state.top_digit.is_none() {
            state.top_const = (neg_n[total_cut] as u64 + 6 * total_cut as u64) % 4;
        }
        (dst_blk, corr_blocks, state)
    }

    /// Multiplication with overflow flag, triangular version (operands of `cut` blocks each).
    ///
    /// Overflow happens iff `a * b >= 4^C`. If no partial product `a_i b_j` with `i + j >= C` is
    /// non-zero, then `a * b < 4^(C+1)`, so it overflows iff digit `C` of the product of the low
    /// pairs is non-zero; otherwise it overflows anyway. Digit `C` is only needed mod 4: it
    /// gathers the high digits and carries leaving column `C - 1`, digit `C` of `-N`, the next
    /// digit of the division by 3, `3 (rho_(C-1) + d_C) mod 4`, where `d_C = -sum of all g`
    /// (the `V` terms of column `C`). In the symmetric case the correction blocks are
    /// `e_k = 2 - h(a_k) - h(b_k)` with `h = -g`, so `3 d_C = 3 (2 C - sum e_k) = 6 C + sum e_k`
    /// mod 4. It is reduced with message-only cleanings. The high pairs are tested without a
    /// PBS per pair: some `a_i b_j`, `i + j >= C`, is non-zero iff `a_i != 0` and `b_j != 0` for
    /// some `j >= C - i`, with a suffix OR over `b`.
    pub fn iop_overflow_mul_tri_raw(
        &self,
        src_a_blocks: &[CiphertextBlock],
        src_b_blocks: &[CiphertextBlock],
    ) -> (Vec<CiphertextBlock>, CiphertextBlock) {
        let spec = *self.spec();
        let c = src_a_blocks.len();
        assert_eq!(c, src_b_blocks.len(), "symmetric operands expected");
        let msg_max = spec.message_mask() as u64;
        let data_max = spec.data_mask() as u64;
        let (digits, corr, mut state) = self.iop_mul_tri_window(
            src_a_blocks,
            src_b_blocks,
            c,
            (0, c),
            TriMulState {
                keep_top: true,
                ..Default::default()
            },
        );

        self.push_comment("ovf".to_string());
        let mut flags = Vec::new();

        // High pairs, merged into one flag on their own: they are ready long before digit C.
        self.push_comment("high_pairs".to_string());
        let mut high = Vec::new();
        if c >= 2 {
            // bsuf[t] = (b_t, ..., b_(C-1)) not all zero, for t in 1..C
            let mut bsuf = vec![None; c];
            let mut prev: Option<CiphertextBlock> = None;
            for t in (1..c).rev() {
                let x = match prev {
                    None => src_b_blocks[t],
                    Some(p) => self.block_add(&src_b_blocks[t], &p),
                };
                let f = self.block_lookup(x, Lut1::is_some(spec));
                bsuf[t] = Some(f);
                prev = Some(f);
            }
            let and_nz = tri_lut_and_nonzero(spec);
            for i in 1..c {
                let b = bsuf[c - i].unwrap();
                high.push(self.block_pack_then_lookup(b, &src_a_blocks[i], and_nz.clone()));
            }
        }
        while high.len() > 1 {
            high = high
                .chunks(NU_BOOL)
                .map(|chunk| {
                    let mut it = chunk.iter();
                    let init = *it.next().unwrap();
                    let sum = it.fold(init, |acc, v| self.block_add(&acc, v));
                    self.block_lookup(sum, Lut1::is_some(spec))
                })
                .collect();
        }
        flags.extend(high);
        self.pop_comment();

        // Digit C mod 4. Its terms come in two waves: the correction blocks and the high digits
        // of the column C - 1 pairs are ready early, the carries leaving column C - 1 and the
        // last division digit only at the very end. The early ones are summed first, so that
        // only a short tail is left once the reduction is over.
        self.push_comment("digit_c".to_string());
        let mut col: Vec<(CiphertextBlock, u64)> =
            corr.into_iter().flatten().map(|e| (e, 2)).collect();
        col.append(&mut state.top_hi);
        col.extend(state.columns.remove(&c).unwrap_or_default());
        if let Some(t) = state.top_digit.take() {
            col.push((t, msg_max));
        }
        // Constant digit of column C, when no division digit carried it.
        if state.top_const > 0 {
            col.push((
                self.block_let_ciphertext(state.top_const as u8),
                state.top_const,
            ));
        }
        if !col.is_empty() {
            let mut it = col.into_iter();
            let (mut acc, mut acc_max) = it.next().unwrap();
            let mut acc_nb = 1;
            for (ct, ct_max) in it {
                if acc_max + ct_max > data_max || acc_nb == NU_BOOL {
                    acc = self.block_lookup(acc, Lut1::msg_only(spec));
                    acc_max = msg_max;
                    acc_nb = 1;
                }
                acc = self.block_add(ct, acc);
                acc_max += ct_max;
                acc_nb += 1;
            }
            flags.push(self.block_lookup(acc, tri_lut_msg_is_some(spec)));
        }
        self.pop_comment();

        // Merge booleans
        self.push_comment("merge".to_string());
        let flag = if flags.is_empty() {
            self.block_let_ciphertext(0)
        } else {
            while flags.len() > 1 {
                flags = flags
                    .chunks(NU_BOOL)
                    .map(|chunk| {
                        let mut it = chunk.iter();
                        let init = *it.next().unwrap();
                        let sum = it.fold(init, |acc, v| self.block_add(&acc, v));
                        self.block_lookup(sum, Lut1::is_some(spec))
                    })
                    .collect();
            }
            flags.pop().unwrap()
        };
        self.pop_comment();
        self.pop_comment();
        (digits, flag)
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
    /// # use zhc_builder::{SpecInteger, Builder};
    /// # let spec = SpecInteger::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_integer_input(spec.int_size());
    /// # let b = builder.ciphertext_integer_input(spec.int_size());
    /// let (product, overflow) = builder.iop_overflow_mul(&a, &b);
    /// ```
    pub fn iop_overflow_mul(
        &self,
        lhs: &CiphertextInteger,
        rhs: &CiphertextInteger,
    ) -> (CiphertextInteger, CiphertextBool) {
        // Get input as array of blk
        let src_a_blocks = self.ciphertext_integer_split(&lhs);
        let src_b_blocks = self.ciphertext_integer_split(&rhs);
        // Only kept LSB to obtain a IxI -> I operations
        let cut_off = lhs.spec().block_count();
        // Call inner function and construct results
        // The triangular version also replaces the per-pair overflow PBS, so it wins earlier:
        // from 8 blocks (14x14 bits loses 7%, 16x16 bits saves 3%, 64x64 bits 41%).
        let (output, flag_block) = if src_a_blocks.len() == cut_off.sas::<usize>()
            && src_b_blocks.len() == cut_off.sas::<usize>()
            && cut_off >= 8
        {
            self.iop_overflow_mul_tri_raw(&src_a_blocks, &src_b_blocks)
        } else {
            self.iop_mul_raw(&src_a_blocks, &src_b_blocks, cut_off)
        };
        (
            self.ciphertext_integer_join(&output, Some(lhs.spec().int_size())),
            self.ciphertext_bool_store_block(flag_block),
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
    /// # use zhc_builder::{SpecInteger, Builder};
    /// # let spec = SpecInteger::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_integer_input(spec.int_size());
    /// # let b = builder.ciphertext_integer_input(spec.int_size());
    /// # let a = builder.ciphertext_integer_split(&a);
    /// # let b = builder.ciphertext_integer_split(&b);
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
        // Each partial product is kept with an upper bound of its value, so that the reduction
        // can sum as many blocks as the data region allows before a PBS is required.
        let mut partial_product_map = BTreeMap::<usize, Vec<(CiphertextBlock, u64)>>::new();
        let mut overflow_v = Vec::<CiphertextBlock>::new();
        let msg_max = self.spec().message_mask() as u64;
        let msg_w = self.spec().message_size() as u32;
        // Largest value the data region (carry + message) can hold.
        let data_max = (1u64 << (self.spec().message_size() + self.spec().carry_size())) - 1;
        // (a * b) & msg_max can reach msg_max; (a * b) >> msg_w is at most msg_max² >> msg_w.
        let lsb_max = msg_max;
        let msb_max = (msg_max * msg_max) >> msg_w;

        for (i, ai) in src_a_blocks.iter().enumerate() {
            for (j, bj) in src_b_blocks.iter().enumerate() {
                if (i + j) < cut_off_block.sas::<usize>() {
                    // Full partial product compution
                    // Pack
                    let packed = self.comment(format!("pack_{i}_{j}")).block_pack(ai, bj);
                    // Compute Lsb
                    partial_product_map.entry(i + j).or_default().push((
                        self.comment(format!("pp_{i}_{j}_lsb"))
                            .block_lookup(packed, Lut1::mult_carry_msg_lsb(*self.spec())),
                        lsb_max,
                    ));
                    // Compute Msb
                    partial_product_map.entry(i + j + 1).or_default().push((
                        self.comment(format!("pp_{i}_{j}_msb"))
                            .block_lookup(packed, Lut1::mult_carry_msg_msb(*self.spec())),
                        msb_max,
                    ));
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

        // Largest bounds first in each column: a cleaning then absorbs the 3-bounded blocks
        // together and the remaining room is filled by the smaller ones, which needs fewer
        // cleanings than mixing them. The carries, added during the reduction, stay last, so a
        // column does not start by waiting for the previous one. The sort is stable.
        for column in partial_product_map.values_mut() {
            column.sort_by(|x, y| y.1.cmp(&x.1));
        }

        // Phase 2  Reduce/Merge:
        //
        // Phase 2.a
        // Gather partial products together at each level.
        // Partial products are summed as long as the sum of their upper bounds fits in the data
        // region (and at most NU_BOOL fresh blocks, for the noise). Then the carry is extracted
        // and injected in the next stage, with its own upper bound.
        // NB: Reduce up to cut_off_block
        let mut dst_blk = Vec::new();
        for k in 0..cut_off_block.sas::<usize>() {
            self.push_comment(format!("reduction_{k}"));
            let stage_sum = partial_product_map.remove(&k).unwrap_or_default();
            if !stage_sum.is_empty() {
                let mut nxt_stage = Vec::new();
                // Fold them one by one while storing optional carry
                let mut stg_iter = stage_sum.into_iter();
                let (mut acc_ct, mut acc_max) = stg_iter.next().unwrap();
                let mut acc_nb = 1;

                // Cleans the accumulator: its carry goes to the next stage, its message stays.
                let mut clean = |acc_ct: CiphertextBlock, acc_max: u64| {
                    let carry = self.block_lookup(acc_ct, Lut1::carry_in_msg(*self.spec()));
                    nxt_stage.push((carry, acc_max >> msg_w));
                    let msg = self.block_lookup(acc_ct, Lut1::msg_only(*self.spec()));
                    (msg, acc_max.min(msg_max))
                };

                // NB: only fresh ciphertext is push in partial_product_map
                for (ct, ct_max) in stg_iter {
                    // Extract carry if the next block could overflow the data region
                    if acc_max + ct_max > data_max || acc_nb == NU_BOOL {
                        (acc_ct, acc_max) = clean(acc_ct, acc_max);
                        acc_nb = 1;
                    }
                    acc_ct = self.block_add(ct, acc_ct);
                    acc_max += ct_max;
                    acc_nb += 1;
                }

                // Current stage is completly reduce. Clear block if needed
                if acc_nb != 1 {
                    (acc_ct, _) = clean(acc_ct, acc_max);
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
            let in_carry_v: Vec<_> = in_carry_v.into_iter().map(|(ct, _)| ct).collect();
            for chunk in in_carry_v.chunks(NU) {
                let mut chunk_iter = chunk.iter();
                let init = *chunk_iter.next().unwrap();
                let chunk_sum = chunk_iter.fold(init, |acc, v| self.block_add(&acc, v));
                let is_some_flag = self.block_lookup(chunk_sum, Lut1::is_some(*self.spec()));
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

        (dst_blk, overflow_flag)
    }
}

/// State passed between consecutive windows of a triangular multiplication.
#[derive(Clone, Default)]
pub struct TriMulState {
    columns: BTreeMap<usize, Vec<(CiphertextBlock, u64)>>,
    /// Estimated PBS level (longest PBS chain from the inputs) of the column items.
    levels: HashMap<ValId, u32>,
    chain: Option<TriChain>,
    /// Keep what leaves column `total_cut - 1` (needed by the overflow check).
    keep_top: bool,
    /// With `keep_top`: digit `total_cut` of the division by 3, and of `-N`.
    top_digit: Option<CiphertextBlock>,
    top_const: u64,
    top_hi: Vec<(CiphertextBlock, u64)>,
}

/// Division-by-3 chain after a column: `rho = r + rmin`, with `rho` in `rmin..=rmax`.
#[derive(Clone, Copy)]
struct TriChain {
    r: CiphertextBlock,
    rmin: i64,
    rmax: i64,
}

/// `x (x - 3) / 2`: `f(s)` for the pair sums, `g(x)` for the digits.
fn tri(x: i64) -> i64 {
    x * (x - 3) / 2
}

/// Next carry of the exact division by 3: `m = 3 v mod 4`, `rho = (v - 3 m) / 4`.
fn rho_of(v: i64) -> i64 {
    let m = (3 * v).rem_euclid(4);
    (v - 3 * m).div_euclid(4)
}

fn rho_range(vmin: i64, vmax: i64) -> (i64, i64) {
    let it = (vmin..=vmax).map(rho_of);
    (it.clone().min().unwrap(), it.max().unwrap())
}

/// Input digits entering correction digit `k`: (operand, index, sign). At most two.
fn tri_corr_terms(la: usize, lb: usize, cut: usize, k: usize) -> Vec<(char, usize, i64)> {
    let mut t = Vec::new();
    if k < la {
        t.push(('a', k, 1));
    }
    if k < lb {
        t.push(('b', k, 1));
    }
    if k >= lb && k - lb < la && k < cut {
        t.push(('a', k - lb, -1));
    }
    if k >= la && k - la < lb && k < cut {
        t.push(('b', k - la, -1));
    }
    assert!(t.len() <= 2);
    t
}

/// Base-4 digits of `-N mod 4^n_digits`, `N = sum over pairs (i < la, j < lb, i + j < cut) of
/// 4^(i + j)`.
fn tri_neg_pair_count_digits(la: usize, lb: usize, cut: usize, n_digits: usize) -> Vec<u8> {
    let mut d = vec![0u64; n_digits + 1];
    for k in 0..cut.min(n_digits) {
        // pairs in column k
        d[k] = (0..la).filter(|&i| i <= k && k - i < lb).count() as u64;
    }
    // normalize N
    for k in 0..n_digits {
        let c = d[k] / 4;
        d[k] %= 4;
        d[k + 1] += c;
    }
    // two's complement in base 4: complement then add one
    let mut out = vec![0u8; n_digits + 1];
    let mut carry = 1u64;
    for k in 0..n_digits {
        let v = 3 - d[k] + carry;
        out[k] = (v % 4) as u8;
        carry = v / 4;
    }
    out
}

fn clamp_data(spec: SpecBlock, v: i64) -> u16 {
    v.clamp(0, spec.data_mask() as i64) as u16
}

fn tri_lut_lo_hi(spec: SpecBlock) -> Lut2 {
    Lut2::from_fn(
        "MulTriLoHi",
        spec,
        |b| {
            b.spec()
                .from_message(((tri(b.raw_data_bits() as i64) + 1) & 3) as u16)
        },
        |b| {
            b.spec().from_data(clamp_data(
                b.spec(),
                (tri(b.raw_data_bits() as i64) + 1) >> 2,
            ))
        },
    )
}

fn tri_lut_lo(spec: SpecBlock) -> Lut1 {
    Lut1::from_fn("MulTriLo", spec, |b| {
        b.spec()
            .from_message(((tri(b.raw_data_bits() as i64) + 1) & 3) as u16)
    })
}

/// One correction term `s * g(x)`, minus its lower bound (`-1` when positive, `0` otherwise).
fn tri_lut_corr1(spec: SpecBlock, sign: i64) -> Lut1 {
    let name = if sign > 0 {
        "MulTriCorrPos"
    } else {
        "MulTriCorrNeg"
    };
    Lut1::from_fn(name, spec, move |b| {
        let x = tri(b.raw_message_bits() as i64);
        b.spec()
            .from_data(clamp_data(b.spec(), if sign > 0 { x + 1 } else { -x }))
    })
}

/// Two correction terms `s1 * g(x) + s2 * g(y)` on `pack(x, y)`, minus their lower bound.
fn tri_lut_corr2(spec: SpecBlock, s1: i64, s2: i64) -> Lut1 {
    let lo = -((s1 > 0) as i64 + (s2 > 0) as i64);
    Lut1::from_fn(format!("MulTriCorr2_{s1}_{s2}"), spec, move |b| {
        let x = tri(b.move_carry_to_message().raw_message_bits() as i64);
        let y = tri(b.raw_message_bits() as i64);
        b.spec()
            .from_data(clamp_data(b.spec(), s1 * x + s2 * y - lo))
    })
}

/// One step of the division by 3 on `w = v - vmin < 8`: (digit `3 v mod 4` plus the constant
/// digit `cst` of `-N`, next carry `rho - next_rmin`), or with `top`, (digit, next digit
/// `3 rho + top_cst mod 4`).
fn tri_lut_div3_step(
    spec: SpecBlock,
    vmin: i64,
    next_rmin: i64,
    top: bool,
    cst: i64,
    top_cst: i64,
) -> Lut2 {
    Lut2::from_fn(
        format!("MulDiv3Step_{vmin}_{next_rmin}_{top}_{cst}_{top_cst}"),
        spec,
        move |b| {
            let v = b.raw_data_bits() as i64 + vmin;
            b.spec().from_data(((3 * v).rem_euclid(4) + cst) as u16)
        },
        move |b| {
            let v = b.raw_data_bits() as i64 + vmin;
            let out = if top {
                (3 * rho_of(v) + top_cst).rem_euclid(4)
            } else {
                rho_of(v) - next_rmin
            };
            b.spec().from_data(clamp_data(b.spec(), out))
        },
    )
}

fn tri_lut_msg_is_some(spec: SpecBlock) -> Lut1 {
    Lut1::from_fn("MsgIsSome", spec, |b| {
        b.spec().from_message((b.raw_message_bits() != 0) as u16)
    })
}

/// On `pack(flag, digit)`: 1 iff the flag is set and the digit is not zero.
fn tri_lut_and_nonzero(spec: SpecBlock) -> Lut1 {
    Lut1::from_fn("FlagAndNonZero", spec, |b| {
        let f = b.move_carry_to_message().raw_message_bits();
        b.spec()
            .from_message((f != 0 && b.raw_message_bits() != 0) as u16)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::SpecInteger;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn correctness_mul_lsb() {
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
        for size in (2..128).step_by(2) {
            mul(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_mul_lsb() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            let (product, flag) = lhs.overflow_mul_lsb(*rhs);
            Some(vec![
                IopValue::CiphertextInteger(product),
                IopValue::CiphertextBool(flag),
            ])
        }
        for size in (2..128).step_by(2) {
            overflow_mul(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_mul_lsb_corners() {
        fn check(size: u16, a: u128, b: u128) {
            let spec = SpecInteger::new(size, 2, 2);
            let builder = Builder::new(spec.block_spec());
            let src_a = builder.ciphertext_integer_input(spec.int_size());
            let src_b = builder.ciphertext_integer_input(spec.int_size());
            // Force the triangular version, whatever the size.
            let ab = builder.ciphertext_integer_split(&src_a);
            let bb = builder.ciphertext_integer_split(&src_b);
            let res = builder.iop_mul_lsb_raw(&ab, &bb, spec.block_count());
            let res = builder.ciphertext_integer_join(&res, Some(size));
            builder.ciphertext_integer_output(res);
            let (va, vb) = (src_a.make_value(a as _), src_b.make_value(b as _));
            let expected = match (&va, &vb) {
                (IopValue::CiphertextInteger(x), IopValue::CiphertextInteger(y)) => {
                    IopValue::CiphertextInteger(x.mul_lsb(*y))
                }
                _ => unreachable!(),
            };
            let out = builder.interpret().with_inputs(&[va, vb]).get_outputs();
            assert_eq!(out, vec![expected], "size {size}: {a} * {b}");
        }
        // Exhaustive on small sizes.
        for size in [2u16, 4, 6] {
            for a in 0..(1u128 << size) {
                for b in 0..(1u128 << size) {
                    check(size, a, b);
                }
            }
        }
        // Extreme digit patterns on large sizes.
        for size in [32u16, 64, 128] {
            let max = if size == 128 {
                u128::MAX
            } else {
                (1u128 << size) - 1
            };
            let pats = [
                0,
                1,
                2,
                3,
                max,
                max - 1,
                max / 3,
                max / 3 * 2,
                max >> 2,
                0xAAAA_AAAA_AAAA_AAAA & max,
            ];
            for &a in &pats {
                for &b in &pats {
                    check(size, a, b);
                }
            }
        }
    }

    fn pats(bits: u16) -> Vec<u128> {
        let max = if bits == 128 {
            u128::MAX
        } else {
            (1u128 << bits) - 1
        };
        let mut v = vec![
            0,
            1,
            2,
            3,
            max,
            max - 1,
            max / 3,
            max / 3 * 2,
            max >> 2,
            max >> (bits / 2),
        ];
        v.push(0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA & max);
        v.push(1u128 << (bits - 1));
        let mut x: u128 = 0x9E37_79B9_7F4A_7C15_F39C_C060_5CED_C834;
        for _ in 0..6 {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            v.push(x & max);
        }
        v
    }

    #[test]
    fn correctness_mul_tri_asym() {
        let mut cases = vec![];
        for (a_bits, b_bits, out_bits) in [
            (64u16, 32u16, 64u16),
            (64, 16, 64),
            (64, 8, 64),
            (32, 64, 64),
            (64, 2, 64),
            (16, 16, 32),
            (16, 8, 24),
            (8, 16, 24),
            (8, 8, 2),
            (10, 6, 12),
            (128, 8, 128),
            (32, 32, 64),
            (6, 4, 10),
        ] {
            cases.push((a_bits, b_bits, out_bits));
        }
        for (a_bits, b_bits, out_bits) in cases {
            let spec = SpecInteger::new(out_bits, 2, 2);
            let builder = Builder::new(spec.block_spec());
            let a = builder.ciphertext_integer_input(a_bits);
            let b = builder.ciphertext_integer_input(b_bits);
            let ab = builder.ciphertext_integer_split(&a);
            let bb = builder.ciphertext_integer_split(&b);
            let res = builder.iop_mul_lsb_raw(&ab, &bb, spec.block_count());
            let res = builder.ciphertext_integer_join(&res, Some(out_bits));
            builder.ciphertext_integer_output(res);
            builder.check_noise();
            let mask = if out_bits == 128 {
                u128::MAX
            } else {
                (1u128 << out_bits) - 1
            };
            for &x in &pats(a_bits) {
                for &y in &pats(b_bits) {
                    let out = builder
                        .interpret()
                        .with_inputs(&[a.make_value(x), b.make_value(y)])
                        .get_outputs();
                    assert_eq!(
                        out,
                        vec![res.make_value(x.wrapping_mul(y) & mask)],
                        "{a_bits}x{b_bits}->{out_bits}: {x} * {y}"
                    );
                }
            }
        }
    }

    #[test]
    fn correctness_overflow_mul_tri_corners() {
        fn check(size: u16, x: u128, y: u128) {
            let spec = SpecInteger::new(size, 2, 2);
            let builder = Builder::new(spec.block_spec());
            let a = builder.ciphertext_integer_input(size);
            let b = builder.ciphertext_integer_input(size);
            // Force the triangular version, whatever the size.
            let ab = builder.ciphertext_integer_split(&a);
            let bb = builder.ciphertext_integer_split(&b);
            let (res, flag) = builder.iop_overflow_mul_tri_raw(&ab, &bb);
            let res = builder.ciphertext_integer_join(&res, Some(size));
            builder.ciphertext_integer_output(res);
            let flag = builder.ciphertext_bool_store_block(flag);
            builder.ciphertext_bool_output(flag);
            builder.check_noise();
            let (va, vb) = (a.make_value(x), b.make_value(y));
            let expected = match (&va, &vb) {
                (IopValue::CiphertextInteger(p), IopValue::CiphertextInteger(q)) => {
                    let (prod, f) = p.overflow_mul_lsb(*q);
                    vec![
                        IopValue::CiphertextInteger(prod),
                        IopValue::CiphertextBool(f),
                    ]
                }
                _ => unreachable!(),
            };
            let out = builder.interpret().with_inputs(&[va, vb]).get_outputs();
            assert_eq!(out, expected, "size {size}: {x} * {y}");
        }
        for size in [2u16, 4, 6] {
            for x in 0..(1u128 << size) {
                for y in 0..(1u128 << size) {
                    check(size, x, y);
                }
            }
        }
        for size in [16u16, 32, 64, 128] {
            let mut p = pats(size);
            // products just below / at / above 2^size
            let half = size / 2;
            p.extend([
                1u128 << half,
                (1u128 << half) - 1,
                (1u128 << half) + 1,
                1u128 << (half - 1),
            ]);
            for &x in &p {
                for &y in &p {
                    check(size, x, y);
                }
            }
        }
    }

    #[test]
    fn test_mul_lsb() {
        let spec = SpecInteger::new(8, 2, 2);
        let ir = zhc_pipeline::passes::optimize_iop(&mul(spec).ir());
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
                // reduction_1   | %52 = add_ct(%23, %14);
                // reduction_1   | %53 = add_ct(%12, %52);
                // reduction_1   | %54 = pbs<Protect, Lut1("CarryInMsg")>(%53);
                // reduction_1   | %55 = pbs<Protect, Lut1("MsgOnly")>(%53);
                // reduction_2   | %56 = add_ct(%26, %17);
                // reduction_2   | %57 = add_ct(%34, %56);
                // reduction_2   | %58 = add_ct(%15, %57);
                // reduction_2   | %59 = add_ct(%24, %58);
                // reduction_2   | %60 = add_ct(%54, %59);
                // reduction_2   | %61 = pbs<Protect, Lut1("CarryInMsg")>(%60);
                // reduction_2   | %62 = pbs<Protect, Lut1("MsgOnly")>(%60);
                // reduction_3   | %63 = add_ct(%29, %20);
                // reduction_3   | %64 = add_ct(%37, %63);
                // reduction_3   | %65 = add_ct(%44, %64);
                // reduction_3   | %66 = add_ct(%18, %65);
                // reduction_3   | %68 = pbs<Protect, Lut1("MsgOnly")>(%66);
                // reduction_3   | %69 = add_ct(%27, %68);
                // reduction_3   | %70 = add_ct(%35, %69);
                // reduction_3   | %71 = add_ct(%61, %70);
                // reduction_3   | %73 = pbs<Protect, Lut1("MsgOnly")>(%71);
                                 | %88 = decl_ct<8>();
                                 | %94 = store_ct_block<0>(%11, %88);
                                 | %95 = store_ct_block<1>(%55, %94);
                                 | %96 = store_ct_block<2>(%62, %95);
                                 | %97 = store_ct_block<3>(%73, %96);
                                 | output<0>(%97);
            "#
        );
    }

    #[test]
    fn noise_mul_lsb() {
        for size in (2..128).step_by(2) {
            mul(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_overflow_mul_lsb() {
        for size in (2..128).step_by(2) {
            overflow_mul(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }
}
