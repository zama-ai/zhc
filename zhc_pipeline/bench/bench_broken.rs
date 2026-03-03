use std::hint::black_box;

use zhc_builder::{CiphertextSpec, add};
use zhc_pipeline::translation::lower_iop_to_hpu;

fn main() {
    let ir = add(CiphertextSpec::new(128, 2, 2)).into_ir();
    println!("Here");
    let _ir = black_box(lower_iop_to_hpu(&ir));
    println!("there");
}
