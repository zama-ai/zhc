//! Simulated HPU latency of any built-in IOP, selected by name on the command line.
//!
//! Run with:
//!   cargo run --release -p zhc --example iop_latency -- <iop> [bits]
//!
//! `<iop>` is matched case-insensitively, ignoring underscores: `cmp_eq`, `CmpEq` and `cmpeq`
//! all select `Iop::CmpEq`. `bits` defaults to 64. Run with `--list` to print all IOP names.

use zhc::compat::Iop;
use zhc::prelude::*;
use zhc_builder::SpecInteger;

/// Variant name of an IOP, without fields (`AddSimd { n: 8 }` -> `AddSimd`).
fn variant_name(iop: &Iop) -> String {
    let dbg = format!("{iop:?}");
    dbg.split([' ', '{', '('])
        .next()
        .unwrap_or(&dbg)
        .to_string()
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| *c != '_' && *c != '-')
        .collect::<String>()
        .to_lowercase()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(name) = args.first() else {
        eprintln!("usage: iop_latency <iop> [bits]   (or --list)");
        std::process::exit(1);
    };

    if name == "--list" {
        for iop in Iop::ALL_STATIC {
            println!("{}", variant_name(iop));
        }
        return;
    }

    let bits: u16 = args
        .get(1)
        .map_or(64, |s| s.parse().expect("bits must be an integer"));
    let Some(iop) = Iop::ALL_STATIC
        .iter()
        .find(|iop| normalize(&variant_name(iop)) == normalize(name))
    else {
        eprintln!("unknown IOP `{name}`; run with --list to see the available ones");
        std::process::exit(1);
    };

    // SpecInteger::new(int_size, carry_bits, message_bits)
    let bd = iop.to_builder(SpecInteger::new(bits, 2, 2));
    let mut pl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());

    println!("{} on {bits} bits", variant_name(iop));
    pl.get_hpu_metrics().dump();
}
