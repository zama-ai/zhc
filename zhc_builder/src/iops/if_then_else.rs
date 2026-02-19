use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::iter::{CollectInSmallVec, MultiZip};

use crate::{Ciphertext, builder::Builder};

/// Creates an IR for a conditional select between two encrypted integers.
///
/// The returned [`Builder`] declares three ciphertext inputs — two integer
/// operands and one single-block boolean condition — and one ciphertext
/// output. When the condition is zero (false) the output equals the first
/// operand; when it is non-zero (true) the output equals the second.
/// Internally delegates to [`Builder::iop_if_then_else`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition. The condition input is automatically sized to a single
/// message block.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, if_then_else};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = if_then_else(spec);
/// let ir = builder.into_ir();
/// ```
pub fn if_then_else(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());
    let cond = builder.input_ciphertext(spec.block_spec().message_size() as u16);
    let output = builder.iop_if_then_else(&src_a, &src_b, &cond);
    builder.output_ciphertext(output);
    builder
}

impl Builder {
    /// Selects between two encrypted integers based on an encrypted condition.
    ///
    /// When `cond` is zero (false) the result equals `src_a`; when `cond` is
    /// non-zero (true) the result equals `src_b`. The selection is performed
    /// block-wise: each block of `src_a` is zeroed when the condition is true
    /// and each block of `src_b` is zeroed when it is false, then the two
    /// are added together.
    ///
    /// Both `src_a` and `src_b` must have the same block decomposition, and
    /// `cond` must be a single-block ciphertext (typically the output of a
    /// comparison operation such as [`iop_cmp`](Self::iop_cmp)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.input_ciphertext(spec.int_size());
    /// # let b = builder.input_ciphertext(spec.int_size());
    /// # let cond = builder.input_ciphertext(spec.block_spec().message_size() as u16);
    /// let selected = builder.iop_if_then_else(&a, &b, &cond);
    /// ```
    pub fn iop_if_then_else(
        &self,
        src_a: &Ciphertext,
        src_b: &Ciphertext,
        cond: &Ciphertext,
    ) -> Ciphertext {
        let src_a_blocks = self.split_ciphertext(src_a);
        let src_b_blocks = self.split_ciphertext(src_b);
        let cond_blocks = self.split_ciphertext(cond);

        let output_blocks = (src_a_blocks.iter(), src_b_blocks.iter())
            .mzip()
            .map(|(a, b)| {
                let cond_a = self.block_pack(&cond_blocks[0], a);
                let cond_a = self.block_lookup(&cond_a, Lut1Def::IfFalseZeroed);
                let cond_b = self.block_pack(&cond_blocks[0], b);
                let cond_b = self.block_lookup(&cond_b, Lut1Def::IfTrueZeroed);
                self.block_add(&cond_a, &cond_b)
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
    fn test_if_then_else() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = if_then_else(spec).into_ir();
        assert_display_is!(
            ir.format(),
            r#"
                %0 : Ct = input<0, Ct>();
                %1 : Ct = input<1, Ct>();
                %2 : Ct = input<2, Ct>();
                %60 : Ct = decl_ct();
                %3 : CtBlock = extract_ct_block<0>(%0 : Ct);
                %4 : CtBlock = extract_ct_block<1>(%0 : Ct);
                %5 : CtBlock = extract_ct_block<2>(%0 : Ct);
                %6 : CtBlock = extract_ct_block<3>(%0 : Ct);
                %7 : CtBlock = extract_ct_block<4>(%0 : Ct);
                %8 : CtBlock = extract_ct_block<5>(%0 : Ct);
                %9 : CtBlock = extract_ct_block<6>(%0 : Ct);
                %10 : CtBlock = extract_ct_block<7>(%0 : Ct);
                %11 : CtBlock = extract_ct_block<0>(%1 : Ct);
                %12 : CtBlock = extract_ct_block<1>(%1 : Ct);
                %13 : CtBlock = extract_ct_block<2>(%1 : Ct);
                %14 : CtBlock = extract_ct_block<3>(%1 : Ct);
                %15 : CtBlock = extract_ct_block<4>(%1 : Ct);
                %16 : CtBlock = extract_ct_block<5>(%1 : Ct);
                %17 : CtBlock = extract_ct_block<6>(%1 : Ct);
                %18 : CtBlock = extract_ct_block<7>(%1 : Ct);
                %19 : CtBlock = extract_ct_block<0>(%2 : Ct);
                %20 : CtBlock = pack_ct<4>(%19 : CtBlock, %3 : CtBlock);
                %22 : CtBlock = pack_ct<4>(%19 : CtBlock, %11 : CtBlock);
                %25 : CtBlock = pack_ct<4>(%19 : CtBlock, %4 : CtBlock);
                %27 : CtBlock = pack_ct<4>(%19 : CtBlock, %12 : CtBlock);
                %30 : CtBlock = pack_ct<4>(%19 : CtBlock, %5 : CtBlock);
                %32 : CtBlock = pack_ct<4>(%19 : CtBlock, %13 : CtBlock);
                %35 : CtBlock = pack_ct<4>(%19 : CtBlock, %6 : CtBlock);
                %37 : CtBlock = pack_ct<4>(%19 : CtBlock, %14 : CtBlock);
                %40 : CtBlock = pack_ct<4>(%19 : CtBlock, %7 : CtBlock);
                %42 : CtBlock = pack_ct<4>(%19 : CtBlock, %15 : CtBlock);
                %45 : CtBlock = pack_ct<4>(%19 : CtBlock, %8 : CtBlock);
                %47 : CtBlock = pack_ct<4>(%19 : CtBlock, %16 : CtBlock);
                %50 : CtBlock = pack_ct<4>(%19 : CtBlock, %9 : CtBlock);
                %52 : CtBlock = pack_ct<4>(%19 : CtBlock, %17 : CtBlock);
                %55 : CtBlock = pack_ct<4>(%19 : CtBlock, %10 : CtBlock);
                %57 : CtBlock = pack_ct<4>(%19 : CtBlock, %18 : CtBlock);
                %21 : CtBlock = pbs<IfFalseZeroed>(%20 : CtBlock);
                %23 : CtBlock = pbs<IfTrueZeroed>(%22 : CtBlock);
                %26 : CtBlock = pbs<IfFalseZeroed>(%25 : CtBlock);
                %28 : CtBlock = pbs<IfTrueZeroed>(%27 : CtBlock);
                %31 : CtBlock = pbs<IfFalseZeroed>(%30 : CtBlock);
                %33 : CtBlock = pbs<IfTrueZeroed>(%32 : CtBlock);
                %36 : CtBlock = pbs<IfFalseZeroed>(%35 : CtBlock);
                %38 : CtBlock = pbs<IfTrueZeroed>(%37 : CtBlock);
                %41 : CtBlock = pbs<IfFalseZeroed>(%40 : CtBlock);
                %43 : CtBlock = pbs<IfTrueZeroed>(%42 : CtBlock);
                %46 : CtBlock = pbs<IfFalseZeroed>(%45 : CtBlock);
                %48 : CtBlock = pbs<IfTrueZeroed>(%47 : CtBlock);
                %51 : CtBlock = pbs<IfFalseZeroed>(%50 : CtBlock);
                %53 : CtBlock = pbs<IfTrueZeroed>(%52 : CtBlock);
                %56 : CtBlock = pbs<IfFalseZeroed>(%55 : CtBlock);
                %58 : CtBlock = pbs<IfTrueZeroed>(%57 : CtBlock);
                %24 : CtBlock = add_ct(%21 : CtBlock, %23 : CtBlock);
                %29 : CtBlock = add_ct(%26 : CtBlock, %28 : CtBlock);
                %34 : CtBlock = add_ct(%31 : CtBlock, %33 : CtBlock);
                %39 : CtBlock = add_ct(%36 : CtBlock, %38 : CtBlock);
                %44 : CtBlock = add_ct(%41 : CtBlock, %43 : CtBlock);
                %49 : CtBlock = add_ct(%46 : CtBlock, %48 : CtBlock);
                %54 : CtBlock = add_ct(%51 : CtBlock, %53 : CtBlock);
                %59 : CtBlock = add_ct(%56 : CtBlock, %58 : CtBlock);
                %61 : Ct = store_ct_block<0>(%24 : CtBlock, %60 : Ct);
                %62 : Ct = store_ct_block<1>(%29 : CtBlock, %61 : Ct);
                %63 : Ct = store_ct_block<2>(%34 : CtBlock, %62 : Ct);
                %64 : Ct = store_ct_block<3>(%39 : CtBlock, %63 : Ct);
                %65 : Ct = store_ct_block<4>(%44 : CtBlock, %64 : Ct);
                %66 : Ct = store_ct_block<5>(%49 : CtBlock, %65 : Ct);
                %67 : Ct = store_ct_block<6>(%54 : CtBlock, %66 : Ct);
                %68 : Ct = store_ct_block<7>(%59 : CtBlock, %67 : Ct);
                output<0, Ct>(%68 : Ct);
            "#
        );
    }
}
