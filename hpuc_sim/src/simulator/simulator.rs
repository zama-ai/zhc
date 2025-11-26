use serde::{Deserialize, Serialize};

use super::*;
use hpuc_utils::tracing::Microseconds;
use std::path::Path;

static ACTIVATE_TRACING: bool = false;
static S_IN_US: f64 = 1_000_000.;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
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

pub struct Simulator<S, D = Dispatcher<<S as Simulatable>::Event>>
where
    S: Simulatable,
    D: Dispatch<Event = S::Event> + Simulate<Event = S::Event>,
{
    pub simulatable: S,
    quantum: Microseconds,
    tracer: Tracer<S::Event>,
    dispatcher: D,
}

impl<S: Simulatable> Simulator<S> {
    pub fn from_simulatable(freq: MHz, simulatable: S) -> Self {
        Self::from_simulatable_and_dispatcher(freq, simulatable, Dispatcher::default())
    }
}

impl<S, D> Simulator<S, D>
where
    S: Simulatable,
    D: Dispatch<Event = S::Event> + Simulate<Event = S::Event>,
{
    pub fn new(freq: MHz) -> Self
    where
        D: Default,
        S: Default,
    {
        let simulatable = S::default();
        let dispatcher = D::default();
        Self::from_simulatable_and_dispatcher(freq, simulatable, dispatcher)
    }

    pub fn from_simulatable_and_dispatcher(freq: MHz, simulatable: S, mut dispatcher: D) -> Self {
        let quantum = freq.period();
        let tracer = Tracer::new();
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
        self.dispatcher.now()
    }

    pub fn now_us(&self) -> Microseconds {
        self.dispatcher.now().as_ts(self.quantum)
    }

    pub fn step(&mut self) -> SimulationState {
        fn nope<D: Dispatch>(_: &Trigger<D::Event>) -> bool {
            false
        }
        self.step_cond(nope::<D>)
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
            if ACTIVATE_TRACING {
                self.tracer.add_event(&trigger.event);
            }
            self.simulatable.handle(&mut self.dispatcher, trigger);
        }

        if ACTIVATE_TRACING {
            self.simulatable.report(&mut self.tracer);
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

    pub fn play_until(&mut self, f: impl Fn(&S::Event) -> bool) {
        let event_cond = |trig: &Trigger<S::Event>| -> bool { f(&trig.event) };
        while let SimulationState::MayContinue = self.step_cond(event_cond) {}
    }

    pub fn dump_trace<P: AsRef<Path>>(&self, path: P) {
        self.tracer.dump(path);
    }

    pub fn simulatable(&self) -> &S {
        &self.simulatable
    }
}

impl<S, D> AsRef<S> for Simulator<S, D>
where
    S: Simulatable,
    D: Dispatch<Event = S::Event> + Simulate<Event = S::Event>,
{
    fn as_ref(&self) -> &S {
        &self.simulatable
    }
}
