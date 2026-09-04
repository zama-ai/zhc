//! Builder implementation for FHE circuit construction.
//!
//! This module provides [`Builder`], the primary interface for constructing FHE circuits.
//! See the [crate-level documentation](super) for an overview of the circuit model, radix
//! decomposition, and operation flavors.
//!
//! # Example
//!
//! ```rust,no_run
//! # use zhc_builder::*;
//! let builder = Builder::new(CiphertextBlockSpec(2, 2));
//! let input = builder.ciphertext_input(8);
//! let blocks = builder.ciphertext_split(&input);
//! let doubled: Vec<_> = blocks.iter().map(|b| builder.block_add(b, b)).collect();
//! let output = builder.ciphertext_join(&doubled, None);
//! builder.ciphertext_output(&output);
//! let ir = builder.optimize_ir();
//! ```

use crate::{
    Interpreter,
    builder::{Ciphertext, CiphertextBlock, Plaintext, PlaintextBlock},
};
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    iter::repeat_n,
    rc::Rc,
};
use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, CiphertextSpec, Flavor, PlaintextBlockSpec, PlaintextSpec,
    lut::LookupCheck,
};
use zhc_ir::{
    AnnIR, IR, OpId, OpMap, PrintWalker, Signature, ValId,
    cse::eliminate_common_subexpressions,
    dce::eliminate_dead_code,
    partition::{PartitionId, PartitionIdRaw, PartitionTable},
    visualization::{Hierarchy, draw_ann_ir_to_html, draw_ir_to_html},
};
use zhc_langs::ioplang::{
    IopInstructionSet, IopLang, IopTypeSystem, IopValue, Lut1Def, Lut2Def, Lut4Def, Lut8Def,
    eliminate_aliases, skip_redundant_stores, skip_store_load,
};
use zhc_utils::{
    Dumpable, FastSet, SafeAs, Store,
    files::FileHandle,
    iter::{Chunk, ChunkIt},
    small::SmallVec,
    svec,
};

/// A circuit I/O type, either encrypted or plaintext.
///
/// [`Type`] is used in [`Signature`] to describe the types of a circuit's inputs and
/// outputs. Each variant carries the corresponding specification that fully describes the
/// integer's bit-width and per-block layout.
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    /// An encrypted integer with the given [`CiphertextSpec`].
    Ciphertext(CiphertextSpec),
    /// A plaintext integer with the given [`PlaintextSpec`].
    Plaintext(PlaintextSpec),
}

impl Type {
    /// Generates a random [`IopValue`] conforming to this type's specification.
    ///
    /// Useful for fuzz-testing circuits by generating randomized inputs that respect the
    /// declared bit-widths and block layouts.
    pub fn random_value(&self) -> IopValue {
        match self {
            Type::Ciphertext(spec) => IopValue::Ciphertext(spec.random()),
            Type::Plaintext(spec) => IopValue::Plaintext(spec.random()),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Ciphertext(spec) => write!(
                f,
                "Ciphertext<{}, {}, {}>",
                spec.int_size(),
                spec.block_spec().carry_size(),
                spec.block_spec().message_size()
            ),
            Type::Plaintext(spec) => write!(
                f,
                "Plaintext<{}, {}>",
                spec.int_size(),
                spec.block_spec().message_size()
            ),
        }
    }
}

/// What kind of IR to work on.
///
/// Passed as argument to the `draw*` and `partition*` methods in order to specify which IR must be
/// used.
pub enum IrKind {
    /// Use the original IR, as built by the builder.
    Original,
    /// Use the optimized IR, notably after dead code elimination.
    Optimized,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InnerBuilder {
    pub(super) ir: IR<IopLang>,
    pub(super) hierarchies: Store<OpId, Hierarchy>,
    pub(super) partitions: Store<OpId, PartitionId>,
    pub(super) sig: Signature<Type>,
}

impl InnerBuilder {
    fn insert_op(
        &mut self,
        op: IopInstructionSet,
        args: SmallVec<zhc_ir::ValId>,
        hierarchy: Hierarchy,
        partition: PartitionId,
    ) -> (zhc_ir::OpId, SmallVec<zhc_ir::ValId>) {
        if !hierarchy.is_root() {
            let (opid, rets) = self.ir.add_op_with_comment(op, args, hierarchy.to_string());
            self.hierarchies.push(hierarchy);
            self.partitions.push(partition);
            (opid, rets)
        } else {
            let (opid, rets) = self.ir.add_op(op, args);
            self.hierarchies.push(hierarchy);
            self.partitions.push(partition);
            (opid, rets)
        }
    }

    fn push_arg_type(&mut self, typ: Type) -> usize {
        self.sig.push_arg(typ);
        self.sig.get_args_arity() - 1
    }

    fn push_ret_type(&mut self, typ: Type) -> usize {
        self.sig.push_ret(typ);
        self.sig.get_returns_arity() - 1
    }
}

/// High-level builder for constructing FHE circuits as IR graphs.
///
/// A [`Builder`] accumulates IR instructions through its methods, using interior mutability
/// so that all operations take `&self`. The typical lifecycle is: create a builder, declare
/// inputs, emit block-level or vector-level operations, declare outputs, and finally call
/// [`optimize_ir`](Self::optimize_ir) to obtain the optimized IR.
///
/// Every builder is parameterized by a single [`CiphertextBlockSpec`] that defines the
/// message/carry bit layout shared by all ciphertext blocks in the circuit. This spec is
/// set at construction time and accessible via [`spec`](Self::spec).
///
/// # Input / Output Ordering
///
/// Inputs and outputs are **positional**: they are recorded in the order they are
/// declared. The first call to [`ciphertext_input`](Self::ciphertext_input)
/// or [`plaintext_input`](Self::plaintext_input) becomes input 0, the
/// second becomes input 1, and so on — both kinds share the same index space. Likewise,
/// the first [`ciphertext_output`](Self::ciphertext_output) becomes
/// output 0. This ordering defines the circuit's [`signature`](Self::signature) and must
/// match the order of values passed to [`Interpreter::with_inputs`].
///
/// # Comments
///
/// The builder maintains a comment stack that annotates IR instructions for debugging and
/// readability. When the stack is non-empty, every emitted instruction is tagged with the
/// full stack joined by ` / `. Use [`with_comment`](Self::with_comment) for scoped
/// annotations, or [`push_comment`](Self::push_comment) /
/// [`pop_comment`](Self::pop_comment) for manual control. Comments nest naturally: a
/// comment pushed inside a [`with_comment`](Self::with_comment) closure appends to the
/// existing stack.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::*;
/// let builder = Builder::new(CiphertextBlockSpec(2, 2));
/// let input = builder.ciphertext_input(8);
/// let blocks = builder.ciphertext_split(&input);
/// // ... operate on blocks ...
/// let output = builder.ciphertext_join(&blocks, None);
/// builder.ciphertext_output(&output);
/// let ir = builder.optimize_ir();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Builder {
    spec: CiphertextBlockSpec,
    inner: Rc<RefCell<InnerBuilder>>,
    hierarchy: RefCell<Hierarchy>,
    partition: RefCell<PartitionId>,
}

impl Builder {
    fn inner(&self) -> Ref<'_, InnerBuilder> {
        self.inner.borrow()
    }

    fn inner_mut(&self) -> RefMut<'_, InnerBuilder> {
        self.inner.borrow_mut()
    }

    fn current_hierarchy(&self) -> Hierarchy {
        self.hierarchy.borrow().clone()
    }

    fn current_partition(&self) -> PartitionId {
        self.partition.borrow().clone()
    }

    /// Creates a new builder with the given block specification.
    ///
    /// The `spec` defines the message and carry bit sizes for every ciphertext block
    /// produced by this builder. The builder starts with an empty IR and no declared
    /// inputs or outputs.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// ```
    pub fn new(spec: CiphertextBlockSpec) -> Self {
        Self {
            spec,
            inner: Rc::new(RefCell::new(InnerBuilder {
                ir: IR::empty(),
                hierarchies: Store::empty(),
                partitions: Store::empty(),
                sig: Signature::empty(),
            })),
            hierarchy: RefCell::new(Hierarchy::new()),
            partition: RefCell::new(PartitionId::new(0, "Inputs")),
        }
    }

    pub fn optimize_ir(&self) -> IR<IopLang> {
        let mut ir = self.ir().clone();
        eliminate_aliases(&mut ir);
        skip_store_load(&mut ir);
        eliminate_dead_code(&mut ir);
        skip_redundant_stores(&mut ir);
        eliminate_dead_code(&mut ir);
        eliminate_common_subexpressions(&mut ir);
        ir
    }

    /// Returns the block specification shared by all ciphertext blocks in this circuit.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.spec
    }

    /// Returns a clone of the circuit's current I/O signature.
    ///
    /// The signature records every input and output declared so far, in declaration order,
    /// as [`Type`] values.
    pub fn signature(&self) -> Signature<Type> {
        self.inner().sig.clone()
    }

    /// Borrows the current (unoptimized) IR graph.
    ///
    /// Unlike [`optimize_ir`](Self::optimize_ir), this does not consume the builder and does not
    /// apply any optimization passes. Useful for debugging and inspection mid-construction.
    pub fn ir(&self) -> Ref<'_, IR<IopLang>> {
        Ref::map(self.inner(), |inner| &inner.ir)
    }

    /// Returns the hierarchy annotations for all operations.
    ///
    /// The returned [`OpMap`] associates each operation with its [`Hierarchy`], derived from the
    /// comment stack active when the operation was emitted. Visualization functions use this to
    /// group related operations together.
    pub fn hierarchy(&self) -> OpMap<Hierarchy> {
        self.ir()
            .partially_mapped_opmap(|op| self.inner().hierarchies.get(*op).cloned())
    }

    /// Creates an interpreter for this circuit.
    ///
    /// Returns an [`Interpreter`] that can be configured with inputs and run to compute
    /// outputs. The interpreter uses the unoptimized IR graph for interpretation.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let a = builder.ciphertext_input(8);
    /// builder.ciphertext_output(&a);
    /// let outputs = builder.interpret()
    ///     .with_inputs(&[a.make_value(42)])
    ///     .get_outputs();
    /// ```
    pub fn interpret(&self) -> Interpreter {
        Interpreter {
            inputs: vec![],
            inner: self.inner.clone(),
            spec: self.spec,
        }
    }

    /// Runs randomized correctness tests against a reference implementation.
    ///
    /// Generates `reps` random inputs matching the circuit's signature and compares
    /// the circuit's outputs against the expected values from `gen_expect`. If
    /// `gen_expect` returns `None`, the test case is skipped (useful for filtering
    /// invalid input combinations).
    ///
    /// # Panics
    ///
    /// Panics if any test case produces outputs that don't match expectations,
    /// dumping the failing interpretation for debugging.
    pub fn test_random(
        &self,
        reps: usize,
        gen_expect: impl Fn(&[IopValue]) -> Option<Vec<IopValue>>,
    ) {
        use zhc_utils::iter::CollectInSmallVec;
        for _ in 0..reps {
            use std::panic::AssertUnwindSafe;
            let inputs = self
                .signature()
                .get_args()
                .iter()
                .map(|a| a.random_value())
                .cosvec();
            if let Some(expectations) = gen_expect(inputs.as_slice()) {
                let outputs = match std::panic::catch_unwind(AssertUnwindSafe(|| {
                    self.interpret().with_inputs(&inputs).get_outputs()
                })) {
                    Ok(outputs) => outputs,
                    Err(_) => {
                        self.interpret().with_inputs(&inputs).dump_and_panic();
                    }
                };
                if false {
                    println!(
                        "Input {:?}:\nExpected:\n{:?}\nOutput:\n{:?}",
                        inputs, expectations, outputs
                    );
                }
                if expectations != outputs {
                    println!(
                        "Random test failed for input {:#?}:\nExpected:\n{:#?}\nOutput:\n{:#?}",
                        inputs, expectations, outputs
                    );
                    self.interpret().with_inputs(inputs).dump_and_panic();
                }
            }
        }
    }

    /// Renders the IR as an interactive HTML visualization.
    ///
    /// The visualization displays the IR as an SVG graph where operations appear as nodes and
    /// data dependencies as edges. Operations sharing the same comment hierarchy are grouped
    /// together visually, making the logical structure of the program easier to follow. The
    /// resulting HTML file supports interactive features such as zooming and panning.
    ///
    /// This is primarily a debugging and exploration tool — call it at any point during
    /// construction to inspect the current state of the IR. The returned handle points at a
    /// freshly created temporary file, which can be displayed in the default browser with its
    /// `open` method.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// # let ct = builder.ciphertext_input(4);
    /// builder.draw(IrKind::Original).open().unwrap();
    /// ```
    pub fn draw(&self, kind: IrKind) -> FileHandle {
        let ir = match kind {
            IrKind::Original => &self.ir(),
            IrKind::Optimized => &self.optimize_ir(),
        };
        draw_ir_to_html(
            ir,
            Some(
                self.ir()
                    .partially_mapped_opmap(|op| self.inner().hierarchies.get(*op).cloned()),
            ),
        )
    }

    /// Renders the given IR as an interactive HTML visualization, coloured by partition.
    ///
    /// This behaves like [`draw`](Self::draw) — an SVG graph of operations and data
    /// dependencies grouped by comment hierarchy — but additionally shades each operation
    /// according to the partition it belongs to, giving a quick visual read on how the program
    /// is split into units of computation. The returned handle points at a freshly created
    /// temporary file, which can be displayed in the default browser with its `open` method.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// # let ct = builder.ciphertext_input(4);
    /// builder.draw_partitions(IrKind::Original).open().unwrap();
    /// ```
    pub fn draw_partitions(&self, kind: IrKind) -> FileHandle {
        let ir = match kind {
            IrKind::Original => &self.ir(),
            IrKind::Optimized => &self.optimize_ir(),
        };
        let ann_ir = AnnIR::new(ir, self.partitions(kind), ir.filled_valmap(()));
        draw_ann_ir_to_html(
            &ann_ir.view(),
            Some(
                self.ir()
                    .partially_mapped_opmap(|op| self.inner().hierarchies.get(*op).cloned()),
            ),
        )
    }

    /// Returns a new builder handle with the given comment appended to the annotation stack.
    ///
    /// Unlike [`push_comment`](Self::push_comment) which mutates the current builder, this
    ///
    /// method returns a *new* [`Builder`] sharing the same underlying IR but with an
    /// independent comment stack containing the new comment. All instructions emitted
    /// through the returned builder are annotated with the full stack including the new
    /// comment; instructions emitted through the original builder remain unaffected. This
    /// is useful for forking annotation contexts without manual push/pop management.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let commented = builder.comment("add phase");
    /// let ct = commented.ciphertext_input(4);
    /// // Instructions through `commented` carry the "add phase" annotation.
    /// ```
    pub fn comment(&self, comment: impl Into<String>) -> Builder {
        let hierarchy = self.hierarchy.clone();
        hierarchy.borrow_mut().push(comment.into());
        Builder {
            spec: self.spec,
            inner: self.inner.clone(),
            hierarchy,
            partition: self.partition.clone(),
        }
    }

    /// Opens a new partition and makes it the current one.
    ///
    /// Every operation emitted through a builder is tagged with its current partition. Calling
    /// this allocates the next partition identity, labels it with `metadata` — anything
    /// convertible into a shared string, such as a `&str` or `String` — and makes it current, so
    /// that all subsequently emitted operations belong to it. The freshly opened
    /// [`PartitionId`] is returned for later reference. Use this to mark the boundary between
    /// successive units of computation while building a program.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let stage = builder.new_partition("Stage 1");
    /// // Operations built from here on belong to the "Stage 1" partition.
    /// let ct = builder.ciphertext_input(4);
    /// ```
    pub fn new_partition(&self, metadata: impl AsRef<str>) -> PartitionId {
        let mut partition = self.partition.borrow_mut();
        let new_partition_id = partition.id + 1;
        *partition = PartitionId::new(new_partition_id, metadata);
        partition.clone()
    }

    /// Looks up the partition with the given identity, if any operation carries it.
    ///
    /// Scans the partitions currently attached to the IR's operations and returns the one whose
    /// identity equals `id`, or `None` when no operation belongs to a partition with that
    /// identity. This recovers the full [`PartitionId`], including its label, from the bare
    /// numeric identity.
    ///
    /// # Panics
    ///
    /// Panics if more than one partition shares `id` with differing metadata, which would
    /// indicate a corrupted partition assignment.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let stage = builder.new_partition("Stage 1");
    /// let ct = builder.ciphertext_input(4);
    /// assert_eq!(builder.get_partition_by_id(stage.id), Some(stage));
    /// ```
    pub fn get_partition_by_id(&self, id: PartitionIdRaw) -> Option<PartitionId> {
        let mut part_set = self
            .partitions(IrKind::Original)
            .into_iter()
            .map(|p| p.1)
            .filter(|p| p.id == id)
            .collect::<FastSet<_>>();
        assert!(
            part_set.len() <= 1,
            "PartitionId shared same Id with differentes metadata"
        );

        part_set.drain().next()
    }

    /// Merges two partitions into one and re-tags every affected operation.
    ///
    /// Fuses `part_a` and `part_b` following [`PartitionId::fuse`] — the result keeps the lower
    /// identity and a combined label — then rewrites every operation belonging to either source
    /// partition so that it points at the fused one. The fused [`PartitionId`] is returned. Use
    /// this to coalesce two units of computation into a single coarser task.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let a = builder.new_partition("Stage 1");
    /// let b = builder.new_partition("Stage 2");
    /// let merged = builder.merge_partitions(a, b);
    /// ```
    pub fn merge_partitions(&self, part_a: PartitionId, part_b: PartitionId) -> PartitionId {
        let fused = PartitionId::fuse(&part_a, &part_b);

        self.inner_mut().partitions.iter_mut().for_each(|p| {
            if (*p == part_a) || (*p == part_b) {
                *p = fused.clone()
            }
        });
        fused
    }

    /// Merges every partition named by the given identities into a single one.
    ///
    /// Resolves each identity in `ids` to its partition and folds them together with
    /// [`merge_partitions`](Self::merge_partitions), rewriting the affected operations along the
    /// way. The resulting coarse partition is returned, or `None` when none of the identities
    /// matches a partition currently present in the IR. Use this to group several units of
    /// computation at once.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let a = builder.new_partition("Stage 1");
    /// let b = builder.new_partition("Stage 2");
    /// let grouped = builder.group_partitions_id([a.id, b.id]);
    /// ```
    pub fn group_partitions_id(&self, ids: impl AsRef<[PartitionIdRaw]>) -> Option<PartitionId> {
        ids.as_ref()
            .iter()
            .filter_map(|id| self.get_partition_by_id(*id))
            .reduce(|acc, p| self.merge_partitions(acc, p))
    }

    /// Returns the partition assigned to every operation of the given IR.
    ///
    /// Produces an [`OpMap`] associating each operation of `ir` with its [`PartitionId`],
    /// capturing the current partition assignment of the whole graph. This is the raw
    /// per-operation view; for a de-duplicated overview see
    /// [`partitions_table`](Self::partitions_table).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// # let ct = builder.ciphertext_input(4);
    /// let per_op = builder.partitions(IrKind::Original);
    /// ```
    pub fn partitions(&self, kind: IrKind) -> OpMap<PartitionId> {
        let ir = match kind {
            IrKind::Original => &self.ir(),
            IrKind::Optimized => &self.optimize_ir(),
        };
        ir.totally_mapped_opmap(|op| self.inner().partitions[*op].clone())
    }

    /// Returns the distinct partitions of the given IR as an inspection table.
    ///
    /// Collapses the per-operation assignment of `ir` into a [`PartitionTable`] — the sorted,
    /// de-duplicated set of partitions the graph is split into — which is convenient for
    /// inspecting the available units of computation at a glance.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// # let ct = builder.ciphertext_input(4);
    /// let table = builder.partitions_table(IrKind::Original);
    /// ```
    pub fn partitions_table(&self, kind: IrKind) -> PartitionTable {
        PartitionTable::from(
            self.partitions(kind)
                .into_iter()
                .map(|(_k, v)| v)
                .collect::<std::collections::BTreeSet<_>>(),
        )
    }

    /// Pushes a comment onto the annotation stack.
    ///
    /// All IR instructions emitted while this comment is on the stack will be annotated
    /// with the full stack joined by ` / `. Use [`pop_comment`](Self::pop_comment) to
    /// remove it, or prefer the RAII-style [`with_comment`](Self::with_comment).
    pub fn push_comment(&self, comment: impl Into<String>) {
        self.hierarchy.borrow_mut().push(comment.into());
    }

    /// Pops the most recent comment from the annotation stack.
    pub fn pop_comment(&self) {
        self.hierarchy.borrow_mut().pop();
    }

    /// Executes a closure with a temporary comment pushed onto the annotation stack.
    ///
    /// The comment is pushed before calling `f` and popped after it returns, ensuring
    /// proper nesting even if `f` itself pushes additional comments. Returns whatever
    /// `f` returns.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let result = builder.with_comment("carry propagation", || {
    ///     builder.block_add(&blocks[0], &blocks[1])
    /// });
    /// ```
    pub fn with_comment<R>(&self, comment: impl Into<String>, f: impl FnOnce() -> R) -> R {
        self.push_comment(comment);
        let result = f();
        self.pop_comment();
        result
    }

    /// Declares an encrypted integer input of the given bit-width.
    ///
    /// Registers a new ciphertext input in the circuit signature and emits the
    /// corresponding IR input instruction. The input is assigned the next positional index
    /// (see [Input / Output Ordering](Self#input--output-ordering)). The `int_size`
    /// specifies the total number of message bits across all blocks (e.g. 8 for an 8-bit
    /// integer). The resulting ciphertext is a radix-decomposed integer with
    /// `int_size / message_size` blocks.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let input = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&input);
    /// ```
    pub fn ciphertext_input(&self, int_size: u16) -> Ciphertext {
        let spec = self.spec.ciphertext_spec(int_size);
        let pos = self.inner_mut().push_arg_type(Type::Ciphertext(spec));
        let (_, inp) = self.inner_mut().insert_op(
            IopInstructionSet::InputCiphertext { pos, int_size },
            svec![],
            self.current_hierarchy(),
            self.current_partition(),
        );
        Ciphertext {
            valid: inp[0],
            spec,
        }
    }

    /// Decomposes a [`Ciphertext`] into its individual radix blocks.
    ///
    /// Returns one [`CiphertextBlock`] per block in the radix-decomposed
    /// representation, ordered from least-significant to most-significant digit. The
    /// length of the returned vector is `int_size / message_size`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct);
    /// assert_eq!(blocks.len(), 4); // 8 bits / 2-bit message = 4 blocks
    /// ```
    pub fn ciphertext_split(&self, inp: impl AsRef<Ciphertext>) -> Vec<CiphertextBlock> {
        let inp = inp.as_ref();
        (0..inp.spec().block_count())
            .map(|index| {
                let (_, ret) = self.inner_mut().insert_op(
                    IopInstructionSet::ExtractCtBlock { index },
                    svec![inp.valid],
                    self.current_hierarchy(),
                    self.current_partition(),
                );
                CiphertextBlock {
                    valid: ret[0],
                    spec: self.spec,
                }
            })
            .collect()
    }

    /// Declares a plaintext integer input of the given bit-width.
    ///
    /// Registers a new plaintext input in the circuit signature and emits the
    /// corresponding IR input instruction. The input is assigned the next positional index,
    /// shared with ciphertext inputs
    /// (see [Input / Output Ordering](Self#input--output-ordering)). The plaintext block
    /// spec is derived from the builder's ciphertext block spec (matching message size, no
    /// carry bits).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let pt = builder.plaintext_input(8);
    /// let ct_blocks = builder.ciphertext_split(&ct);
    /// let pt_blocks = builder.plaintext_split(&pt);
    /// let sum = builder.block_add_plaintext(&ct_blocks[0], &pt_blocks[0]);
    /// ```
    pub fn plaintext_input(&self, int_size: u16) -> Plaintext {
        let spec = self
            .spec
            .matching_plaintext_block_spec()
            .plaintext_spec(int_size);
        let pos = self.inner_mut().push_arg_type(Type::Plaintext(spec));
        let (_, inp) = self.inner_mut().insert_op(
            IopInstructionSet::InputPlaintext { pos, int_size },
            svec![],
            self.current_hierarchy(),
            self.current_partition(),
        );
        Plaintext {
            valid: inp[0],
            spec,
        }
    }

    /// Decomposes a [`Plaintext`] into its individual radix blocks.
    ///
    /// Returns one [`PlaintextBlock`] per digit in the radix-decomposed
    /// representation, ordered from least-significant to most-significant digit. The
    /// length of the returned vector is `int_size / message_size`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let pt = builder.plaintext_input(8);
    /// let blocks = builder.plaintext_split(&pt);
    /// assert_eq!(blocks.len(), 4); // 8 bits / 2-bit message = 4 blocks
    /// ```
    pub fn plaintext_split(&self, inp: impl AsRef<Plaintext>) -> Vec<PlaintextBlock> {
        let inp = inp.as_ref();
        (0..inp.spec().block_count())
            .map(|index| {
                let (_, ret) = self.inner_mut().insert_op(
                    IopInstructionSet::ExtractPtBlock { index },
                    svec![inp.valid],
                    self.current_hierarchy(),
                    self.current_partition(),
                );
                PlaintextBlock {
                    valid: ret[0],
                    spec: self.spec.matching_plaintext_block_spec(),
                }
            })
            .collect()
    }

    /// Reassembles a slice of radix blocks into a single [`Ciphertext`].
    ///
    /// The blocks are stored in order, with block 0 as the least-significant radix block.
    /// When `int_size` is None, the total bit-width is inferred as
    /// `blocks.len() * message_size`. When `int_size` is `Some`, it overrides the
    /// bit-width explicitly. This is useful if the expected bit-width is not a multiple of
    /// the message size.
    ///
    /// # Panics
    ///
    /// Panics if `int_size` is `Some` and the number of blocks exceeds the
    /// capacity implied by the given bit-width.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let input = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&input);
    /// // ... operate on blocks ...
    /// let ct = builder.ciphertext_join(&blocks, None);
    /// builder.ciphertext_output(&ct);
    /// ```
    pub fn ciphertext_join(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        int_size: Option<u16>,
    ) -> Ciphertext {
        let blocks = blocks.as_ref();
        let int_size = match int_size {
            Some(int_size) => {
                let max_blocks_count = int_size.div_ceil(self.spec().message_size().sas::<u16>());
                if max_blocks_count < blocks.len().sas::<u16>() {
                    panic!(
                        "Tried to join ciphertext with specific int_size, but was given more blocks then expected. Expected {max_blocks_count}, found {}.",
                        blocks.len()
                    );
                }
                int_size
            }
            None => blocks.len().sas::<u16>() * self.spec.message_size().sas::<u16>(),
        };
        let spec = self.spec.ciphertext_spec(int_size);
        let (_, acc) = self.inner_mut().insert_op(
            IopInstructionSet::DeclareCiphertext { int_size },
            svec![],
            self.current_hierarchy(),
            self.current_partition(),
        );
        let (_, zero) = self.inner_mut().insert_op(
            IopInstructionSet::LetCiphertextBlock { value: 0 },
            svec![],
            self.current_hierarchy(),
            self.current_partition(),
        );
        let mut acc = acc[0];
        for index in 0..spec.block_count() {
            let index = index.sas::<u8>();
            let (_, ret) = self.inner_mut().insert_op(
                IopInstructionSet::StoreCtBlock { index },
                svec![zero[0], acc],
                self.current_hierarchy(),
                self.current_partition(),
            );
            acc = ret[0];
        }
        for (index, block) in blocks.iter().enumerate() {
            let index = index.sas::<u8>();
            let (_, ret) = self.inner_mut().insert_op(
                IopInstructionSet::StoreCtBlock { index },
                svec![block.valid, acc],
                self.current_hierarchy(),
                self.current_partition(),
            );
            acc = ret[0];
        }
        Ciphertext { valid: acc, spec }
    }

    /// Creates a new IR node that aliases an existing ciphertext.
    ///
    /// The returned ciphertext references the same underlying value but has a distinct IR
    /// node identity. This is useful for debugging, as the node appears separately in IR
    /// dumps and can be annotated with the current comment stack.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let input = builder.ciphertext_input(8);
    /// let labeled = builder.comment("after input").ciphertext_inspect(&input);
    /// ```
    pub fn ciphertext_inspect(&self, src: impl AsRef<Ciphertext>) -> Ciphertext {
        let src = src.as_ref();
        let (_node, ret) = self.inner_mut().insert_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::Ciphertext,
            },
            svec![src.valid],
            self.current_hierarchy(),
            self.current_partition(),
        );
        Ciphertext {
            valid: ret[0],
            spec: src.spec(),
        }
    }

    /// Declares an encrypted integer output for the circuit.
    ///
    /// Registers the ciphertext as a circuit output in the signature and emits the
    /// corresponding IR output instruction. The output is assigned the next positional
    /// index (see [Input / Output Ordering](Self#input--output-ordering)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let input = builder.ciphertext_input(8);
    /// builder.ciphertext_output(&input);
    /// ```
    pub fn ciphertext_output(&self, ct: impl AsRef<Ciphertext>) {
        let ct = ct.as_ref();
        let pos = self.inner_mut().push_ret_type(Type::Ciphertext(ct.spec()));
        self.inner_mut().insert_op(
            IopInstructionSet::OutputCiphertext { pos },
            svec![ct.valid],
            self.current_hierarchy(),
            self.current_partition(),
        );
    }

    /// Creates a constant [`PlaintextBlock`] with the given message value.
    ///
    /// The `value` is stored as a message-only plaintext block. Its bit-width must fit
    /// within the builder's complete block size.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let one = builder.block_let_plaintext(1);
    /// let incremented = builder.block_add_plaintext(&blocks[0], &one);
    /// ```
    pub fn block_let_plaintext(&self, value: u8) -> PlaintextBlock {
        let (_node, ret) = self.inner_mut().insert_op(
            IopInstructionSet::LetPlaintextBlock { value },
            svec![],
            self.current_hierarchy(),
            self.current_partition(),
        );
        PlaintextBlock {
            valid: ret[0],
            spec: PlaintextBlockSpec(self.spec.complete_size()),
        }
    }

    /// Creates a constant [`CiphertextBlock`] with the given value.
    ///
    /// The `value` is stored as a trivially-encrypted block (zero noise). This is useful
    /// for initializing accumulators or providing constant operands in arithmetic. The
    /// value spans the complete block width (padding, carry and message bits) and must
    /// fit within it.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let zero = builder.block_let_ciphertext(0);
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let sum = builder.block_add(&zero, &blocks[0]); // 0 + blocks[0]
    /// ```
    pub fn block_let_ciphertext(&self, value: u8) -> CiphertextBlock {
        self.emit_block(IopInstructionSet::LetCiphertextBlock { value }, svec![])
    }

    /// Creates a new IR node that aliases an existing ciphertext block.
    ///
    /// The returned block references the same underlying value but has a distinct IR
    /// node identity. This is useful for debugging purposes.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let labeled = builder.comment("lsb").block_inspect(&blocks[0]);
    /// ```
    pub fn block_inspect(&self, src: impl AsRef<CiphertextBlock>) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![src.as_ref().valid],
        )
    }

    /// Emits a block-returning instruction and wraps its single result.
    fn emit_block(&self, op: IopInstructionSet, args: SmallVec<ValId>) -> CiphertextBlock {
        let (_node, ret) = self.inner_mut().insert_op(
            op,
            args,
            self.current_hierarchy(),
            self.current_partition(),
        );
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Emits a block instruction returning `N` blocks and wraps its results.
    fn emit_blocks<const N: usize>(
        &self,
        op: IopInstructionSet,
        args: SmallVec<ValId>,
    ) -> [CiphertextBlock; N] {
        let (_node, ret) = self.inner_mut().insert_op(
            op,
            args,
            self.current_hierarchy(),
            self.current_partition(),
        );
        assert_eq!(ret.len(), N);
        std::array::from_fn(|i| CiphertextBlock {
            valid: ret[i],
            spec: self.spec,
        })
    }

    /// Adds two ciphertext blocks with the given flavor.
    ///
    /// Computes `src_a + src_b` at the block level. See
    /// [Operation Flavors](super::super#operation-flavors). Prefer the named shortcuts
    /// [`block_add`](Self::block_add), [`block_temper_add`](Self::block_temper_add) and
    /// [`block_wrapping_add`](Self::block_wrapping_add) when the flavor is fixed.
    pub fn block_add_with(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        flavor: Flavor,
    ) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::AddCt { flavor },
            svec![src_a.as_ref().valid, src_b.as_ref().valid],
        )
    }

    /// Adds two ciphertext blocks (protect flavor).
    ///
    /// Computes `src_a + src_b` at the block level. Uses protect semantics — see
    /// [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let sum = builder.block_add(&blocks[0], &blocks[1]);
    /// ```
    pub fn block_add(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_add_with(src_a, src_b, Flavor::Protect)
    }

    /// Adds two ciphertext blocks (temper flavor).
    ///
    /// Computes `src_a + src_b` at the block level. Uses temper semantics — see
    /// [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let sum = builder.block_temper_add(&blocks[0], &blocks[1]);
    /// ```
    pub fn block_temper_add(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_add_with(src_a, src_b, Flavor::Temper)
    }

    /// Adds two ciphertext blocks (wrapping flavor).
    ///
    /// Computes `src_a + src_b` at the block level. Uses wrapping semantics — see
    /// [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let sum = builder.block_wrapping_add(&blocks[0], &blocks[1]);
    /// ```
    pub fn block_wrapping_add(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_add_with(src_a, src_b, Flavor::Wrapping)
    }

    /// Subtracts two ciphertext blocks with the given flavor.
    ///
    /// Computes `src_a - src_b` at the block level. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_sub_with(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        flavor: Flavor,
    ) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::SubCt { flavor },
            svec![src_a.as_ref().valid, src_b.as_ref().valid],
        )
    }

    /// Subtracts two ciphertext blocks (protect flavor).
    ///
    /// Computes `src_a - src_b` at the block level. Uses protect semantics — see
    /// [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let diff = builder.block_sub(&blocks[1], &blocks[0]);
    /// ```
    pub fn block_sub(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_sub_with(src_a, src_b, Flavor::Protect)
    }

    /// Subtracts two ciphertext blocks (temper flavor).
    ///
    /// Computes `src_a - src_b` at the block level. Operand padding bits may be set, but
    /// the subtraction must not underflow. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_sub(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_sub_with(src_a, src_b, Flavor::Temper)
    }

    /// Subtracts two ciphertext blocks (wrapping flavor).
    ///
    /// Computes `src_a - src_b` at the block level. Uses wrapping semantics — see
    /// [Operation Flavors](super::super#operation-flavors). Underflow wraps
    /// modulo the complete block width instead of panicking.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let diff = builder.block_wrapping_sub(&blocks[0], &blocks[1]);
    /// ```
    pub fn block_wrapping_sub(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_sub_with(src_a, src_b, Flavor::Wrapping)
    }

    /// Shifts a ciphertext block left by `amount` bits with the given flavor.
    ///
    /// Computes `src << amount` at the block level. See
    /// [Operation Flavors](super::super#operation-flavors). Backends lower this to a
    /// multiplication by the constant `2^amount`.
    pub fn block_shl_with(
        &self,
        src: impl AsRef<CiphertextBlock>,
        amount: u8,
        flavor: Flavor,
    ) -> CiphertextBlock {
        assert!(
            amount < self.spec.complete_size(),
            "Tried to shift a block by {amount} bits, but the block only has {} bits.",
            self.spec.complete_size()
        );
        self.emit_block(
            IopInstructionSet::ShlCt { amount, flavor },
            svec![src.as_ref().valid],
        )
    }

    /// Shifts a ciphertext block left by `amount` bits (protect flavor).
    ///
    /// Computes `src << amount` at the block level. The operand padding bit must be clear
    /// and the shifted value must fit in the data bits. See
    /// [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let shifted = builder.block_shl(&blocks[0], 1);
    /// ```
    pub fn block_shl(&self, src: impl AsRef<CiphertextBlock>, amount: u8) -> CiphertextBlock {
        self.block_shl_with(src, amount, Flavor::Protect)
    }

    /// Shifts a ciphertext block left by `amount` bits (temper flavor).
    ///
    /// Computes `src << amount` at the block level. The result may set the padding bit but
    /// must not overflow past it. See [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_shl(
        &self,
        src: impl AsRef<CiphertextBlock>,
        amount: u8,
    ) -> CiphertextBlock {
        self.block_shl_with(src, amount, Flavor::Temper)
    }

    /// Shifts a ciphertext block left by `amount` bits (wrapping flavor).
    ///
    /// Computes `src << amount` modulo the complete block width. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_wrapping_shl(
        &self,
        src: impl AsRef<CiphertextBlock>,
        amount: u8,
    ) -> CiphertextBlock {
        self.block_shl_with(src, amount, Flavor::Wrapping)
    }

    /// Adds a plaintext block to a ciphertext block with the given flavor.
    ///
    /// Computes `src_a + src_b` where `src_a` is encrypted and `src_b` is plaintext. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_add_plaintext_with(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
        flavor: Flavor,
    ) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::AddPt { flavor },
            svec![src_a.as_ref().valid, src_b.as_ref().valid],
        )
    }

    /// Adds a plaintext block to a ciphertext block (protect flavor).
    ///
    /// Computes `src_a + src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses protect semantics — see [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let one = builder.block_let_plaintext(1);
    /// let incremented = builder.block_add_plaintext(&blocks[0], &one);
    /// ```
    pub fn block_add_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_add_plaintext_with(src_a, src_b, Flavor::Protect)
    }

    /// Adds a plaintext block to a ciphertext block (temper flavor).
    ///
    /// Computes `src_a + src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses temper semantics — see [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_add_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_add_plaintext_with(src_a, src_b, Flavor::Temper)
    }

    /// Adds a plaintext block to a ciphertext block (wrapping flavor).
    ///
    /// Computes `src_a + src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses wrapping semantics — see [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let one = builder.block_let_plaintext(1);
    /// let incremented = builder.block_wrapping_add_plaintext(&blocks[0], &one);
    /// ```
    pub fn block_wrapping_add_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_add_plaintext_with(src_a, src_b, Flavor::Wrapping)
    }

    /// Subtracts a plaintext block from a ciphertext block with the given flavor.
    ///
    /// Computes `src_a - src_b` where `src_a` is encrypted and `src_b` is plaintext. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_sub_plaintext_with(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
        flavor: Flavor,
    ) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::SubPt { flavor },
            svec![src_a.as_ref().valid, src_b.as_ref().valid],
        )
    }

    /// Subtracts a plaintext block from a ciphertext block (protect flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses protect semantics — see [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let one = builder.block_let_plaintext(1);
    /// let decremented = builder.block_sub_plaintext(&blocks[0], &one);
    /// ```
    pub fn block_sub_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_sub_plaintext_with(src_a, src_b, Flavor::Protect)
    }

    /// Subtracts a plaintext block from a ciphertext block (temper flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses temper semantics — see [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_sub_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_sub_plaintext_with(src_a, src_b, Flavor::Temper)
    }

    /// Subtracts a plaintext block from a ciphertext block (wrapping flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses wrapping semantics — see [Operation Flavors](super::super#operation-flavors).
    pub fn block_wrapping_sub_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_sub_plaintext_with(src_a, src_b, Flavor::Wrapping)
    }

    /// Subtracts a ciphertext block from a plaintext block with the given flavor.
    ///
    /// Computes `src_a - src_b` where `src_a` is plaintext and `src_b` is encrypted. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_plaintext_sub_with(
        &self,
        src_a: impl AsRef<PlaintextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        flavor: Flavor,
    ) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::PtSub { flavor },
            svec![src_a.as_ref().valid, src_b.as_ref().valid],
        )
    }

    /// Subtracts a ciphertext block from a plaintext block (protect flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is plaintext and `src_b` is encrypted.
    /// The result is a ciphertext block. Uses protect semantics — see
    /// [Operation Flavors](super::super#operation-flavors). Note the reversed operand order
    /// compared to [`block_sub_plaintext`](Self::block_sub_plaintext).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let three = builder.block_let_plaintext(3);
    /// let result = builder.block_plaintext_sub(&three, &blocks[0]); // 3 - blocks[0]
    /// ```
    pub fn block_plaintext_sub(
        &self,
        src_a: impl AsRef<PlaintextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_plaintext_sub_with(src_a, src_b, Flavor::Protect)
    }

    /// Subtracts a ciphertext block from a plaintext block (temper flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is plaintext and `src_b` is encrypted.
    /// Uses temper semantics — see [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_plaintext_sub(
        &self,
        src_a: impl AsRef<PlaintextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_plaintext_sub_with(src_a, src_b, Flavor::Temper)
    }

    /// Subtracts a ciphertext block from a plaintext block (wrapping flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is plaintext and `src_b` is encrypted.
    /// Uses wrapping semantics — see [Operation Flavors](super::super#operation-flavors).
    pub fn block_wrapping_plaintext_sub(
        &self,
        src_a: impl AsRef<PlaintextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_plaintext_sub_with(src_a, src_b, Flavor::Wrapping)
    }

    /// Multiplies a ciphertext block by a plaintext block with the given flavor.
    ///
    /// Computes `src_a * src_b` where `src_a` is encrypted and `src_b` is plaintext. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_mul_plaintext_with(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
        flavor: Flavor,
    ) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::MulPt { flavor },
            svec![src_a.as_ref().valid, src_b.as_ref().valid],
        )
    }

    /// Multiplies a ciphertext block by a plaintext block (protect flavor).
    ///
    /// Computes `src_a * src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses protect semantics — see [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(1);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let two = builder.block_let_plaintext(2);
    /// let doubled = builder.block_mul_plaintext(&blocks[0], &two);
    /// ```
    pub fn block_mul_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_mul_plaintext_with(src_a, src_b, Flavor::Protect)
    }

    /// Multiplies a ciphertext block by a plaintext block (temper flavor).
    ///
    /// Computes `src_a * src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses temper semantics — see [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_mul_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_mul_plaintext_with(src_a, src_b, Flavor::Temper)
    }

    /// Multiplies a ciphertext block by a plaintext block (wrapping flavor).
    ///
    /// Computes `src_a * src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses wrapping semantics — see [Operation Flavors](super::super#operation-flavors).
    pub fn block_wrapping_mul_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.block_mul_plaintext_with(src_a, src_b, Flavor::Wrapping)
    }

    /// Computes `src_a * mul + src_b` (multiply-accumulate) with the given flavor.
    ///
    /// This is a generalization of [`block_pack_with`](Self::block_pack_with) that accepts
    /// an arbitrary immediate multiplier instead of the fixed `2^message_size`. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_mac_with(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        mul: u8,
        flavor: Flavor,
    ) -> CiphertextBlock {
        self.emit_block(
            IopInstructionSet::PackCt { mul, flavor },
            svec![src_a.as_ref().valid, src_b.as_ref().valid],
        )
    }

    /// Computes `src_a * mul + src_b` (multiply-accumulate) on two ciphertext blocks.
    ///
    /// Uses protect semantics: operand padding bits must be clear and the result must fit
    /// in the data bits (carry + message). See
    /// [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// // Compute blocks[0] * 3 + blocks[1]
    /// let mac = builder.block_mac(&blocks[0], &blocks[1], 3);
    /// ```
    pub fn block_mac(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        mul: u8,
    ) -> CiphertextBlock {
        self.block_mac_with(src_a, src_b, mul, Flavor::Protect)
    }

    /// Computes `src_a * mul + src_b` (multiply-accumulate) with temper semantics.
    ///
    /// The result may set the padding bit but must not overflow past it. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_mac(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        mul: u8,
    ) -> CiphertextBlock {
        self.block_mac_with(src_a, src_b, mul, Flavor::Temper)
    }

    /// Computes `src_a * mul + src_b` (multiply-accumulate) with wrapping semantics.
    ///
    /// The result is reduced modulo the complete block width. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_wrapping_mac(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        mul: u8,
    ) -> CiphertextBlock {
        self.block_mac_with(src_a, src_b, mul, Flavor::Wrapping)
    }

    /// Packs two ciphertext blocks into one with the given flavor.
    ///
    /// Computes `src_a * 2^message_size + src_b`, placing `src_a` in the high (carry)
    /// bits and `src_b` in the low (message) bits of the resulting block. See
    /// [Operation Flavors](super::super#operation-flavors).
    ///
    /// # Panics
    ///
    /// Panics if the builder's `carry_size != message_size`, since packing requires
    /// equal-width carry and message fields.
    pub fn block_pack_with(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        flavor: Flavor,
    ) -> CiphertextBlock {
        assert_eq!(
            self.spec().carry_size(),
            self.spec().message_size(),
            "Packing requires equal carry and message sizes."
        );
        let mul = 2u8.pow(self.spec().message_size().sas::<u32>());
        self.block_mac_with(src_a, src_b, mul, flavor)
    }

    /// Packs two ciphertext blocks into one (protect flavor).
    ///
    /// Computes `src_a * 2^message_size + src_b`, placing `src_a` in the high (carry)
    /// bits and `src_b` in the low (message) bits of the resulting block. This is the
    /// standard way to pack two blocks to be processed within a single programmable
    /// bootstrapping (PBS) lookup. Both operands must be clean (padding and carry bits
    /// clear) for the result to fit.
    ///
    /// # Panics
    ///
    /// Panics if the builder's `carry_size != message_size`, since packing requires
    /// equal-width carry and message fields.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let packed = builder.block_pack(&blocks[1], &blocks[0]);
    /// let result = builder.block_lookup(&packed, Lut1Def::MsgOnly);
    /// ```
    pub fn block_pack(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_pack_with(src_a, src_b, Flavor::Protect)
    }

    /// Packs two ciphertext blocks into one (temper flavor).
    ///
    /// Like [`block_pack`](Self::block_pack), but the result may set the padding bit.
    /// Useful before a negacyclic lookup. See
    /// [Operation Flavors](super::super#operation-flavors).
    pub fn block_temper_pack(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_pack_with(src_a, src_b, Flavor::Temper)
    }

    /// Packs two ciphertext blocks into one (wrapping flavor).
    ///
    /// Like [`block_pack`](Self::block_pack), but the result is reduced modulo the
    /// complete block width. See [Operation Flavors](super::super#operation-flavors).
    pub fn block_wrapping_pack(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.block_pack_with(src_a, src_b, Flavor::Wrapping)
    }

    /// Packs two ciphertext blocks and applies a single-output PBS lookup.
    ///
    /// Equivalent to calling [`block_pack`](Self::block_pack) followed by
    /// [`block_lookup`](Self::block_lookup). This is a convenience for the common
    /// pack-then-lookup pattern.
    ///
    /// # Panics
    ///
    /// Panics if `carry_size != message_size` (see [`block_pack`](Self::block_pack)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let result = builder.block_pack_then_lookup(&blocks[1], &blocks[0], Lut1Def::MsgOnly);
    /// ```
    pub fn block_pack_then_lookup(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        def: Lut1Def,
    ) -> CiphertextBlock {
        let packed = self.block_pack(src_a, src_b);
        self.block_lookup(&packed, def)
    }

    /// Computes `src_a * mul + src_b` and applies a single-output PBS lookup.
    ///
    /// Combines a multiply-accumulate with an immediate multiplier and a programmable
    /// bootstrapping in a single convenience method. The `mul` value is typically a small
    /// power of two used for packing or shifting encoded values before the lookup.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// // Compute blocks[0] * 2 + blocks[1], then extract the message
    /// let result = builder.block_mac_then_lookup(&blocks[0], &blocks[1], 2, Lut1Def::MsgOnly);
    /// ```
    pub fn block_mac_then_lookup(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        mul: u8,
        def: Lut1Def,
    ) -> CiphertextBlock {
        let mac = self.block_mac(src_a, src_b, mul);
        self.block_lookup(&mac, def)
    }

    /// Applies a single-output PBS lookup with an explicit padding-check policy.
    ///
    /// The `def` defines the function computed by the bootstrapping. The input block's
    /// data bits (carry + message) index into the lookup table, and the result is a fresh
    /// ciphertext block with clean noise. When the input padding bit is set, the output is
    /// negacyclically negated. The `check` controls which padding bits are asserted clear
    /// — see [Lookup Checks](super::super#lookup-checks). Prefer the named shortcuts
    /// [`block_lookup`](Self::block_lookup),
    /// [`block_padding_lookup`](Self::block_padding_lookup) and
    /// [`block_wrapping_lookup`](Self::block_wrapping_lookup) when the policy is fixed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let out = builder.block_lookup_with(&blocks[0], Lut1Def::MsgOnly, LookupCheck::AllowInputPadding);
    /// ```
    pub fn block_lookup_with(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut1Def,
        check: LookupCheck,
    ) -> CiphertextBlock {
        let lut = def.into_lut(self.spec);
        self.emit_block(
            IopInstructionSet::Pbs { check, lut },
            svec![src.as_ref().valid],
        )
    }

    /// Applies a single-output programmable bootstrapping (PBS) lookup to a block.
    ///
    /// The `def` defines the function computed by the bootstrapping. The input block's
    /// full data bits (carry + message) index into the lookup table, and the result is a
    /// fresh ciphertext block with clean noise. Both the input and the output padding bits
    /// are asserted clear ([`LookupCheck::Protect`]).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// // Extract only the message bits, clearing the carry.
    /// let clean = builder.block_lookup(&blocks[0], Lut1Def::MsgOnly);
    /// ```
    pub fn block_lookup(&self, src: impl AsRef<CiphertextBlock>, def: Lut1Def) -> CiphertextBlock {
        self.block_lookup_with(src, def, LookupCheck::Protect)
    }

    /// Applies a single-output PBS lookup allowing output padding overflow.
    ///
    /// Like [`block_lookup`](Self::block_lookup), but the bootstrapping allows the result
    /// to overflow into the output padding bit ([`LookupCheck::AllowOutputPadding`]). The
    /// input padding bit must still be clear. This is useful when a subsequent operation
    /// will consume the padding bit before the next lookup.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let result = builder.block_padding_lookup(&blocks[0], Lut1Def::MsgOnly);
    /// ```
    pub fn block_padding_lookup(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut1Def,
    ) -> CiphertextBlock {
        self.block_lookup_with(src, def, LookupCheck::AllowOutputPadding)
    }

    /// Applies a single-output PBS lookup using wrapping (negacyclic) semantics.
    ///
    /// Like [`block_lookup`](Self::block_lookup), but no padding bit is checked
    /// ([`LookupCheck::AllowBothPadding`]). This is appropriate when the input block's
    /// padding bit may be set, enabling negacyclic lookup behavior.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let result = builder.block_wrapping_lookup(&blocks[0], Lut1Def::MsgOnly);
    /// ```
    pub fn block_wrapping_lookup(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut1Def,
    ) -> CiphertextBlock {
        self.block_lookup_with(src, def, LookupCheck::AllowBothPadding)
    }

    /// Applies a dual-output PBS lookup with an explicit padding-check policy.
    ///
    /// Like [`block_lookup2`](Self::block_lookup2) with a configurable `check`. Many-LUT
    /// bootstrapping reserves the topmost data bit of the input, so only
    /// [`LookupCheck::Protect`] and [`LookupCheck::AllowOutputPadding`] are accepted; the
    /// interpreter panics on the other policies.
    pub fn block_lookup2_with(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut2Def,
        check: LookupCheck,
    ) -> (CiphertextBlock, CiphertextBlock) {
        let lut = def.into_lut(self.spec);
        let [o0, o1] = self.emit_blocks::<2>(
            IopInstructionSet::Pbs2 { check, lut },
            svec![src.as_ref().valid],
        );
        (o0, o1)
    }

    /// Applies a dual-output programmable bootstrapping (PBS) lookup to a block.
    ///
    /// Like [`block_lookup`](Self::block_lookup), but the bootstrapping produces two
    /// output blocks from a single input. The two lookup functions are defined by the
    /// [`Lut2Def`] variant. This amortizes the cost of a PBS when two related values
    /// need to be extracted simultaneously. The input must have its padding bit and its
    /// topmost data bit clear.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut2Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let packed = builder.block_pack(&blocks[1], &blocks[0]);
    /// let (msg, carry) = builder.block_lookup2(&packed, Lut2Def::ManyCarryMsg);
    /// ```
    pub fn block_lookup2(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut2Def,
    ) -> (CiphertextBlock, CiphertextBlock) {
        self.block_lookup2_with(src, def, LookupCheck::Protect)
    }

    /// Applies a four-output PBS lookup with an explicit padding-check policy.
    ///
    /// Like [`block_lookup4`](Self::block_lookup4) with a configurable `check`. Only
    /// [`LookupCheck::Protect`] and [`LookupCheck::AllowOutputPadding`] are accepted.
    pub fn block_lookup4_with(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut4Def,
        check: LookupCheck,
    ) -> [CiphertextBlock; 4] {
        let lut = def.into_lut(self.spec);
        self.emit_blocks::<4>(
            IopInstructionSet::Pbs4 { check, lut },
            svec![src.as_ref().valid],
        )
    }

    /// Applies a four-output programmable bootstrapping (PBS) lookup to a block.
    ///
    /// Produces four output blocks from a single input, one per function of the
    /// [`Lut4Def`]. The input must have its padding bit and its two topmost data bits
    /// clear, since those bits are reserved by the many-LUT encoding.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut4Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(2);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let def = Lut4Def::custom("shifts", [
    ///     |b| b,
    ///     |b| b.spec().from_data((b.raw_message_bits() << 1) & b.spec().data_mask()),
    ///     |b| b.spec().from_data((b.raw_message_bits() << 2) & b.spec().data_mask()),
    ///     |b| b.spec().from_data((b.raw_message_bits() << 3) & b.spec().data_mask()),
    /// ]);
    /// let [s0, s1, s2, s3] = builder.block_lookup4(&blocks[0], def);
    /// ```
    pub fn block_lookup4(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut4Def,
    ) -> [CiphertextBlock; 4] {
        self.block_lookup4_with(src, def, LookupCheck::Protect)
    }

    /// Applies an eight-output PBS lookup with an explicit padding-check policy.
    ///
    /// Like [`block_lookup8`](Self::block_lookup8) with a configurable `check`. Only
    /// [`LookupCheck::Protect`] and [`LookupCheck::AllowOutputPadding`] are accepted.
    pub fn block_lookup8_with(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut8Def,
        check: LookupCheck,
    ) -> [CiphertextBlock; 8] {
        let lut = def.into_lut(self.spec);
        self.emit_blocks::<8>(
            IopInstructionSet::Pbs8 { check, lut },
            svec![src.as_ref().valid],
        )
    }

    /// Applies an eight-output programmable bootstrapping (PBS) lookup to a block.
    ///
    /// Produces eight output blocks from a single input, one per function of the
    /// [`Lut8Def`]. The input must have its padding bit and its three topmost data bits
    /// clear, since those bits are reserved by the many-LUT encoding. With a
    /// `CiphertextBlockSpec(2, 2)` this leaves a single usable input bit.
    pub fn block_lookup8(
        &self,
        src: impl AsRef<CiphertextBlock>,
        def: Lut8Def,
    ) -> [CiphertextBlock; 8] {
        self.block_lookup8_with(src, def, LookupCheck::Protect)
    }
}

impl Builder {
    /// Packs consecutive pairs of blocks in a slice.
    ///
    /// Iterates over `blocks` in chunks of two, calling [`block_pack`](Self::block_pack)
    /// on each pair. Within each pair, the second element (`blocks[2i+1]`) goes to the
    /// high bits and the first (`blocks[2i]`) to the low bits. If the slice has an odd
    /// number of elements, the trailing block is passed through unchanged.
    ///
    /// The output has length `ceil(blocks.len() / 2)`.
    ///
    /// # Panics
    ///
    /// Panics if `carry_size != message_size` (see [`block_pack`](Self::block_pack)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct); // 4 blocks
    /// let packed = builder.vector_pack(&blocks);  // 2 packed blocks
    /// ```
    pub fn vector_pack(&self, blocks: impl AsRef<[CiphertextBlock]>) -> Vec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .chunk(2)
            .map(|a| match a {
                Chunk::Complete(sv) => self.block_pack(sv[1], sv[0]),
                Chunk::Rest(sv) => *sv[0],
            })
            .collect()
    }

    /// Packs consecutive pairs and applies an identity PBS to clean noise.
    ///
    /// Equivalent to calling [`vector_pack_then_lookup`](Self::vector_pack_then_lookup)
    /// with [`Lut1Def::None`]. The PBS acts as a noise-refresh: each packed pair is
    /// bootstrapped through the identity function, producing a clean block. Trailing
    /// odd blocks are passed through without bootstrapping.
    ///
    /// # Panics
    ///
    /// Panics if `carry_size != message_size` (see [`block_pack`](Self::block_pack)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let cleaned = builder.vector_pack_then_clean(&blocks);
    /// ```
    pub fn vector_pack_then_clean(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
    ) -> Vec<CiphertextBlock> {
        self.vector_pack_then_lookup(blocks, Lut1Def::None)
    }

    /// Packs consecutive pairs and applies a single-output PBS lookup to each.
    ///
    /// Iterates over `blocks` in chunks of two: each pair is
    /// [`block_pack`](Self::block_pack)ed and then passed through
    /// [`block_lookup`](Self::block_lookup) with the given `lut`. If the slice has an odd
    /// number of elements, the trailing block is passed through unchanged (no PBS).
    ///
    /// The output has length `ceil(blocks.len() / 2)`.
    ///
    /// # Panics
    ///
    /// Panics if `carry_size != message_size` (see [`block_pack`](Self::block_pack)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let results = builder.vector_pack_then_lookup(&blocks, Lut1Def::MsgOnly);
    /// ```
    pub fn vector_pack_then_lookup(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        def: Lut1Def,
    ) -> Vec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .chunk(2)
            .map(|a| match a {
                Chunk::Complete(sv) => {
                    let packed = self.block_pack(sv[1], sv[0]);
                    self.block_lookup(&packed, def.clone())
                }
                Chunk::Rest(sv) => *sv[0],
            })
            .collect()
    }

    /// Zips two block slices, packs each pair, and applies a PBS lookup.
    ///
    /// For each position, packs `lhs[i]` into the high bits and `rhs[i]` into the low
    /// bits via [`block_pack`](Self::block_pack), then passes the result through
    /// [`block_lookup`](Self::block_lookup) with the given `lut`. When the two slices
    /// have different lengths, `extension` controls the behavior (see
    /// [`ExtensionBehavior`]).
    ///
    /// # Panics
    ///
    /// Panics if `carry_size != message_size` (see [`block_pack`](Self::block_pack)), or
    /// if the slices differ in length and `extension` is
    /// [`Panic`](ExtensionBehavior::Panic).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let a = builder.ciphertext_input(8);
    /// let b = builder.ciphertext_input(8);
    /// let a_blocks = builder.ciphertext_split(&a);
    /// let b_blocks = builder.ciphertext_split(&b);
    /// let results = builder.vector_zip_then_lookup(
    ///     &a_blocks,
    ///     &b_blocks,
    ///     Lut1Def::MsgOnly,
    ///     ExtensionBehavior::Panic,
    /// );
    /// ```
    pub fn vector_zip_then_lookup(
        &self,
        lhs: impl AsRef<[CiphertextBlock]>,
        rhs: impl AsRef<[CiphertextBlock]>,
        def: Lut1Def,
        extension: ExtensionBehavior,
    ) -> Vec<CiphertextBlock> {
        let mut output = Vec::new();
        let mut lhs_i = lhs.as_ref().iter();
        let mut rhs_i = rhs.as_ref().iter();
        loop {
            match (&extension, lhs_i.next(), rhs_i.next()) {
                (_, Some(li), Some(ri)) => {
                    let packed = self.block_pack(li, ri);
                    output.push(self.block_lookup(packed, def.clone()))
                }
                (_, None, None) => break,
                (ExtensionBehavior::Panic, _, _) => panic!(),
                (ExtensionBehavior::Limit, _, _) => break,
                (ExtensionBehavior::Passthrough, None, Some(v)) => output.push(*v),
                (ExtensionBehavior::Passthrough, Some(v), None) => output.push(*v),
            }
        }
        output
    }

    /// Applies a single-output PBS lookup to every block in a slice.
    ///
    /// Maps [`block_lookup`](Self::block_lookup) over each element. Unlike
    /// [`vector_pack_then_lookup`](Self::vector_pack_then_lookup), no packing is
    /// performed — each block is bootstrapped independently.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let cleaned = builder.vector_lookup(&blocks, Lut1Def::MsgOnly);
    /// ```
    pub fn vector_lookup(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        def: Lut1Def,
    ) -> Vec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_lookup(b, def.clone()))
            .collect()
    }

    /// Applies a dual-output PBS lookup to every block in a slice.
    ///
    /// Maps [`block_lookup2`](Self::block_lookup2) over each element, returning a pair of
    /// output blocks per input block.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// # use zhc_langs::ioplang::Lut2Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let packed = builder.vector_pack(&blocks);
    /// let pairs = builder.vector_lookup2(&packed, Lut2Def::ManyCarryMsg);
    /// ```
    pub fn vector_lookup2(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        def: Lut2Def,
    ) -> Vec<(CiphertextBlock, CiphertextBlock)> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_lookup2(b, def.clone()))
            .collect()
    }

    /// Applies a four-output PBS lookup to every block in a slice.
    ///
    /// Maps [`block_lookup4`](Self::block_lookup4) over each element, returning four
    /// output blocks per input block.
    pub fn vector_lookup4(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        def: Lut4Def,
    ) -> Vec<[CiphertextBlock; 4]> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_lookup4(b, def.clone()))
            .collect()
    }

    /// Applies an eight-output PBS lookup to every block in a slice.
    ///
    /// Maps [`block_lookup8`](Self::block_lookup8) over each element, returning eight
    /// output blocks per input block.
    pub fn vector_lookup8(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        def: Lut8Def,
    ) -> Vec<[CiphertextBlock; 8]> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_lookup8(b, def.clone()))
            .collect()
    }

    /// Applies a single-output PBS lookup with an explicit check to every block in a slice.
    ///
    /// Maps [`block_lookup_with`](Self::block_lookup_with) over each element.
    pub fn vector_lookup_with(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        def: Lut1Def,
        check: LookupCheck,
    ) -> Vec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_lookup_with(b, def.clone(), check))
            .collect()
    }

    /// Adds two block slices element-wise with the given flavor.
    ///
    /// Like [`vector_add`](Self::vector_add), using [`block_add_with`](Self::block_add_with)
    /// for each pair.
    pub fn vector_add_with(
        &self,
        lhs: impl AsRef<[CiphertextBlock]>,
        rhs: impl AsRef<[CiphertextBlock]>,
        flavor: Flavor,
        extension: ExtensionBehavior,
    ) -> Vec<CiphertextBlock> {
        let mut output = Vec::new();
        let mut lhs_i = lhs.as_ref().iter();
        let mut rhs_i = rhs.as_ref().iter();
        loop {
            match (&extension, lhs_i.next(), rhs_i.next()) {
                (_, Some(li), Some(ri)) => output.push(self.block_add_with(li, ri, flavor)),
                (_, None, None) => break,
                (ExtensionBehavior::Panic, _, _) => panic!(),
                (ExtensionBehavior::Limit, _, _) => break,
                (ExtensionBehavior::Passthrough, None, Some(v)) => output.push(*v),
                (ExtensionBehavior::Passthrough, Some(v), None) => output.push(*v),
            }
        }
        output
    }

    /// Adds two block slices element-wise.
    ///
    /// For each position, calls [`block_add`](Self::block_add) on the corresponding pair.
    /// When the two slices have different lengths, `extension` controls the behavior (see
    /// [`ExtensionBehavior`]).
    ///
    /// # Panics
    ///
    /// Panics if the slices differ in length and `extension` is
    /// [`Panic`](ExtensionBehavior::Panic).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let a = builder.ciphertext_input(8);
    /// let b = builder.ciphertext_input(8);
    /// let a_blocks = builder.ciphertext_split(&a);
    /// let b_blocks = builder.ciphertext_split(&b);
    /// let sums = builder.vector_add(&a_blocks, &b_blocks, ExtensionBehavior::Panic);
    /// ```
    pub fn vector_add(
        &self,
        lhs: impl AsRef<[CiphertextBlock]>,
        rhs: impl AsRef<[CiphertextBlock]>,
        extension: ExtensionBehavior,
    ) -> Vec<CiphertextBlock> {
        let mut output = Vec::new();
        let mut lhs_i = lhs.as_ref().iter();
        let mut rhs_i = rhs.as_ref().iter();
        loop {
            match (&extension, lhs_i.next(), rhs_i.next()) {
                (_, Some(li), Some(ri)) => output.push(self.block_add(li, ri)),
                (_, None, None) => break,
                (ExtensionBehavior::Panic, _, _) => panic!(),
                (ExtensionBehavior::Limit, _, _) => break,
                (ExtensionBehavior::Passthrough, None, Some(v)) => output.push(*v),
                (ExtensionBehavior::Passthrough, Some(v), None) => output.push(*v),
            }
        }
        output
    }

    /// Zero-extends a block slice to a given length.
    ///
    /// Pads `inp` with zero-valued constant ciphertext blocks
    /// ([`block_let_ciphertext(0)`](Self::block_let_ciphertext)) until the result
    /// has `size` elements. This implements unsigned integer extension: the original
    /// blocks represent the low-order radix digits, and the appended zeros represent
    /// high-order digits.
    ///
    /// # Panics
    ///
    /// Panics if `inp.len() > size`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(4);        // 2 blocks
    /// let blocks = builder.ciphertext_split(&ct);
    /// let extended = builder.vector_unsigned_extension(&blocks, 4); // now 4 blocks
    /// ```
    pub fn vector_unsigned_extension(
        &self,
        inp: impl AsRef<[CiphertextBlock]>,
        size: usize,
    ) -> Vec<CiphertextBlock> {
        let inp = inp.as_ref();
        assert!(
            inp.len() <= size,
            "Tried to extend a vector that is larger than the extended size."
        );
        inp.iter()
            .cloned()
            .chain(repeat_n(self.block_let_ciphertext(0), size - inp.len()))
            .collect()
    }

    /// Reduces a block slice to a single block by summing all elements (protect flavor).
    ///
    /// Folds [`block_add`](Self::block_add) across every element, returning their
    /// cumulative sum. This is useful for combining partial results from parallel
    /// computations into a single accumulator block.
    ///
    /// # Panics
    ///
    /// Panics if `inp` is empty.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let total = builder.vector_add_reduce(&blocks);
    /// ```
    pub fn vector_add_reduce(&self, inp: impl AsRef<[CiphertextBlock]>) -> CiphertextBlock {
        let inp = inp.as_ref();
        assert!(
            !inp.is_empty(),
            "Tried add-reduce an empty vector of blocks."
        );
        inp.iter()
            .cloned()
            .reduce(|a, n| self.block_add(a, n))
            .unwrap()
    }

    /// Applies [`block_inspect`](Self::block_inspect) to every block in a slice.
    ///
    /// Each block is inspected with an index-based comment (`"0"`, `"1"`, ...) appended to
    /// the current comment stack. This is useful for labeling block positions in IR dumps.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.ciphertext_input(8);
    /// let blocks = builder.ciphertext_split(&ct);
    /// let labeled = builder.comment("radix digits").vector_inspect(&blocks);
    /// ```
    pub fn vector_inspect(&self, inp: impl AsRef<[CiphertextBlock]>) -> Vec<CiphertextBlock> {
        inp.as_ref()
            .iter()
            .enumerate()
            .map(|(i, b)| self.comment(format!("{i}")).block_inspect(b))
            .collect()
    }
}

/// Strategy for handling mismatched slice lengths in binary vector operations.
///
/// Binary vector operations like [`Builder::vector_add`] and [`Builder::vector_zip_then_lookup`]
/// take two block slices that may differ in length. This enum controls what happens
/// once the shorter slice is exhausted.
pub enum ExtensionBehavior {
    /// Panics if the slices have different lengths.
    Panic,
    /// Truncates to the length of the shorter slice, discarding surplus elements.
    Limit,
    /// Passes surplus elements from the longer slice through unchanged.
    Passthrough,
}

impl Dumpable for Builder {
    fn dump_to_string(&self) -> String {
        format!(
            "╔══════════════════════════════════════════════════════════════════════════════
║ Sig: {}
║──────────────────────────────────────────────────────────────────────────────
{}
╚══════════════════════════════════════════════════════════════════════════════",
            self.signature(),
            self.ir()
                .format()
                .with_prefix("║ ")
                .with_walker(PrintWalker::Linear)
        )
    }
}
