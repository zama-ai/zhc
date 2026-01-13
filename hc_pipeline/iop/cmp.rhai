// Import common user function shared between Op
import "iop/toolbox" as tb;

// Retrieved current generation context
let context = get_context();

// Get input as Array of blk
let src_a = tb::input_array(0);
let src_b = tb::input_array(1);

// Create required PbsLut
let cmp_sign = pbs_lut("CmpSign", 1);
let cmp_reduce = pbs_lut("CmpReduce", 1);
let user_mrg = pbs_lut("CmpGtMrg", 1);
let user_cmp = pbs_lut("CmpGt", 1);

// Pack A by pairs
let packed_a = tb::pack(src_a);
// Pack B by pairs
let packed_b = tb::pack(src_b);

// Merge A /B and get sign
let merged = zip(packed_a, packed_b).map(|v| {
    pbs(v[0]-v[1], cmp_sign)
    // pbs(sub(v[0],v[1]), cmp_sign)
    });

// Reduce (tree-based reduce)
while merged.len() > 2 {
    let packed = tb::pack(merged);
    let reduced = packed.map(|x| pbs(x, cmp_reduce));

    // Prepare next iter
    merged = reduced;
}

// Last reduce and cast based on user required cmp
let cmp_res = switch merged.len() {
 2 => pbs(tb::pack(merged)[0], user_mrg),
 1 => pbs(merged[0], user_cmp),
 _ => print(`Error, check your algorithm`)
};

// Store result (boolean) in slot 0 of output 0
tb::output_blk(cmp_res, 0, 0);
