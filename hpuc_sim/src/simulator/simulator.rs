use serde::Deserialize;

use super::*;
use hpuc_utils::tracing::Microseconds;
use std::path::Path;

static ACTIVATE_TRACING: bool = false;
static S_IN_US: f64 = 1_000_000.;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct MHz(pub(crate) f64);

impl MHz {
    fn as_raw_hertz(&self) -> f64 {
        self.0 * 1_000_000.
    }

    fn period(&self) -> Microseconds {
        (1. / self.as_raw_hertz()) * S_IN_US
    }
}

pub enum SimulationState {
    MayContinue,
    SimulationOver,
    CondEncountered,
}

pub struct Simulator<S: Simulatable> {
    pub simulatable: S,
    quantum: Microseconds,
    tracer: Tracer<S::Event>,
    dispatcher: Dispatcher<S::Event>,
}

impl<S: Simulatable> Simulator<S> {
    pub fn new(freq: MHz) -> Self
    where
        S: Default,
    {
        let simulatable = S::default();
        Self::from_simulatable(freq, simulatable)
    }

    pub fn from_simulatable(freq: MHz, simulatable: S) -> Self {
        let quantum = freq.period();
        let tracer = Tracer::new();
        let mut dispatcher = Dispatcher::new();
        simulatable.power_up(&mut dispatcher);
        Simulator {
            simulatable,
            tracer,
            quantum,
            dispatcher,
        }
    }

    pub fn dispatch(&mut self, event: S::Event) {
        self.dispatcher.dispatch_now(event);
    }

    pub fn dispatch_later(&mut self, after_n_cycles: Cycle, event: S::Event) {
        self.dispatcher.dispatch_later(after_n_cycles, event);
    }

    pub fn now(&self) -> Cycle {
        self.dispatcher.now
    }

    pub fn now_us(&self) -> Microseconds {
        self.dispatcher.now.as_ts(self.quantum)
    }

    pub fn step(&mut self) -> SimulationState {
        fn nope<R: Simulatable>(_: &Trigger<R::Event>) -> bool {
            false
        }
        self.step_cond(nope::<S>)
    }

    fn step_cond(&mut self, condition: impl Fn(&Trigger<S::Event>) -> bool) -> SimulationState {
        self.dispatcher.advance();
        self.tracer.set_now(self.dispatcher.now());
        let mut cond_encountered = false;

        if self.now().is_zero() {
            self.simulatable.report(&mut self.tracer);
        }

        if self.dispatcher.is_empty() {
            return SimulationState::SimulationOver;
        }

        while let Some(trigger) = self.dispatcher.pop_now() {
            cond_encountered |= condition(&trigger);
            self.tracer.add_event(&trigger.event);
            self.simulatable.handle(&mut self.dispatcher, trigger);
        }

        if ACTIVATE_TRACING {
            self.simulatable.report(&mut self.tracer);
            // if self.now() > Cycle(150000) {
            //     self.dump_trace("test_profile.json");
            //     panic!();
            // }
        }

        if cond_encountered {
            SimulationState::CondEncountered
        } else if self.dispatcher.is_empty() {
            SimulationState::SimulationOver
        } else {
            SimulationState::MayContinue
        }
    }

    pub fn play(&mut self) {
        while let SimulationState::MayContinue = self.step() {}
    }

    pub fn play_until_event(&mut self, event: S::Event) {
        let event_eq = |trig: &Trigger<S::Event>| -> bool { trig.event == event };
        while let SimulationState::MayContinue = self.step_cond(event_eq) {}
    }

    pub fn dump_trace<P: AsRef<Path>>(&self, path: P) {
        self.tracer.dump(path);
    }

    pub fn simulatable(&self) -> &S {
        &self.simulatable
    }
}

impl<S: Simulatable> AsRef<S> for Simulator<S> {
    fn as_ref(&self) -> &S {
        &self.simulatable
    }
}
