use std::collections::BinaryHeap;

use zhc_ir::{AnnIR, AnnOpRef, IR, OpId, scheduler::reschedule};
use zhc_langs::hpulang::HpuLang;
use zhc_sim::{
    Cycle,
    hpu::{ConstantLatency, FlatLinLatency, HpuConfig},
};
use zhc_utils::{Dumpable, fsm, iter::CollectInVec, svec};

type Height = u16;
type HeightedOpRef<'a, 'b> = AnnOpRef<'a, 'b, HpuLang, Height, ()>;
type HeightedIR<'a> = AnnIR<'a, HpuLang, Height, ()>;

fn analyze_height<'a>(ir: &'a IR<HpuLang>) -> HeightedIR<'a> {
    use zhc_langs::hpulang::HpuInstructionSet::*;
    ir.backward_dataflow_analysis(|opref| match opref.get_instruction() {
        Batch { .. } => {
            let height = opref
                .get_users_iter()
                .map(|p| p.get_annotation().clone().unwrap_analyzed())
                .max()
                .unwrap()
                + 1_u16;
            (height, svec![(); opref.get_return_arity()])
        }
        _ => {
            let height = opref
                .get_users_iter()
                .map(|p| p.get_annotation().clone().unwrap_analyzed())
                .max()
                .unwrap_or(0_u16);
            (height, svec![(); opref.get_return_arity()])
        }
    })
}

#[fsm]
#[derive(Debug)]
enum State {
    Scheduled,
    Ready,
    Waiting(usize),
}

impl Dumpable for State {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
struct RetireDop<'a, 'b> {
    at: Cycle,
    op: HeightedOpRef<'a, 'b>,
}

impl<'a, 'b> Dumpable for RetireDop<'a, 'b> {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl<'a, 'b> PartialEq for RetireDop<'a, 'b> {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at && self.op == other.op
    }
}

impl<'a, 'b> Eq for RetireDop<'a, 'b> {}

impl<'a, 'b> PartialOrd for RetireDop<'a, 'b> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.at.partial_cmp(&self.at)
    }
}

impl<'a, 'b> Ord for RetireDop<'a, 'b> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

enum Affinity {
    Pea,
    Pem,
    Pep,
    Ctl,
}

fn get_op_affinity<'a, 'b>(op: &HeightedOpRef<'a, 'b>) -> Affinity {
    use zhc_langs::hpulang::HpuInstructionSet::*;
    match op.get_instruction() {
        AddCt
        | SubCt
        | Mac { .. }
        | AddPt
        | SubPt
        | PtSub
        | MulPt
        | AddCst { .. }
        | SubCst { .. }
        | CstSub { .. }
        | MulCst { .. } => Affinity::Pea,
        CstCt { .. } => Affinity::Ctl,
        ImmLd { .. } | DstSt { .. } | SrcLd { .. } => Affinity::Pem,
        Batch { .. } => Affinity::Pep,
        _ => unreachable!(),
    }
}

fn get_op_latency<'a, 'b>(op: &HeightedOpRef<'a, 'b>, config: &HpuConfig) -> Cycle {
    match get_op_affinity(op) {
        Affinity::Pea => ConstantLatency::new(config.alu_write_latency).compute_latency(),
        Affinity::Pem => ConstantLatency::new(config.mem_write_latency).compute_latency(),
        Affinity::Pep => {
            let zhc_langs::hpulang::HpuInstructionSet::Batch { block } = op.get_instruction()
            else {
                unreachable!()
            };
            let batch_size = block
                .walk_ops_linear()
                .filter(|op| op.get_instruction().is_pbs())
                .count();
            FlatLinLatency::new(
                config.pbs_processing_latency_a,
                config.pbs_processing_latency_b,
                config.pbs_processing_latency_m,
            )
            .compute_latency(batch_size)
        }
        Affinity::Ctl => Cycle(0),
    }
}

fn forward_schedule<'a, 'b>(ir: &'b HeightedIR<'a>, config: &HpuConfig) -> Vec<OpId> {
    let mut output = Vec::new();

    let mut states = ir.totally_mapped_opmap(|op| match op.get_predecessors_iter().count() {
        0 => State::Ready,
        n => State::Waiting(n),
    });

    let mut pep_busy = false;
    let mut pea_busy = false;
    let mut pem_busy = false;

    let mut events = BinaryHeap::new();
    let mut pep_ready = states
        .iter()
        .filter(|(opid, state)| {
            matches!(
                (state, get_op_affinity(&ir.get_op(*opid))),
                (State::Ready, Affinity::Pep)
            )
        })
        .map(|(opid, _)| ir.get_op(opid))
        .covec();
    let mut pea_ready = states
        .iter()
        .filter(|(opid, state)| {
            matches!(
                (state, get_op_affinity(&ir.get_op(*opid))),
                (State::Ready, Affinity::Pea)
            )
        })
        .map(|(opid, _)| ir.get_op(opid))
        .covec();
    let mut pem_ready = states
        .iter()
        .filter(|(opid, state)| {
            matches!(
                (state, get_op_affinity(&ir.get_op(*opid))),
                (State::Ready, Affinity::Pem)
            )
        })
        .map(|(opid, _)| ir.get_op(opid))
        .covec();
    let mut ctl_ready = states
        .iter()
        .filter(|(opid, state)| {
            matches!(
                (state, get_op_affinity(&ir.get_op(*opid))),
                (State::Ready, Affinity::Ctl)
            )
        })
        .map(|(opid, _)| ir.get_op(opid))
        .covec();

    while !ctl_ready.is_empty() {
        let op = ctl_ready.pop().unwrap();
        output.push(op.get_id());
        events.push(RetireDop { at: Cycle(0), op });
    }

    if !pep_busy && !pep_ready.is_empty() {
        pep_busy = true;
        // pep_ready.shuffle(&mut rand::rng());
        pep_ready.sort_by_key(|op| *op.get_annotation());
        let op = pep_ready.pop().unwrap();
        let at = Cycle(0) + get_op_latency(&op, config);
        output.push(op.get_id());
        events.push(RetireDop { at, op });
    }

    if !pem_busy && !pem_ready.is_empty() {
        pem_busy = true;
        // pem_ready.shuffle(&mut rand::rng());
        pem_ready.sort_by_key(|op| *op.get_annotation());
        let op = pem_ready.pop().unwrap();
        let at = Cycle(0) + get_op_latency(&op, config);
        output.push(op.get_id());
        events.push(RetireDop { at, op });
    }

    if !pea_busy && !pea_ready.is_empty() {
        pea_busy = true;
        // pea_ready.shuffle(&mut rand::rng());
        pea_ready.sort_by_key(|op| *op.get_annotation());
        let op = pea_ready.pop().unwrap();
        let at = Cycle(0) + get_op_latency(&op, config);
        output.push(op.get_id());
        events.push(RetireDop { at, op });
    }

    loop {
        let Some(RetireDop { at, op }) = events.pop() else {
            break;
        };

        let current_cycle = at;
        match get_op_affinity(&op) {
            Affinity::Pea => pea_busy = false,
            Affinity::Pem => pem_busy = false,
            Affinity::Pep => pep_busy = false,
            Affinity::Ctl => {}
        }
        for user in op.get_users_iter() {
            states.get_mut(&user).unwrap().transition(|old| match old {
                State::Waiting(0) => {
                    unreachable!()
                }
                State::Waiting(1) => match get_op_affinity(&user) {
                    Affinity::Pea => {
                        pea_ready.push(user);
                        State::Ready
                    }
                    Affinity::Pem => {
                        pem_ready.push(user);
                        State::Ready
                    }
                    Affinity::Pep => {
                        pep_ready.push(user);
                        State::Ready
                    }
                    Affinity::Ctl => {
                        ctl_ready.push(user);
                        State::Scheduled
                    }
                },
                State::Waiting(n) => State::Waiting(n - 1),
                state => unreachable!("Found unexpected state {state:?} for op: {}", op.format()),
            });
        }

        while !ctl_ready.is_empty() {
            let op = ctl_ready.pop().unwrap();
            let at = current_cycle + 1;
            output.push(op.get_id());
            events.push(RetireDop { at, op });
        }

        if !pep_busy && !pep_ready.is_empty() {
            pep_busy = true;
            // pep_ready.shuffle(&mut rand::rng());
            pep_ready.sort_by_key(|op| *op.get_annotation());
            let op = pep_ready.pop().unwrap();
            let at = current_cycle + get_op_latency(&op, config);
            output.push(op.get_id());
            events.push(RetireDop { at, op });
        }

        if !pem_busy && !pem_ready.is_empty() {
            pem_busy = true;
            // pem_ready.shuffle(&mut rand::rng());
            pem_ready.sort_by_key(|op| *op.get_annotation());
            let op = pem_ready.pop().unwrap();
            let at = current_cycle + get_op_latency(&op, config);
            output.push(op.get_id());
            events.push(RetireDop { at, op });
        }

        if !pea_busy && !pea_ready.is_empty() {
            pea_busy = true;
            // pea_ready.shuffle(&mut rand::rng());
            pea_ready.sort_by_key(|op| *op.get_annotation());
            let op = pea_ready.pop().unwrap();
            let at = current_cycle + get_op_latency(&op, config);
            output.push(op.get_id());
            events.push(RetireDop { at, op });
        }
    }

    output
}

pub fn schedule<'a, 'b>(ir: &'a IR<HpuLang>, config: &HpuConfig) -> IR<HpuLang> {
    let heighted = analyze_height(ir);
    let schedule = forward_schedule(&heighted, config);
    reschedule(ir, schedule.into_iter()).0
}

#[cfg(test)]
mod test {
    use zhc_builder::{CiphertextSpec, count_0};
    use zhc_ir::IR;
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_sim::hpu::{HpuConfig, PhysicalConfig};
    use zhc_utils::assert_display_is;

    use crate::{
        batch_scheduler::schedule, batcher::batch, test::check_iop_hpu_equivalence,
        translation::lower_iop_to_hpu,
    };

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = lower_iop_to_hpu(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let batched = batch(&ir, &config);
        let scheduled = schedule(&batched, &config);
        scheduled
    }

    #[test]
    fn test_scheduler() {
        let ir = pipeline(&count_0(CiphertextSpec::new(16, 2, 2)).into_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 = src_ld<0.7_tsrc>();
                %1 = src_ld<0.6_tsrc>();
                %2 = src_ld<0.5_tsrc>();
                %3 = src_ld<0.4_tsrc>();
                %4 = src_ld<0.3_tsrc>();
                %5 = src_ld<0.2_tsrc>();
                %6 = src_ld<0.1_tsrc>();
                %7 = src_ld<0.0_tsrc>();
                %8, %9, %10, %11, %12, %13, %14, %15, %16, %17, %18, %19, %20, %21, %22, %23 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = batch_arg<5, CtRegister>();
                    %a6 = batch_arg<6, CtRegister>();
                    %a7 = batch_arg<7, CtRegister>();
                    %a8, %a9 = pbs_2<Lut@71>(%a0);
                    %a10, %a11 = pbs_2<Lut@71>(%a3);
                    %a12, %a13 = pbs_2<Lut@71>(%a4);
                    %a14, %a15 = pbs_2<Lut@71>(%a1);
                    %a16, %a17 = pbs_2<Lut@71>(%a5);
                    %a18, %a19 = pbs_2<Lut@71>(%a2);
                    %a20, %a21 = pbs_2<Lut@71>(%a6);
                    %a22, %a23 = pbs_2f<Lut@71>(%a7);
                    batch_ret<0, CtRegister>(%a8);
                    batch_ret<1, CtRegister>(%a9);
                    batch_ret<2, CtRegister>(%a14);
                    batch_ret<3, CtRegister>(%a15);
                    batch_ret<4, CtRegister>(%a18);
                    batch_ret<5, CtRegister>(%a19);
                    batch_ret<6, CtRegister>(%a10);
                    batch_ret<7, CtRegister>(%a11);
                    batch_ret<8, CtRegister>(%a12);
                    batch_ret<9, CtRegister>(%a13);
                    batch_ret<10, CtRegister>(%a16);
                    batch_ret<11, CtRegister>(%a17);
                    batch_ret<12, CtRegister>(%a20);
                    batch_ret<13, CtRegister>(%a21);
                    batch_ret<14, CtRegister>(%a22);
                    batch_ret<15, CtRegister>(%a23);
                }(%7, %6, %5, %4, %3, %2, %1, %0);
                %24 = add_ct(%22, %23);
                %25 = add_ct(%15, %16);
                %26 = add_ct(%25, %17);
                %27 = add_ct(%26, %18);
                %28 = add_ct(%27, %19);
                %29 = add_ct(%28, %20);
                %30 = add_ct(%29, %21);
                %31 = add_ct(%8, %9);
                %32 = add_ct(%31, %10);
                %33 = add_ct(%32, %11);
                %34 = add_ct(%33, %12);
                %35 = add_ct(%34, %13);
                %36 = add_ct(%35, %14);
                %37, %38, %39, %40, %41, %42 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3, %a4 = pbs_2<Lut@70>(%a0);
                    %a5, %a6 = pbs_2<Lut@70>(%a1);
                    %a7, %a8 = pbs_2f<Lut@65>(%a2);
                    batch_ret<0, CtRegister>(%a3);
                    batch_ret<1, CtRegister>(%a4);
                    batch_ret<2, CtRegister>(%a5);
                    batch_ret<3, CtRegister>(%a6);
                    batch_ret<4, CtRegister>(%a7);
                    batch_ret<5, CtRegister>(%a8);
                }(%36, %30, %24);
                %43 = add_ct(%38, %40);
                %44 = add_ct(%43, %42);
                %45 = add_ct(%37, %39);
                %46 = add_ct(%45, %41);
                %47, %48, %49, %50 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = pbs<Lut@3>(%a0);
                    %a3, %a4 = pbs_2<Lut@26>(%a1);
                    %a5 = pbs_f<Lut@1>(%a0);
                    batch_ret<0, CtRegister>(%a5);
                    batch_ret<1, CtRegister>(%a2);
                    batch_ret<2, CtRegister>(%a3);
                    batch_ret<3, CtRegister>(%a4);
                }(%46, %44);
                dst_st<0.0_tdst>(%47);
                %51 = add_ct(%48, %49);
                %52, %53 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1, %a2 = pbs_2f<Lut@26>(%a0);
                    batch_ret<0, CtRegister>(%a1);
                    batch_ret<1, CtRegister>(%a2);
                }(%51);
                dst_st<0.1_tdst>(%52);
                %54 = add_ct(%53, %50);
                %55, %56 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1, %a2 = pbs_2f<Lut@26>(%a0);
                    batch_ret<0, CtRegister>(%a1);
                    batch_ret<1, CtRegister>(%a2);
                }(%54);
                dst_st<0.2_tdst>(%55);
            "#
        )
    }

    #[test]
    fn correctness() {
        use zhc_builder::*;
        let check = |b: Builder| {
            let spec = *b.spec();
            let iop_ir = b.into_ir();
            let hpu_ir = pipeline(&iop_ir);
            check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
        };
        for size in 2..=64 {
            let spec = CiphertextSpec::new(size, 2, 2);
            check(add(spec));
            check(bitwise_and(spec));
            check(bitwise_or(spec));
            check(bitwise_xor(spec));
            check(if_then_else(spec));
            check(if_then_zero(spec));
            check(mul_lsb(spec));
        }
    }
}
