use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::{
    SafeAs,
    iter::{CollectInSmallVec, MultiZip},
};

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
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let cond = builder.ciphertext_input(spec.block_spec().message_size().sas());
    let output = builder.iop_if_then_else(&src_a, &src_b, &cond);
    builder.ciphertext_output(output);
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
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// # let cond = builder.ciphertext_input(spec.block_spec().message_size() as u16);
    /// let selected = builder.iop_if_then_else(&a, &b, &cond);
    /// ```
    pub fn iop_if_then_else(
        &self,
        src_a: &Ciphertext,
        src_b: &Ciphertext,
        cond: &Ciphertext,
    ) -> Ciphertext {
        let src_a_blocks = self.ciphertext_split(src_a);
        let src_b_blocks = self.ciphertext_split(src_b);
        let cond_blocks = self.ciphertext_split(cond);

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

        self.ciphertext_join(output_blocks, None)
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
                %0 = input_ciphertext<0, 16>();
                %1 = input_ciphertext<1, 16>();
                %2 = input_ciphertext<2, 2>();
                %3 = extract_ct_block<0>(%0);
                %4 = extract_ct_block<1>(%0);
                %5 = extract_ct_block<2>(%0);
                %6 = extract_ct_block<3>(%0);
                %7 = extract_ct_block<4>(%0);
                %8 = extract_ct_block<5>(%0);
                %9 = extract_ct_block<6>(%0);
                %10 = extract_ct_block<7>(%0);
                %11 = extract_ct_block<0>(%1);
                %12 = extract_ct_block<1>(%1);
                %13 = extract_ct_block<2>(%1);
                %14 = extract_ct_block<3>(%1);
                %15 = extract_ct_block<4>(%1);
                %16 = extract_ct_block<5>(%1);
                %17 = extract_ct_block<6>(%1);
                %18 = extract_ct_block<7>(%1);
                %19 = extract_ct_block<0>(%2);
                %20 = pack_ct<4>(%19, %3);
                %21 = pbs<Protect, Lut1("IfFalseZeroed")>(%20);
                %22 = pack_ct<4>(%19, %11);
                %23 = pbs<Protect, Lut1("IfTrueZeroed")>(%22);
                %24 = add_ct(%21, %23);
                %25 = pack_ct<4>(%19, %4);
                %26 = pbs<Protect, Lut1("IfFalseZeroed")>(%25);
                %27 = pack_ct<4>(%19, %12);
                %28 = pbs<Protect, Lut1("IfTrueZeroed")>(%27);
                %29 = add_ct(%26, %28);
                %30 = pack_ct<4>(%19, %5);
                %31 = pbs<Protect, Lut1("IfFalseZeroed")>(%30);
                %32 = pack_ct<4>(%19, %13);
                %33 = pbs<Protect, Lut1("IfTrueZeroed")>(%32);
                %34 = add_ct(%31, %33);
                %35 = pack_ct<4>(%19, %6);
                %36 = pbs<Protect, Lut1("IfFalseZeroed")>(%35);
                %37 = pack_ct<4>(%19, %14);
                %38 = pbs<Protect, Lut1("IfTrueZeroed")>(%37);
                %39 = add_ct(%36, %38);
                %40 = pack_ct<4>(%19, %7);
                %41 = pbs<Protect, Lut1("IfFalseZeroed")>(%40);
                %42 = pack_ct<4>(%19, %15);
                %43 = pbs<Protect, Lut1("IfTrueZeroed")>(%42);
                %44 = add_ct(%41, %43);
                %45 = pack_ct<4>(%19, %8);
                %46 = pbs<Protect, Lut1("IfFalseZeroed")>(%45);
                %47 = pack_ct<4>(%19, %16);
                %48 = pbs<Protect, Lut1("IfTrueZeroed")>(%47);
                %49 = add_ct(%46, %48);
                %50 = pack_ct<4>(%19, %9);
                %51 = pbs<Protect, Lut1("IfFalseZeroed")>(%50);
                %52 = pack_ct<4>(%19, %17);
                %53 = pbs<Protect, Lut1("IfTrueZeroed")>(%52);
                %54 = add_ct(%51, %53);
                %55 = pack_ct<4>(%19, %10);
                %56 = pbs<Protect, Lut1("IfFalseZeroed")>(%55);
                %57 = pack_ct<4>(%19, %18);
                %58 = pbs<Protect, Lut1("IfTrueZeroed")>(%57);
                %59 = add_ct(%56, %58);
                %60 = decl_ct<16>();
                %61 = store_ct_block<0>(%24, %60);
                %62 = store_ct_block<1>(%29, %61);
                %63 = store_ct_block<2>(%34, %62);
                %64 = store_ct_block<3>(%39, %63);
                %65 = store_ct_block<4>(%44, %64);
                %66 = store_ct_block<5>(%49, %65);
                %67 = store_ct_block<6>(%54, %66);
                %68 = store_ct_block<7>(%59, %67);
                output<0>(%68);
            "#
        );
    }
}
