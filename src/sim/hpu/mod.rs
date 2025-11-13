use super::{Dispatcher, Simulatable, Tracer, Trigger};

mod dops;
mod events;
mod isc;
mod latencies;
mod pe_alu;
mod pe_mem;
mod pe_pbs;
mod retirement;
mod config;
#[cfg(test)]
mod test;

pub use dops::*;
pub use events::*;
pub use isc::*;
pub use latencies::*;
use pe_alu::PeAlu;
use pe_mem::PeMem;
pub use pe_pbs::*;
pub use retirement::*;
pub use config::*;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Hpu {
    scheduler: InstructionScheduler,
    pe_mem: PeMem,
    pe_pbs: PePbs,
    pe_alu: PeAlu,
    retirement: Retirement,
}

impl Simulatable for Hpu {
    type Event = Events;

    fn handle(&mut self, dispatcher: &mut Dispatcher<Self::Event>, trigger: Trigger<Self::Event>) {
        self.scheduler.handle(dispatcher, trigger.clone());
        self.pe_mem.handle(dispatcher, trigger.clone());
        self.pe_pbs.handle(dispatcher, trigger.clone());
        self.pe_alu.handle(dispatcher, trigger.clone());
        self.retirement.handle(dispatcher, trigger.clone());
    }

    fn power_up(&self, dispatcher: &mut Dispatcher<Self::Event>) {
        self.scheduler.power_up(dispatcher);
        self.pe_mem.power_up(dispatcher);
        self.pe_pbs.power_up(dispatcher);
        self.pe_alu.power_up(dispatcher);
        self.retirement.power_up(dispatcher);
    }

    fn report<'t>(&self, tracer: &mut Tracer<Self::Event>) {
        tracer.add_simulatable(&self.scheduler);
        tracer.add_simulatable(&self.pe_mem);
        tracer.add_simulatable(&self.pe_pbs);
        tracer.add_simulatable(&self.pe_alu);
        tracer.add_simulatable(&self.retirement);
    }
}

impl Hpu {
    pub fn new(config: HpuConfig) -> Self {
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
                config.pbs_batch_size as BatchSize,
                config.pbs_policy,
                ConstantLatency::new(config.pbs_load_unload_latency),
                FlatLinLatency::new(
                    config.pbs_processing_latency_a,
                    config.pbs_processing_latency_b,
                    config.pbs_processing_latency_m,
                ),
            ),
            retirement: Retirement::default(),
        }
    }
}
