use crate::{
    gir::{scheduling::{ForwardListScheduler, Ready}, OpId, IR},
    ioplang::Ioplang,
    sim::{
        hpu::{Affinity, Hpu, HpuConfig}, Simulator
    }, utils::Store,
};

pub struct Scheduler<'ir> {
    simulator: Simulator<Hpu>,
    ir: &'ir IR<Ioplang>,
    affinities: Store<OpId, Option<Affinity>>,
    mem_buffer: Vec<OpId>,
    alu_buffer: Vec<OpId>,
    pbs_buffer: Vec<OpId>,
}

impl<'ir> ForwardListScheduler for Scheduler<'ir> {
    type Dialect = Ioplang;

    type Config = HpuConfig;

    fn init<'a>(ir: &'a crate::gir::IR<Self::Dialect>, config: Self::Config) -> Scheduler<'a> {
        let simulator = Simulator::from_simulatable(config.freq, Hpu::new(config));
        let affinities = ir.map_ops_to_store(|op| match op.get_operation() {
            crate::ioplang::Operations::Input { .. } => Affinity::Ctl,
            crate::ioplang::Operations::Output { .. } => Affinity::Ctl,
            crate::ioplang::Operations::Variable { .. } => Affinity::Ctl,
            crate::ioplang::Operations::Constant { .. } => Affinity::Ctl,
            crate::ioplang::Operations::GenerateLut { .. } => Affinity::Ctl,
            crate::ioplang::Operations::AddCt => Affinity::Alu,
            crate::ioplang::Operations::SubCt => Affinity::Alu,
            crate::ioplang::Operations::Mac => Affinity::Alu,
            crate::ioplang::Operations::AddPt => Affinity::Alu,
            crate::ioplang::Operations::SubPt => Affinity::Alu,
            crate::ioplang::Operations::PtSub => Affinity::Alu,
            crate::ioplang::Operations::MulPt => Affinity::Alu,
            crate::ioplang::Operations::ExtractCtBlock => Affinity::Mem,
            crate::ioplang::Operations::ExtractPtBlock => Affinity::Mem,
            crate::ioplang::Operations::StoreCtBlock => Affinity::Mem,
            crate::ioplang::Operations::Pbs => Affinity::Pbs,
            crate::ioplang::Operations::Pbs2 => Affinity::Pbs,
            crate::ioplang::Operations::Pbs4 => Affinity::Pbs,
            crate::ioplang::Operations::Pbs8 => Affinity::Pbs,
        });
        let priorities = ir.map_ops_to_store(|op| op.get_depth());
        Scheduler {
            simulator,
            affinities,
            ir,
            mem_buffer: Vec::new(),
            alu_buffer: Vec::new(),
            pbs_buffer: Vec::new(),
        }
    }

    fn select(
        &mut self,
        ready: impl Iterator<Item = crate::gir::scheduling::Ready>,
    ) -> impl Iterator<Item = crate::gir::scheduling::Selected> + '_ {
        let mut output = Vec::new();
        for Ready(opid) in ready {
            match self.affinities[opid].unwrap() {
                Affinity::Alu => self.alu_buffer.push(opid),
                Affinity::Mem => self.mem_buffer.push(opid),
                Affinity::Pbs => self.pbs_buffer.push(opid),
                Affinity::Ctl => output.push(opid),
            }
        }
        if !self.simulator.simulatable().pe_alu.busy() {
            self.alu_buffer.sort_unstable_by_key(|a| self.priorities[a].unwrap());
            if let Some(val) = self.alu_buffer.first() {
                output.push(*val);
            }
        }
        if !self.simulator.simulatable().pe_mem.busy() {
            self.alu_buffer.sort_unstable_by_key(|a| self.priorities[a].unwrap());
            if let Some(val) = self.alu_buffer.first() {
                output.push(*val);
            }
        }

        self.mem_buffer.sort_unstable_by_key(|a| self.priorities[a].unwrap());
        self.pbs_buffer.sort_unstable_by_key(|a| self.priorities[a].unwrap());


    }

    fn advance(&mut self) -> impl Iterator<Item = crate::gir::scheduling::Retired> {
        todo!()
    }
}
