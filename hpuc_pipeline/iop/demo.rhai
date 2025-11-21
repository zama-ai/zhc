// Import common user function shared between Op
import "iop/toolbox" as tb;

// Retrieved current generation context
let context = get_context();

// Get input as Array of blk
let src_a = tb::input_array(0);
let src_b = tb::input_array(1);

// Create required PbsLut
let lut_shuffle = pbs_lut("Shuffle", 1);

// Pack A&B in same vector
let packed = tb::zip_pack(src_a, src_b);

// Apply Pbs on each block
let shuffle = packed.map(|x| pbs(x, lut_shuffle));

// Store block in output
tb::output_array(shuffle, 0);
