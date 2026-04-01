use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::{Lut1Def, Lut2Def};

use crate::{
    BitType, CiphertextBlock, PropagationDirection,
    builder::{Builder, Ciphertext},
};

pub fn div(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let (quotient, remainder) = builder.iop_divx(&src_a, &src_b);
    builder.ciphertext_output(quotient);
    builder.ciphertext_output(remainder);
    builder
}

/// Initialize the division.
/// It computes:
/// * keep_div : boolean to keep the division result, or set the default value.
/// * div_x1_is_not_null_a : list of booleans indicating if the divider is null from a certain block
///   position.
/// * div_x2_is_not_null_a : list of booleans indicating if the divider x2 is null from a certain
///   block position.
/// * div_x3_is_not_null_a : list of booleans indicating if the divider x3 is null from a ce;rtain
///   block position.
/// * mdiv_x1_a : opposite value of divider
/// * mdiv_x2_a : opposite value of divider x2
/// * mdiv_x3_a : opposite value of divider x3
struct IopDivInitStruct {
    keep_div: CiphertextBlock,
    div_x2_a: Vec<CiphertextBlock>,
    div_x3_a: Vec<CiphertextBlock>,
    div_x1_is_not_null_a: Vec<CiphertextBlock>,
    div_x2_is_not_null_a: Vec<CiphertextBlock>,
    div_x3_is_not_null_a: Vec<CiphertextBlock>,
    mdiv_x1_a: Vec<CiphertextBlock>,
    mdiv_x2_a: Vec<CiphertextBlock>,
    mdiv_x3_a: Vec<CiphertextBlock>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum IfThenElse0Select {
    SelPos0Msg,       // Select bool in position 0 of carry part, output msg
    SelPos1Msg,       // Select bool in position 1 of carry part, output msg
    #[allow(unused)]
    SelPos1MsgCarry1, // Select bool in position 1 of carry part, output msg + 1bit of carry
}

impl Builder {
    pub fn iop_divx(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> (Ciphertext, Ciphertext) {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let (quotient_blocks, remain_blocks) = self.iop_div_corev(lhs_blocks, rhs_blocks);
        (
            self.ciphertext_join(quotient_blocks, None),
            self.ciphertext_join(remain_blocks, None),
        )
    }

    fn iop_div_corev(
        &self,
        src_a: impl AsRef<[CiphertextBlock]>,
        src_b: impl AsRef<[CiphertextBlock]>,
    ) -> (Vec<CiphertextBlock>, Vec<CiphertextBlock>) {
        let src_a = src_a.as_ref();
        let src_b = src_b.as_ref();

        // Wrapped required lookup table in MetaVar
        // let pbs_msg_not_null_pos1 = new_pbs!(prog, "MsgNotNullPos1");
        // let pbs_is_null_pos1 = new_pbs!(prog, "IsNullPos1");
        // let pbs_xor = new_pbs!(prog, "BwXor");
        // let pbs_none = new_pbs!(prog, "None");
        // let pbs_solve_quotient_pos1 = new_pbs!(prog, "SolveQuotientPos1");

        // Renaming for clarity
        let num_a = src_a;
        let div_x1_a = src_b;

        // Initialization
        let IopDivInitStruct {
            keep_div,
            div_x2_a,
            div_x3_a,
            div_x1_is_not_null_a,
            div_x2_is_not_null_a,
            div_x3_is_not_null_a,
            mdiv_x1_a,
            mdiv_x2_a,
            mdiv_x3_a,
        } = self.iop_div_initv(&div_x1_a);

        let div_x_v = [div_x1_a, div_x2_a.as_slice(), div_x3_a.as_slice()];
        let div_x_is_not_null_v = [
            div_x1_is_not_null_a,
            div_x2_is_not_null_a,
            div_x3_is_not_null_a,
        ];
        let mdiv_x_v = [mdiv_x1_a, mdiv_x2_a, mdiv_x3_a];

        // Loop
        let mut quotient_a = Vec::new();
        let mut remain_a = Vec::new();
        let cst_sign = self.block_let_ciphertext((1 << self.spec().1) - 1);

        for loop_idx in 0..num_a.len() {
            let block_nb = loop_idx + 1;
            let mut entry_num = Vec::new();
            entry_num.push(num_a[num_a.len() - 1 - loop_idx].clone());
            entry_num.append(&mut remain_a);
            remain_a = entry_num; // rename

            let mut diff_x_v: Vec<Vec<_>> = Vec::new();
            let mut r_lt_div_x_v = Vec::new();

            diff_x_v.push(remain_a.clone()); // Corresponds to remain - (div * 0)
            for xi in 0..3 {
                // for x1, x2, x3
                // Step 1
                // Sign extension
                let mut ext_mdiv_x_a = Vec::new();
                for (_k, ct) in (0..block_nb).zip(mdiv_x_v[xi].iter()) {
                    ext_mdiv_x_a.push(ct.clone());
                }
                for _k in mdiv_x_v[xi].len()..block_nb {
                    ext_mdiv_x_a.push(cst_sign.clone());
                }
                ext_mdiv_x_a.push(cst_sign.clone());

                // Step2
                // Compute remain - div
                // Here, do not clean the last block, which is the carry
                let diff_x_a = self.iop_add_hillis_steele_raw(&remain_a, &ext_mdiv_x_a, false);

                // Step3
                // Comparison : look at the sign block
                let mut is_lt;
                if block_nb < div_x_v[xi].len() {
                    // Take the msb of div_x into account.
                    // The sign block contains either 'b100 (positive) or 'b11 (negative).
                    // The remain is less than div_x if:
                    // div_x msb is not null => div_x_is_not_null_v[xi][block_nb] != 0
                    // or the difference is negative.
                    // Note that if we subtract div_x_is_not_null to the sign block,
                    // if the result is 'b100, this means that the result is positive.
                    // In all other case ('b11 or 'b10) the result is negative.
                    is_lt = self.block_sub(&diff_x_a[block_nb], &div_x_is_not_null_v[xi][block_nb]);
                    is_lt = self.block_lookup(is_lt, Lut1Def::MsgNotNullPos1);
                } else {
                    is_lt = self.block_lookup(diff_x_a[block_nb], Lut1Def::MsgNotNullPos1);
                }
                // Note that here the lt boolean is stored in position 1 and not 0
                // to ease the if_then_else later.
                r_lt_div_x_v.push(is_lt);

                diff_x_v.push(diff_x_a);
            } // for xi

            // Do not compute the remain for the very last iteration, since not needed anymore.
            // Find the 1hot corresponding to the 1rst factor of div which is not greater than r.
            // {r_lt_div_x3, r_lt_div_x2, r_lt_div_x1, 0} xor {1, r_lt_div_x3,
            // r_lt_div_x2,r_lt_div_x1}
            let mut q_1h = Vec::new();
            let ct1 =
                self.block_pack_then_lookup(r_lt_div_x_v[0], &r_lt_div_x_v[1], Lut1Def::BwXor);
            let ct2 =
                self.block_pack_then_lookup(r_lt_div_x_v[1], &r_lt_div_x_v[2], Lut1Def::BwXor);
            let ct3 = self.block_lookup(r_lt_div_x_v[2], Lut1Def::IsNullPos1);
            q_1h.push(r_lt_div_x_v[0].clone());
            q_1h.push(ct1);
            q_1h.push(ct2);
            q_1h.push(ct3);

            // Select the remain with the 1-hot
            // Mask then Or
            // Note that the sign block is not used here.
            let mut remain_tmp_v = Vec::new();
            for (sel, diff) in q_1h.iter().zip(diff_x_v.iter()) {
                remain_tmp_v.push(self.iop_if_then_else_0v(
                    &diff[0..block_nb],
                    sel,
                    Some(IfThenElse0Select::SelPos1Msg),
                ));
            }

            remain_a = Vec::new();
            #[allow(clippy::needless_range_loop)]
            for i in 0..block_nb {
                remain_tmp_v[0][i] = self.block_add(&remain_tmp_v[0][i], &remain_tmp_v[1][i]);
                remain_tmp_v[2][i] = self.block_add(&remain_tmp_v[2][i], &remain_tmp_v[3][i]);
                remain_tmp_v[0][i] = self.block_add(&remain_tmp_v[0][i], &remain_tmp_v[2][i]);
                remain_a.push(self.block_lookup(remain_tmp_v[0][i], Lut1Def::None));
            }

            // Quotient
            // Note that {r_lt_div_x3, r_lt_div_x2, r_lt_div_x1, 0} is a multi-hot.
            // with the 1s in the MBSs. Therefore, we can deduce the quotient 2 bits
            // from the nb of 1.
            // Note : In r_lt_div_x the boolean is stored in position 1 instead of 0.
            // 'b0000 => 3 * 2
            // 'b1000 => 2 * 2
            // 'b1100 => 1 * 2
            // 'b1110 => 0 * 2
            let ct01 = r_lt_div_x_v[0].clone(); // + 0
            let ct23 = self.block_add(&r_lt_div_x_v[1], &r_lt_div_x_v[2]);
            let ct0123 = self.block_add(&ct01, &ct23);
            quotient_a.push(self.block_lookup(ct0123, Lut1Def::SolveQuotientPos1));
        } // for loop_idx

        quotient_a.reverse();
        quotient_a =
            self.iop_if_then_else_0v(&quotient_a, &keep_div, Some(IfThenElse0Select::SelPos0Msg));

        (quotient_a, remain_a)
    }

    /// select is a boolean.
    /// There several select type. See IfThenElse0Select description.
    fn iop_if_then_else_0v(
        &self,
        src: impl AsRef<[CiphertextBlock]>,
        select: impl AsRef<CiphertextBlock>,
        select_type: Option<IfThenElse0Select>, // Default SelPos0Msg
    ) -> Vec<CiphertextBlock> {
        let src = src.as_ref();
        let select = select.as_ref();
        src.iter()
            .map(
                |ct| match select_type.unwrap_or(IfThenElse0Select::SelPos0Msg) {
                    IfThenElse0Select::SelPos0Msg => {
                        self.block_pack_then_lookup(select, ct, Lut1Def::IfFalseZeroed)
                    }
                    IfThenElse0Select::SelPos1Msg => {
                        self.block_pack_then_lookup(select, ct, Lut1Def::IfPos1FalseZeroed)
                    }
                    IfThenElse0Select::SelPos1MsgCarry1 => {
                        self.block_pack_then_lookup(select, ct, Lut1Def::IfPos1FalseZeroedMsgCarry1)
                    }
                },
            )
            .collect::<Vec<_>>()
    }

    fn iop_div_initv(&self, div_x1_a: impl AsRef<[CiphertextBlock]>) -> IopDivInitStruct {
        // let props = prog.params();
        // let tfhe_params: asm::DigitParameters = props.clone().into();
        let div_x1_a = div_x1_a.as_ref();

        // Note that div_x2 and div_x3 has an additional ct in msb
        let (div_x2_a, div_x3_a) = self.iop_x2_x3v(div_x1_a);

        let div_x1_is_not_null_a = self.propagate_blocks(
            div_x1_a,
            BitType::One,
            PropagationDirection::MsbToLsb,
            false,
        );
        let div_x2_is_not_null_a = self.propagate_blocks(
            &div_x2_a,
            BitType::One,
            PropagationDirection::MsbToLsb,
            false,
        );
        let div_x3_is_not_null_a = self.propagate_blocks(
            &div_x3_a,
            BitType::One,
            PropagationDirection::MsbToLsb,
            false,
        );

        // If the divider is null set quotient to 0
        let keep_div = div_x1_is_not_null_a[0].clone();

        // During the operation, we need to subtract div_x1, div_x2, and div_x3.
        // Compute here (-div_x1), (-div_x2), (-div_x3) in 2s complement.
        // Note that the opposite values have an additional ct for the sign.
        let mdiv_x1_a = self.iop_opposite_nopropv(div_x1_a);
        let mdiv_x2_a = self.iop_opposite_nopropv(&div_x2_a);
        let mdiv_x3_a = self.iop_opposite_nopropv(&div_x3_a);

        IopDivInitStruct {
            keep_div,
            div_x2_a,
            div_x3_a,
            div_x1_is_not_null_a,
            div_x2_is_not_null_a,
            div_x3_is_not_null_a,
            mdiv_x1_a,
            mdiv_x2_a,
            mdiv_x3_a,
        }
    }

    pub fn iop_opposite_nopropv(&self, src: impl AsRef<[CiphertextBlock]>) -> Vec<CiphertextBlock> {
        let src = src.as_ref();
        let cst_msg = self.block_let_plaintext(1 << self.spec().1);
        let cst_msg_m1 = self.block_let_plaintext((1 << self.spec().1) - 1);
        let mut m_src_a = Vec::new();
        let ct = self.block_plaintext_sub(cst_msg, src[0]);
        m_src_a.push(ct);
        for ct in src.iter().skip(1) {
            let tmp = self.block_plaintext_sub(cst_msg_m1, ct);
            m_src_a.push(tmp);
        }

        // Add the sign
        // Create sign cst backed in register
        let sign = self.block_let_ciphertext((1 << self.spec().1) - 1);
        m_src_a.push(sign);
        m_src_a
    }

    /// Outputs a tuple corresponding to (src x2, src x3)
    fn iop_x2_x3v(
        &self,
        src: impl AsRef<[CiphertextBlock]>,
    ) -> (Vec<CiphertextBlock>, Vec<CiphertextBlock>) {
        let src = src.as_ref();
        // let pbs_many_msg_split_shift1 = new_pbs!(prog, "ManyMsgSplitShift1");

        // First step
        // Compute x2
        let mut x2_a: Vec<CiphertextBlock> = Vec::new(); // Will contain lsb part of the msg
        let last_msb = src.iter().fold(None, |prev_msb, x| {
            let (mut lsb, msb) = self.block_lookup2(x, Lut2Def::ManyMsgSplitShift1);
            if let Some(v) = prev_msb {
                lsb = self.block_add(lsb, v); // add with previous msb
            }
            x2_a.push(lsb);
            Some(msb)
        });
        x2_a.push(last_msb.unwrap());

        // Second step compute x3
        let x3_a = self.iop_add_hillis_steele_raw(&x2_a, src, true);

        (x2_a, x3_a)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::iter::CollectInSmallVec;

    #[test]
    fn test_div() {
        let spec = CiphertextSpec::new(64, 2, 2);
        let ir = div(spec);
        let eval = ir.eval().with_inputs([
            IopValue::Ciphertext(spec.from_int(20)),
            IopValue::Ciphertext(spec.from_int(3)),
        ]).get_outputs();
        let eval = eval
            .into_iter()
            .map(|a| a.unwrap_ciphertext().as_storage())
            .cosvec();
        dbg!(&eval);
    }

    #[test]
    fn correctness() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            if rhs.as_storage() == 0 {
                return None;
            }
            let quotient = lhs.as_storage().div_euclid(rhs.as_storage());
            let remainder = lhs.as_storage().rem_euclid(rhs.as_storage());
            Some(vec![
                IopValue::Ciphertext(lhs.spec().from_int(quotient)),
                IopValue::Ciphertext(rhs.spec().from_int(remainder)),
            ])
        }
        for size in (2..128).step_by(2) {
            div(CiphertextSpec::new(size, 2, 2)).test_random(10, semantic);
        }
        for size in [16, 32, 64, 128] {
            div(CiphertextSpec::new(size, 2, 2)).test_random(1000, semantic);
        }

    }
}
