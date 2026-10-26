//! Simulated HPU latency of an unsigned division whose operands have different sizes.
//!
//! Run with:
//!   cargo run --release -p zhc --example div_asym_latency -- <num_bits> <div_bits> [div|rem|divx]
//!
//! `div` outputs the quotient only, `rem` the remainder only, `divx` both (default).
//! Unused outputs are removed by dead-code elimination, so the mode affects latency.

use zhc::prelude::*;
use zhc_builder::{Builder, SpecInteger};
use zhc_langs::ioplang::IopValue;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Div,
    Rem,
    DivX,
}

/// Builds `num / den` (and/or `num % den`) with `num` and `den` of different widths.
/// The quotient has the dividend's width. The remainder's width depends on the ZHC version:
/// the dividend's width upstream, the divisor's width with the remainder-capping patch.
/// It is returned alongside the builder so the oracle can match it.
fn div_asym(num_bits: u16, den_bits: u16, mode: Mode) -> (Builder, SpecInteger) {
    // SpecInteger::new(int_size, carry_bits, message_bits)
    let bd = Builder::new(SpecInteger::new(num_bits, 2, 2).block_spec());
    let num = bd.ciphertext_integer_input(num_bits);
    let den = bd.ciphertext_integer_input(den_bits);
    let (q, r) = bd.iop_divx(&num, &den);
    let r_spec = r.spec();
    if mode != Mode::Rem {
        bd.ciphertext_integer_output(q);
    }
    if mode != Mode::Div {
        bd.ciphertext_integer_output(r);
    }
    (bd, r_spec)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let num_bits: u16 = args.first().map_or(64, |s| s.parse().unwrap());
    let den_bits: u16 = args.get(1).map_or(16, |s| s.parse().unwrap());
    let mode = match args.get(2).map(String::as_str) {
        Some("div") => Mode::Div,
        Some("rem") => Mode::Rem,
        _ => Mode::DivX,
    };

    let (bd, r_spec) = div_asym(num_bits, den_bits, mode);

    // Sanity checks on emulated semantics (no crypto, fast): noise and functional correctness.
    bd.check_noise();
    let q_spec = SpecInteger::new(num_bits, 2, 2);
    bd.test_random(200, |inp| {
        let [IopValue::CiphertextInteger(a), IopValue::CiphertextInteger(b)] = inp else {
            unreachable!()
        };
        let (a, b) = (a.as_storage(), b.as_storage());
        if b == 0 {
            return None; // Division by zero is unspecified: skip.
        }
        let q = IopValue::CiphertextInteger(q_spec.from_int(a / b));
        let r = IopValue::CiphertextInteger(r_spec.from_int(a % b));
        Some(match mode {
            Mode::Div => vec![q],
            Mode::Rem => vec![r],
            Mode::DivX => vec![q, r],
        })
    });

    let mut pl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());

    let name = match mode {
        Mode::Div => "DIV",
        Mode::Rem => "REM",
        Mode::DivX => "DIVX",
    };
    println!("{name} {num_bits}b / {den_bits}b");
    pl.get_hpu_metrics().dump();
}
