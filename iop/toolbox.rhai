// Utilitiles function could be define in script
// More evolved one could be defined on rust side and bind into the Rhai engine

/// Lazyly moved ciphertext to register
/// Do nothing if already in register
fn lazy_mv(maybe_in_reg) {
      switch type_of(maybe_in_reg) {
          "hpu_compiler::ir::args::MemCell" => load(maybe_in_reg),
          _ => maybe_in_reg
      };
}

/// Pack a vector of ct in pair
/// pack (digit[i+1], digit[i]) in one digit with:
/// * digit[i+1] in carry field
/// * digit[i] in msg field
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
/// ZipPack take two vector of digit `in_a` and `in_b`
/// Pack them in one vector where digit[i] contains:
/// * B[i] in carry field
/// * A[i] in msg field
fn zip_pack(in_a, in_b) {
    let pack_shift = imm_cst(4);

    zip(in_a, in_b).map(|pair| {
        if pair.len() != 2 {
            throw "Error in zip_pack. Couldn't pack vector of != length"
        } else {
            let lsb = lazy_mv(pair[0]);
            let msb = lazy_mv(pair[1]);
            mac(lsb, msb, pack_shift)
        }
    })
}

/// Clean noise on a vector of digit
/// Some lib-operation could output noisy ciphertext
/// This function enforce that all digit are clean and composed only of msg field
fn clean_ct(src) {
    let pbs_msg_only = pbs_lut(
        ```|x| {
            let ctx = get_context;
            x & ((1<< ctx.msg_w) -1)
        }```,
        ```|x| {
            let ctx = get_context;
            ((1<< ctx.msg_w) -1)
        }```);
    src.map(|x| pbs(x, pbs_msg_only))
}

/// Bitwise inversion
/// Usefull to convert a value in its opposite
 fn bw_inv(src) {
     let ctx = get_context();
     let inv = (0..ctx.blk_w).collect().map(|_x| imm_cst((1 << ctx.msg_w) -1));

     zip(inv, src).map(|pair| {
        let inv_src = lazy_mv(pair[0]);
        if pair.len() == 2 {
            let val = lazy_mv(pair[1]);
            inv_src = ssub(inv_src, val);
        } 
        inv_src
    })
 }
