use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{IopLang, Lut1Def, Lut2Def};
use hc_utils::{
    iter::{
        ChunkIt, CollectInSmallVec, Intermediate, IterMapFirst, MultiZip, Slide,
        filter_out_postludes,
    },
    svec,
};

use crate::builder::{Builder, Ciphertext, ExtensionBehavior};

pub fn add(spec: CiphertextSpec) -> IR<IopLang> {
    // Add 2 integers, using the Hillis Steel method.
    // Outputs a list containing the resulting blocks.
    // clean_ct : option to clean the ct at the output.
    // Note that if the ct is not cleaned, its noise
    // has increased by 2 additions, compared to the input.

    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.declare_ciphertext_input(spec.int_size());
    let src_b = builder.declare_ciphertext_input(spec.int_size());
    let res = builder.iop_add_hillis_steele(&src_a, &src_b);
    builder.declare_ciphertext_output(res);
    builder.into_ir()
}

impl Builder {
    pub fn iop_add_hillis_steele(&mut self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        // Step 0 =================================================================
        let lhs_blocks = self.split_ciphertext(lhs);
        let rhs_blocks = self.split_ciphertext(rhs);

        // Step 1 =================================================================
        // Add blocks together, no fancy stuff here. Just pack two input vector (with msg only)
        // into one vector with msg + [1b of carry]
        // Also handle src of different sizes.
        self.push_comment("Raw sum");
        let sums = self.vector_add(&lhs_blocks, &rhs_blocks, ExtensionBehavior::Passthrough);
        assert!(
            sums.len().is_multiple_of(4),
            "Addition for non-multiple-of-4 integers is not supported yet."
        );
        self.pop_comment();

        // Step 2 =================================================================
        // Compute P/G/N state for each block.
        // States are encoded with tfhe-rs flag with a linear merging of 4 blocks in mind
        // P => 0b01 << (block_id % 4)
        // N => 0b00 << (block_id % 4)
        // G => 0b10 << (block_id % 4)
        self.push_comment("Computing PGNs");
        let pgns = sums
            .iter()
            .chunk(4)
            .map(|c| c.unwrap_complete())
            .enumerate()
            // NB: LSB bloc is handle differently to reduce the work
            //     -> input carry is known, so it could be directly resolved
            .map_first(|(_, chunk)| {
                self.with_comment("Special first", || {
                    [
                        self.block_lookup2(&chunk[0], Lut2Def::ManyCarryMsg).1,
                        self.block_lookup(&sums[1], Lut1Def::ExtractPropGroup0),
                        self.block_lookup(&sums[2], Lut1Def::ExtractPropGroup1),
                        self.block_lookup(&sums[3], Lut1Def::ExtractPropGroup2),
                    ]
                })
            })
            .map_rest(|(i, chunk)| {
                self.with_comment(format!("{i}-th group"), || {
                    [
                        self.block_lookup(chunk[0], Lut1Def::ExtractPropGroup0),
                        self.block_lookup(chunk[1], Lut1Def::ExtractPropGroup1),
                        self.block_lookup(chunk[2], Lut1Def::ExtractPropGroup2),
                        self.block_lookup(chunk[3], Lut1Def::ExtractPropGroup3),
                    ]
                })
            })
            .cosvec();
        self.pop_comment();

        // Step 3 =================================================================
        // Spread propagation status *within* each chunk
        // spread_pgns [0,1,2] contains the sum of the propagation status at each step
        // spread_pgns [3] contains the propagation status of the chunk
        self.push_comment("Spread PGNs within groups");
        let spread_pgns = pgns
            .iter()
            .enumerate()
            // NB: chunk #0 is particular, since the status is actually
            // the carry value => This chunk is directly solved
            .map_first(|(_, chunk)| {
                self.with_comment("Special first", || {
                    let s0 = chunk[0];
                    let s1 = self.block_add(&s0, &chunk[1]);
                    let s2 = self.block_add(&s1, &chunk[2]);
                    let _s3 = self.block_add(&s2, &chunk[3]);
                    let s3 = self.block_lookup(&_s3, Lut1Def::SolvePropGroupFinal2);
                    [s0, s1, s2, s3]
                })
            })
            .map_rest(|(i, chunk)| {
                self.with_comment(format!("{i}-th group"), || {
                    let s0 = chunk[0];
                    let s1 = self.block_add(&s0, &chunk[1]);
                    let s2 = self.block_add(&s1, &chunk[2]);
                    let _s3 = self.block_add(&s2, &chunk[3]);
                    let _s3 = self.block_lookup(&_s3, Lut1Def::ReduceCarryPad);
                    let cst_1 = self.block_const_plaintext(1);
                    let s3 = self.block_wrapping_add_plaintext(&_s3, &cst_1);
                    [s0, s1, s2, s3]
                })
            })
            .cosvec();
        self.pop_comment();

        // Step 4 ===========================================
        // Resolve PGN status across each group with parallel scan (based on Hillis-Steel algorithm)
        // I.e. solve PGN into GN for each chunk => Will end with a carry info for each group
        self.push_comment("Resolve PGNs across groups");
        let mut chunk_gns = spread_pgns.iter().map(|group| group[3]).cosvec();
        let chunk_nb = chunk_gns.len();
        let stage_nb = (chunk_nb as f32).log2().ceil() as usize;
        for stage in 0..stage_nb {
            self.push_comment(format!("{stage}-th stage"));
            // After stage_nb the propagation will be complete.
            let stride = 1usize << stage;
            // We override the chunks_gns with the current stage one.
            chunk_gns = chunk_gns
                .into_iter()
                // We chunk by increasing stride, and assume complete chunks.
                .chunk(stride)
                .map(|c| c.unwrap_complete())
                // We need to assemble data from two chunks later down the pipe.
                // Prelude will be useful for the first chunk, as we will see,
                // but Postlude is not needed.
                .slide::<2>()
                .filter(filter_out_postludes)
                // The first chunk of the result is already solved at the previous level.
                // We get it from the prelude of the slide, and call it a day.
                .map_first(|slider| {
                    let sv = slider.unwrap_prelude();
                    sv[0].clone().into_iter()
                })
                // The next chunk combines two chunks of the previous stage with the carry lut.
                .map_first(|slider| {
                    let sv = slider.unwrap_complete();
                    assert_eq!(sv[0].len(), sv[1].len());
                    (sv[0].iter(), sv[1].iter())
                        .mzip()
                        .map(|(li, ri)| {
                            self.block_pack_then_lookup(ri, li, Lut1Def::SolvePropCarry)
                        })
                        .intermediate()
                })
                // The rest of the chunks combine chunks of the previous stage with the prop lut.
                .map_rest(|slider| {
                    let sv = slider.unwrap_complete();
                    assert_eq!(sv[0].len(), sv[1].len());
                    (sv[0].iter(), sv[1].iter())
                        .mzip()
                        .map(|(li, ri)| self.block_pack_then_lookup(ri, li, Lut1Def::SolveProp))
                        .intermediate()
                })
                .flatten()
                // We only take enough to build the new iterate.
                .take(chunk_nb)
                .collect();
            assert_eq!(chunk_gns.len(), chunk_nb);
            self.pop_comment();
        }
        self.pop_comment();

        // Step 5 =================================================================
        // Final resolution: Solve PGN status inside chunk
        // Convert back 2d array in vector
        self.push_comment("Final resolution");
        let carries = (spread_pgns.into_iter(), chunk_gns.into_iter())
            .mzip()
            .slide::<2>()
            .filter(filter_out_postludes)
            .enumerate()
            .map_first(|(_, slider)| {
                self.with_comment("Special first", || {
                    let (spread_pgn, chunk_gn) = slider.unwrap_prelude()[0];
                    [
                        spread_pgn[0],
                        self.block_lookup(&spread_pgn[1], Lut1Def::SolvePropGroupFinal0),
                        self.block_lookup(&spread_pgn[2], Lut1Def::SolvePropGroupFinal1),
                        chunk_gn,
                    ]
                })
            })
            .map_rest(|(i, slider)| {
                self.with_comment(format!("{i}-th group"), || {
                    let sv = slider.unwrap_complete();
                    let (_, prev_chunk_gn) = sv[0];
                    let (spread_pgn, chunk_gn) = sv[1];
                    let s0 = self.block_add(&spread_pgn[0], &prev_chunk_gn);
                    let s0 = self.block_lookup(&s0, Lut1Def::SolvePropGroupFinal0);
                    let s1 = self.block_add(&spread_pgn[1], &prev_chunk_gn);
                    let s1 = self.block_lookup(&s1, Lut1Def::SolvePropGroupFinal1);
                    let s2 = self.block_add(&spread_pgn[2], &prev_chunk_gn);
                    let s2 = self.block_lookup(&s2, Lut1Def::SolvePropGroupFinal2);
                    [s0, s1, s2, chunk_gn]
                })
            })
            .flatten()
            .cosvec();
        self.pop_comment();

        // Step 6 =================================================================
        // Carry is known now, propagate them in sum
        self.push_comment("Carry propagation");
        let mut result = svec![self.block_lookup2(&sums[0], Lut2Def::ManyCarryMsg).0];
        result.extend(
            (sums.into_iter().skip(1), carries.into_iter())
                .mzip()
                .map(|(sum, carry)| self.block_add(&sum, &carry)),
        );
        self.pop_comment();

        // Step 7 =================================================================
        // Cleanup
        self.push_comment("Cleanup");
        let result = result
            .into_iter()
            .map(|ct| self.block_lookup(&ct, Lut1Def::MsgOnly))
            .cosvec();
        self.pop_comment();

        self.join_ciphertext(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hc_utils::assert_display_is;

    #[test]
    fn test_add() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = add(spec);
        assert_display_is!(
            ir.format()
                .with_walker(hc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_types(false),
            r#"
                                                               | %0 = input<0, CtInt>();
                                                               | %1 = input<1, CtInt>();
                                                               | %2 = extract_ct_block<0>(%0);
                                                               | %3 = extract_ct_block<1>(%0);
                                                               | %4 = extract_ct_block<2>(%0);
                                                               | %5 = extract_ct_block<3>(%0);
                                                               | %6 = extract_ct_block<4>(%0);
                                                               | %7 = extract_ct_block<5>(%0);
                                                               | %8 = extract_ct_block<6>(%0);
                                                               | %9 = extract_ct_block<7>(%0);
                                                               | %10 = extract_ct_block<0>(%1);
                                                               | %11 = extract_ct_block<1>(%1);
                                                               | %12 = extract_ct_block<2>(%1);
                                                               | %13 = extract_ct_block<3>(%1);
                                                               | %14 = extract_ct_block<4>(%1);
                                                               | %15 = extract_ct_block<5>(%1);
                                                               | %16 = extract_ct_block<6>(%1);
                                                               | %17 = extract_ct_block<7>(%1);
                // Raw sum                                     | %18 = add_ct(%2, %10);
                // Raw sum                                     | %19 = add_ct(%3, %11);
                // Raw sum                                     | %20 = add_ct(%4, %12);
                // Raw sum                                     | %21 = add_ct(%5, %13);
                // Raw sum                                     | %22 = add_ct(%6, %14);
                // Raw sum                                     | %23 = add_ct(%7, %15);
                // Raw sum                                     | %24 = add_ct(%8, %16);
                // Raw sum                                     | %25 = add_ct(%9, %17);
                // Computing PGNs / Special first              | %26, %27 = pbs2<ManyCarryMsg>(%18);
                // Computing PGNs / Special first              | %28 = pbs<ExtractPropGroup0>(%19);
                // Computing PGNs / Special first              | %29 = pbs<ExtractPropGroup1>(%20);
                // Computing PGNs / Special first              | %30 = pbs<ExtractPropGroup2>(%21);
                // Computing PGNs / 1-th group                 | %31 = pbs<ExtractPropGroup0>(%22);
                // Computing PGNs / 1-th group                 | %32 = pbs<ExtractPropGroup1>(%23);
                // Computing PGNs / 1-th group                 | %33 = pbs<ExtractPropGroup2>(%24);
                // Spread PGNs within groups / Special first   | %35 = add_ct(%27, %28);
                // Spread PGNs within groups / Special first   | %36 = add_ct(%35, %29);
                // Spread PGNs within groups / Special first   | %37 = add_ct(%36, %30);
                // Spread PGNs within groups / Special first   | %38 = pbs<SolvePropGroupFinal2>(%37);
                // Spread PGNs within groups / 1-th group      | %39 = add_ct(%31, %32);
                // Spread PGNs within groups / 1-th group      | %40 = add_ct(%39, %33);
                // Final resolution / Special first            | %47 = pbs<SolvePropGroupFinal0>(%35);
                // Final resolution / Special first            | %48 = pbs<SolvePropGroupFinal1>(%36);
                // Final resolution / 1-th group               | %49 = add_ct(%31, %38);
                // Final resolution / 1-th group               | %50 = pbs<SolvePropGroupFinal0>(%49);
                // Final resolution / 1-th group               | %51 = add_ct(%39, %38);
                // Final resolution / 1-th group               | %52 = pbs<SolvePropGroupFinal1>(%51);
                // Final resolution / 1-th group               | %53 = add_ct(%40, %38);
                // Final resolution / 1-th group               | %54 = pbs<SolvePropGroupFinal2>(%53);
                // Carry propagation                           | %57 = add_ct(%19, %27);
                // Carry propagation                           | %58 = add_ct(%20, %47);
                // Carry propagation                           | %59 = add_ct(%21, %48);
                // Carry propagation                           | %60 = add_ct(%22, %38);
                // Carry propagation                           | %61 = add_ct(%23, %50);
                // Carry propagation                           | %62 = add_ct(%24, %52);
                // Carry propagation                           | %63 = add_ct(%25, %54);
                // Cleanup                                     | %64 = pbs<MsgOnly>(%26);
                // Cleanup                                     | %65 = pbs<MsgOnly>(%57);
                // Cleanup                                     | %66 = pbs<MsgOnly>(%58);
                // Cleanup                                     | %67 = pbs<MsgOnly>(%59);
                // Cleanup                                     | %68 = pbs<MsgOnly>(%60);
                // Cleanup                                     | %69 = pbs<MsgOnly>(%61);
                // Cleanup                                     | %70 = pbs<MsgOnly>(%62);
                // Cleanup                                     | %71 = pbs<MsgOnly>(%63);
                                                               | %72 = zero_ct();
                                                               | %73 = store_ct_block<0>(%64, %72);
                                                               | %74 = store_ct_block<1>(%65, %73);
                                                               | %75 = store_ct_block<2>(%66, %74);
                                                               | %76 = store_ct_block<3>(%67, %75);
                                                               | %77 = store_ct_block<4>(%68, %76);
                                                               | %78 = store_ct_block<5>(%69, %77);
                                                               | %79 = store_ct_block<6>(%70, %78);
                                                               | %80 = store_ct_block<7>(%71, %79);
                                                               | output<0, CtInt>(%80);
            "#
        );
    }
}
