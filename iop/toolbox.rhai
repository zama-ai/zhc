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

