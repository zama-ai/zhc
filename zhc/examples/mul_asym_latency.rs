//! Simulated HPU latency of a multiplication whose operands have different sizes.
//!
//! Run with:
//!   cargo run --release -p zhc --example mul_asym_latency -- <lhs_bits> <rhs_bits> [out_bits]
//!
//! `out_bits` defaults to `lhs_bits` (wrapping product, like `Iop::Mul`).
//! Use `lhs_bits + rhs_bits` to get the full, non-truncated product.

use zhc::prelude::*;
use zhc_builder::{Builder, SpecInteger};
use zhc_langs::ioplang::IopValue;

/// Builds `out = (lhs * rhs) mod 2^out_bits` with `lhs` and `rhs` of different widths.
fn mul_asym(lhs_bits: u16, rhs_bits: u16, out_bits: u16, carry: u8, msg: u8) -> Builder {
    // The block spec (message/carry bits) is shared, only the integer sizes differ.
    let bd = Builder::new(SpecInteger::new(lhs_bits, carry, msg).block_spec());
    let lhs = bd.ciphertext_integer_input(lhs_bits);
    let rhs = bd.ciphertext_integer_input(rhs_bits);

    let a = bd.ciphertext_integer_split(&lhs);
    let b = bd.ciphertext_integer_split(&rhs);

    // Number of output blocks to compute (partial products of degree >= cut_off are dropped).
    let out_blocks = SpecInteger::new(out_bits, carry, msg).block_count();
    let (res, _overflow) = bd.iop_mul_raw(&a, &b, out_blocks);

    let out = bd.ciphertext_integer_join(&res, Some(out_bits));
    bd.ciphertext_integer_output(out);
    bd
}

fn main() {
    let args: Vec<u16> = std::env::args()
        .skip(1)
        .map(|s| s.parse().expect("bit widths must be integers"))
        .collect();
    let (lhs_bits, rhs_bits) = match args.as_slice() {
        [l, r, ..] => (*l, *r),
        _ => (64, 16),
    };
    let out_bits = args.get(2).copied().unwrap_or(lhs_bits);

    let bd = mul_asym(lhs_bits, rhs_bits, out_bits, 2, 2);

    // Sanity checks on emulated semantics (no crypto, fast): noise and functional correctness.
    bd.check_noise();
    let out_spec = SpecInteger::new(out_bits, 2, 2);
    bd.test_random(200, |inp| {
        let [IopValue::CiphertextInteger(a), IopValue::CiphertextInteger(b)] = inp else {
            unreachable!()
        };
        let mask = if out_bits >= 128 { u128::MAX } else { (1u128 << out_bits) - 1 };
        let expected = a.as_storage().wrapping_mul(b.as_storage()) & mask;
        Some(vec![IopValue::CiphertextInteger(out_spec.from_int(expected))])
    });

    let mut pl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());

    println!("MUL {lhs_bits}b x {rhs_bits}b -> {out_bits}b");
    pl.get_hpu_metrics().dump();
}
