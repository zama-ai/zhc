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
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Greater);
    builder.ciphertext_output(output);
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
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::GreaterOrEqual);
    builder.ciphertext_output(output);
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
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Lower);
    builder.ciphertext_output(output);
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
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::LowerOrEqual);
    builder.ciphertext_output(output);
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
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Equal);
    builder.ciphertext_output(output);
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
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::NotEqual);
    builder.ciphertext_output(output);
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
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let is_eq = builder.iop_cmp(&a, &b, CmpKind::Equal);
    /// ```
    pub fn iop_cmp(&self, src_a: &Ciphertext, src_b: &Ciphertext, kind: CmpKind) -> Ciphertext {
        // get input as array of blk
        let src_a_blocks = self.ciphertext_split(&src_a);
        let src_b_blocks = self.ciphertext_split(&src_b);

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

        self.ciphertext_join([cmp_res], None)
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
            ir.into_ir()
                .format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 16>();
                %1 = input_ciphertext<1, 16>();
                %2 = extract_ct_block<0>(%0);
                %3 = extract_ct_block<1>(%0);
                %4 = extract_ct_block<2>(%0);
                %5 = extract_ct_block<3>(%0);
                %6 = extract_ct_block<4>(%0);
                %7 = extract_ct_block<5>(%0);
                %8 = extract_ct_block<6>(%0);
                %9 = extract_ct_block<7>(%0);
                %10 = extract_ct_block<0>(%1);
                %11 = extract_ct_block<1>(%1);
                %12 = extract_ct_block<2>(%1);
                %13 = extract_ct_block<3>(%1);
                %14 = extract_ct_block<4>(%1);
                %15 = extract_ct_block<5>(%1);
                %16 = extract_ct_block<6>(%1);
                %17 = extract_ct_block<7>(%1);
                %18 = pack_ct<4>(%3, %2);
                %19 = pbs<Protect, Lut1("None")>(%18);
                %20 = pack_ct<4>(%5, %4);
                %21 = pbs<Protect, Lut1("None")>(%20);
                %22 = pack_ct<4>(%7, %6);
                %23 = pbs<Protect, Lut1("None")>(%22);
                %24 = pack_ct<4>(%9, %8);
                %25 = pbs<Protect, Lut1("None")>(%24);
                %26 = pack_ct<4>(%11, %10);
                %27 = pbs<Protect, Lut1("None")>(%26);
                %28 = pack_ct<4>(%13, %12);
                %29 = pbs<Protect, Lut1("None")>(%28);
                %30 = pack_ct<4>(%15, %14);
                %31 = pbs<Protect, Lut1("None")>(%30);
                %32 = pack_ct<4>(%17, %16);
                %33 = pbs<Protect, Lut1("None")>(%32);
                %34 = sub_ct(%19, %27);
                %35 = pbs<Protect, Lut1("CmpSign")>(%34);
                %36 = let_pt_block<1>();
                %37 = add_pt(%35, %36);
                %38 = sub_ct(%21, %29);
                %39 = pbs<Protect, Lut1("CmpSign")>(%38);
                %41 = add_pt(%39, %36);
                %42 = sub_ct(%23, %31);
                %43 = pbs<Protect, Lut1("CmpSign")>(%42);
                %45 = add_pt(%43, %36);
                %46 = sub_ct(%25, %33);
                %47 = pbs<Protect, Lut1("CmpSign")>(%46);
                %49 = add_pt(%47, %36);
                %50 = pack_ct<4>(%41, %37);
                %51 = pack_ct<4>(%49, %45);
                %52 = pbs<Protect, Lut1("CmpReduce")>(%50);
                %53 = pbs<Protect, Lut1("CmpReduce")>(%51);
                %54 = pack_ct<4>(%53, %52);
                %55 = pbs<Protect, Lut1("CmpEqMrg")>(%54);
                %56 = decl_ct<2>();
                %57 = store_ct_block<0>(%55, %56);
                output<0>(%57);
            "#
        );
    }

    #[test]
    fn bitwise_or_16() {
        let spec = zhc_crypto::integer_semantics::CiphertextBlockSpec(2, 2);
        let builder = crate::Builder::new(spec);
        let lhs = builder.ciphertext_input(4);
        let rhs = builder.ciphertext_input(4);
        println!("{rhs:?}");
        let lhs_blocks = builder.ciphertext_split(lhs);
        let rhs_blocks = builder.ciphertext_split(rhs);
        let res = builder.vector_zip_then_lookup(
            lhs_blocks,
            rhs_blocks,
            zhc_langs::ioplang::Lut1Def::BwOr,
            crate::ExtensionBehavior::Panic,
        );
        let res = builder.ciphertext_join(res, None);
        builder.ciphertext_output(res);
        builder.draw("testttttt.html");
    }
}
