use std::cmp::max;

use hc_ir::IR;
use hc_langs::ioplang::Ioplang;
use hc_utils::iter::{ChunkIt, CollectInSmallVec, Intermediate, IterMapFirst, MultiZip, Slide, filter_out_postludes};

use crate::builder::{
    BlockConfig, Builder, EncryptedInteger, ExtensionBehavior, Lut1Type, Lut2Type,
};

pub fn add(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    // Add 2 integers, using the Hillis Steel method.
    // Outputs a list containing the resulting blocks.
    // clean_ct : option to clean the ct at the output.
    // Note that if the ct is not cleaned, its noise
    // has increased by 2 additions, compared to the input.

    let mut builder = Builder::new(config);
    let src_a = builder.eint_input(width);
    let src_b = builder.eint_input(width);
    let res = builder.iop_add_hillis_steele(src_a, src_b);
    builder.eint_output(res);
    builder.into_ir()
}

impl Builder {
    pub fn iop_add_hillis_steele(
        &mut self,
        lhs: EncryptedInteger,
        rhs: EncryptedInteger,
    ) -> EncryptedInteger {
        // We declare the luts.
        let lut_many_carry_msg = self.lut2(Lut2Type::ManyCarryMsg);
        let lut_reduce_carry_pad = self.lut(Lut1Type::ReduceCarryPad);
        let lut_solve_prop_group_final0 = self.lut(Lut1Type::SolvePropGroupFinal0);
        let lut_solve_prop_group_final1 = self.lut(Lut1Type::SolvePropGroupFinal1);
        let lut_solve_prop_group_final2 = self.lut(Lut1Type::SolvePropGroupFinal2);
        let lut_extract_prop_group_0 = self.lut(Lut1Type::ExtractPropGroup0);
        let lut_extract_prop_group_1 = self.lut(Lut1Type::ExtractPropGroup1);
        let lut_extract_prop_group_2 = self.lut(Lut1Type::ExtractPropGroup2);
        let lut_extract_prop_group_3 = self.lut(Lut1Type::ExtractPropGroup3);
        let lut_solve_prop_carry = self.lut(Lut1Type::SolvePropCarry);
        let lut_solve_prop = self.lut(Lut1Type::SolveProp);

        // Step 1 =================================================================
        // Add blocks together, no fancy stuff here. Just pack two input vector (with msg only)
        // into one vector with msg + [1b of carry]
        // Also handle src of different sizes.
        let sums = self.vector_add(&lhs, &rhs, ExtensionBehavior::Passthrough);
        assert!(sums.len().is_multiple_of(4), "Addition for non-multiple-of-4 integers is not supported yet.");

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
                    self.block_pbs2(&chunk[0], &lut_many_carry_msg).1,
                    self.block_pbs(&sums[1], &lut_extract_prop_group_0),
                    self.block_pbs(&sums[2], &lut_extract_prop_group_1),
                    self.block_pbs(&sums[3], &lut_extract_prop_group_2),
                ]
            })
            .map_rest(|chunk| {
                [
                    self.block_pbs(chunk[0], &lut_extract_prop_group_0),
                    self.block_pbs(chunk[1], &lut_extract_prop_group_1),
                    self.block_pbs(chunk[2], &lut_extract_prop_group_2),
                    self.block_pbs(chunk[3], &lut_extract_prop_group_3),
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
                let s3 = self.block_pbs(&_s3, &lut_solve_prop_group_final2);
                [s0, s1, s2, s3]
            })
            .map_rest(|chunk| {
                let s0 = chunk[0];
                let s1 = self.block_add(&s0, &chunk[1]);
                let s2 = self.block_add(&s1, &chunk[2]);
                let cst_1 = self.block_constant(1);
                let _s3 = self.block_add(&s2, &chunk[3]);
                let _s3 = self.block_pbs(&_s3, &lut_reduce_carry_pad);
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
                        .map(|(li, ri)| self.block_pack_lut(ri, li, &lut_solve_prop_carry))
                        .intermediate()
                })
                // The rest of the chunks combine chunks of the previous stage with the prop lut.
                .map_rest(|slider| {
                    let sv = slider.unwrap_complete();
                    assert_eq!(sv[0].len(), sv[1].len());
                    (sv[0].iter(), sv[1].iter())
                        .mzip()
                        .map(|(li, ri)| self.block_pack_lut(ri, li, &lut_solve_prop))
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
                    self.block_pbs(&spread_pgn[1], &lut_solve_prop_group_final0),
                    self.block_pbs(&spread_pgn[2], &lut_solve_prop_group_final1),
                    chunk_gn,
                ]
            })
            .map_rest(|slider| {
                let sv = slider.unwrap_complete();
                let (_, prev_chunk_gn) = sv[0];
                let (spread_pgn, chunk_gn) = sv[1];
                let s0 = self.block_add(&spread_pgn[0], &prev_chunk_gn);
                let s0 = self.block_pbs(&s0, &lut_solve_prop_group_final0);
                let s1 = self.block_add(&spread_pgn[1], &prev_chunk_gn);
                let s1 = self.block_pbs(&s1, &lut_solve_prop_group_final1);
                let s2 = self.block_add(&spread_pgn[2], &prev_chunk_gn);
                let s2 = self.block_pbs(&s2, &lut_solve_prop_group_final2);
                [s0, s1, s2, chunk_gn]
            })
            .flatten()
            .cosvec();
        carries.push(self.block_pbs2(&sums[0], &lut_many_carry_msg).0);

        // Step 6 =================================================================
        // Carry is known now, propagate them in sum
        let result = (sums.into_iter(), carries.into_iter().skip(1))
            .mzip()
            .map(|(sum, carry)| self.block_add(&sum, &carry))
            .cosvec();

        EncryptedInteger {
            blocks: result,
            width: max(lhs.width(), rhs.width()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{builder::BlockConfig, iops::add::add};

    #[test]
    fn test_add() {
        let config = BlockConfig {
            message_width: 2,
            carry_width: 2,
        };
        let ir = add(16, &config);
        ir.check_ir(
            "
            %0 : Ciphertext = input<0, Ciphertext>();
            %1 : Index = constant<0_idx>();
            %2 : Index = constant<1_idx>();
            %3 : Index = constant<2_idx>();
            %4 : Index = constant<3_idx>();
            %5 : Index = constant<4_idx>();
            %6 : Index = constant<5_idx>();
            %7 : Index = constant<6_idx>();
            %8 : Index = constant<7_idx>();
            %9 : Ciphertext = input<1, Ciphertext>();
            %18 : Lut2 = gen_lut2<ManyCarryMsg>();
            %19 : Lut1 = gen_lut1<ReduceCarryPad>();
            %20 : Lut1 = gen_lut1<SolvePropGroupFinal0>();
            %21 : Lut1 = gen_lut1<SolvePropGroupFinal1>();
            %22 : Lut1 = gen_lut1<SolvePropGroupFinal2>();
            %23 : Lut1 = gen_lut1<ExtractPropGroup0>();
            %24 : Lut1 = gen_lut1<ExtractPropGroup1>();
            %25 : Lut1 = gen_lut1<ExtractPropGroup2>();
            %26 : Lut1 = gen_lut1<ExtractPropGroup3>();
            %27 : Lut1 = gen_lut1<SolvePropCarry>();
            %29 : PlaintextBlock = constant<1_pt_block>();
            %30 : PlaintextBlock = constant<4_pt_block>();
            %31 : Ciphertext = let<Ciphertext>();
            %40 : CiphertextBlock = extract_ct_block(%0, %1);
            %41 : CiphertextBlock = extract_ct_block(%0, %2);
            %42 : CiphertextBlock = extract_ct_block(%0, %3);
            %43 : CiphertextBlock = extract_ct_block(%0, %4);
            %44 : CiphertextBlock = extract_ct_block(%0, %5);
            %45 : CiphertextBlock = extract_ct_block(%0, %6);
            %46 : CiphertextBlock = extract_ct_block(%0, %7);
            %47 : CiphertextBlock = extract_ct_block(%0, %8);
            %48 : CiphertextBlock = extract_ct_block(%9, %1);
            %49 : CiphertextBlock = extract_ct_block(%9, %2);
            %50 : CiphertextBlock = extract_ct_block(%9, %3);
            %51 : CiphertextBlock = extract_ct_block(%9, %4);
            %52 : CiphertextBlock = extract_ct_block(%9, %5);
            %53 : CiphertextBlock = extract_ct_block(%9, %6);
            %54 : CiphertextBlock = extract_ct_block(%9, %7);
            %55 : CiphertextBlock = extract_ct_block(%9, %8);
            %56 : CiphertextBlock = add_ct(%40, %48);
            %57 : CiphertextBlock = add_ct(%41, %49);
            %58 : CiphertextBlock = add_ct(%42, %50);
            %59 : CiphertextBlock = add_ct(%43, %51);
            %60 : CiphertextBlock = add_ct(%44, %52);
            %61 : CiphertextBlock = add_ct(%45, %53);
            %62 : CiphertextBlock = add_ct(%46, %54);
            %63 : CiphertextBlock = add_ct(%47, %55);
            %64 : CiphertextBlock, %65 : CiphertextBlock = pbs2(%56, %18);
            %66 : CiphertextBlock = pbs(%57, %23);
            %67 : CiphertextBlock = pbs(%58, %24);
            %68 : CiphertextBlock = pbs(%59, %25);
            %69 : CiphertextBlock = pbs(%60, %23);
            %70 : CiphertextBlock = pbs(%61, %24);
            %71 : CiphertextBlock = pbs(%62, %25);
            %72 : CiphertextBlock = pbs(%63, %26);
            %75 : CiphertextBlock = add_ct(%65, %66);
            %76 : CiphertextBlock = add_ct(%69, %70);
            %77 : CiphertextBlock = add_ct(%63, %64);
            %78 : CiphertextBlock = add_ct(%75, %67);
            %79 : CiphertextBlock = add_ct(%76, %71);
            %80 : CiphertextBlock = pbs(%75, %20);
            %81 : CiphertextBlock = add_ct(%78, %68);
            %82 : CiphertextBlock = add_ct(%79, %72);
            %83 : CiphertextBlock = pbs(%78, %21);
            %84 : CiphertextBlock = add_ct(%56, %80);
            %85 : CiphertextBlock = pbs(%81, %22);
            %86 : CiphertextBlock = pbs(%82, %19);
            %87 : CiphertextBlock = add_ct(%57, %83);
            %88 : Ciphertext = store_ct_block(%84, %31, %1);
            %89 : CiphertextBlock = add_pt(%86, %29);
            %90 : CiphertextBlock = add_ct(%69, %85);
            %91 : CiphertextBlock = add_ct(%76, %85);
            %92 : CiphertextBlock = add_ct(%79, %85);
            %93 : CiphertextBlock = add_ct(%58, %85);
            %94 : Ciphertext = store_ct_block(%87, %88, %2);
            %95 : CiphertextBlock = mac(%30, %89, %85);
            %96 : CiphertextBlock = pbs(%90, %20);
            %97 : CiphertextBlock = pbs(%91, %21);
            %98 : CiphertextBlock = pbs(%92, %22);
            %99 : Ciphertext = store_ct_block(%93, %94, %3);
            %100 : CiphertextBlock = pbs(%95, %27);
            %101 : CiphertextBlock = add_ct(%59, %96);
            %102 : CiphertextBlock = add_ct(%60, %97);
            %103 : CiphertextBlock = add_ct(%61, %98);
            %104 : CiphertextBlock = add_ct(%62, %100);
            %105 : Ciphertext = store_ct_block(%101, %99, %4);
            %106 : Ciphertext = store_ct_block(%102, %105, %5);
            %107 : Ciphertext = store_ct_block(%103, %106, %6);
            %108 : Ciphertext = store_ct_block(%104, %107, %7);
            %109 : Ciphertext = store_ct_block(%77, %108, %8);
            output<0, Ciphertext>(%109);
        ",
        );
    }
}
