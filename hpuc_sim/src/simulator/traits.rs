use serde::Serialize;
use std::fmt::Display;

use hpuc_utils::type_name_of_val;

use super::*;

/// Traits for types representing an event.
pub trait Event: Display + Clone + Serialize + PartialEq {}

/// Traits for types handling event dispatch
pub trait Dispatch {
    type Event: Event;
    fn contains_event(&self, event: &Self::Event) -> bool;

    fn dispatch(&mut self, event: Self::Event, delay: Option<Cycle>);

    fn dispatch_now(&mut self, event: Self::Event) {
        self.dispatch(event, None);
    }

    fn dispatch_next(&mut self, event: Self::Event) {
        self.dispatch(event, Some(Cycle::ONE));
    }

    fn dispatch_later(&mut self, after_n_cycles: Cycle, event: Self::Event) {
        self.dispatch(event, Some(after_n_cycles));
    }
}

/// Traits for types handling event simulation
// TODO Find a better trait name ?
pub trait Simulate {
    type Event: Event;

    fn now(&self) -> Cycle;
    fn is_empty(&self) -> bool;

    fn advance(&mut self);

    fn pop_now(&mut self) -> Option<Trigger<Self::Event>>;
}

/// Trait implemented by types that can be simulated.
pub trait Simulatable: Sized + Serialize {
    type Event: Event;

    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    );

    fn power_up(&self, _: &mut impl Dispatch<Event = Self::Event>) {}

    fn name(&self) -> String {
        type_name_of_val(self).into()
    }

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
