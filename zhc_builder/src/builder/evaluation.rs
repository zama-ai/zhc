//! IR interpreter for validating circuit correctness.
//!
//! The [`Evaluator`] provides a fluent interface for interpreting circuits without
//! actual FHE operations. This is useful for testing circuit correctness by comparing
//! computed outputs against expected values.

use std::{cell::RefCell, rc::Rc};
use zhc_crypto::integer_semantics::CiphertextBlockSpec;
use zhc_ir::PrintWalker;
use zhc_langs::ioplang::{IopInterepreterContext, IopValue};
use zhc_utils::{Dumpable, FastMap};

use crate::builder::InnerBuilder;

/// A fluent IR interpreter for testing circuit correctness.
///
/// Obtained via [`Builder::eval()`](crate::Builder::eval), the evaluator runs the
/// unoptimized IR graph with provided inputs and returns computed outputs. This
/// enables rapid validation without actual FHE execution.
///
/// # Example
///
/// ```rust,no_run
/// # use zhc_builder::*;
/// let builder = Builder::new(CiphertextBlockSpec(2, 2));
/// let a = builder.ciphertext_input(8);
/// builder.ciphertext_output(&a);
/// let outputs = builder.eval()
///     .with_inputs(&[a.make_value(42)])
///     .get_outputs();
/// ```
pub struct Evaluator {
    pub(super) spec: CiphertextBlockSpec,
    pub(super) inputs: Vec<IopValue>,
    pub(super) inner: Rc<RefCell<InnerBuilder>>,
}

impl Evaluator {
    /// Sets the input values for interpretation.
    ///
    /// The inputs must match the circuit's declared input signature in order and length.
    pub fn with_inputs(mut self, inps: impl AsRef<[IopValue]>) -> Self {
        self.inputs = inps.as_ref().to_vec();
        self
    }

    /// Runs the interpreter and returns the computed output values.
    ///
    /// # Panics
    ///
    /// Panics if interpretation fails (e.g., due to a malformed graph or missing inputs).
    pub fn get_outputs(self) -> Vec<IopValue> {
        let context = IopInterepreterContext {
            spec: self.spec,
            inputs: self.inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::new(),
        };
        let context = match self.inner.borrow().ir.interpret(context) {
            Ok((_, context)) => context,
            Err((ann_ir, _)) => panic!(
                "Failed to get outputs of evaluations:\n{}\nEvaluation panicked...",
                ann_ir.format()
            ),
        };
        let mut output: Vec<_> = context.outputs.into_iter().collect();
        output.sort_unstable_by_key(|a| a.0);
        output.into_iter().map(|a| a.1).collect()
    }
}

impl Dumpable for Evaluator {
    fn dump_to_string(&self) -> String {
        let context = IopInterepreterContext {
            spec: self.spec,
            inputs: self.inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::new(),
        };
        let ir = &self.inner.borrow().ir;

        match ir.interpret(context) {
            Ok((ann_ir, _)) => {
                format!("╔══════════════════════════════════════════════════════════════════════════════
║ Evaluation for : {}
║──────────────────────────────────────────────────────────────────────────────
{}
║──────────────────────────────────────────────────────────────────────────────
║ Evaluation succeeeded 😃 !
╚══════════════════════════════════════════════════════════════════════════════",
                    self.inputs.dump_to_string(),
                    ann_ir.format().with_prefix("║ ").with_walker(PrintWalker::Linear).show_val_ann_alternate(true),
                )
            }
            Err((ann_ir, _)) => {
                format!("╔══════════════════════════════════════════════════════════════════════════════
║ Evaluation for : {}
║──────────────────────────────────────────────────────────────────────────────
{}
║──────────────────────────────────────────────────────────────────────────────
║ Evaluation failed 😭 ....
╚══════════════════════════════════════════════════════════════════════════════",
                    self.inputs.dump_to_string(),
                    ann_ir.format().with_prefix("║ ").with_walker(PrintWalker::Linear).show_val_ann_alternate(true),
                )
            }
        }
    }
}
