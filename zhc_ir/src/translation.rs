//! Driver-based IR-to-IR translation framework.
//!
//! Provides a callback-driven translation mechanism with configurable traversal
//! order. The caller supplies:
//! - An [`Order`] specifying how operations should be visited
//! - A *driver* closure that receives each source operation and a translator handle, responsible
//!   for emitting output-dialect operations and registering value mappings
//!
//! |       |        [`IR`]       |         [`AnnIRView`]    |
//! |-------|---------------------|--------------------------|
//! | Plain | [`translate`]       | [`translate_ann`]        |

use crate::{
    AnnIR, AnnIRView, AnnOpRef, Annotation, AsValId, Dialect, IR, OpId, OpMap, OpRef,
    State, ValId, ValMap,
};
use std::marker::PhantomData;
use zhc_utils::{
    iter::{CollectInSmallVec, MultiZip},
    small::SmallVec,
};

/// Specifies the order in which operations are visited during translation.
///
/// The choice of traversal order affects which values are available when the driver is called
/// for a given operation. [`Linear`](Order::Linear) matches IR construction order,
/// [`Topological`](Order::Topological) guarantees dependencies are visited first, and
/// [`Custom`](Order::Custom) allows caller-controlled scheduling for advanced use cases
/// like batching.
pub enum Order {
    /// Visit operations in insertion order.
    Linear,
    /// Visit operations in topological order (dependencies before dependents).
    Topological,
    /// Visit operations in a caller-specified order.
    Custom(Vec<OpId>),
}


/// Maps operations and registered values in a [`Translation`]'s output IR back to their source.
///
/// Indexing with an output-IR operation ID yields the [`Provenance`] recording which source
/// operation produced it. Use [`project_opmap`](Self::project_opmap) to re-key an [`OpMap`]
/// indexed by source operations into one indexed by the corresponding output operations. Value
/// provenance is exposed through [`Translation::annotated_output`].
pub struct ProvenanceMap {
    operations: OpMap<OpId>,
    values: ValMap<ValId>,
}

impl ProvenanceMap {
    /// Re-keys `opmap`, indexed by source operations, into one indexed by output operations.
    ///
    /// For every operation in this translation's output IR, looks up the value `opmap` stores for
    /// the source operation it was translated from and clones it into the result at that
    /// position; if `opmap` has no value there, the resulting entry is left empty. The returned
    /// map preserves the active/inactive structure of the provenance map.
    pub fn project_opmap<T: Clone>(&self, opmap: &OpMap<T>) -> OpMap<T> {
        let mut n_stored = 0;
        let store = self
            .operations
            .store
            .iter()
            .map(|state| match state {
                State::Active(from) => {
                    let projected = from.as_ref().and_then(|from| opmap.get(from).cloned());
                    if projected.is_some() {
                        n_stored += 1;
                    }
                    State::Active(projected)
                }
                State::Inactive(_) => State::Inactive(None),
            })
            .collect();
        OpMap {
            store,
            n_stored,
            n_inactive: self.operations.n_inactive,
        }
    }

    /// Returns the output-operation to source-operation provenance map.
    pub fn operations(&self) -> &OpMap<OpId> {
        &self.operations
    }

    /// Returns the output-value to source-value provenance map.
    pub fn values(&self) -> &ValMap<ValId> {
        &self.values
    }
}

/// The result of translating an IR: the output IR together with its provenance.
///
/// Pairs the [`IR<OD>`] produced by [`translate`] or [`translate_ann`] with an
/// [`ProvenanceMap`] recording the source operations and registered source values from which its
/// contents were translated.
pub struct Translation<OD: Dialect> {
    /// The translated output IR.
    pub output: IR<OD>,
    /// Maps operations and registered values in `output` back to their source IDs.
    pub provenance_map: ProvenanceMap,
}

impl<OD: Dialect> Translation<OD> {
    /// Borrows the output IR with its source operation and value IDs as annotations.
    ///
    /// Operations and values created during the translation are annotated with the source ID
    /// registered for them. Elements added to [`output`](Self::output) after the translation, and
    /// output values for which the driver registered no translation, are annotated with `None`.
    pub fn annotated_output(&self) -> AnnIR<'_, OD, Option<OpId>, Option<ValId>> {
        let op_annotations = self.output.totally_mapped_opmap(|op| {
            if self.provenance_map.operations.may_store(&op) {
                self.provenance_map
                    .operations
                    .get(&op)
                    .map(|provenance| *provenance)
            } else {
                None
            }
        });
        let val_annotations = self.output.totally_mapped_valmap(|val| {
            if self.provenance_map.values.may_store(&val) {
                self.provenance_map.values.get(&val).copied()
            } else {
                None
            }
        });
        AnnIR::new(&self.output, op_annotations, val_annotations)
    }
}

/// Mutable translation state for dialect-to-dialect IR translation.
///
/// Passed to the driver callback by [`translate`] and [`translate_ann`]. The
/// driver uses this handle to look up already-translated values, emit
/// operations in the output dialect, and register value correspondences.
pub struct Translator<ID: Dialect, OD: Dialect> {
    pub output: IR<OD>,
    valmap: ValMap<ValId>,
    operation_provenance: Vec<Option<OpId>>,
    value_provenance: Vec<Option<ValId>>,
    current: Option<OpId>,
    phantom: PhantomData<ID>,
}

impl<ID: Dialect, OD: Dialect> Translator<ID, OD> {
    /// Returns the output-dialect [`ValId`] corresponding to `old`.
    ///
    /// # Panics
    ///
    /// Panics if no translation has been registered for `old`.
    pub fn translate_val(&self, old: impl AsValId) -> ValId {
        match self.valmap.get(old.val_id()) {
            Some(val) => val.clone(),
            None => panic!("Failed to translate val {}", old.val_id()),
        }
    }

    /// Emits an operation in the output [`IR`] and returns its newly created return values.
    ///
    /// The `args` must be output-dialect [`ValId`]s obtained from prior [`add_op`](Self::add_op)
    /// or [`translate_val`](Self::translate_val) calls. The number of returned values is
    /// determined by `instr`'s signature.
    pub fn add_op(&mut self, instr: OD::InstructionSet, args: SmallVec<ValId>) -> SmallVec<ValId> {
        let (opid, valids) = self.output.add_op(instr, args);
        self.operation_provenance
            .resize_with(opid.0 as usize + 1, || None);
        self.operation_provenance[opid.0 as usize] = Some(self.current.unwrap());
        if let Some(last) = valids.last() {
            self.value_provenance.resize(last.0 as usize + 1, None);
        }
        valids
    }

    /// Returns whether a translation has been registered for `old`.
    pub fn has_translation(&self, old: impl AsValId) -> bool {
        self.valmap.contains_key(old)
    }

    /// Records a mapping from source value `old` to output value `new`.
    ///
    /// # Panics
    ///
    /// Panics if a translation has already been registered for `old`.
    pub fn register_translation(&mut self, old: impl AsValId, new: impl AsValId) {
        let old = old.val_id();
        let new = new.val_id();
        assert!(
            self.valmap.insert(old, new).is_none(),
            "Tried to register a translation twice for {old}"
        );
        self.value_provenance.resize(new.0 as usize + 1, None);
        self.value_provenance[new.0 as usize] = Some(old);
    }

    /// Performs a one-to-one operation translation.
    ///
    /// Translates every argument of `op` via [`translate_val`](Self::translate_val),
    /// emits a single output operation with instruction `instr` and those
    /// translated arguments, then registers the return-value correspondences.
    ///
    /// # Panics
    ///
    /// Panics if any argument lacks a registered translation, if the return
    /// arity differs, or if any return value already has a translation.
    pub fn direct_translation<'a>(&mut self, op: &OpRef<'a, ID>, instr: OD::InstructionSet) {
        let new_args = op
            .get_arg_valids()
            .iter()
            .map(|v| self.translate_val(*v))
            .cosvec();
        let new_rets = self.add_op(instr, new_args);
        assert_eq!(new_rets.len(), op.get_return_arity());
        (new_rets.into_iter(), op.get_return_valids().iter())
            .mzip()
            .for_each(|(new, old)| self.register_translation(*old, new));
    }

    fn into_translation(self) -> Translation<OD> {
        let mut operations = self.output.empty_opmap();
        for op in self.output.walk_ops_linear() {
            if let Some(Some(provenance)) = self.operation_provenance.get(op.get_id().0 as usize) {
                operations.insert(&op, *provenance);
            }
        }
        let mut values = self.output.empty_valmap();
        for val in self.output.walk_vals_linear() {
            if let Some(Some(provenance)) = self.value_provenance.get(val.get_id().0 as usize) {
                values.insert(&val, *provenance);
            }
        }
        Translation {
            output: self.output,
            provenance_map: ProvenanceMap { operations, values },
        }
    }
}

/// Translates an [`IR<ID>`] into an [`IR<OD>`] by visiting operations in the specified order.
///
/// The `driver` is invoked once per operation. It receives an [`OpRef`] into the source IR and a
/// mutable [`Translator`] handle, and must emit corresponding output operations and register all
/// value translations. Returns a [`Translation`] pairing the output IR with an
/// [`ProvenanceMap`] tracing each output operation back to the source operation that produced
/// it.
pub fn translate<'a, ID: Dialect, OD: Dialect>(
    ir: &'a IR<ID>,
    order: Order,
    driver: impl Fn(OpRef<'a, ID>, &mut Translator<ID, OD>),
) -> Translation<OD> {
    let output = IR::empty();
    let valmap = ir.empty_valmap();
    let current = None;
    let mut translator = Translator {
        output,
        valmap,
        operation_provenance: Vec::new(),
        value_provenance: Vec::new(),
        current,
        phantom: PhantomData,
    };
    match order {
        Order::Linear => {
            for op in ir.walk_ops_linear() {
                translator.current = Some(op.get_id());
                driver(op, &mut translator);
            }
        }
        Order::Topological => {
            for op in ir.walk_ops_topological() {
                translator.current = Some(op.get_id());
                driver(op, &mut translator);
            }
        }
        Order::Custom(ids) => {
            for op in ir.walk_ops_with(ids.into_iter()) {
                translator.current = Some(op.get_id());
                driver(op, &mut translator);
            }
        }
    }
    translator.into_translation()
}

/// Translates an [`AnnIRView`] into an [`IR<OD>`] by visiting operations in the specified order.
///
/// Annotation-aware variant of [`translate`]. The `driver` receives [`AnnOpRef`]s carrying
/// per-operation and per-value annotations. Returns a [`Translation`] pairing the output IR with
/// an [`ProvenanceMap`] tracing each output operation back to the source operation that
/// produced it.
pub fn translate_ann<'a, 'b, ID: Dialect, OpAnn: Annotation, ValAnn: Annotation, OD: Dialect>(
    ir: AnnIRView<'a, 'b, ID, OpAnn, ValAnn>,
    order: Order,
    driver: impl Fn(AnnOpRef<'a, 'b, ID, OpAnn, ValAnn>, &mut Translator<ID, OD>),
) -> Translation<OD> {
    let output = IR::with_capacity(ir.n_vals(), ir.n_ops());
    let valmap = ir.empty_valmap();
    let current = None;
    let mut translator = Translator {
        output,
        valmap,
        operation_provenance: Vec::new(),
        value_provenance: Vec::new(),
        current,
        phantom: PhantomData,
    };
    match order {
        Order::Linear => {
            for op in ir.walk_ops_linear() {
                translator.current = Some(op.get_id());
                driver(op, &mut translator);
            }
        }
        Order::Topological => {
            for op in ir.walk_ops_topological() {
                translator.current = Some(op.get_id());
                driver(op, &mut translator);
            }
        }
        Order::Custom(ids) => {
            for op in ir.walk_ops_with(ids.into_iter()) {
                translator.current = Some(op.get_id());
                driver(op, &mut translator);
            }
        }
    }
    translator.into_translation()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testlang::{TestInstructionSet, TestLang};
    use zhc_utils::svec;

    #[test]
    fn annotated_output_tracks_operation_and_value_provenance() {
        let mut source = IR::<TestLang>::empty();
        let (input_op, input_vals) =
            source.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (inc_op, inc_vals) = source.add_op(TestInstructionSet::Inc, svec![input_vals[0]]);

        let mut translation: Translation<TestLang> =
            translate(&source, Order::Linear, |op, translator| {
                match op.get_instruction() {
                    TestInstructionSet::IntInput { pos } => {
                        translator
                            .output
                            .add_op(TestInstructionSet::BoolConstant { val: true }, svec![]);
                        let translated =
                            translator.add_op(TestInstructionSet::IntInput { pos: *pos }, svec![]);
                        translator.register_translation(op.get_return_valids()[0], translated[0]);
                    }
                    instruction => translator.direct_translation(&op, instruction.clone()),
                }
            });

        let translated_inc = translation
            .output
            .walk_vals_linear()
            .last()
            .unwrap()
            .get_id();
        translation
            .output
            .add_op(TestInstructionSet::Inc, svec![translated_inc]);

        let annotated = translation.annotated_output();
        let op_annotations: Vec<_> = annotated
            .walk_ops_linear()
            .map(|op| *op.get_annotation())
            .collect();
        let val_annotations: Vec<_> = annotated
            .walk_vals_linear()
            .map(|val| *val.get_annotation())
            .collect();

        assert_eq!(op_annotations, [None, Some(input_op), Some(inc_op), None]);
        assert_eq!(
            val_annotations,
            [None, Some(input_vals[0]), Some(inc_vals[0]), None]
        );
    }
}
