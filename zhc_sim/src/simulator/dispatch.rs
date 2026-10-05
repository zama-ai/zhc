use std::{cmp::Reverse, collections::BinaryHeap, marker::PhantomData};

use zhc_utils::units::Cycle;

use super::*;

/// Event dispatcher managing scheduled events using a priority queue.
pub struct Dispatcher<E: Event> {
    now: Cycle,
    next_issue: u64,
    triggers: BinaryHeap<Reverse<Trigger<E>>>,
}

impl<E: Event> Default for Dispatcher<E> {
    fn default() -> Self {
        Self {
            now: Cycle::ZERO,
            next_issue: 0,
            triggers: BinaryHeap::new(),
        }
    }
}

impl<E: Event> Dispatch for Dispatcher<E> {
    type Event = E;

    fn contains_event(&self, event: &Self::Event) -> bool {
        self.triggers
            .iter()
            .map(|trigger| &trigger.0.event)
            .find(|e| *e == event)
            .is_some()
    }
    fn dispatch(&mut self, event: Self::Event, delay: Option<Cycle>) {
        let dispatch_cycle = self.now + delay.unwrap_or(Cycle::ZERO);
        self.triggers.push(Reverse(Trigger {
            at: dispatch_cycle,
            issue: self.next_issue,
            event,
        }));
        self.next_issue += 1;
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

/// A [`Dispatch`] adapter presenting an inner dispatcher under a foreign event
/// type.
///
/// Wraps a mutable borrow of an inner dispatcher together with a function `F`
/// that converts events of the adapter's type `E` into the inner dispatcher's
/// event type; every dispatched or queried event is passed through `F` before
/// reaching the inner dispatcher. Created by [`MapDispatch::map`].
pub struct MappedDispatcher<'a, D: Dispatch, E: Event, F: Fn(E) -> D::Event> {
    inner: &'a mut D,
    map: F,
    phantom: PhantomData<E>,
}

impl<'a, D: Dispatch, E: Event, F: Fn(E) -> D::Event> Dispatch for MappedDispatcher<'a, D, E, F> {
    type Event = E;

    fn contains_event(&self, event: &Self::Event) -> bool {
        self.inner.contains_event(&(self.map)(event.to_owned()))
    }

    fn dispatch(&mut self, event: Self::Event, delay: Option<Cycle>) {
        self.inner.dispatch((self.map)(event), delay);
    }
}

/// Extension trait adapting any [`Dispatch`] to a foreign event type.
pub trait MapDispatch
where
    Self: Dispatch + Sized,
{
    /// Adapts this dispatcher to accept events of type `E`, translating each
    /// through `f` into this dispatcher's event type before dispatch.
    fn map<E: Event, F: Fn(E) -> Self::Event>(&mut self, f: F) -> MappedDispatcher<'_, Self, E, F>;
}

impl<D: Dispatch + Sized> MapDispatch for D {
    fn map<E: Event, F: Fn(E) -> Self::Event>(&mut self, f: F) -> MappedDispatcher<'_, Self, E, F> {
        MappedDispatcher {
            inner: self,
            map: f,
            phantom: PhantomData,
        }
    }
}
