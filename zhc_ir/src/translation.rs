//! Driver-based IR-to-IR translation framework.
//!
//! Provides a callback-driven translation mechanism with configurable traversal
//! order. The caller supplies:
//! - An [`Order`] specifying how operations should be visited
//! - A *driver* closure that receives each source operation and a translator handle, responsible
//!   for emitting output-dialect operations and registering value mappings
//!
//! |       |        [`IR`]       |         [`AnnIR`]       |
//! |-------|---------------------|-------------------------|
//! | Plain | [`translate`]       | [`translate_ann`]       |

use crate::{
    AnnIR, AnnOpRef, Annotation, AsOpId, AsValId, Dialect, IR, OpId, OpMap, OpRef, State, ValId,
    ValMap,
};
use std::{marker::PhantomData, ops::Index};
use zhc_utils::{
    SafeAs,
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

pub struct TranslatedFrom(pub OpId);

pub struct OpProvenanceMap(Vec<TranslatedFrom>);

impl OpProvenanceMap {
    pub fn project_opmap<T: Clone>(&self, opmap: &OpMap<T>) -> OpMap<T> {
        let store = self
            .0
            .iter()
            .map(|from| State::Active(opmap.get(from.0).cloned()))
            .collect();
        OpMap {
            store,
            n_stored: self.0.len().sas(),
            n_inactive: 0,
        }
    }
}

impl<A: AsOpId> Index<A> for OpProvenanceMap {
    type Output = TranslatedFrom;

    fn index(&self, index: A) -> &Self::Output {
        &self.0[index.op_id().0 as usize]
    }
}

pub struct Translation<OD: Dialect> {
    pub output: IR<OD>,
    pub op_provenance_map: OpProvenanceMap,
}

/// Mutable translation state for dialect-to-dialect IR translation.
///
/// Passed to the driver callback by [`translate`] and [`translate_ann`]. The
/// driver uses this handle to look up already-translated values, emit
/// operations in the output dialect, and register value correspondences.
pub struct Translator<ID: Dialect, OD: Dialect> {
    pub output: IR<OD>,
    valmap: ValMap<ValId>,
    pub op_provenance_map: OpProvenanceMap,
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
        let (_, valids) = self.output.add_op(instr, args);
        self.op_provenance_map
            .0
            .push(TranslatedFrom(self.current.unwrap()));
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
        Translation {
            output: self.output,
            op_provenance_map: self.op_provenance_map,
        }
    }
}

/// Translates an [`IR<ID>`] into an [`IR<OD>`] by visiting operations in the
/// specified order.
///
/// The `driver` is invoked once per operation. It receives an [`OpRef`] into
/// the source IR and a mutable [`Translator`] handle, and must emit
/// corresponding output operations and register all value translations.
pub fn translate<'a, ID: Dialect, OD: Dialect>(
    ir: &'a IR<ID>,
    order: Order,
    driver: impl Fn(OpRef<'a, ID>, &mut Translator<ID, OD>),
) -> Translation<OD> {
    let output = IR::empty();
    let valmap = ir.empty_valmap();
    let op_provenance_map = OpProvenanceMap(Vec::new());
    let current = None;
    let mut translator = Translator {
        output,
        valmap,
        op_provenance_map,
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

/// Translates an [`AnnIR`] into an [`IR<OD>`] by visiting operations in the
/// specified order.
///
/// Annotation-aware variant of [`translate`]. The `driver` receives
/// [`AnnOpRef`]s carrying per-operation and per-value annotations.
pub fn translate_ann<'a, 'b, ID: Dialect, OpAnn: Annotation, ValAnn: Annotation, OD: Dialect>(
    ir: &'b AnnIR<'a, ID, OpAnn, ValAnn>,
    order: Order,
    driver: impl Fn(AnnOpRef<'a, 'b, ID, OpAnn, ValAnn>, &mut Translator<ID, OD>),
) -> Translation<OD> {
    let output = IR::empty();
    let valmap = ir.empty_valmap();
    let op_provenance_map = OpProvenanceMap(Vec::new());
    let current = None;
    let mut translator = Translator {
        output,
        valmap,
        op_provenance_map,
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
