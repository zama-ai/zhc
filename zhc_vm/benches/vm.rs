use criterion::{criterion_group, criterion_main, Criterion};
use std::{hint::black_box, time::Instant};
use zhc_utils::Dumpable;

use tfhe::integer::RadixCiphertext;
use tfhe::shortint::parameters::current_params::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
use zhc::{builder::CiphertextSpec, pipeline::compat::Iop, prelude::BuilderExt};
use zhc_utils::SafeAs;
use zhc_vm::{VMParams, Value, ValueMut, VM};

fn bench_vm(c: &mut Criterion) {
    let p = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    let spec = CiphertextSpec::new(64, 2, 2);
    let n_blocks = 64 / 2; // 2 message bits per block over a 64-bit integer.

    let params = VMParams::from_ks32_params(p, 1024, None);

    let (ck, sk) = tfhe::generate_keys(tfhe::ConfigBuilder::with_custom_parameters(p));
    let ck = ck.into_raw_parts().0;
    let mut vm = VM::new(&params);
    vm.set_server_key(sk);

    let mut group = c.benchmark_group("vm");

    for iop in [Iop::BwAnd] {
        let t = Instant::now();
        let circuit = iop.to_builder(spec);
        let plan = circuit.get_vm_plan(params.n_threads.sas());
        println!("Compiling {:?} took {} us", iop, t.elapsed().as_micros());

        let sig = circuit.signature();
        let n_in = sig.get_args_arity();
        let n_out = sig.get_returns_arity();

        let in_cts: Vec<RadixCiphertext> =
            (0..n_in).map(|_| ck.encrypt_radix(99u64, n_blocks)).collect();
        let mut out_cts: Vec<RadixCiphertext> =
            (0..n_out).map(|_| ck.encrypt_radix(0u64, n_blocks)).collect();

        let in_vals: Vec<Value> = in_cts
            .iter()
            .map(|c| Value::FheUint(c as *const RadixCiphertext))
            .collect();
        let mut out_vals: Vec<ValueMut> = out_cts
            .iter_mut()
            .map(|c| ValueMut::FheUint(c as *mut RadixCiphertext))
            .collect();

        vm.reset_profile();

        group.bench_function(format!("{iop:?}"), |b| {
            b.iter(|| vm.execute(black_box(&plan), black_box(&in_vals), &mut out_vals));
        });

        vm.profile().dump();

    }

    group.finish();
}

criterion_group!(benches, bench_vm);
criterion_main!(benches);
