use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

use crate::{Ciphertext, CiphertextBlock, builder::Builder};

/// Creates an IR mapping an encrypted integer through a clear table.
/// See [`Builder::iop_match_value`].
pub fn match_value(spec: CiphertextSpec, table: &[(u128, u128)], out_size: u16) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let (out, flag) = builder.iop_match_value(&src, table, out_size);
    builder.ciphertext_output(out);
    builder.ciphertext_output(flag);
    builder
}

impl Builder {
    /// Maps encrypted integer through a table of (input, output) pairs, returns (output, matched);
    /// an unmatched input yields (0,0). Same contract as tfhe-rs `match_value_parallelized`,
    /// different construction: tfhe-rs one-hot selects over the table entries, cost scaling
    /// with the entry count; this muxes over the input space, cost scaling with the input
    /// width regardless of table size (~200 PBS for the AES S-box).
    ///
    /// The input is split into a packed 4-bit low part and the remaining high bits.
    /// A PBS can only see the low part, so for each possible high value `h`, dedicated table-
    /// derived LUT maps the low part to the table output as if the high bits were `h`.
    /// This yields one candidate result per `h`; a mux tree driven by the encrypted high bits
    /// then selects the candidate matching the actual high value.
    ///
    /// NOTE: candidates grow as 4^(blocks - 2), so inputs are capped at 8 bits;
    /// the AES S-box is the only consumer today. Emits `Lut1Def::Table` LUTs;
    /// lowering allocates gids for them and the pipeline exposes their payload
    /// via `get_hpu_lut_payload`, but uploading it to the device is on the caller.
    ///
    /// # Panics
    ///
    /// - Panics if the input exceeds 8 bits, the table is empty, has duplicate keys, or holds
    ///   keys/values exceeding the input/output widths.
    /// - Panics if the spec is not (2, 2) message/carry, which the table LUTs assume.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(8, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let x = builder.ciphertext_input(spec.int_size());
    /// let (sbox, matched) = builder.iop_match_value(&x, &[(0, 0x63), (1, 0x7c)], 8);
    /// ```
    pub fn iop_match_value(
        &self,
        src: &Ciphertext,
        table: &[(u128, u128)],
        out_size: u16,
    ) -> (Ciphertext, Ciphertext) {
        let in_size = src.spec().int_size();

        // Bypass for AES S-box and xtime optimisations
        if in_size == 8 && out_size == 8 {
            let bypass = if super::is_aes_sbox(table) {
                Some(self.iop_sbox(src))
            } else if super::is_xtime(table) {
                Some(self.iop_xtime(src))
            } else {
                None
            };
            if let Some(out) = bypass {
                let flag = self.ciphertext_join([self.block_let_ciphertext(1)], None);
                return (out, flag);
            }
        }

        let src_blocks = self.ciphertext_split(src);

        // guardrails
        assert!(!table.is_empty(), "Tried to match against an empty table.");
        // in_size as 8 maximum has been choosen because only AES uses this iop for now.
        // TODO : if needed one day, think of a way to do for > 8
        assert!(
            in_size <= 8,
            "Tried to match a value wider than 8 bits, candidates grow as 4^blocks."
        );
        assert_eq!(
            (self.spec().message_size(), self.spec().carry_size()),
            (2, 2),
            "Table LUTs assume (2, 2) blocks."
        );
        // Dense view of the table indexed by input value, doubling as key validation.
        // Sized to the packed input space, which odd `in_size`s do not fill.
        let mut lookup: Vec<Option<u128>> = vec![None; 1 << (2 * src_blocks.len())];
        for (key, value) in table {
            assert!(*key < 1 << in_size, "Key {key} exceeds the input width.");
            assert!(
                u32::from(out_size) >= u128::BITS || *value < 1 << out_size,
                "Value {value} exceeds the output width."
            );
            assert!(
                lookup[*key as usize].replace(*value).is_none(),
                "Duplicate key {key} in the table."
            );
        }

        let low = match &src_blocks[..] {
            [single] => *single,
            blocks => self.block_pack(&blocks[1], &blocks[0]),
        };
        let low_bits = 2 * src_blocks.len().min(2);
        let high_bits = 2 * src_blocks.len().saturating_sub(2);

        // Column j holds output digit j of every candidate; last column holds the matched flag.
        let out_blocks: usize = out_size.div_ceil(2).into();
        let digit = |x: u128, j: usize| -> u8 {
            match lookup[x as usize] {
                Some(v) if j < out_blocks => (v >> (2 * j) & 3) as u8,
                Some(_) => 1,
                None => 0,
            }
        };

        self.push_comment("candidates");
        let mut columns: Vec<Vec<CiphertextBlock>> = (0..=out_blocks)
            .map(|j| {
                (0..1usize << high_bits)
                    .map(|h| {
                        let entries = (0..16)
                            .map(|p| {
                                if p < 1 << low_bits {
                                    digit((h as u128) << low_bits | p as u128, j)
                                } else {
                                    0
                                }
                            })
                            .collect();
                        let name = format!("MatchValue<{h},{j}>");
                        self.block_lookup(
                            &low,
                            Lut1Def::Table {
                                name,
                                table: entries,
                            },
                        )
                    })
                    .collect()
            })
            .collect();
        self.pop_comment();

        // Mux tree: high bit b is message bit b % 2 of block 2 + b/2.
        for b in 0..high_bits {
            self.push_comment(format!("mux_{b}"));
            let cond = src_blocks[2 + b / 2];
            for column in &mut columns {
                *column = column
                    .chunks(2)
                    .map(|pair| self.mux_bit(&cond, b % 2, &pair[0], &pair[1]))
                    .collect();
            }
            self.pop_comment();
        }

        let flag = columns.pop().expect("The flag column always exists.");
        let out: Vec<_> = columns.iter().map(|column| column[0]).collect();
        (
            self.ciphertext_join(out, None),
            self.ciphertext_join(flag, None),
        )
    }

    /// 2-way select on message bit `pos` of `cond`: `if0` when clear, `if1` when set.
    /// Packing moves the tested bit into the carry field, where the `IfPos*` LUTs read it.
    fn mux_bit(
        &self,
        cond: &CiphertextBlock,
        pos: usize,
        if0: &CiphertextBlock,
        if1: &CiphertextBlock,
    ) -> CiphertextBlock {
        let (true_zeroed, false_zeroed) = match pos {
            0 => (Lut1Def::IfPos0TrueZeroed, Lut1Def::IfPos0FalseZeroed),
            _ => (Lut1Def::IfPos1TrueZeroed, Lut1Def::IfPos1FalseZeroed),
        };
        let keep0 = self.block_pack(cond, if0);
        let keep0 = self.block_lookup(&keep0, true_zeroed);
        let keep1 = self.block_pack(cond, if1);
        let keep1 = self.block_lookup(&keep1, false_zeroed);
        self.block_add(&keep0, &keep1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    fn oracle(
        table: Vec<(u128, u128)>,
        out_size: u16,
    ) -> impl Fn(&[IopValue]) -> Option<Vec<IopValue>> {
        move |inp: &[IopValue]| {
            let [IopValue::Ciphertext(src)] = inp else {
                unreachable!()
            };
            let hit = table.iter().find(|(k, _)| *k == src.as_storage());
            let (value, flag) = hit.map_or((0, 0), |(_, v)| (*v, 1));
            Some(vec![
                IopValue::Ciphertext(CiphertextSpec::new(out_size, 2, 2).from_int(value)),
                IopValue::Ciphertext(CiphertextSpec::new(2, 2, 2).from_int(flag)),
            ])
        }
    }

    #[test]
    fn test_match_value() {
        let ir = match_value(CiphertextSpec::new(2, 2, 2), &[(1, 3)], 2).optimize_ir();
        assert_display_is!(
            ir.format()
                .show_comments(false)
                .show_types(false)
                .show_opid(false)
                .with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 2>();
                %1 = extract_ct_block<0>(%0);
                %2 = pbs<Protect, Lut1("MatchValue<0,0>")>(%1);
                %3 = pbs<Protect, Lut1("MatchValue<0,1>")>(%1);
                %4 = decl_ct<2>();
                %7 = store_ct_block<0>(%2, %4);
                %11 = store_ct_block<0>(%3, %4);
                output<0>(%7);
                output<1>(%11);
            "#
        );
    }

    #[test]
    fn correctness_match_value() {
        // Sparse 4-bit table with unmatched inputs.
        let sparse = vec![(1, 9), (7, 0), (12, 15)];
        match_value(CiphertextSpec::new(4, 2, 2), &sparse, 4).test_random(50, oracle(sparse, 4));

        // Full 8-bit permutation, S-box shaped:
        // mul-swap-mul, every out digit depends on high input bits, keeping the mux tree exercised
        let full: Vec<(u128, u128)> = (0..256u128)
            .map(|k| {
                let x = (k * 167) & 0xFF;
                let x = (x >> 4 | x << 4) & 0xFF;
                (k, ((x * 41) ^ 0x5A) & 0xFF)
            })
            .collect();
        match_value(CiphertextSpec::new(8, 2, 2), &full, 8).test_random(20, oracle(full, 8));

        // 6-bit input, 2-bit output: odd block counts on both sides.
        let narrow: Vec<(u128, u128)> = (0..40).step_by(3).map(|k| (k, k & 3)).collect();
        match_value(CiphertextSpec::new(6, 2, 2), &narrow, 2).test_random(50, oracle(narrow, 2));
    }
}
