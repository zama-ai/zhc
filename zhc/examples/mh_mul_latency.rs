//! Simulated latency of MhMul, the multiplication spread over several HPU boards.
//!
//! Run with:
//!   cargo run --release -p zhc --example mh_mul_latency -- [auto|manual] [n_hpus] [bits]
//!
//! `auto` (default) lets the compiler partition the circuit. `manual` uses the hand-grouped
//! partitions of `Iop::MhMul`, one group per board. `n_hpus` is 2, 4 or 8 (default 8), `bits`
//! defaults to 64.

use zhc::PipelineExt;
use zhc::compat::Iop;
use zhc_builder::SpecInteger;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_langs::doplang::DopInstructionSet;
use zhc_pipeline::Pipeline;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manual = match args.first().map(String::as_str) {
        None | Some("auto") => false,
        Some("manual") => true,
        Some(other) => panic!("unknown partitioning `{other}`, expected `auto` or `manual`"),
    };
    let n_hpus: u8 = args
        .get(1)
        .map_or(8, |v| v.parse().expect("n_hpus must be 2, 4 or 8"));
    assert!(
        matches!(n_hpus, 2 | 4 | 8),
        "n_hpus must be 2, 4 or 8, got {n_hpus}"
    );
    let bits: u16 = args
        .get(2)
        .map_or(64, |v| v.parse().expect("bits must be an integer"));

    let spec = SpecInteger::new(bits, 2, 2);
    let builder = Iop::MhMul { par_n: n_hpus }.to_builder(spec);
    let config = MultiHpuConfig {
        n_hpus,
        ..Default::default()
    };
    println!(
        "MhMul {bits} bits on {n_hpus} HPUs, {} partitioning",
        if manual { "manual" } else { "auto" }
    );
    let pl = Pipeline::new()
        .with_builder(builder)
        .with_multi_hpu_config(config);
    let mut pl = if manual {
        pl.with_builder_partitions(true)
    } else {
        pl
    };

    let per_board = pl
        .get_multi_doplang()
        .iter()
        .map(|ir| {
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
