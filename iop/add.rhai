// Import common user function shared between Op
import "iop/toolbox" as tb;
import "iop/adder" as adder;

// Retrieved current generation context
let context = get_context();

// Get input as Array of blk
let src_a = tb::input_array(0);
let src_b = tb::input_array(1);

// Use HillisSteel algorithm
let res = adder::add_hs_raw(src_a, src_b);

tb::output_array(res, 0);
