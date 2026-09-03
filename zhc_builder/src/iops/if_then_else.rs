use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::{
    SafeAs,
    iter::{CollectInSmallVec, MultiZip},
};

use crate::{Ciphertext, builder::Builder};

/// Creates an IR for conditional select between two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_if_then_else`]. Declares two
/// integer inputs, one boolean condition input, and one output.
/// See the builder method for details.
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
                let sum = self.block_add(&cond_a, &cond_b);
                self.block_lookup(&sum, Lut1Def::MsgOnly)
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
        let ir = if_then_else(spec).optimize_ir();
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
                %25 = pbs<Protect, Lut1("MsgOnly")>(%24);
                %26 = pack_ct<4>(%19, %4);
                %27 = pbs<Protect, Lut1("IfFalseZeroed")>(%26);
                %28 = pack_ct<4>(%19, %12);
                %29 = pbs<Protect, Lut1("IfTrueZeroed")>(%28);
                %30 = add_ct(%27, %29);
                %31 = pbs<Protect, Lut1("MsgOnly")>(%30);
                %32 = pack_ct<4>(%19, %5);
                %33 = pbs<Protect, Lut1("IfFalseZeroed")>(%32);
                %34 = pack_ct<4>(%19, %13);
                %35 = pbs<Protect, Lut1("IfTrueZeroed")>(%34);
                %36 = add_ct(%33, %35);
                %37 = pbs<Protect, Lut1("MsgOnly")>(%36);
                %38 = pack_ct<4>(%19, %6);
                %39 = pbs<Protect, Lut1("IfFalseZeroed")>(%38);
                %40 = pack_ct<4>(%19, %14);
                %41 = pbs<Protect, Lut1("IfTrueZeroed")>(%40);
                %42 = add_ct(%39, %41);
                %43 = pbs<Protect, Lut1("MsgOnly")>(%42);
                %44 = pack_ct<4>(%19, %7);
                %45 = pbs<Protect, Lut1("IfFalseZeroed")>(%44);
                %46 = pack_ct<4>(%19, %15);
                %47 = pbs<Protect, Lut1("IfTrueZeroed")>(%46);
                %48 = add_ct(%45, %47);
                %49 = pbs<Protect, Lut1("MsgOnly")>(%48);
                %50 = pack_ct<4>(%19, %8);
                %51 = pbs<Protect, Lut1("IfFalseZeroed")>(%50);
                %52 = pack_ct<4>(%19, %16);
                %53 = pbs<Protect, Lut1("IfTrueZeroed")>(%52);
                %54 = add_ct(%51, %53);
                %55 = pbs<Protect, Lut1("MsgOnly")>(%54);
                %56 = pack_ct<4>(%19, %9);
                %57 = pbs<Protect, Lut1("IfFalseZeroed")>(%56);
                %58 = pack_ct<4>(%19, %17);
                %59 = pbs<Protect, Lut1("IfTrueZeroed")>(%58);
                %60 = add_ct(%57, %59);
                %61 = pbs<Protect, Lut1("MsgOnly")>(%60);
                %62 = pack_ct<4>(%19, %10);
                %63 = pbs<Protect, Lut1("IfFalseZeroed")>(%62);
                %64 = pack_ct<4>(%19, %18);
                %65 = pbs<Protect, Lut1("IfTrueZeroed")>(%64);
                %66 = add_ct(%63, %65);
                %67 = pbs<Protect, Lut1("MsgOnly")>(%66);
                %68 = decl_ct<16>();
                %78 = store_ct_block<0>(%25, %68);
                %79 = store_ct_block<1>(%31, %78);
                %80 = store_ct_block<2>(%37, %79);
                %81 = store_ct_block<3>(%43, %80);
                %82 = store_ct_block<4>(%49, %81);
                %83 = store_ct_block<5>(%55, %82);
                %84 = store_ct_block<6>(%61, %83);
                %85 = store_ct_block<7>(%67, %84);
                output<0>(%85);
            "#
        );
    }

    #[test]
    fn noise_if_then_else() {
        for size in (2..128).step_by(2) {
            if_then_else(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
