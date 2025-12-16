use serde::{Deserialize, Serialize};

use super::*;
use hpuc_utils::tracing::Microseconds;
use std::path::Path;

static ACTIVATE_TRACING: bool = cfg!(debug_assertions);
static S_IN_US: f64 = 1_000_000.;

/// Represents a frequency in megahertz for simulation timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct MHz(pub usize);

impl MHz {
    fn as_raw_hertz(&self) -> f64 {
        self.0 as f64 * 1_000_000.
    }

    /// Calculates the period duration in microseconds for this frequency.
    pub fn period(&self) -> Microseconds {
        (1. / self.as_raw_hertz()) * S_IN_US
    }
}

/// Represents the current state of simulation execution.
pub enum SimulationState {
    /// Simulation can continue with more cycles to process.
    MayContinue,
    /// Simulation has completed with no more events to process.
    SimulationOver,
    /// A user-specified condition was encountered during execution.
    CondEncountered,
}

/// Discrete event simulator that drives simulatable components at a given frequency.
pub struct Simulator<S>
where
    S: Simulatable,
{
    pub simulatable: S,
    quantum: Microseconds,
    tracer: Tracer<S::Event>,
    dispatcher: Dispatcher<S::Event>,
}

impl<S: Simulatable> Simulator<S> {
    /// Creates a simulator from the given `freq` and `simulatable` component.
    pub fn from_simulatable(freq: MHz, simulatable: S) -> Self {
        Self::from_simulatable_and_dispatcher(freq, simulatable, Dispatcher::default())
    }
}

impl<S: Simulatable> Simulator<S>
where
    S: Simulatable,
{
    /// Creates a new simulator at the given `freq` with a default simulatable component.
    pub fn new(freq: MHz) -> Self
    where
        S: Default,
    {
        let simulatable = S::default();
        let dispatcher = Dispatcher::default();
        Self::from_simulatable_and_dispatcher(freq, simulatable, dispatcher)
    }

    /// Creates a simulator from the given `freq`, `simulatable` component, and `dispatcher`.
    ///
    /// This constructor allows full control over the initial dispatcher state.
    /// The simulatable component is powered up during construction.
    pub fn from_simulatable_and_dispatcher(
        freq: MHz,
        simulatable: S,
        mut dispatcher: Dispatcher<S::Event>,
    ) -> Self {
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

    /// Dispatches an `event` for immediate processing.
    pub fn dispatch(&mut self, event: S::Event) {
        self.dispatcher.dispatch_now(event);
    }

    /// Dispatches an `event` to be processed after `after_n_cycles` cycles.
    pub fn dispatch_later(&mut self, after_n_cycles: Cycle, event: S::Event) {
        self.dispatcher.dispatch_after(after_n_cycles, event);
    }

    /// Returns the current simulation cycle.
    pub fn now(&self) -> Cycle {
        self.dispatcher.now()
    }

    /// Returns the current simulation time in microseconds.
    pub fn now_us(&self) -> Microseconds {
        self.dispatcher.now().as_ts(self.quantum)
    }

    /// Advances simulation by one cycle and processes all scheduled events.
    pub fn step(&mut self) -> SimulationState {
        fn nope<E: Event>(_: &Trigger<E>) -> bool {
            false
        }
        self.step_cond(nope::<S::Event>)
    }

    fn step_cond(&mut self, condition: impl Fn(&Trigger<S::Event>) -> bool) -> SimulationState {
        self.dispatcher.advance();
        let mut cond_encountered = false;

        if self.now().is_zero() {
            self.simulatable.report(self.now(), &mut self.tracer);
        }

        if self.dispatcher.is_empty() {
            return SimulationState::SimulationOver;
        }

        while let Some(trigger) = self.dispatcher.pop_now() {
            cond_encountered |= condition(&trigger);
            if ACTIVATE_TRACING {
                self.tracer.add_event(self.now(), &trigger.event);
            }
            if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.simulatable.handle(&mut self.dispatcher, trigger);
            })) {
                if ACTIVATE_TRACING {
                    self.dump_trace("test.json");
                    eprintln!("Panic caught during simulatable.handle(): {:?}", e);
                    panic!();
                }
            }
        }

        if ACTIVATE_TRACING {
            self.simulatable.report(self.now(), &mut self.tracer);
        }

        if cond_encountered {
            SimulationState::CondEncountered
        } else if self.dispatcher.is_empty() {
            SimulationState::SimulationOver
        } else {
            SimulationState::MayContinue
        }
    }

    /// Runs the simulation until completion or no more events remain.
    pub fn play(&mut self) {
        while let SimulationState::MayContinue = self.step() {}
    }

    /// Runs the simulation until the specified `event` is encountered.
    pub fn play_until_event(&mut self, event: S::Event) {
        let event_eq = |trig: &Trigger<S::Event>| -> bool { trig.event == event };
        loop {
            match self.step_cond(event_eq) {
                SimulationState::MayContinue => {},
                SimulationState::CondEncountered => return,
                SimulationState::SimulationOver => {
                    if ACTIVATE_TRACING {
                        self.dump_trace("test.json");
                    }
                    panic!("Simulation finished while waiting for an event.")
                },
            }
        }
    }

    /// Runs the simulation until the given condition function `f` returns true for an event.
    pub fn play_until(&mut self, f: impl Fn(&S::Event) -> bool) {
        let event_cond = |trig: &Trigger<S::Event>| -> bool { f(&trig.event) };
        loop {
            match self.step_cond(event_cond) {
                SimulationState::MayContinue => {},
                SimulationState::CondEncountered => return,
                SimulationState::SimulationOver => {
                    if ACTIVATE_TRACING {
                        self.dump_trace("test.json");
                    }
                    panic!("Simulation finished while waiting for an event.")
                },
            }
        }
    }

    /// Writes simulation trace data to the specified file `path`.
    pub fn dump_trace<P: AsRef<Path>>(&self, path: P) {
        self.tracer.dump(self.now(), path);
    }

    /// Returns a reference to the simulatable component.
    pub fn simulatable(&self) -> &S {
        &self.simulatable
    }

    /// Returns a mutable reference to the simulatable component.
    pub fn simulatable_mut(&mut self) -> &mut S {
        &mut self.simulatable
    }
}

impl<S> AsRef<S> for Simulator<S>
where
    S: Simulatable,
{
    fn as_ref(&self) -> &S {
        &self.simulatable
    }
}
