//! IR interpretation framework.
//!
//! Provides traits and utilities for defining and executing interpretations of
//! dialect operations. An interpretation assigns semantic values to SSA values,
//! propagating them through the IR via forward dataflow analysis.
//!
//! The framework is structured around three traits:
//! - [`Interpretation`] defines the semantic domain (what values look like)
//! - [`Interprets`] connects dialect types to interpretation values (type semantics)
//! - [`Interpretable`] defines how operations compute on interpretation values
//!
//! The main entry point is [`interpret_ir`], which executes an interpretation
//! over an entire IR, producing an annotated IR with interpretation values
//! attached to each SSA value.

use hc_utils::{
    iter::{CollectInSmallVec, MultiZip},
    small::SmallVec,
};

use crate::{AnnIR, Annotation, Dialect, DialectInstructionSet, DialectTypeSystem, IR, ValMap};

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
    type Context;

    /// Executes the operation on the given arguments.
    fn interpret(&self, context: &mut Self::Context, arguments: SmallVec<I>) -> SmallVec<I>;
}

/// Interprets an IR, returning an annotated IR with interpretation values.
///
/// # Panics
///
/// Panics if type checking fails for any operation's arguments or returns.
pub fn interpret_ir<'ir, D: Dialect, V: Interpretation>(
    ir: &'ir IR<D>,
    context: &mut <D::InstructionSet as Interpretable<V>>::Context,
) -> AnnIR<'ir, D, (), V>
where
    D::InstructionSet: Interpretable<V>,
    D::TypeSystem: InterpretsTo<V>,
{
    ir.forward_dataflow_analysis(|_, valmap: &ValMap<V>, opref| {
        let sig = opref.get_operation().get_signature();

        // Retrieve the arguments
        let arguments = opref
            .get_arg_valids()
            .iter()
            .map(|valid| valmap.get(valid).unwrap().clone())
            .cosvec();

        // Typecheck the arguments
        for (i, (arg, expected_type)) in
            (arguments.iter(), sig.get_args().iter()).mzip().enumerate()
        {
            if !expected_type.is_inhabited_by(arg) {
                panic!(
                    "Unexpected argument type encountered while interpreting {opref}. \
                     At position {i}, expected type {expected_type}, but encountered {}.",
                    D::TypeSystem::type_of(arg)
                )
            }
        }

        // Interpret
        let returns = opref.get_operation().interpret(context, arguments);

        // Typechecks the returns
        for (i, (ret, expected_type)) in (returns.iter(), sig.get_returns().iter())
            .mzip()
            .enumerate()
        {
            if !expected_type.is_inhabited_by(ret) {
                panic!(
                    "Unexpected return type encountered while interpreting {opref}. \
                     At position {i}, expected type {expected_type}, but encountered {}.",
                    D::TypeSystem::type_of(ret)
                )
            }
        }

        // Returns
        ((), returns)
    })
}
