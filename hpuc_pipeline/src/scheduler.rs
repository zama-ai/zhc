use hpuc_ir::{
    IR, OpId, OpMap, OpRef,
    scheduling::{Ready, Retired, Selected, forward::ForwardSimulator},
};
use hpuc_langs::{
    doplang::{Affinity, Argument},
    hpulang::Hpulang,
};
use hpuc_sim::{
    Cycle, Simulator,
    hpu::{DOp, DOpId, Events, Hpu, HpuConfig, Policy, RawDOp},
};
use hpuc_utils::{SmallSet, SmallVec, StoreIndex};

pub struct Scheduler<'ir> {
    simulator: Simulator<Hpu>,
    ir: &'ir IR<Hpulang>,
    affinities: OpMap<Affinity>,
    priorities: OpMap<u8>,
    mem_buffer: Vec<OpId>,
    alu_buffer: Vec<OpId>,
    pbs_buffer: Vec<OpId>,
    should_flush: SmallSet<OpId>,
    last_pbs: Option<(OpId, Cycle)>,
}

impl<'ir> Scheduler<'ir> {
    pub fn init(ir: &IR<Hpulang>, config: HpuConfig) -> Scheduler {
        use hpuc_langs::hpulang::Operations::*;
        let simulator = Simulator::from_simulatable(config.freq, Hpu::new(config));
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
            last_pbs: None,
        }
    }

    pub fn into_flusher(self) -> Flusher {
        Flusher(self.should_flush)
    }
}

pub struct Flusher(SmallSet<OpId>);

impl Flusher {
    pub fn apply_flushes(&self, ir: &mut IR<Hpulang>) {
        ir.mutate_ops_with_walker(self.0.iter().copied(), |operation| {
            use hpuc_langs::hpulang::Operations::*;
            *operation = match operation {
                Pbs { lut } | PbsF { lut } => PbsF {
                    lut: std::mem::take(lut),
                },
                Pbs2 { lut } | Pbs2F { lut } => Pbs2F {
                    lut: std::mem::take(lut),
                },
                Pbs4 { lut } | Pbs4F { lut } => Pbs4F {
                    lut: std::mem::take(lut),
                },
                Pbs8 { lut } | Pbs8F { lut } => Pbs8F {
                    lut: std::mem::take(lut),
                },
                _ => unreachable!(),
            };
        });
    }
}

impl<'ir> ForwardSimulator for Scheduler<'ir> {
    type Dialect = Hpulang;

    fn select(
        &mut self,
        ready: impl Iterator<Item = Ready>,
    ) -> impl Iterator<Item = Selected> + '_ {
        let hpu = self.simulator.simulatable();

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
        if let Some(val) = self.alu_buffer.first()
            && hpu.pe_alu.available()
        {
            output.push(Selected(*val));
        }
        // PEM Scheduling
        if let Some(val) = self.mem_buffer.first()
            && hpu.pe_mem.available()
        {
            output.push(Selected(*val));
        }
        // PEP Scheduling
        if let Some(val) = self.pbs_buffer.first()
            && hpu.pe_pbs.available()
        {
            output.push(Selected(*val));

            // Flush policy.
            if hpu.pe_pbs.memory().n_waiting() >= hpu.config.pbs_min_batch_size - 1 {
                self.should_flush.insert(*val);
            }
            if let (Policy::Timeout(timeout), Some((last_pbs_opid, last_pbs_cycle))) =
                (hpu.config.pbs_policy, self.last_pbs)
            {
                let span_since_last = self.simulator.now() - last_pbs_cycle;
                if span_since_last > timeout && !self.should_flush.contains(&last_pbs_opid) {
                    // Timeout was reached, which means that last_pbs should have been a flush...
                    self.should_flush.insert(last_pbs_opid);
                }
            }
            self.last_pbs = Some((*val, self.simulator.now()));
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
        let retired_opid = self
            .simulator
            .simulatable()
            .retirement
            .last_retired()
            .unwrap()
            .id;
        std::iter::once(Retired(OpId::from_usize(retired_opid.0)))
    }
}

fn opref_to_dop<'a>(opref: OpRef<'a, Hpulang>, force_flush: bool) -> Option<DOp> {
    use hpuc_langs::hpulang::Operations::*;
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
            dst: Argument::ct_var(to.dst_pos, to.block_pos),
            src: Argument::ct_reg(opref.get_arg_valids()[0]),
        }),
        SrcLd { from } => Some(RawDOp::LD {
            dst: Argument::ct_reg(opref.get_return_valids()[0]),
            src: Argument::ct_var(from.src_pos, from.block_pos),
        }),
        Pbs { .. } if !force_flush => Some(RawDOp::PBS {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg(opref.get_return_valids()[0]),
        }),
        Pbs2 { .. } if !force_flush => Some(RawDOp::PBS_ML2 {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg2(opref.get_return_valids()[0]),
        }),
        Pbs4 { .. } if !force_flush => Some(RawDOp::PBS_ML4 {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg4(opref.get_return_valids()[0]),
        }),
        Pbs8 { .. } if !force_flush => Some(RawDOp::PBS_ML8 {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg8(opref.get_return_valids()[0]),
        }),
        PbsF { .. } | Pbs { .. } if force_flush => Some(RawDOp::PBS_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg(opref.get_return_valids()[0]),
        }),
        Pbs2F { .. } | Pbs2 { .. } if force_flush => Some(RawDOp::PBS_ML2_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg2(opref.get_return_valids()[0]),
        }),
        Pbs4F { .. } | Pbs4 { .. } if force_flush => Some(RawDOp::PBS_ML4_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg4(opref.get_return_valids()[0]),
        }),
        Pbs8F { .. } | Pbs8 { .. } if force_flush => Some(RawDOp::PBS_ML8_F {
            dst: Argument::ct_reg(opref.get_arg_valids()[0]),
            src: Argument::ct_reg8(opref.get_return_valids()[0]),
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
    use hpuc_ir::{
        scheduling::forward::ForwardScheduler, translation::Translator, traversal::OpWalkerVerifier, IR,
    };
    use hpuc_langs::{hpulang::Hpulang, ioplang::Ioplang};
    use hpuc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{
        test::{get_add_ir, get_cmp_ir, get_sub_ir},
        translation::IoplangToHpulang,
    };

    use super::Scheduler;

    fn pipeline(ir: &IR<Ioplang>) -> IR<Hpulang> {
        let mut ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        let mut scheduler = Scheduler::init(&ir, config);
        let schedule = scheduler.schedule(&ir);
        assert_eq!(ir.n_ops() as usize, schedule.len());
        assert!(schedule.get_walker().is_topo_sorted(&ir));
        let flusher = scheduler.into_flusher();
        flusher.apply_flushes(&mut ir);
        ir
    }

    #[test]
    fn test_schedule_add_ir() {
        let ir = pipeline(&get_add_ir(16, 2, 2));
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<1.0_tsrc>();
            %8 : CtRegister = src_ld<1.1_tsrc>();
            %9 : CtRegister = src_ld<1.2_tsrc>();
            %10 : CtRegister = src_ld<1.3_tsrc>();
            %11 : CtRegister = src_ld<1.4_tsrc>();
            %12 : CtRegister = src_ld<1.5_tsrc>();
            %13 : CtRegister = src_ld<1.6_tsrc>();
            %14 : CtRegister = add_ct(%0, %7);
            %15 : CtRegister = add_ct(%1, %8);
            %16 : CtRegister = add_ct(%2, %9);
            %17 : CtRegister = add_ct(%3, %10);
            %18 : CtRegister = add_ct(%4, %11);
            %19 : CtRegister = add_ct(%5, %12);
            %20 : CtRegister = add_ct(%6, %13);
            %21 : CtRegister, %22 : CtRegister = pbs_2<Lut@0>(%14);
            %23 : CtRegister = pbs<Lut@0>(%15);
            %24 : CtRegister = pbs<Lut@0>(%16);
            %25 : CtRegister = pbs<Lut@0>(%17);
            %26 : CtRegister = pbs<Lut@0>(%18);
            %27 : CtRegister = pbs<Lut@0>(%19);
            %28 : CtRegister = pbs_f<Lut@0>(%20);
            %29 : CtRegister = add_ct(%23, %22);
            %30 : CtRegister = add_ct(%27, %26);
            %31 : CtRegister = add_ct(%15, %22);
            dst_st<0.0_tdst>(%21);
            %32 : CtRegister = add_ct(%24, %29);
            %33 : CtRegister = add_ct(%28, %30);
            %34 : CtRegister = pbs_f<Lut@0>(%29);
            dst_st<0.1_tdst>(%31);
            %35 : CtRegister = add_ct(%25, %32);
            %36 : CtRegister = pbs_f<Lut@0>(%32);
            %37 : CtRegister = add_ct(%16, %34);
            %38 : CtRegister = pbs_f<Lut@0>(%35);
            %39 : CtRegister = add_ct(%17, %36);
            dst_st<0.2_tdst>(%37);
            %40 : CtRegister = add_ct(%26, %38);
            %41 : CtRegister = add_ct(%30, %38);
            %42 : CtRegister = add_ct(%33, %38);
            dst_st<0.3_tdst>(%39);
            %43 : CtRegister = pbs<Lut@0>(%40);
            %44 : CtRegister = pbs<Lut@0>(%41);
            %45 : CtRegister = pbs<Lut@0>(%42);
            %46 : CtRegister = add_ct(%18, %43);
            %47 : CtRegister = add_ct(%19, %44);
            %48 : CtRegister = add_ct(%20, %45);
            dst_st<0.4_tdst>(%46);
            dst_st<0.5_tdst>(%47);
            dst_st<0.6_tdst>(%48);
            ",
        );
    }

    #[test]
    fn test_schedule_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<0.7_tsrc>();
            %8 : CtRegister = src_ld<1.0_tsrc>();
            %9 : CtRegister = src_ld<1.1_tsrc>();
            %10 : CtRegister = src_ld<1.2_tsrc>();
            %11 : CtRegister = src_ld<1.3_tsrc>();
            %12 : CtRegister = src_ld<1.4_tsrc>();
            %13 : CtRegister = src_ld<1.5_tsrc>();
            %14 : CtRegister = src_ld<1.6_tsrc>();
            %15 : CtRegister = src_ld<1.7_tsrc>();
            %16 : CtRegister = mac<4_imm>(%1, %0);
            %17 : CtRegister = mac<4_imm>(%3, %2);
            %18 : CtRegister = mac<4_imm>(%5, %4);
            %19 : CtRegister = mac<4_imm>(%7, %6);
            %20 : CtRegister = mac<4_imm>(%9, %8);
            %21 : CtRegister = mac<4_imm>(%11, %10);
            %22 : CtRegister = mac<4_imm>(%13, %12);
            %23 : CtRegister = mac<4_imm>(%15, %14);
            %24 : CtRegister = sub_ct(%16, %20);
            %25 : CtRegister = sub_ct(%17, %21);
            %26 : CtRegister = sub_ct(%18, %22);
            %27 : CtRegister = sub_ct(%19, %23);
            %28 : CtRegister = pbs<Lut@0>(%24);
            %29 : CtRegister = pbs<Lut@0>(%25);
            %30 : CtRegister = pbs<Lut@0>(%26);
            %31 : CtRegister = pbs_f<Lut@0>(%27);
            %32 : CtRegister = mac<4_imm>(%29, %28);
            %33 : CtRegister = mac<4_imm>(%31, %30);
            %34 : CtRegister = pbs<Lut@0>(%32);
            %35 : CtRegister = pbs_f<Lut@0>(%33);
            %36 : CtRegister = mac<4_imm>(%35, %34);
            %37 : CtRegister = pbs<Lut@0>(%36);
            dst_st<0.0_tdst>(%37);
            ",
        );
    }

    #[test]
    fn test_schedule_sub_ir() {
        let ir = pipeline(&get_sub_ir(16, 2, 2));
        ir.check_ir(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<1.0_tsrc>();
            %8 : CtRegister = src_ld<1.1_tsrc>();
            %9 : CtRegister = src_ld<1.2_tsrc>();
            %10 : CtRegister = src_ld<1.3_tsrc>();
            %11 : CtRegister = src_ld<1.4_tsrc>();
            %12 : CtRegister = src_ld<1.5_tsrc>();
            %13 : CtRegister = src_ld<1.6_tsrc>();
            %14 : CtRegister = cst_sub<3_imm>(%7);
            %15 : CtRegister = cst_sub<3_imm>(%8);
            %16 : CtRegister = cst_sub<3_imm>(%9);
            %17 : CtRegister = cst_sub<3_imm>(%10);
            %18 : CtRegister = cst_sub<3_imm>(%11);
            %19 : CtRegister = cst_sub<3_imm>(%12);
            %20 : CtRegister = cst_sub<3_imm>(%13);
            %21 : CtRegister = add_ct(%0, %14);
            %22 : CtRegister = add_ct(%1, %15);
            %23 : CtRegister = add_ct(%2, %16);
            %24 : CtRegister = add_ct(%3, %17);
            %25 : CtRegister = add_ct(%4, %18);
            %26 : CtRegister = add_ct(%5, %19);
            %27 : CtRegister = add_ct(%6, %20);
            %28 : CtRegister, %29 : CtRegister = pbs_2<Lut@0>(%21);
            %30 : CtRegister = pbs<Lut@0>(%22);
            %31 : CtRegister = pbs<Lut@0>(%23);
            %32 : CtRegister = pbs<Lut@0>(%24);
            %33 : CtRegister = pbs<Lut@0>(%25);
            %34 : CtRegister = pbs<Lut@0>(%26);
            %35 : CtRegister = pbs<Lut@0>(%27);
            %36 : CtRegister = add_ct(%30, %29);
            %37 : CtRegister = add_ct(%34, %33);
            %38 : CtRegister = add_ct(%22, %29);
            %39 : CtRegister = pbs<Lut@0>(%28);
            %40 : CtRegister = add_ct(%31, %36);
            %41 : CtRegister = add_ct(%35, %37);
            %42 : CtRegister = pbs_f<Lut@0>(%36);
            %43 : CtRegister = pbs<Lut@0>(%38);
            dst_st<0.0_tdst>(%39);
            %44 : CtRegister = add_ct(%32, %40);
            %45 : CtRegister = pbs<Lut@0>(%40);
            %46 : CtRegister = add_ct(%23, %42);
            dst_st<0.1_tdst>(%43);
            %47 : CtRegister = pbs<Lut@0>(%44);
            %48 : CtRegister = add_ct(%24, %45);
            %49 : CtRegister = pbs_f<Lut@0>(%46);
            %50 : CtRegister = add_ct(%33, %47);
            %51 : CtRegister = add_ct(%37, %47);
            %52 : CtRegister = add_ct(%41, %47);
            %53 : CtRegister = pbs<Lut@0>(%48);
            dst_st<0.2_tdst>(%49);
            %54 : CtRegister = pbs<Lut@0>(%50);
            %55 : CtRegister = pbs<Lut@0>(%51);
            %56 : CtRegister = pbs_f<Lut@0>(%52);
            dst_st<0.3_tdst>(%53);
            %57 : CtRegister = add_ct(%25, %54);
            %58 : CtRegister = add_ct(%26, %55);
            %59 : CtRegister = add_ct(%27, %56);
            %60 : CtRegister = pbs<Lut@0>(%57);
            %61 : CtRegister = pbs<Lut@0>(%58);
            %62 : CtRegister = pbs<Lut@0>(%59);
            dst_st<0.4_tdst>(%60);
            dst_st<0.5_tdst>(%61);
            dst_st<0.6_tdst>(%62);            ",
        );
    }
}
