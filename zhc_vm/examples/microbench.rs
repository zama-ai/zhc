use std::hint::black_box;
use std::time::{Duration, Instant};

use tfhe::shortint::parameters::v1_6::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
use zhc::prelude::VmExecutionPlan;
use zhc_config::vm::VmConfig;
use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::LutRegistry};
use zhc_langs::{ioplang::Lut1Def, vmlang::VmByteCode};
use zhc_utils::{SafeAs, small::SmallVec, svec};
use zhc_vm::{Vm, VmConfigExt};

use zhc_ir::OpIdRaw;
const REPS: u32 = 3;

/// Builtin tables the PBS microbench cycles through, so that consecutive bootstrappings
/// do not hit the same accumulator in cache.
const PBS_LUTS: [Lut1Def; 16] = [
    Lut1Def::None,
    Lut1Def::MsgOnly,
    Lut1Def::CarryOnly,
    Lut1Def::CarryInMsg,
    Lut1Def::MultCarryMsg,
    Lut1Def::MultCarryMsgLsb,
    Lut1Def::MultCarryMsgMsb,
    Lut1Def::BwAnd,
    Lut1Def::BwOr,
    Lut1Def::BwXor,
    Lut1Def::CmpSign,
    Lut1Def::CmpReduce,
    Lut1Def::CmpGt,
    Lut1Def::CmpGte,
    Lut1Def::CmpLt,
    Lut1Def::CmpLte,
];

fn pbs_registry(params: &VmConfig) -> LutRegistry {
    let spec = CiphertextBlockSpec(params.carry_size.sas(), params.message_size.sas());
    let mut reg = LutRegistry::empty();
    for def in PBS_LUTS {
        reg.register_l1(&def.into_lut(spec));
    }
    reg
}

fn repeated_plan(
    mk: impl Fn(OpIdRaw) -> VmByteCode,
    lut_reg: &LutRegistry,
    count: u32,
    n_threads: usize,
) -> VmExecutionPlan {
    let mut irs = vec![Vec::new(); n_threads];
    irs[0] = (0..count).map(&mk).collect();
    let nregs = irs[0]
        .iter()
        .flat_map(|op| [op.get_dst1(), op.get_dst2(), op.get_src1(), op.get_src2()])
        .flatten()
        .max()
        .map(|m| m as usize + 1)
        .unwrap_or(0);
    let successors_table: Vec<SmallVec<OpIdRaw>> = vec![svec![]; count as usize];
    VmExecutionPlan {
        irs,
        lut_reg: lut_reg.clone(),
        locks_table: vec![0u8; count as usize],
        successors_table,
        nregs,
    }
}

fn time_op(
    vm: &mut Vm,
    mk: impl Fn(OpIdRaw) -> VmByteCode,
    lut_reg: &LutRegistry,
    count: u32,
    n_threads: usize,
) -> Duration {
    let plan = repeated_plan(mk, lut_reg, count, n_threads);
    vm.execute(&plan, &[], &mut []); // warmup
    let t = Instant::now();
    for _ in 0..REPS {
        vm.execute(black_box(&plan), &[], &mut []);
    }
    t.elapsed() / (REPS * count)
}

fn main() {
    let p = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    let params = VmConfig::from_ks32_params(p, 1024);

    let (_ck, sk) = tfhe::generate_keys(tfhe::ConfigBuilder::with_custom_parameters(p));
    let mut vm = Vm::new(&params, None);
    let n_threads = vm.get_topology().n_processors();
    vm.set_server_key(sk);

    let regf = params.regf_size;
    let reg = move |i: OpIdRaw, arity: usize, k: usize| ((i as usize * arity + k) % regf) as u16;
    let lut_reg = pbs_registry(&params);
    let n_luts = lut_reg.iter_luts().count() as u32;

    let t_add = time_op(
        &mut vm,
        |id| VmByteCode::ADD {
            id,
            dst: reg(id, 3, 0),
            src1: reg(id, 3, 1),
            src2: reg(id, 3, 2),
        },
        &lut_reg,
        200_000,
        n_threads,
    );
    let t_mac = time_op(
        &mut vm,
        |id| VmByteCode::MAC {
            id,
            dst: reg(id, 3, 0),
            src1: reg(id, 3, 1),
            src2: reg(id, 3, 2),
            cst: 3,
        },
        &lut_reg,
        200_000,
        n_threads,
    );
    let t_ks = time_op(
        &mut vm,
        |id| VmByteCode::KS {
            id,
            dst: reg(id, 2, 0),
            src: reg(id, 2, 1),
        },
        &lut_reg,
        2_000,
        n_threads,
    );
    let t_pbs = time_op(
        &mut vm,
        |id| VmByteCode::PBS {
            id,
            dst: reg(id, 2, 0),
            src: reg(id, 2, 1),
            lut: (id % n_luts) as u8,
        },
        &lut_reg,
        200,
        n_threads,
    );

    println!("threads: {n_threads} (1 active, {} idle)", n_threads - 1);
    println!("reps: {REPS}\n");
    println!("{:<6} {:>8} {:>14}", "op", "count", "latency");
    for (name, t, count) in [
        ("ADD", t_add, 200_000),
        ("MAC", t_mac, 200_000),
        ("KS", t_ks, 2_000),
        ("PBS", t_pbs, 200),
    ] {
        println!("{name:<6} {count:>8} {t:>14.3?}");
    }
}
