use std::collections::BinaryHeap;

use zhc_ir::{AnnIR, AnnOpRef, IR, OpId, scheduler::reschedule};
use zhc_langs::hpulang::HpuLang;
use zhc_sim::{
    Cycle,
    hpu::{ConstantLatency, FlatLinLatency, HpuConfig},
};
use zhc_utils::{
    Dumpable, FastMap, fsm,
    iter::{CollectInSmallVec, CollectInVec},
    svec,
};

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
        TransferOut { .. } => {
            let max_height = 1 << 14; // whatever makes it better lol.
            (max_height, svec![(); opref.get_return_arity()])
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
        CstCt { .. } | TransferIn { .. } | TransferOut { .. } => Affinity::Ctl,
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
    let mut schedule = forward_schedule(&heighted, config);
    // Ugly patch to push the transfers to occur as late as possible.
    let transfer_ins: FastMap<_, _> = ir
        .walk_ops_linear()
        .filter(|p| p.get_instruction().is_transfer_in())
        .map(|op| {
            (
                op.get_id(),
                op.get_users_iter().map(|u| u.get_id()).cosvec(),
            )
        })
        .collect();
    for (tid, uids) in transfer_ins.into_iter() {
        let pos = schedule.iter().position(|op| *op == tid).unwrap();
        schedule.remove(pos);
        let pos = schedule
            .iter()
            .position(|op| uids.as_slice().contains(op))
            .unwrap();
        schedule.insert(pos, tid);
    }
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
                %0 : CtRegister = src_ld<0.7_tsrc>();
                %1 : CtRegister = src_ld<0.6_tsrc>();
                %2 : CtRegister = src_ld<0.5_tsrc>();
                %3 : CtRegister = src_ld<0.4_tsrc>();
                %4 : CtRegister = src_ld<0.3_tsrc>();
                %5 : CtRegister = src_ld<0.2_tsrc>();
                %6 : CtRegister = src_ld<0.1_tsrc>();
                %7 : CtRegister = src_ld<0.0_tsrc>();
                %8 : CtRegister, %9 : CtRegister, %10 : CtRegister, %11 : CtRegister, %12 : CtRegister, %13 : CtRegister, %14 : CtRegister, %15 : CtRegister, %16 : CtRegister, %17 : CtRegister, %18 : CtRegister, %19 : CtRegister, %20 : CtRegister, %21 : CtRegister, %22 : CtRegister, %23 : CtRegister = batch {
                    %a0 : CtRegister = batch_arg<0, CtRegister>();
                    %a1 : CtRegister = batch_arg<1, CtRegister>();
                    %a2 : CtRegister = batch_arg<2, CtRegister>();
                    %a3 : CtRegister = batch_arg<3, CtRegister>();
                    %a4 : CtRegister = batch_arg<4, CtRegister>();
                    %a5 : CtRegister = batch_arg<5, CtRegister>();
                    %a6 : CtRegister = batch_arg<6, CtRegister>();
                    %a7 : CtRegister = batch_arg<7, CtRegister>();
                    %a8 : CtRegister, %a9 : CtRegister = pbs_2<Lut@71>(%a0 : CtRegister);
                    %a10 : CtRegister, %a11 : CtRegister = pbs_2<Lut@71>(%a3 : CtRegister);
                    %a12 : CtRegister, %a13 : CtRegister = pbs_2<Lut@71>(%a4 : CtRegister);
                    %a14 : CtRegister, %a15 : CtRegister = pbs_2<Lut@71>(%a1 : CtRegister);
                    %a16 : CtRegister, %a17 : CtRegister = pbs_2<Lut@71>(%a5 : CtRegister);
                    %a18 : CtRegister, %a19 : CtRegister = pbs_2<Lut@71>(%a2 : CtRegister);
                    %a20 : CtRegister, %a21 : CtRegister = pbs_2<Lut@71>(%a6 : CtRegister);
                    %a22 : CtRegister, %a23 : CtRegister = pbs_2f<Lut@71>(%a7 : CtRegister);
                    batch_ret<0, CtRegister>(%a8 : CtRegister);
                    batch_ret<1, CtRegister>(%a9 : CtRegister);
                    batch_ret<2, CtRegister>(%a14 : CtRegister);
                    batch_ret<3, CtRegister>(%a15 : CtRegister);
                    batch_ret<4, CtRegister>(%a18 : CtRegister);
                    batch_ret<5, CtRegister>(%a19 : CtRegister);
                    batch_ret<6, CtRegister>(%a10 : CtRegister);
                    batch_ret<7, CtRegister>(%a11 : CtRegister);
                    batch_ret<8, CtRegister>(%a12 : CtRegister);
                    batch_ret<9, CtRegister>(%a13 : CtRegister);
                    batch_ret<10, CtRegister>(%a16 : CtRegister);
                    batch_ret<11, CtRegister>(%a17 : CtRegister);
                    batch_ret<12, CtRegister>(%a20 : CtRegister);
                    batch_ret<13, CtRegister>(%a21 : CtRegister);
                    batch_ret<14, CtRegister>(%a22 : CtRegister);
                    batch_ret<15, CtRegister>(%a23 : CtRegister);
                }(%7 : CtRegister, %6 : CtRegister, %5 : CtRegister, %4 : CtRegister, %3 : CtRegister, %2 : CtRegister, %1 : CtRegister, %0 : CtRegister);
                %24 : CtRegister = add_ct(%15 : CtRegister, %16 : CtRegister);
                %30 : CtRegister = add_ct(%8 : CtRegister, %9 : CtRegister);
                %36 : CtRegister = add_ct(%22 : CtRegister, %23 : CtRegister);
                %25 : CtRegister = add_ct(%24 : CtRegister, %17 : CtRegister);
                %31 : CtRegister = add_ct(%30 : CtRegister, %10 : CtRegister);
                %26 : CtRegister = add_ct(%25 : CtRegister, %18 : CtRegister);
                %32 : CtRegister = add_ct(%31 : CtRegister, %11 : CtRegister);
                %27 : CtRegister = add_ct(%26 : CtRegister, %19 : CtRegister);
                %33 : CtRegister = add_ct(%32 : CtRegister, %12 : CtRegister);
                %28 : CtRegister = add_ct(%27 : CtRegister, %20 : CtRegister);
                %34 : CtRegister = add_ct(%33 : CtRegister, %13 : CtRegister);
                %29 : CtRegister = add_ct(%28 : CtRegister, %21 : CtRegister);
                %35 : CtRegister = add_ct(%34 : CtRegister, %14 : CtRegister);
                %37 : CtRegister, %38 : CtRegister, %39 : CtRegister, %40 : CtRegister, %41 : CtRegister, %42 : CtRegister = batch {
                    %a0 : CtRegister = batch_arg<0, CtRegister>();
                    %a1 : CtRegister = batch_arg<1, CtRegister>();
                    %a2 : CtRegister = batch_arg<2, CtRegister>();
                    %a3 : CtRegister, %a4 : CtRegister = pbs_2<Lut@70>(%a1 : CtRegister);
                    %a5 : CtRegister, %a6 : CtRegister = pbs_2<Lut@70>(%a2 : CtRegister);
                    %a7 : CtRegister, %a8 : CtRegister = pbs_2f<Lut@65>(%a0 : CtRegister);
                    batch_ret<0, CtRegister>(%a7 : CtRegister);
                    batch_ret<1, CtRegister>(%a8 : CtRegister);
                    batch_ret<2, CtRegister>(%a3 : CtRegister);
                    batch_ret<3, CtRegister>(%a4 : CtRegister);
                    batch_ret<4, CtRegister>(%a5 : CtRegister);
                    batch_ret<5, CtRegister>(%a6 : CtRegister);
                }(%36 : CtRegister, %35 : CtRegister, %29 : CtRegister);
                %43 : CtRegister = add_ct(%40 : CtRegister, %42 : CtRegister);
                %45 : CtRegister = add_ct(%39 : CtRegister, %41 : CtRegister);
                %44 : CtRegister = add_ct(%43 : CtRegister, %38 : CtRegister);
                %46 : CtRegister = add_ct(%45 : CtRegister, %37 : CtRegister);
                %47 : CtRegister, %48 : CtRegister, %49 : CtRegister, %50 : CtRegister = batch {
                    %a0 : CtRegister = batch_arg<0, CtRegister>();
                    %a1 : CtRegister = batch_arg<1, CtRegister>();
                    %a2 : CtRegister = pbs<Lut@3>(%a0 : CtRegister);
                    %a3 : CtRegister, %a4 : CtRegister = pbs_2<Lut@26>(%a1 : CtRegister);
                    %a5 : CtRegister = pbs_f<Lut@1>(%a0 : CtRegister);
                    batch_ret<0, CtRegister>(%a5 : CtRegister);
                    batch_ret<1, CtRegister>(%a2 : CtRegister);
                    batch_ret<2, CtRegister>(%a3 : CtRegister);
                    batch_ret<3, CtRegister>(%a4 : CtRegister);
                }(%46 : CtRegister, %44 : CtRegister);
                dst_st<0.0_tdst>(%47 : CtRegister);
                %51 : CtRegister = add_ct(%48 : CtRegister, %49 : CtRegister);
                %52 : CtRegister, %53 : CtRegister = batch {
                    %a0 : CtRegister = batch_arg<0, CtRegister>();
                    %a1 : CtRegister, %a2 : CtRegister = pbs_2f<Lut@26>(%a0 : CtRegister);
                    batch_ret<0, CtRegister>(%a1 : CtRegister);
                    batch_ret<1, CtRegister>(%a2 : CtRegister);
                }(%51 : CtRegister);
                dst_st<0.1_tdst>(%52 : CtRegister);
                %54 : CtRegister = add_ct(%53 : CtRegister, %50 : CtRegister);
                %55 : CtRegister, %56 : CtRegister = batch {
                    %a0 : CtRegister = batch_arg<0, CtRegister>();
                    %a1 : CtRegister, %a2 : CtRegister = pbs_2f<Lut@26>(%a0 : CtRegister);
                    batch_ret<0, CtRegister>(%a1 : CtRegister);
                    batch_ret<1, CtRegister>(%a2 : CtRegister);
                }(%54 : CtRegister);
                dst_st<0.2_tdst>(%55 : CtRegister);
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
