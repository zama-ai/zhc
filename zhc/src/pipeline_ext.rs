use zhc_builder::Builder;
use zhc_pipeline::Pipeline;

/// Connects the circuit builder to the compilation pipeline.
pub trait PipelineExt {
    /// Supplies a snapshot of the builder's IR and its metadata.
    ///
    /// The snapshot is taken immediately. Optimization, noise checking and target compilation
    /// remain lazy steps of the pipeline. Later changes through another clone of the builder do
    /// not affect this pipeline.
    ///
    /// ```
    /// use zhc::prelude::*;
    ///
    /// let builder = Builder::new(SpecBlock(2, 2));
    /// let input = builder.ciphertext_integer_input(8);
    /// builder.ciphertext_integer_output(&input);
    /// let mut pipeline = Pipeline::new().with_builder(builder);
    /// assert_eq!(pipeline.get_prototype().get_args_arity(), 1);
    /// ```
    fn with_builder(self, builder: Builder) -> Self;

    /// In-place form of [`with_builder`](Self::with_builder).
    fn set_builder(&mut self, builder: Builder);
}

impl PipelineExt for Pipeline {
    fn with_builder(mut self, builder: Builder) -> Self {
        self.set_builder(builder);
        self
    }

    fn set_builder(&mut self, builder: Builder) {
        self.set_unchecked_ioplang(builder.ir().clone());
        self.set_partitions(builder.partitions());
        self.set_prototype(builder.signature());
        self.set_ciphertext_block_spec(*builder.spec());
    }
}
