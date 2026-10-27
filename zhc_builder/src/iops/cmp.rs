use crate::{CiphertextBool, CiphertextInteger, NU_BOOL, builder::Builder};
use zhc_crypto::integer_semantics::{SpecBlock, SpecInteger};
use zhc_langs::ioplang::Lut1;
use zhc_utils::iter::MultiZip;

/// Creates an IR for greater-than comparison of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_cmp`] with [`CmpKind::Greater`].
/// See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{SpecInteger, cmp_gt};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = cmp_gt(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn cmp_gt(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Greater);
    builder.ciphertext_bool_output(output);
    builder
}

/// Creates an IR for greater-or-equal comparison of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_cmp`] with [`CmpKind::GreaterOrEqual`].
/// See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{SpecInteger, cmp_gte};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = cmp_gte(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn cmp_gte(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::GreaterOrEqual);
    builder.ciphertext_bool_output(output);
    builder
}

/// Creates an IR for less-than comparison of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_cmp`] with [`CmpKind::Lower`].
/// See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{SpecInteger, cmp_lt};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = cmp_lt(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn cmp_lt(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Lower);
    builder.ciphertext_bool_output(output);
    builder
}

/// Creates an IR for less-or-equal comparison of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_cmp`] with [`CmpKind::LowerOrEqual`].
/// See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{SpecInteger, cmp_lte};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = cmp_lte(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn cmp_lte(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    let output = builder.iop_cmp(&src_a, &src_b, CmpKind::LowerOrEqual);
    builder.ciphertext_bool_output(output);
    builder
}

/// Creates an IR for equality comparison of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_cmp`] with [`CmpKind::Equal`].
/// See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{SpecInteger, cmp_eq};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = cmp_eq(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn cmp_eq(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    // let output = builder.iop_cmp(&src_a, &src_b, CmpKind::Equal);
    let output = builder.iop_eq(&src_a, &src_b);
    builder.ciphertext_bool_output(output);
    builder
}

/// Creates an IR for inequality comparison of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_cmp`] with [`CmpKind::NotEqual`].
/// See that method for algorithm details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{SpecInteger, cmp_neq};
/// # let spec = SpecInteger::new(16, 2, 2);
/// let builder = cmp_neq(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn cmp_neq(spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_integer_input(spec.int_size());
    let src_b = builder.ciphertext_integer_input(spec.int_size());
    let output = builder.iop_neq(&src_a, &src_b);
    builder.ciphertext_bool_output(output);
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
    fn merge(&self, spec: SpecBlock) -> Lut1 {
        match self {
            CmpKind::Greater => Lut1::cmp_gt_mrg(spec),
            CmpKind::GreaterOrEqual => Lut1::cmp_gte_mrg(spec),
            CmpKind::Lower => Lut1::cmp_lt_mrg(spec),
            CmpKind::LowerOrEqual => Lut1::cmp_lte_mrg(spec),
            CmpKind::Equal => Lut1::cmp_eq_mrg(spec),
            CmpKind::NotEqual => Lut1::cmp_neq_mrg(spec),
        }
    }

    fn compare(&self, spec: SpecBlock) -> Lut1 {
        match self {
            CmpKind::Greater => Lut1::cmp_gt(spec),
            CmpKind::GreaterOrEqual => Lut1::cmp_gte(spec),
            CmpKind::Lower => Lut1::cmp_lt(spec),
            CmpKind::LowerOrEqual => Lut1::cmp_lte(spec),
            CmpKind::Equal => Lut1::cmp_eq(spec),
            CmpKind::NotEqual => Lut1::cmp_neq(spec),
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
    /// returned [`CiphertextBool`] encodes `true` when the relation holds and `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{SpecInteger, Builder, CmpKind};
    /// # let spec = SpecInteger::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_integer_input(spec.int_size());
    /// # let b = builder.ciphertext_integer_input(spec.int_size());
    /// let is_eq = builder.iop_cmp(&a, &b, CmpKind::Equal);
    /// ```
    pub fn iop_cmp(
        &self,
        src_a: &CiphertextInteger,
        src_b: &CiphertextInteger,
        kind: CmpKind,
    ) -> CiphertextBool {
        // get input as array of blk
        let src_a_blocks = self.ciphertext_integer_split(&src_a);
        let src_b_blocks = self.ciphertext_integer_split(&src_b);

        // pack cts
        let packed_a = self.comment("Pack A").vector_pack(src_a_blocks);
        let packed_b = self.comment("Pack B").vector_pack(src_b_blocks);

        // merge a /b and get sign
        self.push_comment("Compare blocks");
        let mut merged = (packed_a.iter(), packed_b.iter())
            .mzip()
            .enumerate()
            .map(|(i, (l, r))| {
                self.with_comment(format!("{i}-th"), || {
                    let sub_lr = self.block_wrapping_sub(l, r);
                    let pbsed = self.block_wrapping_lookup(&sub_lr, Lut1::cmp_sign(*self.spec()));
                    let cst = self.block_let_plaintext(1);
                    self.block_wrapping_add_plaintext(&pbsed, &cst)
                })
            })
            .collect::<Vec<_>>();
        self.pop_comment();

        // reduce (tree-based reduce)
        self.push_comment("Reduce comparison");
        while merged.len() > 2 {
            // use vector_pack_then_lookup instead of vector_pack to handle odd number
            // of elements and not apply Reduce to a single element out of CmpSign
            merged =
                self.vector_pack_then_lookup(merged.as_slice(), Lut1::cmp_reduce(*self.spec()));
        }
        self.pop_comment();

        // last reduce and cast based on user required cmp
        let cmp_res = match merged.len() {
            2 => {
                let p = self.vector_pack(merged.as_slice());
                self.block_lookup(&p[0], kind.merge(*self.spec()))
            }
            1 => self.block_lookup(&merged[0], kind.compare(*self.spec())),
            _ => unreachable!(),
        };

        self.ciphertext_bool_store_block(cmp_res)
    }

    pub fn iop_eq(&self, src_a: &CiphertextInteger, src_b: &CiphertextInteger) -> CiphertextBool {
        self.iop_eq_neq(src_a, src_b, false)
    }

    pub fn iop_neq(&self, src_a: &CiphertextInteger, src_b: &CiphertextInteger) -> CiphertextBool {
        self.iop_eq_neq(src_a, src_b, true)
    }

    /// Equality (`neq == false`) or inequality (`neq == true`) of two integers.
    fn iop_eq_neq(
        &self,
        src_a: &CiphertextInteger,
        src_b: &CiphertextInteger,
        neq: bool,
    ) -> CiphertextBool {
        // get input as array of blk
        let src_a_blocks = self.ciphertext_integer_split(&src_a);
        let src_b_blocks = self.ciphertext_integer_split(&src_b);

        let a_packed = self.vector_pack(&src_a_blocks);
        let b_packed = self.vector_pack(&src_b_blocks);

        // Here we have half number of blocks.
        // a - b is in [-15, 15]: wrapping keeps it within the block, and it is 0 iff a == b.
        let diff: Vec<_> = a_packed
            .iter()
            .zip(b_packed.iter())
            .map(|(a, b)| self.block_wrapping_sub(a, b))
            .collect();

        let eq_mh: Vec<_> = diff
            .iter()
            .map(|d| self.block_wrapping_lookup(d, Lut1::is_null(*self.spec())))
            .collect();

        // Split in groups of NU_BOOL
        let op_nb_bool = NU_BOOL;
        let mut res_v = eq_mh.clone();
        let mut blk_nb = res_v.len();

        // For neq, the last reduction step uses NotNull instead of IsNull. With a single
        // packed block, one reduction step is still run to apply it.
        let mut neq_pending = neq;

        while blk_nb > 1 || neq_pending {
            let mut v = Vec::new();
            let last_step = blk_nb <= op_nb_bool;
            let lut = if neq && last_step {
                neq_pending = false;
                Lut1::not_null(*self.spec())
            } else {
                Lut1::is_null(*self.spec())
            };

            for chk in res_v.chunks(op_nb_bool) {
                // Sum the booleans of the group
                let acc = self.vector_add_reduce_tree(chk);

                // len - sum is in [0, len], and is 0 iff every block of the group is equal.
                // It is never negative, so the padding bit stays clear and NotNull is valid.
                let k = self.block_let_plaintext(chk.len() as u8);
                let acc = self.block_plaintext_sub(&k, &acc);
                v.push(self.block_lookup(&acc, lut.clone()));
            }

            res_v = v.clone();
            blk_nb = res_v.len();
        }
        self.ciphertext_bool_store_block(&res_v[0])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::SpecInteger;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    use super::cmp_eq;

    #[test]
    fn test_cmp() {
        let spec = SpecInteger::new(16, 2, 2);
        let ir = cmp_eq(spec);
        assert_display_is!(
            ir.optimize_ir()
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
                %19 = pack_ct<4>(%5, %4);
                %20 = pack_ct<4>(%7, %6);
                %21 = pack_ct<4>(%9, %8);
                %22 = pack_ct<4>(%11, %10);
                %23 = pack_ct<4>(%13, %12);
                %24 = pack_ct<4>(%15, %14);
                %25 = pack_ct<4>(%17, %16);
                %26 = wrapping_sub_ct(%18, %22);
                %27 = wrapping_sub_ct(%19, %23);
                %28 = wrapping_sub_ct(%20, %24);
                %29 = wrapping_sub_ct(%21, %25);
                %30 = pbs<Permissive, Lut1("IsNull")>(%26);
                %31 = pbs<Permissive, Lut1("IsNull")>(%27);
                %32 = pbs<Permissive, Lut1("IsNull")>(%28);
                %33 = pbs<Permissive, Lut1("IsNull")>(%29);
                %34 = add_ct(%30, %31);
                %35 = add_ct(%34, %32);
                %36 = add_ct(%35, %33);
                %37 = let_pt_block<4>();
                %38 = pt_sub(%37, %36);
                %39 = pbs<Protect, Lut1("IsNull")>(%38);
                %40 = store_ctbool_block(%39);
                output_bool<0>(%40);
            "#
        );
    }

    #[test]
    fn bitwise_or_16() {
        let spec = zhc_crypto::integer_semantics::SpecBlock(2, 2);
        let builder = crate::Builder::new(spec);
        let lhs = builder.ciphertext_integer_input(4);
        let rhs = builder.ciphertext_integer_input(4);
        println!("{rhs:?}");
        let lhs_blocks = builder.ciphertext_integer_split(lhs);
        let rhs_blocks = builder.ciphertext_integer_split(rhs);
        let res = builder.vector_zip_then_lookup(
            lhs_blocks,
            rhs_blocks,
            zhc_langs::ioplang::Lut1::bw_or(spec),
            crate::ExtensionBehavior::Panic,
        );
        let res = builder.ciphertext_integer_join(res, None);
        builder.ciphertext_integer_output(res);
        builder.draw(crate::IrKind::Original);
    }

    #[test]
    fn correctness_cmp_gt() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            let res = lhs.cgt(*rhs);
            Some(vec![IopValue::CiphertextBool(res)])
        }
        for size in (2..128).step_by(2) {
            cmp_gt(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_cmp_gte() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            let res = lhs.cgte(*rhs);
            Some(vec![IopValue::CiphertextBool(res)])
        }
        for size in (2..128).step_by(2) {
            cmp_gte(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_cmp_lt() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            let res = lhs.clt(*rhs);
            Some(vec![IopValue::CiphertextBool(res)])
        }
        for size in (2..128).step_by(2) {
            cmp_lt(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_cmp_lte() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            let res = lhs.clte(*rhs);
            Some(vec![IopValue::CiphertextBool(res)])
        }
        for size in (2..128).step_by(2) {
            cmp_lte(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_cmp_eq() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            let res = lhs.equal(*rhs);
            Some(vec![IopValue::CiphertextBool(res)])
        }
        for size in (2..128).step_by(2) {
            cmp_eq(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_cmp_neq() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [
                IopValue::CiphertextInteger(lhs),
                IopValue::CiphertextInteger(rhs),
            ] = inp
            else {
                unreachable!()
            };
            let res = lhs.not_equal(*rhs);
            Some(vec![IopValue::CiphertextBool(res)])
        }
        for size in (2..128).step_by(2) {
            cmp_neq(SpecInteger::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn noise_cmp_gt() {
        for size in (2..128).step_by(2) {
            cmp_gt(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_cmp_gte() {
        for size in (2..128).step_by(2) {
            cmp_gte(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_cmp_lt() {
        for size in (2..128).step_by(2) {
            cmp_lt(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_cmp_lte() {
        for size in (2..128).step_by(2) {
            cmp_lte(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_cmp_eq() {
        for size in (2..128).step_by(2) {
            cmp_eq(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }

    #[test]
    fn noise_cmp_neq() {
        for size in (2..128).step_by(2) {
            cmp_neq(SpecInteger::new(size, 2, 2)).check_noise();
        }
    }
}
