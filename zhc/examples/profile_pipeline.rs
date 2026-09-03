use zhc_builder::CiphertextSpec;
use zhc_pipeline::{Pipeline, compat::Iop};

fn main() {
    let bd = Iop::Mul.to_builder(CiphertextSpec::new(64, 2, 2));
    let mut ppl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());
    ppl.get_hpu_stream();
    ppl.draw_state().open().unwrap();
}
