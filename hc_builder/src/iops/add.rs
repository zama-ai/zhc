use std::cmp::max;

use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{IopLang, Lut1Def, Lut2Def};
use hc_utils::iter::{
    ChunkIt, CollectInSmallVec, Intermediate, IterMapFirst, MultiZip, Slide, filter_out_postludes,
};

use crate::builder::{Builder, Ciphertext, ExtensionBehavior};

pub fn add(spec: CiphertextSpec) -> IR<IopLang> {
    // Add 2 integers, using the Hillis Steel method.
    // Outputs a list containing the resulting blocks.
    // clean_ct : option to clean the ct at the output.
    // Note that if the ct is not cleaned, its noise
    // has increased by 2 additions, compared to the input.

    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.eint_input(spec.int_size());
    let src_b = builder.eint_input(spec.int_size());
    let res = builder.iop_add_hillis_steele(src_a, src_b);
    builder.eint_output(res);
    builder.into_ir()
}

impl Builder {
    pub fn iop_add_hillis_steele(&mut self, lhs: Ciphertext, rhs: Ciphertext) -> Ciphertext {
        // Step 1 =================================================================
        // Add blocks together, no fancy stuff here. Just pack two input vector (with msg only)
        // into one vector with msg + [1b of carry]
        // Also handle src of different sizes.
        let sums = self.vector_add(&lhs, &rhs, ExtensionBehavior::Passthrough);
        assert!(
            sums.len().is_multiple_of(4),
            "Addition for non-multiple-of-4 integers is not supported yet."
        );

        // Step 2 =================================================================
        // Compute P/G/N state for each block.
        // States are encoded with tfhe-rs flag with a linear merging of 4 blocks in mind
        // P => 0b01 << (block_id % 4)
        // N => 0b00 << (block_id % 4)
        // G => 0b10 << (block_id % 4)
        let pgns = sums
            .iter()
            .chunk(4)
            .map(|c| c.unwrap_complete())
            // NB: LSB bloc is handle differently to reduce the work
            //     -> input carry is known, so it could be directly resolved
            .map_first(|chunk| {
                [
                    self.block_pbs2(&chunk[0], Lut2Def::ManyCarryMsg).1,
                    self.block_pbs(&sums[1], Lut1Def::ExtractPropGroup0),
                    self.block_pbs(&sums[2], Lut1Def::ExtractPropGroup1),
                    self.block_pbs(&sums[3], Lut1Def::ExtractPropGroup2),
                ]
            })
            .map_rest(|chunk| {
                [
                    self.block_pbs(chunk[0], Lut1Def::ExtractPropGroup0),
                    self.block_pbs(chunk[1], Lut1Def::ExtractPropGroup1),
                    self.block_pbs(chunk[2], Lut1Def::ExtractPropGroup2),
                    self.block_pbs(chunk[3], Lut1Def::ExtractPropGroup3),
                ]
            })
            .cosvec();

        // Step 3 =================================================================
        // Spread propagation status *within* each chunk
        // spread_pgns [0,1,2] contains the sum of the propagation status at each step
        // spread_pgns [3] contains the propagation status of the chunk
        let spread_pgns = pgns
            .iter()
            // NB: chunk #0 is particular, since the status is actually
            // the carry value => This chunk is directly solved
            .map_first(|chunk| {
                let s0 = chunk[0];
                let s1 = self.block_add(&s0, &chunk[1]);
                let s2 = self.block_add(&s1, &chunk[2]);
                let _s3 = self.block_add(&s2, &chunk[3]);
                let s3 = self.block_pbs(&_s3, Lut1Def::SolvePropGroupFinal2);
                [s0, s1, s2, s3]
            })
            .map_rest(|chunk| {
                let s0 = chunk[0];
                let s1 = self.block_add(&s0, &chunk[1]);
                let s2 = self.block_add(&s1, &chunk[2]);
                let cst_1 = self.block_constant(1);
                let _s3 = self.block_add(&s2, &chunk[3]);
                let _s3 = self.block_pbs(&_s3, Lut1Def::ReduceCarryPad);
                let s3 = self.block_adds(&_s3, &cst_1);
                [s0, s1, s2, s3]
            })
            .cosvec();

        // Step 4 ===========================================
        // Resolve PGN status across each group with parallel scan (based on Hillis-Steel algorithm)
        // I.e. solve PGN into GN for each chunk => Will end with a carry info for each group
        let mut chunk_gns = spread_pgns.iter().map(|group| group[3]).cosvec();
        let chunk_nb = chunk_gns.len();
        let stage_nb = (chunk_nb as f32).log2().ceil() as usize;
        for stage in 0..stage_nb {
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
                        .map(|(li, ri)| self.block_pack_lut(ri, li, Lut1Def::SolvePropCarry))
                        .intermediate()
                })
                // The rest of the chunks combine chunks of the previous stage with the prop lut.
                .map_rest(|slider| {
                    let sv = slider.unwrap_complete();
                    assert_eq!(sv[0].len(), sv[1].len());
                    (sv[0].iter(), sv[1].iter())
                        .mzip()
                        .map(|(li, ri)| self.block_pack_lut(ri, li, Lut1Def::SolveProp))
                        .intermediate()
                })
                .flatten()
                // We only take enough to build the new iterate.
                .take(chunk_nb)
                .collect();
            assert_eq!(chunk_gns.len(), chunk_nb);
        }

        // Step 5 =================================================================
        // Final resolution: Solve PGN status inside chunk
        // Convert back 2d array in vector
        let mut carries = (spread_pgns.into_iter(), chunk_gns.into_iter())
            .mzip()
            .slide::<2>()
            .filter(filter_out_postludes)
            .map_first(|slider| {
                let (spread_pgn, chunk_gn) = slider.unwrap_prelude()[0];
                [
                    spread_pgn[0],
                    self.block_pbs(&spread_pgn[1], Lut1Def::SolvePropGroupFinal0),
                    self.block_pbs(&spread_pgn[2], Lut1Def::SolvePropGroupFinal1),
                    chunk_gn,
                ]
            })
            .map_rest(|slider| {
                let sv = slider.unwrap_complete();
                let (_, prev_chunk_gn) = sv[0];
                let (spread_pgn, chunk_gn) = sv[1];
                let s0 = self.block_add(&spread_pgn[0], &prev_chunk_gn);
                let s0 = self.block_pbs(&s0, Lut1Def::SolvePropGroupFinal0);
                let s1 = self.block_add(&spread_pgn[1], &prev_chunk_gn);
                let s1 = self.block_pbs(&s1, Lut1Def::SolvePropGroupFinal1);
                let s2 = self.block_add(&spread_pgn[2], &prev_chunk_gn);
                let s2 = self.block_pbs(&s2, Lut1Def::SolvePropGroupFinal2);
                [s0, s1, s2, chunk_gn]
            })
            .flatten()
            .cosvec();
        carries.push(self.block_pbs2(&sums[0], Lut2Def::ManyCarryMsg).0);

        // Step 6 =================================================================
        // Carry is known now, propagate them in sum
        let result = (sums.into_iter(), carries.into_iter().skip(1))
            .mzip()
            .map(|(sum, carry)| self.block_add(&sum, &carry))
            .cosvec();

        Ciphertext {
            blocks: result,
            spec: self
                .spec()
                .ciphertext_spec(max(lhs.int_size(), rhs.int_size())),
        }
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
            ir.format(),
            r#"
            %0 : CtInt = input<0, CtInt>();
            %9 : CtInt = input<1, CtInt>();
            %41 : PtBlock = let_pt_block<1>();
            %65 : CtInt = zero_ct();
            %1 : CtBlock = extract_ct_block<0>(%0 : CtInt);
            %2 : CtBlock = extract_ct_block<1>(%0 : CtInt);
            %3 : CtBlock = extract_ct_block<2>(%0 : CtInt);
            %4 : CtBlock = extract_ct_block<3>(%0 : CtInt);
            %5 : CtBlock = extract_ct_block<4>(%0 : CtInt);
            %6 : CtBlock = extract_ct_block<5>(%0 : CtInt);
            %7 : CtBlock = extract_ct_block<6>(%0 : CtInt);
            %8 : CtBlock = extract_ct_block<7>(%0 : CtInt);
            %10 : CtBlock = extract_ct_block<0>(%9 : CtInt);
            %11 : CtBlock = extract_ct_block<1>(%9 : CtInt);
            %12 : CtBlock = extract_ct_block<2>(%9 : CtInt);
            %13 : CtBlock = extract_ct_block<3>(%9 : CtInt);
            %14 : CtBlock = extract_ct_block<4>(%9 : CtInt);
            %15 : CtBlock = extract_ct_block<5>(%9 : CtInt);
            %16 : CtBlock = extract_ct_block<6>(%9 : CtInt);
            %17 : CtBlock = extract_ct_block<7>(%9 : CtInt);
            %18 : CtBlock = add_ct(%1 : CtBlock, %10 : CtBlock);
            %19 : CtBlock = add_ct(%2 : CtBlock, %11 : CtBlock);
            %20 : CtBlock = add_ct(%3 : CtBlock, %12 : CtBlock);
            %21 : CtBlock = add_ct(%4 : CtBlock, %13 : CtBlock);
            %22 : CtBlock = add_ct(%5 : CtBlock, %14 : CtBlock);
            %23 : CtBlock = add_ct(%6 : CtBlock, %15 : CtBlock);
            %24 : CtBlock = add_ct(%7 : CtBlock, %16 : CtBlock);
            %25 : CtBlock = add_ct(%8 : CtBlock, %17 : CtBlock);
            %26 : CtBlock, %27 : CtBlock = pbs2<ManyCarryMsg>(%18 : CtBlock);
            %28 : CtBlock = pbs<ExtractPropGroup0>(%19 : CtBlock);
            %29 : CtBlock = pbs<ExtractPropGroup1>(%20 : CtBlock);
            %30 : CtBlock = pbs<ExtractPropGroup2>(%21 : CtBlock);
            %31 : CtBlock = pbs<ExtractPropGroup0>(%22 : CtBlock);
            %32 : CtBlock = pbs<ExtractPropGroup1>(%23 : CtBlock);
            %33 : CtBlock = pbs<ExtractPropGroup2>(%24 : CtBlock);
            %34 : CtBlock = pbs<ExtractPropGroup3>(%25 : CtBlock);
            %35 : CtBlock = add_ct(%27 : CtBlock, %28 : CtBlock);
            %39 : CtBlock = add_ct(%31 : CtBlock, %32 : CtBlock);
            %64 : CtBlock = add_ct(%25 : CtBlock, %26 : CtBlock);
            %36 : CtBlock = add_ct(%35 : CtBlock, %29 : CtBlock);
            %40 : CtBlock = add_ct(%39 : CtBlock, %33 : CtBlock);
            %47 : CtBlock = pbs<SolvePropGroupFinal0>(%35 : CtBlock);
            %37 : CtBlock = add_ct(%36 : CtBlock, %30 : CtBlock);
            %42 : CtBlock = add_ct(%40 : CtBlock, %34 : CtBlock);
            %48 : CtBlock = pbs<SolvePropGroupFinal1>(%36 : CtBlock);
            %57 : CtBlock = add_ct(%18 : CtBlock, %47 : CtBlock);
            %38 : CtBlock = pbs<SolvePropGroupFinal2>(%37 : CtBlock);
            %43 : CtBlock = pbs<ReduceCarryPad>(%42 : CtBlock);
            %58 : CtBlock = add_ct(%19 : CtBlock, %48 : CtBlock);
            %66 : CtInt = store_ct_block<0>(%57 : CtBlock, %65 : CtInt);
            %44 : CtBlock = add_pt(%43 : CtBlock, %41 : PtBlock);
            %49 : CtBlock = add_ct(%31 : CtBlock, %38 : CtBlock);
            %51 : CtBlock = add_ct(%39 : CtBlock, %38 : CtBlock);
            %53 : CtBlock = add_ct(%40 : CtBlock, %38 : CtBlock);
            %59 : CtBlock = add_ct(%20 : CtBlock, %38 : CtBlock);
            %67 : CtInt = store_ct_block<1>(%58 : CtBlock, %66 : CtInt);
            %45 : CtBlock = pack_ct<4>(%44 : CtBlock, %38 : CtBlock);
            %50 : CtBlock = pbs<SolvePropGroupFinal0>(%49 : CtBlock);
            %52 : CtBlock = pbs<SolvePropGroupFinal1>(%51 : CtBlock);
            %54 : CtBlock = pbs<SolvePropGroupFinal2>(%53 : CtBlock);
            %68 : CtInt = store_ct_block<2>(%59 : CtBlock, %67 : CtInt);
            %46 : CtBlock = pbs<SolvePropCarry>(%45 : CtBlock);
            %60 : CtBlock = add_ct(%21 : CtBlock, %50 : CtBlock);
            %61 : CtBlock = add_ct(%22 : CtBlock, %52 : CtBlock);
            %62 : CtBlock = add_ct(%23 : CtBlock, %54 : CtBlock);
            %63 : CtBlock = add_ct(%24 : CtBlock, %46 : CtBlock);
            %69 : CtInt = store_ct_block<3>(%60 : CtBlock, %68 : CtInt);
            %70 : CtInt = store_ct_block<4>(%61 : CtBlock, %69 : CtInt);
            %71 : CtInt = store_ct_block<5>(%62 : CtBlock, %70 : CtInt);
            %72 : CtInt = store_ct_block<6>(%63 : CtBlock, %71 : CtInt);
            %73 : CtInt = store_ct_block<7>(%64 : CtBlock, %72 : CtInt);
            output<0, CtInt>(%73 : CtInt);
        "#
        );
    }
}
