//! Checks iop_divx on chosen values against plaintext / and %.
//!   cargo run --release -p zhc --example div_check -- <num_bits> <den_bits> <random_cases>
use zhc_builder::{Builder, SpecInteger};
use zhc_langs::ioplang::IopValue;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }
    /// Random value whose bit length is uniform in 1..=bits (top bit set).
    fn with_random_len(&mut self, bits: u32) -> u128 {
        let len = 1 + (self.next() % bits as u64) as u32;
        let v = (self.next() as u128) | ((self.next() as u128) << 64);
        let mask = if len >= 128 { u128::MAX } else { (1u128 << len) - 1 };
        (v & mask) | (1u128 << (len - 1))
    }
    fn full(&mut self, bits: u32) -> u128 {
        let v = (self.next() as u128) | ((self.next() as u128) << 64);
        if bits >= 128 { v } else { v & ((1u128 << bits) - 1) }
    }
}

fn main() {
    let a: Vec<u32> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect();
    let (nb, db, n_rand) = (a[0], a[1], a[2] as usize);
    let (ns, ds) = (SpecInteger::new(nb as u16, 2, 2), SpecInteger::new(db as u16, 2, 2));
    let bd = Builder::new(ns.block_spec());
    let x = bd.ciphertext_integer_input(nb as u16);
    let y = bd.ciphertext_integer_input(db as u16);
    let (q, r) = bd.iop_divx(&x, &y);
    let (q_spec, r_spec) = (q.spec(), r.spec());
    bd.ciphertext_integer_output(q);
    bd.ciphertext_integer_output(r);

    let nmax = if nb >= 128 { u128::MAX } else { (1u128 << nb) - 1 };
    let dmax = if db >= 128 { u128::MAX } else { (1u128 << db) - 1 };
    let mut cases: Vec<(u128, u128)> = Vec::new();
    // Edge cases.
    let mut dens = vec![1u128, 2, 3, 4, 5, 7, dmax, dmax - 1, dmax / 2, dmax / 2 + 1, dmax / 3];
    for k in 1..db { dens.push(1u128 << k); dens.push((1u128 << k) - 1); dens.push((1u128 << k) + 1); }
    let nums = [0u128, 1, 2, 3, nmax, nmax - 1, nmax / 2, nmax / 3, nmax / 7 * 7, dmax.min(nmax), (dmax.min(nmax)).saturating_sub(1)];
    for &d in dens.iter().filter(|&&d| d >= 1 && d <= dmax) {
        for &n in &nums { cases.push((n, d)); }
    }
    // Random cases: varied-length divisor, with varied-length and full-width dividends.
    let mut rng = Rng(0x9E3779B97F4A7C15 ^ ((nb as u64) << 32) ^ db as u64);
    for i in 0..n_rand {
        let d = rng.with_random_len(db);
        let n = if i % 2 == 0 { rng.with_random_len(nb) } else { rng.full(nb) };
        cases.push((n, d));
    }

    let mut fails = 0usize;
    let mut qlen_hist = [0usize; 129];
    for &(n, d) in &cases {
        let out = bd
            .interpret()
            .with_inputs(&[IopValue::CiphertextInteger(ns.from_int(n)), IopValue::CiphertextInteger(ds.from_int(d))])
            .get_outputs();
        let expected = vec![
            IopValue::CiphertextInteger(q_spec.from_int(n / d)),
            IopValue::CiphertextInteger(r_spec.from_int(n % d)),
        ];
        if out.to_vec() != expected {
            fails += 1;
            if fails <= 5 { println!("  FAIL {n} / {d}: got {:?}", out); }
        }
        qlen_hist[(128 - (n / d).leading_zeros()) as usize] += 1;
    }
    // Division by zero: report what the circuit returns (convention, not checked).
    let out0 = bd.interpret().with_inputs(&[IopValue::CiphertextInteger(ns.from_int(12345u128.min(nmax))), IopValue::CiphertextInteger(ds.from_int(0))]).get_outputs();
    let distinct = qlen_hist.iter().filter(|&&c| c > 0).count();
    println!("{nb}b / {db}b: {} cases ({} edge + {n_rand} random), {fails} failures, quotient bit lengths covered: {distinct} distinct (0..={})",
        cases.len(), cases.len() - n_rand, qlen_hist.iter().rposition(|&c| c > 0).unwrap_or(0));
    println!("  {} / 0 -> {:?}", 12345u128.min(nmax), out0);
}
