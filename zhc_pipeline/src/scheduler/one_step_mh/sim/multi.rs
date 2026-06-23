use std::fmt::Display;

use serde::Serialize;
use zhc_ir::AnnIR;
use zhc_langs::hpulang::{HpuId, HpuLang};
use zhc_sim::{Cycle, Dispatch, Event, MapDispatch, Simulatable, Tracer, TracingLevel, Trigger, hpu::MultiHpuConfig};

use crate::scheduler::{SchedPolicy, one_step_mh::{HpuEvents, LightHpu, Stats}};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiHpuEvents {
    Hpu(HpuId, HpuEvents)
}

impl Display for MultiHpuEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiHpuEvents::Hpu(id, event) => write!(f, "Hpu({id}, {event})"),
        }
    }
}

impl Event for MultiHpuEvents {}


#[derive(Serialize)]
pub struct LightMultiHpu<'a, 'b> {
    pub hpus: Vec<LightHpu<'a, 'b>>,
}

impl<'a, 'b> LightMultiHpu<'a, 'b> {
    pub fn new(
        ir: &'b AnnIR<'a, HpuLang, Stats, ()>,
        config: &MultiHpuConfig,
        policy: SchedPolicy,
    ) -> Self {
        let hpus = (0..config.n_hpus).map(|i| LightHpu::new(ir, &config.hpu_config, policy, HpuId(i))).collect();
        LightMultiHpu { hpus }
    }
}

impl<'a, 'b> Simulatable for LightMultiHpu<'a, 'b> {
    type Event = MultiHpuEvents;

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match trigger.event {
            MultiHpuEvents::Hpu(_, HpuEvents::TransferOut(id, opid)) => {
                dispatcher.dispatch_now(MultiHpuEvents::Hpu(id, HpuEvents::TransferIn(opid)));
            }
            MultiHpuEvents::Hpu(hpu_id, hpu_event) => {
                self.hpus[hpu_id.0 as usize].handle(
                    &mut dispatcher.map(|e| MultiHpuEvents::Hpu(hpu_id, e)),
                    Trigger {
                        at: trigger.at,
                        event: hpu_event,
                    },
                );
            }
        }
    }

    fn power_up(&mut self, dispatcher: &mut impl Dispatch<Event = MultiHpuEvents>) {
        for hpu in self.hpus.iter_mut() {
            let id = hpu.id;
            hpu.power_up(&mut dispatcher.map(|e| MultiHpuEvents::Hpu(id, e)));
        }
    }

    fn report<'t>(&self, at: Cycle, tracer: &mut Tracer, tracing_level: TracingLevel) {
        for hpu in self.hpus.iter() {
            hpu.report(at, tracer, tracing_level);
        }
    }
}
