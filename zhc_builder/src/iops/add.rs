use std::collections::HashMap;

use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_ir::partition::PartitionId;
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::{
    iter::{ChunkIt, CollectInSmallVec, IterMapFirst, MultiZip, ReconcilerOf2, Slide, SliderExt},
    small::SmallVec,
    svec,
};

use crate::{
    CiphertextBlock,
    builder::{Builder, Ciphertext, ExtensionBehavior},
};

/// The native encoding of an adder's carry-out block.
///
/// A carry-out is not always a 0/1 flag.
/// Ripple and Hillis-Steele end on a LUT that already yields 0/1;
/// Kogge-Stone prefix network yields a PG status, in which a carry reads as `2`.
///
/// Both encodings are a single `CiphertextBlock` holding an encrypted value, so nothing can recover which one it is:
/// the encoding has to travel alongside the block, which is what this tag does.
/// Read a PG carry as a flag and you get `2` where `1` was expected, with nothing to signal it.
/// [`Builder::resolve_carry`] converts a carry of any encoding into a 0/1 flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryEncoding {
    /// Already a clean 0/1 block (Needs no further LUT):
    /// ripple (`ManyCarryMsg`), Hillis-Steele and single-board grouped-BS (`CarryIsSome`).
    Clean,

    /// PG (propagate/generate) carry status :
    /// `0` kill, `1` propagate, `2` generate. Must be Cleaned to a 0/1 flag with `IsSome` LUT.
    /// Kogge-Stone, Beaumont-Smith, and grouped-BS v2 (single- and multi-board).
    Pg,

    /// Already 0/1 because the adder resolved it itself (Needs no further LUT):
    /// CCS (single- and multi-board) and grouped-BS multi-board. Like `Clean` for `resolve_carry`.
    Resolved,
}

/// An adder carry-out block tagged with its [`CarryEncoding`].
///
/// Produced by every `_raw` adder in its native encoding.
/// Consumers call [`Builder::resolve_carry`] for a clean 0/1 flag, or read [`block`](Self::block) directly when
/// they deliberately consume the native encoding, as `iop_overflow_sub` does by feeding a PG carry to `IsNull`.
#[derive(Debug, Clone, Copy)]
pub struct CarryOut {
    block: CiphertextBlock,
    encoding: CarryEncoding,
}

impl CarryOut {
    /// Tag `block` as an already clean 0/1 carry.
    pub fn clean(block: CiphertextBlock) -> Self {
        Self {
            block,
            encoding: CarryEncoding::Clean,
        }
    }

    /// Tag `block` as a PG-encoded carry status.
    pub fn pg(block: CiphertextBlock) -> Self {
        Self {
            block,
            encoding: CarryEncoding::Pg,
        }
    }

    /// Tag `block` as a 0/1 carry the adder resolved itself.
    pub fn resolved(block: CiphertextBlock) -> Self {
        Self {
            block,
            encoding: CarryEncoding::Resolved,
        }
    }

    /// The raw carry block in its native encoding (see [`Self::encoding`]).
    pub fn block(&self) -> CiphertextBlock {
        self.block
    }

    /// The carry block's native encoding.
    pub fn encoding(&self) -> CarryEncoding {
        self.encoding
    }
}

/// Creates an IR for addition of two encrypted integers.
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_add`].
/// See that method for algorithm details.
pub fn add(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_add(&src_a, &src_b, None);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for batched (SIMD) addition of `SIMD_N` pairs of encrypted integers.
///
/// Declares `SIMD_N * 2` inputs and `SIMD_N` outputs. Each pair is added independently
/// using [`Builder::iop_add_ripple_carry`]. Optimized for throughput over latency.
pub fn add_simd(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    for _ in 0..crate::SIMD_N {
        let src_a = builder.ciphertext_input(spec.int_size());
        let src_b = builder.ciphertext_input(spec.int_size());
        let res = builder.iop_add_ripple_carry(&src_a, &src_b, None).0;
        builder.ciphertext_output(res);
    }
    builder
}

/// Creates an IR for addition using Kogge-Stone carry propagation.
///
/// Convenience wrapper that calls [`Builder::iop_add_kogge_stone`] with explicit
/// `par_w`. See that method for algorithm details.
pub fn add_kogge_stone(spec: CiphertextSpec, par_w: usize) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (res, _) = builder.iop_add_kogge_stone(&src_a, &src_b, None, par_w);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for addition using a Beaumont-Smith parallel prefix.
pub fn add_beaumont_smith(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (res, _) = builder.iop_add_beaumont_smith(&src_a, &src_b, None);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for addition using the grouped Beaumont-Smith adder.
pub fn add_bs_grouped(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (a_b, b_b) = (
        builder.ciphertext_split(&src_a),
        builder.ciphertext_split(&src_b),
    );
    let (res, _) = builder.iop_add_bs_grouped_raw(a_b, b_b, None);
    builder.ciphertext_output(builder.ciphertext_join(res, Some(spec.int_size())));
    builder
}

/// Creates an IR for addition using the depth-reduced grouped Beaumont-Smith (v2).
pub fn add_bs_grouped_v2(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (a_b, b_b) = (
        builder.ciphertext_split(&src_a),
        builder.ciphertext_split(&src_b),
    );
    let (res, _) = builder.iop_add_bs_grouped_v2_raw(a_b, b_b, None);
    builder.ciphertext_output(builder.ciphertext_join(res, Some(spec.int_size())));
    builder
}

/// Creates an IR for addition using the Compressed-Carry-State (CCS) prefix.
pub fn add_ccs(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (a_b, b_b) = (
        builder.ciphertext_split(&src_a),
        builder.ciphertext_split(&src_b),
    );
    let (res, _) = builder.iop_add_ccs_raw(a_b, b_b, None);
    builder.ciphertext_output(builder.ciphertext_join(res, Some(spec.int_size())));
    builder
}

/// Creates an IR for a Beaumont-Smith addition partitioned across `n_boards`.
pub fn add_beaumont_smith_mh(spec: CiphertextSpec, n_boards: usize) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let a_blocks = builder.ciphertext_split(&src_a);
    let b_blocks = builder.ciphertext_split(&src_b);
    let (res, _cout) = builder.iop_add_beaumont_smith_mh_raw(&a_blocks, &b_blocks, None, n_boards);
    let out = builder.ciphertext_join(res, Some(spec.int_size()));
    builder.ciphertext_output(out);
    builder
}

/// Creates an IR for addition using Hillis-Steele carry propagation.
///
/// Convenience wrapper that calls [`Builder::iop_add_hillis_steele`]. Prefer [`add`]
/// for automatic algorithm selection. See the builder method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, add_hillis_steele};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = add_hillis_steele(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn add_hillis_steele(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (res, _) = builder.iop_add_hillis_steele(&src_a, &src_b, None);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for addition using ripple-carry propagation.
///
/// Convenience wrapper that calls [`Builder::iop_add_ripple_carry`]. Prefer [`add`]
/// for automatic algorithm selection. See the builder method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, add_ripple_carry};
/// # let spec = CiphertextSpec::new(8, 2, 2);
/// let builder = add_ripple_carry(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn add_ripple_carry(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (res, _) = builder.iop_add_ripple_carry(&src_a, &src_b, None);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for addition with overflow detection.
///
/// Convenience wrapper that calls [`Builder::iop_overflow_add`]. Returns two outputs:
/// the wrapping sum and a single-block overflow flag. See the builder method for details.
pub fn overflow_add(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (res, flag) = builder.iop_overflow_add(&src_a, &src_b, None);
    builder.ciphertext_output(res);
    builder.ciphertext_output(flag);
    builder
}

impl Builder {
    /// Adds two encrypted integers, automatically selecting the best algorithm.
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
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let sum = builder.iop_add(&a, &b, None);
    /// ```
    pub fn iop_add(
        &self,
        lhs: &Ciphertext,
        rhs: &Ciphertext,
        cin: Option<&CiphertextBlock>,
    ) -> Ciphertext {
        self.iop_overflow_add(lhs, rhs, cin).0
    }

    /// Adds two encrypted integers with overflow detection.
    ///
    /// Returns a tuple `(sum, overflow)` where `sum` is the wrapping addition result
    /// and `overflow` is a single-block ciphertext encoding the carry-out (1 if the
    /// addition overflowed, 0 otherwise). Automatically selects the best carry
    /// propagation algorithm based on bit-width.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (sum, overflow) = builder.iop_overflow_add(&a, &b, None);
    /// ```
    pub fn iop_overflow_add(
        &self,
        lhs: &Ciphertext,
        rhs: &Ciphertext,
        cin: Option<&CiphertextBlock>,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let int_size = lhs.spec().int_size();

        let (output_blocks, carry_out) = self.iop_add_raw(int_size, lhs_blocks, rhs_blocks, cin);
        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry")
                .ciphertext_join([carry_out.block()], None),
        )
    }

    pub fn iop_add_raw(
        &self,
        int_size: u16,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        match int_size {
            0..8 => self.iop_add_ripple_carry_raw(&lhs_blocks, &rhs_blocks, cin),
            8..17 => self.iop_add_hillis_steele_raw(&lhs_blocks, &rhs_blocks, cin, true),
            17..256 => {
                // select internal par_w based on integer_w
                let par_w = match int_size {
                    16..24 => 7,
                    24..256 => 12,
                    _ => 1,
                };
                self.iop_add_kogge_stone_raw(&lhs_blocks, &rhs_blocks, cin, par_w)
            }
            _ => todo!(),
        }
    }

    /// Encode an optional carry-in as a PG (propagate/generate) block for the prefix networks.
    /// `ManyGenProp` places generate in bit 1, so the `*2` turns a plaintext 0/1 carry-in into a "generate" PG value.
    /// With no carry-in, the neutral value is a fresh zero ciphertext.
    fn cin_to_pg(&self, cin: Option<&CiphertextBlock>) -> CiphertextBlock {
        match cin {
            Some(c) => {
                // this is only working if carry is in fact a plaintext block which is the case for subtraction
                // TODO: find a way to support ciphertext carry in if needed
                let two = self.block_let_plaintext(2);
                self.block_mul_plaintext(c, &two)
            }
            None => self.block_let_ciphertext(0),
        }
    }

    /// Normalize an adder carry-out to a clean 0/1 block.
    ///
    /// `Clean` and `Resolved` carries pass through untouched, emitting nothing;
    /// a `Pg` carry is cleaned with the `IsSome` LUT.
    pub fn resolve_carry(&self, c: CarryOut) -> CiphertextBlock {
        match c.encoding {
            CarryEncoding::Clean | CarryEncoding::Resolved => c.block,
            CarryEncoding::Pg => self.block_lookup(&c.block, Lut1Def::IsSome),
        }
    }

    /// Adds two encrypted integers using sequential ripple-carry propagation.
    ///
    /// Processes blocks from LSB to MSB, computing each block's sum and carry in turn.
    /// The optional `cin` injects an initial carry (useful for subtraction via two's
    /// complement). Each block requires two PBS operations: one to extract the message
    /// and one to extract the carry.
    ///
    /// Returns `(sum, carry_out)` where `carry_out` is a single-block ciphertext
    /// encoding the final carry (1 if overflow occurred, 0 otherwise).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(8, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (sum, carry_out) = builder.iop_add_ripple_carry(&a, &b, None);
    /// ```
    pub fn iop_add_ripple_carry(
        &self,
        lhs: &Ciphertext,
        rhs: &Ciphertext,
        cin: Option<&CiphertextBlock>,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let (output_blocks, carry_out) = self.iop_add_ripple_carry_raw(lhs_blocks, rhs_blocks, cin);
        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry")
                .ciphertext_join([carry_out.block()], None),
        )
    }

    /// Raw ripple-carry addition on block slices.
    ///
    /// The carry-out is [`CarryEncoding::Clean`]: it comes out of `ManyCarryMsg` and is already a 0/1 block.
    pub fn iop_add_ripple_carry_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        let mut carry = cin.cloned().unwrap_or_else(|| self.block_let_ciphertext(0));
        let mut output_blocks = Vec::new();
        let wider_inputs = lhs_blocks
            .as_ref()
            .iter()
            .len()
            .max(rhs_blocks.as_ref().iter().len());
        for i in 0..wider_inputs {
            self.push_comment(format!("{i}-th"));
            let raw_sum = match (lhs_blocks.as_ref().get(i), rhs_blocks.as_ref().get(i)) {
                (Some(lhs), Some(rhs)) => self.block_add(lhs, rhs),
                (Some(lhs), None) => lhs.clone(),
                (None, Some(rhs)) => rhs.clone(),
                _ => unreachable!(),
            };
            let sum = self.block_add(raw_sum, carry);
            let (message, carry_tmp) = self.block_lookup2(sum, Lut2Def::ManyCarryMsg);
            carry = carry_tmp;
            output_blocks.push(message);
            self.pop_comment();
        }

        // carry is now the carry-out of the last block (clean 0/1 via ManyCarryMsg)
        (output_blocks, CarryOut::clean(carry))
    }

    /// Adds two encrypted integers using Hillis-Steele carry propagation.
    ///
    /// Groups blocks into fours, computes per-group propagation states, then resolves
    /// inter-group carries with a parallel prefix scan. The optional `cin` injects an
    /// initial carry into the LSB position. This algorithm offers O(log n) depth for
    /// n groups, making it efficient for medium-width integers (roughly 8–16 blocks).
    ///
    /// Returns `(sum, carry_out)` where `carry_out` is a single-block ciphertext
    /// encoding the final carry (1 if overflow occurred, 0 otherwise).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (sum, carry_out) = builder.iop_add_hillis_steele(&a, &b, None);
    /// ```
    pub fn iop_add_hillis_steele(
        &self,
        lhs: &Ciphertext,
        rhs: &Ciphertext,
        cin: Option<&CiphertextBlock>,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let (output_blocks, carry_out) =
            self.iop_add_hillis_steele_raw(lhs_blocks, rhs_blocks, cin, true);
        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry")
                .ciphertext_join([carry_out.block()], None),
        )
    }

    /// Raw Hillis-Steele addition on block slices.
    ///
    /// Carry-out `Clean`, the carry-out is cleaned to a 0/1 flag by `CarryIsSome` inside
    /// [`grouped_finalize`](Self::grouped_finalize) (independent of the `clean` message flag).
    pub(crate) fn iop_add_hillis_steele_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
        clean: bool,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
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

        let (sums, output_size) = self.grouped_extend_sums(lhs_blocks, rhs_blocks, cin);
        let group_states = self.grouped_states(&sums);

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

        let carries = self.grouped_final_resolution(group_states, group_carries);
        let (result, carry_out) = self.grouped_finalize(sums, carries, output_size, clean);
        (
            result.as_slice()[..output_size].into(),
            CarryOut::clean(carry_out),
        )
    }

    // ----------------------------------------------------------------------------------------------------------------
    // Shared phases of the grouped carry adders
    // ----------------------------------------------------------------------------------------------------------------
    //
    // A "grouped" adder works on groups of 4 blocks.
    // `ExtractPropGroup` packs the 1-bit carry-propagation statuses of 4 consecutive blocks into a
    // single block for free (it is an arithmetic add, not a PBS), one PBS per group then summarises all 4.
    // Carry propagation over n blocks becomes a prefix problem over n/4 group carries.
    //
    // Every adder in the family runs the same five phases:
    //   1. `grouped_extend_sums`      block-wise sum, fold cin, 0-extend to a workable size
    //   2. `grouped_states`           per-block statuses -> [b0, b1, b2, group_carry] per group
    //   3. the inter-group prefix     resolve each group's incoming carry from the n/4 carries
    //   4. `grouped_final_resolution` group summary + incoming carry -> per-block carries
    //   5. `grouped_finalize`         carries into the messages, carry-out, optional cleanup
    //
    // Phase 3 is the only one they differ in, so it stays with each adder while 1, 2, 4 and 5
    // live here.

    /// First phase of the grouped carry adders:
    /// - raw block-wise sum, fold the optional carry-in into block 0,
    /// - 0-extend to next 4-multiple pow2 so the 4-grouping/prefix are on favorable size (+dead-code).
    /// Returns the extended sums and the original (pre-extension) output width.
    fn grouped_extend_sums(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, usize) {
        let mut sums = self.comment("Raw sum").vector_add(
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
        (sums, output_size)
    }

    /// Block-status + group-status computation shared by the grouped carry adders.
    /// Packs each 4-block group's carry-propagation statuses (tfhe-rs encoding) into per-block states,
    /// then reduces each group to a `[b0, b1, b2, group_carry]` summary.
    /// The adder-specific inter-group carry prefix then consumes `group[3]`.
    fn grouped_states(&self, sums: &[CiphertextBlock]) -> SmallVec<[CiphertextBlock; 4]> {
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
        group_states
    }

    /// Final per-block resolution shared by the grouped carry adders:
    /// given the per-group summaries and the resolved inter-group carries,
    /// produce the per-block carries used during carry propagation.
    fn grouped_final_resolution(
        &self,
        group_states: SmallVec<[CiphertextBlock; 4]>,
        group_carries: SmallVec<CiphertextBlock>,
    ) -> SmallVec<CiphertextBlock> {
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
        carries
    }

    /// Shared tail of the grouped carry adders:
    /// - carry propagation into the message blocks
    /// - carry-out extraction (from the last block before cleanup)
    /// - optional cleanup to message-only blocks
    /// Returns the (possibly extended) result blocks and carry-out.
    fn grouped_finalize(
        &self,
        sums: Vec<CiphertextBlock>,
        carries: SmallVec<CiphertextBlock>,
        output_size: usize,
        clean: bool,
    ) -> (SmallVec<CiphertextBlock>, CiphertextBlock) {
        self.push_comment("Carry propagation");
        let mut result = svec![self.block_lookup2(&sums[0], Lut2Def::ManyCarryMsg).0];
        result.extend(
            (sums.into_iter().skip(1), carries.into_iter())
                .mzip()
                .map(|(sum, carry)| self.block_add(&sum, &carry)),
        );
        self.pop_comment();

        // Carry-out: the last result block (before cleanup) has the carry-out
        // in its carry field. Extract it before MsgOnly strips carry info.
        let carry_out = self.block_lookup(&result[output_size - 1], Lut1Def::CarryIsSome);

        if clean {
            self.push_comment("Cleanup");
            result = result
                .into_iter()
                .map(|ct| self.block_lookup(&ct, Lut1Def::MsgOnly))
                .cosvec();
            self.pop_comment();
        }

        (result, carry_out)
    }

    /// Grouped Beaumont-Smith: `ExtractPropGroup` packs 4 blocks' statuses into one (~free),
    /// a Sklansky prefix runs over the `n/4` group carries, `SolvePropGroupFinal` resolves each block.
    /// Same grouping/resolve as iop_add_hillis_steele_raw; only the inter-group prefix differs
    /// (Sklansky vs HS scan): fewer PBS, and it partitions cleanly across boards.
    pub(crate) fn iop_add_bs_grouped_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        let (sums, output_size) = self.grouped_extend_sums(lhs_blocks, rhs_blocks, cin);
        let group_states = self.grouped_states(&sums);

        // Sklansky prefix over the group-carry statuses (`SolveProp`), then resolve each
        // group against group 0's resolved seed (`SolvePropCarry`).
        self.push_comment("Group carries");
        let mut group_carries = group_states.iter().map(|group| group[3]).cosvec();
        let nb_groups = group_carries.len();
        if nb_groups > 1 {
            let seed = group_carries[0];
            let mut ps: Vec<CiphertextBlock> = group_carries.iter().skip(1).copied().collect(); // statuses of groups 1+
            let m = ps.len();
            let mut d = 1usize;
            while d < m {
                let prev = ps.clone();
                let block = d * 2;
                self.push_comment(format!("Sklansky d{d}"));
                for q in 0..m {
                    let bs = (q / block) * block;
                    if q >= bs + d {
                        let pivot = prev[bs + d - 1];
                        ps[q] = self.block_pack_then_lookup(&prev[q], &pivot, Lut1Def::SolveProp);
                    }
                }
                self.pop_comment();
                d = block;
            }
            for g in 1..nb_groups {
                group_carries[g] =
                    self.block_pack_then_lookup(&ps[g - 1], &seed, Lut1Def::SolvePropCarry);
            }
        }
        self.pop_comment();

        let carries = self.grouped_final_resolution(group_states, group_carries);
        let (result, carry_out) = self.grouped_finalize(sums, carries, output_size, true);
        (
            result.as_slice()[..output_size].into(),
            CarryOut::clean(carry_out),
        )
    }

    /// Grouped Beaumont-Smith v2: depth-reduced `iop_add_bs_grouped_raw`.
    /// Removes two serial levels of v1 by folding cin into the group prefix (no separate seed-resolve)
    /// and fusing the resolve/clean tail into one `GenPropAdd` (as plain BS).
    /// Depth 8 -> 6 at 64-bit, ~same PBS.
    ///
    /// Carry-out `Pg` (`v[n_groups]`); clean with `IsSome`.
    pub(crate) fn iop_add_bs_grouped_v2_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        let sums = self.comment("Raw sum").vector_add(
            &lhs_blocks,
            &rhs_blocks,
            ExtensionBehavior::Passthrough,
        );
        let n = sums.len();
        let group = 4usize;

        // L1: ManyGenProp per block -> (PG carry-status, clean message).
        self.push_comment("GenProp");
        let mut pg = Vec::with_capacity(n);
        let mut msg = Vec::with_capacity(n);
        for (i, sum) in sums.iter().enumerate() {
            let (g, m) = self
                .comment(format!("{i}"))
                .block_lookup2(sum, Lut2Def::ManyGenProp);
            pg.push(g);
            msg.push(m);
        }
        self.pop_comment();

        // L2: per-group carry-out status = combine of the group's block PG values.
        self.push_comment("Group status");
        let n_groups = n.div_ceil(group);
        let mut group_status = Vec::with_capacity(n_groups);
        for j in 0..n_groups {
            let lo = j * group;
            let hi = (lo + group).min(n);
            let parts: Vec<CiphertextBlock> = pg[lo..hi].to_vec();
            group_status.push(self.comment(format!("{j}")).beaumont_smith_combine(&parts));
        }
        self.pop_comment();

        // Inclusive Sklansky prefix over [cin, group_status...]; `v[j]` = carry into group j.
        let cin_pg = self.cin_to_pg(cin);
        let mut entries = Vec::with_capacity(n_groups + 1);
        entries.push(cin_pg);
        entries.extend(group_status.iter().copied());
        self.push_comment("Group prefix");
        let v = self.beaumont_smith_prefix(entries);
        self.pop_comment();

        // Per-block carry = combine(carry-into-group, preceding block statuses), then fused GenPropAdd.
        self.push_comment("Expand + add");
        let mut outputs = Vec::with_capacity(n);
        for j in 0..n_groups {
            let lo = j * group;
            let hi = (lo + group).min(n);
            for b in lo..hi {
                let k = b - lo;
                let carry_pg = if k == 0 {
                    v[j]
                } else {
                    let mut parts = Vec::with_capacity(k + 1);
                    parts.push(v[j]);
                    parts.extend(pg[lo..b].iter().copied());
                    self.beaumont_smith_combine(&parts)
                };
                outputs.push(self.comment(format!("{b}")).block_pack_then_lookup(
                    &carry_pg,
                    &msg[b],
                    Lut1Def::GenPropAdd,
                ));
            }
        }
        self.pop_comment();

        (outputs, CarryOut::pg(v[n_groups]))
    }
}

// ---------------------------------------------------------------------------
// Kogge-Stone carry propagation
// ---------------------------------------------------------------------------

/// A single entry in the Kogge tree, holding both the raw accumulated value
/// and a reduced (fresh) PG-encoded version.
#[derive(Copy, Clone, Debug)]
struct KoggeEntry {
    /// Raw accumulated MAC value (may span multiple PG positions).
    block: CiphertextBlock,
    /// Bit-width of the raw value (number of PG positions accumulated).
    cpos: usize,
    /// Reduced PG-encoded value (cpos conceptually == 1).
    fresh: CiphertextBlock,
}

/// Kogge-Stone prefix tree over PG-encoded carry values.
///
/// Mirrors the `KoggeTree` in `tfhe-rs/.../kogge.rs`. The tree lazily
/// computes prefix reductions using MAC (multiply-accumulate via doubling)
/// and PBS reduction operations (`ReduceCarry2`, `ReduceCarry3`,
/// `ReduceCarryPad`).
struct KoggeTree<'a> {
    builder: &'a Builder,
    cache: HashMap<(usize, usize), KoggeEntry>,
    /// `carry_size + message_size` for the block spec (4 for (2,2) params).
    total_width: usize,
}

impl<'a> KoggeTree<'a> {
    fn new(builder: &'a Builder, inputs: Vec<KoggeEntry>) -> Self {
        let total_width = builder.spec().data_size() as usize;
        let mut cache = HashMap::new();
        for (i, block) in inputs.into_iter().enumerate() {
            cache.insert((i, i), block);
        }
        KoggeTree {
            builder,
            cache,
            total_width,
        }
    }

    /// Splits a range into two sub-ranges using Kogge decomposition.
    /// Identical to the `get_subindex` logic in the tfhe-rs reference.
    fn get_subindex(start: usize, end: usize) -> ((usize, usize), (usize, usize)) {
        let range = end - start + 1;
        let pow = 1usize << range.ilog2();
        let mid = if pow == range {
            start + (pow >> 1)
        } else {
            start + pow
        };
        ((start, mid - 1), (mid, end))
    }

    /// Recursively builds the subtree for the range `[start, end]` and
    /// caches the result.
    fn insert_subtree(&mut self, start: usize, end: usize) {
        if self.cache.contains_key(&(start, end)) {
            return;
        }

        let ((ls, le), (ms, me)) = Self::get_subindex(start, end);
        self.insert_subtree(ls, le);
        self.insert_subtree(ms, me);

        let lsb = &self.cache[&(ls, le)];
        let msb = &self.cache[&(ms, me)];

        let cpos_trial = lsb.cpos + msb.cpos;

        // Choose which values to combine and the resulting cpos / shift.
        let (lsb_val, msb_val, cpos, msb_shift) = if cpos_trial > self.total_width {
            if msb.cpos + 1 > self.total_width {
                // Both sides must be reduced.
                (&lsb.fresh, &msb.fresh, 2usize, 2u8)
            } else {
                // Only lsb side needs reduction.
                (&lsb.fresh, &msb.block, msb.cpos + 1, 2u8)
            }
        } else {
            // Raw values fit without reduction.
            (&lsb.block, &msb.block, cpos_trial, 1 << lsb.cpos)
        };

        // MAC: lsb_val + (2^log_shift) * msb_val — implemented via doubling.
        let mac = self.builder.block_mac(msb_val, lsb_val, msb_shift);

        // Reduce via PBS based on cpos.
        let fresh = match cpos {
            2 => self.builder.block_lookup(&mac, Lut1Def::ReduceCarry2),
            3 => self.builder.block_lookup(&mac, Lut1Def::ReduceCarry3),
            tw if tw == self.total_width => {
                let r = self
                    .builder
                    .block_wrapping_lookup(&mac, Lut1Def::ReduceCarryPad);
                self.builder
                    .block_wrapping_add_plaintext(&r, &self.builder.block_let_plaintext(1))
            }
            _ => unreachable!(
                "Unexpected cpos={cpos} with total_width={}",
                self.total_width
            ),
        };

        // all carry processed are inserted in the cache
        self.cache.insert(
            (start, end),
            KoggeEntry {
                block: mac,
                cpos,
                fresh,
            },
        );
    }

    /// Returns the prefix entry for the range `[start, end]`.
    fn get_prefix(&mut self, start: usize, end: usize) -> &KoggeEntry {
        self.insert_subtree(start, end);
        // here returning only the needed carry
        // a part of the cached carry will be removed
        // by dead code removal
        &self.cache[&(start, end)]
    }
}

impl Builder {
    /// Adds two encrypted integers using Kogge-Stone carry propagation.
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
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (sum, carry_out) = builder.iop_add_kogge_stone(&a, &b, None, 12);
    /// ```
    pub fn iop_add_kogge_stone(
        &self,
        lhs: &Ciphertext,
        rhs: &Ciphertext,
        cin: Option<&CiphertextBlock>,
        par_w: usize,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let (output_blocks, carry_out) =
            self.iop_add_kogge_stone_raw(lhs_blocks, rhs_blocks, cin, par_w);
        let co_issome = self.resolve_carry(carry_out);
        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry")
                .ciphertext_join([co_issome], None),
        )
    }

    /// Raw Kogge-Stone addition on block slices, with optional carry-in and
    /// parallel-width chunking.
    ///
    /// The carry-out is [`CarryEncoding::Pg`]: it is the full-span prefix entry,
    /// still in PG form, so it needs `IsSome` before it can be read as a 0/1 flag.
    pub(crate) fn iop_add_kogge_stone_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
        par_w: usize,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        let sums = self.comment("Raw sum").vector_add(
            &lhs_blocks,
            &rhs_blocks,
            ExtensionBehavior::Passthrough,
        );

        // Convert cin to PG encoding (or zero if absent).
        let cin_pg = self.cin_to_pg(cin);
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
        (result, CarryOut::pg(carry_out))
    }

    /// Propagates carries through a slice of carry-save sums using a Kogge
    /// tree. Returns `(output_blocks, carry_out_pg)`.
    fn kogge_propagate_carry(
        &self,
        sums: &[CiphertextBlock],
        cin_pg: &KoggeEntry,
    ) -> (Vec<CiphertextBlock>, KoggeEntry) {
        let n = sums.len();

        // Split each sum into (PG, msg) via ManyGenProp.
        let mut carry_vec = Vec::with_capacity(n + 1);
        let mut msgs = Vec::with_capacity(n);
        for (i, sum) in sums.iter().enumerate() {
            let (pg, msg) = self
                .comment(format!("GenProp {i}"))
                .block_lookup2(sum, Lut2Def::ManyGenProp);
            let pg_ke = KoggeEntry {
                block: pg,
                cpos: 1,
                fresh: pg,
            };
            carry_vec.push(pg_ke);
            msgs.push(msg);
        }

        // Build carry chain: [cin_pg, pg_0, pg_1, ..., pg_{n-1}]
        carry_vec.insert(0, cin_pg.clone());

        // Build Kogge carry_tree.
        let mut carry_tree = KoggeTree::new(self, carry_vec);

        // For each block, query the resolved carry and combine with msg.
        let mut output = Vec::with_capacity(n);
        for i in 0..n {
            let carry_fresh = carry_tree.get_prefix(0, i).fresh;
            // Pack carry_fresh (carry/high) + msg (message/low), then apply GenPropAdd.
            let resolved = self.block_pack_then_lookup(&carry_fresh, &msgs[i], Lut1Def::GenPropAdd);
            output.push(resolved);
        }

        // Carry-out is the prefix over the full chain.
        let cout = carry_tree.get_prefix(0, n);

        (output, *cout)
    }
}

// ---------------------------------------------------------------------------
// Beaumont-Smith carry propagation (radix-4 Sklansky parallel prefix)
// ---------------------------------------------------------------------------

/// Per-board partition bookkeeping for a multi-board operation.
///
/// Each board-phase is emitted into its own partition (`mh_phase`); `mh_merge` collapses each
/// board's phases into one, so `partitions` yields exactly `n_boards`. Threading one `MhBoards`
/// through several ops keeps board `j` on the same partition throughout.
pub(crate) struct MhBoards {
    n_boards: usize,
    /// Number of carry positions (`n_blocks + 1`), used for balanced placement.
    n_pos: usize,
    board_parts: Vec<Vec<PartitionId>>,
}

impl MhBoards {
    pub(crate) fn new(n_boards: usize, n_blocks: usize) -> Self {
        assert!(n_boards >= 1, "n_boards must be >= 1");
        assert!(
            n_boards <= n_blocks,
            "cannot split {n_blocks} blocks across {n_boards} boards"
        );
        // Input / split ops precede any new_partition and live in partition 0 ->
        // attribute them to board 0.
        let mut board_parts = vec![Vec::new(); n_boards];
        board_parts[0].push(PartitionId::new(0, "Inputs"));
        Self {
            n_boards,
            n_pos: n_blocks + 1,
            board_parts,
        }
    }

    /// Board owning carry position `q` (balanced contiguous ranges over `0..=n`).
    fn board_of(&self, q: usize) -> usize {
        (q * self.n_boards / self.n_pos).min(self.n_boards - 1)
    }
}

impl Builder {
    /// Opens a fresh partition for board `j`'s next phase and makes it current.
    fn mh_phase(&self, boards: &mut MhBoards, j: usize) {
        boards.board_parts[j].push(self.new_partition(format!("mh_b{j}")));
    }

    /// Merges every board's phase-partitions into a single group -> `n_boards`
    /// distinct partitions.
    pub(crate) fn mh_merge(&self, boards: &MhBoards) {
        for parts in &boards.board_parts {
            parts
                .iter()
                .cloned()
                .reduce(|acc, p| self.merge_partitions(acc, p));
        }
    }

    /// Adds two encrypted integers using a Beaumont-Smith parallel prefix.
    ///
    /// Radix-4 Sklansky prefix network over the same PG encoding as
    /// [`iop_add_kogge_stone`](Self::iop_add_kogge_stone).
    /// Sklansky trades fanout for depth, and fanout is free in TFHE (a ciphertext is just read again),
    /// so the network reaches its minimum depth of `1 + ceil(log4(n+1)) + 1` PBS levels for `n` blocks
    /// (5 levels for 64-bit).
    ///
    /// Every level is a set of independent per-position PBS that partition cleanly by block range,
    /// which is what makes this the adder to use when the work is spread over several HPU boards.
    /// On a single board prefer [`iop_add`](Self::iop_add).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(64, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let (sum, carry_out) = builder.iop_add_beaumont_smith(&a, &b, None);
    /// ```
    pub fn iop_add_beaumont_smith(
        &self,
        lhs: &Ciphertext,
        rhs: &Ciphertext,
        cin: Option<&CiphertextBlock>,
    ) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let (output_blocks, carry_out) =
            self.iop_add_beaumont_smith_raw(lhs_blocks, rhs_blocks, cin);
        let co_clean = self.resolve_carry(carry_out);
        (
            self.comment("Join Output")
                .ciphertext_join(output_blocks, None),
            self.comment("Join Carry").ciphertext_join([co_clean], None),
        )
    }

    /// Raw Beaumont-Smith addition on block slices, with optional carry-in.
    ///
    /// Carry-out `Pg`, returns `v[n]`, the PG-encoded carry-out (`IsSome` yields a clean 0/1 flag).
    pub(crate) fn iop_add_beaumont_smith_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        let sums = self.comment("Raw sum").vector_add(
            &lhs_blocks,
            &rhs_blocks,
            ExtensionBehavior::Passthrough,
        );
        let n = sums.len();

        // Split each raw sum into (pg, msg) via ManyGenProp.
        self.push_comment("GenProp");
        let mut pg = Vec::with_capacity(n);
        let mut msg = Vec::with_capacity(n);
        for (i, sum) in sums.iter().enumerate() {
            let (g, m) = self
                .comment(format!("{i}"))
                .block_lookup2(sum, Lut2Def::ManyGenProp);
            pg.push(g);
            msg.push(m);
        }
        self.pop_comment();

        // Carry entries e[0..=n]: e[0] = cin (PG-encoded), e[q] = pg[q-1].
        // The prefix combine is delegated to beaumont_smith_prefix below.
        let cin_pg = self.cin_to_pg(cin);
        let mut entries: Vec<CiphertextBlock> = Vec::with_capacity(n + 1);
        entries.push(cin_pg);
        entries.extend(pg.iter().copied());

        // v[q] = combine(e[0..=q]) = carry into block q; v[n] = carry-out.
        self.push_comment("Prefix");
        let v = self.beaumont_smith_prefix(entries);
        self.pop_comment();

        // Final: result block b = GenPropAdd(pack(carry = v[b], msg = msg[b])).
        self.push_comment("Carry propagation");
        let outputs: Vec<CiphertextBlock> = (0..n)
            .map(|b| {
                self.comment(format!("{b}")).block_pack_then_lookup(
                    &v[b],
                    &msg[b],
                    Lut1Def::GenPropAdd,
                )
            })
            .collect();
        self.pop_comment();

        // Carry-out is the prefix over every entry (PG-encoded).
        (outputs, CarryOut::pg(v[n]))
    }

    /// Combine 1..=`data_size` fresh PG carries (low..high) into one, via MAC-pack + one
    /// `ReduceCarry` PBS (a single input passes through). Radix-`data_size` prefix primitive.
    fn beaumont_smith_combine(&self, parts: &[CiphertextBlock]) -> CiphertextBlock {
        let total_width = self.spec().data_size() as usize;
        debug_assert!(
            (1..=total_width).contains(&parts.len()),
            "radix-{total_width} combine expects 1..={total_width} fresh PG values, got {}",
            parts.len()
        );
        if parts.len() == 1 {
            return parts[0];
        }
        // acc = parts[0] + 2*parts[1] + 4*parts[2] + ... (each fresh value occupies one
        // "position"); cpos tracks how many positions are now packed into acc.
        let mut acc = parts[0];
        let mut cpos = 1u8;
        for p in &parts[1..] {
            acc = self.block_mac(p, &acc, 1u8 << cpos);
            cpos += 1;
        }
        match cpos as usize {
            2 => self.block_lookup(&acc, Lut1Def::ReduceCarry2),
            3 => self.block_lookup(&acc, Lut1Def::ReduceCarry3),
            tw if tw == total_width => {
                let r = self.block_wrapping_lookup(&acc, Lut1Def::ReduceCarryPad);
                self.block_wrapping_add_plaintext(&r, &self.block_let_plaintext(1))
            }
            _ => unreachable!("cpos {cpos} out of range for data_size {total_width}"),
        }
    }

    /// Radix-4 Sklansky prefix gather: pure index math, emits no ops.
    ///
    /// At sub-block size `sub` (`block = 4*sub`), returns the PG values position `q` combines
    /// (preceding sub-block summaries + its running prefix), or `None` if `q` is already its
    /// sub-block's prefix. Pure, so BS and grouped-BS-mh share it while keeping their own placement.
    fn bs_prefix_gather(
        prev: &[CiphertextBlock],
        q: usize,
        sub: usize,
        block: usize,
    ) -> Option<Vec<CiphertextBlock>> {
        let bs = (q / block) * block; // block start
        let s_count = (q - bs) / sub; // number of preceding sub-blocks (0..=3)
        if s_count == 0 {
            return None; // v[q] already the prefix within its sub-block
        }
        let mut parts: Vec<CiphertextBlock> = (0..s_count)
            .map(|s| prev[bs + s * sub + sub - 1]) // summary = last of sub-block s
            .collect();
        parts.push(prev[q]);
        Some(parts)
    }

    /// Radix-4 Sklansky parallel prefix over `entries` (low..high): `prefix[q] = combine(entries[0..=q])`.
    /// One MAC + one `ReduceCarry` PBS per non-trivial position/level; depth `ceil(log4(n))` levels.
    fn beaumont_smith_prefix(&self, entries: Vec<CiphertextBlock>) -> Vec<CiphertextBlock> {
        let mut v = entries;
        let m = v.len().saturating_sub(1); // highest index
        let mut level = 0u32;
        loop {
            let sub = 4usize.pow(level); // sub-block size at this level
            if sub > m {
                break; // one block already spans every entry -> prefix is global
            }
            let block = sub * 4;
            let prev = v.clone();
            self.push_comment(format!("L{level}"));
            for q in 0..=m {
                if let Some(parts) = Self::bs_prefix_gather(&prev, q, sub, block) {
                    v[q] = self.comment(format!("{q}")).beaumont_smith_combine(&parts);
                }
            }
            self.pop_comment();
            level += 1;
        }
        v
    }

    /// Multi-board Beaumont-Smith: the single-board `iop_add_beaumont_smith_raw` value graph,
    /// each carry position placed on the board owning it (contiguous ranges), so correctness is
    /// inherited. A prefix level reads only prior-level summaries, so cross-board reads are one
    /// wave old (cheap transfers, not serial deps).
    ///
    /// Critical path `1 + ceil(log4(n+1)) + 1` PBS levels (5 at 64-bit), independent of `n_boards`;
    /// cross-board reads grow with board count, so past a few boards transfers dominate. Emits
    /// `n_boards` partitions.
    ///
    /// Carry-out `Pg` (`v[n]`); clean with `IsSome`.
    pub(crate) fn iop_add_beaumont_smith_mh_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
        n_boards: usize,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        let lhs = lhs_blocks.as_ref();
        let rhs = rhs_blocks.as_ref();
        let mut boards = MhBoards::new(n_boards, lhs.len());
        let out = self.bs_mh_add_shared(&mut boards, lhs, rhs, cin, true);
        self.mh_merge(&boards);
        out
    }

    /// One multi-board Beaumont-Smith add sharing an `MhBoards` context; appends per-board phases
    /// but does not merge, so a caller can thread `boards` through several adds and merge once.
    ///
    /// `want_sum == false` skips the finals (empty sum, only the carry-out meaningful, for a
    /// comparison); `optimize_ir` drops the dead prefix positions.
    ///
    /// Carry-out `Pg` (`v[n]`); clean with `IsSome`.
    fn bs_mh_add_shared(
        &self,
        boards: &mut MhBoards,
        lhs: &[CiphertextBlock],
        rhs: &[CiphertextBlock],
        cin: Option<&CiphertextBlock>,
        want_sum: bool,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        let n = lhs.len();
        assert_eq!(rhs.len(), n, "operands must have equal block counts");
        assert_eq!(
            boards.n_pos,
            n + 1,
            "board context sized for a different width"
        );
        let n_boards = boards.n_boards;

        let cin_pg = self.cin_to_pg(cin);

        // ---- GenProp: block b (-> pg[b] = e[b+1], msg[b]) on board_of(b) ----
        let mut pg: Vec<Option<CiphertextBlock>> = vec![None; n];
        let mut msg: Vec<Option<CiphertextBlock>> = vec![None; n];
        for j in 0..n_boards {
            self.mh_phase(boards, j);
            self.push_comment(format!("board{j} genprop"));
            for b in 0..n {
                if boards.board_of(b) != j {
                    continue;
                }
                let raw = self.block_add(&lhs[b], &rhs[b]);
                let (g, m) = self.block_lookup2(&raw, Lut2Def::ManyGenProp);
                pg[b] = Some(g);
                msg[b] = Some(m);
            }
            self.pop_comment();
        }

        // Carry entries e[0..=n]: e[0] = cin, e[q] = pg[q-1].
        let mut v: Vec<CiphertextBlock> = Vec::with_capacity(n + 1);
        v.push(cin_pg);
        v.extend((0..n).map(|b| pg[b].unwrap()));

        // ---- Global radix-4 Sklansky prefix, each position on board_of(q) ----
        let m = n; // highest position (carry-out)
        let mut level = 0u32;
        loop {
            let sub = 4usize.pow(level);
            if sub > m {
                break; // one block spans every position -> prefix is global
            }
            let block = sub * 4;
            let prev = v.clone(); // level l reads only level l-1 values
            for j in 0..n_boards {
                self.mh_phase(boards, j);
                self.push_comment(format!("board{j} L{level}"));
                for q in 0..=m {
                    if boards.board_of(q) != j {
                        continue;
                    }
                    if let Some(parts) = Self::bs_prefix_gather(&prev, q, sub, block) {
                        v[q] = self.beaumont_smith_combine(&parts);
                    }
                }
                self.pop_comment();
            }
            level += 1;
        }

        // ---- Finals: result block b = GenPropAdd(pack(v[b], msg[b])) on board_of(b) ----
        let outputs = if want_sum {
            let mut outputs: Vec<Option<CiphertextBlock>> = vec![None; n];
            for j in 0..n_boards {
                self.mh_phase(boards, j);
                self.push_comment(format!("board{j} final"));
                for b in 0..n {
                    if boards.board_of(b) != j {
                        continue;
                    }
                    outputs[b] = Some(self.block_pack_then_lookup(
                        &v[b],
                        &msg[b].unwrap(),
                        Lut1Def::GenPropAdd,
                    ));
                }
                self.pop_comment();
            }
            outputs.into_iter().map(|x| x.unwrap()).collect()
        } else {
            Vec::new()
        };

        (outputs, CarryOut::pg(v[n])) // v[n] = carry-out (PG-encoded)
    }

    // ===== Compressed-Carry-State (CCS) adder =====
    // Digits by msg_len: Block=2, CCS=1, CCS_bar=0. A prop_step MAC-packs a strided window and
    // applies `ccs` (-> carry-state) or `ccs_bar` (-> resolved carry);
    // radix grows as digits compress.
    // Early-merge stops the scan short, then a 2-PBS resolve finishes each block.

    /// `ccs`: reduce a packed window (msg-width `width`) to a carry-state {kill=0,prop=1,gen=2}.
    /// The CCS state is exactly BS's PG encoding, so the standard `ReduceCarry{k}` LUTs already
    /// compute it (widths 0|1 identity, 2/3 -> `ReduceCarry2`/`3`, >=4 -> `ReduceCarryPad` +1).
    fn ccs_state_apply(&self, packed: &CiphertextBlock, width: u32) -> CiphertextBlock {
        match width {
            0 | 1 => *packed,
            2 => self.block_lookup(packed, Lut1Def::ReduceCarry2),
            3 => self.block_lookup(packed, Lut1Def::ReduceCarry3),
            _ => {
                let r = self.block_wrapping_lookup(packed, Lut1Def::ReduceCarryPad);
                self.block_wrapping_add_plaintext(&r, &self.block_let_plaintext(1))
            }
        }
    }

    /// `ccs_bar`: resolve a packed window (msg-width `width`) to a carry {0,1} using existing LUTs.
    /// Width-`k` extracts bit `k`, which `SolvePropGroupFinal{k-1}` reads (widths 0 identity,
    /// 1/2/3 -> `SolvePropGroupFinal0`/`1`/`2`). Width >=4: bit 4 is in the pad bit, so two PBS
    /// (`ReduceCarryPad`+1 then `SolvePropGroupFinal0`) on the off-critical-path low block.
    fn ccs_bar_apply(&self, packed: &CiphertextBlock, width: u32) -> CiphertextBlock {
        match width {
            // A width-0 window is a lone already-resolved carry, so `packed` is the 0/1 value
            // itself: masking it again would only spend a PBS to reproduce it.
            0 => *packed,
            1 => self.block_lookup(packed, Lut1Def::SolvePropGroupFinal0),
            2 => self.block_lookup(packed, Lut1Def::SolvePropGroupFinal1),
            3 => self.block_lookup(packed, Lut1Def::SolvePropGroupFinal2),
            _ => {
                let status = self.ccs_state_apply(packed, width);
                self.block_lookup(&status, Lut1Def::SolvePropGroupFinal0)
            }
        }
    }

    /// Pack a window (low..high) into one block by MAC-shifting each digit by its msg_len.
    /// Returns (packed, total_width).
    fn ccs_pack(&self, window: &[(CiphertextBlock, u8)]) -> (CiphertextBlock, u32) {
        let mut acc = self.block_let_ciphertext(0);
        let mut shift = 0u32;
        for (blk, ml) in window {
            acc = self.block_mac(blk, &acc, 1u8 << shift);
            shift += *ml as u32;
        }
        (acc, shift)
    }

    /// Strided window of up to `n` digits ending at index `i` (i, i-step, ...), low..high.
    fn ccs_window(_state: &[(CiphertextBlock, u8)], i: usize, n: usize, step: usize) -> Vec<usize> {
        let mut idxs = Vec::with_capacity(n);
        let mut idx = i as isize;
        while idxs.len() < n && idx >= 0 {
            idxs.push(idx as usize);
            idx -= step as isize;
        }
        idxs.reverse();
        idxs
    }

    /// One position of a prop step: pack its strided window and reduce to a resolved carry
    /// (`msg_len` 0) or a carry-state (`msg_len` 1).
    /// Shared with the multi-board prefix, which visits only its board's positions.
    fn ccs_prop_position(
        &self,
        state: &[(CiphertextBlock, u8)],
        i: usize,
        n_inputs: usize,
        step: usize,
    ) -> (CiphertextBlock, u8) {
        let idxs = Self::ccs_window(state, i, n_inputs, step);
        let window: Vec<(CiphertextBlock, u8)> = idxs.iter().map(|&j| state[j]).collect();
        let (packed, width) = self.ccs_pack(&window);
        let lowest_ml = window.first().unwrap().1;
        // ccs_bar (resolved) if the lowest digit is CCS_bar, or a Block at a low
        // position whose window already reaches the LSB.
        let use_bar = lowest_ml == 0 || (lowest_ml == 2 && i < n_inputs);
        if use_bar {
            (self.ccs_bar_apply(&packed, width), 0u8)
        } else {
            (self.ccs_state_apply(&packed, width), 1u8)
        }
    }

    /// One growing-radix prop step. Returns (next state, radix used).
    fn ccs_prop_step(
        &self,
        state: &[(CiphertextBlock, u8)],
        step: usize,
    ) -> (Vec<(CiphertextBlock, u8)>, usize) {
        let in_msg_len = state.last().unwrap().1 as usize; // 2 (Block) or 1 (CCS)
        let n_inputs = 4 / in_msg_len;
        let n = state.len();
        let mut res = Vec::with_capacity(n);
        for i in 0..n {
            res.push(self.ccs_prop_position(state, i, n_inputs, step));
        }
        (res, n_inputs)
    }

    fn ccs_merge_needed(state: &[(CiphertextBlock, u8)], step: usize) -> bool {
        let last_ml = state.last().unwrap().1 as usize;
        if last_ml == 0 {
            return false; // everything resolved
        }
        let first_ml = state.first().unwrap().1 as usize;
        // (plaintext_len - (Block.msg_len + 1)) / last_ml + (first is CCS_bar ? 1 : 0)
        let n_ccs = (4 - (2 + 1)) / last_ml + usize::from(first_ml == 0);
        step * n_ccs + 1 < state.len()
    }

    /// Single-board CCS adder.
    ///
    /// Precondition: `cin` must be `None`. With a carry-in, block 0's sum can reach 7, the width-4
    /// pack hits `complete_mask` (31), and the carry-out resolves to 0 instead of 1.
    ///
    /// Carry-out `Resolved` (`ccs_bar` resolves the top window directly).
    pub(crate) fn iop_add_ccs_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        cin: Option<&CiphertextBlock>,
    ) -> (Vec<CiphertextBlock>, CarryOut) {
        debug_assert!(
            cin.is_none(),
            "iop_add_ccs_raw is correct only for cin=None (width-4 carry-out edge case)"
        );
        let sums = self.comment("Raw sum").vector_add(
            &lhs_blocks,
            &rhs_blocks,
            ExtensionBehavior::Passthrough,
        );
        let mut sums: Vec<CiphertextBlock> = sums.into_iter().collect();
        if let Some(c) = cin {
            sums[0] = self.block_add(&sums[0], c);
        }
        let n = sums.len();

        // Prefix over carry states.
        self.push_comment("CCS prefix");
        let mut state: Vec<(CiphertextBlock, u8)> = sums.iter().map(|s| (*s, 2u8)).collect();
        let mut step = 1usize;
        let mut level = 0u32;
        while Self::ccs_merge_needed(&state, step) {
            self.push_comment(format!("prop {level}"));
            let (next, l) = self.ccs_prop_step(&state, step);
            self.pop_comment();
            state = next;
            step *= l;
            level += 1;
        }
        self.pop_comment();

        self.push_comment("CCS resolve");
        let mut out = Vec::with_capacity(n);
        out.push(self.block_lookup(&sums[0], Lut1Def::MsgOnly));
        for b in 1..n {
            let top = state[b - 1];
            let carry = if top.1 == 0 {
                top.0
            } else {
                let idxs = Self::ccs_window(&state, b - 1, 2, step);
                let window: Vec<(CiphertextBlock, u8)> = idxs.iter().map(|&j| state[j]).collect();
                let (packed, width) = self.ccs_pack(&window);
                self.ccs_bar_apply(&packed, width)
            };
            out.push(self.block_lookup(&self.block_add(&sums[b], &carry), Lut1Def::MsgOnly));
        }
        let top = state[n - 1];
        let carry_out = if top.1 == 0 {
            top.0
        } else {
            let idxs = Self::ccs_window(&state, n - 1, 3, step);
            let window: Vec<(CiphertextBlock, u8)> = idxs.iter().map(|&j| state[j]).collect();
            let (packed, width) = self.ccs_pack(&window);
            self.ccs_bar_apply(&packed, width)
        };
        self.pop_comment();
        (out, CarryOut::resolved(carry_out))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    /// Expected sum of a wrapping add.
    fn add_semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
            unreachable!()
        };
        Some(vec![IopValue::Ciphertext(lhs.add(*rhs))])
    }

    /// Expected sum and overflow flag.
    fn overflow_add_semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
            unreachable!()
        };
        let (sum, flag) = lhs.overflow_add(*rhs);
        Some(vec![IopValue::Ciphertext(sum), IopValue::Ciphertext(flag)])
    }

    #[test]
    fn beaumont_smith_edge_cases() {
        use zhc_crypto::integer_semantics::EmulatedCiphertext;
        for bits in [8u16, 16, 32, 64, 128] {
            let spec = CiphertextSpec::new(bits, 2, 2);
            let mask: u128 = if bits == 128 {
                u128::MAX
            } else {
                (1u128 << bits) - 1
            };
            let alt = 0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAAu128 & mask;
            let alt2 = 0x5555_5555_5555_5555_5555_5555_5555_5555u128 & mask;
            // worst-case carry patterns: full propagation, MSB carry, propagate-no-generate
            let cases: &[(u128, u128)] = &[
                (mask, 1),      // all-ones + 1 -> 0, carry through EVERY block
                (mask, mask),   // all-ones + all-ones
                (mask >> 1, 1), // 0x7f..f + 1 -> carry into top bit
                (alt, alt2),    // 0b1010.. + 0b0101.. = all-ones, pure propagate
                (alt, 1),
                (0, 0),
                (mask, 0),
            ];
            let builder = add_beaumont_smith(spec);
            for &(av, bv) in cases {
                let inputs = vec![
                    IopValue::Ciphertext(EmulatedCiphertext::new(av, spec)),
                    IopValue::Ciphertext(EmulatedCiphertext::new(bv, spec)),
                ];
                let out = builder.interpret().with_inputs(&inputs).get_outputs();
                let IopValue::Ciphertext(res) = &out[0] else {
                    unreachable!()
                };
                let want = av.wrapping_add(bv) & mask;
                assert_eq!(
                    res.as_storage() & mask,
                    want,
                    "BS {bits}b wrong: {av:#x} + {bv:#x} = {:#x}, want {want:#x}",
                    res.as_storage() & mask
                );
            }
        }
    }

    #[test]
    fn test_add() {
        let spec = CiphertextSpec::new(18, 2, 2);
        let ir = add(spec).optimize_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_types(false),
            r#"
                                                    | %0 = input_ciphertext<0, 18>();
                                                    | %1 = input_ciphertext<1, 18>();
                                                    | %2 = extract_ct_block<0>(%0);
                                                    | %3 = extract_ct_block<1>(%0);
                                                    | %4 = extract_ct_block<2>(%0);
                                                    | %5 = extract_ct_block<3>(%0);
                                                    | %6 = extract_ct_block<4>(%0);
                                                    | %7 = extract_ct_block<5>(%0);
                                                    | %8 = extract_ct_block<6>(%0);
                                                    | %9 = extract_ct_block<7>(%0);
                                                    | %10 = extract_ct_block<8>(%0);
                                                    | %11 = extract_ct_block<0>(%1);
                                                    | %12 = extract_ct_block<1>(%1);
                                                    | %13 = extract_ct_block<2>(%1);
                                                    | %14 = extract_ct_block<3>(%1);
                                                    | %15 = extract_ct_block<4>(%1);
                                                    | %16 = extract_ct_block<5>(%1);
                                                    | %17 = extract_ct_block<6>(%1);
                                                    | %18 = extract_ct_block<7>(%1);
                                                    | %19 = extract_ct_block<8>(%1);
                // Raw sum                          | %20 = add_ct(%2, %11);
                // Raw sum                          | %21 = add_ct(%3, %12);
                // Raw sum                          | %22 = add_ct(%4, %13);
                // Raw sum                          | %23 = add_ct(%5, %14);
                // Raw sum                          | %24 = add_ct(%6, %15);
                // Raw sum                          | %25 = add_ct(%7, %16);
                // Raw sum                          | %26 = add_ct(%8, %17);
                // Raw sum                          | %27 = add_ct(%9, %18);
                // Raw sum                          | %28 = add_ct(%10, %19);
                                                    | %29 = let_ct_block<0>();
                // Kogge chunk [0..7) / GenProp 0   | %30, %31 = pbs2<Protect, Lut2("ManyGenProp")>(%20);
                // Kogge chunk [0..7) / GenProp 1   | %32, %33 = pbs2<Protect, Lut2("ManyGenProp")>(%21);
                // Kogge chunk [0..7) / GenProp 2   | %34, %35 = pbs2<Protect, Lut2("ManyGenProp")>(%22);
                // Kogge chunk [0..7) / GenProp 3   | %36, %37 = pbs2<Protect, Lut2("ManyGenProp")>(%23);
                // Kogge chunk [0..7) / GenProp 4   | %38, %39 = pbs2<Protect, Lut2("ManyGenProp")>(%24);
                // Kogge chunk [0..7) / GenProp 5   | %40, %41 = pbs2<Protect, Lut2("ManyGenProp")>(%25);
                // Kogge chunk [0..7) / GenProp 6   | %42, %43 = pbs2<Protect, Lut2("ManyGenProp")>(%26);
                // Kogge chunk [0..7)               | %44 = pack_ct<4>(%29, %31);
                // Kogge chunk [0..7)               | %45 = pbs<Protect, Lut1("GenPropAdd")>(%44);
                // Kogge chunk [0..7)               | %46 = pack_ct<2>(%30, %29);
                // Kogge chunk [0..7)               | %47 = pbs<Protect, Lut1("ReduceCarry2")>(%46);
                // Kogge chunk [0..7)               | %48 = pack_ct<4>(%47, %33);
                // Kogge chunk [0..7)               | %49 = pbs<Protect, Lut1("GenPropAdd")>(%48);
                // Kogge chunk [0..7)               | %50 = pack_ct<4>(%32, %46);
                // Kogge chunk [0..7)               | %51 = pbs<Protect, Lut1("ReduceCarry3")>(%50);
                // Kogge chunk [0..7)               | %52 = pack_ct<4>(%51, %35);
                // Kogge chunk [0..7)               | %53 = pbs<Protect, Lut1("GenPropAdd")>(%52);
                // Kogge chunk [0..7)               | %54 = pack_ct<2>(%34, %32);
                // Kogge chunk [0..7)               | %56 = pack_ct<4>(%54, %46);
                // Kogge chunk [0..7)               | %57 = pbs<AllowBothPadding, Lut1("ReduceCarryPad")>(%56);
                // Kogge chunk [0..7)               | %58 = let_pt_block<1>();
                // Kogge chunk [0..7)               | %59 = wrapping_add_pt(%57, %58);
                // Kogge chunk [0..7)               | %60 = pack_ct<4>(%59, %37);
                // Kogge chunk [0..7)               | %61 = pbs<Protect, Lut1("GenPropAdd")>(%60);
                // Kogge chunk [0..7)               | %62 = pack_ct<2>(%36, %59);
                // Kogge chunk [0..7)               | %63 = pbs<Protect, Lut1("ReduceCarry2")>(%62);
                // Kogge chunk [0..7)               | %64 = pack_ct<4>(%63, %39);
                // Kogge chunk [0..7)               | %65 = pbs<Protect, Lut1("GenPropAdd")>(%64);
                // Kogge chunk [0..7)               | %66 = pack_ct<2>(%38, %36);
                // Kogge chunk [0..7)               | %68 = pack_ct<2>(%66, %59);
                // Kogge chunk [0..7)               | %69 = pbs<Protect, Lut1("ReduceCarry3")>(%68);
                // Kogge chunk [0..7)               | %70 = pack_ct<4>(%69, %41);
                // Kogge chunk [0..7)               | %71 = pbs<Protect, Lut1("GenPropAdd")>(%70);
                // Kogge chunk [0..7)               | %72 = pack_ct<4>(%40, %66);
                // Kogge chunk [0..7)               | %74 = pack_ct<2>(%72, %59);
                // Kogge chunk [0..7)               | %75 = pbs<AllowBothPadding, Lut1("ReduceCarryPad")>(%74);
                // Kogge chunk [0..7)               | %77 = wrapping_add_pt(%75, %58);
                // Kogge chunk [0..7)               | %78 = pack_ct<4>(%77, %43);
                // Kogge chunk [0..7)               | %79 = pbs<Protect, Lut1("GenPropAdd")>(%78);
                // Kogge chunk [0..7)               | %80 = pack_ct<2>(%42, %40);
                // Kogge chunk [0..7)               | %82 = pack_ct<4>(%80, %66);
                // Kogge chunk [0..7)               | %83 = pbs<AllowBothPadding, Lut1("ReduceCarryPad")>(%82);
                // Kogge chunk [0..7)               | %85 = wrapping_add_pt(%83, %58);
                // Kogge chunk [0..7)               | %86 = pack_ct<2>(%85, %59);
                // Kogge chunk [0..7)               | %87 = pbs<Protect, Lut1("ReduceCarry2")>(%86);
                // Kogge chunk [7..9) / GenProp 0   | %88, %89 = pbs2<Protect, Lut2("ManyGenProp")>(%27);
                // Kogge chunk [7..9) / GenProp 1   | %90, %91 = pbs2<Protect, Lut2("ManyGenProp")>(%28);
                // Kogge chunk [7..9)               | %92 = pack_ct<4>(%87, %89);
                // Kogge chunk [7..9)               | %93 = pbs<Protect, Lut1("GenPropAdd")>(%92);
                // Kogge chunk [7..9)               | %94 = pack_ct<4>(%88, %86);
                // Kogge chunk [7..9)               | %95 = pbs<Protect, Lut1("ReduceCarry3")>(%94);
                // Kogge chunk [7..9)               | %96 = pack_ct<4>(%95, %91);
                // Kogge chunk [7..9)               | %97 = pbs<Protect, Lut1("GenPropAdd")>(%96);
                // Join Output                      | %102 = decl_ct<18>();
                // Join Output                      | %113 = store_ct_block<0>(%45, %102);
                // Join Output                      | %114 = store_ct_block<1>(%49, %113);
                // Join Output                      | %115 = store_ct_block<2>(%53, %114);
                // Join Output                      | %116 = store_ct_block<3>(%61, %115);
                // Join Output                      | %117 = store_ct_block<4>(%65, %116);
                // Join Output                      | %118 = store_ct_block<5>(%71, %117);
                // Join Output                      | %119 = store_ct_block<6>(%79, %118);
                // Join Output                      | %120 = store_ct_block<7>(%93, %119);
                // Join Output                      | %121 = store_ct_block<8>(%97, %120);
                                                    | output<0>(%121);
            "#
        );
    }

    #[test]
    fn correctness_add_hillis_steele() {
        for size in (2..128).step_by(2) {
            add_hillis_steele(CiphertextSpec::new(size, 2, 2)).test_random(100, add_semantic);
        }
    }

    #[test]
    fn correctness_add_ripple() {
        for size in (2..128).step_by(2) {
            add_ripple_carry(CiphertextSpec::new(size, 2, 2)).test_random(100, add_semantic);
        }
    }

    #[test]
    fn correctness_add() {
        for size in (2..128).step_by(2) {
            add(CiphertextSpec::new(size, 2, 2)).test_random(100, add_semantic);
        }
    }

    #[test]
    fn correctness_add_simd() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            inp.chunks(2)
                .flat_map(|chunk| {
                    let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = chunk else {
                        unreachable!()
                    };
                    vec![IopValue::Ciphertext(lhs.add(*rhs))]
                })
                .collect::<Vec<_>>()
                .into()
        }
        for size in (2..128).step_by(2) {
            add_simd(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_add_kogge_stone() {
        for size in (2..128).step_by(2) {
            add_kogge_stone(CiphertextSpec::new(size, 2, 2), 12).test_random(100, add_semantic);
        }
    }

    #[test]
    fn correctness_add_kogge_stone_par_w() {
        for par_w in [1, 2, 4, 8, 10, 12] {
            let spec = CiphertextSpec::new(32, 2, 2);
            let builder = Builder::new(spec.block_spec());
            let a = builder.ciphertext_input(spec.int_size());
            let b = builder.ciphertext_input(spec.int_size());
            let a_blocks = builder.ciphertext_split(&a);
            let b_blocks = builder.ciphertext_split(&b);
            let (res, _carry_out) =
                builder.iop_add_kogge_stone_raw(a_blocks, b_blocks, None, par_w);
            let out = builder.ciphertext_join(res, None);
            builder.ciphertext_output(out);
            builder.test_random(100, add_semantic);
        }
    }

    #[test]
    fn correctness_overflow_add() {
        for size in (2..128).step_by(2) {
            overflow_add(CiphertextSpec::new(size, 2, 2)).test_random(100, overflow_add_semantic);
        }
    }

    #[test]
    fn correctness_add_beaumont_smith() {
        // Cover non-power-of-4 block counts and level boundaries (block counts
        // straddling 4, 16, 64) so the radix-4 prefix termination is exercised.
        for size in (2..128).step_by(2) {
            add_beaumont_smith(CiphertextSpec::new(size, 2, 2)).test_random(100, add_semantic);
        }
    }

    #[test]
    fn correctness_beaumont_smith_carry() {
        // Validates both the sum and the PG-derived carry-out flag.
        for size in (2..128).step_by(2) {
            let spec = CiphertextSpec::new(size, 2, 2);
            let builder = Builder::new(spec.block_spec());
            let a = builder.ciphertext_input(spec.int_size());
            let b = builder.ciphertext_input(spec.int_size());
            let (sum, carry) = builder.iop_add_beaumont_smith(&a, &b, None);
            builder.ciphertext_output(sum);
            builder.ciphertext_output(carry);
            builder.test_random(50, overflow_add_semantic);
        }
    }

    #[test]
    fn correctness_add_beaumont_smith_mh() {
        // Partitioning is placement only, so the sum must be correct for any board
        // count. Covers even and uneven segment splits (e.g. 32 blocks / 3 boards).
        // (int_size_bits, n_boards); blocks = bits / 2.
        for (size, boards) in [
            (16u16, 2usize),
            (24, 3),
            (32, 4),
            (64, 3),
            (64, 4),
            (128, 7),
            (8, 4),
        ] {
            add_beaumont_smith_mh(CiphertextSpec::new(size, 2, 2), boards)
                .test_random(50, add_semantic);
        }
    }

    #[test]
    fn correctness_add_ccs() {
        use zhc_crypto::integer_semantics::EmulatedCiphertext;
        // Emit an explicit carry-out flag alongside the sum so the single-board CCS carry-out is checked:
        // it is easy to return the carry *into* the top block instead of out of it, and return a dummy 0 when n == 1
        let build = |spec: CiphertextSpec| {
            let builder = Builder::new(spec.block_spec());
            let a = builder.ciphertext_input(spec.int_size());
            let b = builder.ciphertext_input(spec.int_size());
            let (a_b, b_b) = (builder.ciphertext_split(&a), builder.ciphertext_split(&b));
            let (res, cout) = builder.iop_add_ccs_raw(a_b, b_b, None);
            let flag = builder.resolve_carry(cout);
            builder.ciphertext_output(builder.ciphertext_join(res, Some(spec.int_size())));
            builder.ciphertext_output(builder.ciphertext_join([flag], None));
            builder
        };
        for size in (2..128).step_by(2) {
            build(CiphertextSpec::new(size, 2, 2)).test_random(50, overflow_add_semantic);
        }

        // Deterministic edge case: 4-bit 12 + 4 = 16 wraps to 0 with carry-out 1.
        // Before the C1 fix this returned carry-out 0.
        let spec = CiphertextSpec::new(4, 2, 2);
        let inputs = vec![
            IopValue::Ciphertext(EmulatedCiphertext::new(12, spec)),
            IopValue::Ciphertext(EmulatedCiphertext::new(4, spec)),
        ];
        let out = build(spec).interpret().with_inputs(&inputs).get_outputs();
        let (IopValue::Ciphertext(sum), IopValue::Ciphertext(flag)) = (&out[0], &out[1]) else {
            unreachable!()
        };
        assert_eq!(sum.as_storage() & 0xF, 0, "4b 12+4 sum should wrap to 0");
        assert_eq!(flag.as_storage(), 1, "4b 12+4 carry-out must be 1");
    }

    #[test]
    fn correctness_add_bs_grouped() {
        // Sweep widths incl. non-multiple-of-4 and non-power-of-2 group counts (DCE pads).
        for size in (2..128).step_by(2) {
            add_bs_grouped(CiphertextSpec::new(size, 2, 2)).test_random(50, add_semantic);
        }
    }

    #[test]
    fn correctness_add_bs_grouped_v2() {
        // Sweep widths incl. non-multiple-of-4 and non-power-of-2 group counts.
        for size in (2..128).step_by(2) {
            add_bs_grouped_v2(CiphertextSpec::new(size, 2, 2)).test_random(50, add_semantic);
        }
    }

    #[test]
    fn correctness_beaumont_smith_mh_carry() {
        // Validates the multi-board sum and carry-out together.
        for (size, boards) in [(16u16, 2usize), (64, 3), (64, 4)] {
            let spec = CiphertextSpec::new(size, 2, 2);
            let builder = Builder::new(spec.block_spec());
            let a = builder.ciphertext_input(spec.int_size());
            let b = builder.ciphertext_input(spec.int_size());
            let a_blocks = builder.ciphertext_split(&a);
            let b_blocks = builder.ciphertext_split(&b);
            let (sum, carry) =
                builder.iop_add_beaumont_smith_mh_raw(&a_blocks, &b_blocks, None, boards);
            // carry is PG-encoded; resolve_carry applies the same IsSome clean.
            let flag = builder.resolve_carry(carry);
            builder.ciphertext_output(builder.ciphertext_join(sum, Some(spec.int_size())));
            builder.ciphertext_output(builder.ciphertext_join([flag], None));
            builder.test_random(50, overflow_add_semantic);
        }
    }
}
