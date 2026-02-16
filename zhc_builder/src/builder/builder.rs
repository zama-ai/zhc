use crate::builder::{Ciphertext, CiphertextBlock, Plaintext, PlaintextBlock};
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    iter::repeat_n,
};
use zhc_crypto::integer_semantics::{CiphertextBlockSpec, CiphertextSpec, PlaintextSpec};
use zhc_ir::{
    IR, PrintWalker, Signature, cse::eliminate_common_subexpressions, dce::eliminate_dead_code,
};
use zhc_langs::ioplang::{
    IopInstructionSet, IopInterepreterContext, IopLang, IopTypeSystem, IopValue, Lut1Def, Lut2Def,
    eliminate_aliases, skip_store_load,
};
use zhc_utils::{
    FastMap,
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

struct InnerBuilder {
    ir: IR<IopLang>,
    sig: Signature<Type>,
    comment_stack: Vec<String>,
}

impl InnerBuilder {
    fn current_comment(&self) -> Option<String> {
        if self.comment_stack.is_empty() {
            None
        } else {
            Some(self.comment_stack.join(" / "))
        }
    }

    fn add_op(
        &mut self,
        op: IopInstructionSet,
        args: SmallVec<zhc_ir::ValId>,
    ) -> (zhc_ir::OpId, SmallVec<zhc_ir::ValId>) {
        match self.current_comment() {
            Some(comment) => self.ir.add_op_with_comment(op, args, comment).unwrap(),
            None => self.ir.add_op(op, args).unwrap(),
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

/// A scoped comment guard returned by [`Builder::comment`].
///
/// This wrapper holds a reference to the parent [`Builder`] and delegates every builder
/// method to it. When dropped, it automatically pops the comment that was pushed when
/// [`Builder::comment`] was called. This RAII pattern ensures comments are properly
/// nested even when chaining multiple operations.
///
/// All public methods mirror the corresponding [`Builder`] methods and consume `self`,
/// meaning you can only call one builder method per [`CommentBuilder`]. To emit multiple
/// operations under the same comment, use [`Builder::with_comment`] instead.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::builder::*;
/// let builder = Builder::new(CiphertextBlockSpec(2, 2));
/// let ct = builder.declare_ciphertext_input(4);
/// let blocks = builder.split_ciphertext(&ct);
/// // The comment is popped after block_add returns.
/// let sum = builder.comment("add first two blocks").block_add(&blocks[0], &blocks[1]);
/// ```
pub struct CommentBuilder<'a> {
    builder: &'a Builder,
}

impl<'a> CommentBuilder<'a> {
    /// Declares an encrypted integer input. See [`Builder::declare_ciphertext_input`].
    pub fn declare_ciphertext_input(self, int_size: u16) -> Ciphertext {
        self.builder.declare_ciphertext_input(int_size)
    }

    /// Decomposes a ciphertext into blocks. See [`Builder::split_ciphertext`].
    pub fn split_ciphertext(self, inp: impl AsRef<Ciphertext>) -> Vec<CiphertextBlock> {
        self.builder.split_ciphertext(inp)
    }

    /// Declares a plaintext integer input. See [`Builder::declare_plaintext_input`].
    pub fn declare_plaintext_input(self, int_size: u16) -> Plaintext {
        self.builder.declare_plaintext_input(int_size)
    }

    /// Decomposes a plaintext into blocks. See [`Builder::split_plaintext`].
    pub fn split_plaintext(self, inp: impl AsRef<Plaintext>) -> Vec<PlaintextBlock> {
        self.builder.split_plaintext(inp)
    }

    /// Reassembles blocks into a ciphertext. See [`Builder::join_ciphertext`].
    pub fn join_ciphertext(self, blocks: impl AsRef<[CiphertextBlock]>) -> Ciphertext {
        self.builder.join_ciphertext(blocks)
    }

    /// Declares an encrypted integer output. See [`Builder::declare_ciphertext_output`].
    pub fn declare_ciphertext_output(self, ct: impl AsRef<Ciphertext>) {
        self.builder.declare_ciphertext_output(ct)
    }

    /// Creates a constant plaintext block. See [`Builder::block_const_plaintext`].
    pub fn block_const_plaintext(self, constant: u8) -> PlaintextBlock {
        self.builder.block_const_plaintext(constant)
    }

    /// Creates a constant ciphertext block. See [`Builder::block_const_ciphertext`].
    pub fn block_const_ciphertext(self, constant: u8) -> CiphertextBlock {
        self.builder.block_const_ciphertext(constant)
    }

    /// Adds two ciphertext blocks (protect). See [`Builder::block_add`].
    pub fn block_add(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_add(src_a, src_b)
    }

    /// Creates an alias for a ciphertext block. See [`Builder::block_alias`].
    pub fn block_alias(self, src: impl AsRef<CiphertextBlock>) -> CiphertextBlock {
        self.builder.block_alias(src)
    }

    /// Adds two ciphertext blocks (temper). See [`Builder::block_temper_add`].
    pub fn block_temper_add(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_temper_add(src_a, src_b)
    }

    /// Adds two ciphertext blocks (wrapping). See [`Builder::block_wrapping_add`].
    pub fn block_wrapping_add(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_wrapping_add(src_a, src_b)
    }

    /// Adds a plaintext to a ciphertext block (protect). See [`Builder::block_add_plaintext`].
    pub fn block_add_plaintext(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_add_plaintext(src_a, src_b)
    }

    /// Adds a plaintext to a ciphertext block (wrapping). See
    /// [`Builder::block_wrapping_add_plaintext`].
    pub fn block_wrapping_add_plaintext(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_wrapping_add_plaintext(src_a, src_b)
    }

    /// Subtracts two ciphertext blocks (protect). See [`Builder::block_sub`].
    pub fn block_sub(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_sub(src_a, src_b)
    }

    /// Subtracts a plaintext from a ciphertext block (protect). See
    /// [`Builder::block_sub_plaintext`].
    pub fn block_sub_plaintext(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_sub_plaintext(src_a, src_b)
    }

    /// Subtracts a ciphertext from a plaintext block (protect). See
    /// [`Builder::block_plaintext_sub`].
    pub fn block_plaintext_sub(
        self,
        src_a: impl AsRef<PlaintextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_plaintext_sub(src_a, src_b)
    }

    /// Packs two ciphertext blocks into one. See [`Builder::block_pack`].
    pub fn block_pack(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        self.builder.block_pack(src_a, src_b)
    }

    /// Packs two blocks and applies a PBS lookup. See [`Builder::block_pack_then_lookup`].
    pub fn block_pack_then_lookup(
        self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        lut: Lut1Def,
    ) -> CiphertextBlock {
        self.builder.block_pack_then_lookup(src_a, src_b, lut)
    }

    /// Applies a single-output PBS lookup. See [`Builder::block_lookup`].
    pub fn block_lookup(self, src: impl AsRef<CiphertextBlock>, lut: Lut1Def) -> CiphertextBlock {
        self.builder.block_lookup(src, lut)
    }

    /// Applies a single-output PBS lookup (wrapping). See [`Builder::block_wrapping_lookup`].
    pub fn block_wrapping_lookup(
        self,
        src: impl AsRef<CiphertextBlock>,
        lut: Lut1Def,
    ) -> CiphertextBlock {
        self.builder.block_wrapping_lookup(src, lut)
    }

    /// Applies a dual-output PBS lookup. See [`Builder::block_lookup2`].
    pub fn block_lookup2(
        self,
        src: impl AsRef<CiphertextBlock>,
        lut: Lut2Def,
    ) -> (CiphertextBlock, CiphertextBlock) {
        self.builder.block_lookup2(src, lut)
    }

    /// Packs consecutive pairs of blocks. See [`Builder::vector_pack`].
    pub fn vector_pack(self, blocks: impl AsRef<[CiphertextBlock]>) -> Vec<CiphertextBlock> {
        self.builder.vector_pack(blocks)
    }

    /// Packs consecutive pairs and applies an identity PBS. See
    /// [`Builder::vector_pack_then_clean`].
    pub fn vector_pack_then_clean(
        self,
        blocks: impl AsRef<[CiphertextBlock]>,
    ) -> Vec<CiphertextBlock> {
        self.builder.vector_pack_then_clean(blocks)
    }

    /// Packs consecutive pairs and applies a PBS lookup. See
    /// [`Builder::vector_pack_then_lookup`].
    pub fn vector_pack_then_lookup(
        self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
    ) -> Vec<CiphertextBlock> {
        self.builder.vector_pack_then_lookup(blocks, lut)
    }

    /// Zips two block slices, packs each pair, and applies a PBS lookup. See
    /// [`Builder::vector_zip_then_lookup`].
    pub fn vector_zip_then_lookup(
        self,
        lhs: impl AsRef<[CiphertextBlock]>,
        rhs: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
        extension: ExtensionBehavior,
    ) -> Vec<CiphertextBlock> {
        self.builder
            .vector_zip_then_lookup(lhs, rhs, lut, extension)
    }

    /// Applies a PBS lookup to every block. See [`Builder::vector_lookup`].
    pub fn vector_lookup(
        self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
    ) -> Vec<CiphertextBlock> {
        self.builder.vector_lookup(blocks, lut)
    }

    /// Applies a dual-output PBS lookup to every block. See [`Builder::vector_lookup2`].
    pub fn vector_lookup2(
        self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut2Def,
    ) -> Vec<(CiphertextBlock, CiphertextBlock)> {
        self.builder.vector_lookup2(blocks, lut)
    }

    /// Adds two block slices element-wise. See [`Builder::vector_add`].
    pub fn vector_add(
        self,
        lhs: impl AsRef<[CiphertextBlock]>,
        rhs: impl AsRef<[CiphertextBlock]>,
        extension: ExtensionBehavior,
    ) -> Vec<CiphertextBlock> {
        self.builder.vector_add(lhs, rhs, extension)
    }

    /// Zero-extends a block slice to a given length. See [`Builder::vector_unsigned_extension`].
    pub fn vector_unsigned_extension(
        self,
        inp: impl AsRef<[CiphertextBlock]>,
        size: usize,
    ) -> Vec<CiphertextBlock> {
        self.builder.vector_unsigned_extension(inp, size)
    }
}

impl<'a> Drop for CommentBuilder<'a> {
    fn drop(&mut self) {
        self.builder.pop_comment();
    }
}

/// High-level builder for constructing FHE circuits as IR graphs.
///
/// A [`Builder`] accumulates IR instructions through its methods, using interior mutability
/// so that all operations take `&self`. The typical lifecycle is: create a builder, declare
/// inputs, emit block-level or vector-level operations, declare outputs, and finally call
/// [`into_ir`](Self::into_ir) to obtain the optimized IR.
///
/// Every builder is parameterized by a single [`CiphertextBlockSpec`] that defines the
/// message/carry bit layout shared by all ciphertext blocks in the circuit. This spec is
/// set at construction time and accessible via [`spec`](Self::spec).
///
/// # Input / Output Ordering
///
/// Inputs and outputs are **positional**: they are recorded in the order they are
/// declared. The first call to [`declare_ciphertext_input`](Self::declare_ciphertext_input)
/// or [`declare_plaintext_input`](Self::declare_plaintext_input) becomes input 0, the
/// second becomes input 1, and so on — both kinds share the same index space. Likewise,
/// the first [`declare_ciphertext_output`](Self::declare_ciphertext_output) becomes
/// output 0. This ordering defines the circuit's [`signature`](Self::signature) and must
/// match the order of values passed to [`eval`](Self::eval).
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
/// # use zhc_builder::builder::*;
/// let builder = Builder::new(CiphertextBlockSpec(2, 2));
/// let input = builder.declare_ciphertext_input(8);
/// let blocks = builder.split_ciphertext(&input);
/// // ... operate on blocks ...
/// let output = builder.join_ciphertext(&blocks);
/// builder.declare_ciphertext_output(&output);
/// let ir = builder.into_ir();
/// ```
pub struct Builder {
    spec: CiphertextBlockSpec,
    inner: RefCell<InnerBuilder>,
}

impl Builder {
    fn inner(&self) -> Ref<'_, InnerBuilder> {
        self.inner.borrow()
    }

    fn inner_mut(&self) -> RefMut<'_, InnerBuilder> {
        self.inner.borrow_mut()
    }

    /// Pushes a comment and returns a guard that delegates builder methods.
    ///
    /// This is a fluent alternative to [`with_comment`](Self::with_comment) for single
    /// operations. The returned [`CommentBuilder`] forwards one builder call under the
    /// pushed comment, then automatically pops it when dropped.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.declare_ciphertext_input(4);
    /// let blocks = builder.split_ciphertext(&ct);
    /// let sum = builder.comment("add blocks").block_add(&blocks[0], &blocks[1]);
    /// ```
    pub fn comment(&self, comment: impl Into<String>) -> CommentBuilder<'_> {
        self.push_comment(comment);
        CommentBuilder { builder: self }
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
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// ```
    pub fn new(spec: CiphertextBlockSpec) -> Self {
        Self {
            spec: spec,
            inner: RefCell::new(InnerBuilder {
                ir: IR::empty(),
                sig: Signature::empty(),
                comment_stack: Vec::new(),
            }),
        }
    }

    /// Pushes a comment onto the annotation stack.
    ///
    /// All IR instructions emitted while this comment is on the stack will be annotated
    /// with the full stack joined by ` / `. Use [`pop_comment`](Self::pop_comment) to
    /// remove it, or prefer the RAII-style [`with_comment`](Self::with_comment).
    pub fn push_comment(&self, comment: impl Into<String>) {
        self.inner_mut().comment_stack.push(comment.into());
    }

    /// Pops the most recent comment from the annotation stack.
    pub fn pop_comment(&self) {
        self.inner_mut().comment_stack.pop();
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
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.declare_ciphertext_input(4);
    /// let blocks = builder.split_ciphertext(&ct);
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

    /// Consumes the builder and returns the optimized IR graph.
    ///
    /// Finalizes the circuit by running optimization passes — alias elimination, dead-code
    /// elimination, and common subexpression elimination — then returns the resulting IR.
    /// This is typically the last step after declaring all inputs, emitting operations, and
    /// declaring outputs.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let input = builder.declare_ciphertext_input(8);
    /// // ... build circuit ...
    /// builder.declare_ciphertext_output(&input);
    /// let ir = builder.into_ir();
    /// ```
    pub fn into_ir(self) -> IR<IopLang> {
        let mut ir = self.inner.into_inner().ir;
        eliminate_aliases(&mut ir);
        skip_store_load(&mut ir);
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
    /// Unlike [`into_ir`](Self::into_ir), this does not consume the builder and does not
    /// apply any optimization passes. Useful for debugging and inspection mid-construction.
    pub fn ir(&self) -> Ref<'_, IR<IopLang>> {
        Ref::map(self.inner(), |inner| &inner.ir)
    }

    /// Prints the current IR to stdout and panics.
    ///
    /// This is a debugging helper intended for use during circuit development. It dumps a
    /// human-readable representation of the unoptimized IR graph, then unconditionally
    /// panics to halt execution.
    ///
    /// # Panics
    ///
    /// Always panics after printing.
    pub fn dump_and_panic(&self) {
        println!(
            "{:#}",
            self.ir()
                .format()
                .with_walker(PrintWalker::Linear)
                .show_comments(true)
                .show_types(false)
        );
        panic!()
    }

    /// Interprets the current IR with the given inputs, prints the annotated result, and panics.
    ///
    /// Like [`dump_and_panic`](Self::dump_and_panic), but first runs the IR interpreter so
    /// each node is annotated with its computed value. The `inputs` slice must match the
    /// declared input signature in order and length. The ciphertext spec used for
    /// interpretation is inferred from the maximum `int_size` among the ciphertext inputs.
    ///
    /// # Panics
    ///
    /// Always panics after printing, regardless of whether interpretation succeeded.
    pub fn dump_eval_and_panic(&self, inputs: impl AsRef<[IopValue]>) {
        let max_int_size = inputs
            .as_ref()
            .iter()
            .filter_map(|a| match a {
                IopValue::Ciphertext(ciphertext) => Some(ciphertext.spec().int_size()),
                _ => None,
            })
            .max()
            .unwrap();
        let context = IopInterepreterContext {
            spec: self.spec.ciphertext_spec(max_int_size),
            inputs: inputs.as_ref().iter().cloned().enumerate().collect(),
            outputs: FastMap::new(),
        };
        let ir = self.ir();
        match ir.interpret(context) {
            Ok((interpreted, _)) => {
                println!(
                    "{}",
                    interpreted
                        .format()
                        .with_walker(PrintWalker::Linear)
                        .show_comments(true)
                        .show_types(false)
                        .show_val_ann_alternate(true)
                );
                panic!("dump_eval_panic: interpretation succeeded")
            }
            Err((partial, _)) => {
                println!(
                    "{}",
                    partial
                        .format()
                        .with_walker(PrintWalker::Linear)
                        .show_comments(true)
                        .show_types(false)
                        .show_val_ann_alternate(true)
                );
                panic!("dump_eval_panic: interpretation failed")
            }
        }
    }

    /// Interprets the current IR with the given inputs and returns the output values.
    ///
    /// Runs the IR interpreter on the unoptimized graph with the provided `inputs`, which
    /// must match the declared input signature in order and length. Returns the computed
    /// output values in declaration order. The ciphertext spec used for interpretation is
    /// inferred from the maximum `int_size` among the ciphertext inputs.
    ///
    /// This is useful for validating circuit correctness without running actual FHE
    /// operations. Construct input values with the [`make_value`](Ciphertext::make_value)
    /// methods on the handle types.
    ///
    /// # Panics
    ///
    /// Panics if interpretation fails (e.g. due to a malformed graph).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let a = builder.declare_ciphertext_input(8);
    /// let b = builder.declare_ciphertext_input(8);
    /// // ... build circuit ...
    /// let outputs = builder.eval(&[a.make_value(42), b.make_value(7)]);
    /// ```
    pub fn eval(&self, inputs: impl AsRef<[IopValue]>) -> Vec<IopValue> {
        let inputs = inputs.as_ref();
        let max_int_size = inputs
            .iter()
            .filter_map(|a| match a {
                IopValue::Ciphertext(ciphertext) => Some(ciphertext.spec().int_size()),
                _ => None,
            })
            .max()
            .unwrap();
        let context = IopInterepreterContext {
            spec: self.spec.ciphertext_spec(max_int_size),
            inputs: inputs.iter().cloned().enumerate().collect(),
            outputs: FastMap::new(),
        };
        let (_, context) = self.ir().interpret(context).unwrap();
        let mut output: Vec<_> = context.outputs.into_iter().collect();
        output.sort_unstable_by_key(|a| a.0);
        output.into_iter().map(|a| a.1).collect()
    }

    #[cfg(test)]
    pub fn test_random(&self, reps: usize, gen_expect: impl Fn(&[IopValue]) -> Vec<IopValue>) {
        use zhc_utils::iter::CollectInSmallVec;
        for _ in 0..reps {
            use std::panic::AssertUnwindSafe;

            let inputs = self
                .signature()
                .get_args()
                .iter()
                .map(|a| a.random_value())
                .cosvec();
            let expectations = gen_expect(inputs.as_slice());
            let outputs = match std::panic::catch_unwind(AssertUnwindSafe(|| self.eval(&inputs))) {
                Ok(outputs) => outputs,
                Err(_) => {
                    self.dump_eval_and_panic(&inputs);
                    unreachable!()
                }
            };
            if expectations != outputs {
                println!(
                    "Random test failed for input {:?}:\nExpected:\n{:?}\nOutput:\n{:?}",
                    inputs, expectations, outputs
                );
                self.dump_eval_and_panic(inputs);
            }
        }
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
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let input = builder.declare_ciphertext_input(8);
    /// let blocks = builder.split_ciphertext(&input);
    /// ```
    pub fn declare_ciphertext_input(&self, int_size: u16) -> Ciphertext {
        let spec = self.spec.ciphertext_spec(int_size);
        let pos = self.inner_mut().push_arg_type(Type::Ciphertext(spec));
        let (_, inp) = self.inner_mut().add_op(
            IopInstructionSet::Input {
                pos,
                typ: IopTypeSystem::Ciphertext,
            },
            svec![],
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
    pub fn split_ciphertext(&self, inp: impl AsRef<Ciphertext>) -> Vec<CiphertextBlock> {
        let inp = inp.as_ref();
        (0..inp.spec().block_count())
            .map(|index| {
                let (_, ret) = self.inner_mut().add_op(
                    IopInstructionSet::ExtractCtBlock { index },
                    svec![inp.valid],
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
    pub fn declare_plaintext_input(&self, int_size: u16) -> Plaintext {
        let spec = self
            .spec
            .matching_plaintext_block_spec()
            .plaintext_spec(int_size);
        let pos = self.inner_mut().push_arg_type(Type::Plaintext(spec));
        let (_, inp) = self.inner_mut().add_op(
            IopInstructionSet::Input {
                pos,
                typ: IopTypeSystem::Plaintext,
            },
            svec![],
        );
        Plaintext {
            valid: inp[0],
            spec,
        }
    }

    /// Decomposes a [`Plaintext`] into its individual radix blocks.
    ///
    /// Returns one [`PlaintextBlock`] per digit in the radix-decompoosed
    /// representation, ordered from least-significant to most-significant digit. The
    /// length of the returned vector is `int_size / message_size`.
    pub fn split_plaintext(&self, inp: impl AsRef<Plaintext>) -> Vec<PlaintextBlock> {
        let inp = inp.as_ref();
        (0..inp.spec().block_count())
            .map(|index| {
                let (_, ret) = self.inner_mut().add_op(
                    IopInstructionSet::ExtractPtBlock { index },
                    svec![inp.valid],
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
    /// The total integer bit-width of the resulting ciphertext is
    /// `blocks.len() * message_size`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let input = builder.declare_ciphertext_input(8);
    /// let blocks = builder.split_ciphertext(&input);
    /// // ... operate on blocks ...
    /// let ct = builder.join_ciphertext(&blocks);
    /// builder.declare_ciphertext_output(&ct);
    /// ```
    pub fn join_ciphertext(&self, blocks: impl AsRef<[CiphertextBlock]>) -> Ciphertext {
        let blocks = blocks.as_ref();
        let int_size = blocks.len() as u16 * self.spec.message_size() as u16;
        let spec = self.spec.ciphertext_spec(int_size);
        let (_, acc) = self
            .inner_mut()
            .add_op(IopInstructionSet::DeclareCiphertext, svec![]);
        let mut acc = acc[0];
        for (index, block) in blocks.iter().enumerate() {
            let index = index as u8;
            let (_, ret) = self.inner_mut().add_op(
                IopInstructionSet::StoreCtBlock { index },
                svec![block.valid, acc],
            );
            acc = ret[0];
        }
        Ciphertext { valid: acc, spec }
    }

    /// Declares an encrypted integer output for the circuit.
    ///
    /// Registers the ciphertext as a circuit output in the signature and emits the
    /// corresponding IR output instruction. The output is assigned the next positional
    /// index (see [Input / Output Ordering](Self#input--output-ordering)).
    pub fn declare_ciphertext_output(&self, ct: impl AsRef<Ciphertext>) {
        let ct = ct.as_ref();
        let pos = self.inner_mut().push_ret_type(Type::Ciphertext(ct.spec()));
        self.inner_mut().add_op(
            IopInstructionSet::Output {
                pos,
                typ: IopTypeSystem::Ciphertext,
            },
            svec![ct.valid],
        );
    }

    /// Creates a constant [`PlaintextBlock`] with the given message value.
    ///
    /// The `constant` is stored as a message-only plaintext block. Its bit-width must fit
    /// within the builder's message size.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.declare_ciphertext_input(4);
    /// let blocks = builder.split_ciphertext(&ct);
    /// let one = builder.block_const_plaintext(1);
    /// let incremented = builder.block_add_plaintext(&blocks[0], &one);
    /// ```
    pub fn block_const_plaintext(&self, value: u8) -> PlaintextBlock {
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::LetPlaintextBlock { value }, svec![]);
        PlaintextBlock {
            valid: ret[0],
            spec: self.spec.matching_plaintext_block_spec(),
        }
    }

    /// Adds two ciphertext blocks (protect flavor).
    ///
    /// Computes `src_a + src_b` at the block level. Uses protect semantics — see
    /// [Operation Flavors](super#operation-flavors).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.declare_ciphertext_input(4);
    /// let blocks = builder.split_ciphertext(&ct);
    /// let sum = builder.block_add(&blocks[0], &blocks[1]);
    /// ```
    pub fn block_add(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::AddCt, svec![src_a.valid, src_b.valid]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Creates a new IR node that aliases an existing ciphertext block.
    ///
    /// The returned block references the same underlying value but has a distinct IR
    /// node identity. This is useful for debugging purposes.
    pub fn block_alias(&self, src: impl AsRef<CiphertextBlock>) -> CiphertextBlock {
        let src = src.as_ref();
        let (_node, ret) = self.inner_mut().add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![src.valid],
        );
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Adds two ciphertext blocks (temper flavor).
    ///
    /// Computes `src_a + src_b` at the block level. Uses temper semantics — see
    /// [Operation Flavors](super#operation-flavors).
    pub fn block_temper_add(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self.inner_mut().add_op(
            IopInstructionSet::TemperAddCt,
            svec![src_a.valid, src_b.valid],
        );
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Adds two ciphertext blocks (wrapping flavor).
    ///
    /// Computes `src_a + src_b` at the block level. Uses wrapping semantics — see
    /// [Operation Flavors](super#operation-flavors).
    pub fn block_wrapping_add(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self.inner_mut().add_op(
            IopInstructionSet::WrappingAddCt,
            svec![src_a.valid, src_b.valid],
        );
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Adds a plaintext block to a ciphertext block (protect flavor).
    ///
    /// Computes `src_a + src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses protect semantics — see [Operation Flavors](super#operation-flavors).
    pub fn block_add_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::AddPt, svec![src_a.valid, src_b.valid]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Adds a plaintext block to a ciphertext block (wrapping flavor).
    ///
    /// Computes `src_a + src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses wrapping semantics — see [Operation Flavors](super#operation-flavors).
    pub fn block_wrapping_add_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self.inner_mut().add_op(
            IopInstructionSet::WrappingAddPt,
            svec![src_a.valid, src_b.valid],
        );
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Subtracts two ciphertext blocks (protect flavor).
    ///
    /// Computes `src_a - src_b` at the block level. Uses protect semantics — see
    /// [Operation Flavors](super#operation-flavors).
    pub fn block_sub(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::SubCt, svec![src_a.valid, src_b.valid]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Subtracts a plaintext block from a ciphertext block (protect flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is encrypted and `src_b` is plaintext.
    /// Uses protect semantics — see [Operation Flavors](super#operation-flavors).
    pub fn block_sub_plaintext(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<PlaintextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::SubPt, svec![src_a.valid, src_b.valid]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Subtracts a ciphertext block from a plaintext block (protect flavor).
    ///
    /// Computes `src_a - src_b` where `src_a` is plaintext and `src_b` is encrypted.
    /// The result is a ciphertext block. Uses protect semantics — see
    /// [Operation Flavors](super#operation-flavors). Note the reversed operand order
    /// compared to [`block_sub_plaintext`](Self::block_sub_plaintext).
    pub fn block_plaintext_sub(
        &self,
        src_a: impl AsRef<PlaintextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::PtSub, svec![src_a.valid, src_b.valid]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Packs two ciphertext blocks into one.
    ///
    /// Computes `src_a * 2^message_size + src_b`, placing `src_a` in the high (carry)
    /// bits and `src_b` in the low (message) bits of the resulting block. This is the
    /// standard way to pack two blocks to be processed within a single programmable
    /// bootstrapping (PBS) lookup.
    ///
    /// # Panics
    ///
    /// Panics if the builder's `carry_size != message_size`, since packing requires
    /// equal-width carry and message fields.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.declare_ciphertext_input(4);
    /// let blocks = builder.split_ciphertext(&ct);
    /// let packed = builder.block_pack(&blocks[1], &blocks[0]);
    /// let result = builder.block_lookup(&packed, Lut1Def::MsgOnly);
    /// ```
    pub fn block_pack(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
    ) -> CiphertextBlock {
        assert_eq!(self.spec().carry_size(), self.spec().message_size());
        let (src_a, src_b) = (src_a.as_ref(), src_b.as_ref());
        let (_node, ret) = self.inner_mut().add_op(
            IopInstructionSet::PackCt {
                mul: 2u8.pow(self.spec().message_size() as u32),
            },
            svec![src_a.valid, src_b.valid],
        );
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
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
    pub fn block_pack_then_lookup(
        &self,
        src_a: impl AsRef<CiphertextBlock>,
        src_b: impl AsRef<CiphertextBlock>,
        lut: Lut1Def,
    ) -> CiphertextBlock {
        let packed = self.block_pack(src_a, src_b);
        self.block_lookup(&packed, lut)
    }

    /// Applies a single-output programmable bootstrapping (PBS) lookup to a block.
    ///
    /// The `lut` defines the function computed by the bootstrapping. The input block's
    /// full data bits (carry + message) index into the lookup table, and the result is a
    /// fresh ciphertext block with clean noise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::builder::*;
    /// # use zhc_langs::ioplang::Lut1Def;
    /// let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let ct = builder.declare_ciphertext_input(4);
    /// let blocks = builder.split_ciphertext(&ct);
    /// // Extract only the message bits, clearing the carry.
    /// let clean = builder.block_lookup(&blocks[0], Lut1Def::MsgOnly);
    /// ```
    pub fn block_lookup(&self, src: impl AsRef<CiphertextBlock>, lut: Lut1Def) -> CiphertextBlock {
        let src = src.as_ref();
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::Pbs { lut }, svec![src.valid]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Applies a single-output PBS lookup using wrapping (negacyclic) semantics.
    ///
    /// Like [`block_lookup`](Self::block_lookup), but uses wrapping semantics for the
    /// lookup — see [Operation Flavors](super#operation-flavors). This is appropriate
    /// when the input block's padding bit may be set, enabling negacyclic lookup
    /// behavior.
    pub fn block_wrapping_lookup(
        &self,
        src: impl AsRef<CiphertextBlock>,
        lut: Lut1Def,
    ) -> CiphertextBlock {
        let src = src.as_ref();
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::WrappingPbs { lut }, svec![src.valid]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Applies a dual-output programmable bootstrapping (PBS) lookup to a block.
    ///
    /// Like [`block_lookup`](Self::block_lookup), but the bootstrapping produces two
    /// output blocks from a single input. The two lookup functions are defined by the
    /// [`Lut2Def`] variant. This amortizes the cost of a PBS when two related values
    /// need to be extracted simultaneously.
    pub fn block_lookup2(
        &self,
        src: impl AsRef<CiphertextBlock>,
        lut: Lut2Def,
    ) -> (CiphertextBlock, CiphertextBlock) {
        let src = src.as_ref();
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::Pbs2 { lut }, svec![src.valid]);
        (
            CiphertextBlock {
                valid: ret[0],
                spec: self.spec,
            },
            CiphertextBlock {
                valid: ret[1],
                spec: self.spec,
            },
        )
    }

    /// Creates a constant [`CiphertextBlock`] with the given value.
    ///
    /// The `value` is stored as a trivially-encrypted block (zero noise). This is useful
    /// for initializing accumulators or providing constant operands in arithmetic. The
    /// value's bit-width must fit within the block's data bits (carry + message).
    pub fn block_const_ciphertext(&self, value: u8) -> CiphertextBlock {
        let (_node, ret) = self
            .inner_mut()
            .add_op(IopInstructionSet::LetCiphertextBlock { value }, svec![]);
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
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
    pub fn vector_pack_then_lookup(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
    ) -> Vec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .chunk(2)
            .map(|a| match a {
                Chunk::Complete(sv) => {
                    let packed = self.block_pack(sv[1], sv[0]);
                    self.block_lookup(&packed, lut)
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
    pub fn vector_zip_then_lookup(
        &self,
        lhs: impl AsRef<[CiphertextBlock]>,
        rhs: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
        extension: ExtensionBehavior,
    ) -> Vec<CiphertextBlock> {
        let mut output = Vec::new();
        let mut lhs_i = lhs.as_ref().iter();
        let mut rhs_i = rhs.as_ref().iter();
        loop {
            match (&extension, lhs_i.next(), rhs_i.next()) {
                (_, Some(li), Some(ri)) => {
                    let packed = self.block_pack(li, ri);
                    output.push(self.block_lookup(packed, lut))
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
    pub fn vector_lookup(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
    ) -> Vec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_lookup(b, lut))
            .collect()
    }

    /// Applies a dual-output PBS lookup to every block in a slice.
    ///
    /// Maps [`block_lookup2`](Self::block_lookup2) over each element, returning a pair of
    /// output blocks per input block.
    pub fn vector_lookup2(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut2Def,
    ) -> Vec<(CiphertextBlock, CiphertextBlock)> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_lookup2(b, lut))
            .collect()
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
        return output;
    }

    /// Zero-extends a block slice to a given length.
    ///
    /// Pads `inp` with zero-valued constant ciphertext blocks
    /// ([`block_const_ciphertext(0)`](Self::block_const_ciphertext)) until the result
    /// has `size` elements. This implements unsigned integer extension: the original
    /// blocks represent the low-order radix digits, and the appended zeros represent
    /// high-order digits.
    ///
    /// # Panics
    ///
    /// Panics if `inp.len() > size`.
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
            .chain(repeat_n(self.block_const_ciphertext(0), size - inp.len()))
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
