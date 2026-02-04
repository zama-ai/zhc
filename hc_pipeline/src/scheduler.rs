//! Operation scheduling for optimal HPU execution.
//!
//! This module provides scheduling algorithms that reorder operations in the
//! intermediate representation to minimize execution time while preserving
//! program semantics. The scheduler considers hardware constraints and
//! operation dependencies to produce an optimized execution order.

use hc_ir::{
    Depth, IR, OpId, OpMap, OpRef, ValId, ValMap,
    scheduling::{
        Ready, Retired, Selected,
        forward::{ForwardScheduler, ForwardSimulator},
    },
    traversal::OpWalkerVerifier,
};
use hc_langs::{
    doplang::{Affinity, Argument},
    hpulang::HpuLang,
};
use hc_sim::{
    Cycle, Simulatable, Simulator,
    hpu::{DOp, DOpId, Events, Hpu, HpuConfig, RawDOp},
};
use hc_utils::{StoreIndex, iter::MultiZip, small::SmallSet, small::SmallVec};
use serde::Serialize;

/// Schedules operations in the IR for optimal execution on the target HPU.
///
/// Takes an intermediate representation `ir` containing HPU operations and the
/// hardware configuration `config` to produce a new IR with operations
/// reordered for better performance while preserving semantic correctness.
pub fn schedule<'a, 'b>(ir: &'a IR<HpuLang>, config: &'b HpuConfig) -> IR<HpuLang> {
    let mut scheduler = Scheduler::init(ir, config);
    let schedule = scheduler.schedule(&ir);
    assert_eq!(ir.n_ops() as usize, schedule.len());
    assert!(schedule.get_walker().is_topo_sorted(&ir));
    let should_flush = scheduler.into_should_flush();
    // Produce the scheduled ir.
    let flusher = |opref: &OpRef<HpuLang>| {
        use hc_langs::hpulang::HpuInstructionSet::*;
        if should_flush.contains(&opref.get_id()) {
            match opref.get_operation() {
                Pbs { lut } | PbsF { lut } => PbsF { lut },
                Pbs2 { lut } | Pbs2F { lut } => Pbs2F { lut },
                Pbs4 { lut } | Pbs4F { lut } => Pbs4F { lut },
                Pbs8 { lut } | Pbs8F { lut } => Pbs8F { lut },
                _ => unreachable!(),
            }
        } else {
            opref.get_operation()
        }
    };
    let mut output = IR::empty();
    let mut valmap: ValMap<ValId> = ir.empty_valmap();
    for op in ir.walk_ops_with(schedule.get_walker()) {
        let (_, new_valids) = output
            .add_op(
                flusher(&op),
                op.get_arg_valids()
                    .iter()
                    .map(|a| valmap.get(a).unwrap().to_owned())
                    .collect(),
            )
            .unwrap();
        for (new_valid, old_valid) in (new_valids.iter(), op.get_return_valids().iter()).mzip() {
            valmap.insert(*old_valid, *new_valid);
        }
    }
    output
}

#[derive(Serialize)]
struct Timeouter(Option<DOpId>);

impl Timeouter {
    pub fn new() -> Self {
        Timeouter(None)
    }

    pub fn timeout(&mut self, dopid: DOpId) {
        assert!(std::mem::replace(&mut self.0, Some(dopid)).is_none())
    }

    pub fn did_timeout(&self) -> bool {
        self.0.is_some()
    }

    pub fn acknowledge(&mut self) -> DOpId {
        std::mem::replace(&mut self.0, None).unwrap()
    }
}

impl Simulatable for Timeouter {
    type Event = Events;

    fn handle(
        &mut self,
        _: &mut impl hc_sim::Dispatch<Event = Self::Event>,
        trigger: hc_sim::Trigger<Self::Event>,
    ) {
        match trigger.event {
            Events::NotifyStartOnTimeout { last_in } => {
                self.timeout(last_in.id);
            }
            _ => {}
        }
    }
}

struct Scheduler<'ir> {
    simulator: Simulator<(Hpu, Timeouter)>,
    ir: &'ir IR<HpuLang>,
    affinities: OpMap<Affinity>,
    priorities: OpMap<Depth>,
    mem_buffer: Vec<OpId>,
    alu_buffer: Vec<OpId>,
    pbs_buffer: Vec<OpId>,
    should_flush: SmallSet<OpId>,
    last_pbs_submitted: Option<OpId>,
}

impl<'ir> Scheduler<'ir> {
    pub fn init(ir: &'ir IR<HpuLang>, config: &HpuConfig) -> Scheduler<'ir> {
        use hc_langs::hpulang::HpuInstructionSet::*;
        let config = config.to_owned();
        let simulator =
            Simulator::from_simulatable(config.freq, (Hpu::new(&config), Timeouter::new()));
        let affinities = ir.totally_mapped_opmap(|op| match op.get_operation() {
            AddCt => Affinity::Alu,
            SubCt => Affinity::Alu,
            Mac { .. } => Affinity::Alu,
            AddPt => Affinity::Alu,
            SubPt => Affinity::Alu,
            PtSub => Affinity::Alu,
            MulPt => Affinity::Alu,
            AddCst { .. } => Affinity::Alu,
            SubCst { .. } => Affinity::Alu,
            CstSub { .. } => Affinity::Alu,
            MulCst { .. } => Affinity::Alu,
            ImmLd { .. } => Affinity::Ctl,
            DstSt { .. } => Affinity::Mem,
            SrcLd { .. } => Affinity::Mem,
            Pbs { .. } => Affinity::Pbs,
            Pbs2 { .. } => Affinity::Pbs,
            Pbs4 { .. } => Affinity::Pbs,
            Pbs8 { .. } => Affinity::Pbs,
            PbsF { .. } => Affinity::Pbs,
            Pbs2F { .. } => Affinity::Pbs,
            Pbs4F { .. } => Affinity::Pbs,
            Pbs8F { .. } => Affinity::Pbs,
            _ => unreachable!(
                "Encountered unexpected operations at scheduler init: {}",
                op.format()
            ),
        });
        let priorities = ir.partially_mapped_opmap(|op| Some(op.get_depth()));
        Scheduler {
            simulator,
            affinities,
            priorities,
            ir,
            mem_buffer: Vec::new(),
            alu_buffer: Vec::new(),
            pbs_buffer: Vec::new(),
            should_flush: SmallSet::new(),
            last_pbs_submitted: None,
        }
    }

    pub fn into_should_flush(mut self) -> SmallSet<OpId> {
        if let Some(last_pbs_opid) = self.last_pbs_submitted {
            self.should_flush.insert(last_pbs_opid);
        }
        self.should_flush
    }
}

impl<'ir> ForwardSimulator for Scheduler<'ir> {
    type Dialect = HpuLang;

    fn select(
        &mut self,
        ready: impl Iterator<Item = Ready>,
    ) -> impl Iterator<Item = Selected> + '_ {
        let (hpu, _) = self.simulator.simulatable();

        let mut output = SmallVec::new();

        self.alu_buffer.clear();
        self.mem_buffer.clear();
        self.pbs_buffer.clear();
        for Ready(opid) in ready {
            match self.affinities[opid] {
                Affinity::Alu => self.alu_buffer.push(opid),
                Affinity::Mem => self.mem_buffer.push(opid),
                Affinity::Pbs => self.pbs_buffer.push(opid),
                Affinity::Ctl => output.push(Selected(opid)),
            }
        }
        self.alu_buffer
            .sort_unstable_by_key(|a| self.priorities[*a]);
        self.mem_buffer
            .sort_unstable_by_key(|a| self.priorities[*a]);
        self.pbs_buffer
            .sort_unstable_by_key(|a| self.priorities[*a]);

        // PEA Scheduling
        if let Some(op) = self.alu_buffer.first()
            && hpu.pe_alu.available()
        {
            output.push(Selected(*op));
        }
        // PEM Scheduling
        if let Some(op) = self.mem_buffer.first()
            && hpu.pe_mem.available()
        {
            output.push(Selected(*op));
        }
        // PEP Scheduling
        if let Some(op) = self.pbs_buffer.first()
            && hpu.pe_pbs.available()
        {
            output.push(Selected(*op));

            // Flush policy.
            let size_of_last_waiting_batch = hpu
                .pe_pbs
                .memory()
                .not_yet_workings()
                .iter()
                .fold(0, |acc, a| if a.raw.is_pbs_flush() { 0 } else { acc + 1 });
            if size_of_last_waiting_batch >= hpu.config.pbs_min_batch_size - 1 {
                self.should_flush.insert(*op);
            }
        }

        // Dispatch the selected operations on the simulator .
        self.simulator.dispatch_later(
            Cycle::ONE,
            Events::IscPushDOps(
                output
                    .iter()
                    .filter_map(|Selected(opid)| {
                        opref_to_dop(self.ir.get_op(*opid), self.should_flush.contains(opid))
                    })
                    .collect(),
            ),
        );
        output.into_iter()
    }

    fn advance(&mut self) -> impl Iterator<Item = Retired> {
        self.simulator
            .play_until(|e| matches!(e, Events::IscRetireDOp(_)));
        let (_, timeout) = self.simulator.simulatable_mut();
        if timeout.did_timeout() {
            self.should_flush
                .insert(OpId::from_usize(timeout.acknowledge().0));
        }
        let (hpu, _) = self.simulator.simulatable();
        let retired_opid = hpu.retirement.last_retired().unwrap().id;
        std::iter::once(Retired(OpId::from_usize(retired_opid.0)))
    }
}

fn opref_to_dop<'a>(opref: OpRef<'a, HpuLang>, force_flush: bool) -> Option<DOp> {
    use hc_langs::hpulang::HpuInstructionSet::*;
    let raw = match opref.get_operation() {
        AddCt => Some(RawDOp::ADD {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src1: Argument::ct_reg(opref.get_arg_valids()[0]),
            src2: Argument::ct_reg(opref.get_arg_valids()[1]),
        }),
        SubCt => Some(RawDOp::SUB {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src1: Argument::ct_reg(opref.get_arg_valids()[0]),
            src2: Argument::ct_reg(opref.get_arg_valids()[1]),
        }),
        Mac { .. } => Some(RawDOp::MAC {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src1: Argument::ct_reg(opref.get_arg_valids()[0]),
            src2: Argument::ct_reg(opref.get_arg_valids()[1]),
            cst: Argument::pt_const(0),
        }),
        AddPt | AddCst { .. } => Some(RawDOp::ADDS {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src: Argument::ct_reg(opref.get_arg_valids()[0]),
            cst: Argument::pt_const(0),
        }),
        SubPt | SubCst { .. } => Some(RawDOp::SUBS {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src: Argument::ct_reg(opref.get_arg_valids()[0]),
            cst: Argument::pt_const(0),
        }),
        PtSub | CstSub { .. } => Some(RawDOp::SSUB {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src: Argument::ct_reg(opref.get_arg_valids()[0]),
            cst: Argument::pt_const(0),
        }),
        MulPt | MulCst { .. } => Some(RawDOp::MULS {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src: Argument::ct_reg(opref.get_arg_valids()[0]),
            cst: Argument::pt_const(0),
        }),
        ImmLd { .. } => None,
        DstSt { to } => Some(RawDOp::ST {
            dst: Argument::ct_var(
                to.dst_pos.try_into().unwrap(),
                to.block_pos.try_into().unwrap(),
            ),
            src: Argument::ct_reg(opref.get_arg_valids()[0]),
        }),
        SrcLd { from } => Some(RawDOp::LD {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src: Argument::ct_var(
                from.src_pos.try_into().unwrap(),
                from.block_pos.try_into().unwrap(),
            ),
        }),
        Pbs { lut } if !force_flush => Some(RawDOp::PBS {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        Pbs2 { lut } if !force_flush => Some(RawDOp::PBS_ML2 {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg2(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        Pbs4 { lut } if !force_flush => Some(RawDOp::PBS_ML4 {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg4(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        Pbs8 { lut } if !force_flush => Some(RawDOp::PBS_ML8 {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg8(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        PbsF { lut } | Pbs { lut } if force_flush => Some(RawDOp::PBS_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        Pbs2F { lut } | Pbs2 { lut } if force_flush => Some(RawDOp::PBS_ML2_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg2(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        Pbs4F { lut } | Pbs4 { lut } if force_flush => Some(RawDOp::PBS_ML4_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg4(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        Pbs8F { lut } | Pbs8 { lut } if force_flush => Some(RawDOp::PBS_ML8_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg8(opref.get_return_valids()[0]),
            lut: Argument::lut_id(lut),
        }),
        a => unreachable!("Entered unreachable state: {}", a),
    };

    raw.map(|raw| DOp {
        raw,
        id: DOpId(opref.get_id().into()),
    })
}

#[cfg(test)]
mod test {
    use hc_ir::{IR, PrintWalker, translation::Translator};
    use hc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use hc_sim::hpu::{HpuConfig, PhysicalConfig};
    use hc_utils::assert_display_is;

    use crate::{
        test::{get_add_ir, get_cmp_ir},
        translation::IoplangToHpulang,
    };

    use super::schedule;

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        schedule(&ir, &config)
    }

    #[test]
    fn test_schedule_add_ir() {
        let ir = pipeline(&get_add_ir(16, 2, 2));
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 : CtRegister = src_ld<0.0_tsrc>();
                %1 : CtRegister = src_ld<0.1_tsrc>();
                %2 : CtRegister = src_ld<0.2_tsrc>();
                %3 : CtRegister = src_ld<0.3_tsrc>();
                %4 : CtRegister = src_ld<0.4_tsrc>();
                %5 : CtRegister = src_ld<0.5_tsrc>();
                %6 : CtRegister = src_ld<0.6_tsrc>();
                %7 : CtRegister = src_ld<0.7_tsrc>();
                %8 : CtRegister = src_ld<1.0_tsrc>();
                %9 : CtRegister = add_ct(%0 : CtRegister, %8 : CtRegister);
                %10 : CtRegister = src_ld<1.1_tsrc>();
                %11 : CtRegister = src_ld<1.2_tsrc>();
                %12 : CtRegister = src_ld<1.3_tsrc>();
                %13 : CtRegister = src_ld<1.4_tsrc>();
                %14 : CtRegister = add_ct(%1 : CtRegister, %10 : CtRegister);
                %15 : CtRegister, %16 : CtRegister = pbs_2<Lut@26>(%9 : CtRegister);
                %17 : CtRegister = src_ld<1.5_tsrc>();
                %18 : CtRegister = src_ld<1.6_tsrc>();
                %19 : CtRegister = src_ld<1.7_tsrc>();
                %20 : CtRegister = add_ct(%2 : CtRegister, %11 : CtRegister);
                %21 : CtRegister = pbs<Lut@47>(%14 : CtRegister);
                %22 : CtRegister = add_ct(%3 : CtRegister, %12 : CtRegister);
                %23 : CtRegister = pbs<Lut@48>(%20 : CtRegister);
                %24 : CtRegister = add_ct(%4 : CtRegister, %13 : CtRegister);
                %25 : CtRegister = pbs<Lut@49>(%22 : CtRegister);
                %26 : CtRegister = add_ct(%5 : CtRegister, %17 : CtRegister);
                %27 : CtRegister = pbs<Lut@47>(%24 : CtRegister);
                %28 : CtRegister = add_ct(%6 : CtRegister, %18 : CtRegister);
                %29 : CtRegister = pbs<Lut@48>(%26 : CtRegister);
                %30 : CtRegister = add_ct(%7 : CtRegister, %19 : CtRegister);
                %31 : CtRegister = pbs_f<Lut@49>(%28 : CtRegister);
                %32 : CtRegister = add_ct(%14 : CtRegister, %16 : CtRegister);
                %33 : CtRegister = pbs<Lut@1>(%15 : CtRegister);
                %34 : CtRegister = add_ct(%16 : CtRegister, %21 : CtRegister);
                %35 : CtRegister = pbs<Lut@1>(%32 : CtRegister);
                %36 : CtRegister = add_ct(%27 : CtRegister, %29 : CtRegister);
                %37 : CtRegister = pbs_f<Lut@44>(%34 : CtRegister);
                %38 : CtRegister = add_ct(%34 : CtRegister, %23 : CtRegister);
                %39 : CtRegister = add_ct(%36 : CtRegister, %31 : CtRegister);
                dst_st<0.0_tdst>(%33 : CtRegister);
                dst_st<0.1_tdst>(%35 : CtRegister);
                %40 : CtRegister = add_ct(%38 : CtRegister, %25 : CtRegister);
                %41 : CtRegister = pbs<Lut@45>(%38 : CtRegister);
                %42 : CtRegister = add_ct(%20 : CtRegister, %37 : CtRegister);
                %43 : CtRegister = pbs<Lut@46>(%40 : CtRegister);
                %44 : CtRegister = pbs_f<Lut@1>(%42 : CtRegister);
                %45 : CtRegister = add_ct(%22 : CtRegister, %41 : CtRegister);
                dst_st<0.2_tdst>(%44 : CtRegister);
                %46 : CtRegister = add_ct(%27 : CtRegister, %43 : CtRegister);
                %47 : CtRegister = pbs<Lut@1>(%45 : CtRegister);
                %48 : CtRegister = add_ct(%36 : CtRegister, %43 : CtRegister);
                %49 : CtRegister = pbs<Lut@44>(%46 : CtRegister);
                %50 : CtRegister = add_ct(%39 : CtRegister, %43 : CtRegister);
                %51 : CtRegister = pbs<Lut@45>(%48 : CtRegister);
                %52 : CtRegister = add_ct(%24 : CtRegister, %43 : CtRegister);
                %53 : CtRegister = pbs<Lut@46>(%50 : CtRegister);
                %54 : CtRegister = pbs_f<Lut@1>(%52 : CtRegister);
                dst_st<0.3_tdst>(%47 : CtRegister);
                %55 : CtRegister = add_ct(%26 : CtRegister, %49 : CtRegister);
                dst_st<0.4_tdst>(%54 : CtRegister);
                %56 : CtRegister = add_ct(%28 : CtRegister, %51 : CtRegister);
                %57 : CtRegister = pbs<Lut@1>(%55 : CtRegister);
                %58 : CtRegister = add_ct(%30 : CtRegister, %53 : CtRegister);
                %59 : CtRegister = pbs<Lut@1>(%56 : CtRegister);
                %60 : CtRegister = pbs_f<Lut@1>(%58 : CtRegister);
                dst_st<0.5_tdst>(%57 : CtRegister);
                dst_st<0.6_tdst>(%59 : CtRegister);
                dst_st<0.7_tdst>(%60 : CtRegister);
            "#
        );
    }

    #[test]
    fn test_schedule_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = mac<4_imm>(%1 : CtRegister, %0 : CtRegister);
            %3 : CtRegister = src_ld<0.2_tsrc>();
            %4 : CtRegister = src_ld<0.3_tsrc>();
            %5 : CtRegister = src_ld<0.4_tsrc>();
            %6 : CtRegister = src_ld<0.5_tsrc>();
            %7 : CtRegister = mac<4_imm>(%4 : CtRegister, %3 : CtRegister);
            %8 : CtRegister = pbs<Lut@0>(%2 : CtRegister);
            %9 : CtRegister = src_ld<0.6_tsrc>();
            %10 : CtRegister = src_ld<0.7_tsrc>();
            %11 : CtRegister = src_ld<1.0_tsrc>();
            %12 : CtRegister = src_ld<1.1_tsrc>();
            %13 : CtRegister = mac<4_imm>(%6 : CtRegister, %5 : CtRegister);
            %14 : CtRegister = pbs<Lut@0>(%7 : CtRegister);
            %15 : CtRegister = src_ld<1.2_tsrc>();
            %16 : CtRegister = src_ld<1.3_tsrc>();
            %17 : CtRegister = src_ld<1.4_tsrc>();
            %18 : CtRegister = src_ld<1.5_tsrc>();
            %19 : CtRegister = mac<4_imm>(%10 : CtRegister, %9 : CtRegister);
            %20 : CtRegister = pbs<Lut@0>(%13 : CtRegister);
            %21 : CtRegister = src_ld<1.6_tsrc>();
            %22 : CtRegister = src_ld<1.7_tsrc>();
            %23 : CtRegister = mac<4_imm>(%12 : CtRegister, %11 : CtRegister);
            %24 : CtRegister = pbs<Lut@0>(%19 : CtRegister);
            %25 : CtRegister = mac<4_imm>(%16 : CtRegister, %15 : CtRegister);
            %26 : CtRegister = pbs<Lut@0>(%23 : CtRegister);
            %27 : CtRegister = mac<4_imm>(%18 : CtRegister, %17 : CtRegister);
            %28 : CtRegister = pbs<Lut@0>(%25 : CtRegister);
            %29 : CtRegister = mac<4_imm>(%22 : CtRegister, %21 : CtRegister);
            %30 : CtRegister = pbs<Lut@0>(%27 : CtRegister);
            %31 : CtRegister = pbs_f<Lut@0>(%29 : CtRegister);
            %32 : CtRegister = sub_ct(%8 : CtRegister, %26 : CtRegister);
            %33 : CtRegister = sub_ct(%14 : CtRegister, %28 : CtRegister);
            %34 : CtRegister = pbs<Lut@10>(%32 : CtRegister);
            %35 : CtRegister = sub_ct(%20 : CtRegister, %30 : CtRegister);
            %36 : CtRegister = pbs<Lut@10>(%33 : CtRegister);
            %37 : CtRegister = sub_ct(%24 : CtRegister, %31 : CtRegister);
            %38 : CtRegister = pbs<Lut@10>(%35 : CtRegister);
            %39 : CtRegister = pbs_f<Lut@10>(%37 : CtRegister);
            %40 : CtRegister = add_cst<1_imm>(%34 : CtRegister);
            %41 : CtRegister = add_cst<1_imm>(%36 : CtRegister);
            %42 : CtRegister = add_cst<1_imm>(%38 : CtRegister);
            %43 : CtRegister = add_cst<1_imm>(%39 : CtRegister);
            %44 : CtRegister = mac<4_imm>(%41 : CtRegister, %40 : CtRegister);
            %45 : CtRegister = mac<4_imm>(%43 : CtRegister, %42 : CtRegister);
            %46 : CtRegister = pbs<Lut@11>(%44 : CtRegister);
            %47 : CtRegister = pbs_f<Lut@11>(%45 : CtRegister);
            %48 : CtRegister = mac<4_imm>(%47 : CtRegister, %46 : CtRegister);
            %49 : CtRegister = pbs_f<Lut@27>(%48 : CtRegister);
            dst_st<0.0_tdst>(%49 : CtRegister);
        "#
        );
    }
}
