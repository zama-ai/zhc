use zhc_crypto::integer_semantics::CiphertextSpec;

use crate::{
    CiphertextBlock, PlaintextBlock,
    builder::{Builder, Ciphertext, ExtensionBehavior, Plaintext},
};
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::{
    iter::{ChunkIt, CollectInSmallVec, IterMapFirst, MultiZip, ReconcilerOf2, Slide, SliderExt},
    svec,
};

use crate::iops::add::KoggeEntry;

/// Creates an IR for the addition of 1 encrypted integer and a scalar.
///

pub fn adds(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let par_w = match spec.int_size() {
        8..16 => 1,
        16..24 => 7,
        24..256 => 12,
        _ => 1,
    };
    let res = match spec.int_size() {
        0..8 => builder.iop_adds_ripple_carry(&src_c, &src_p, None).0,
        8..17 => builder.iop_adds_hillis_steele(&src_c, &src_p, None).0,
        17..256 => builder.iop_adds_kogge_stone(&src_c, &src_p, None, par_w).0,
        _ => todo!(),
    };
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for `ct + imm` with overflow (carry) detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_adds`]. Returns two outputs:
/// the wrapping sum and a single-block carry flag.
pub fn overflow_adds(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_adds(&src_c, &src_p);
    builder.ciphertext_output(res);
    builder.ciphertext_output(flag);
    builder
}

/// Creates an IR for the addition of an encrypted integers and a scalar using Hillis-Steele
/// carry propagation.
///
/// The returned [`Builder`] declares one ciphertext input, one plaintext input and one ciphertext
/// output representing the wrapping sum of the operands. This variant explicitly selects the
/// Hillis-Steele algorithm, which groups blocks into fours and resolves carries with
/// logarithmic depth. Prefer [`adds`] for automatic algorithm selection based on bit-width.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, add_hillis_steele};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = add_hillis_steele(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn adds_hillis_steele(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let res = builder.iop_adds_hillis_steele(&src_c, &src_p, None).0;
    builder.ciphertext_output(res);
    builder
}

pub fn adds_ripple_carry(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let res = builder.iop_adds_ripple_carry(&src_c, &src_p, None).0;
    builder.ciphertext_output(res);
    builder
}

pub fn adds_kogge_stone(spec: CiphertextSpec, par_w: usize) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_c = builder.ciphertext_input(spec.int_size());
    let src_p = builder.plaintext_input(spec.int_size());
    let res = builder.iop_adds_kogge_stone(&src_c, &src_p, None, par_w).0;
    builder.ciphertext_output(res);
    builder
}
impl Builder {
    /// Adds an encrypted integer with an immediate, automatically selecting the best algorithm.
    ///
    /// Chooses between ripple-carry, Hillis-Steele, and Kogge-Stone based on the
    /// operand bit-width: ripple-carry for small integers (< 8 bits), Hillis-Steele
    /// for medium (8–16 bits), and Kogge-Stone for larger widths. The result is the
    /// wrapping sum of the two operands.
    ///
    /// Both operands must have the same [`CiphertextSpec`]. For explicit algorithm
    /// selection, use [`iop_add_ripple_carry`](Self::iop_add_ripple_carry),
    /// [`iop_add_hillis_steele`](Self::iop_add_hillis_steele), or
    /// [`iop_add_kogge_stone`](Self::iop_add_kogge_stone).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(32, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let sum = builder.iop_adds(&a, &b);
    /// ```
    pub fn iop_adds(&self, lhs: &Ciphertext, rhs: &Plaintext) -> Ciphertext {
        let par_w = match lhs.spec().int_size() {
            8..16 => 1,
            16..24 => 7,
            24..256 => 12,
            _ => 1,
        };
        match lhs.spec().int_size() {
            0..8 => self.iop_adds_ripple_carry(&lhs, &rhs, None).0,
            8..17 => self.iop_adds_hillis_steele(&lhs, &rhs, None).0,
            17..256 => self.iop_adds_kogge_stone(&lhs, &rhs, None, par_w).0,
            _ => todo!(),
        }
    }

    /// Adds an encrypted integer and an immediate with overflow (carry) detection.
    ///
    /// Returns `(sum, overflow)` where `sum` is `lhs + rhs` (wrapping) and `overflow` is a
    /// single-block ciphertext: 1 if the unsigned sum does not fit in the operand width, 0
    /// otherwise.
    ///
    /// The carry-out of the adder *is* the overflow flag, so this costs exactly the same as
    /// [`iop_adds`](Self::iop_adds) -- the flag comes for free, and dead code elimination is
    /// what makes it disappear from [`iop_adds`](Self::iop_adds).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(32, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let (sum, overflow) = builder.iop_overflow_adds(&a, &b);
    /// ```
    pub fn iop_overflow_adds(&self, lhs: &Ciphertext, rhs: &Plaintext) -> (Ciphertext, Ciphertext) {
        let par_w = match lhs.spec().int_size() {
            8..16 => 1,
            16..24 => 7,
            24..256 => 12,
            _ => 1,
        };
        match lhs.spec().int_size() {
            0..8 => self.iop_adds_ripple_carry(lhs, rhs, None),
            8..17 => self.iop_adds_hillis_steele(lhs, rhs, None),
            17..256 => self.iop_adds_kogge_stone(lhs, rhs, None, par_w),
            _ => todo!(),
        }
    }

    /// Adds two encrypted integers using sequential ripple-carry propagation.
    ///
    /// Processes blocks from LSB to MSB, computing each block's sum and carry in turn.
    /// The optional `cin` injects an initial carry (useful for subtraction via two's
    /// complement). Each block requires two PBS operations: one to extract the message
    /// and one to extract the carry.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(8, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let (sum, carry_out) = builder.iop_adds_ripple_carry(&a, &b, None);
    /// ```
    pub fn iop_adds_ripple_carry(
        &self,
        lhs: &Ciphertext,
        rhs: &Plaintext,
        cin: Option<&CiphertextBlock>,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.plaintext_split(rhs);

        let mut carry = cin.cloned().unwrap_or_else(|| self.block_let_ciphertext(0));
        let mut output_blocks = Vec::new();
        for i in 0..lhs_blocks.iter().len() {
            self.push_comment(format!("{i}-th"));
            let raw_sum = self.block_add_plaintext(lhs_blocks[i], rhs_blocks[i]);
            let sum = self.block_add(raw_sum, carry);
            let (message, carry_tmp) = self.block_lookup2(sum, Lut2Def::ManyCarryMsg);
            carry = carry_tmp;
            output_blocks.push(message);
            self.pop_comment();
        }

        // carry is now the carry-out of the last block (clean 0/1 via CarryInMsg)
        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry").ciphertext_join([carry], None),
        )
    }

    /// Adds two encrypted integers using Hillis-Steele carry propagation.
    ///
    /// Groups blocks into fours, computes per-group propagation states, then resolves
    /// inter-group carries with a parallel prefix scan. The optional `cin` injects an
    /// initial carry into the LSB position. This algorithm offers O(log n) depth for
    /// n groups, making it efficient for medium-width integers (roughly 8–16 blocks).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let (sum, carry_out) = builder.iop_adds_hillis_steele(&a, &b, None);
    /// ```
    pub fn iop_adds_hillis_steele(
        &self,
        lhs: &Ciphertext,
        rhs: &Plaintext,
        cin: Option<&CiphertextBlock>,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.plaintext_split(rhs);

        let (output_blocks, carry_out) =
            self.iop_adds_hillis_steele_raw(lhs_blocks, rhs_blocks, cin, true);

        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry")
                .ciphertext_join([carry_out], None),
        )
    }

    pub(super) fn iop_adds_hillis_steele_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[PlaintextBlock]>,
        cin: Option<&CiphertextBlock>,
        clean: bool,
    ) -> (Vec<CiphertextBlock>, CiphertextBlock) {
        // Implements the addition with carry-propagation using the hillis-steele resolution and
        // group of size 4. The encoding of propagation status is the same as the one used
        // in TFHE-RS. The carry is resolved as soon as possible.
        //
        // Dead code elimination
        // =====================
        //
        // Depending on the size of the input integer, the computation may require some care to
        // handle edge cases:
        // + Non multiple of 4 number of blocks (due to 4-grouping for status computation)
        // + Non power-of-two number of groups (due to the hillis-steele resolution)
        //
        // Hopefully, thanks to dead-code elimination happening down the pipeline, we can describe
        // the computation in a larger, more favorable case, and let DCE cut the un-necessary
        // computation. This improves code readability.

        let mut sums = self.comment("Raw sum").vector_add_plaintext(
            &lhs_blocks,
            &rhs_blocks,
            ExtensionBehavior::Passthrough,
        );
        if let Some(c) = cin {
            sums[0] = self.block_add(&sums[0], c);
        }

        let output_size = sums.len();
        let compute_size = sums.len().next_multiple_of(4).next_power_of_two();
        let sums = self
            .comment("Extend sum")
            .vector_unsigned_extension(sums, compute_size);

        self.push_comment("Block States");
        let block_states = sums
            .iter()
            .chunk(4)
            .map(|c| c.unwrap_complete())
            .map_first(|sum| {
                [
                    self.comment("G0-B0")
                        .block_lookup2(sum[0], Lut2Def::ManyCarryMsg)
                        .1,
                    self.comment("G0-B1")
                        .block_lookup(sum[1], Lut1Def::ExtractPropGroup0),
                    self.comment("G0-B2")
                        .block_lookup(sum[2], Lut1Def::ExtractPropGroup1),
                    self.comment("G0-B3")
                        .block_lookup(sum[3], Lut1Def::ExtractPropGroup2),
                ]
            })
            .map_rest(|sum| {
                [
                    self.comment("GN-B0")
                        .block_lookup(sum[0], Lut1Def::ExtractPropGroup0),
                    self.comment("GN-B1")
                        .block_lookup(sum[1], Lut1Def::ExtractPropGroup1),
                    self.comment("GN-B2")
                        .block_lookup(sum[2], Lut1Def::ExtractPropGroup2),
                    self.comment("GN-B3")
                        .block_padding_lookup(sum[3], Lut1Def::ExtractPropGroup3),
                ]
            })
            .cosvec();
        self.pop_comment();

        self.push_comment("Group states");
        let group_states = block_states
            .iter()
            .map_first(|states| {
                // NB: group #0 is particular, since the status is actually
                // the carry value => This group is directly solved
                let b0 = states[0];
                let b1 = self.block_add(&b0, &states[1]);
                let b2 = self.block_add(&b1, &states[2]);
                let b3 = self.block_temper_add(&b2, &states[3]);
                let b3 = self.block_lookup(&b3, Lut1Def::SolvePropGroupFinal2);
                [
                    self.comment("G0-B0").block_inspect(b0),
                    self.comment("G0-B1").block_inspect(b1),
                    self.comment("G0-B2").block_inspect(b2),
                    self.comment("G0-B3").block_inspect(b3),
                ]
            })
            .map_rest(|states| {
                let b0 = states[0];
                let b1 = self.block_add(&b0, &states[1]);
                let b2 = self.block_add(&b1, &states[2]);
                let b3 = self.block_temper_add(&b2, &states[3]);
                let b3 = self.block_wrapping_lookup(&b3, Lut1Def::ReduceCarryPad);
                let b3 = self.block_wrapping_add_plaintext(&b3, &self.block_let_plaintext(1));
                [
                    self.comment("GN-B0").block_inspect(b0),
                    self.comment("GN-B1").block_inspect(b1),
                    self.comment("GN-B2").block_inspect(b2),
                    self.comment("GN-B3").block_inspect(b3),
                ]
            })
            .cosvec();
        self.pop_comment();

        self.push_comment("Group carries");
        let mut group_carries = group_states.iter().map(|group| group[3]).cosvec();
        let nb_groups = group_carries.len();
        let nb_stages = (nb_groups as f32).log2().ceil() as usize;
        for stage in 0..nb_stages {
            self.push_comment(format!("HS {stage}-th stage"));
            let stride = 1usize << stage;
            group_carries = group_carries
                .into_iter()
                // We chunk by increasing stride, and assume complete chunks.
                .chunk(stride)
                .map(|c| c.unwrap_complete())
                // We need to assemble data from two chunks later down the pipe.
                // Prelude will be useful for the first chunk, as we will see,
                // but Postlude is not needed.
                .slide::<2>()
                .skip_postludes()
                // The first chunk of the result is already solved at the previous level.
                // We get it from the prelude of the slide, and call it a day.
                .map_first(|slider| {
                    let sv = slider.unwrap_prelude();
                    sv[0].clone().into_iter().reconcile_1_of_2()
                })
                // The next chunk combines two chunks of the previous stage with the carry lut.
                .map_first(|slider| {
                    let [prev_carry, status] = slider.unwrap_complete().into_array();
                    self.vector_zip_then_lookup(
                        status,
                        prev_carry,
                        Lut1Def::SolvePropCarry,
                        ExtensionBehavior::Panic,
                    )
                    .into_iter()
                    .reconcile_2_of_2()
                })
                // The rest of the chunks combine chunks of the previous stage with the prop lut.
                .map_rest(|slider| {
                    let [prev_carry, status] = slider.unwrap_complete().into_array();
                    self.vector_zip_then_lookup(
                        status,
                        prev_carry,
                        Lut1Def::SolveProp,
                        ExtensionBehavior::Panic,
                    )
                    .into_iter()
                    .reconcile_2_of_2()
                })
                .flatten()
                // We only take enough to build the new iterate.
                .take(nb_groups)
                .collect();
            assert_eq!(group_carries.len(), nb_groups);
            self.pop_comment();
        }
        self.pop_comment();

        self.push_comment("Final resolution");
        let carries = (group_states.into_iter(), group_carries.into_iter())
            .mzip()
            .slide::<2>()
            .skip_postludes()
            .map_first(|slider| {
                let (states, carry) = slider.unwrap_prelude()[0];
                let b1 = self.block_lookup(&states[1], Lut1Def::SolvePropGroupFinal0);
                let b2 = self.block_lookup(&states[2], Lut1Def::SolvePropGroupFinal1);
                [
                    self.comment("G0-B0").block_inspect(states[0]),
                    self.comment("G0-B1").block_inspect(b1),
                    self.comment("G0-B2").block_inspect(b2),
                    self.comment("G0-B3").block_inspect(carry),
                ]
            })
            .map_rest(|slider| {
                let [(_, previous_carry), (states, carry)] = slider.unwrap_complete().into_array();
                let b0 = self.block_add(&states[0], &previous_carry);
                let b0 = self.block_lookup(&b0, Lut1Def::SolvePropGroupFinal0);
                let b1 = self.block_add(&states[1], &previous_carry);
                let b1 = self.block_lookup(&b1, Lut1Def::SolvePropGroupFinal1);
                let b2 = self.block_add(&states[2], &previous_carry);
                let b2 = self.block_lookup(&b2, Lut1Def::SolvePropGroupFinal2);
                [
                    self.comment("GN-B0").block_inspect(b0),
                    self.comment("GN-B1").block_inspect(b1),
                    self.comment("GN-B2").block_inspect(b2),
                    self.comment("GN-B3").block_inspect(carry),
                ]
            })
            .flatten()
            .cosvec();
        self.pop_comment();

        self.push_comment("Carry propagation");
        let mut result = svec![self.block_lookup2(&sums[0], Lut2Def::ManyCarryMsg).0];
        result.extend(
            (sums.into_iter().skip(1), carries.into_iter())
                .mzip()
                .map(|(sum, carry)| self.block_add(&sum, &carry)),
        );
        self.pop_comment();

        let carry_out = self.block_lookup(&result[output_size - 1], Lut1Def::CarryIsSome);

        if clean {
            self.push_comment("Cleanup");
            result = result
                .into_iter()
                .map(|ct| self.block_lookup(&ct, Lut1Def::MsgOnly))
                .cosvec();
            self.pop_comment();
        }

        (result.as_slice()[..output_size].into(), carry_out)
    }
}

impl Builder {
    /// Adds an encrypted integer with a plaintext using Kogge-Stone carry propagation.
    ///
    /// Builds a prefix tree over generate-propagate (PG) encoded carries, lazily computing
    /// and reducing intermediate MAC values. The `par_w` parameter controls the chunk width:
    /// carries are resolved within each chunk, then chained across chunks. Larger `par_w`
    /// reduces PBS count at the cost of deeper trees; values around 7–12 work well for
    /// typical 16–64 bit integers.
    ///
    /// Returns `(sum, carry_out)` where `carry_out` is a single-block ciphertext
    /// encoding the final carry (1 if overflow occurred, 0 otherwise).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(32, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.plaintext_input(spec.int_size());
    /// let (sum, carry_out) = builder.iop_adds_kogge_stone(&a, &b, None, 12);
    /// ```
    pub fn iop_adds_kogge_stone(
        &self,
        lhs: &Ciphertext,
        rhs: &Plaintext,
        cin: Option<&CiphertextBlock>,
        par_w: usize,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.plaintext_split(rhs);
        let (output_blocks, carry_out) =
            self.iop_adds_kogge_stone_raw(lhs_blocks, rhs_blocks, cin, par_w, false);
        let co_issome = self.block_lookup(&carry_out, Lut1Def::IsSome);
        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry")
                .ciphertext_join([co_issome], None),
        )
    }

    /// Raw Kogge-Stone addition on block slices, with optional carry-in and
    /// parallel-width chunking.
    pub(crate) fn iop_adds_kogge_stone_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[PlaintextBlock]>,
        cin: Option<&CiphertextBlock>,
        par_w: usize,
        clean: bool,
    ) -> (Vec<CiphertextBlock>, CiphertextBlock) {
        let sums = self.comment("Raw sum").vector_add_plaintext(
            &lhs_blocks,
            &rhs_blocks,
            ExtensionBehavior::Passthrough,
        );

        // Convert cin to PG encoding (or zero if absent).
        let cin_pg = match cin {
            Some(c) => {
                // this is only working if carry is in fact a plaintext block
                // which is the case for subtraction
                // TODO: find a way to support ciphertext carry in if needed
                let two = self.block_let_plaintext(2);
                self.block_mul_plaintext(c, &two)
            }
            None => self.block_let_ciphertext(0),
        };
        let mut cin_pg_kogge_entry = KoggeEntry {
            block: cin_pg,
            cpos: 1,
            fresh: cin_pg,
        };

        let n = sums.len();
        let mut result = Vec::with_capacity(n);

        // Process chunks of par_w, chaining carry-out → carry-in.
        let mut pos = 0;
        while pos < n {
            let end = (pos + par_w).min(n);
            let chunk = &sums[pos..end];

            self.push_comment(format!("Kogge chunk [{pos}..{end})"));
            let (chunk_result, carry_out) = self.kogge_propagate_carry(chunk, &cin_pg_kogge_entry);
            self.pop_comment();

            result.extend(chunk_result);
            cin_pg_kogge_entry = carry_out.clone();
            pos = end;
        }

        // Carry-out: the final PG entry spans cin through all blocks.
        // Because it is a PG carry the carry is really in bit 1
        let carry_out = cin_pg_kogge_entry.fresh;

        if clean {
            self.push_comment("Cleanup");
            result = result
                .into_iter()
                .map(|ct| self.block_lookup(&ct, Lut1Def::MsgOnly))
                .collect();
            self.pop_comment();
        }

        (result, carry_out)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    // use zhc_utils::assert_display_is;

    #[test]
    fn test_adds() {
        let spec = CiphertextSpec::new(18, 2, 2);
        let ir = adds(spec).optimize_ir();
        println!(
            "{}",
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_types(false)
        );
    }

    #[test]
    fn correctness_adds_hillis_steele() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.adds(*rhs))])
        }
        for size in (2..128).step_by(2) {
            adds_hillis_steele(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_adds_ripple() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.adds(*rhs))])
        }
        for size in (2..128).step_by(2) {
            adds_ripple_carry(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_adds_kogge_stone() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.adds(*rhs))])
        }
        for size in (2..128).step_by(2) {
            adds_kogge_stone(CiphertextSpec::new(size, 2, 2), 12).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_overflow_adds() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Plaintext(rhs)] = inp else {
                unreachable!()
            };
            let (sum, flag) = lhs.overflow_adds(*rhs);
            Some(vec![IopValue::Ciphertext(sum), IopValue::Ciphertext(flag)])
        }
        for size in (2..128).step_by(2) {
            // overflow_adds(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
            let spec = CiphertextSpec::new(size, 2, 2);
            let builder = Builder::new(spec.block_spec());
            let src_c = builder.ciphertext_input(spec.int_size());
            let src_p = builder.plaintext_input(spec.int_size());
            let (res, flag) = builder.iop_overflow_adds(&src_c, &src_p);
            builder.ciphertext_output(res);
            builder.ciphertext_output(flag);

            builder.test_random(100, semantic);

            // `test_random` on its own only ever observes the flag at 0: `CiphertextSpec::random`
            // draws its bit-window bounds from `1..int_size`, so the top bit of an operand is
            // never set and the sum can never carry out. Stimulate both answers explicitly.
            let max = spec.int_mask();
            let half = max / 2;
            let msb = half + 1;
            for (a, b) in [
                (max, 1),    // wraps around to 0
                (max, max),  // widest overflow
                (msb, msb),  // sum is exactly 2^int_size
                (msb, half), // largest sum that does not overflow
                (max, 0),    // no overflow
                (0, 0),      // no overflow
            ] {
                let src_p_value = src_p.make_value(b);
                let inputs = vec![IopValue::Ciphertext(spec.from_int(a)), src_p_value.clone()];
                let outputs = builder.interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(
                    outputs,
                    semantic(&inputs).unwrap(),
                    "overflow_adds({a:#x}, {b:#x}) on {size} bits"
                );
            }
        }
    }

    //#[test]
    // fn adds_ripple_comment() {
    //  let size = 4;
    //  let bd = adds_ripple_carry(CiphertextSpec::new(size, 2, 2));
    //  println!("{}", bd.dump_to_string());
    //}

    //#[test]
    // fn adds_hillis_steele_comment() {
    //  let size = 8;
    //  let bd = adds_hillis_steele(CiphertextSpec::new(size, 2, 2));
    //  println!("{}", bd.dump_to_string());
    //}

    //#[test]
    // fn adds_kogge_comment() {
    //  let size = 17;
    //  let bd = adds_kogge_stone(CiphertextSpec::new(size, 2, 2), 12);
    //  println!("{}", bd.dump_to_string());
    //}
}
