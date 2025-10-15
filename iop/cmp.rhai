// Import common user function shared between Op
import "iop/toolbox" as tb;

// Retrieved current generation context
let context = get_context();

let src_a = input_vars(0, context.integer_w);
let src_b = input_vars(1, context.integer_w);
let dst = output_vars(0, 1); // Boolean output

let pbs_none = pbs_lut("|x| x", "|x| x");
// let cmp_sign = pbs_lut("CmpSign");
// let cmp_reduce = pbs_lut("CmpSign");
// let user_mrg = pbs_lut("UserMrg");
// let user_cmp = pbs_lut("UserCmp");
let cmp_sign = pbs_lut("|x| if x != 0 {1} else {0}",
                       "|x| 1");
let cmp_reduce = pbs_lut("|x| todo",
                       "|x| 2");
let user_mrg = pbs_lut("|x| x", "|x| x");
let user_cmp = pbs_lut("|x| x", "|x| x");

// Pack A by pairs
let packed_a = tb::pack(src_a);
// Pack B by pairs
let packed_b = tb::pack(src_b);

// Merge A /B and get sign
let merged = zip(packed_a, packed_b).map(|v| pbs(v[0]-v[1], cmp_sign));

// Reduce (tree-based reduce
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

store(cmp_res, dst[0]);
