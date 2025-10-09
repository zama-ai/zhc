// Utilitiles function could be define in script
// More evolved one could be defined on rust side and bind into the Rhai engine
fn lazy_mv(maybe_in_reg) {
      switch type_of(maybe_in_reg) {
          "hpuic::ir::args::MemCell" => load(maybe_in_reg),
          _ => maybe_in_reg
      };
}

fn pack(unpack) {
    let pack_shift = imm_cst(4);

    // => For-loop version
    // for pair in unpack.chunks_by(2) {
    //   let lsb = lazy_mv(pair[0]);
    //   let out = if pair.len() == 2 {
    //       let msb = lazy_mv(pair[1]);
    //       mac(lsb, msb, pack_shift)
    //   } else {
    //   lsb
    //   }
    // })
    // => map version
    unpack.chunks_by(2).map(|pair| {
      let lsb = lazy_mv(pair[0]);
      if pair.len() == 2 {
          let msb = lazy_mv(pair[1]);
          mac(lsb, msb, pack_shift)
      } else {
      lsb
      }
    })
}

// Must be retrieved from engine
// like get_properties
let integer_w = 16;
let msg_w= 2;
let carry_w= 2;
let nu= 5;

let src_a = input_vars(0, integer_w);
let src_b = input_vars(1,integer_w);
let dst = output_vars(0, 1); // Boolean output

let pbs_none = pbs_lut("None");
let cmp_sign = pbs_lut("CmpSign");
let cmp_reduce = pbs_lut("CmpSign");
let user_mrg = pbs_lut("UserMrg");
let user_cmp = pbs_lut("UserCmp");

// Pack A by pairs
let packed_a = pack(src_a);
// Pack B by pairs
let packed_b = pack(src_b);

// Merge A /B and get sign
let merged = zip(packed_a, packed_b).map(|v| pbs(v[0]-v[1], cmp_sign));

// Reduce (tree-based reduce
while merged.len() > 2 {
    let packed = pack(merged);
    let reduced = packed.map(|x| pbs(x, cmp_reduce));

    // Prepare next iter
    merged = reduced;
}

// Last reduce and cast based on user required cmp
let cmp_res = switch merged.len() {
 2 => pbs(pack(merged)[0], user_mrg),
 1 => pbs(merged[0], user_cmp),
 _ => print(`Error, check your algorithm`)
};

store(cmp_res, dst[0]);
