//! Simulation of a multi-board HPU system.
//!
//! [`MultiHpu`] composes a fixed set of [`Hpu`] boards into a single [`Simulatable`], and its
//! [`Events`] enum addresses each of them by [`HpuId`]: `Events::Hpu(id, e)` delivers the wrapped
//! HPU event `e` to board `id`, while `Events::PushDOps` and `Events::ProcessOver` mark
//! system-wide start and completion.
//!
//! Two board-level events are intercepted instead of forwarded, since a lone [`Hpu`] has no way
//! to reach its peers. A `UCoreTransferOutReady` is re-addressed to its destination board as a
//! `UCoreTransferInNotified` delayed by [`NOTIFY_LATENCY`], which closes the cross-board transfer
//! handshake; a `UCoreStarved` instead counts toward termination, and `Events::ProcessOver` is
//! emitted once every board has starved.
//!
//! Pushing a set of DOP streams whose length differs from the board count, or addressing an
//! out-of-range [`HpuId`], panics.

use crate::{
    Dispatch, MapDispatch, Simulatable, Tracer, TracingLevel, Trigger,
    hpu::{Hpu, NOTIFY_LATENCY},
};
use serde::Serialize;

mod events;

pub use events::*;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_langs::hpulang::HpuId;
use zhc_utils::units::Cycle;

use super::hpu::Events as HpuEvents;

/// Simulator for a system of [`Hpu`] boards sharing a common configuration.
///
/// Owns every board and acts as the router between them, forwarding addressed events to their
/// target board while mediating the inter-board transfer handshake and the detection of
/// system-wide completion.
#[derive(Debug, Serialize)]
pub struct MultiHpu {
    hpus: Vec<Hpu>,
    done: u8,
    config: MultiHpuConfig,
}

impl MultiHpu {
    /// Creates a system of `config.n_hpus` boards, each built from `config.hpu_config`.
    ///
    /// The `i`-th board is identified by `HpuId(i)`. Every board starts idle, awaiting an
    /// `Events::PushDOps` to receive its DOP stream.
    pub fn new(config: &MultiHpuConfig) -> MultiHpu {
        let hpus = (0..config.n_hpus)
            .map(|i| Hpu::new(&config.hpu_config, HpuId(i)))
            .collect();
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
                        issue: trigger.issue,
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
