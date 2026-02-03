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

    #[test]
    fn test_add() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = add(spec);
        ir.check_ir(
            "
            %0 : CtInt = input<0, CtInt>();
            %1 : CtInt = input<1, CtInt>();
            %2 : PtBlock = let_pt_block<1>();
            %3 : CtInt = zero_ct();
            %4 : CtBlock = extract_ct_block<0>(%0);
            %5 : CtBlock = extract_ct_block<1>(%0);
            %6 : CtBlock = extract_ct_block<2>(%0);
            %7 : CtBlock = extract_ct_block<3>(%0);
            %8 : CtBlock = extract_ct_block<4>(%0);
            %9 : CtBlock = extract_ct_block<5>(%0);
            %10 : CtBlock = extract_ct_block<6>(%0);
            %11 : CtBlock = extract_ct_block<7>(%0);
            %12 : CtBlock = extract_ct_block<0>(%1);
            %13 : CtBlock = extract_ct_block<1>(%1);
            %14 : CtBlock = extract_ct_block<2>(%1);
            %15 : CtBlock = extract_ct_block<3>(%1);
            %16 : CtBlock = extract_ct_block<4>(%1);
            %17 : CtBlock = extract_ct_block<5>(%1);
            %18 : CtBlock = extract_ct_block<6>(%1);
            %19 : CtBlock = extract_ct_block<7>(%1);
            %20 : CtBlock = add_ct(%4, %12);
            %21 : CtBlock = add_ct(%5, %13);
            %22 : CtBlock = add_ct(%6, %14);
            %23 : CtBlock = add_ct(%7, %15);
            %24 : CtBlock = add_ct(%8, %16);
            %25 : CtBlock = add_ct(%9, %17);
            %26 : CtBlock = add_ct(%10, %18);
            %27 : CtBlock = add_ct(%11, %19);
            %28 : CtBlock, %29 : CtBlock = pbs2<ManyCarryMsg>(%20);
            %30 : CtBlock = pbs<ExtractPropGroup0>(%21);
            %31 : CtBlock = pbs<ExtractPropGroup1>(%22);
            %32 : CtBlock = pbs<ExtractPropGroup2>(%23);
            %33 : CtBlock = pbs<ExtractPropGroup0>(%24);
            %34 : CtBlock = pbs<ExtractPropGroup1>(%25);
            %35 : CtBlock = pbs<ExtractPropGroup2>(%26);
            %36 : CtBlock = pbs<ExtractPropGroup3>(%27);
            %39 : CtBlock = add_ct(%29, %30);
            %40 : CtBlock = add_ct(%33, %34);
            %41 : CtBlock = add_ct(%27, %28);
            %42 : CtBlock = add_ct(%39, %31);
            %43 : CtBlock = add_ct(%40, %35);
            %44 : CtBlock = pbs<SolvePropGroupFinal0>(%39);
            %45 : CtBlock = add_ct(%42, %32);
            %46 : CtBlock = add_ct(%43, %36);
            %47 : CtBlock = pbs<SolvePropGroupFinal1>(%42);
            %48 : CtBlock = add_ct(%20, %44);
            %49 : CtBlock = pbs<SolvePropGroupFinal2>(%45);
            %50 : CtBlock = pbs<ReduceCarryPad>(%46);
            %51 : CtBlock = add_ct(%21, %47);
            %52 : CtInt = store_ct_block<0>(%48, %3);
            %53 : CtBlock = add_pt(%50, %2);
            %54 : CtBlock = add_ct(%33, %49);
            %55 : CtBlock = add_ct(%40, %49);
            %56 : CtBlock = add_ct(%43, %49);
            %57 : CtBlock = add_ct(%22, %49);
            %58 : CtInt = store_ct_block<1>(%51, %52);
            %59 : CtBlock = pack_ct<4>(%53, %49);
            %60 : CtBlock = pbs<SolvePropGroupFinal0>(%54);
            %61 : CtBlock = pbs<SolvePropGroupFinal1>(%55);
            %62 : CtBlock = pbs<SolvePropGroupFinal2>(%56);
            %63 : CtInt = store_ct_block<2>(%57, %58);
            %64 : CtBlock = pbs<SolvePropCarry>(%59);
            %65 : CtBlock = add_ct(%23, %60);
            %66 : CtBlock = add_ct(%24, %61);
            %67 : CtBlock = add_ct(%25, %62);
            %68 : CtBlock = add_ct(%26, %64);
            %69 : CtInt = store_ct_block<3>(%65, %63);
            %70 : CtInt = store_ct_block<4>(%66, %69);
            %71 : CtInt = store_ct_block<5>(%67, %70);
            %72 : CtInt = store_ct_block<6>(%68, %71);
            %73 : CtInt = store_ct_block<7>(%41, %72);
            output<0, CtInt>(%73);
        ",
        );
    }
}
