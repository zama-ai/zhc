use serde::Serialize;
use std::fmt::Display;

use hc_utils::type_name_of_val;

use super::*;

/// Represents simulation events that can be dispatched and handled.
pub trait Event: Display + Clone + Serialize + PartialEq {}

/// Manages event scheduling and delivery within the simulation.
pub trait Dispatch {
    type Event: Event;

    /// Checks if the given `event` is already scheduled for dispatch.
    /// Could filtered on a given cycle
    fn contains_event(&self, event: &Self::Event, filter: Option<Cycle>) -> bool;

    /// Schedules an `event` for dispatch after the specified `delay` in cycles.
    ///
    /// If `delay` is `None`, the event is dispatched immediately.
    fn dispatch(&mut self, event: Self::Event, delay: Option<Cycle>);

    /// Schedules an `event` for immediate dispatch.
    fn dispatch_now(&mut self, event: Self::Event) {
        self.dispatch(event, None);
    }

    /// Schedules an `event` for dispatch in the next cycle.
    fn dispatch_next(&mut self, event: Self::Event) {
        self.dispatch(event, Some(Cycle::ONE));
    }

    /// Schedules an `event` for dispatch after `after_n_cycles` cycles.
    fn dispatch_after(&mut self, after_n_cycles: Cycle, event: Self::Event) {
        self.dispatch(event, Some(after_n_cycles));
    }

    /// Schedules an `event` for dispatch after `after_n_cycles` cycles if not already scheduled.
    fn dispatch_after_if_no_there(&mut self, after_n_cycles: Cycle, event: Self::Event) {
        if !self.contains_event(&event, None) {
            self.dispatch_after(after_n_cycles, event);
        }
    }
}

/// Defines the interface for components that participate in discrete event simulation.
pub trait Simulatable: Sized + Serialize {
    type Event: Event;

    /// Processes the given `trigger` event and updates internal state.
    ///
    /// The component can use the `dispatcher` to schedule future events.
    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    );

    /// Initializes the component at simulation start.
    ///
    /// This method is called once before simulation begins and can be used
    /// to schedule initial events or set up the component's initial state.
    fn power_up(&self, _: &mut impl Dispatch<Event = Self::Event>) {}

    /// Returns the name of this simulatable component.
    fn name(&self) -> String {
        type_name_of_val(self).into()
    }

    /// Records the component's state in the `tracer` at the specified cycle.
    fn report(&self, at: Cycle, tracer: &mut Tracer<Self::Event>) {
        tracer.add_simulatable(at, self);
    }
}

macro_rules! impl_simulatable_for_tuple {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<E: Event, $($T),+> Simulatable for ($($T,)+)
        where
            $($T: Simulatable<Event = E>,)+
        {
            type Event = E;

            fn handle(&mut self, dispatcher: &mut impl Dispatch<Event = Self::Event>, trigger: Trigger<Self::Event>) {
                let ($($T),+) = self ;
                $(
                $T.handle(dispatcher, trigger.clone());
                )+
            }

            fn power_up(&self, dispatcher: &mut impl Dispatch<Event = Self::Event>) {
                let ($($T),+) = self ;
                $(
                $T.power_up(dispatcher);
                )+
            }

            fn report(&self, at: Cycle, tracer: &mut Tracer<Self::Event>) {
                let ($($T),+) = self ;
                $(
                $T.report(at, tracer);
                )+
            }
        }
    };
}

impl_simulatable_for_tuple!(A, B);
impl_simulatable_for_tuple!(A, B, C);
impl_simulatable_for_tuple!(A, B, C, D);
impl_simulatable_for_tuple!(A, B, C, D, F);
impl_simulatable_for_tuple!(A, B, C, D, F, G);
impl_simulatable_for_tuple!(A, B, C, D, F, G, H);
