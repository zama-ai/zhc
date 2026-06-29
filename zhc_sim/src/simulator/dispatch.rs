use std::{cmp::Reverse, collections::BinaryHeap, marker::PhantomData};

use super::*;

/// Event dispatcher managing scheduled events using a priority queue.
pub struct Dispatcher<E: Event> {
    now: Cycle,
    triggers: BinaryHeap<Reverse<Trigger<E>>>,
}

impl<E: Event> Dispatcher<E> {
    pub fn from_raw_parts(now: Cycle, triggers: impl Iterator<Item = Trigger<E>>) -> Self {
        Dispatcher {
            now,
            triggers: triggers.map(Reverse).collect(),
        }
    }
}

impl<E: Event> Default for Dispatcher<E> {
    fn default() -> Self {
        Self {
            now: Cycle::ZERO,
            triggers: BinaryHeap::new(),
        }
    }
}

impl<E: Event> Dispatch for Dispatcher<E> {
    type Event = E;

    fn contains_event(&self, event: &Self::Event, filter: Option<Cycle>) -> bool {
        if let Some(filter_at) = filter.as_ref() {
            self.triggers
                .iter()
                .find(|Reverse(Trigger { at, event: e })| (e == event) && (at == filter_at))
                .is_some()
        } else {
            self.triggers
                .iter()
                .map(|trigger| &trigger.0.event)
                .find(|e| *e == event)
                .is_some()
        }
    }
    fn dispatch(&mut self, event: Self::Event, delay: Option<Cycle>) {
        let dispatch_cycle = self.now + delay.unwrap_or(Cycle::ZERO);
        // NB: Discard event dispach in the current cycle if already present
        if !self.contains_event(&event, Some(dispatch_cycle)) {
            self.triggers.push(Reverse(Trigger {
                at: dispatch_cycle,
                event,
            }));
        }
    }
}

impl<E: Event> Dispatcher<E> {
    /// Returns the current simulation cycle.
    pub fn now(&self) -> Cycle {
        self.now
    }

    /// Checks if there are no scheduled events remaining.
    pub fn is_empty(&self) -> bool {
        self.triggers.is_empty()
    }

    /// Advances the simulation time to the next scheduled event.
    pub fn advance(&mut self) {
        if let Some(trigger) = self.triggers.peek() {
            self.now = trigger.0.at
        }
    }

    /// Removes and returns the next event scheduled for the current cycle.
    ///
    /// Returns `None` if no events are scheduled for the current cycle.
    pub fn pop_now(&mut self) -> Option<Trigger<E>> {
        if let Some(trigger) = self.triggers.peek()
            && trigger.0.at == self.now
        {
            self.triggers.pop().map(|a| a.0)
        } else {
            None
        }
    }
}

pub struct MappedDispatcher<'a, D: Dispatch, E: Event, F: Fn(E) -> D::Event> {
    inner:&'a mut D,
    map: F,
    phantom: PhantomData<E>
}


impl<'a, D: Dispatch, E: Event, F: Fn(E) -> D::Event> Dispatch for MappedDispatcher<'a, D,E,F> {
    type Event = E;

    fn contains_event(&self, event: &Self::Event, filter: Option<Cycle>) -> bool {
        self.inner.contains_event(&(self.map)(event.to_owned()), filter)
    }

    fn dispatch(&mut self, event: Self::Event, delay: Option<Cycle>) {
        self.inner.dispatch((self.map)(event), delay);
    }
}

pub trait MapDispatch where Self: Dispatch + Sized {
    fn map<E: Event, F: Fn(E) -> Self::Event>(&mut self, f: F) -> MappedDispatcher<'_, Self, E, F>;
}

impl<D: Dispatch + Sized> MapDispatch for D {
    fn map<E: Event, F: Fn(E) -> Self::Event>(&mut self, f: F) -> MappedDispatcher<'_, Self, E, F> {
        MappedDispatcher { inner: self, map: f, phantom: PhantomData }
    }
}
