use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::iter::CollectInSmallVec;

use crate::{Ciphertext, builder::Builder};

/// Creates an IR for a conditional zeroing of an encrypted integer.
///
/// The returned [`Builder`] declares two ciphertext inputs — one integer
/// operand and one single-block boolean condition — and one ciphertext
/// output. When the condition is zero (false) the output equals the
/// operand; when it is non-zero (true) the output is zero. Internally
/// delegates to [`Builder::iop_if_then_zero`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition. The condition input is automatically sized to a single
/// message block.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, if_then_zero};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = if_then_zero(spec);
/// let ir = builder.into_ir();
/// ```
pub fn if_then_zero(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.input_ciphertext(spec.int_size());
    let cond = builder.input_ciphertext(spec.block_spec().message_size() as u16);
    let output = builder.iop_if_then_zero(&src, &cond);
    builder.output_ciphertext(output);
    builder
}

impl Builder {
    /// Zeroes an encrypted integer when a condition is true.
    ///
    /// When `cond` is zero (false) the result equals `src`; when `cond` is
    /// non-zero (true) the result is zero. The operation applies a single
    /// programmable bootstrapping per block, making it cheaper than a full
    /// [`iop_if_then_else`](Self::iop_if_then_else).
    ///
    /// The `cond` operand must be a single-block ciphertext (typically the
    /// output of a comparison operation such as
    /// [`iop_cmp`](Self::iop_cmp)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let src = builder.input_ciphertext(spec.int_size());
    /// # let cond = builder.input_ciphertext(spec.block_spec().message_size() as u16);
    /// let zeroed = builder.iop_if_then_zero(&src, &cond);
    /// ```
    pub fn iop_if_then_zero(&self, src: &Ciphertext, cond: &Ciphertext) -> Ciphertext {
        let src_blocks = self.split_ciphertext(src);
        let cond_blocks = self.split_ciphertext(cond);

        let output_blocks = src_blocks
            .iter()
            .map(|b| {
                let out = self.block_pack(&cond_blocks[0], b);
                self.block_lookup(&out, Lut1Def::IfFalseZeroed)
            })
            .cosvec();

        self.join_ciphertext(output_blocks)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_if_then_zero() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = if_then_zero(spec).into_ir();
        assert_display_is!(
            ir.format(),
            r#"
                %0 : Ct = input<0, Ct>();
                %1 : Ct = input<1, Ct>();
                %27 : Ct = decl_ct();
                %2 : CtBlock = extract_ct_block<0>(%0 : Ct);
                %3 : CtBlock = extract_ct_block<1>(%0 : Ct);
                %4 : CtBlock = extract_ct_block<2>(%0 : Ct);
                %5 : CtBlock = extract_ct_block<3>(%0 : Ct);
                %6 : CtBlock = extract_ct_block<4>(%0 : Ct);
                %7 : CtBlock = extract_ct_block<5>(%0 : Ct);
                %8 : CtBlock = extract_ct_block<6>(%0 : Ct);
                %9 : CtBlock = extract_ct_block<7>(%0 : Ct);
                %10 : CtBlock = extract_ct_block<0>(%1 : Ct);
                %11 : CtBlock = pack_ct<4>(%10 : CtBlock, %2 : CtBlock);
                %13 : CtBlock = pack_ct<4>(%10 : CtBlock, %3 : CtBlock);
                %15 : CtBlock = pack_ct<4>(%10 : CtBlock, %4 : CtBlock);
                %17 : CtBlock = pack_ct<4>(%10 : CtBlock, %5 : CtBlock);
                %19 : CtBlock = pack_ct<4>(%10 : CtBlock, %6 : CtBlock);
                %21 : CtBlock = pack_ct<4>(%10 : CtBlock, %7 : CtBlock);
                %23 : CtBlock = pack_ct<4>(%10 : CtBlock, %8 : CtBlock);
                %25 : CtBlock = pack_ct<4>(%10 : CtBlock, %9 : CtBlock);
                %12 : CtBlock = pbs<Protect, IfFalseZeroed>(%11 : CtBlock);
                %14 : CtBlock = pbs<Protect, IfFalseZeroed>(%13 : CtBlock);
                %16 : CtBlock = pbs<Protect, IfFalseZeroed>(%15 : CtBlock);
                %18 : CtBlock = pbs<Protect, IfFalseZeroed>(%17 : CtBlock);
                %20 : CtBlock = pbs<Protect, IfFalseZeroed>(%19 : CtBlock);
                %22 : CtBlock = pbs<Protect, IfFalseZeroed>(%21 : CtBlock);
                %24 : CtBlock = pbs<Protect, IfFalseZeroed>(%23 : CtBlock);
                %26 : CtBlock = pbs<Protect, IfFalseZeroed>(%25 : CtBlock);
                %28 : Ct = store_ct_block<0>(%12 : CtBlock, %27 : Ct);
                %29 : Ct = store_ct_block<1>(%14 : CtBlock, %28 : Ct);
                %30 : Ct = store_ct_block<2>(%16 : CtBlock, %29 : Ct);
                %31 : Ct = store_ct_block<3>(%18 : CtBlock, %30 : Ct);
                %32 : Ct = store_ct_block<4>(%20 : CtBlock, %31 : Ct);
                %33 : Ct = store_ct_block<5>(%22 : CtBlock, %32 : Ct);
                %34 : Ct = store_ct_block<6>(%24 : CtBlock, %33 : Ct);
                %35 : Ct = store_ct_block<7>(%26 : CtBlock, %34 : Ct);
                output<0, Ct>(%35 : Ct);
            "#
        );
    }
}
