use crate::TracingLevel;
use serde::Serialize;
use zhc_langs::hpulang::HpuId;
use zhc_utils::tracing::Microseconds;
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
mod ucore;
mod statistics;

pub use config::*;
pub use dops::*;
pub use events::*;
pub use isc::*;
pub use latencies::*;
pub use pe_alu::*;
pub use pe_ctl::*;
pub use pe_mem::*;
pub use pe_pbs::*;
pub use ucore::*;
pub use statistics::*;

pub const MHDMA_LATENCY: Microseconds = 8.;
pub const NOTIFY_LATENCY: Microseconds = 1.;

/// HPU simulator containing all processing elements and scheduling logic.
#[derive(Debug, Serialize)]
pub struct Hpu {
    pub scheduler: InstructionScheduler,
    pub pe_mem: PeMem,
    pub pe_pbs: PePbs,
    pub pe_alu: PeAlu,
    pub pe_ctl: PeCtl,
    pub ucore: UCore,
    pub statistics: Statistics,
    pub config: HpuConfig,
    pub id: HpuId
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
        self.ucore.handle(dispatcher, trigger.clone());
        self.statistics.handle(dispatcher, trigger.clone());
    }

    fn power_up(&mut self, dispatcher: &mut impl Dispatch<Event = Events>) {
        self.scheduler.power_up(dispatcher);
        self.pe_mem.power_up(dispatcher);
        self.pe_pbs.power_up(dispatcher);
        self.pe_alu.power_up(dispatcher);
        self.pe_ctl.power_up(dispatcher);
        self.ucore.power_up(dispatcher);
        self.statistics.power_up(dispatcher);
    }

    fn report<'t>(&self, at: Cycle, tracer: &mut Tracer, tracing_level: TracingLevel) {
        tracer.add_state(tracing_level, at, Some(self.id.0.into()), self.scheduler.name(), &self.scheduler);
        tracer.add_state(tracing_level, at, Some(self.id.0.into()), self.pe_mem.name(), &self.pe_mem);
        tracer.add_state(tracing_level, at, Some(self.id.0.into()), self.pe_pbs.name(), &self.pe_pbs);
        tracer.add_state(tracing_level, at, Some(self.id.0.into()), self.pe_alu.name(), &self.pe_alu);
        tracer.add_state(tracing_level, at, Some(self.id.0.into()), self.pe_ctl.name(), &self.pe_ctl);
        tracer.add_state(tracing_level, at, Some(self.id.0.into()), self.ucore.name(), &self.ucore);
        tracer.add_state(tracing_level, at, Some(self.id.0.into()), self.statistics.name(), &self.statistics);

        // PE loading counters
        tracer.add_counter(
            tracing_level,
            at,
            Some(self.id.0.into()),
            "pe_alu_busy",
            self.pe_alu.busy() as u8 as f64,
        );
        tracer.add_counter(
            tracing_level,
            at,
            Some(self.id.0.into()),
            "pe_mem_busy",
            self.pe_mem.busy() as u8 as f64,
        );
        tracer.add_counter(
            tracing_level,
            at,
            Some(self.id.0.into()),
            "pe_pbs_working",
            self.pe_pbs.memory().n_working() as f64,
        );
    }
}

impl Hpu {
    /// Creates a new HPU instance configured with the given `config` parameters.
    ///
    /// All processing elements are initialized with their respective capacities,
    /// latencies, and operational parameters as specified in the configuration.
    pub fn new(config: &HpuConfig, id: HpuId) -> Self {
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
                config.pbs_timeout,
                ConstantLatency::new(config.pbs_load_unload_latency),
                FlatLinLatency::new(
                    config.pbs_processing_latency_a,
                    config.pbs_processing_latency_b,
                    config.pbs_processing_latency_m,
                ),
            ),
            pe_ctl: PeCtl,
            ucore: UCore::new(ConstantLatency::new(config.freq.n_cycles(MHDMA_LATENCY))),
            statistics: Statistics::default(),
            config: config.clone(),
            id
        }
    }
}

#[cfg(test)]
pub mod test;
