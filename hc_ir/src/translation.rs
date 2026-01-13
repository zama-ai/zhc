use crate::{Dialect, IR};

/// Translates IR from one dialect to another.
///
/// This trait enables conversion between different IR dialects, allowing
/// transformation pipelines that operate on different representations
/// while maintaining semantic equivalence.
pub trait Translator {
    /// The source dialect to translate from.
    type InputDialect: Dialect;
    /// The target dialect to translate to.
    type OutputDialect: Dialect;

    /// Translates an IR from the input dialect to the output dialect.
    ///
    /// The resulting IR should be semantically equivalent to the input,
    /// but expressed using the types and operations of the output dialect.
    fn translate(&mut self, input: &IR<Self::InputDialect>) -> IR<Self::OutputDialect>;
}
