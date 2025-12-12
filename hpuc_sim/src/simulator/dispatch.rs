use std::collections::BinaryHeap;

use super::*;

/// Event dispatcher managing scheduled events using a priority queue.
pub struct Dispatcher<E: Event> {
    now: Cycle,
    triggers: BinaryHeap<Trigger<E>>,
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
    fn contains_event(&self, event: &Self::Event) -> bool {
        self.triggers
            .iter()
            .map(|trigger| &trigger.event)
            .find(|e| *e == event)
            .is_some()
    }
    fn dispatch(&mut self, event: Self::Event, delay: Option<Cycle>) {
        let delay = delay.unwrap_or(Cycle::ZERO);

        self.triggers.push(Trigger {
            at: self.now + delay,
            event,
        });
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
            self.now = trigger.at
        }
    }

    /// Removes and returns the next event scheduled for the current cycle.
    ///
    /// Returns `None` if no events are scheduled for the current cycle.
    pub fn pop_now(&mut self) -> Option<Trigger<E>> {
        if let Some(trigger) = self.triggers.peek()
            && trigger.at == self.now
        {
            self.triggers.pop()
        } else {
            None
        }
    }
}
