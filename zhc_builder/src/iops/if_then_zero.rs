use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::{SafeAs, iter::CollectInSmallVec};

use crate::{Ciphertext, builder::Builder};

/// Creates an IR for conditional zeroing of an encrypted integer.
///
/// Convenience wrapper that calls [`Builder::iop_if_then_zero`]. Declares one
/// integer input, one boolean condition input, and one output.
/// See the builder method for details.
pub fn if_then_zero(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let cond = builder.ciphertext_input(spec.block_spec().message_size().sas());
    let output = builder.iop_if_then_zero(&src, &cond);
    builder.ciphertext_output(output);
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
    /// # let src = builder.ciphertext_input(spec.int_size());
    /// # let cond = builder.ciphertext_input(spec.block_spec().message_size() as u16);
    /// let zeroed = builder.iop_if_then_zero(&src, &cond);
    /// ```
    pub fn iop_if_then_zero(&self, src: &Ciphertext, cond: &Ciphertext) -> Ciphertext {
        let src_blocks = self.ciphertext_split(src);
        let cond_blocks = self.ciphertext_split(cond);

        let output_blocks = src_blocks
            .iter()
            .map(|b| {
                let out = self.block_pack(&cond_blocks[0], b);
                self.block_lookup(&out, Lut1Def::IfFalseZeroed)
            })
            .cosvec();

        self.ciphertext_join(output_blocks, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_if_then_zero() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = if_then_zero(spec).optimize_ir();
        assert_display_is!(
            ir.format(),
            r#"
                %0 = input_ciphertext<0, 16>();
                %1 = input_ciphertext<1, 2>();
                %2 = extract_ct_block<0>(%0);
                %3 = extract_ct_block<1>(%0);
                %4 = extract_ct_block<2>(%0);
                %5 = extract_ct_block<3>(%0);
                %6 = extract_ct_block<4>(%0);
                %7 = extract_ct_block<5>(%0);
                %8 = extract_ct_block<6>(%0);
                %9 = extract_ct_block<7>(%0);
                %10 = extract_ct_block<0>(%1);
                %11 = pack_ct<4>(%10, %2);
                %12 = pbs<Protect, Lut1("IfFalseZeroed")>(%11);
                %13 = pack_ct<4>(%10, %3);
                %14 = pbs<Protect, Lut1("IfFalseZeroed")>(%13);
                %15 = pack_ct<4>(%10, %4);
                %16 = pbs<Protect, Lut1("IfFalseZeroed")>(%15);
                %17 = pack_ct<4>(%10, %5);
                %18 = pbs<Protect, Lut1("IfFalseZeroed")>(%17);
                %19 = pack_ct<4>(%10, %6);
                %20 = pbs<Protect, Lut1("IfFalseZeroed")>(%19);
                %21 = pack_ct<4>(%10, %7);
                %22 = pbs<Protect, Lut1("IfFalseZeroed")>(%21);
                %23 = pack_ct<4>(%10, %8);
                %24 = pbs<Protect, Lut1("IfFalseZeroed")>(%23);
                %25 = pack_ct<4>(%10, %9);
                %26 = pbs<Protect, Lut1("IfFalseZeroed")>(%25);
                %27 = decl_ct<16>();
                %37 = store_ct_block<0>(%12, %27);
                %38 = store_ct_block<1>(%14, %37);
                %39 = store_ct_block<2>(%16, %38);
                %40 = store_ct_block<3>(%18, %39);
                %41 = store_ct_block<4>(%20, %40);
                %42 = store_ct_block<5>(%22, %41);
                %43 = store_ct_block<6>(%24, %42);
                %44 = store_ct_block<7>(%26, %43);
                output<0>(%44);
            "#
        );
    }

    #[test]
    fn noise_if_then_zero() {
        for size in (2..128).step_by(2) {
            if_then_zero(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
