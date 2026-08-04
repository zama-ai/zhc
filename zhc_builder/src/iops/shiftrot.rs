use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::SafeAs;

use crate::{
    CiphertextBlock, PlaintextBlock, NU, NU_BOOL,
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

/// Number of message bits of the control word consumed by the scalar shift/rotate iops.
///
/// See [`shiftrot_ctrl_word`] for the layout: one digit per amount bit plus the `keep` digit.
///
/// # Panics
///
/// Panics if `int_size` is not a power of two.
pub fn shiftrot_ctrl_size(spec: CiphertextSpec) -> u16 {
    assert!(
        spec.int_size().is_power_of_two(),
        "Scalar shift/rotate needs a power of two int_size."
    );
    let msg_w = spec.block_spec().message_size().sas::<u16>();
    (spec.int_size().ilog2().sas::<u16>() + 1) * msg_w
}

/// Encodes a clear shift/rotate amount into the control word expected by
/// [`Builder::iop_shiftrots`].
 ///
/// The datapath selects with plaintext multiplications, which need one *digit* per control bit —
/// and the IOp language cannot slice a digit out of a packed immediate. The amount is therefore
/// advertised bit per bit, which is free for the host since it holds it in the clear:
///
/// ```text
///  digit i         = bit i of (amount % int_size)   for i < log2(int_size)
///  digit log2(W)   = keep = 1 if amount < int_size, else 0
/// ```
///
/// `keep` is what makes an overflowing *shift* return zero at no cost; rotations ignore it, and it
/// is always set for them since `amount % int_size` is the whole story.
///
/// # Panics
///
/// Panics if `int_size` is not a power of two.
///
/// # Examples
///
/// ```rust
/// # use zhc_builder::{CiphertextSpec, ShiftRotKind, shiftrot_ctrl_word};
/// let spec = CiphertextSpec::new(64, 2, 2);
/// // 7 == 0b000111 -> digits [1, 1, 1, 0, 0, 0 | keep = 1]
/// assert_eq!(shiftrot_ctrl_word(spec, ShiftRotKind::ShiftLeft, 7), 0x1015);
/// // 70 >= 64 -> keep = 0, the result is null whatever the digits say
/// assert_eq!(shiftrot_ctrl_word(spec, ShiftRotKind::ShiftLeft, 70), 0x0014);
/// ```
pub fn shiftrot_ctrl_word(spec: CiphertextSpec, kind: ShiftRotKind, amount: u32) -> u128 {
    let msg_w = spec.block_spec().message_size().sas::<u32>();
    let log_w = spec.int_size().ilog2();
    let mut ctrl = 0_u128;
    // Only the low log2(int_size) bits matter, which is exactly `amount % int_size`.
    for i in 0..log_w {
        ctrl |= u128::from((amount >> i) & 1) << (i * msg_w);
    }
    if !kind.is_shift() || u32::from(spec.int_size()) > amount {
        ctrl |= 1_u128 << (log_w * msg_w);
    }
    ctrl
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
/// The immediate is a control word, see [`shiftrot_ctrl_word`].
pub fn shifts_right(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::ShiftRight)
}

/// Creates an IR for the logical left shift of an encrypted integer by a scalar.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrots`] with [`ShiftRotKind::ShiftLeft`].
/// The immediate is a control word, see [`shiftrot_ctrl_word`].
pub fn shifts_left(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::ShiftLeft)
}

/// Creates an IR for the right rotation of an encrypted integer by a scalar.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrots`] with [`ShiftRotKind::RotateRight`].
/// The immediate is a control word, see [`shiftrot_ctrl_word`].
pub fn rots_right(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::RotateRight)
}

/// Creates an IR for the left rotation of an encrypted integer by a scalar.
///
/// Convenience wrapper that calls [`Builder::iop_shiftrots`] with [`ShiftRotKind::RotateLeft`].
/// The immediate is a control word, see [`shiftrot_ctrl_word`].
pub fn rots_left(spec: CiphertextSpec) -> Builder {
    shiftrots(spec, ShiftRotKind::RotateLeft)
}

/// Creates an IR for a scalar shift or rotation, of the given [`ShiftRotKind`].
pub fn shiftrots(spec: CiphertextSpec, kind: ShiftRotKind) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let ctrl = builder.plaintext_input(shiftrot_ctrl_size(spec));
    let res = builder.iop_shiftrots(&src, &ctrl, kind);
    builder.ciphertext_output(res);
    builder
}

impl Builder {
    /// Shifts or rotates an encrypted integer by a scalar amount.
    ///
    /// Same barrel shifter as [`iop_shiftrot`](Self::iop_shiftrot), except that the swap
    /// conditions come from the immediate instead of a ciphertext, which makes the whole butterfly
    /// **free**: with a control digit `b` in `{0, 1}`,
    ///
    /// ```text
    ///  orig + swap * b - orig * b   ==   orig  if b == 0
    ///                                    swap  if b == 1
    /// ```
    ///
    /// and every term is a linear operation. `b == 0` turns `MulPt` into the null ciphertext while
    /// `b == 1` turns it into the identity, so the result is not merely *equal* to one of the two
    /// operands: it **is** that operand, noise included — the `orig` contribution cancels against
    /// itself. No cleanup lookup is needed between stages, and only the intra-block stage still
    /// spends PBS, i.e. `2 * block_count` in total against `2 * block_count * (1 + log2(N))` for
    /// the encrypted-amount flavor.
    ///
    /// The same trick gates the result: shifting by `int_size` or more must return zero, which is
    /// one `MulPt` per block by the `keep` digit of the control word. Rotations do not need it,
    /// their amount being taken modulo `int_size`.
    ///
    /// `ctrl` is *not* the raw amount — see [`shiftrot_ctrl_word`] for its layout and for the
    /// reason the host has to explode the amount bit per bit.
    ///
    /// # Panics
    ///
    /// Panics if `int_size` is not a power of two, or if `ctrl` is not
    /// [`shiftrot_ctrl_size`] wide.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder, ShiftRotKind, shiftrot_ctrl_size};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let val = builder.ciphertext_input(spec.int_size());
    /// # let ctrl = builder.plaintext_input(shiftrot_ctrl_size(spec));
    /// let shifted = builder.iop_shiftrots(&val, &ctrl, ShiftRotKind::ShiftLeft);
    /// ```
    pub fn iop_shiftrots(
        &self,
        src: &Ciphertext,
        ctrl: &Plaintext,
        kind: ShiftRotKind,
    ) -> Ciphertext {
        assert_eq!(
            ctrl.spec().int_size(),
            shiftrot_ctrl_size(src.spec()),
            "Spec mismatch."
        );
        let src_blocks = self.ciphertext_split(src);
        let ctrl_digits = self.plaintext_split(ctrl);
        let blk_w = src_blocks.len();
        let num_stages = (2 * blk_w).ilog2().sas::<usize>();

        // Stage 1 & 2: the intra-block shift is driven by bit 0 of the amount, which the shift
        // luts read from the carry field of a ciphertext block. Lifting that single digit is one
        // linear DOp, so the stage is shared with the encrypted-amount flavor as is.
        let amount_lsb = self
            .comment("Lift Amount Lsb")
            .block_add_plaintext(self.block_let_ciphertext(0), ctrl_digits[0]);
        let mut merged = self.shiftrot_inner_pass(kind, &src_blocks, &amount_lsb);

        // Stage 3: block swap, one butterfly stage per remaining amount bit. Every stage is
        // linear: no PBS, no noise growth, no degree growth.
        for stg in 1..num_stages {
            self.push_comment(format!("Swap stage {stg}"));
            let stride = 1_usize << (stg - 1);
            let sel = ctrl_digits[stg];

            let prev = merged.clone();
            merged = (0..blk_w)
                .map(|i| {
                    let swap = shiftrot_swap_source(kind, &prev, i, stride);
                    self.shiftrot_block_select(&prev[i], swap, &sel)
                })
                .collect();
            self.pop_comment();
        }

        // Shifting by int_size or more empties the integer: `keep` is null in that case, and
        // multiplying a block by a null immediate yields the null ciphertext.
        if kind.is_shift() {
            self.push_comment("Keep");
            let keep = ctrl_digits[num_stages];
            merged = merged
                .iter()
                .map(|block| self.block_mul_plaintext(block, keep))
                .collect();
            self.pop_comment();
        }

        self.comment("Join").ciphertext_join(merged, None)
    }

    /// Selects between `src_orig` and `src_swap` on a plaintext condition, with no PBS.
    ///
    /// `sel` must be 0 or 1, which the control word guarantees. `src_swap` being `None` means
    /// there is nothing to shift in, so the block is zeroed when the condition holds.
    fn shiftrot_block_select(
        &self,
        src_orig: &CiphertextBlock,
        src_swap: Option<&CiphertextBlock>,
        sel: &PlaintextBlock,
    ) -> CiphertextBlock {
        // sel * orig is either the null ciphertext or `orig` itself.
        let masked_orig = self.block_mul_plaintext(src_orig, sel);
        match src_swap {
            Some(swap) => {
                let masked_swap = self.block_mul_plaintext(swap, sel);
                // At most `2 * msg_mask`, and the subtraction brings it back to a single digit.
                let sum = self.block_add(src_orig, masked_swap);
                // if there is a swap block calculating src_orig + swap*sel - src_orig*sel
                // so if sel = 1 => swap else => src_orig
                self.block_sub(sum, masked_orig)
            }
            // if there is no swap block calculating src_orig - src_orig*sel
            // so if sel = 1 => 0 else => src_orig
            None => self.block_sub(src_orig, masked_orig),
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

        // Stage 1 & 2: Inner shift on bit 0 of the amount, then merge with the neighbours.
        let mut merged = self.shiftrot_inner_pass(kind, &src_blocks, &amount_blocks[0]);

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

        self.comment("Join").ciphertext_join(merged, None)
    }

    /// Runs the intra-block stage of the barrel shifter and merges each block with the overflow of
    /// its neighbour, i.e. stages 1 and 2 of [`iop_shiftrot`](Self::iop_shiftrot).
    ///
    /// `amount_lsb` carries bit 0 of the amount; the scalar flavor lifts it from its immediate.
    /// Costs two PBS per block and nothing else.
    fn shiftrot_inner_pass(
        &self,
        kind: ShiftRotKind,
        src_blocks: &[CiphertextBlock],
        amount_lsb: &CiphertextBlock,
    ) -> Vec<CiphertextBlock> {
        let blk_w = src_blocks.len();

        self.push_comment("Inner shift");
        let (shiftrot_msg, shiftrot_next): (Vec<_>, Vec<_>) = src_blocks
            .iter()
            .enumerate()
            .map(|(i, block)| {
                self.push_comment(format!("Block {i}"));
                let r = self.shiftrot_inner(kind, block, amount_lsb);
                self.pop_comment();
                r
            })
            .unzip();
        self.pop_comment();

        self.push_comment("Merge");
        let merged = (0..blk_w)
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

        merged
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

    /// Expected result of a scalar shift/rotate, `amount` being a *clear* value that may exceed
    /// `int_size` -- in which case a shift empties the integer while a rotation wraps around.
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

    /// Sweeps *every* amount in `0..2 * int_size`, so both the in-range behaviour and the
    /// overflowing one are covered. `test_random` cannot be used here: it would draw arbitrary
    /// control words, whose digits must be 0 or 1 to mean anything.
    fn exercise_scalar(kind: ShiftRotKind, int_size: u16, reps: usize) {
        let spec = CiphertextSpec::new(int_size, 2, 2);
        let builder = Builder::new(spec.block_spec());
        let src = builder.ciphertext_input(spec.int_size());
        let ctrl = builder.plaintext_input(shiftrot_ctrl_size(spec));
        let res = builder.iop_shiftrots(&src, &ctrl, kind);
        builder.ciphertext_output(res);

        for amount in 0..(2 * u32::from(int_size)) {
            let ctrl_value = ctrl.make_value(shiftrot_ctrl_word(spec, kind, amount));
            for _ in 0..reps {
                let value = spec.random();
                let outputs = builder
                    .interpret()
                    .with_inputs([IopValue::Ciphertext(value), ctrl_value.clone()])
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

    /// Every butterfly stage and the `keep` gate must be free of any `pbs`: only the two
    /// intra-block lookups per block are left.
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
                                           | %2 = extract_ct_block<0>(%0);
                                           | %3 = extract_ct_block<1>(%0);
                                           | %4 = extract_ct_block<2>(%0);
                                           | %5 = extract_ct_block<3>(%0);
                                           | %6 = extract_pt_block<0>(%1);
                                           | %7 = extract_pt_block<1>(%1);
                                           | %8 = extract_pt_block<2>(%1);
                                           | %9 = extract_pt_block<3>(%1);
                                           | %10 = let_ct_block<0>();
                // Lift Amount Lsb         | %11 = add_pt(%10, %6);
                // Inner shift / Block 0   | %12 = pack_ct<4>(%11, %2);
                // Inner shift / Block 0   | %13 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%12);
                // Inner shift / Block 0   | %14 = pbs<Protect, Lut1("ShiftLeftByCarryPos0MsgNext")>(%12);
                // Inner shift / Block 1   | %15 = pack_ct<4>(%11, %3);
                // Inner shift / Block 1   | %16 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%15);
                // Inner shift / Block 1   | %17 = pbs<Protect, Lut1("ShiftLeftByCarryPos0MsgNext")>(%15);
                // Inner shift / Block 2   | %18 = pack_ct<4>(%11, %4);
                // Inner shift / Block 2   | %19 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%18);
                // Inner shift / Block 2   | %20 = pbs<Protect, Lut1("ShiftLeftByCarryPos0MsgNext")>(%18);
                // Inner shift / Block 3   | %21 = pack_ct<4>(%11, %5);
                // Inner shift / Block 3   | %22 = pbs<Protect, Lut1("ShiftLeftByCarryPos0Msg")>(%21);
                // Merge                   | %24 = add_ct(%16, %14);
                // Merge                   | %25 = add_ct(%19, %17);
                // Merge                   | %26 = add_ct(%22, %20);
                // Swap stage 1            | %27 = mul_pt(%13, %7);
                // Swap stage 1            | %28 = sub_ct(%13, %27);
                // Swap stage 1            | %29 = mul_pt(%24, %7);
                // Swap stage 1            | %31 = add_ct(%24, %27);
                // Swap stage 1            | %32 = sub_ct(%31, %29);
                // Swap stage 1            | %33 = mul_pt(%25, %7);
                // Swap stage 1            | %35 = add_ct(%25, %29);
                // Swap stage 1            | %36 = sub_ct(%35, %33);
                // Swap stage 1            | %37 = mul_pt(%26, %7);
                // Swap stage 1            | %39 = add_ct(%26, %33);
                // Swap stage 1            | %40 = sub_ct(%39, %37);
                // Swap stage 2            | %41 = mul_pt(%28, %8);
                // Swap stage 2            | %42 = sub_ct(%28, %41);
                // Swap stage 2            | %43 = mul_pt(%32, %8);
                // Swap stage 2            | %44 = sub_ct(%32, %43);
                // Swap stage 2            | %45 = mul_pt(%36, %8);
                // Swap stage 2            | %47 = add_ct(%36, %41);
                // Swap stage 2            | %48 = sub_ct(%47, %45);
                // Swap stage 2            | %49 = mul_pt(%40, %8);
                // Swap stage 2            | %51 = add_ct(%40, %43);
                // Swap stage 2            | %52 = sub_ct(%51, %49);
                // Keep                    | %53 = mul_pt(%42, %9);
                // Keep                    | %54 = mul_pt(%44, %9);
                // Keep                    | %55 = mul_pt(%48, %9);
                // Keep                    | %56 = mul_pt(%52, %9);
                // Join                    | %57 = decl_ct<8>();
                // Join                    | %63 = store_ct_block<0>(%53, %57);
                // Join                    | %64 = store_ct_block<1>(%54, %63);
                // Join                    | %65 = store_ct_block<2>(%55, %64);
                // Join                    | %66 = store_ct_block<3>(%56, %65);
                                           | output<0>(%66);
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
}
