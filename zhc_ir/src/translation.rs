//! Driver-based IR-to-IR translation framework.
//!
//! Provides two translation strategies — **eager** and **lazy** — each in plain
//! and annotation-aware variants, forming a 2×2 family of translators:
//!
//! |           |        [`IR`]       |         [`AnnIR`]       |
//! |-----------|---------------------|-------------------------|
//! | **Eager** | [`eager_translate`] | [`eager_translate_ann`] |
//! | **Lazy**  | [`lazy_translate`]  | [`lazy_translate_ann`]  |
//!
//! Both strategies are callback-driven: the caller supplies a *driver* closure
//! that receives each source operation and a translator handle, and is
//! responsible for emitting corresponding output-dialect operations and
//! registering value mappings.
//!
//! **Eager** translation visits every operation in insertion order. The driver
//! is called exactly once per operation regardless of reachability.
//!
//! **Lazy** translation is demand-driven: only effect operations (zero return
//! values) are visited as roots. Dependencies are pulled in recursively when
//! the driver calls [`LazyTranslator::translate_val`] on a not-yet-translated
//! value. Operations unreachable from any effect are never visited.

use crate::{AnnIR, AnnOpRef, AnnValRef, Annotation, Dialect, IR, OpRef, ValId, ValMap, ValRef};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use zhc_utils::{
    iter::{CollectInSmallVec, MultiZip},
    small::SmallVec,
};

/// Mutable translation state for eager dialect-to-dialect IR translation.
///
/// Passed to the driver callback by [`eager_translate`] for each operation in
/// linear order. The driver uses this handle to look up already-translated
/// values, emit operations in the output dialect, and register value
/// correspondences between the source and output IRs.
pub struct EagerTranslator<ID: Dialect, OD: Dialect> {
    output: IR<OD>,
    valmap: ValMap<ValId>,
    phantom: PhantomData<ID>,
}

impl<ID: Dialect, OD: Dialect> EagerTranslator<ID, OD> {
    /// Returns the output-dialect [`ValId`] corresponding to `old`.
    ///
    /// # Panics
    ///
    /// Panics if no translation has been registered for `old`.
    pub fn translate_val<'a>(&self, old: ValId) -> ValId {
        self.valmap.get(&old).unwrap().clone()
    }

    /// Emits an operation in the output [`IR`] and returns the [`ValId`]s of
    /// the newly created return values.
    ///
    /// Delegates to [`IR::add_op`]; `instr` provides the instruction whose
    /// signature determines the number and types of return values. The `args`
    /// must be output-dialect [`ValId`]s obtained from prior calls to
    /// [`translate_val`](Self::translate_val) or [`add_op`](Self::add_op).
    pub fn add_op(&mut self, instr: OD::InstructionSet, args: SmallVec<ValId>) -> SmallVec<ValId> {
        self.output.add_op(instr, args).1
    }

    /// Records a mapping from source value `old` to output value `new`.
    ///
    /// # Panics
    ///
    /// Panics if a translation has already been registered for `old`.
    pub fn register_translation(&mut self, old: ValId, new: ValId) {
        assert!(
            self.valmap.insert(old, new).is_none(),
            "Tried to register a translation twice for {old}"
        );
    }
}

/// Translates an [`IR<ID>`] into an [`IR<OD>`] by visiting every operation in
/// insertion order.
///
/// The `driver` is invoked once per operation in `ir`. It receives an
/// [`OpRef`] into the source IR and a mutable [`EagerTranslator`] handle,
/// and must emit corresponding output operations and register all value
/// translations so that subsequent driver calls can resolve their arguments.
pub fn eager_translate<'a, ID: Dialect, OD: Dialect>(
    ir: &'a IR<ID>,
    driver: impl Fn(OpRef<'a, ID>, &mut EagerTranslator<ID, OD>),
) -> IR<OD> {
    let output = IR::empty();
    let valmap = ir.empty_valmap();
    let mut translator = EagerTranslator {
        output,
        valmap,
        phantom: PhantomData,
    };
    for op in ir.walk_ops_linear() {
        driver(op, &mut translator)
    }
    translator.output
}

/// Mutable translation state for eager translation of annotation-carrying IR.
///
/// Annotation-aware variant of [`EagerTranslator`]. Passed to the driver by
/// [`eager_translate_ann`], giving the driver access to per-operation and
/// per-value annotations through the [`AnnOpRef`] it receives.
pub struct AnnEagerTranslator<ID: Dialect, OpAnn: Annotation, ValAnn: Annotation, OD: Dialect> {
    output: IR<OD>,
    valmap: ValMap<ValId>,
    phantom: PhantomData<(ID, OpAnn, ValAnn)>,
}

impl<ID: Dialect, OpAnn: Annotation, ValAnn: Annotation, OD: Dialect>
    AnnEagerTranslator<ID, OpAnn, ValAnn, OD>
{
    /// Returns the output-dialect [`ValId`] corresponding to `old`.
    ///
    /// # Panics
    ///
    /// Panics if no translation has been registered for `old`.
    pub fn translate_val(&self, old: ValId) -> ValId {
        self.valmap.get(&old).unwrap().clone()
    }

    /// Performs a one-to-one operation translation.
    ///
    /// Translates every argument of `op` via
    /// [`translate_val`](Self::translate_val), emits a single output operation
    /// with instruction `instr` and those translated arguments, then registers
    /// the return-value correspondences between `op`'s returns and the new
    /// operation's returns.
    ///
    /// # Panics
    ///
    /// Panics if any argument of `op` lacks a registered translation, if the
    /// return arity of `instr` differs from that of `op`, or if any return
    /// value of `op` already has a registered translation.
    pub fn direct_translation<'a, 'b>(
        &mut self,
        op: AnnOpRef<'a, 'b, ID, OpAnn, ValAnn>,
        instr: OD::InstructionSet,
    ) {
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

    /// Emits an operation in the output [`IR`] and returns the [`ValId`]s of
    /// the newly created return values.
    ///
    /// Delegates to [`IR::add_op`]; see [`EagerTranslator::add_op`] for full
    /// contract.
    pub fn add_op(&mut self, instr: OD::InstructionSet, args: SmallVec<ValId>) -> SmallVec<ValId> {
        self.output.add_op(instr, args).1
    }

    /// Records a mapping from source value `old` to output value `new`.
    ///
    /// # Panics
    ///
    /// Panics if a translation has already been registered for `old`.
    pub fn register_translation(&mut self, old: ValId, new: ValId) {
        assert!(
            self.valmap.insert(old, new).is_none(),
            "Tried to register a translation twice for {old}"
        );
    }
}

/// Translates an [`AnnIR`] into an [`IR<OD>`] by visiting every operation in
/// insertion order.
///
/// Annotation-aware variant of [`eager_translate`]. The `driver` receives
/// [`AnnOpRef`]s carrying per-operation and per-value annotations, enabling
/// annotation-informed translation decisions.
pub fn eager_translate_ann<
    'a,
    'b,
    ID: Dialect,
    OpAnn: Annotation,
    ValAnn: Annotation,
    OD: Dialect,
>(
    ir: &'b AnnIR<'a, ID, OpAnn, ValAnn>,
    driver: impl Fn(AnnOpRef<'a, 'b, ID, OpAnn, ValAnn>, &mut AnnEagerTranslator<ID, OpAnn, ValAnn, OD>),
) -> IR<OD> {
    let output = IR::empty();
    let valmap = ir.empty_valmap();
    let mut translator = AnnEagerTranslator {
        output,
        valmap,
        phantom: PhantomData,
    };
    for op in ir.walk_ops_linear() {
        driver(op, &mut translator)
    }
    translator.output
}

/// Translation state for demand-driven dialect-to-dialect IR translation.
///
/// Unlike [`EagerTranslator`], methods take `&self` rather than `&mut self`.
/// Calling [`translate_val`](Self::translate_val) on a not-yet-translated
/// value triggers the driver on the producing operation, enabling recursive
/// demand-driven traversal of the source IR.
pub struct LazyTranslator<'a, ID: Dialect, OD: Dialect> {
    driver: Rc<dyn Fn(OpRef<'a, ID>, &LazyTranslator<'a, ID, OD>) + 'a>,
    output: Rc<RefCell<IR<OD>>>,
    valmap: Rc<RefCell<ValMap<ValId>>>,
}

impl<'a, ID: Dialect, OD: Dialect> LazyTranslator<'a, ID, OD> {
    /// Returns the output-dialect [`ValId`] for a source-dialect value.
    ///
    /// If `valref` has not yet been translated, invokes the driver on the
    /// operation that produces it, then looks up the result.
    ///
    /// # Panics
    ///
    /// Panics if the driver fails to register a translation for `valref`
    /// during its invocation.
    pub fn translate_val(&self, valref: ValRef<'a, ID>) -> ValId {
        if !self.valmap.borrow().contains_key(&*valref) {
            (self.driver)(valref.get_origin().opref, self);
        }
        self.valmap.borrow().get(&*valref).unwrap().clone()
    }

    fn ignite(&self, opref: OpRef<'a, ID>) {
        assert!(
            opref.is_effect(),
            "Tried to ignite translation on a non-effect op."
        );
        for arg in opref.get_args_iter() {
            self.translate_val(arg);
        }
        (self.driver)(opref, self)
    }

    /// Emits an operation in the output [`IR`] and returns the [`ValId`]s of
    /// the newly created return values.
    ///
    /// Delegates to [`IR::add_op`]; see [`EagerTranslator::add_op`] for full
    /// contract.
    pub fn add_op(&self, instr: OD::InstructionSet, args: SmallVec<ValId>) -> SmallVec<ValId> {
        self.output.borrow_mut().add_op(instr, args).1
    }

    /// Records a mapping from source value `old` to output value `new`.
    ///
    /// # Panics
    ///
    /// Panics if a translation has already been registered for `old`.
    pub fn register_translation(&self, old: ValId, new: ValId) {
        assert!(
            self.valmap.borrow_mut().insert(old, new).is_none(),
            "Tried to register a translation twice for {old}"
        );
    }
}

/// Translates an [`IR<ID>`] into an [`IR<OD>`] using demand-driven evaluation.
///
/// Only effect operations (zero return values) are visited as roots. Their
/// transitive dependencies are pulled in recursively as the driver calls
/// [`LazyTranslator::translate_val`]. Operations unreachable from any effect
/// are not translated and do not appear in the output.
pub fn lazy_translate<'a, ID: Dialect, OD: Dialect>(
    ir: &'a IR<ID>,
    driver: impl Fn(OpRef<'a, ID>, &LazyTranslator<'a, ID, OD>) + 'a,
) -> IR<OD> {
    let output = Rc::new(RefCell::new(IR::empty()));
    let valmap = Rc::new(RefCell::new(ir.empty_valmap()));
    let driver = Rc::new(driver);
    let translator = LazyTranslator {
        driver,
        output,
        valmap,
    };
    for effect in ir.walk_ops_linear().filter(|op| op.is_effect()) {
        translator.ignite(effect)
    }
    RefCell::into_inner(Rc::try_unwrap(translator.output).unwrap())
}

/// Translation state for demand-driven translation of annotation-carrying IR.
///
/// Annotation-aware variant of [`LazyTranslator`]. See that type for the
/// demand-driven evaluation model. The driver receives [`AnnOpRef`]s carrying
/// per-operation and per-value annotations.
pub struct AnnLazyTranslator<
    'a,
    'b,
    ID: Dialect,
    OpAnn: Annotation,
    ValAnn: Annotation,
    OD: Dialect,
> {
    driver: Rc<
        dyn Fn(
                AnnOpRef<'a, 'b, ID, OpAnn, ValAnn>,
                &AnnLazyTranslator<'a, 'b, ID, OpAnn, ValAnn, OD>,
            ) + 'a,
    >,
    output: Rc<RefCell<IR<OD>>>,
    valmap: Rc<RefCell<ValMap<ValId>>>,
}

impl<'a, 'b, ID: Dialect, OpAnn: Annotation, ValAnn: Annotation, OD: Dialect>
    AnnLazyTranslator<'a, 'b, ID, OpAnn, ValAnn, OD>
{
    /// Returns the output-dialect [`ValId`] for an annotated source-dialect
    /// value.
    ///
    /// If `valref` has not yet been translated, invokes the driver on the
    /// operation that produces it, then looks up the result.
    ///
    /// # Panics
    ///
    /// Panics if the driver fails to register a translation for `valref`
    /// during its invocation.
    pub fn translate_val(&self, valref: AnnValRef<'a, 'b, ID, OpAnn, ValAnn>) -> ValId {
        if !self.valmap.borrow().contains_key(&*valref) {
            (self.driver)(valref.get_origin().opref, self);
        }
        self.valmap.borrow().get(&*valref).unwrap().clone()
    }

    fn ignite(&self, opref: AnnOpRef<'a, 'b, ID, OpAnn, ValAnn>) {
        assert!(
            opref.is_effect(),
            "Tried to ignite translation on a non-effect op."
        );
        for arg in opref.get_args_iter() {
            self.translate_val(arg);
        }
        (self.driver)(opref, self)
    }

    /// Emits an operation in the output [`IR`] and returns the [`ValId`]s of
    /// the newly created return values.
    ///
    /// Delegates to [`IR::add_op`]; see [`EagerTranslator::add_op`] for full
    /// contract.
    pub fn push_new_op(&self, instr: OD::InstructionSet, args: SmallVec<ValId>) -> SmallVec<ValId> {
        self.output.borrow_mut().add_op(instr, args).1
    }

    /// Records a mapping from source value `old` to output value `new`.
    ///
    /// # Panics
    ///
    /// Panics if a translation has already been registered for `old`.
    pub fn register_translation(&self, old: ValId, new: ValId) {
        assert!(
            self.valmap.borrow_mut().insert(old, new).is_none(),
            "Tried to register a translation twice for {old}"
        );
    }
}

/// Translates an [`AnnIR`] into an [`IR<OD>`] using demand-driven evaluation.
///
/// Annotation-aware variant of [`lazy_translate`]. Only effect operations are
/// visited as roots; dependencies are pulled in recursively via
/// [`AnnLazyTranslator::translate_val`]. Operations unreachable from any
/// effect are not translated.
pub fn lazy_translate_ann<
    'a,
    'b,
    ID: Dialect,
    OpAnn: Annotation,
    ValAnn: Annotation,
    OD: Dialect,
>(
    ir: &'b AnnIR<'a, ID, OpAnn, ValAnn>,
    driver: impl Fn(
        AnnOpRef<'a, 'b, ID, OpAnn, ValAnn>,
        &AnnLazyTranslator<'a, 'b, ID, OpAnn, ValAnn, OD>,
    ) + 'a,
) -> IR<OD> {
    let output = Rc::new(RefCell::new(IR::empty()));
    let valmap = Rc::new(RefCell::new(ir.empty_valmap()));
    let driver = Rc::new(driver);
    let translator = AnnLazyTranslator {
        driver,
        output,
        valmap,
    };
    for effect in ir.walk_ops_linear().filter(|op| op.is_effect()) {
        translator.ignite(effect)
    }
    RefCell::into_inner(Rc::try_unwrap(translator.output).unwrap())
}
