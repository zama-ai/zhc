use crate::{
    Cycle, Dispatch, MapDispatch, Simulatable, Tracer, TracingLevel, Trigger,
    hpu::{Hpu, MultiHpuConfig, NOTIFY_LATENCY},
};
use serde::Serialize;

mod events;

pub use events::*;
use zhc_langs::hpulang::HpuId;

use super::hpu::Events as HpuEvents;

#[derive(Debug, Serialize)]
pub struct MultiHpu {
    hpus: Vec<Hpu>,
    done: u8,
    config: MultiHpuConfig,
}

impl MultiHpu {
    pub fn new(config: &MultiHpuConfig) -> MultiHpu {
        let hpus = (0..config.n_hpus).map(|i| Hpu::new(&config.hpu_config, HpuId(i))).collect();
        MultiHpu {
            hpus,
            config: config.to_owned(),
            done: 0,
        }
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
            Events::Hpu(_, HpuEvents::UCoreTransferOutReady(hid, tid)) => {
                dispatcher.dispatch_after(
                    Cycle(self.config.hpu_config.freq.n_cycles(NOTIFY_LATENCY)),
                    Events::Hpu(hid, HpuEvents::UCoreTransferInNotified(tid)),
                );
            }
            Events::Hpu(_, HpuEvents::UCoreStarved) => {
                self.done += 1;
                if self.done as usize == self.hpus.len() {
                    dispatcher.dispatch_now(Events::ProcessOver);
                }
            }
            Events::Hpu(hpu_id, hpu_event) => {
                self.hpus[hpu_id.0 as usize].handle(
                    &mut dispatcher.map(|e| Events::Hpu(hpu_id, e)),
                    Trigger {
                        at: trigger.at,
                        event: hpu_event,
                    },
                );
            }
            Events::PushDOps(streams) => {
                assert_eq!(streams.len(), self.hpus.len());
                self.hpus
                    .iter()
                    .zip(streams.into_iter())
                    .for_each(|(hpu, stream)| {
                        dispatcher
                            .dispatch_now(Events::Hpu(hpu.id, HpuEvents::UCorePushDOps(stream)))
                    });
            }
            _ => {}
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
fn test_multi() {
    use super::hpu::Events as HpuEvents;
    use crate::Simulator;
    let config = MultiHpuConfig::default();
    let mut sim =
        Simulator::from_simulatable(config.hpu_config.freq, MultiHpu::new(&config), TracingLevel::Events);
    let (stream, _leg_lst) = super::hpu::test::legacy::DIV();
    sim.dispatch(Events::Hpu(
        HpuId(0),
        HpuEvents::UCorePushDOps(stream.collect()),
    ));
    sim.play_until_event(Events::Hpu(HpuId(0), HpuEvents::UCoreStarved));
    sim.dump_trace("gfsdanfdsnafds.json");
}
