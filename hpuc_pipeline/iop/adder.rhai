/// Implement various adder function based on State-of-the-art
/// Various names across literature but principle is common: // prefix form of carry-lookahead adder.
///  
/// Those function aims are to be easily reused
import "toolbox" as tb;


/// Implement hillisSteel Adder
/// Function is tagged as raw since output could be more noisy than a clear Ciphertext
/// Noise could be at most 2 additions away from input
fn add_hs_raw(src_a, src_b) {

    // Define set of requiered PBS
    let pbs_ml2_carry_msg = pbs_lut("ManyCarryMsg", 2);
    let pbs_msg_only = pbs_lut("MsgOnly", 1);
    let pbs_reduce_carry_pad = pbs_lut("ReduceCarryPad", 1);
    let pbs_solve_prop = pbs_lut("SolveProp", 1);
    let pbs_solve_prop_carry = pbs_lut("SolvePropCarry", 1);
    let pbs_solve_prop_group_final0 = pbs_lut("SolvePropGroupFinal0", 1);
    let pbs_solve_prop_group_final1 = pbs_lut("SolvePropGroupFinal1", 1);
    let pbs_solve_prop_group_final2 = pbs_lut("SolvePropGroupFinal2", 1);
    let pbs_extract_prop_group_0 = pbs_lut("ExtractPropGroup0", 1);
    let pbs_extract_prop_group_1 = pbs_lut("ExtractPropGroup1", 1);
    let pbs_extract_prop_group_2 = pbs_lut("ExtractPropGroup2", 1);
    let pbs_extract_prop_group_3 = pbs_lut("ExtractPropGroup3", 1);

    // Define Pbs groups
    let pbs_solve_prop_group_final_v = [
        pbs_solve_prop_group_final0,
        pbs_solve_prop_group_final1,
        pbs_solve_prop_group_final2,
    ];
    let pbs_extract_prop_group_v = [
        pbs_extract_prop_group_0,
        pbs_extract_prop_group_1,
        pbs_extract_prop_group_2,
        pbs_extract_prop_group_3,
    ];

    // Step 1 =================================================================
    // Add blocks together, no fancy stuff here. Just pack two input vector (with msg only)
    // into one vector with msg + [1b of carry]
    // Also handle src of different sizes.
    let long_src;
    let short_src;
    let add_size = if src_a.len() < src_b.len() {
        long_src = src_b;
        short_src = src_a;
        src_a.len()
    } else {
        long_src = src_a;
        short_src = src_b;
        src_b.len()
    };

    let sum_a = zip(long_src,short_src).map(|zip| {
        zip[0] + zip[1]
    });
    for i in add_size..long_src.len() {
        sum_a += long_src[i];
    }

    // Step 2 =================================================================
    // Compute P/G/N state for each block.
    // States are encoded with tfhe-rs flag with a linear merging of 4 blocks in mind
    // P => 0b01 << (block_id % 4)
    // N => 0b00 << (block_id % 4)
    // G => 0b10 << (block_id % 4)
    // 
    // NB: LSB bloc is handle differently to reduce the work
    //     -> input carry is known, so it could be directly resolved
    let msg0;
    let sum_2da = sum_a.chunks_by(4);

    let pgn_chunk_2da = [];
    for item in enumerate(sum_2da) { // Iterate over chunk
        let idx = item.index;
        let chunk = item.value;

        let pgn_chunk = [];
        if 0 == idx { // First chunk direct resolution
            // [0] carry is already known.
            let res_ml = pbs_ml(chunk[0], pbs_ml2_carry_msg);
            msg0 = res_ml[0];
            pgn_chunk += res_ml[1];

            for i in 1..4 {
                pgn_chunk += pbs(chunk[i], pbs_extract_prop_group_v[i - 1])
            }
        } else {
            for i in 0..4 { // Iterate inside chunk
                pgn_chunk += pbs(chunk[i], pbs_extract_prop_group_v[i])
            }
        }
   
        pgn_chunk_2da.push(pgn_chunk);
    }
   
    // Step 3 =================================================================
    // Step 3.a ===========================================
    // Spread propagation status *within* each chunk
    // pgn_chunk [0,1,2] contains the sum of the propagation status at each step
    // pgn_chunk[3] contains the propagation status of the chunk
    // 
    // NB: chunk #0 is particular, since the status is actually
    // the carry value => This chunk is directly solved
    let spgn_chunk_2da = [];

    for item in enumerate(pgn_chunk_2da) { // Iterate over chunk
        let idx = item.index;
        let chunk = item.value;

        let spgn_chunk = [];

        // [0] low border of the chunk
        spgn_chunk += chunk[0];
   
        // [1,2] middle of the chunk => partial sum will be processed later
        for i in 1..=2 {
          spgn_chunk += chunk[i] + spgn_chunk[i-1];
        }

        // [3] high border of the chunk => Sum & Reduce Carry
        let last_sum = chunk[3] + spgn_chunk[2];
        spgn_chunk += if 0 == idx { // First chunk direct resolution
             pbs(last_sum, pbs_solve_prop_group_final_v[3 - 1])
        } else { // Reduce carry
             pbs(last_sum, pbs_reduce_carry_pad) + pt_cst(1)
        };
   
        spgn_chunk_2da.push(spgn_chunk);
    }

    // Step 3.b ===========================================
    // Resolve PGN status across each group with parallel scan (based on Hillis-Steel algorithm)
    // I.e. solve PGN into GN for each chunk => Will end with a carry info for each group
    let chunk_nb = spgn_chunk_2da.len();
    let stage_nb = clog2(chunk_nb);
    for stage in 0..stage_nb {
        let stride = 1 << stage;

       let solved_msb = [];
        for idx in range(stride..chunk_nb, stride) {
            for k in 0..stride {
                let cur_idx = idx + k;
                if cur_idx < chunk_nb { 
                    let neighbours = spgn_chunk_2da[cur_idx - stride][3];
                    let current = spgn_chunk_2da[cur_idx][3];
                    let packed = tb::pack([current, neighbours])[0];

                    solved_msb += if  idx == stride { // Solve carry
                        pbs(packed, pbs_solve_prop_carry)
                    } else {
                        pbs(packed, pbs_solve_prop)
                    };
                } 
            }
        }
        // Writeback solved_msb
        for idx in 0..solved_msb.len() {
            spgn_chunk_2da[stride + idx][3] = solved_msb[idx];
        }
    }

    // Step 5 =================================================================
    // Final resolution: Solve PGN status inside chunk
    // Convert back 2d array in vector
    let carry_a = [];
    // First chunk is handle differently
    for idx in 0..3 {
        let status = spgn_chunk_2da[0][idx];
        let carry = switch idx {
            0 | 3 => status, // Already solved
            1 | 2 => pbs(status, pbs_solve_prop_group_final_v[idx -1]),
        };
        carry_a += carry;
    }
    // Other chunk {
    for item in enumerate(spgn_chunk_2da.extract(1)) { // NB: equivalent to open range 1..
       let prv_cidx = item.index; // Enumerate is based on 1.. range thus already shifted by 1
       let chunk  = item.value;

        for idx in 0..3 {
            let carry = switch idx {
                3 => chunk[idx], // Already solved
                0| 1 | 2 => {
                    let combined = chunk[idx] + spgn_chunk_2da[prv_cidx][3];
                    pbs(combined, pbs_solve_prop_group_final_v[idx -1])
                },
            };
            carry_a += carry;
        }
    }

    // Step 6 =================================================================
    // Carry is known now, propagate them in sum
    let res = [msg0];
    for zip in zip(sum_a.extract(1), carry_a) {
        res += zip[0] + zip[1]
    }
    res
}
