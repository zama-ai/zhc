use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::{hint::black_box, time::Instant};

use tfhe::integer::RadixCiphertext;
use tfhe::shortint::parameters::current_params::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
use zhc::pipeline::scheduler::vm::VmExecutionPlan;
use zhc::{builder::CiphertextSpec, prelude::Builder, prelude::BuilderExt};
use zhc_langs::vmlang::VmByteCode;
use zhc_utils::{Dumpable, SafeAs};
use zhc_vm::{VMParams, Value, ValueMut, VM};

use rand::{rngs::SmallRng, RngExt, SeedableRng};

fn single_lane(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_from = builder.ciphertext_input(spec.int_size());
    let src_to = builder.ciphertext_input(spec.int_size());
    let src_amount = builder.ciphertext_input(spec.int_size());
    let (new_from, new_to) = builder.iop_erc_7984_ripple(&src_from, &src_to, &src_amount);
    builder.ciphertext_output(new_from);
    builder.ciphertext_output(new_to);
    builder
}

fn remap(op: &VmByteCode, op_off: u32, reg_off: u16, in_off: u16, out_off: u16) -> VmByteCode {
    use VmByteCode::*;
    let r = |x: u16| x + reg_off;
    let i = |x: u32| x + op_off;
    let s = |x: u16| x + in_off;
    match *op {
        ADD { id, dst, src1, src2 } => ADD { id: i(id), dst: r(dst), src1: r(src1), src2: r(src2) },
        SUB { id, dst, src1, src2 } => SUB { id: i(id), dst: r(dst), src1: r(src1), src2: r(src2) },
        MAC { id, dst, src1, src2, cst } => {
            MAC { id: i(id), dst: r(dst), src1: r(src1), src2: r(src2), cst }
        }
        ADDS { id, dst, src, s_id, s_blk } => ADDS { id: i(id), dst: r(dst), src: r(src), s_id: s(s_id), s_blk },
        SUBS { id, dst, src, s_id, s_blk } => SUBS { id: i(id), dst: r(dst), src: r(src), s_id: s(s_id), s_blk },
        SSUB { id, dst, src, s_id, s_blk } => SSUB { id: i(id), dst: r(dst), src: r(src), s_id: s(s_id), s_blk },
        MULS { id, dst, src, s_id, s_blk } => MULS { id: i(id), dst: r(dst), src: r(src), s_id: s(s_id), s_blk },
        ADDC { id, dst, src, cst } => ADDC { id: i(id), dst: r(dst), src: r(src), cst },
        SUBC { id, dst, src, cst } => SUBC { id: i(id), dst: r(dst), src: r(src), cst },
        CSUB { id, dst, src, cst } => CSUB { id: i(id), dst: r(dst), src: r(src), cst },
        MULC { id, dst, src, cst } => MULC { id: i(id), dst: r(dst), src: r(src), cst },
        LD { id, dst, src_id, src_blk } => LD { id: i(id), dst: r(dst), src_id: s(src_id), src_blk },
        ST { id, dst_id, dst_blk, src } => ST { id: i(id), dst_id: dst_id + out_off, dst_blk, src: r(src) },
        KS { id, dst, src } => KS { id: i(id), dst: r(dst), src: r(src) },
        PBS { id, dst, src, lut } => PBS { id: i(id), dst: r(dst), src: r(src), lut },
        PBS_ML2 { id, dst1, dst2, src, lut } => {
            PBS_ML2 { id: i(id), dst1: r(dst1), dst2: r(dst2), src: r(src), lut }
        }
        DEF { id, dst, cst } => DEF { id: i(id), dst: r(dst), cst },
    }
}

fn lane_reg_count(stream: &[VmByteCode]) -> u16 {
    stream
        .iter()
        .flat_map(|op| [op.get_dst1(), op.get_dst2(), op.get_src1(), op.get_src2()])
        .flatten()
        .max()
        .map(|m| m + 1)
        .unwrap_or(0)
}

fn replicate(
    single: &VmExecutionPlan,
    n_cores: usize,
    n_in: u8,
    n_out: u8,
) -> (VmExecutionPlan, usize, usize) {
    assert_eq!(single.irs.len(), 1, "expected a single-thread plan");
    let stream = &single.irs[0];
    let n_ops = single.locks_table.len() as u32;
    let n_regs = lane_reg_count(stream);
    assert!((n_cores as u32) * (n_regs as u32) <= u16::MAX as u32, "register id space (u16) exhausted");
    let n_in_slots = n_cores * n_in as usize;
    assert!(n_in_slots <= u16::MAX as usize + 1, "input id space (u16) exhausted");
    let n_out_slots = n_cores * n_out as usize;
    assert!(n_out_slots <= u16::MAX as usize, "output id space (u16) exhausted");

    let mut irs = Vec::with_capacity(n_cores);
    let mut locks_table = Vec::with_capacity(n_cores * n_ops as usize);
    let mut successors_table = Vec::with_capacity(n_cores * n_ops as usize);

    for c in 0..n_cores as u32 {
        let op_off = c * n_ops;
        let reg_off = (c as u16) * n_regs;
        let in_off = (c * n_in as u32) as u16;
        let out_off = (c as u16) * n_out as u16;

        irs.push(stream.iter().map(|op| remap(op, op_off, reg_off, in_off, out_off)).collect());
        locks_table.extend_from_slice(&single.locks_table);
        for succ in &single.successors_table {
            successors_table.push(succ.iter().map(|s| s + op_off).collect());
        }
    }

    let nregs = n_cores * n_regs as usize;
    (VmExecutionPlan { irs, locks_table, successors_table, nregs }, n_in_slots, n_out_slots)
}

fn verify_correctness() {
    let p = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    let spec = CiphertextSpec::new(64, 2, 2);
    let n_blocks = 64 / 2;

    let n_cores = std::env::var("VM_CORES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().unwrap().get());

    let circuit = single_lane(spec);
    let single = circuit.get_vm_plan(1);
    let sig = circuit.signature();
    let n_in = sig.get_args_arity();
    let n_out = sig.get_returns_arity();
    assert_eq!(n_in.sas::<usize>(), 3, "expected (from, to, amount) inputs");

    let (plan, _n_in_slots, n_out_slots) =
        replicate(&single, n_cores, n_in.sas(), n_out.sas());

    let params = VMParams::from_ks32_params(p, plan.nregs, Some(n_cores));

    let (ck, sk) = tfhe::generate_keys(tfhe::ConfigBuilder::with_custom_parameters(p));
    let ck = ck.into_raw_parts().0;
    let mut vm = VM::new(&params);
    vm.set_server_key(sk);

    let n_out = n_out.sas::<usize>();

    let seed = std::env::var("VM_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    let n_rounds: usize =
        std::env::var("VM_ROUNDS").ok().and_then(|s| s.parse().ok()).unwrap_or(10);
    let mut prng = SmallRng::seed_from_u64(seed);

    let poison = 0xDEAD_BEEF_DEAD_BEEFu64;

    let mut all_ok = true;

    for round in 0..n_rounds {
        let cases: Vec<(u64, u64, u64)> = (0..n_cores)
            .map(|_| (prng.random::<u64>(), prng.random::<u64>(), prng.random::<u64>()))
            .collect();
        let expected: Vec<(u64, u64)> = cases
            .iter()
            .map(|&(from, to, amount)| {
                if from >= amount {
                    (from.wrapping_sub(amount), to.wrapping_add(amount))
                } else {
                    (from, to)
                }
            })
            .collect();

        let in_cts: Vec<RadixCiphertext> = cases
            .iter()
            .flat_map(|&(from, to, amount)| {
                [
                    ck.encrypt_radix(from, n_blocks),
                    ck.encrypt_radix(to, n_blocks),
                    ck.encrypt_radix(amount, n_blocks),
                ]
            })
            .collect();
        let mut out_cts: Vec<RadixCiphertext> =
            (0..n_out_slots).map(|_| ck.encrypt_radix(poison, n_blocks)).collect();
        let in_vals: Vec<Value> =
            in_cts.iter().map(|c| Value::FheUint(c as *const RadixCiphertext)).collect();
        let mut out_vals: Vec<ValueMut> =
            out_cts.iter_mut().map(|c| ValueMut::FheUint(c as *mut RadixCiphertext)).collect();
        vm.execute(black_box(&plan), black_box(&in_vals), &mut out_vals);

        let mut wrong = 0usize;
        let mut unexercised = 0usize;
        for c in 0..n_cores {
            let (exp_from, exp_to) = expected[c];
            let got_from: u64 = ck.decrypt_radix(&out_cts[c * n_out]);
            let got_to: u64 = ck.decrypt_radix(&out_cts[c * n_out + 1]);
            if (got_from, got_to) != (exp_from, exp_to) {
                wrong += 1;
                if (got_from, got_to) == (poison, poison) {
                    unexercised += 1;
                }
                if wrong <= 3 {
                    let (from, to, amount) = cases[c];
                    println!(
                        "  ✗ lane {c}: transfer(from={from}, to={to}, amount={amount}) \
                         got (new_from={got_from}, new_to={got_to}), expected ({exp_from}, {exp_to}){}",
                        if (got_from, got_to) == (poison, poison) { " [unexercised: poison intact]" } else { "" },
                    );
                }
            }
        }
        all_ok &= wrong == 0;
        println!(
            "{} round {round}: {}/{n_cores} lanes correct ({wrong} wrong, {unexercised} unexercised)",
            if wrong == 0 { "✓" } else { "✗" },
            n_cores - wrong,
        );
    }
    assert!(all_ok, "some lanes produced wrong or missing results");
}

fn bench_throughput(c: &mut Criterion) {
    if std::env::var_os("VM_VERIFY").is_some() {
        verify_correctness();
        return;
    }

    let p = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    let spec = CiphertextSpec::new(64, 2, 2);
    let n_blocks = 64 / 2;

    let n_cores = std::thread::available_parallelism().unwrap().get();

    let t = Instant::now();
    let circuit = single_lane(spec);
    let single = circuit.get_vm_plan(1);
    let sig = circuit.signature();
    let n_in = sig.get_args_arity();
    let n_out = sig.get_returns_arity();
    let (plan, n_in_slots, n_out_slots) =
        replicate(&single, n_cores, n_in.sas(), n_out.sas());
    println!("Building the replicated plan took {} us", t.elapsed().as_micros());

    let params = VMParams::from_ks32_params(p, plan.nregs, Some(n_cores));

    let (ck, sk) = tfhe::generate_keys(tfhe::ConfigBuilder::with_custom_parameters(p));
    let ck = ck.into_raw_parts().0;
    let mut vm = VM::new(&params);
    vm.set_server_key(sk);

    let in_cts: Vec<RadixCiphertext> =
        (0..n_in_slots).map(|_| ck.encrypt_radix(99u64, n_blocks)).collect();
    let mut out_cts: Vec<RadixCiphertext> =
        (0..n_out_slots).map(|_| ck.encrypt_radix(0u64, n_blocks)).collect();

    let in_vals: Vec<Value> = in_cts
        .iter()
        .map(|c| Value::FheUint(c as *const RadixCiphertext))
        .collect();
    let mut out_vals: Vec<ValueMut> = out_cts
        .iter_mut()
        .map(|c| ValueMut::FheUint(c as *mut RadixCiphertext))
        .collect();

    vm.reset_profile();

    let mut group = c.benchmark_group("throughput");
    group.sample_size(10);
    group.throughput(Throughput::Elements(n_cores as u64));
    group.bench_function(format!("erc7984_replicated_x{n_cores}"), |b| {
        b.iter(|| vm.execute(black_box(&plan), black_box(&in_vals), &mut out_vals));
    });
    group.finish();

    vm.profile().dump();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
