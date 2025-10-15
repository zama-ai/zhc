// Utilitiles function could be define in script
// More evolved one could be defined on rust side and bind into the Rhai engine

/// Lazyly moved ciphertext to register
/// Do nothing if already in register
fn lazy_mv(maybe_in_reg) {
      switch type_of(maybe_in_reg) {
          "hpuic::ir::args::MemCell" => load(maybe_in_reg),
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



