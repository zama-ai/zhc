//! IR interpretation framework.
//!
//! Provides traits and utilities for defining and executing interpretations of
//! dialect operations. An interpretation assigns semantic values to SSA values,
//! propagating them through the IR via forward dataflow analysis.
//!
//! The framework is structured around three traits:
//! - [`Interpretation`] defines the semantic domain (what values look like)
//! - [`InterpretsTo`] connects dialect types to interpretation values (type semantics)
//! - [`Interpretable`] defines how operations compute on interpretation values
//!
//! The main entry point is [`interpret_ir`], which executes an interpretation
//! over an entire IR, producing an annotated IR with interpretation values
//! attached to each SSA value.

use std::fmt::Debug;
use std::panic::{AssertUnwindSafe, catch_unwind};

use hc_utils::{
    iter::{CollectInSmallVec, MultiZip},
    small::SmallVec,
    svec,
};

use crate::{AnnIR, Annotation, Dialect, DialectInstructionSet, DialectTypeSystem, IR, ValMap};

/// State of a value during interpretation.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum InterpState<V: Interpretation> {
    /// Value has not yet been computed.
    Pending,
    /// Value was successfully computed.
    Interpreted(V),
    /// Upstream computation failed; this value cannot be computed.
    Poisoned,
    /// The operation producing this value panicked.
    Panicked(String),
}

impl<V: Interpretation> std::fmt::Debug for InterpState<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpState::Pending => write!(f, "Pending"),
            InterpState::Interpreted(v) => v.fmt(f),
            InterpState::Poisoned => write!(f, "Poisoned"),
            InterpState::Panicked(msg) => f.debug_tuple("Panicked").field(msg).finish(),
        }
    }
}

impl<V: Interpretation> InterpState<V> {
    /// Returns `true` if the state is `Interpreted`.
    pub fn is_interpreted(&self) -> bool {
        matches!(self, InterpState::Interpreted(_))
    }

    /// Returns `true` if the state is `Poisoned` or `Panicked`.
    pub fn is_failed(&self) -> bool {
        matches!(self, InterpState::Poisoned | InterpState::Panicked(_))
    }

    /// Unwraps the interpreted value, panicking if not `Interpreted`.
    pub fn unwrap(self) -> V {
        match self {
            InterpState::Interpreted(v) => v,
            InterpState::Pending => panic!("Called unwrap on Pending"),
            InterpState::Poisoned => panic!("Called unwrap on Poisoned"),
            InterpState::Panicked(msg) => panic!("Called unwrap on Panicked: {msg}"),
        }
    }

    /// Returns a reference to the interpreted value, if any.
    pub fn as_interpreted(&self) -> Option<&V> {
        match self {
            InterpState::Interpreted(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `self` if not `Pending`, otherwise panics with the given message.
    pub fn expect_not_pending(&self, msg: &str) -> &Self {
        if matches!(self, InterpState::Pending) {
            panic!("{msg}")
        }
        self
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

/// Marker trait for types that serve as interpretation values.
pub trait Interpretation: Annotation {}

/// Defines the type semantics for an interpretation domain.
pub trait InterpretsTo<I: Interpretation>: DialectTypeSystem {
    /// Returns the type of an interpretation value.
    fn type_of(interp: &I) -> Self;

    /// Returns true if the value inhabits this type.
    fn is_inhabited_by(&self, interp: &I) -> bool {
        Self::type_of(interp) == *self
    }
}

/// Defines how an operation computes on interpretation values.
pub trait Interpretable<I: Interpretation>: DialectInstructionSet
where
    <Self as DialectInstructionSet>::TypeSystem: InterpretsTo<I>,
{
    /// Mutable state threaded through the interpretation.
    type Context: Debug;

    /// Executes the operation on the given arguments.
    fn interpret(&self, context: &mut Self::Context, arguments: SmallVec<I>) -> SmallVec<I>;
}

/// Interprets an IR, returning an annotated IR with interpretation values.
///
/// Returns `Ok` with fully interpreted values on success, or `Err` with
/// partial interpretation state (containing `Panicked` and `Poisoned` markers)
/// on failure.
pub fn interpret_ir<'ir, D: Dialect, V: Interpretation>(
    ir: &'ir IR<D>,
    context: &mut <D::InstructionSet as Interpretable<V>>::Context,
) -> Result<AnnIR<'ir, D, (), V>, AnnIR<'ir, D, (), InterpState<V>>>
where
    D::InstructionSet: Interpretable<V>,
    D::TypeSystem: InterpretsTo<V>,
{
    let mut had_failure = false;

    let annotated = ir.forward_dataflow_analysis(|_, valmap: &ValMap<InterpState<V>>, opref| {
        let sig = opref.get_operation().get_signature();
        let n_returns = sig.get_returns().len();

        // Retrieve arguments and check for upstream failures
        let arguments = opref
            .get_arg_valids()
            .iter()
            .map(|valid| {
                let state = valmap
                    .get(valid)
                    .expect("Annotation not available in valmap.")
                    .expect_not_pending("Pending value encountered during interpretation");
                state.clone()
            })
            .cosvec();

        // If any input failed, outputs are poisoned
        if arguments.iter().any(|arg| arg.is_failed()) {
            return ((), svec![InterpState::Poisoned; n_returns]);
        }

        // Extract interpreted values (none are failed/pending at this point)
        let interpreted_args = arguments.into_iter().map(InterpState::unwrap).cosvec();

        // Typecheck the arguments
        for (i, (arg, expected_type)) in (interpreted_args.iter(), sig.get_args().iter())
            .mzip()
            .enumerate()
        {
            if !expected_type.is_inhabited_by(arg) {
                panic!(
                    "Unexpected argument type encountered while interpreting {}. \
                     At position {i}, expected type {expected_type}, but encountered {}.",
                    opref.format(),
                    D::TypeSystem::type_of(arg)
                )
            }
        }

        // Interpret with panic catching
        let interpret_result = catch_unwind(AssertUnwindSafe(|| {
            opref.get_operation().interpret(context, interpreted_args)
        }));

        match interpret_result {
            Ok(returns) => {
                // Typechecks the returns
                for (i, (ret, expected_type)) in (returns.iter(), sig.get_returns().iter())
                    .mzip()
                    .enumerate()
                {
                    if !expected_type.is_inhabited_by(ret) {
                        panic!(
                            "Unexpected return type encountered while interpreting {}. \
                             At position {i}, expected type {expected_type}, but encountered {}.",
                            opref.format(),
                            D::TypeSystem::type_of(ret)
                        )
                    }
                }
                let returns = returns.into_iter().map(InterpState::Interpreted).cosvec();
                ((), returns)
            }
            Err(payload) => {
                had_failure = true;
                let msg = extract_panic_message(payload);
                ((), svec![InterpState::Panicked(msg); n_returns])
            }
        }
    });

    if had_failure {
        Err(annotated)
    } else {
        // Unwrap all InterpState::Interpreted to V
        Ok(annotated.map_valann(|valref| valref.get_annotation().clone().unwrap()))
    }
}
