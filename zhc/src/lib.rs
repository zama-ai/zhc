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
    pub use zhc_langs::ioplang::IopValue;
    pub use zhc_langs::ioplang::{Lut1Def, Lut2Def, Lut4Def, Lut8Def};
    use zhc_pipeline::hpu_metrics::HpuMetrics;
    use zhc_pipeline::pbs_metrics::PbsMetrics;
    use zhc_pipeline::{
        compute_hpu_metrics, compute_latency, compute_pbs_metrics, trace_execution,
    };
    use zhc_sim::MHz;
    use zhc_sim::hpu::HpuConfig;
    pub use zhc_utils::Dumpable;

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
    }
}
