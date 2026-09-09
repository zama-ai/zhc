//! IR interpreter for validating circuit correctness.
//!
//! The [`Interpreter`] provides a fluent interface for interpreting circuits without
//! actual FHE operations. This is useful for testing circuit correctness by comparing
//! computed outputs against expected values.

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
use zhc_crypto::integer_semantics::CiphertextBlockSpec;
use zhc_ir::{
    PrintWalker, ValId,
    visualization::{
        DynamicElement, NoClass, TextBox, VStack, VisualAnnotation, draw_ann_ir_to_html,
    },
};
use zhc_langs::ioplang::{IopInterepreterContext, IopValue};
use zhc_utils::{Dumpable, FastMap, files::FileHandle, small::SmallVec};

use crate::builder::InnerBuilder;

/// A fluent IR interpreter for testing circuit correctness.
///
/// Obtained via [`Builder::interpret()`](crate::Builder::interpret), the interpreter runs
/// the unoptimized IR graph with provided inputs and returns computed outputs. This
/// enables rapid validation without actual FHE execution.
///
/// # Example
///
/// ```rust,no_run
/// # use zhc_builder::*;
/// let builder = Builder::new(CiphertextBlockSpec(2, 2));
/// let a = builder.integer_ciphertext_input(8);
/// builder.integer_ciphertext_output(&a);
/// let outputs = builder.interpret()
///     .with_inputs(&[a.make_value(42)])
///     .get_outputs();
/// ```
pub struct Interpreter {
    pub(super) spec: CiphertextBlockSpec,
    pub(super) inputs: Vec<IopValue>,
    pub(super) inner: Rc<RefCell<InnerBuilder>>,
}

impl Interpreter {
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
        let mut context = IopInterepreterContext {
            spec: self.spec,
            inputs: self.inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::default(),
        };
        if let Err(interp_ir) = self.inner.borrow().ir.evaluate(&mut context) {
            panic!(
                "Failed to get outputs of interpretation:\n{}\nInterpretation panicked...",
                interp_ir.format()
            )
        };
        let mut output: Vec<_> = context.outputs.into_iter().collect();
        output.sort_unstable_by_key(|a| a.0);
        output.into_iter().map(|a| a.1).collect()
    }

    /// Interprets the IR and renders the result as an interactive HTML visualization.
    ///
    /// Unlike [`Builder::draw`](crate::Builder::draw) which shows the static graph structure,
    /// this method first runs the interpreter with the configured inputs, then renders a
    /// visualization annotated with the computed values at each node. Every operation displays
    /// its input and output values, making it easy to trace how data flows through the circuit
    /// during execution.
    ///
    /// Operations sharing the same comment hierarchy are grouped together visually. The
    /// resulting HTML file supports interactive features such as zooming and panning.
    ///
    /// The returned handle points at a freshly created temporary file, which can be
    /// displayed in the default browser with its `open` method.
    ///
    /// # Panics
    ///
    /// Panics if interpretation fails (e.g., due to a malformed graph or missing inputs), or
    /// if the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// # let a = builder.integer_ciphertext_input(8);
    /// # builder.integer_ciphertext_output(&a);
    /// builder.interpret()
    ///     .with_inputs(&[a.make_value(42)])
    ///     .draw()
    ///     .open()
    ///     .unwrap();
    /// ```
    pub fn draw(self) -> FileHandle {
        let mut context = IopInterepreterContext {
            spec: self.spec,
            inputs: self.inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::default(),
        };
        match self.inner.borrow().ir.evaluate(&mut context) {
            Ok(value_ir) => {
                #[derive(Debug, Clone, PartialEq, Eq)]
                struct InterpretationAnnotation(SmallVec<(ValId, IopValue)>);
                impl VisualAnnotation for InterpretationAnnotation {
                    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
                        Some(Box::new(VStack::<TextBox<NoClass>, NoClass>::new(
                            None,
                            self.0
                                .iter()
                                .map(|a| TextBox::new(None, format!("{}: {:#?}", a.0, a.1)))
                                .collect(),
                        )))
                    }
                }
                let ann_ir = value_ir.map_opann(|op| {
                    InterpretationAnnotation(
                        op.get_args_iter()
                            .chain(op.get_returns_iter())
                            .map(|val| (val.get_id(), val.get_annotation().to_owned()))
                            .collect(),
                    )
                });
                draw_ann_ir_to_html(
                    &ann_ir.view(),
                    Some(
                        Ref::map(self.inner.borrow(), |inner| &inner.ir).partially_mapped_opmap(
                            |op| self.inner.borrow().hierarchies.get(*op).cloned(),
                        ),
                    ),
                )
            }
            Err(interp_ir) => panic!(
                "Failed to get outputs of interpretation:\n{}\nInterpretation panicked...",
                interp_ir.format()
            ),
        }
    }
}

impl Dumpable for Interpreter {
    fn dump_to_string(&self) -> String {
        let mut context = IopInterepreterContext {
            spec: self.spec,
            inputs: self.inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::default(),
        };
        let ir = &self.inner.borrow().ir;

        match ir.evaluate(&mut context) {
            Ok(value_ir) => {
                format!("╔══════════════════════════════════════════════════════════════════════════════
║ Interpretation for : {}
║──────────────────────────────────────────────────────────────────────────────
{}
╚══════════════════════════════════════════════════════════════════════════════",
                    self.inputs.dump_to_string(),
                    value_ir.format().with_prefix("║ ").with_walker(PrintWalker::Linear).show_val_ann_alternate(true),
                )
            }
            Err(interp_ir) => {
                format!("╔══════════════════════════════════════════════════════════════════════════════
║ Interpretation for : {}
║──────────────────────────────────────────────────────────────────────────────
{}
╚══════════════════════════════════════════════════════════════════════════════",
                    self.inputs.dump_to_string(),
                    interp_ir.format().with_prefix("║ ").with_walker(PrintWalker::Linear).show_val_ann_alternate(true),
                )
            }
        }
    }
}
