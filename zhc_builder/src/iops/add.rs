use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::{
    iter::{ChunkIt, CollectInSmallVec, IterMapFirst, MultiZip, ReconcilerOf2, Slide, SliderExt},
    svec,
};

use crate::{
    CiphertextBlock,
    builder::{Builder, Ciphertext, ExtensionBehavior},
};

/// Creates an IR for the addition of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one ciphertext
/// output representing the wrapping sum of the operands. Internally the
/// addition uses [`Builder::iop_add_hillis_steele`] for carry propagation.
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, add};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = add(spec);
/// let ir = builder.into_ir();
/// ```
pub fn add(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_add_hillis_steele(&src_a, &src_b);
    builder.ciphertext_output(res);
    builder
}

impl Builder {
    pub fn iop_add_hillis_steele(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);

        let output_blocks = self.iop_add_hillis_steele_raw(lhs_blocks, rhs_blocks, true);

        self.comment("Join").ciphertext_join(output_blocks, None)
    }

    pub(super) fn iop_add_hillis_steele_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        clean: bool,
    ) -> Vec<CiphertextBlock> {
        // Implements the addition with carry-propagation using the hillis-steele resolution and
        // group of size 4. The encoding of propagation status is the same as the one used
        // in TFHE-RS. The carry is resolved as soon as possible.
        //
        // Dead code elimination
        // =====================
        //
        // Depending on the size of the input integer, the computation may require some care to
        // handle edge cases:
        // + Non multiple of 4 number of blocks (due to 4-grouping for status computation)
        // + Non power-of-two number of groups (due to the hillis-steele resolution)
        //
        // Hopefully, thanks to dead-code elimination happening down the pipeline, we can describe
        // the computation in a larger, more favorable case, and let DCE cut the un-necessary
        // computation. This improves code readability.

        let sums = self.comment("Raw sum").vector_add(
            &lhs_blocks,
            &rhs_blocks,
            ExtensionBehavior::Passthrough,
        );

        let output_size = sums.len();
        let compute_size = sums.len().next_multiple_of(4).next_power_of_two();
        let sums = self
            .comment("Extend sum")
            .vector_unsigned_extension(sums, compute_size);

        self.push_comment("Block States");
        let block_states = sums
            .iter()
            .chunk(4)
            .map(|c| c.unwrap_complete())
            .map_first(|sum| {
                [
                    self.comment("G0-B0")
                        .block_lookup2(sum[0], Lut2Def::ManyCarryMsg)
                        .1,
                    self.comment("G0-B1")
                        .block_lookup(sum[1], Lut1Def::ExtractPropGroup0),
                    self.comment("G0-B2")
                        .block_lookup(sum[2], Lut1Def::ExtractPropGroup1),
                    self.comment("G0-B3")
                        .block_lookup(sum[3], Lut1Def::ExtractPropGroup2),
                ]
            })
            .map_rest(|sum| {
                [
                    self.comment("GN-B0")
                        .block_lookup(sum[0], Lut1Def::ExtractPropGroup0),
                    self.comment("GN-B1")
                        .block_lookup(sum[1], Lut1Def::ExtractPropGroup1),
                    self.comment("GN-B2")
                        .block_lookup(sum[2], Lut1Def::ExtractPropGroup2),
                    self.comment("GN-B3")
                        .block_padding_lookup(sum[3], Lut1Def::ExtractPropGroup3),
                ]
            })
            .cosvec();
        self.pop_comment();

        self.push_comment("Group states");
        let group_states = block_states
            .iter()
            .map_first(|states| {
                // NB: group #0 is particular, since the status is actually
                // the carry value => This group is directly solved
                let b0 = states[0];
                let b1 = self.block_add(&b0, &states[1]);
                let b2 = self.block_add(&b1, &states[2]);
                let b3 = self.block_temper_add(&b2, &states[3]);
                let b3 = self.block_lookup(&b3, Lut1Def::SolvePropGroupFinal2);
                [
                    self.comment("G0-B0").block_inspect(b0),
                    self.comment("G0-B1").block_inspect(b1),
                    self.comment("G0-B2").block_inspect(b2),
                    self.comment("G0-B3").block_inspect(b3),
                ]
            })
            .map_rest(|states| {
                let b0 = states[0];
                let b1 = self.block_add(&b0, &states[1]);
                let b2 = self.block_add(&b1, &states[2]);
                let b3 = self.block_temper_add(&b2, &states[3]);
                let b3 = self.block_wrapping_lookup(&b3, Lut1Def::ReduceCarryPad);
                let b3 = self.block_wrapping_add_plaintext(&b3, &self.block_let_plaintext(1));
                [
                    self.comment("GN-B0").block_inspect(b0),
                    self.comment("GN-B1").block_inspect(b1),
                    self.comment("GN-B2").block_inspect(b2),
                    self.comment("GN-B3").block_inspect(b3),
                ]
            })
            .cosvec();
        self.pop_comment();

        self.push_comment("Group carries");
        let mut group_carries = group_states.iter().map(|group| group[3]).cosvec();
        let nb_groups = group_carries.len();
        let nb_stages = (nb_groups as f32).log2().ceil() as usize;
        for stage in 0..nb_stages {
            self.push_comment(format!("HS {stage}-th stage"));
            let stride = 1usize << stage;
            group_carries = group_carries
                .into_iter()
                // We chunk by increasing stride, and assume complete chunks.
                .chunk(stride)
                .map(|c| c.unwrap_complete())
                // We need to assemble data from two chunks later down the pipe.
                // Prelude will be useful for the first chunk, as we will see,
                // but Postlude is not needed.
                .slide::<2>()
                .skip_postludes()
                // The first chunk of the result is already solved at the previous level.
                // We get it from the prelude of the slide, and call it a day.
                .map_first(|slider| {
                    let sv = slider.unwrap_prelude();
                    sv[0].clone().into_iter().reconcile_1_of_2()
                })
                // The next chunk combines two chunks of the previous stage with the carry lut.
                .map_first(|slider| {
                    let [prev_carry, status] = slider.unwrap_complete().into_array();
                    self.vector_zip_then_lookup(
                        status,
                        prev_carry,
                        Lut1Def::SolvePropCarry,
                        ExtensionBehavior::Panic,
                    )
                    .into_iter()
                    .reconcile_2_of_2()
                })
                // The rest of the chunks combine chunks of the previous stage with the prop lut.
                .map_rest(|slider| {
                    let [prev_carry, status] = slider.unwrap_complete().into_array();
                    self.vector_zip_then_lookup(
                        status,
                        prev_carry,
                        Lut1Def::SolveProp,
                        ExtensionBehavior::Panic,
                    )
                    .into_iter()
                    .reconcile_2_of_2()
                })
                .flatten()
                // We only take enough to build the new iterate.
                .take(nb_groups)
                .collect();
            assert_eq!(group_carries.len(), nb_groups);
            self.pop_comment();
        }
        self.pop_comment();

        self.push_comment("Final resolution");
        let carries = (group_states.into_iter(), group_carries.into_iter())
            .mzip()
            .slide::<2>()
            .skip_postludes()
            .map_first(|slider| {
                let (states, carry) = slider.unwrap_prelude()[0];
                let b1 = self.block_lookup(&states[1], Lut1Def::SolvePropGroupFinal0);
                let b2 = self.block_lookup(&states[2], Lut1Def::SolvePropGroupFinal1);
                [
                    self.comment("G0-B0").block_inspect(states[0]),
                    self.comment("G0-B1").block_inspect(b1),
                    self.comment("G0-B2").block_inspect(b2),
                    self.comment("G0-B3").block_inspect(carry),
                ]
            })
            .map_rest(|slider| {
                let [(_, previous_carry), (states, carry)] = slider.unwrap_complete().into_array();
                let b0 = self.block_add(&states[0], &previous_carry);
                let b0 = self.block_lookup(&b0, Lut1Def::SolvePropGroupFinal0);
                let b1 = self.block_add(&states[1], &previous_carry);
                let b1 = self.block_lookup(&b1, Lut1Def::SolvePropGroupFinal1);
                let b2 = self.block_add(&states[2], &previous_carry);
                let b2 = self.block_lookup(&b2, Lut1Def::SolvePropGroupFinal2);
                [
                    self.comment("GN-B0").block_inspect(b0),
                    self.comment("GN-B1").block_inspect(b1),
                    self.comment("GN-B2").block_inspect(b2),
                    self.comment("GN-B3").block_inspect(carry),
                ]
            })
            .flatten()
            .cosvec();
        self.pop_comment();

        self.push_comment("Carry propagation");
        let mut result = svec![self.block_lookup2(&sums[0], Lut2Def::ManyCarryMsg).0];
        result.extend(
            (sums.into_iter().skip(1), carries.into_iter())
                .mzip()
                .map(|(sum, carry)| self.block_add(&sum, &carry)),
        );
        self.pop_comment();

        if clean {
            self.push_comment("Cleanup");
            result = result
                .into_iter()
                .map(|ct| self.block_lookup(&ct, Lut1Def::MsgOnly))
                .cosvec();
            self.pop_comment();
        }

        result.as_slice()[..output_size].into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_add() {
        let spec = CiphertextSpec::new(18, 2, 2);
        let ir = add(spec).into_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_types(false),
            r#"
                                                   | %0 = input_ciphertext<0, 18>();
                                                   | %1 = input_ciphertext<1, 18>();
                                                   | %2 = extract_ct_block<0>(%0);
                                                   | %3 = extract_ct_block<1>(%0);
                                                   | %4 = extract_ct_block<2>(%0);
                                                   | %5 = extract_ct_block<3>(%0);
                                                   | %6 = extract_ct_block<4>(%0);
                                                   | %7 = extract_ct_block<5>(%0);
                                                   | %8 = extract_ct_block<6>(%0);
                                                   | %9 = extract_ct_block<7>(%0);
                                                   | %10 = extract_ct_block<8>(%0);
                                                   | %11 = extract_ct_block<0>(%1);
                                                   | %12 = extract_ct_block<1>(%1);
                                                   | %13 = extract_ct_block<2>(%1);
                                                   | %14 = extract_ct_block<3>(%1);
                                                   | %15 = extract_ct_block<4>(%1);
                                                   | %16 = extract_ct_block<5>(%1);
                                                   | %17 = extract_ct_block<6>(%1);
                                                   | %18 = extract_ct_block<7>(%1);
                                                   | %19 = extract_ct_block<8>(%1);
                // Raw sum                         | %20 = add_ct(%2, %11);
                // Raw sum                         | %21 = add_ct(%3, %12);
                // Raw sum                         | %22 = add_ct(%4, %13);
                // Raw sum                         | %23 = add_ct(%5, %14);
                // Raw sum                         | %24 = add_ct(%6, %15);
                // Raw sum                         | %25 = add_ct(%7, %16);
                // Raw sum                         | %26 = add_ct(%8, %17);
                // Raw sum                         | %27 = add_ct(%9, %18);
                // Raw sum                         | %28 = add_ct(%10, %19);
                // Block States / G0-B0            | %30, %31 = pbs2<Protect, Lut2("ManyCarryMsg")>(%20);
                // Block States / G0-B1            | %32 = pbs<Protect, Lut1("ExtractPropGroup0")>(%21);
                // Block States / G0-B2            | %33 = pbs<Protect, Lut1("ExtractPropGroup1")>(%22);
                // Block States / G0-B3            | %34 = pbs<Protect, Lut1("ExtractPropGroup2")>(%23);
                // Block States / GN-B0            | %35 = pbs<Protect, Lut1("ExtractPropGroup0")>(%24);
                // Block States / GN-B1            | %36 = pbs<Protect, Lut1("ExtractPropGroup1")>(%25);
                // Block States / GN-B2            | %37 = pbs<Protect, Lut1("ExtractPropGroup2")>(%26);
                // Block States / GN-B3            | %38 = pbs<AllowOutputPadding, Lut1("ExtractPropGroup3")>(%27);
                // Group states                    | %47 = add_ct(%31, %32);
                // Group states                    | %48 = add_ct(%47, %33);
                // Group states                    | %49 = temper_add_ct(%48, %34);
                // Group states                    | %50 = pbs<Protect, Lut1("SolvePropGroupFinal2")>(%49);
                // Group states                    | %55 = add_ct(%35, %36);
                // Group states                    | %56 = add_ct(%55, %37);
                // Group states                    | %57 = temper_add_ct(%56, %38);
                // Group states                    | %58 = pbs<AllowBothPadding, Lut1("ReduceCarryPad")>(%57);
                // Group states                    | %59 = let_pt_block<1>();
                // Group states                    | %60 = wrapping_add_pt(%58, %59);
                // Group carries / HS 0-th stage   | %85 = pack_ct<4>(%60, %50);
                // Group carries / HS 0-th stage   | %86 = pbs<Protect, Lut1("SolvePropCarry")>(%85);
                // Final resolution                | %95 = pbs<Protect, Lut1("SolvePropGroupFinal0")>(%47);
                // Final resolution                | %96 = pbs<Protect, Lut1("SolvePropGroupFinal1")>(%48);
                // Final resolution                | %101 = add_ct(%35, %50);
                // Final resolution                | %102 = pbs<Protect, Lut1("SolvePropGroupFinal0")>(%101);
                // Final resolution                | %103 = add_ct(%55, %50);
                // Final resolution                | %104 = pbs<Protect, Lut1("SolvePropGroupFinal1")>(%103);
                // Final resolution                | %105 = add_ct(%56, %50);
                // Final resolution                | %106 = pbs<Protect, Lut1("SolvePropGroupFinal2")>(%105);
                // Carry propagation               | %133 = add_ct(%21, %31);
                // Carry propagation               | %134 = add_ct(%22, %95);
                // Carry propagation               | %135 = add_ct(%23, %96);
                // Carry propagation               | %136 = add_ct(%24, %50);
                // Carry propagation               | %137 = add_ct(%25, %102);
                // Carry propagation               | %138 = add_ct(%26, %104);
                // Carry propagation               | %139 = add_ct(%27, %106);
                // Carry propagation               | %140 = add_ct(%28, %86);
                // Cleanup                         | %148 = pbs<Protect, Lut1("MsgOnly")>(%30);
                // Cleanup                         | %149 = pbs<Protect, Lut1("MsgOnly")>(%133);
                // Cleanup                         | %150 = pbs<Protect, Lut1("MsgOnly")>(%134);
                // Cleanup                         | %151 = pbs<Protect, Lut1("MsgOnly")>(%135);
                // Cleanup                         | %152 = pbs<Protect, Lut1("MsgOnly")>(%136);
                // Cleanup                         | %153 = pbs<Protect, Lut1("MsgOnly")>(%137);
                // Cleanup                         | %154 = pbs<Protect, Lut1("MsgOnly")>(%138);
                // Cleanup                         | %155 = pbs<Protect, Lut1("MsgOnly")>(%139);
                // Cleanup                         | %156 = pbs<Protect, Lut1("MsgOnly")>(%140);
                // Join                            | %164 = decl_ct<18>();
                // Join                            | %165 = store_ct_block<0>(%148, %164);
                // Join                            | %166 = store_ct_block<1>(%149, %165);
                // Join                            | %167 = store_ct_block<2>(%150, %166);
                // Join                            | %168 = store_ct_block<3>(%151, %167);
                // Join                            | %169 = store_ct_block<4>(%152, %168);
                // Join                            | %170 = store_ct_block<5>(%153, %169);
                // Join                            | %171 = store_ct_block<6>(%154, %170);
                // Join                            | %172 = store_ct_block<7>(%155, %171);
                // Join                            | %173 = store_ct_block<8>(%156, %172);
                                                   | output<0>(%173);
            "#
        );
    }

    #[test]
    fn correctness() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.add(*rhs))])
        }
        for size in (2..128).step_by(2) {
            add(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }
}
