use std::collections::BinaryHeap;

use super::*;

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

    fn now(&self) -> Cycle {
        self.now
    }

    fn dispatch_now(&mut self, event: Self::Event) {
        self.dispatch_later(Cycle::ZERO, event);
    }

    fn dispatch_next(&mut self, event: Self::Event) {
        self.dispatch_later(Cycle::ONE, event);
    }

    fn dispatch_later(&mut self, after_n_cycles: Cycle, event: Self::Event) {
        self.triggers.push(Trigger {
            at: self.now + after_n_cycles,
            event,
        });
    }

    fn is_empty(&self) -> bool {
        self.triggers.is_empty()
    }

    fn advance(&mut self) {
        if let Some(trigger) = self.triggers.peek() {
            self.now = trigger.at
        }
    }

    fn pop_now(&mut self) -> Option<Trigger<Self::Event>> {
        if let Some(trigger) = self.triggers.peek()
            && trigger.at == self.now
        {
            self.triggers.pop()
        } else {
            None
        }
    }

    fn contains_event(&self, event: &Self::Event) -> bool {
        self.triggers
            .iter()
            .map(|trigger| &trigger.event)
            .find(|e| *e == event)
            .is_some()
    }
}
