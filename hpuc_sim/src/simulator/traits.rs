use serde::Serialize;
use std::fmt::Display;

use hpuc_utils::type_name_of_val;

use super::*;

/// Traits for types representing an event.
pub trait Event: Display + Clone + Serialize + PartialEq {}

/// Traits for types handling event dispatch
pub trait Dispatch {
    type Event: Event;

    fn now(&self) -> Cycle;

    fn dispatch_now(&mut self, event: Self::Event);

    fn dispatch_next(&mut self, event: Self::Event);

    fn dispatch_later(&mut self, after_n_cycles: Cycle, event: Self::Event);

    fn is_empty(&self) -> bool;

    fn advance(&mut self);

    fn pop_now(&mut self) -> Option<Trigger<Self::Event>>;

    fn contains_event(&self, event: &Self::Event) -> bool;
}

/// Trait implemented by types that can be simulated.
pub trait Simulatable<D: Dispatch>: Sized + Serialize {
    fn handle(&mut self, dispatcher: &mut D, trigger: Trigger<D::Event>);

    fn power_up(&self, _: &mut D) {}

    fn name(&self) -> String {
        type_name_of_val(self).into()
    }

    fn report(&self, tracer: &mut Tracer<D::Event>) {
        tracer.add_simulatable(self);
    }
}

macro_rules! impl_simulatable_for_tuple {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<Dispatcher: Dispatch, $($T),+> Simulatable<Dispatcher> for ($($T,)+)
        where
            $($T: Simulatable<Dispatcher>,)+
        {
            fn handle(&mut self, dispatcher: &mut Dispatcher, trigger: Trigger<Dispatcher::Event>) {
                let ($($T),+) = self ;
                $(
                $T.handle(dispatcher, trigger.clone());
                )+
            }

            fn power_up(&self, dispatcher: &mut Dispatcher) {
                let ($($T),+) = self ;
                $(
                $T.power_up(dispatcher);
                )+
            }

            fn report(&self, tracer: &mut Tracer<Dispatcher::Event>) {
                let ($($T),+) = self ;
                $(
                $T.report(tracer);
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
