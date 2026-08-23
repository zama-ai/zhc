use crate::{Ciphertext, Plaintext, builder::Builder};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_utils::iter::CollectInSmallVec;

/// Creates an IR for trivial encryption of a plaintext integer.
///
/// Convenience wrapper that declares a plaintext input and a ciphertext output, then calls
/// [`Builder::iop_trivial_encrypt`]. See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, trivial_encrypt};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = trivial_encrypt(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn trivial_encrypt(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.plaintext_input(spec.int_size());
    let output = builder.iop_trivial_encrypt(&src);
    builder.ciphertext_output(output);
    builder
}

impl Builder {
    /// Trivially encrypts a plaintext integer, producing a ciphertext with no noise.
    ///
    /// A *trivial ciphertext* encodes the plaintext openly: the mask is zero, so the message
    /// is directly readable from the ciphertext without a secret key. This makes it the
    /// standard mechanism for injecting public constants into a homomorphic computation —
    /// the resulting [`Ciphertext`] can be combined with truly encrypted operands in any
    /// subsequent operation.
    ///
    /// Each plaintext block of `src` is added to a freshly allocated zero ciphertext block via
    /// `add_plaintext`. The resulting blocks are then joined into a [`Ciphertext`] whose spec
    /// matches `src`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let pt = builder.plaintext_input(spec.int_size());
    /// let ct = builder.iop_trivial_encrypt(&pt);
    /// ```
    pub fn iop_trivial_encrypt(&self, src: &Plaintext) -> Ciphertext {
        let src_blocks = self.plaintext_split(src);
        let zero_ct = self.block_let_ciphertext(0);
        let blocks = src_blocks
            .iter()
            .map(|block| self.block_add_plaintext(zero_ct, block))
            .cosvec();
        // Pass the width explicitly: inferring it from the block count would round an `int_size`
        // that is not a multiple of `message_size` up, and the result is documented to match `src`.
        self.ciphertext_join(blocks, Some(src.spec().int_size()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::CiphertextSpec;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_trivial_encrypt() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = trivial_encrypt(spec);
        assert_display_is!(
            ir.optimize_ir()
                .format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_plaintext<0, 16>();
                %1 = extract_pt_block<0>(%0);
                %2 = extract_pt_block<1>(%0);
                %3 = extract_pt_block<2>(%0);
                %4 = extract_pt_block<3>(%0);
                %5 = extract_pt_block<4>(%0);
                %6 = extract_pt_block<5>(%0);
                %7 = extract_pt_block<6>(%0);
                %8 = extract_pt_block<7>(%0);
                %9 = let_ct_block<0>();
                %10 = add_pt(%9, %1);
                %11 = add_pt(%9, %2);
                %12 = add_pt(%9, %3);
                %13 = add_pt(%9, %4);
                %14 = add_pt(%9, %5);
                %15 = add_pt(%9, %6);
                %16 = add_pt(%9, %7);
                %17 = add_pt(%9, %8);
                %18 = decl_ct<16>();
                %28 = store_ct_block<0>(%10, %18);
                %29 = store_ct_block<1>(%11, %28);
                %30 = store_ct_block<2>(%12, %29);
                %31 = store_ct_block<3>(%13, %30);
                %32 = store_ct_block<4>(%14, %31);
                %33 = store_ct_block<5>(%15, %32);
                %34 = store_ct_block<6>(%16, %33);
                %35 = store_ct_block<7>(%17, %34);
                output<0>(%35);
            "#
        );
    }

    #[test]
    fn correctness_trivial_encrypt() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Plaintext(src)] = inp else {
                unreachable!()
            };
            let output =
                CiphertextSpec::new(src.spec().int_size(), 2, 2).from_int(src.as_storage());
            Some(vec![IopValue::Ciphertext(output)])
        }
        for size in (2..128).step_by(2) {
            trivial_encrypt(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }
}
