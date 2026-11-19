//! Old (packed partial products) vs triangular multiplication, simulated on one HPU.
//!
//! Run with:
//!   cargo run --release -p zhc --example mul_tri_compare -- [lhs_bits rhs_bits out_bits ovf]...
//!
//! Without arguments, runs the 64-bit symmetric, overflow and asymmetric cases.

use zhc::prelude::*;
use zhc_builder::{Builder, SpecInteger};

fn build(a_bits: u16, b_bits: u16, out_bits: u16, ovf: bool, tri: bool) -> Builder {
    let spec = SpecInteger::new(out_bits, 2, 2);
    let bd = Builder::new(spec.block_spec());
    let a = bd.ciphertext_integer_input(a_bits);
    let b = bd.ciphertext_integer_input(b_bits);
    let ab = bd.ciphertext_integer_split(&a);
    let bb = bd.ciphertext_integer_split(&b);
    let cut = spec.block_count();
    let (res, flag) = match (ovf, tri) {
        (true, true) => {
            let (r, f) = bd.iop_overflow_mul_tri_raw(&ab, &bb);
            (r, Some(f))
        }
        (true, false) => {
            let (r, f) = bd.iop_mul_raw(&ab, &bb, cut);
            (r, Some(f))
        }
        (false, true) => (bd.iop_mul_lsb_raw(&ab, &bb, cut), None),
        (false, false) => (bd.iop_mul_raw(&ab, &bb, cut).0, None),
    };
    let out = bd.ciphertext_integer_join(&res, Some(out_bits));
    bd.ciphertext_integer_output(out);
    if let Some(f) = flag {
        let f = bd.ciphertext_bool_store_block(f);
        bd.ciphertext_bool_output(f);
    }
    bd
}

fn run(bd: Builder) -> (f64, usize) {
    let mut pl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());
    let m = pl.get_hpu_metrics();
    (
        format!("{}", m.latency)
            .trim_end_matches(|c: char| !c.is_ascii_digit())
            .trim()
            .parse()
            .unwrap_or(f64::NAN),
        m.slots_filled,
    )
}

fn run_multi(bd: Builder, n_hpus: u8) -> String {
    use zhc_config::multi_hpu::MultiHpuConfig;
    let mut pl = Pipeline::new()
        .with_builder(bd)
        .with_multi_hpu_config(MultiHpuConfig {
            n_hpus,
            ..Default::default()
        });
    let m = pl.get_multi_hpu_metrics();
    format!("{}", m.latency)
}

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    if raw.first().map(String::as_str) == Some("multi") {
        for n in [2u8, 4, 8] {
            let old = run_multi(build(64, 64, 64, false, false), n);
            let new = run_multi(build(64, 64, 64, false, true), n);
            println!("Mul 64 auto on {n} HPUs: old {old}, new {new}");
        }
        return;
    }
    let args: Vec<u16> = raw.iter().map(|s| s.parse().unwrap()).collect();
    let cases: Vec<(u16, u16, u16, bool)> = if args.is_empty() {
        vec![
            (64, 64, 64, false),
            (64, 64, 64, true),
            (64, 32, 64, false),
            (64, 16, 64, false),
            (64, 8, 64, false),
            (32, 32, 32, false),
            (32, 32, 32, true),
            (128, 128, 128, false),
        ]
    } else {
        args.chunks(4)
            .map(|c| (c[0], c[1], c[2], c[3] != 0))
            .collect()
    };
    println!(
        "{:<22} {:>12} {:>7} {:>12} {:>7} {:>8}",
        "case", "old (us)", "PBS", "new (us)", "PBS", "gain"
    );
    for (a, b, o, ovf) in cases {
        let (lo, po) = run(build(a, b, o, ovf, false));
        let (ln, pn) = run(build(a, b, o, ovf, true));
        let name = format!("{a}x{b}->{o}{}", if ovf { " ovf" } else { "" });
        println!(
            "{name:<22} {lo:>12.2} {po:>7} {ln:>12.2} {pn:>7} {:>7.1}%",
            100.0 * (ln - lo) / lo
        );
    }
}
