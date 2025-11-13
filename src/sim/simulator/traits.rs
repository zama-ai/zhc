use serde::Serialize;
use std::fmt::Display;

use crate::utils::type_name_of_val;

use super::*;

/// Traits for types representing an event.
pub trait Event: Display + Clone + Serialize + PartialEq {}

/// Trait implemented by types that can be simulated.
pub trait Simulatable: Sized + Serialize {
    type Event: Event;

    fn handle(
        &mut self,
        dispatcher: &mut Dispatcher<Self::Event>,
        trigger: Trigger<Self::Event>,
    );

    fn power_up(&self, _: &mut Dispatcher<Self::Event>){}

    fn name(&self) -> String {
        type_name_of_val(self).into()
    }

    fn report(&self, tracer: &mut Tracer<Self::Event>) {
        tracer.add_simulatable(self);
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

            fn handle(&mut self, dispatcher: &mut Dispatcher<Self::Event>, trigger: Trigger<Self::Event>) {
                let ($($T),+) = self ;
                $(
                $T.handle(dispatcher, trigger.clone());
                )+
            }

            fn power_up(&self, dispatcher: &mut Dispatcher<Self::Event>) {
                let ($($T),+) = self ;
                $(
                $T.power_up(dispatcher);
                )+
            }

            fn report(&self, tracer: &mut Tracer<Self::Event>) {
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
