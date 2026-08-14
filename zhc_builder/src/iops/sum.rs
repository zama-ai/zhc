use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

use crate::{Ciphertext, CiphertextBlock, NU, builder::Builder};

/// Which sum algorithm to emit.
/// Both sum the same columns of equal-weight blocks and wrap at the width those
/// blocks span, `message_size * block_count`, which neither algorithm checks
/// against `int_size`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SumKind {
    /// Columns reduced in rounds, then one addition resolves every carry:
    /// [`Builder::iop_sum_column_reduce`].
    #[default]
    MinLatency,
    /// Each column cleaned in turn, its carry rippling into the next:
    /// [`Builder::iop_sum_ripple_carry`].
    MinPbs,
}

/// Creates an IR summing `n` encrypted integers with the default algorithm.
/// See [`Builder::iop_sum`].
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, sum};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = sum(spec, 4);
/// let ir = builder.optimize_ir();
/// ```
pub fn sum(spec: CiphertextSpec, n: usize) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let srcs = sum_inputs(&builder, spec, n);
    let output = builder.iop_sum(&srcs, SumKind::default());
    builder.ciphertext_output(output);
    builder
}

/// Creates an IR summing `n` encrypted integers by reducing columns.
/// See [`Builder::iop_sum_column_reduce`].
pub fn sum_column_reduce(spec: CiphertextSpec, n: usize) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let srcs = sum_inputs(&builder, spec, n);
    let output = builder.iop_sum_column_reduce(&srcs);
    builder.ciphertext_output(output);
    builder
}

/// Creates an IR summing `n` encrypted integers with rippling carries.
/// See [`Builder::iop_sum_ripple_carry`].
pub fn sum_ripple_carry(spec: CiphertextSpec, n: usize) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let srcs = sum_inputs(&builder, spec, n);
    let output = builder.iop_sum_ripple_carry(&srcs);
    builder.ciphertext_output(output);
    builder
}

/// Declares the `n` operands, which the other iops spell out inline but cannot
/// here because their count is variable.
fn sum_inputs(builder: &Builder, spec: CiphertextSpec, n: usize) -> Vec<Ciphertext> {
    (0..n)
        .map(|_| builder.ciphertext_input(spec.int_size()))
        .collect()
}

impl Builder {
    /// Sums encrypted integers, wrapping at the width their blocks span.
    ///
    /// Dispatches on `kind`, which trades PBS count against latency.
    ///
    /// # Panics
    ///
    /// Panics unless there are at least three operands, all of the same spec.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder, SumKind};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// # let c = builder.ciphertext_input(spec.int_size());
    /// let total = builder.iop_sum(&[a, b, c], SumKind::MinLatency);
    /// ```
    pub fn iop_sum(&self, srcs: &[Ciphertext], kind: SumKind) -> Ciphertext {
        match kind {
            SumKind::MinLatency => self.iop_sum_column_reduce(srcs),
            SumKind::MinPbs => self.iop_sum_ripple_carry(srcs),
        }
    }

    /// Sums encrypted integers by reducing columns, then adding once.
    ///
    /// Blocks of equal weight form columns, summed for free in the carry space.
    /// While any column exceeds `NU` terms, every column is reduced:
    /// full chunks are split into a message and a carry, the carry joining the column above.
    /// Once all of them fit, one addition resolves all carries at once, which keeps logarithmic
    /// depth.
    ///
    /// # Panics
    ///
    /// Panics unless there are at least three operands, all of the same spec.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// # let c = builder.ciphertext_input(spec.int_size());
    /// let total = builder.iop_sum_column_reduce(&[a, b, c]);
    /// ```
    pub fn iop_sum_column_reduce(&self, srcs: &[Ciphertext]) -> Ciphertext {
        let spec = self.check_sum_operands(srcs);
        let block_count: usize = spec.block_count().into();
        let mut columns = self.sum_columns(srcs, block_count);
        let mut round = 0;

        // Reduction phase, governed by NU.
        while columns.iter().any(|column| column.len() > NU) {
            self.push_comment(format!("reduction_{round}"));

            // Fresh vector as carries are sent upward (position + 1)
            let mut reduced: Vec<Vec<CiphertextBlock>> = vec![Vec::new(); block_count];

            for (position, column) in columns.iter().enumerate() {
                let chunks = column.chunks_exact(NU);
                reduced[position].extend(chunks.remainder());
                for chunk in chunks {
                    let acc = self.vector_add_reduce(chunk); // one dirty block: message + carry occupied, 0 PBS
                    reduced[position].push(self.block_lookup(&acc, Lut1Def::MsgOnly)); // low digit, clean, stays at this weight
                    if position + 1 < block_count {
                        reduced[position + 1].push(self.block_lookup(&acc, Lut1Def::CarryInMsg)); // high digit, clean, re-based one weight up
                    }
                }
            }
            columns = reduced;
            self.pop_comment();
            round += 1;
        }

        // Every column now fits the carry space:
        // sum & split it into message/carry radix whose sum is the result
        self.push_comment("split");
        let mut messages = Vec::with_capacity(block_count);
        let mut carries = vec![self.block_let_ciphertext(0); block_count];

        for (position, column) in columns.iter().enumerate() {
            let acc = self.vector_add_reduce(column);
            if column.len() == 1 {
                // Already a clean digit, both lookups would be no-ops.
                messages.push(acc);
                continue;
            }
            messages.push(self.block_lookup(&acc, Lut1Def::MsgOnly));
            if position + 1 < block_count {
                carries[position + 1] = self.block_lookup(&acc, Lut1Def::CarryInMsg);
            }
        }
        self.pop_comment();

        self.comment("resolve carries").iop_add(
            &self.ciphertext_join(messages, None),
            &self.ciphertext_join(carries, None),
            None,
        )
    }

    /// Sums encrypted integers the schoolbook way, in `NU`-sized chunks.
    ///
    /// Needs no final addition, and spends fewer PBS than `iop_sum_column_reduce`,
    /// but its depth grows with the block count.
    /// Worth choosing only where the surrounding graph is already wide enough to fill PBS batches.
    ///
    /// # Panics
    ///
    /// Panics unless there are at least three operands, all of the same spec.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// # let c = builder.ciphertext_input(spec.int_size());
    /// let total = builder.iop_sum_ripple_carry(&[a, b, c]);
    /// ```
    pub fn iop_sum_ripple_carry(&self, srcs: &[Ciphertext]) -> Ciphertext {
        let spec = self.check_sum_operands(srcs);
        let block_count = spec.block_count().into();
        let mut output_blocks = Vec::with_capacity(block_count);

        // Terms awaiting reduction, indexed by block position.
        // Enumerating in blocks of each ct
        let mut terms = self.sum_columns(srcs, block_count);

        // per position reduction
        for position in 0..block_count {
            self.push_comment(format!("reduction_{position}"));
            // A carry out of the top position weighs 2^int_size, so it is never
            // even computed there, which is what wraps the sum.
            let sends_carry = position + 1 < block_count;
            let position_terms = std::mem::take(&mut terms[position]);
            let mut acc_iter = position_terms.iter();
            let mut acc = *acc_iter.next().expect("Every position holds a term.");
            let mut acc_terms = 1;
            let mut carry = Vec::new();

            for term in acc_iter {
                acc = self.block_add(&acc, term);
                acc_terms += 1;

                // Carry space is full, extract it before it overflows the padding bit.
                if acc_terms == NU {
                    if sends_carry {
                        carry.push(self.block_lookup(&acc, Lut1Def::CarryInMsg));
                    }
                    acc = self.block_lookup(&acc, Lut1Def::MsgOnly);
                    acc_terms = 1;
                }
            }

            // Outputs must be carry-clean, so a partly filled accumulator is split too.
            if acc_terms != 1 {
                if sends_carry {
                    carry.push(self.block_lookup(&acc, Lut1Def::CarryInMsg));
                }
                acc = self.block_lookup(&acc, Lut1Def::MsgOnly);
            }
            output_blocks.push(acc);

            if sends_carry {
                terms[position + 1].extend(carry);
            }
            self.pop_comment();
        }

        self.ciphertext_join(output_blocks, None)
    }

    fn check_sum_operands(&self, srcs: &[Ciphertext]) -> CiphertextSpec {
        assert!(
            srcs.len() >= 3,
            "Tried to sum fewer than three ciphertexts, use iop_add for a pair."
        );
        let spec = srcs[0].spec();
        assert!(
            srcs.iter().all(|src| src.spec() == spec),
            "Tried to sum ciphertexts of different specs."
        );
        spec
    }

    /// Groups the blocks of every operand by weight.
    fn sum_columns(&self, srcs: &[Ciphertext], block_count: usize) -> Vec<Vec<CiphertextBlock>> {
        let mut columns = vec![Vec::new(); block_count];
        for src in srcs {
            for (position, block) in self.ciphertext_split(src).into_iter().enumerate() {
                columns[position].push(block);
            }
        }
        columns
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    fn expected_sum(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let mut terms = inp.iter().map(|v| match v {
            IopValue::Ciphertext(ct) => ct,
            _ => unreachable!(),
        });
        let first = terms.next().unwrap();
        let spec = first.spec();
        let total = terms.fold(first.as_storage(), |acc, ct| {
            acc.wrapping_add(ct.as_storage())
        });
        Some(vec![IopValue::Ciphertext(
            spec.from_int(total & spec.int_mask()),
        )])
    }

    /// `test_random` masks values to a random bit window, so it never saturates the carry space.
    /// This pins the worst case down explicitly.
    fn check_all_max_blocks(builder: Builder, spec: CiphertextSpec, n: usize) {
        let inputs = vec![IopValue::Ciphertext(spec.from_int(spec.int_mask())); n];
        let outputs = builder.interpret().with_inputs(&inputs).get_outputs();
        assert_eq!(outputs, expected_sum(&inputs).unwrap(), "{n} terms");
    }

    fn check_sizes_and_term_counts(factory: impl Fn(CiphertextSpec, usize) -> Builder) {
        for size in [2, 4, 8, 16, 32] {
            for n in [3, 5, 6, 9, 11, 13, 17, 26] {
                let spec = CiphertextSpec::new(size, 2, 2);
                factory(spec, n).test_random(20, expected_sum);
                check_all_max_blocks(factory(spec, n), spec, n);
            }
        }
    }

    #[test]
    fn test_sum_column_reduce() {
        let ir = sum_column_reduce(CiphertextSpec::new(4, 2, 2), 3).optimize_ir();
        assert_display_is!(
            ir.format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 4>();
                %1 = input_ciphertext<1, 4>();
                %2 = input_ciphertext<2, 4>();
                %3 = extract_ct_block<0>(%0);
                %4 = extract_ct_block<1>(%0);
                %5 = extract_ct_block<0>(%1);
                %6 = extract_ct_block<1>(%1);
                %7 = extract_ct_block<0>(%2);
                %8 = extract_ct_block<1>(%2);
                %9 = let_ct_block<0>();
                %10 = add_ct(%3, %5);
                %11 = add_ct(%10, %7);
                %12 = pbs<Protect, Lut1("MsgOnly")>(%11);
                %13 = pbs<Protect, Lut1("CarryInMsg")>(%11);
                %14 = add_ct(%4, %6);
                %15 = add_ct(%14, %8);
                %16 = pbs<Protect, Lut1("MsgOnly")>(%15);
                %34 = add_ct(%12, %9);
                %35 = add_ct(%34, %9);
                %36, %37 = pbs2<Protect, Lut2("ManyCarryMsg")>(%35);
                %38 = add_ct(%16, %13);
                %39 = add_ct(%38, %37);
                %40, %41 = pbs2<Protect, Lut2("ManyCarryMsg")>(%39);
                %42 = decl_ct<4>();
                %46 = store_ct_block<0>(%36, %42);
                %47 = store_ct_block<1>(%40, %46);
                output<0>(%47);
            "#
        );
    }

    #[test]
    #[should_panic(expected = "fewer than three")]
    fn sum_rejects_a_pair() {
        sum(CiphertextSpec::new(4, 2, 2), 2);
    }

    #[test]
    fn test_sum_ripple_carry() {
        let ir = sum_ripple_carry(CiphertextSpec::new(4, 2, 2), 3).optimize_ir();
        assert_display_is!(
            ir.format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 4>();
                %1 = input_ciphertext<1, 4>();
                %2 = input_ciphertext<2, 4>();
                %3 = extract_ct_block<0>(%0);
                %4 = extract_ct_block<1>(%0);
                %5 = extract_ct_block<0>(%1);
                %6 = extract_ct_block<1>(%1);
                %7 = extract_ct_block<0>(%2);
                %8 = extract_ct_block<1>(%2);
                %9 = add_ct(%3, %5);
                %10 = add_ct(%9, %7);
                %11 = pbs<Protect, Lut1("CarryInMsg")>(%10);
                %12 = pbs<Protect, Lut1("MsgOnly")>(%10);
                %13 = add_ct(%4, %6);
                %14 = add_ct(%13, %8);
                %15 = add_ct(%14, %11);
                %16 = pbs<Protect, Lut1("MsgOnly")>(%15);
                %17 = decl_ct<4>();
                %21 = store_ct_block<0>(%12, %17);
                %22 = store_ct_block<1>(%16, %21);
                output<0>(%22);
            "#
        );
    }

    #[test]
    fn correctness_sum_column_reduce() {
        check_sizes_and_term_counts(sum_column_reduce);
    }

    #[test]
    fn correctness_sum_ripple_carry() {
        check_sizes_and_term_counts(sum_ripple_carry);
    }
}
