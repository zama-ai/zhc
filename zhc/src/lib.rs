pub use zhc_builder as builder;
pub use zhc_crypto as crypto;
pub use zhc_ir as ir;
pub use zhc_langs as langs;
pub use zhc_pipeline as pipeline;
pub use zhc_sim as sim;
pub use zhc_utils as utils;

/// Convenience re-exports for common ZHC usage patterns.
///
/// This prelude provides the most commonly used types and traits for building and analyzing
/// FHE circuits. Import with `use zhc::prelude::*` to get started quickly.
pub mod prelude {
    use std::path::Path;

    pub use zhc_builder::Builder;
    pub use zhc_crypto::integer_semantics::CiphertextBlockSpec;
    use zhc_ir::visualization::{StyleModifier, VisualAnnotation};
    pub use zhc_langs::ioplang::IopValue;
    pub use zhc_langs::ioplang::{Lut1Def, Lut2Def};
    use zhc_pipeline::gpu_metrics::GpuMetrics;
    use zhc_pipeline::hpu_metrics::HpuMetrics;
    use zhc_pipeline::pbs_metrics::PbsMetrics;
    use zhc_pipeline::{
        compute_gpu_metrics, compute_hpu_metrics, compute_latency, compute_pbs_metrics, draw_slack,
        trace_execution,
    };
    use zhc_sim::MHz;
    use zhc_sim::hpu::HpuConfig;
    pub use zhc_utils::Dumpable;
    use zhc_utils::graphics::ColorScale;
    use zhc_utils::svec;

    /// Extension trait providing HPU analysis methods on [`Builder`].
    ///
    /// These methods use default HPU configuration and clock frequency (400 MHz).
    pub trait BuilderExt {
        /// Computes the estimated HPU execution latency in microseconds.
        fn compute_hpu_latency(&self) -> f64;

        /// Writes an execution trace to a Perfetto-compatible JSON file.
        fn trace_hpu_execution(&self, path: impl AsRef<Path>);

        /// Computes PBS-level metrics (count, critical path, slack distribution).
        fn compute_pbs_metrics(&self) -> PbsMetrics;

        /// Computes HPU-level metrics (latency, efficiency, batch statistics).
        fn compute_hpu_metrics(&self) -> HpuMetrics;

        /// Computes GPU-level metrics (batch statistics).
        fn compute_gpu_metrics(&self, optimal_batch_size: usize) -> GpuMetrics;

        /// Renders a slack heatmap of the IR as an interactive HTML file.
        ///
        /// See [`draw_slack()`] for details.
        fn draw_slack(&self, path: impl AsRef<Path>);

        fn draw_affinity(&self, path: impl AsRef<Path>);
    }

    impl BuilderExt for Builder {
        fn compute_hpu_latency(&self) -> f64 {
            compute_latency(self, HpuConfig::default(), MHz::default())
        }

        fn trace_hpu_execution(&self, path: impl AsRef<Path>) {
            trace_execution(self, HpuConfig::default(), path);
        }

        fn compute_pbs_metrics(&self) -> PbsMetrics {
            compute_pbs_metrics(self)
        }

        fn compute_hpu_metrics(&self) -> HpuMetrics {
            compute_hpu_metrics(self)
        }

        fn compute_gpu_metrics(&self, optimal_batch_size: usize) -> GpuMetrics {
            compute_gpu_metrics(self, optimal_batch_size)
        }

        fn draw_slack(&self, path: impl AsRef<Path>) {
            draw_slack(self, path);
        }

        fn draw_affinity(&self, path: impl AsRef<Path>) {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Affinity {
                PBS = 0,
                MEM = 1,
                ALU = 2,
                CTL = 3,
            }
            impl VisualAnnotation for Affinity {
                fn style_modifier(&self) -> Option<StyleModifier> {
                    Some(StyleModifier {
                        fill_color: Some(ColorScale::RAINBOW.interpolate(*self as u32 as f64 / 4.)),
                        ..Default::default()
                    })
                }
            }
            use zhc_langs::ioplang::IopInstructionSet::*;
            self.ir().backward_dataflow_analysis(|op| {
                let aff = match op.get_instruction() {
                    InputCiphertext { .. }
                    | InputPlaintext { .. }
                    | OutputCiphertext { .. }
                    | _Consume { .. }
                    | Inspect { .. }
                    | DeclareCiphertext { .. }
                    | LetPlaintextBlock { .. }
                    | LetCiphertextBlock { .. } => Affinity::CTL,
                    AddCt
                    | WrappingAddCt
                    | TemperAddCt
                    | SubCt
                    | WrappingSubCt
                    | PackCt { .. }
                    | AddPt
                    | WrappingAddPt
                    | SubPt
                    | PtSub
                    | MulPt => Affinity::ALU,
                    ExtractCtBlock { .. } | ExtractPtBlock { .. } | StoreCtBlock { .. } => {
                        Affinity::MEM
                    }
                    Pbs { .. } | Pbs2 { .. } => Affinity::PBS,
                };
                (aff, svec![(); op.get_return_arity()])
            }).draw_to_html(Some(self.hierarchy()), path);
        }
    }
}

#[cfg(test)]
mod test {
    use zhc_builder::CiphertextSpec;
    use zhc_pipeline::compat::Iop;
    use crate::prelude::BuilderExt;

    #[test]
    fn testngjdsabngkjdsa() {
        Iop::Mul
            .to_builder(CiphertextSpec::new(8, 2, 2))
            .draw_affinity("affinity.html");

    }
}
