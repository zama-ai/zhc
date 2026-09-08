use zhc_config::hpu::HpuConfig;
use zhc_ir::IR;
use zhc_langs::hpulang::HpuLang;
use zhc_sim::Simulator;

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
pub fn schedule<'a>(ir: &'a IR<HpuLang>, config: &HpuConfig, policy: SchedPolicy) -> IR<HpuLang> {
    let ann_ir = analyze(ir);
    let mut sim = Simulator::from_simulatable(
        MHz(400),
        LightHpu::new(&ann_ir, config, policy),
        zhc_sim::TracingLevel::None,
    );
    sim.play();
    let schedule = sim.into_simulatable().schedule;
    let oup = batch(ir, schedule.into());
    oup
}
