use crate::{Ciphertext, builder::Builder};
use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::iter::{CollectInSmallVec, MultiZip};

/// Creates an IR for a *greater-than* comparison of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one
/// single-block ciphertext output encoding the boolean result (1 when the
/// first operand is strictly greater than the second, 0 otherwise).
/// Internally delegates to [`Builder::iop_cmp`] with [`CmpKind::Greater`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, cmp_gt};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = cmp_gt(spec);
/// let ir = builder.into_ir();
/// ```
pub fn cmp_gt(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Greater);
    builder.output_ciphertext(output);
    builder
}

/// Creates an IR for a *greater-or-equal* comparison of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one
/// single-block ciphertext output encoding the boolean result (1 when the
/// first operand is greater than or equal to the second, 0 otherwise).
/// Internally delegates to [`Builder::iop_cmp`] with
/// [`CmpKind::GreaterOrEqual`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, cmp_gte};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = cmp_gte(spec);
/// let ir = builder.into_ir();
/// ```
pub fn cmp_gte(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::GreaterOrEqual);
    builder.output_ciphertext(output);
    builder
}

/// Creates an IR for a *less-than* comparison of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one
/// single-block ciphertext output encoding the boolean result (1 when the
/// first operand is strictly less than the second, 0 otherwise).
/// Internally delegates to [`Builder::iop_cmp`] with [`CmpKind::Lower`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, cmp_lt};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = cmp_lt(spec);
/// let ir = builder.into_ir();
/// ```
pub fn cmp_lt(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Lower);
    builder.output_ciphertext(output);
    builder
}

/// Creates an IR for a *less-or-equal* comparison of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one
/// single-block ciphertext output encoding the boolean result (1 when the
/// first operand is less than or equal to the second, 0 otherwise).
/// Internally delegates to [`Builder::iop_cmp`] with
/// [`CmpKind::LowerOrEqual`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, cmp_lte};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = cmp_lte(spec);
/// let ir = builder.into_ir();
/// ```
pub fn cmp_lte(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::LowerOrEqual);
    builder.output_ciphertext(output);
    builder
}

/// Creates an IR for an *equality* comparison of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one
/// single-block ciphertext output encoding the boolean result (1 when the
/// two operands are equal, 0 otherwise). Internally delegates to
/// [`Builder::iop_cmp`] with [`CmpKind::Equal`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, cmp_eq};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = cmp_eq(spec);
/// let ir = builder.into_ir();
/// ```
pub fn cmp_eq(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Equal);
    builder.output_ciphertext(output);
    builder
}

/// Creates an IR for an *inequality* comparison of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one
/// single-block ciphertext output encoding the boolean result (1 when the
/// two operands differ, 0 otherwise). Internally delegates to
/// [`Builder::iop_cmp`] with [`CmpKind::NotEqual`].
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, cmp_neq};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = cmp_neq(spec);
/// let ir = builder.into_ir();
/// ```
pub fn cmp_neq(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let src_b = builder.input_ciphertext(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::NotEqual);
    builder.output_ciphertext(output);
    builder
}

/// The comparison relation to evaluate between two encrypted integers.
///
/// Each variant selects the appropriate look-up tables used during the
/// tree-based reduction in [`Builder::iop_cmp`].
pub enum CmpKind {
    /// Strictly greater than (`>`).
    Greater,
    /// Greater than or equal to (`>=`).
    GreaterOrEqual,
    /// Strictly less than (`<`).
    Lower,
    /// Less than or equal to (`<=`).
    LowerOrEqual,
    /// Equal (`==`).
    Equal,
    /// Not equal (`!=`).
    NotEqual,
}

impl CmpKind {
    fn merge(&self) -> Lut1Def {
        match self {
            CmpKind::Greater => Lut1Def::CmpGtMrg,
            CmpKind::GreaterOrEqual => Lut1Def::CmpGteMrg,
            CmpKind::Lower => Lut1Def::CmpLtMrg,
            CmpKind::LowerOrEqual => Lut1Def::CmpLteMrg,
            CmpKind::Equal => Lut1Def::CmpEqMrg,
            CmpKind::NotEqual => Lut1Def::CmpNeqMrg,
        }
    }

    fn compare(&self) -> Lut1Def {
        match self {
            CmpKind::Greater => Lut1Def::CmpGt,
            CmpKind::GreaterOrEqual => Lut1Def::CmpGte,
            CmpKind::Lower => Lut1Def::CmpLt,
            CmpKind::LowerOrEqual => Lut1Def::CmpLte,
            CmpKind::Equal => Lut1Def::CmpEq,
            CmpKind::NotEqual => Lut1Def::CmpNeq,
        }
    }
}

impl Builder {
    /// Compares two encrypted integers under the given relation.
    ///
    /// The comparison proceeds in three phases: block-wise subtraction with
    /// sign extraction, tree-based reduction of the per-block results, and a
    /// final merge PBS that produces the boolean output. The `kind` parameter
    /// selects which relation is evaluated (see [`CmpKind`]).
    ///
    /// Both `src_a` and `src_b` must have the same block decomposition. The
    /// returned [`Ciphertext`] is a single-block integer encoding the boolean
    /// result: 1 when the relation holds, 0 otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder, CmpKind};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.input_ciphertext(spec.int_size());
    /// # let b = builder.input_ciphertext(spec.int_size());
    /// let is_eq = builder.iop_cmp(&a, &b, CmpKind::Equal);
    /// ```
    pub fn iop_cmp(&self, src_a: &Ciphertext, src_b: &Ciphertext, kind: CmpKind) -> Ciphertext {
        // get input as array of blk
        let src_a_blocks = self.split_ciphertext(&src_a);
        let src_b_blocks = self.split_ciphertext(&src_b);

        // pack cts
        let packed_a = self.comment("Pack A").vector_pack_then_clean(src_a_blocks);
        let packed_b = self.comment("Pack B").vector_pack_then_clean(src_b_blocks);

        // merge a /b and get sign
        self.push_comment("Compare blocks");
        let mut merged = (packed_a.iter(), packed_b.iter())
            .mzip()
            .enumerate()
            .map(|(i, (l, r))| {
                self.with_comment(format!("{i}-th"), || {
                    let sub_lr = self.block_sub(l, r);
                    let pbsed = self.block_lookup(&sub_lr, Lut1Def::CmpSign);
                    let cst = self.block_let_plaintext(1);
                    self.block_add_plaintext(&pbsed, &cst)
                })
            })
            .cosvec();
        self.pop_comment();

        // reduce (tree-based reduce)
        self.push_comment("Reduce comparison");
        while merged.len() > 2 {
            let packed = self.vector_pack(merged.as_slice());
            let reduced = packed
                .iter()
                .map(|x| self.block_lookup(x, Lut1Def::CmpReduce))
                .cosvec();
            // prepare next iter
            merged = reduced;
        }
        self.pop_comment();

        // last reduce and cast based on user required cmp
        let cmp_res = match merged.len() {
            2 => {
                let p = self.vector_pack(merged.as_slice());
                self.block_lookup(&p[0], kind.merge())
            }
            1 => self.block_lookup(&merged[0], kind.compare()),
            _ => unreachable!(),
        };

        self.join_ciphertext([cmp_res], None)
    }
}

#[cfg(test)]
mod test {
    use zhc_crypto::integer_semantics::CiphertextSpec;
    use zhc_utils::assert_display_is;

    use super::cmp_eq;

    #[test]
    fn test_cmp() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = cmp_eq(spec);
        assert_display_is!(
            ir.into_ir().format().show_comments(true).show_opid(true),
            r#"
                @00                              | %0 : Ct = input_ciphertext<0, 16>();
                @01                              | %1 : Ct = input_ciphertext<1, 16>();
                @36   // Compare blocks / 0-th   | %36 : PtBlock = let_pt_block<1>();
                @56                              | %56 : Ct = decl_ct<2>();
                @02                              | %2 : CtBlock = extract_ct_block<0>(%0 : Ct);
                @03                              | %3 : CtBlock = extract_ct_block<1>(%0 : Ct);
                @04                              | %4 : CtBlock = extract_ct_block<2>(%0 : Ct);
                @05                              | %5 : CtBlock = extract_ct_block<3>(%0 : Ct);
                @06                              | %6 : CtBlock = extract_ct_block<4>(%0 : Ct);
                @07                              | %7 : CtBlock = extract_ct_block<5>(%0 : Ct);
                @08                              | %8 : CtBlock = extract_ct_block<6>(%0 : Ct);
                @09                              | %9 : CtBlock = extract_ct_block<7>(%0 : Ct);
                @10                              | %10 : CtBlock = extract_ct_block<0>(%1 : Ct);
                @11                              | %11 : CtBlock = extract_ct_block<1>(%1 : Ct);
                @12                              | %12 : CtBlock = extract_ct_block<2>(%1 : Ct);
                @13                              | %13 : CtBlock = extract_ct_block<3>(%1 : Ct);
                @14                              | %14 : CtBlock = extract_ct_block<4>(%1 : Ct);
                @15                              | %15 : CtBlock = extract_ct_block<5>(%1 : Ct);
                @16                              | %16 : CtBlock = extract_ct_block<6>(%1 : Ct);
                @17                              | %17 : CtBlock = extract_ct_block<7>(%1 : Ct);
                @18   // Pack A                  | %18 : CtBlock = pack_ct<4>(%3 : CtBlock, %2 : CtBlock);
                @20   // Pack A                  | %20 : CtBlock = pack_ct<4>(%5 : CtBlock, %4 : CtBlock);
                @22   // Pack A                  | %22 : CtBlock = pack_ct<4>(%7 : CtBlock, %6 : CtBlock);
                @24   // Pack A                  | %24 : CtBlock = pack_ct<4>(%9 : CtBlock, %8 : CtBlock);
                @26   // Pack B                  | %26 : CtBlock = pack_ct<4>(%11 : CtBlock, %10 : CtBlock);
                @28   // Pack B                  | %28 : CtBlock = pack_ct<4>(%13 : CtBlock, %12 : CtBlock);
                @30   // Pack B                  | %30 : CtBlock = pack_ct<4>(%15 : CtBlock, %14 : CtBlock);
                @32   // Pack B                  | %32 : CtBlock = pack_ct<4>(%17 : CtBlock, %16 : CtBlock);
                @19   // Pack A                  | %19 : CtBlock = pbs<Protect, None>(%18 : CtBlock);
                @21   // Pack A                  | %21 : CtBlock = pbs<Protect, None>(%20 : CtBlock);
                @23   // Pack A                  | %23 : CtBlock = pbs<Protect, None>(%22 : CtBlock);
                @25   // Pack A                  | %25 : CtBlock = pbs<Protect, None>(%24 : CtBlock);
                @27   // Pack B                  | %27 : CtBlock = pbs<Protect, None>(%26 : CtBlock);
                @29   // Pack B                  | %29 : CtBlock = pbs<Protect, None>(%28 : CtBlock);
                @31   // Pack B                  | %31 : CtBlock = pbs<Protect, None>(%30 : CtBlock);
                @33   // Pack B                  | %33 : CtBlock = pbs<Protect, None>(%32 : CtBlock);
                @34   // Compare blocks / 0-th   | %34 : CtBlock = sub_ct(%19 : CtBlock, %27 : CtBlock);
                @38   // Compare blocks / 1-th   | %38 : CtBlock = sub_ct(%21 : CtBlock, %29 : CtBlock);
                @42   // Compare blocks / 2-th   | %42 : CtBlock = sub_ct(%23 : CtBlock, %31 : CtBlock);
                @46   // Compare blocks / 3-th   | %46 : CtBlock = sub_ct(%25 : CtBlock, %33 : CtBlock);
                @35   // Compare blocks / 0-th   | %35 : CtBlock = pbs<Protect, CmpSign>(%34 : CtBlock);
                @39   // Compare blocks / 1-th   | %39 : CtBlock = pbs<Protect, CmpSign>(%38 : CtBlock);
                @43   // Compare blocks / 2-th   | %43 : CtBlock = pbs<Protect, CmpSign>(%42 : CtBlock);
                @47   // Compare blocks / 3-th   | %47 : CtBlock = pbs<Protect, CmpSign>(%46 : CtBlock);
                @37   // Compare blocks / 0-th   | %37 : CtBlock = add_pt(%35 : CtBlock, %36 : PtBlock);
                @41   // Compare blocks / 1-th   | %41 : CtBlock = add_pt(%39 : CtBlock, %36 : PtBlock);
                @45   // Compare blocks / 2-th   | %45 : CtBlock = add_pt(%43 : CtBlock, %36 : PtBlock);
                @49   // Compare blocks / 3-th   | %49 : CtBlock = add_pt(%47 : CtBlock, %36 : PtBlock);
                @50   // Reduce comparison       | %50 : CtBlock = pack_ct<4>(%41 : CtBlock, %37 : CtBlock);
                @51   // Reduce comparison       | %51 : CtBlock = pack_ct<4>(%49 : CtBlock, %45 : CtBlock);
                @52   // Reduce comparison       | %52 : CtBlock = pbs<Protect, CmpReduce>(%50 : CtBlock);
                @53   // Reduce comparison       | %53 : CtBlock = pbs<Protect, CmpReduce>(%51 : CtBlock);
                @54                              | %54 : CtBlock = pack_ct<4>(%53 : CtBlock, %52 : CtBlock);
                @55                              | %55 : CtBlock = pbs<Protect, CmpEqMrg>(%54 : CtBlock);
                @57                              | %57 : Ct = store_ct_block<0>(%55 : CtBlock, %56 : Ct);
                @58                              | output<0>(%57 : Ct);
            "#
        );
    }
}
