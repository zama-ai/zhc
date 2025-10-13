let src_a = input_vars(0, 8);
let src_b = input_vars(1, 8);
let dst = output_vars(0, 8);

let pack = imm_cst(4);
let msg_only = pbs_lut("|x| [ x &0x3]", "|x| 2");

for i in 0..4 {
 let a = load(src_a[i]);
 let b = load(src_b[i]);
 let c = mac(a, b, pack);
 let d = pbs(c, msg_only);
 store(d, dst[i])
}
