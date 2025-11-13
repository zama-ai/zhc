use super::*;
use crate::sim::Simulator;

mod legacy;


#[test]
fn test_hpu_simulation() {
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
    let mut sim = Simulator::from_simulatable(config.freq, Hpu::new(config));
    let stream = legacy::adds();
    sim.dispatch(Events::IscPushDOps(stream.collect()));
    sim.play();
    sim.dump_trace("test_profile.json");
}
