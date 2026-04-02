use zhc_crypto::integer_semantics::{CiphertextBlockSpec};
use zhc_langs::ioplang::Lut1Def;
use zhc_utils::Dumpable;

use crate::Builder;

#[test]
fn custom_lut_1_def() {
    let builder = Builder::new(CiphertextBlockSpec(2, 2));
    let val = builder.block_let_ciphertext(0);
    let val = builder.block_lookup(val, Lut1Def::custom("identity", Ciph |e| e));
}
