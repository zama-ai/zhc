use serde::Serialize;
use crate::{Cycle, Dispatch, MapDispatch, Simulatable, Simulator, Tracer, TracingLevel, Trigger, hpu::{Hpu, HpuConfig, HpuId, PhysicalConfig}};

mod events;

pub use events::*;


#[derive(Debug, Serialize)]
pub struct MultiHpu {
    hpus: Vec<Hpu>
}

impl MultiHpu {
    pub fn new(config: &HpuConfig, n: usize) -> MultiHpu {
        let hpus = (0..n).map(|i| Hpu::new(config, HpuId(i))).collect();
        MultiHpu { hpus }
    }
}

impl Simulatable for MultiHpu {
    type Event = Events;

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match trigger.event {
            Events::Hpu(hpu_id, hpu_event) => {
                self.hpus[hpu_id.0].handle(&mut dispatcher.map(|e| Events::Hpu(hpu_id, e)), Trigger { at: trigger.at, event: hpu_event });
            },
        }
    }

    fn power_up(&mut self, dispatcher: &mut impl Dispatch<Event = Events>) {
        for hpu in self.hpus.iter_mut() {
            let id = hpu.id;
            hpu.power_up(&mut dispatcher.map(|e| Events::Hpu(id, e)));
        }
    }

    fn report<'t>(&self, at: Cycle, tracer: &mut Tracer, tracing_level: TracingLevel) {
        for hpu in self.hpus.iter() {
            hpu.report(at, tracer, tracing_level);
        }
    }
}

#[cfg(test)]
#[test]
fn testmulti() {
    use super::hpu::Events as HpuEvents;
    let mut config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
    config.pbs_timeout = Cycle(100_000);
    let mut sim = Simulator::from_simulatable(config.freq, MultiHpu::new(&config, 2), TracingLevel::Events);
    let (stream, leg_lst) = super::hpu::test::legacy::DIV();
    sim.dispatch(Events::Hpu(HpuId(0), HpuEvents::IscPushDOps(stream.collect())));
    sim.play_until_event(Events::Hpu(HpuId(0), HpuEvents::IscProcessOver));
    sim.dump_trace("gfsdanfdsnafds.json");

}
