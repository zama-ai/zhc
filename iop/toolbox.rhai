// Utilitiles function could be define in script
// More evolved one could be defined on rust side and bind into the Rhai engine

/// View input as Array of blocks
fn input_array(index){
    let context = get_context();
    let input = input(index);
    (0..context.blk_nb).collect().map(|i| load(input, i))
}

/// Store one blk in output index at position
fn output_blk(blk, index, pos){
    let acc = ct_var();
    store(blk, acc, pos);
    output(acc, index);
}

/// Store array of blk in output
fn output_array(blka, index){
    let acc = ct_var();

    for item in enumerate(blka) { // Iterate over blk array
        let idx = item.index;
        let blk = item.value;
        store(blk, acc, idx)
    }
    output(output, index)
}

/// Pack a vector of ct in pair
/// pack (digit[i+1], digit[i]) in one digit with:
/// * digit[i+1] in carry field
/// * digit[i] in msg field
fn pack(unpack) {
    let pack_shift = pt_cst(4);
    unpack.chunks_by(2).map(|pair| {
        let lsb = pair[0];
        if pair.len() == 2 {
            let msb = pair[1];
            mac(pack_shift, msb, lsb)
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
    let pack_shift = pt_cst(4);

    zip(in_a, in_b).map(|pair| {
        if pair.len() != 2 {
            throw "Error in zip_pack. Couldn't pack vector of != length"
        } else {
            let lsb = pair[0];
            let msb = pair[1];
            mac(pack_shift, msb, lsb)
        }
    })
}

/// 

/// Clean noise on a vector of digit
/// Some lib-operation could output noisy ciphertext
/// This function enforce that all digit are clean and composed only of msg field
fn clean_ct(src) {
    // let pbs_msg_only = pbs_lut(
    //     ```|x| {
    //         let ctx = get_context;
    //         x & ((1<< ctx.msg_w) -1)
    //     }```,
    //     ```|x| {
    //         let ctx = get_context;
    //         ((1<< ctx.msg_w) -1)
    //     }```);
    let pbs_msg_only = pbs_lut("MsgOnly", 1);
    src.map(|x| pbs(x, pbs_msg_only))
}

/// Bitwise inversion
/// Usefull to convert a value in its opposite
 fn bw_inv(src) {
     let ctx = get_context();
     let inv = (0..ctx.blk_nb).collect().map(|_x| pt_cst((1 << ctx.msg_w) -1));

     zip(inv, src).map(|pair| {
        let inv_src = pair[0];
        if pair.len() == 2 {
            inv_src = ssub(inv_src, pair[1]);
        } 
        inv_src
    })
 }
