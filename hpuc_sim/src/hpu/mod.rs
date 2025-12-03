use super::{Cycle, Dispatch, Simulatable, Tracer, Trigger};

mod config;
mod dops;
mod events;
mod isc;
mod latencies;
mod pe_alu;
mod pe_ctl;
mod pe_mem;
mod pe_pbs;
mod retirement;
#[cfg(test)]
mod test;
pub use config::*;
pub use dops::*;
pub use events::*;
pub use isc::*;
pub use latencies::*;
use pe_alu::PeAlu;
pub use pe_ctl::*;
use pe_mem::PeMem;
pub use pe_pbs::*;
pub use retirement::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Hpu {
    pub scheduler: InstructionScheduler,
    pub pe_mem: PeMem,
    pub pe_pbs: PePbs,
    pub pe_alu: PeAlu,
    pub pe_ctl: PeCtl,
    pub retirement: Retirement,
    pub config: HpuConfig,
}

impl Simulatable for Hpu {
    type Event = Events;

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        self.scheduler.handle(dispatcher, trigger.clone());
        self.pe_mem.handle(dispatcher, trigger.clone());
        self.pe_pbs.handle(dispatcher, trigger.clone());
        self.pe_alu.handle(dispatcher, trigger.clone());
        self.pe_ctl.handle(dispatcher, trigger.clone());
        self.retirement.handle(dispatcher, trigger.clone());
    }

    fn power_up(&self, dispatcher: &mut impl Dispatch<Event = Events>) {
        self.scheduler.power_up(dispatcher);
        self.pe_mem.power_up(dispatcher);
        self.pe_pbs.power_up(dispatcher);
        self.pe_alu.power_up(dispatcher);
        self.pe_ctl.power_up(dispatcher);
        self.retirement.power_up(dispatcher);
    }

    fn report<'t>(&self, at: Cycle, tracer: &mut Tracer<Events>) {
        tracer.add_simulatable(at, &self.scheduler);
        tracer.add_simulatable(at, &self.pe_mem);
        tracer.add_simulatable(at, &self.pe_pbs);
        tracer.add_simulatable(at, &self.pe_alu);
        tracer.add_simulatable(at, &self.pe_ctl);
        tracer.add_simulatable(at, &self.retirement);
    }
}

impl Hpu {
    pub fn new(config: &HpuConfig) -> Self {
        Hpu {
            scheduler: InstructionScheduler::new(config.isc_query_period, config.isc_depth),
            pe_mem: PeMem::new(
                config.mem_fifo_capacity,
                ConstantLatency::new(config.mem_read_latency),
                ConstantLatency::new(config.mem_write_latency),
            ),
            pe_alu: PeAlu::new(
                config.alu_fifo_capacity,
                ConstantLatency::new(config.alu_read_latency),
                ConstantLatency::new(config.alu_write_latency),
            ),
            pe_pbs: PePbs::new(
                config.pbs_fifo_capacity,
                config.pbs_memory_capacity,
                config.pbs_max_batch_size as BatchSize,
                config.pbs_policy,
                ConstantLatency::new(config.pbs_load_unload_latency),
                FlatLinLatency::new(
                    config.pbs_processing_latency_a,
                    config.pbs_processing_latency_b,
                    config.pbs_processing_latency_m,
                ),
            ),
            pe_ctl: PeCtl,
            retirement: Retirement::default(),
            config: config.clone(),
        }
    }
}
