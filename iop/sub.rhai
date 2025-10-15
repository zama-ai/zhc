// Import common user function shared between Op
import "iop/toolbox" as tb;
import "iop/adder" as adder;


// Retrieved current generation context
let context = get_context();

let src_a = input_vars(0, context.integer_w);
let src_b = input_vars(1, context.integer_w);
let dst = output_vars(0,  context.integer_w);

// Sub is a + -b
let inv_b = tb::bw_inv(src_b);
let res_raw = adder::add_hs_raw(src_a, inv_b);

// clean noise
let res = tb::clean_ct(res_raw);

// Store in destination variable
zip(res, dst).map(|zip| store(zip[0], zip[1]));
