//! Simulated latency of MhMulF8, the 64 bit multiplication spread over 8 HPU boards.
//!
//! Run with `cargo run --release -p zhc --example mh_mul_latency`.

use zhc::compat::mh_mul;
use zhc_builder::CiphertextSpec;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_langs::doplang::DopInstructionSet;

fn main() {
    let spec = CiphertextSpec::new(64, 2, 2);
    // `MultiHpuConfig::default()` uses 4 boards, so 8 has to be given explicitly.
    let config = MultiHpuConfig {
        n_hpus: 8,
        ..Default::default()
    };
    let mut pl = mh_mul(spec, config);

    // Every accessor takes `&mut self`, so each borrow has to end before the next call.
    let per_board = pl
        .get_multi_doplang()
        .iter()
        .map(|ir| {
            // A receiving board has only 63 transfer flags.
            let received = ir
                .walk_ops_linear()
                .filter(|op| matches!(op.get_instruction(), DopInstructionSet::WAIT { .. }))
                .count();
            (ir.n_ops(), received)
        })
        .collect::<Vec<_>>();
    let latency = pl.get_multi_hpu_metrics().latency;

    for (hpu, (n_ops, received)) in per_board.iter().enumerate() {
        println!("  HPU {hpu}: {n_ops:>5} device operations, {received:>3} transfers in");
    }
    println!(
        "  total  : {} device operations, max {} received on one board, budget 63",
        per_board.iter().map(|(n, _)| n).sum::<u32>(),
        per_board.iter().map(|(_, r)| r).max().unwrap(),
    );
    println!("simulated : {latency}");
}
