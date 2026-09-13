use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

use crate::{
    CiphertextBlock, NU, NU_BOOL,
    builder::{Builder, Ciphertext, Plaintext},
};

/// The kind of shift or rotate operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftRotKind {
    /// Logical right shift — zeros fill vacated MSB positions.
    ShiftRight,
    /// Logical left shift — zeros fill vacated LSB positions.
    ShiftLeft,
    /// Right rotation — bits shifted out of LSB re-enter at MSB.
    RotateRight,
    /// Left rotation — bits shifted out of MSB re-enter at LSB.
    RotateLeft,
}

impl ShiftRotKind {
    /// Whether vacated positions are zero-filled instead of wrapping around, i.e. whether an
    /// amount of `int_size` or more must produce a null result.
    fn is_shift(&self) -> bool {
        matches!(self, ShiftRotKind::ShiftRight | ShiftRotKind::ShiftLeft)
    }
}

/// The block that block `i` receives when the datapath moves data by `stride` blocks.
///
/// `None` means nothing shifts in, i.e. the position must be zero-filled. Shared by the merge
/// stage (`stride == 1`) and by every butterfly stage of both flavors.
fn shiftrot_swap_source<'b>(
    kind: ShiftRotKind,
    blocks: &'b [CiphertextBlock],
    i: usize,
    stride: usize,
) -> Option<&'b CiphertextBlock> {
    let blk_w = blocks.len();
    match kind {
        ShiftRotKind::ShiftRight => blocks.get(i + stride),
        ShiftRotKind::ShiftLeft => i.checked_sub(stride).and_then(|j| blocks.get(j)),
        ShiftRotKind::RotateRight => Some(&blocks[(i + stride) % blk_w]),
        ShiftRotKind::RotateLeft => Some(&blocks[(i + blk_w - stride % blk_w) % blk_w]),
    }
}

/// Which bit position in the carry field to test as the swap condition.
#[derive(Debug, Clone, Copy)]
enum CondPos {
    /// Bit 0 of the carry (LSB).
    Pos0,
    /// Bit 1 of the carry.
    Pos1,
}

/// Creates an IR for logical right shift of an encrypted integer.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrot`] with
/// [`ShiftRotKind::ShiftRight`], followed by overshift detection. When
/// `amount >= int_size`, the result is zeroed.
pub fn shift_right(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let amount = builder.ciphertext_input(spec.int_size());
    let shifted = builder.iop_shiftrot(&src, &amount, ShiftRotKind::ShiftRight);
    let res = builder.iop_overshift_zero(&shifted, &amount);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for logical left shift of an encrypted integer.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrot`] with
/// [`ShiftRotKind::ShiftLeft`], followed by overshift detection. When
/// `amount >= int_size`, the result is zeroed.
pub fn shift_left(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let amount = builder.ciphertext_input(spec.int_size());
    let shifted = builder.iop_shiftrot(&src, &amount, ShiftRotKind::ShiftLeft);
    let res = builder.iop_overshift_zero(&shifted, &amount);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for right rotation of an encrypted integer.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrot`] with
/// [`ShiftRotKind::RotateRight`]. See that method for algorithm details.
pub fn rotate_right(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let amount = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_shiftrot(&src, &amount, ShiftRotKind::RotateRight);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for left rotation of an encrypted integer.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrot`] with [`ShiftRotKind::RotateLeft`]. See
/// that method for algorithm details.
pub fn rotate_left(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let amount = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_shiftrot(&src, &amount, ShiftRotKind::RotateLeft);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for the logical right shift of an encrypted integer by a scalar.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrots`] with [`ShiftRotKind::ShiftRight`].
/// The immediate is the amount itself. When `amount >= int_size`, the result is zeroed.
pub fn shifts_right(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::ShiftRight)
}

/// Creates an IR for the logical left shift of an encrypted integer by a scalar.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrots`] with [`ShiftRotKind::ShiftLeft`].
/// The immediate is the amount itself. When `amount >= int_size`, the result is zeroed.
pub fn shifts_left(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::ShiftLeft)
}

/// Creates an IR for the right rotation of an encrypted integer by a scalar.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrots`] with [`ShiftRotKind::RotateRight`].
/// The immediate is the amount itself.
pub fn rots_right(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::RotateRight)
}

/// Creates an IR for the left rotation of an encrypted integer by a scalar.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrots`] with [`ShiftRotKind::RotateLeft`].
/// The immediate is the amount itself.
pub fn rots_left(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::RotateLeft)
}

/// Creates an IR for a scalar shift or rotation, of the given [`ShiftRotKind`].
pub fn shiftrots(spec: CiphertextSpec, kind: ShiftRotKind) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let amount = builder.plaintext_input(spec.int_size());
    let res = builder.iop_shiftrots(&src, &amount, kind);
    builder.ciphertext_output(res);
    builder
}

impl Builder {
    /// Shifts or rotates an encrypted integer by a scalar amount.
    ///
    /// The immediate is the amount itself, like in the other scalar iops. The IOp language has no
    /// arithmetic in the plaintext domain, so the amount is first lifted into the ciphertext
    /// domain with [`iop_trivial_encrypt`](Self::iop_trivial_encrypt), then the usual
    /// [`iop_shiftrot`](Self::iop_shiftrot) does the work. The lift is one linear DOp per digit
    /// and no PBS, so `SHIFTS` costs what `SHIFT` costs.
    ///
    /// A shift by `int_size` or more returns zero, which
    /// [`iop_overshift_zero`](Self::iop_overshift_zero) takes care of. A rotation does not need
    /// it, its amount being taken modulo `int_size`.
    ///
    /// # Panics
    ///
    /// Panics if the operands do not share the same integer width and block count.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder, ShiftRotKind};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let val = builder.ciphertext_input(spec.int_size());
    /// # let amt = builder.plaintext_input(spec.int_size());
    /// let shifted = builder.iop_shiftrots(&val, &amt, ShiftRotKind::ShiftLeft);
    /// ```
    pub fn iop_shiftrots(
        &self,
        src: &Ciphertext,
        amount: &Plaintext,
        kind: ShiftRotKind,
    ) -> Ciphertext {
        assert_eq!(
            src.spec().int_size(),
            amount.spec().int_size(),
            "Spec mismatch."
        );
        assert_eq!(
            src.spec().block_count(),
            amount.spec().block_count(),
            "Spec mismatch."
        );
        let amount_ct = self.comment("Lift Amount").iop_trivial_encrypt(amount);
        let shifted = self.iop_shiftrot(src, &amount_ct, kind);
        if kind.is_shift() {
            self.iop_overshift_zero(&shifted, &amount_ct)
        } else {
            shifted
        }
    }

    /// Shifts or rotates an encrypted integer by an encrypted amount.
    ///
    /// Implements a barrel shifter with three stages:
    /// 1. **Inner shift** — handles bit 0 of the amount (intra-block shift by 0 or 1 position
    ///    within each block's message bits).
    /// 2. **Merge** — combines each block's shifted message with the overflow from the neighboring
    ///    block (direction depends on shift kind).
    /// 3. **Block swap** — log₂ butterfly stages that conditionally swap whole blocks based on
    ///    higher bits of the amount.
    ///
    /// The effective shift amount is `amount mod int_size` (for power-of-two
    /// integer sizes).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder, ShiftRotKind};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let val = builder.ciphertext_input(spec.int_size());
    /// # let amt = builder.ciphertext_input(spec.int_size());
    /// let shifted = builder.iop_shiftrot(&val, &amt, ShiftRotKind::ShiftLeft);
    /// ```
    pub fn iop_shiftrot(
        &self,
        src: &Ciphertext,
        amount: &Ciphertext,
        kind: ShiftRotKind,
    ) -> Ciphertext {
        let src_blocks = self.ciphertext_split(src);
        let amount_blocks = self.ciphertext_split(amount);
        let blk_w = src_blocks.len();
        let msg_w = self.spec().message_size() as usize;

        // Stage 1: Inner shift — process bit 0 of amount.
        self.push_comment("Inner shift");
        let (shiftrot_msg, shiftrot_next): (Vec<_>, Vec<_>) = src_blocks
            .iter()
            .enumerate()
            .map(|(i, block)| {
                self.push_comment(format!("Block {i}"));
                let r = self.shiftrot_inner(kind, block, &amount_blocks[0]);
                self.pop_comment();
                r
            })
            .unzip();
        self.pop_comment();

        // Stage 2: Fuse msg and msg_next from neighboring blocks.
        self.push_comment("Merge");
        let mut merged: Vec<CiphertextBlock> = (0..blk_w)
            .map(|i| {
                // The overflow of a block always lands one position away, in the shift direction.
                let neighbor = shiftrot_swap_source(kind, &shiftrot_next, i, 1);
                match neighbor {
                    Some(n) => self.block_add(&shiftrot_msg[i], n),
                    None => shiftrot_msg[i],
                }
            })
            .collect();
        self.pop_comment();

        // Stage 3: Block swap — butterfly stages for higher amount bits.
        // Each stage handles one bit of the shift amount. Stage `stg` tests
        // bit `stg` of the amount (spread across amount blocks, 2 bits per
        // block for (2,2) params).
        let num_stages = (2 * blk_w).ilog2() as usize;
        for stg in 1..num_stages {
            self.push_comment(format!("Swap stage {stg}"));
            let stride = 1usize << (stg - 1);
            let cond_block = &amount_blocks[stg / msg_w];
            let cond_pos = if stg % 2 == 1 {
                CondPos::Pos1
            } else {
                CondPos::Pos0
            };

            let prev = merged.clone();
            merged = (0..blk_w)
                .map(|i| {
                    let swap = shiftrot_swap_source(kind, &prev, i, stride);
                    self.shiftrot_block_swap(&prev[i], swap, cond_block, cond_pos)
                })
                .collect();
            self.pop_comment();
        }

        if matches!(kind, ShiftRotKind::RotateLeft | ShiftRotKind::RotateRight) {
            merged = merged
                .into_iter()
                .map(|b| self.block_lookup(&b, Lut1Def::MsgOnly))
                .collect();
        }

        self.comment("Join").ciphertext_join(merged, None)
    }

    /// Computes the intra-block shift for a single block.
    ///
    /// Packs the block value (message) with the LSB amount block (carry) and
    /// applies the appropriate shift LUTs. Returns `(msg, msg_next)` where
    /// `msg` is the portion that stays in this block and `msg_next` is the
    /// overflow that contributes to the neighboring block.
    fn shiftrot_inner(
        &self,
        kind: ShiftRotKind,
        src: &CiphertextBlock,
        amount_lsb: &CiphertextBlock,
    ) -> (CiphertextBlock, CiphertextBlock) {
        let (lut_msg, lut_next) = match kind {
            ShiftRotKind::ShiftRight | ShiftRotKind::RotateRight => (
                Lut1Def::ShiftRightByCarryPos0Msg,
                Lut1Def::ShiftRightByCarryPos0MsgNext,
            ),
            ShiftRotKind::ShiftLeft | ShiftRotKind::RotateLeft => (
                Lut1Def::ShiftLeftByCarryPos0Msg,
                Lut1Def::ShiftLeftByCarryPos0MsgNext,
            ),
        };

        // Pack: amount_lsb in carry (high), src in message (low).
        let packed = self.block_pack(amount_lsb, src);
        let msg = self.block_lookup(&packed, lut_msg);
        let msg_next = self.block_lookup(&packed, lut_next);
        (msg, msg_next)
    }

    /// Conditionally selects between the original block and a swap block based
    /// on a condition bit.
    ///
    /// When the tested bit of `cond` is 0, returns `src_orig`. When 1, returns
    /// `src_swap` (or zero if `src_swap` is `None`).
    fn shiftrot_block_swap(
        &self,
        src_orig: &CiphertextBlock,
        src_swap: Option<&CiphertextBlock>,
        cond: &CiphertextBlock,
        cond_pos: CondPos,
    ) -> CiphertextBlock {
        let (lut_true_zeroed, lut_false_zeroed) = match cond_pos {
            CondPos::Pos0 => (Lut1Def::IfPos0TrueZeroed, Lut1Def::IfPos0FalseZeroed),
            CondPos::Pos1 => (Lut1Def::IfPos1TrueZeroed, Lut1Def::IfPos1FalseZeroed),
        };

        // Pack: cond in carry (high), value in message (low).
        let pack_orig = self.block_pack(cond, src_orig);
        if let Some(swap) = src_swap {
            let pack_swap = self.block_pack(cond, swap);
            // TrueZeroed: if cond bit = 1 → zero (suppress orig)
            // FalseZeroed: if cond bit = 0 → zero (suppress swap)
            // Sum gives: cond=0 → orig+0, cond=1 → 0+swap
            let orig_part = self.block_lookup(&pack_orig, lut_true_zeroed);
            let swap_part = self.block_lookup(&pack_swap, lut_false_zeroed);
            self.block_add(&orig_part, &swap_part)
        } else {
            // No swap source — zero the block when condition is true.
            self.block_lookup(&pack_orig, lut_true_zeroed)
        }
    }

    /// Zeros `shifted` when `amount >= int_size` (unsigned overshift).
    ///
    /// The barrel shifter from iop_shiftrot consumes `num_stages = log₂(int_size)` bits of
    /// `amount`.  Any bit set above that range means `amount >= int_size`,
    /// so the result must be zero.
    ///
    /// Instead of a full integer comparison, only the blocks *not* consumed
    /// by the barrel shifter ("high blocks") are tested for non-zero.  Raw
    /// high blocks are summed in groups of NU with block_add
    /// operations before a single IsSome PBS per group, reducing the PBS
    /// count from one-per-block to ⌈n/NU⌉.  When `num_stages` is not a
    /// multiple of `msg_w`, the topmost consumed block has an unused high
    /// bit; that bit is extracted with IfPos1FalseZeroed (valid for
    /// `msg_w = 2`).
    ///
    /// The resulting boolean signals (each 0 or 1) are reduced by summing
    /// them with block_add operations (no PBS needed since the sum
    /// stays within the carry budget), then a single IsSome PBS checks
    /// whether the sum is non-zero.  This is repeated in chunks of
    /// size NU_BOOL.
    pub fn iop_overshift_zero(&self, shifted: &Ciphertext, amount: &Ciphertext) -> Ciphertext {
        let amount_blocks = self.ciphertext_split(amount);
        let msg_w = self.spec().message_size() as usize;
        let blk_w = amount_blocks.len();
        let num_stages = (2 * blk_w).ilog2() as usize; // = log₂(int_size)
        let num_low_blocks = num_stages.div_ceil(msg_w);

        self.push_comment("Overshift detection");

        let mut nz_signals: Vec<CiphertextBlock> = Vec::new();

        // Mixed block: the topmost low block may have unused high bit(s)
        // that signal overshift.  For msg_w = 2, only bit 1 (Pos1) is
        // unused when num_stages is odd.  Extract it via pack + IfPos1FalseZeroed.
        if num_stages % msg_w != 0 {
            let mixed = &amount_blocks[num_low_blocks - 1];
            let one = self.block_let_ciphertext(1);
            let packed = self.block_pack(mixed, &one);
            nz_signals.push(self.block_lookup(&packed, Lut1Def::IfPos1FalseZeroed));
        }

        // High blocks: sum raw blocks in groups of NU,
        // then one IsSome PBS per group.
        let high_blocks = &amount_blocks[num_low_blocks..];
        for chunk in high_blocks.chunks(NU) {
            let sum = chunk[1..]
                .iter()
                .fold(chunk[0], |acc, b| self.block_add(&acc, b));
            self.push_comment("IsSome on chunk");
            nz_signals.push(self.block_lookup(&sum, Lut1Def::IsSome));
            self.pop_comment();
        }

        self.pop_comment();

        if nz_signals.is_empty() {
            // All amount bits are consumed by the barrel shifter —
            // no overshift is possible.
            return *shifted;
        }

        // Reduce: sum boolean signals in chunks of NU_BOOL,
        // then apply one IsSome PBS per chunk.  Repeat until a
        // single boolean block remains.
        self.push_comment("Overshift reduce");
        while nz_signals.len() > 1 {
            nz_signals = nz_signals
                .chunks(NU_BOOL)
                .map(|chunk| {
                    let sum = chunk[1..]
                        .iter()
                        .fold(chunk[0], |acc, b| self.block_add(&acc, b));
                    if chunk.len() > 1 {
                        self.block_lookup(&sum, Lut1Def::IsSome)
                    } else {
                        // Single element — already a boolean, no PBS needed.
                        sum
                    }
                })
                .collect();
        }
        self.pop_comment();

        self.push_comment("return 0 if overshift");
        // nz_signals[0] is 1 if overshift, 0 otherwise.
        // IfTrueZeroed: if cond != 0 → 0; if cond == 0 → value.
        let shifted_blocks = self.ciphertext_split(shifted);
        let output_blocks: Vec<CiphertextBlock> = shifted_blocks
            .iter()
            .map(|b| {
                let packed = self.block_pack(&nz_signals[0], b);
                self.block_lookup(&packed, Lut1Def::IfTrueZeroed)
            })
            .collect();
        self.pop_comment();
        self.ciphertext_join(output_blocks, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    // Expected result of a scalar shift or rotate. `amount` is a clear value that may be larger
    // than `int_size`: a shift then gives zero, a rotation wraps around.
    fn scalar_reference(kind: ShiftRotKind, int_size: u16, value: u128, amount: u32) -> u128 {
        let w = u32::from(int_size);
        let mask = u128::MAX >> (128 - w);
        let n = amount % w;
        match kind {
            ShiftRotKind::ShiftRight if amount >= w => 0,
            ShiftRotKind::ShiftLeft if amount >= w => 0,
            ShiftRotKind::ShiftRight => value >> amount,
            ShiftRotKind::ShiftLeft => (value << amount) & mask,
            ShiftRotKind::RotateRight if n == 0 => value,
            ShiftRotKind::RotateLeft if n == 0 => value,
            ShiftRotKind::RotateRight => ((value >> n) | (value << (w - n))) & mask,
            ShiftRotKind::RotateLeft => ((value << n) | (value >> (w - n))) & mask,
        }
    }

    // Sweeps every amount in `0..2 * int_size`, so the normal case and the overshift one are both
    // covered. `test_random` draws amounts on the full width, which almost always overshift.
    fn exercise_scalar(kind: ShiftRotKind, int_size: u16, reps: usize) {
        let spec = CiphertextSpec::new(int_size, 2, 2);
        let builder = Builder::new(spec.block_spec());
        let src = builder.ciphertext_input(spec.int_size());
        let amount_pt = builder.plaintext_input(spec.int_size());
        let res = builder.iop_shiftrots(&src, &amount_pt, kind);
        builder.ciphertext_output(res);

        for amount in 0..(2 * u32::from(int_size)) {
            let amount_value = amount_pt.make_value(u128::from(amount));
            for _ in 0..reps {
                let value = spec.random();
                let outputs = builder
                    .interpret()
                    .with_inputs([IopValue::Ciphertext(value), amount_value.clone()])
                    .get_outputs();
                let expected = scalar_reference(kind, int_size, value.as_storage(), amount);
                assert_eq!(
                    outputs,
                    vec![IopValue::Ciphertext(spec.from_int(expected))],
                    "{kind:?} of {:#x} by {amount} on {int_size} bits",
                    value.as_storage()
                );
            }
        }
    }

    // The immediate is the plain amount: it is lifted with `add_pt` on a null block, then the
    // stream is the one of the encrypted flavor.
    #[test]
    fn test_shifts_left() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let ir = shifts_left(spec).optimize_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true),
            r#"
                                                           | %0 = input_ciphertext<0, 8>();
                                                           | %1 = input_plaintext<1, 8>();
                // Lift Amount                             | %2 = extract_pt_block<0>(%1);
                // Lift Amount                             | %3 = extract_pt_block<1>(%1);
                // Lift Amount                             | %4 = extract_pt_block<2>(%1);
                // Lift Amount                             | %5 = extract_pt_block<3>(%1);
                // Lift Amount                             | %6 = let_ct_block<0>();
                // Lift Amount                             | %7 = add_pt(%6, %2);
                // Lift Amount                             | %8 = add_pt(%6, %3);
                // Lift Amount                             | %9 = add_pt(%6, %4);
                // Lift Amount                             | %10 = add_pt(%6, %5);
                                                           | %21 = extract_ct_block<0>(%0);
                                                           | %22 = extract_ct_block<1>(%0);
                                                           | %23 = extract_ct_block<2>(%0);
                                                           | %24 = extract_ct_block<3>(%0);
                // Inner shift / Block 0                   | %29 = pack_ct<4>(%7, %21);
                // Inner shift / Block 0                   | %30 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%29);
                // Inner shift / Block 0                   | %31 = pbs<Protect, Lut1("ShiftLeftByCarryPos0MsgNext")>(%29);
                // Inner shift / Block 1                   | %32 = pack_ct<4>(%7, %22);
                // Inner shift / Block 1                   | %33 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%32);
                // Inner shift / Block 1                   | %34 = pbs<Protect, Lut1("ShiftLeftByCarryPos0MsgNext")>(%32);
                // Inner shift / Block 2                   | %35 = pack_ct<4>(%7, %23);
                // Inner shift / Block 2                   | %36 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%35);
                // Inner shift / Block 2                   | %37 = pbs<Protect, Lut1("ShiftLeftByCarryPos0MsgNext")>(%35);
                // Inner shift / Block 3                   | %38 = pack_ct<4>(%7, %24);
                // Inner shift / Block 3                   | %39 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%38);
                // Merge                                   | %41 = add_ct(%33, %31);
                // Merge                                   | %42 = add_ct(%36, %34);
                // Merge                                   | %43 = add_ct(%39, %37);
                // Swap stage 1                            | %44 = pack_ct<4>(%7, %30);
                // Swap stage 1                            | %45 = pbs<Protect, Lut1("IfPos1TrueZeroed")>(%44);
                // Swap stage 1                            | %46 = pack_ct<4>(%7, %41);
                // Swap stage 1                            | %48 = pbs<Protect, Lut1("IfPos1TrueZeroed")>(%46);
                // Swap stage 1                            | %49 = pbs<Protect, Lut1("IfPos1FalseZeroed")>(%44);
                // Swap stage 1                            | %50 = add_ct(%48, %49);
                // Swap stage 1                            | %51 = pack_ct<4>(%7, %42);
                // Swap stage 1                            | %53 = pbs<Protect, Lut1("IfPos1TrueZeroed")>(%51);
                // Swap stage 1                            | %54 = pbs<Protect, Lut1("IfPos1FalseZeroed")>(%46);
                // Swap stage 1                            | %55 = add_ct(%53, %54);
                // Swap stage 1                            | %56 = pack_ct<4>(%7, %43);
                // Swap stage 1                            | %58 = pbs<Protect, Lut1("IfPos1TrueZeroed")>(%56);
                // Swap stage 1                            | %59 = pbs<Protect, Lut1("IfPos1FalseZeroed")>(%51);
                // Swap stage 1                            | %60 = add_ct(%58, %59);
                // Swap stage 2                            | %61 = pack_ct<4>(%8, %45);
                // Swap stage 2                            | %62 = pbs<Protect, Lut1("IfPos0TrueZeroed")>(%61);
                // Swap stage 2                            | %63 = pack_ct<4>(%8, %50);
                // Swap stage 2                            | %64 = pbs<Protect, Lut1("IfPos0TrueZeroed")>(%63);
                // Swap stage 2                            | %65 = pack_ct<4>(%8, %55);
                // Swap stage 2                            | %67 = pbs<Protect, Lut1("IfPos0TrueZeroed")>(%65);
                // Swap stage 2                            | %68 = pbs<Protect, Lut1("IfPos0FalseZeroed")>(%61);
                // Swap stage 2                            | %69 = add_ct(%67, %68);
                // Swap stage 2                            | %70 = pack_ct<4>(%8, %60);
                // Swap stage 2                            | %72 = pbs<Protect, Lut1("IfPos0TrueZeroed")>(%70);
                // Swap stage 2                            | %73 = pbs<Protect, Lut1("IfPos0FalseZeroed")>(%63);
                // Swap stage 2                            | %74 = add_ct(%72, %73);
                // Overshift detection                     | %89 = let_ct_block<1>();
                // Overshift detection                     | %90 = pack_ct<4>(%8, %89);
                // Overshift detection                     | %91 = pbs<Protect, Lut1("IfPos1FalseZeroed")>(%90);
                // Overshift detection                     | %92 = add_ct(%9, %10);
                // Overshift detection / IsSome on chunk   | %93 = pbs<Protect, Lut1("IsSome")>(%92);
                // Overshift reduce                        | %94 = add_ct(%91, %93);
                // Overshift reduce                        | %95 = pbs<Protect, Lut1("IsSome")>(%94);
                // return 0 if overshift                   | %100 = pack_ct<4>(%95, %62);
                // return 0 if overshift                   | %101 = pbs<Protect, Lut1("IfTrueZeroed")>(%100);
                // return 0 if overshift                   | %102 = pack_ct<4>(%95, %64);
                // return 0 if overshift                   | %103 = pbs<Protect, Lut1("IfTrueZeroed")>(%102);
                // return 0 if overshift                   | %104 = pack_ct<4>(%95, %69);
                // return 0 if overshift                   | %105 = pbs<Protect, Lut1("IfTrueZeroed")>(%104);
                // return 0 if overshift                   | %106 = pack_ct<4>(%95, %74);
                // return 0 if overshift                   | %107 = pbs<Protect, Lut1("IfTrueZeroed")>(%106);
                                                           | %108 = decl_ct<8>();
                                                           | %114 = store_ct_block<0>(%101, %108);
                                                           | %115 = store_ct_block<1>(%103, %114);
                                                           | %116 = store_ct_block<2>(%105, %115);
                                                           | %117 = store_ct_block<3>(%107, %116);
                                                           | output<0>(%117);
            "#
        );
    }

    #[test]
    fn correctness_shifts_right() {
        for size in [4, 8, 16, 32, 64] {
            exercise_scalar(ShiftRotKind::ShiftRight, size, 4);
        }
    }

    #[test]
    fn correctness_shifts_left() {
        for size in [4, 8, 16, 32, 64] {
            exercise_scalar(ShiftRotKind::ShiftLeft, size, 4);
        }
    }

    #[test]
    fn correctness_rots_right() {
        for size in [4, 8, 16, 32, 64] {
            exercise_scalar(ShiftRotKind::RotateRight, size, 4);
        }
    }

    #[test]
    fn correctness_rots_left() {
        for size in [4, 8, 16, 32, 64] {
            exercise_scalar(ShiftRotKind::RotateLeft, size, 4);
        }
    }

    #[test]
    fn correctness_shifts_right_random() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Plaintext(amount)] = inp else {
                unreachable!()
            };
            let amount = src.spec().from_int(amount.as_storage());
            Some(vec![IopValue::Ciphertext(src.shift_right(amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            shifts_right(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_shifts_left_random() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Plaintext(amount)] = inp else {
                unreachable!()
            };
            let amount = src.spec().from_int(amount.as_storage());
            Some(vec![IopValue::Ciphertext(src.shift_left(amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            shifts_left(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_rots_right_random() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Plaintext(amount)] = inp else {
                unreachable!()
            };
            let amount = src.spec().from_int(amount.as_storage());
            Some(vec![IopValue::Ciphertext(src.rotate_right(amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            rots_right(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_rots_left_random() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Plaintext(amount)] = inp else {
                unreachable!()
            };
            let amount = src.spec().from_int(amount.as_storage());
            Some(vec![IopValue::Ciphertext(src.rotate_left(amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            rots_left(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_shift_right() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Ciphertext(amount)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(src.shift_right(*amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            shift_right(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_shift_left() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Ciphertext(amount)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(src.shift_left(*amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            shift_left(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_rotate_right() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Ciphertext(amount)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(src.rotate_right(*amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            rotate_right(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_rotate_left() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src), IopValue::Ciphertext(amount)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(src.rotate_left(*amount))])
        }
        for size in [4, 8, 16, 32, 64] {
            rotate_left(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn noise_shifts_right() {
        for size in [4, 8, 16, 32, 64] {
            shifts_right(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_shifts_left() {
        for size in [4, 8, 16, 32, 64] {
            shifts_left(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_rots_right() {
        for size in [4, 8, 16, 32, 64] {
            rots_right(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_rots_left() {
        for size in [4, 8, 16, 32, 64] {
            rots_left(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_shift_right() {
        for size in [4, 8, 16, 32, 64] {
            shift_right(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_shift_left() {
        for size in [4, 8, 16, 32, 64] {
            shift_left(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_rotate_right() {
        for size in [4, 8, 16, 32, 64] {
            rotate_right(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_rotate_left() {
        for size in [4, 8, 16, 32, 64] {
            rotate_left(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
