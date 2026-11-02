use crate::{CiphertextInteger, builder::Builder};
use zhc_crypto::integer_semantics::SpecInteger;
use zhc_utils::SafeAs;

/// Creates an IR casting a ciphertext of `spec` to `to_size` bits. See [`Builder::iop_cast`].
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{SpecInteger, cast};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = cast(spec, 32);
/// let ir = builder.ir();
/// ```
pub fn cast(spec: SpecInteger, to_size: u16) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_integer_input(spec.int_size());
    let output = builder.iop_cast(&src, to_size);
    builder.ciphertext_integer_output(output);
    builder
}

impl Builder {
    /// Casts a ciphertext to `to_size` bits: widening zero-extends,
    /// narrowing truncates MSB blocks (value modulo `2^to_size`).
    /// No PBS. Same-size degenerates to a block copy.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{SpecInteger, Builder};
    /// # let spec = SpecInteger::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let ct = builder.ciphertext_integer_input(spec.int_size());
    /// let wide = builder.iop_cast(&ct, 32);
    /// ```
    // NOTE: unsigned only; sign extension unsuported in zhc today
    pub fn iop_cast(&self, src: &CiphertextInteger, to_size: u16) -> CiphertextInteger {
        let to_block_count = to_size
            .div_ceil(self.spec().message_size().sas::<u16>())
            .sas::<usize>();
        let src_blocks = self.ciphertext_integer_split(src);
        let kept = src_blocks.len().min(to_block_count);
        self.ciphertext_integer_join(&src_blocks[..kept], Some(to_size))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::SpecInteger;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_cast_widen() {
        let ir = cast(SpecInteger::new(8, 2, 2), 16);
        assert_display_is!(
            zhc_pipeline::passes::optimize_iop(&ir.ir())
                .format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 8>();
                %1 = extract_ct_block<0>(%0);
                %2 = extract_ct_block<1>(%0);
                %3 = extract_ct_block<2>(%0);
                %4 = extract_ct_block<3>(%0);
                %5 = decl_ct<16>();
                %6 = let_ct_block<0>();
                %11 = store_ct_block<4>(%6, %5);
                %12 = store_ct_block<5>(%6, %11);
                %13 = store_ct_block<6>(%6, %12);
                %14 = store_ct_block<7>(%6, %13);
                %15 = store_ct_block<0>(%1, %14);
                %16 = store_ct_block<1>(%2, %15);
                %17 = store_ct_block<2>(%3, %16);
                %18 = store_ct_block<3>(%4, %17);
                output<0>(%18);
            "#
        );
    }

    #[test]
    fn test_cast_narrow() {
        let ir = cast(SpecInteger::new(16, 2, 2), 8);
        assert_display_is!(
            zhc_pipeline::passes::optimize_iop(&ir.ir())
                .format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 16>();
                %1 = extract_ct_block<0>(%0);
                %2 = extract_ct_block<1>(%0);
                %3 = extract_ct_block<2>(%0);
                %4 = extract_ct_block<3>(%0);
                %9 = decl_ct<8>();
                %15 = store_ct_block<0>(%1, %9);
                %16 = store_ct_block<1>(%2, %15);
                %17 = store_ct_block<2>(%3, %16);
                %18 = store_ct_block<3>(%4, %17);
                output<0>(%18);
            "#
        );
    }

    #[test]
    fn correctness_cast() {
        fn semantic(to_size: u16) -> impl Fn(&[IopValue]) -> Option<Vec<IopValue>> {
            move |inp: &[IopValue]| {
                let [IopValue::CiphertextInteger(src)] = inp else {
                    unreachable!()
                };
                let to_spec = SpecInteger::new(to_size, 2, 2);
                let output = to_spec.from_int(src.as_storage() & to_spec.int_mask());
                Some(vec![IopValue::CiphertextInteger(output)])
            }
        }
        // (from, to) pairs covering widen, narrow, same-size copy, bool→uint and
        // 128-bit extremes.
        for (from, to) in [
            (2, 16),
            (2, 64),
            (8, 16),
            (16, 8),
            (16, 16),
            (16, 2),
            (64, 128),
            (128, 64),
        ] {
            cast(SpecInteger::new(from, 2, 2), to).test_random(100, semantic(to));
        }
    }
}
