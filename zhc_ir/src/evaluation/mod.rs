//! IR evaluation framework.
//!
//! Provides the traits and drivers used to execute the operations of a dialect over a semantic
//! domain, binding a value to every SSA value of an [`IR`](crate::IR).
//!
//! A domain is described by three traits: [`Evaluation`] marks a type as an evaluation value,
//! [`EvaluatesTo`] connects a dialect's type system to that domain so values can be checked
//! against operation signatures, and [`Evaluable`] gives each operation of an instruction set its
//! computational behaviour together with the mutable [`Context`](Evaluable::Context) threaded
//! through a whole run. A dialect may be evaluated in several domains at once, each selected by
//! its value type.
//!
//! Three drivers consume those traits. [`PureEvaluator`] and [`EffectfulEvaluator`] are
//! push-based: they sweep the IR in dependency order, advancing operations whose arguments have
//! already settled. [`LazyEvaluator`] is pull-based: the caller names a value or an operation and
//! the driver evaluates exactly the dependencies needed to produce it. All record their outcome
//! as one [`OpState`] per operation and one [`ValState`] per value, and all can be turned into an
//! annotated IR carrying those states.
//!
//! An operation may also report through [`EvalOutcome::Blocked`] that it is not ready, when its
//! readiness depends on the context rather than on its arguments. Only [`EffectfulEvaluator`]
//! supports this: it sweeps again until every operation has settled, and reports the ones still
//! blocked when no sweep makes progress. The two other drivers panic on a blocked operation.
//!
//! Runtime failures are contained rather than propagated. A panic raised by
//! [`eval`](Evaluable::eval) is captured as [`OpState::Panicked`] holding its message, and the
//! operation's results become [`ValState::PoisonedBy`] carrying that operation's [`OpId`]. Any
//! operation reading a poisoned value is skipped and re-emits the same id, so a poison marker
//! always identifies the operation that actually failed rather than the nearest failing
//! predecessor. Violations of the framework's own contract — a value whose type does not inhabit
//! the one declared by a signature — panic out of the driver instead, since they indicate a
//! faulty dialect implementation rather than a failure of the program being evaluated.

use crate::{
    Annotation, DialectInstructionSet, DialectTypeSystem, OpId,
    visualization::{StyleModifier, VisualAnnotation},
};
use std::{fmt::Debug, time::Duration};

mod eager;
mod effectful;
mod lazy;
mod pure;

pub(crate) use eager::*;
pub use effectful::*;
pub use lazy::*;
pub use pure::*;
use zhc_utils::{graphics::Color, small::SmallVec};

/// Marker trait for types that serve as evaluation values.
///
/// The implementing type names an evaluation domain, selecting which [`EvaluatesTo`] and
/// [`Evaluable`] implementations a driver picks up.
pub trait Evaluation: Annotation {}

/// Defines the type semantics for an evaluation domain.
///
/// Implemented by a dialect's type system for every value type it can be evaluated to, letting the
/// drivers check that the values flowing through an operation match its declared signature.
pub trait EvaluatesTo<I: Evaluation>: DialectTypeSystem {
    /// Returns the type inhabited by an evaluation value.
    ///
    /// Must be total over the domain: every value of `interp`'s type reachable during a run has to
    /// map to a type, since the result is used both to check operation signatures and to report
    /// the offending type on a mismatch.
    fn type_of(interp: &I) -> Self;

    /// Returns `true` if the given value inhabits this type.
    ///
    /// The default implementation compares `self` against [`type_of`](Self::type_of) for equality.
    /// Override it when a type is inhabited by values whose canonical type differs from it, for
    /// instance when the type system carries information no value can recover.
    fn is_inhabited_by(&self, interp: &I) -> bool {
        Self::type_of(interp) == *self
    }
}

/// Defines how an operation computes on evaluation values.
///
/// Implemented by a dialect's instruction set for every evaluation domain its operations support.
/// The instruction set's own type system must evaluate to the same domain, so that arguments and
/// results can be checked against operation signatures.
pub trait Evaluable<I: Evaluation>: DialectInstructionSet
where
    <Self as DialectInstructionSet>::TypeSystem: EvaluatesTo<I>,
{
    /// Mutable state threaded through the evaluation.
    ///
    /// A single instance is borrowed by every operation of a run, making it the place to hold the
    /// machine state operations mutate as a side effect.
    type Context: Debug;

    /// Executes the operation on the given arguments.
    ///
    /// `arguments` holds one value per argument of the operation's signature, in declaration
    /// order. On [`EvalOutcome::Evaluated`], the returned vector must likewise hold one value per
    /// declared result, in the same order; the drivers bind results positionally without checking
    /// the count. `context` is shared with every other operation of the run and may be mutated
    /// freely.
    ///
    /// An operation whose readiness depends on the context rather than on its arguments alone,
    /// such as one waiting for a message another operation has not yet posted, returns
    /// [`EvalOutcome::Blocked`]. The operation then stays pending and may be evaluated again
    /// later. An implementation must check its readiness before mutating `context`, so that a
    /// blocked call leaves no trace of itself and a later retry does not apply an effect twice.
    ///
    /// Panicking is the supported way to signal a runtime failure: the drivers catch the panic,
    /// record its message on the operation and poison the operation's results rather than
    /// unwinding any further.
    fn eval(&self, context: &mut Self::Context, arguments: SmallVec<&I>) -> EvalOutcome<I>;
}

/// Result of a single [`eval`](Evaluable::eval) call.
///
/// Separates an operation that ran from one that could not run yet. Only operations with effects
/// on the [`Context`](Evaluable::Context) ever block; a pure operation always evaluates once its
/// arguments are available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalOutcome<I: Evaluation> {
    /// The operation ran and produced one value per declared result.
    Evaluated(SmallVec<I>),
    /// The operation is not ready yet and left the context untouched.
    Blocked,
}

impl<I: Evaluation> EvalOutcome<I> {
    /// Returns `true` if the outcome is [`Blocked`](EvalOutcome::Blocked).
    pub fn is_blocked(&self) -> bool {
        matches!(self, EvalOutcome::Blocked)
    }

    /// Returns the produced values.
    ///
    /// # Panics
    ///
    /// Panics if the outcome is [`Blocked`](EvalOutcome::Blocked).
    pub fn unwrap_evaluated(self) -> SmallVec<I> {
        match self {
            EvalOutcome::Evaluated(values) => values,
            EvalOutcome::Blocked => panic!("Called unwrap_evaluated on Blocked"),
        }
    }
}

/// State of a value during evaluation.
///
/// Every active value of the IR starts out pending and settles at most once, either into
/// `Evaluated` once its producing operation has run or into `PoisonedBy`, which carries the
/// [`OpId`] of the operation whose panic ultimately made the value uncomputable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValState<V: Evaluation> {
    /// Value has not yet been evaluated.
    Pending,
    /// Value was successfully evaluated.
    Evaluated(V),
    /// Upstream computation failed; this value cannot be computed.
    PoisonedBy(OpId),
}

impl<V: Evaluation> ValState<V> {
    /// Returns `true` if the state is [`Evaluated`](ValState::Evaluated).
    pub fn is_evaluated(&self) -> bool {
        matches!(self, ValState::Evaluated(_))
    }

    /// Returns `true` if the state is [`PoisonedBy`](ValState::PoisonedBy).
    pub fn is_failed(&self) -> bool {
        matches!(self, ValState::PoisonedBy(_))
    }

    /// Returns the evaluated value, consuming the state.
    ///
    /// # Panics
    ///
    /// Panics if the state is not [`Evaluated`](ValState::Evaluated).
    pub fn unwrap_evaluated(self) -> V {
        match self {
            ValState::Evaluated(v) => v,
            ValState::Pending => panic!("Called unwrap on Pending"),
            ValState::PoisonedBy(_) => panic!("Called unwrap on PoisonedBy"),
        }
    }

    /// Returns a reference to the evaluated value, or `None` in any other state.
    pub fn as_evaluated(&self) -> Option<&V> {
        match self {
            ValState::Evaluated(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `self` unchanged if not pending.
    ///
    /// # Panics
    ///
    /// Panics with `msg` if the state is [`Pending`](ValState::Pending).
    pub fn ensure_not_pending(&self, msg: &str) -> &Self {
        if matches!(self, ValState::Pending) {
            panic!("{msg}")
        }
        self
    }
}

/// State of an operation during evaluation.
///
/// Mirrors [`ValState`] on the operation side. An operation starts out pending and settles at most
/// once, into `Evaluated`, into `Panicked` holding the message of the panic raised by
/// [`eval`](Evaluable::eval), or into `PoisonedBy` holding the [`OpId`] of the operation that
/// panicked upstream. Only `Panicked` marks an operation as the origin of a failure; `PoisonedBy`
/// means the operation was skipped because one of its arguments was already uncomputable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpState {
    Pending,
    Evaluated(Option<Duration>),
    Panicked(String),
    PoisonedBy(OpId),
}

impl OpState {
    /// Returns `true` if the state is [`Pending`](OpState::Pending).
    pub fn is_pending(&self) -> bool {
        matches!(self, OpState::Pending)
    }

    /// Returns `true` if the state is [`Panicked`](OpState::Pending).
    pub fn is_panic(&self) -> bool {
        matches!(self, OpState::Panicked(_))
    }

    pub fn unwrap_panic(self) -> String {
        let OpState::Panicked(s) = self else { panic!() };
        s
    }

    /// Asserts that the operation evaluated successfully.
    ///
    /// # Panics
    ///
    /// Panics if the state is not [`Evaluated`](OpState::Evaluated).
    pub fn unwrap_evaluated(self) -> Option<Duration> {
        match self {
            Self::Evaluated(v) => v,
            _ => panic!(),
        }
    }
}

impl VisualAnnotation for OpState {
    fn style_modifier(&self) -> Option<StyleModifier> {
        match self {
            OpState::Pending => Some(StyleModifier {
                fill: Some(Color::WHITE.into()),
                ..Default::default()
            }),
            OpState::Evaluated(_) => Some(StyleModifier {
                fill: Some(Color::GREEN.into()),
                ..Default::default()
            }),
            OpState::Panicked(_) => Some(StyleModifier {
                fill: Some(Color::RED.into()),
                ..Default::default()
            }),
            OpState::PoisonedBy(_) => Some(StyleModifier {
                fill: Some(Color::ORANGE.into()),
                ..Default::default()
            }),
        }
    }
}

/// Extracts the panic message from a panic payload.
fn extract_panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(s) => *s,
        Err(payload) => match payload.downcast::<&str>() {
            Ok(s) => s.to_string(),
            Err(_) => "unknown panic".to_string(),
        },
    }
}

/// Reason a value could not be read back from an evaluator.
///
/// `UnknownValId` means the evaluator holds no state at all for the queried value, while
/// `PendingValId` and `PoisonedValId` report its current [`ValState`], the latter carrying the
/// [`OpId`] of the operation that panicked.
#[derive(Debug, Clone)]
pub enum EvalError {
    UnknownValId,
    PendingValId,
    PoisonedValId(OpId),
}
