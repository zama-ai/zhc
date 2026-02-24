use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::{
    iter::{ChunkIt, UnwrapChunks},
    n_bits_to_encode,
};

use crate::{
    CiphertextBlock, NU,
    builder::{Builder, Ciphertext},
};

/// Selects which bit value to count in an encrypted integer.
///
/// Passed to [`count_0`], [`count_1`], or [`Builder::iop_count`] to choose
/// whether the operation counts zero bits or one bits. The semantics mirror
/// the standard `count_zeros` / `count_ones` methods on Rust integer types,
/// restricted to the declared `int_size` of the [`CiphertextSpec`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountKind {
    /// Counts the number of **zero** bits in the encrypted integer.
    Zeros,
    /// Counts the number of **one** bits in the encrypted integer.
    Ones,
}

/// Creates an IR that counts the number of **zero** bits in an encrypted integer.
///
/// The returned [`Builder`] declares one ciphertext input of `spec.int_size()`
/// bits and one ciphertext output whose width is `⌈log₂(int_size + 1)⌉` bits
/// — just enough to represent every possible count from 0 to `int_size`.
/// Internally delegates to [`Builder::iop_count`] with [`CountKind::Zeros`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, count_0};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = count_0(spec);
/// let ir = builder.into_ir();
/// ```
pub fn count_0(spec: CiphertextSpec) -> Builder {
    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let res = builder.iop_count(&src_a, CountKind::Zeros);
    builder.output_ciphertext(res);
    builder
}

/// Creates an IR that counts the number of **one** bits in an encrypted integer.
///
/// The returned [`Builder`] declares one ciphertext input of `spec.int_size()`
/// bits and one ciphertext output whose width is `⌈log₂(int_size + 1)⌉` bits
/// — just enough to represent every possible count from 0 to `int_size`.
/// Internally delegates to [`Builder::iop_count`] with [`CountKind::Ones`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, count_1};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = count_1(spec);
/// let ir = builder.into_ir();
/// ```
pub fn count_1(spec: CiphertextSpec) -> Builder {
    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let res = builder.iop_count(&src_a, CountKind::Ones);
    builder.output_ciphertext(res);
    builder
}

type Column = Vec<CiphertextBlock>;

impl Builder {
    fn countx_reduce_rec(&mut self, inp: impl AsRef<[Column]>, kind: CountKind) -> Vec<Column> {
        // The workhorse recursive reduction implementation.
        //
        // Works on columns. The last column contains only bits and as such can be added for longer
        // before pbs. At the first iteration, there is only a single column, with all bits
        // inside. Then as the reduction goes, carries are propagated to the column above.
        // The reduction is finished when there are only one message per column. The columns are
        // then turned to a ciphertext.
        let inp = inp.as_ref();

        if inp.iter().all(|col| col.len() <= 1) {
            // Reduction is finished, can return.
            return inp.to_vec();
        } else {
            let op_nb = NU;
            let op_nb_bool = 1 << ((op_nb as f64).log2().ceil() as usize);
            let op_nb_single = op_nb_bool - 1;
            let reduction_iteration = inp.len();

            let mut output: Vec<Column> = vec![Column::new(); inp.len() + 1];
            for (col_idx, col) in inp.iter().enumerate() {
                if col.len() == 1 {
                    output[col_idx].push(col[0]);
                } else if col_idx == inp.len() - 1 {
                    self.push_comment(format!("Last Column {col_idx}"));
                    col.iter()
                        .cloned()
                        .chunk(op_nb_single)
                        .unwrap_chunks()
                        .map(|chunk| {
                            let sum = self.vector_add_reduce(&chunk);
                            if kind == CountKind::Zeros && reduction_iteration == 1 {
                                self.with_comment("Zero and first reduction", || {
                                    match chunk.len() {
                                        1 => self.block_lookup2(sum, Lut2Def::ManyInv1CarryMsg),
                                        2 => self.block_lookup2(sum, Lut2Def::ManyInv2CarryMsg),
                                        3 => self.block_lookup2(sum, Lut2Def::ManyInv3CarryMsg),
                                        4 => self.block_lookup2(sum, Lut2Def::ManyInv4CarryMsg),
                                        5 => self.block_lookup2(sum, Lut2Def::ManyInv5CarryMsg),
                                        6 => self.block_lookup2(sum, Lut2Def::ManyInv6CarryMsg),
                                        7 => self.block_lookup2(sum, Lut2Def::ManyInv7CarryMsg),
                                        _ => unreachable!(),
                                    }
                                })
                            } else {
                                self.with_comment("Common branch", || {
                                    self.block_lookup2(sum, Lut2Def::ManyCarryMsg)
                                })
                            }
                        })
                        .for_each(|(msg, carry)| {
                            output[col_idx].push(msg);
                            output[col_idx + 1].push(carry);
                        });
                    self.pop_comment();
                } else {
                    self.push_comment(format!("Regular Column {col_idx}"));
                    col.iter()
                        .cloned()
                        .chunk(op_nb)
                        .unwrap_chunks()
                        .map(|chunk| {
                            let sum = self.vector_add_reduce(&chunk);
                            if chunk.len() <= 2 {
                                // We have enough room to use a 2lookup in this case
                                self.block_lookup2(sum, Lut2Def::ManyCarryMsg)
                            } else {
                                // We don't have enough room. We must do two pbses
                                (
                                    self.block_lookup(sum, Lut1Def::MsgOnly),
                                    self.block_lookup(sum, Lut1Def::CarryInMsg),
                                )
                            }
                        })
                        .for_each(|(msg, carry)| {
                            output[col_idx].push(msg);
                            output[col_idx + 1].push(carry);
                        });
                    self.pop_comment();
                }
            }
            self.comment("Reduce").countx_reduce_rec(output, kind)
        }
    }

    /// Counts the number of zero or one bits in an encrypted integer.
    ///
    /// The operation splits `inp` into individual bits, then performs a
    /// recursive column-based reduction that sums them with carry
    /// propagation. When `kind` is [`CountKind::Zeros`], the first
    /// reduction pass uses inverted look-up tables so that each bit
    /// contributes 1 when it is zero. The returned [`Ciphertext`] has a
    /// width of `⌈log₂(int_size + 1)⌉` bits, just enough to represent
    /// every possible count from 0 to `int_size`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder, CountKind};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let mut builder = Builder::new(spec.block_spec());
    /// # let a = builder.input_ciphertext(spec.int_size());
    /// let pop = builder.iop_count(&a, CountKind::Ones);
    /// ```
    pub fn iop_count(&mut self, inp: &Ciphertext, kind: CountKind) -> Ciphertext {
        let blocks = self.comment("Split").split_ciphertext(inp);
        let bits = self
            .comment("Split Bits")
            .vector_lookup2(blocks, Lut2Def::ManyMsgSplit)
            .into_iter()
            .flat_map(|(l, r)| [l, r].into_iter())
            .take(inp.spec().int_size() as usize)
            .collect::<Vec<_>>();
        let res: Vec<Column> = self.comment("Reduce").countx_reduce_rec(vec![bits], kind);
        let res: Vec<CiphertextBlock> = res
            .into_iter()
            .filter(|col| !col.is_empty())
            .map(|col| col[0])
            .collect();
        let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
        let n_blocks = output_size.div_ceil(self.spec().message_size() as u16) as usize;
        self.join_ciphertext(&res[..n_blocks], Some(output_size))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_count0() {
        let spec = CiphertextSpec::new(18, 2, 2);
        let ir = count_0(spec).into_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_types(false),
            r#"
                                                                          | %0 = input_ciphertext<0, 18>();
                // Split                                                  | %1 = extract_ct_block<0>(%0);
                // Split                                                  | %2 = extract_ct_block<1>(%0);
                // Split                                                  | %3 = extract_ct_block<2>(%0);
                // Split                                                  | %4 = extract_ct_block<3>(%0);
                // Split                                                  | %5 = extract_ct_block<4>(%0);
                // Split                                                  | %6 = extract_ct_block<5>(%0);
                // Split                                                  | %7 = extract_ct_block<6>(%0);
                // Split                                                  | %8 = extract_ct_block<7>(%0);
                // Split                                                  | %9 = extract_ct_block<8>(%0);
                // Split Bits                                             | %10, %11 = pbs2<ManyMsgSplit>(%1);
                // Split Bits                                             | %12, %13 = pbs2<ManyMsgSplit>(%2);
                // Split Bits                                             | %14, %15 = pbs2<ManyMsgSplit>(%3);
                // Split Bits                                             | %16, %17 = pbs2<ManyMsgSplit>(%4);
                // Split Bits                                             | %18, %19 = pbs2<ManyMsgSplit>(%5);
                // Split Bits                                             | %20, %21 = pbs2<ManyMsgSplit>(%6);
                // Split Bits                                             | %22, %23 = pbs2<ManyMsgSplit>(%7);
                // Split Bits                                             | %24, %25 = pbs2<ManyMsgSplit>(%8);
                // Split Bits                                             | %26, %27 = pbs2<ManyMsgSplit>(%9);
                // Reduce / Last Column 0                                 | %28 = add_ct(%10, %11);
                // Reduce / Last Column 0                                 | %29 = add_ct(%28, %12);
                // Reduce / Last Column 0                                 | %30 = add_ct(%29, %13);
                // Reduce / Last Column 0                                 | %31 = add_ct(%30, %14);
                // Reduce / Last Column 0                                 | %32 = add_ct(%31, %15);
                // Reduce / Last Column 0                                 | %33 = add_ct(%32, %16);
                // Reduce / Last Column 0 / Zero and first reduction      | %34, %35 = pbs2<ManyInv7CarryMsg>(%33);
                // Reduce / Last Column 0                                 | %36 = add_ct(%17, %18);
                // Reduce / Last Column 0                                 | %37 = add_ct(%36, %19);
                // Reduce / Last Column 0                                 | %38 = add_ct(%37, %20);
                // Reduce / Last Column 0                                 | %39 = add_ct(%38, %21);
                // Reduce / Last Column 0                                 | %40 = add_ct(%39, %22);
                // Reduce / Last Column 0                                 | %41 = add_ct(%40, %23);
                // Reduce / Last Column 0 / Zero and first reduction      | %42, %43 = pbs2<ManyInv7CarryMsg>(%41);
                // Reduce / Last Column 0                                 | %44 = add_ct(%24, %25);
                // Reduce / Last Column 0                                 | %45 = add_ct(%44, %26);
                // Reduce / Last Column 0                                 | %46 = add_ct(%45, %27);
                // Reduce / Last Column 0 / Zero and first reduction      | %47, %48 = pbs2<ManyInv4CarryMsg>(%46);
                // Reduce / Reduce / Regular Column 0                     | %49 = add_ct(%34, %42);
                // Reduce / Reduce / Regular Column 0                     | %50 = add_ct(%49, %47);
                // Reduce / Reduce / Regular Column 0                     | %51 = pbs<Protect, MsgOnly>(%50);
                // Reduce / Reduce / Regular Column 0                     | %52 = pbs<Protect, CarryInMsg>(%50);
                // Reduce / Reduce / Last Column 1                        | %53 = add_ct(%35, %43);
                // Reduce / Reduce / Last Column 1                        | %54 = add_ct(%53, %48);
                // Reduce / Reduce / Last Column 1 / Common branch        | %55, %56 = pbs2<ManyCarryMsg>(%54);
                // Reduce / Reduce / Reduce / Regular Column 1            | %57 = add_ct(%52, %55);
                // Reduce / Reduce / Reduce / Regular Column 1            | %58, %59 = pbs2<ManyCarryMsg>(%57);
                // Reduce / Reduce / Reduce / Reduce / Regular Column 2   | %60 = add_ct(%59, %56);
                // Reduce / Reduce / Reduce / Reduce / Regular Column 2   | %61, %62 = pbs2<ManyCarryMsg>(%60);
                                                                          | %63 = decl_ct<5>();
                                                                          | %64 = store_ct_block<0>(%51, %63);
                                                                          | %65 = store_ct_block<1>(%58, %64);
                                                                          | %66 = store_ct_block<2>(%61, %65);
                                                                          | output<0>(%66);
            "#
        );
    }

    #[test]
    fn correctness_count0() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res = inp.as_storage().count_zeros() - (u128::BITS - inp.spec().int_size() as u32);
            let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
            vec![IopValue::Ciphertext(
                inp.spec()
                    .block_spec()
                    .ciphertext_spec(output_size)
                    .from_int(res as u128),
            )]
        }

        for size in (2..128).step_by(2) {
            count_0(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_count1() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res = inp.as_storage().count_ones();
            let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
            vec![IopValue::Ciphertext(
                inp.spec()
                    .block_spec()
                    .ciphertext_spec(output_size)
                    .from_int(res as u128),
            )]
        }

        for size in (2..128).step_by(2) {
            count_1(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }
}
