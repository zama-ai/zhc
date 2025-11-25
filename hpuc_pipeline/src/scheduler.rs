use hpuc_ir::{
    scheduling::{forward::ForwardSimulator, Ready, Retired, Selected}, OpId, OpMap, OpRef, IR
};
use hpuc_langs::hpulang::Hpulang;
use hpuc_sim::{
    Cycle, Simulator,
    hpu::{Affinity, Argument, DOp, DOpId, Events, Hpu, HpuConfig, Policy, RawDOp},
};
use hpuc_utils::{SmallVec, StoreIndex};

pub struct Scheduler<'ir> {
    simulator: Simulator<Hpu>,
    ir: &'ir IR<Hpulang>,
    affinities: OpMap<Affinity>,
    priorities: OpMap<u8>,
    mem_buffer: Vec<OpId>,
    alu_buffer: Vec<OpId>,
    pbs_buffer: Vec<OpId>,
    should_flush: Vec<OpId>,
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
            HeapSt => Affinity::Mem,
            HeapLd => Affinity::Mem,
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
            should_flush: Vec::new(),
            last_pbs: None,
        }
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
                self.should_flush.push(*val);
            }
            if let (Policy::Timeout(timeout), Some((last_pbs_opid, last_pbs_cycle))) =
                (hpu.config.pbs_policy, self.last_pbs)
            {
                let span_since_last = self.simulator.now() - last_pbs_cycle;
                if span_since_last > timeout && !self.should_flush.contains(&last_pbs_opid) {
                    // Timeout was reached, which means that last_pbs should have been a flush...
                    self.should_flush.push(last_pbs_opid);
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
                    .filter_map(|opid| {
                        opref_to_dop(
                            self.ir.get_op(opid.0),
                            self.should_flush
                                .last()
                                .map(|a| *a == opid.0)
                                .unwrap_or(false),
                        )
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
            dst: Argument::reg(opref.get_return_valids()[0]),
            src1: Argument::reg(opref.get_arg_valids()[0]),
            src2: Argument::reg(opref.get_arg_valids()[1]),
        }),
        SubCt => Some(RawDOp::SUB {
            dst: Argument::reg(opref.get_return_valids()[0]),
            src1: Argument::reg(opref.get_arg_valids()[0]),
            src2: Argument::reg(opref.get_arg_valids()[1]),
        }),
        Mac { .. } => Some(RawDOp::MAC {
            dst: Argument::reg(opref.get_return_valids()[0]),
            src1: Argument::reg(opref.get_arg_valids()[0]),
            src2: Argument::reg(opref.get_arg_valids()[1]),
            cst: Argument::IMM_ZERO,
        }),
        AddPt | AddCst { .. } => Some(RawDOp::ADDS {
            dst: Argument::reg(opref.get_return_valids()[0]),
            src: Argument::reg(opref.get_arg_valids()[0]),
            cst: Argument::IMM_ZERO,
        }),
        SubPt | SubCst { .. } => Some(RawDOp::SUBS {
            dst: Argument::reg(opref.get_return_valids()[0]),
            src: Argument::reg(opref.get_arg_valids()[0]),
            cst: Argument::IMM_ZERO,
        }),
        PtSub | CstSub { .. } => Some(RawDOp::SSUB {
            dst: Argument::reg(opref.get_return_valids()[0]),
            src: Argument::reg(opref.get_arg_valids()[0]),
            cst: Argument::IMM_ZERO,
        }),
        MulPt | MulCst { .. } => Some(RawDOp::MULS {
            dst: Argument::reg(opref.get_return_valids()[0]),
            src: Argument::reg(opref.get_arg_valids()[0]),
            cst: Argument::IMM_ZERO,
        }),
        ImmLd { .. } => None,
        DstSt { .. } | HeapSt => Some(RawDOp::ST {
            dst: Argument::MEM_ZERO,
            src: Argument::reg(opref.get_arg_valids()[0]),
        }),
        SrcLd { .. } | HeapLd => Some(RawDOp::LD {
            dst: Argument::reg(opref.get_return_valids()[0]),
            src: Argument::MEM_ZERO,
        }),
        Pbs { .. } if !force_flush => Some(RawDOp::PBS {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg(opref.get_return_valids()[0]),
        }),
        Pbs2 { .. } if !force_flush => Some(RawDOp::PBS_ML2 {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg2(opref.get_return_valids()[0]),
        }),
        Pbs4 { .. } if !force_flush => Some(RawDOp::PBS_ML4 {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg4(opref.get_return_valids()[0]),
        }),
        Pbs8 { .. } if !force_flush => Some(RawDOp::PBS_ML8 {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg8(opref.get_return_valids()[0]),
        }),
        PbsF { .. } | Pbs { .. } if force_flush => Some(RawDOp::PBS_F {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg(opref.get_return_valids()[0]),
        }),
        Pbs2F { .. } | Pbs2 { .. } if force_flush => Some(RawDOp::PBS_ML2_F {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg2(opref.get_return_valids()[0]),
        }),
        Pbs4F { .. } | Pbs4 { .. } if force_flush => Some(RawDOp::PBS_ML4_F {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg4(opref.get_return_valids()[0]),
        }),
        Pbs8F { .. } | Pbs8 { .. } if force_flush => Some(RawDOp::PBS_ML8_F {
            dst: Argument::reg(opref.get_arg_valids()[0]),
            src: Argument::reg8(opref.get_return_valids()[0]),
        }),
        _ => unreachable!(),
    };

    raw.map(|raw| DOp {
        raw,
        id: DOpId(opref.get_id().into()),
    })
}

#[cfg(test)]
mod test {
    use hpuc_ir::{scheduling::forward::ForwardScheduler, translation::Translator, traversal::OpWalkerVerifier};
    use hpuc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{
        test::{get_add_ir, get_cmp_ir, get_sub_ir},
        translation::IoplangToHpulang,
    };

    use super::Scheduler;

    #[test]
    fn test_schedule_add_ir() {
        let ir = get_add_ir(16, 2, 2);
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        let mut scheduler = Scheduler::init(&ir, config);
        let schedule = scheduler.schedule(&ir);
        assert_eq!(ir.n_ops() as usize, schedule.len());
        assert!(schedule.get_walker().is_topo_sorted(&ir));
    }

    #[test]
    fn test_schedule_cmp_ir() {
        let ir = get_cmp_ir(16, 2, 2);
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        let mut scheduler = Scheduler::init(&ir, config);
        let schedule = scheduler.schedule(&ir);
        assert_eq!(ir.n_ops() as usize, schedule.len());
        assert!(schedule.get_walker().is_topo_sorted(&ir));
    }

    #[test]
    fn test_schedule_sub_ir() {
        let ir = get_sub_ir(16, 2, 2);
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        let mut scheduler = Scheduler::init(&ir, config);
        let schedule = scheduler.schedule(&ir);
        assert_eq!(ir.n_ops() as usize, schedule.len());
        assert!(schedule.get_walker().is_topo_sorted(&ir));
    }
}
