use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::{IR, OpMap};
use zhc_langs::hpulang::{HpuLang, HpuLocality};
use zhc_sim::{SimulationState, Simulator};

mod affinity;
mod analyze;
mod batcher;
mod sim;

pub use affinity::*;
pub use analyze::*;
pub use batcher::*;
pub use sim::*;
use zhc_utils::units::MHz;

use crate::SchedPolicy;

#[allow(unused)]
pub fn schedule<'a>(
    ir: &'a IR<HpuLang>,
    localities: &OpMap<HpuLocality>,
    config: &MultiHpuConfig,
    policy: SchedPolicy,
) -> Vec<IR<HpuLang>> {
    let ann_ir = analyze(ir, localities);
    let mut sim = Simulator::from_simulatable(
        MHz(400),
        LightMultiHpu::new(&ann_ir, config, policy),
        zhc_sim::TracingLevel::None,
    );
    let state = sim.play_until_event_or_over(MultiHpuEvents::ScheduleComplete);
    let simulatable = sim.into_simulatable();
    if matches!(state, SimulationState::SimulationOver) {
        panic!("{}", simulatable.stall_report());
    }
    let transfer_map = simulatable.transfers_map;
    let hpus = simulatable.hpus;
    hpus
        .into_iter()
        .map(|hpu| batch(ir, hpu.id, hpu.schedule.into(), &transfer_map))
        .collect()
}
