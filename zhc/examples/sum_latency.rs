//! Simulated HPU latency of summing `n` encrypted integers.
//!
//! Run with:
//!   cargo run --release -p zhc --example sum_latency -- <bits> <n> [latency|pbs]
//!
//! `latency` (default) uses `SumKind::MinLatency` (column reduction),
//! `pbs` uses `SumKind::MinPbs` (ripple carry). `n` must be at least 3.

use zhc::prelude::*;
use zhc_builder::{SpecInteger, sum_column_reduce, sum_ripple_carry};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let bits: u16 = args
        .first()
        .map_or(64, |s| s.parse().expect("bits must be an integer"));
    let n: usize = args
        .get(1)
        .map_or(4, |s| s.parse().expect("n must be an integer"));
    let min_pbs = matches!(args.get(2).map(String::as_str), Some("pbs"));

    let spec = SpecInteger::new(bits, 2, 2);
    let bd = if min_pbs {
        sum_ripple_carry(spec, n)
    } else {
        sum_column_reduce(spec, n)
    };
    bd.check_noise();

    let mut pl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());

    let kind = if min_pbs { "MinPbs" } else { "MinLatency" };
    println!("SUM of {n} x {bits}b ({kind})");
    pl.get_hpu_metrics().dump();
}
