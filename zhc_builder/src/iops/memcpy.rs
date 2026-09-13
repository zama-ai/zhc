use zhc_crypto::integer_semantics::CiphertextSpec;

use crate::builder::{Builder, Ciphertext};

/// Shortest legal IOp stream, in words (one length word plus one word per DOp).
///
/// The instruction scheduler rejects shorter streams, because too few DOps per IOp lets the sync
/// id overflow, and every board configuration uses 4. The value cannot be read from
/// [`CiphertextSpec`] nor from the HPU configuration (`min_iop_size` is not deserialized), so it is
/// written here. It only bites the single-block case, see [`Builder::iop_memcpy`].
const MIN_IOP_WORDS: usize = 4;

/// Creates an IR for the copy of an encrypted integer (`dst = src`).
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_memcpy`].
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, memcpy};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = memcpy(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn memcpy(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_memcpy(&src);
    builder.ciphertext_output(res);
    builder
}

impl Builder {
    /// Copies an encrypted integer block for block.
    ///
    /// Splitting then joining is the whole operation: the extracts read the source and the stores
    /// fill a fresh ciphertext, which lowers to one `LD` and one `ST` per block. The target has no
    /// memory to memory move, so going through the register file is the only way.
    ///
    /// A one-block integer gives a two-DOp stream, one word short of the minimum stream length.
    /// That case is padded with the addition of a null plaintext: one linear DOp, no PBS, and it
    /// changes neither the value nor the degree of the block. The padding has to sit on the data
    /// path, as a dangling constant would simply be dropped by dead code elimination.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// let copy = builder.iop_memcpy(&a);
    /// ```
    pub fn iop_memcpy(&self, src: &Ciphertext) -> Ciphertext {
        let src_blocks = self.ciphertext_split(src);
        let words = 1 + 2 * src_blocks.len();
        let blocks = if words >= MIN_IOP_WORDS {
            src_blocks
        } else {
            let zero = self.block_let_plaintext(0);
            src_blocks
                .iter()
                .map(|block| self.comment("Pad Stream").block_add_plaintext(block, zero))
                .collect()
        };
        self.ciphertext_join(&blocks, Some(src.spec().int_size()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn correctness_memcpy() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(src)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(*src)])
        }
        // From 2 bits, i.e. from the single-block case that needs stream padding.
        for size in (2..128).step_by(2) {
            memcpy(CiphertextSpec::new(size, 2, 2)).test_random(10, semantic);
        }
    }

    // A copy must not be optimized away, and must stay a plain extract/store pair.
    #[test]
    fn test_memcpy() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let ir = memcpy(spec).optimize_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true),
            r#"
                %0 = input_ciphertext<0, 8>();
                %1 = extract_ct_block<0>(%0);
                %2 = extract_ct_block<1>(%0);
                %3 = extract_ct_block<2>(%0);
                %4 = extract_ct_block<3>(%0);
                %5 = decl_ct<8>();
                %11 = store_ct_block<0>(%1, %5);
                %12 = store_ct_block<1>(%2, %11);
                %13 = store_ct_block<2>(%3, %12);
                %14 = store_ct_block<3>(%4, %13);
                output<0>(%14);
            "#
        );
    }

    #[test]
    fn noise_memcpy() {
        for size in (2..128).step_by(2) {
            memcpy(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }
}
