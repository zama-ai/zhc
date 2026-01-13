// Import common user function shared between Op
import "iop/toolbox" as tb;
import "iop/adder" as adder;


// Retrieved current generation context
let context = get_context();

// Get input as Array of blk
let src_a = tb::input_array(0);
let src_b = tb::input_array(1);

// Sub is a + -b
let inv_b = tb::bw_inv(src_b);
let res_raw = adder::add_hs_raw(src_a, inv_b);

// clean noise
let res = tb::clean_ct(res_raw);

tb::output_array(res, 0);
