use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::{
    SafeAs,
    iter::{CollectInSmallVec, MultiZip},
};

use crate::{Ciphertext, builder::Builder};

/// Creates an IR for a conditional swap of two encrypted integers.
/// See [`Builder::iop_flip`].
pub fn flip(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let cond = builder.ciphertext_input(spec.block_spec().message_size().sas());
    let (out_a, out_b) = builder.iop_flip(&src_a, &src_b, &cond);
    builder.ciphertext_output(out_a);
    builder.ciphertext_output(out_b);
    builder
}

impl Builder {
    /// Conditionally swaps two encrypted integers:
    /// returns `(src_b, src_a)` when `cond` is non-zero, `(src_a, src_b)` otherwise.
    /// Mirrors tfhe-rs `flip_parallelized`; both packs per block are shared
    /// by the two outputs, so it costs 4 PBS per block instead of the 4 packs
    /// and 4 PBS of two `iop_if_then_else`.
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
    /// let (x, y) = builder.iop_flip(&a, &b, &cond);
    /// ```
    pub fn iop_flip(
        &self,
        src_a: &Ciphertext,
        src_b: &Ciphertext,
        cond: &Ciphertext,
    ) -> (Ciphertext, Ciphertext) {
        let src_a_blocks = self.ciphertext_split(src_a);
        let src_b_blocks = self.ciphertext_split(src_b);
        let cond_blocks = self.ciphertext_split(cond);

        let (out_a_blocks, out_b_blocks): (Vec<_>, Vec<_>) =
            (src_a_blocks.iter(), src_b_blocks.iter())
                .mzip()
                .map(|(a, b)| {
                    // Mask each value both ways depending on the conditition
                    let packed_a = self.block_pack(&cond_blocks[0], a);
                    let a_if_true = self.block_lookup(&packed_a, Lut1Def::IfFalseZeroed);
                    let a_if_false = self.block_lookup(&packed_a, Lut1Def::IfTrueZeroed);

                    let packed_b = self.block_pack(&cond_blocks[0], b);
                    let b_if_true = self.block_lookup(&packed_b, Lut1Def::IfFalseZeroed);
                    let b_if_false = self.block_lookup(&packed_b, Lut1Def::IfTrueZeroed);

                    // Crossing the masks is the swap; one operand per add is zero.
                    (
                        self.block_add(&a_if_false, &b_if_true),
                        self.block_add(&a_if_true, &b_if_false),
                    )
                })
                .unzip();

        (
            self.ciphertext_join(out_a_blocks.into_iter().cosvec(), None),
            self.ciphertext_join(out_b_blocks.into_iter().cosvec(), None),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_flip() {
        let ir = flip(CiphertextSpec::new(4, 2, 2)).optimize_ir();
        assert_display_is!(
            ir.format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 4>();
                %1 = input_ciphertext<1, 4>();
                %2 = input_ciphertext<2, 2>();
                %3 = extract_ct_block<0>(%0);
                %4 = extract_ct_block<1>(%0);
                %5 = extract_ct_block<0>(%1);
                %6 = extract_ct_block<1>(%1);
                %7 = extract_ct_block<0>(%2);
                %8 = pack_ct<4>(%7, %3);
                %9 = pbs<Protect, Lut1("IfFalseZeroed")>(%8);
                %10 = pbs<Protect, Lut1("IfTrueZeroed")>(%8);
                %11 = pack_ct<4>(%7, %5);
                %12 = pbs<Protect, Lut1("IfFalseZeroed")>(%11);
                %13 = pbs<Protect, Lut1("IfTrueZeroed")>(%11);
                %14 = add_ct(%10, %12);
                %15 = add_ct(%9, %13);
                %16 = pack_ct<4>(%7, %4);
                %17 = pbs<Protect, Lut1("IfFalseZeroed")>(%16);
                %18 = pbs<Protect, Lut1("IfTrueZeroed")>(%16);
                %19 = pack_ct<4>(%7, %6);
                %20 = pbs<Protect, Lut1("IfFalseZeroed")>(%19);
                %21 = pbs<Protect, Lut1("IfTrueZeroed")>(%19);
                %22 = add_ct(%18, %20);
                %23 = add_ct(%17, %21);
                %24 = decl_ct<4>();
                %28 = store_ct_block<0>(%14, %24);
                %29 = store_ct_block<1>(%22, %28);
                %34 = store_ct_block<0>(%15, %24);
                %35 = store_ct_block<1>(%23, %34);
                output<0>(%29);
                output<1>(%35);
            "#
        );
    }

    #[test]
    fn correctness_flip() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::Ciphertext(a),
                IopValue::Ciphertext(b),
                IopValue::Ciphertext(cond),
            ] = inp
            else {
                unreachable!()
            };
            let (x, y) = if cond.as_storage() != 0 {
                (b, a)
            } else {
                (a, b)
            };
            Some(vec![
                IopValue::Ciphertext(x.clone()),
                IopValue::Ciphertext(y.clone()),
            ])
        }
        for size in (2..=128).step_by(2) {
            flip(CiphertextSpec::new(size, 2, 2)).test_random(20, semantic);
        }
    }
}
