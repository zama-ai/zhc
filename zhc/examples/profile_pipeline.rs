use zhc::prelude::BuilderExt;
use zhc_builder::{Builder, CiphertextBlockSpec};
use zhc_pipeline::Pipeline;

fn main() {
    let bd = Builder::new(CiphertextBlockSpec(2,2));
    let blocks = bd.ciphertext_split(bd.ciphertext_input(2));
    let cst = bd.block_let_plaintext(3);
    let a = bd.block_mul_plaintext(blocks[0], cst);
    let a = bd.block_mul_plaintext(a, cst);
    let a = bd.block_mul_plaintext(a, cst);
    bd.ciphertext_output(bd.ciphertext_join([a], None));
    bd.dump_noise_budget();

    let mut ppl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());
    ppl.get_hpu_stream();
}
