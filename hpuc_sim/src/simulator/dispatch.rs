use std::collections::BinaryHeap;

use super::*;

pub struct Dispatcher<E: Event> {
    pub(super) now: Cycle,
    pub(super) triggers: BinaryHeap<Trigger<E>>,
}

impl<E: Event> Dispatcher<E> {
    pub fn new() -> Self {
        Dispatcher {
            now: Cycle::ZERO,
            triggers: BinaryHeap::new(),
        }
    }

    pub fn now(&self) -> Cycle {
        self.now
    }

    pub fn dispatch_now(&mut self, event: E) {
        self.dispatch_later(Cycle::ZERO, event);
    }

    pub fn dispatch_next(&mut self, event: E) {
        self.dispatch_later(Cycle::ONE, event);
    }

    pub fn dispatch_later(&mut self, after_n_cycles: Cycle, event: E) {
        self.triggers.push(Trigger {
            at: self.now + after_n_cycles,
            event,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.triggers.is_empty()
    }

    pub fn advance(&mut self) {
        if let Some(trigger) = self.triggers.peek() {
            self.now = trigger.at
        }
    }

    pub fn pop_now(&mut self) -> Option<Trigger<E>> {
        if let Some(trigger) = self.triggers.peek()
            && trigger.at == self.now
        {
            self.triggers.pop()
        } else {
            None
        }
    }

    pub fn contains_event(&self, event: &E) -> bool {
        self.triggers
            .iter()
            .map(|trigger| &trigger.event)
            .find(|e| *e == event)
            .is_some()
    }
}
