// Import common user function shared between Op
import "iop/toolbox" as tb;
import "iop/adder" as adder;

// Retrieved current generation context
let context = get_context();

let src_a = input_vars(0, context.integer_w);
let src_b = input_vars(1, context.integer_w);
let dst = output_vars(0,  context.integer_w);

let res = adder::add_hs_raw(src_a, src_b);

zip(res, dst).map(|zip| store(zip[0], zip[1]));
