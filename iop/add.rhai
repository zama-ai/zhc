// Import common user function shared between Op
import "iop/toolbox" as tb;
import "iop/adder" as adder;

// Must be retrieved from engine
// like get_properties
let integer_w = 16;
let msg_w= 2;
let carry_w= 2;
let nu= 5;

let src_a = input_vars(0, integer_w);
let src_b = input_vars(1,integer_w);
let dst = output_vars(0, integer_w);

let res = adder::add_hs_raw(src_a, src_b);

zip(res, dst).map(|zip| store(zip[0], zip[1]));
