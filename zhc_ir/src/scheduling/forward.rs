//! Forward list scheduling implementation for instruction scheduling.
//!
//! This module provides a small framework for implementing forward list scheduling algorithms.
use crate::{Dialect, IR, OpMap};

use super::{Ready, Retired, Schedule, Selected};

/// Trait for implementing forward list scheduling algorithms.
///
/// Implementers of this trait define the scheduling policy by providing
/// selection logic and simulation of execution timing.
pub trait ForwardSimulator {
    /// The IR dialect this scheduler operates on.
    type Dialect: Dialect;

    /// Selects which ready operations to issue in the current cycle.
    ///
    /// Given an iterator of `ready` operations that can be scheduled,
    /// this method returns an iterator of operations that are selected
    /// for execution.
    fn select(&mut self, ready: impl Iterator<Item = Ready>)
    -> impl Iterator<Item = Selected> + '_;

    /// Advances the simulation and returns operations that have completed.
    ///
    /// This method simulates the passage of time in the execution model
    /// and returns an iterator of operations that have finished executing
    /// and can be retired.
    fn advance(&mut self) -> impl Iterator<Item = Retired>;
}

/// Extension of [`ForwardSimulator`] that drives the full scheduling loop.
///
/// Automatically implemented for all [`ForwardSimulator`] implementors.
pub trait ForwardScheduler: ForwardSimulator {
    /// Performs forward list scheduling on the given IR using the specified scheduler.
    ///
    /// This function implements the main scheduling loop that coordinates between
    /// the scheduler policy (provided by `S`) and the dependency tracking.
    /// It repeatedly selects ready operations, issues them, simulates execution,
    /// and retires completed operations until all operations are scheduled.
    ///
    /// The scheduling process maintains correctness by respecting all data dependencies
    /// while allowing the scheduler implementation to reorder operations for optimization.
    fn schedule(&mut self, ir: &IR<Self::Dialect>) -> Schedule;
}

impl<T: ForwardSimulator> ForwardScheduler for T {
    fn schedule(&mut self, ir: &IR<Self::Dialect>) -> Schedule {
        let mut sched = Schedule::empty();
        let mut tracker = Tracker::from_ir(ir);
        let mut selected = Vec::new();
        let mut retired = Vec::new();

        while !tracker.over() {
            let selected_iter = self.select(tracker.ready_iter());
            selected.extend(selected_iter);
            tracker.issue_selected(selected.iter().copied());
            sched.issue_selected(selected.iter().copied());
            let retired_iter = self.advance();
            retired.extend(retired_iter);
            tracker.retire(retired.iter().copied());
            selected.clear();
            retired.clear();
        }

        sched
    }
}

/// Tracks the state of operations during the scheduling process.
///
/// The tracker maintains the current state of each operation and handles
/// state transitions as operations move through the scheduling pipeline.
struct Tracker<'i, D: Dialect> {
    states: OpMap<State>,
    ir: &'i IR<D>,
}

impl<'i, D: Dialect> Tracker<'i, D> {
    /// Creates a new tracker from the given IR.
    ///
    /// Initializes the state of each operation based on its dependencies:
    /// - Operations with no predecessors start in [`State::Ready`]
    /// - Operations with dependencies start in [`State::Locked`] with their dependency count
    fn from_ir(ir: &'i IR<D>) -> Self {
        let mut states = ir.filled_opmap(State::Retired);
        ir.walk_ops_linear()
            .for_each(|op| match op.get_predecessors_iter().count() {
                0 => {
                    states.insert(op.get_id(), State::Ready);
                }
                a => {
                    states.insert(op.get_id(), State::Locked(a));
                }
            });
        Self { states, ir }
    }

    /// Checks if all operations have completed scheduling.
    ///
    /// Returns `true` when all operations are in the [`State::Retired`] state,
    /// indicating that the scheduling process is complete.
    fn over(&self) -> bool {
        self.states.iter().all(|(_, a)| *a == State::Retired)
    }

    /// Returns an iterator over operations that are ready to be scheduled.
    ///
    /// Yields all operations currently in the [`State::Ready`] state,
    /// which can be considered for selection by the scheduler.
    fn ready_iter(&self) -> impl Iterator<Item = Ready> {
        self.states
            .iter()
            .filter(|(_, s)| **s == State::Ready)
            .map(|(opid, _)| Ready(opid))
    }

    /// Marks selected operations as active.
    ///
    /// Transitions the given `selected` operations from [`State::Ready`] to
    /// [`State::Active`], indicating they have been issued for execution.
    ///
    /// # Panics
    ///
    /// Panics if any of the selected operations is not in the [`State::Ready`] state.
    fn issue_selected(&mut self, selected: impl Iterator<Item = Selected>) {
        selected.for_each(|sel| {
            assert_eq!(*self.states.get(&sel.0).unwrap(), State::Ready);
            self.states.insert(sel.0, State::Active);
        });
    }

    /// Retires completed operations and updates dependent operations.
    ///
    /// Transitions the `retired` operations from [`State::Active`] to [`State::Retired`]
    /// and decrements the dependency count of their dependent operations.
    /// Operations whose dependency count reaches zero are moved to [`State::Ready`].
    ///
    /// # Panics
    ///
    /// Panics if any of the retired operations is not in the [`State::Active`] state,
    /// or if a dependent operation is in an unexpected state.
    fn retire(&mut self, retired: impl Iterator<Item = Retired>) {
        for Retired(ret) in retired {
            assert_eq!(*self.states.get(&ret).unwrap(), State::Active);
            self.states.insert(ret, State::Retired);
            for dep in self.ir.get_op(ret).get_users_iter() {
                let depid = dep.get_id();
                let val = match self.states.get(&depid).unwrap() {
                    State::Locked(1) => State::Ready,
                    State::Locked(a) if *a > 1 => State::Locked(a - 1),
                    _ => unreachable!(),
                };
                self.states.insert(depid, val);
            }
        }
    }
}

/// Represents the execution state of an operation during scheduling.
#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    /// Operation is waiting for dependencies, with count of remaining dependencies.
    Locked(usize),
    /// Operation is ready to be scheduled.
    Ready,
    /// Operation has been issued and is executing.
    Active,
    /// Operation has completed execution.
    Retired,
}
