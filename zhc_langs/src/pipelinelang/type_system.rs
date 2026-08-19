use zhc_ir::DialectTypeSystem;
use zhc_utils::DisplayVariant;

/// Type system for the pipeline meta-dialect.
///
/// Each variant names one kind of compilation artifact flowing through the pipeline, grouped by
/// [`Affinity`](super::Affinity) branch: the shared frontend artifacts (the source `Builder`
/// circuit and what is derived from it), the single-HPU branch (its configuration and the
/// successive `HpuLang*`/`DopLang` IR forms down to streams, assembly, metrics, and traces), the
/// multi-HPU branch (the same shapes prefixed `Multi`, plus the `MultiHpuLocalities` placement
/// information), and the VM branch (its configuration, machine `Topology`, `VmLang` IR, and final
/// `VmExecutionPlan`).
#[derive(DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineTypeSystem {
    // Commons
    Builder,
    CiphertextBlockSpec,
    UncheckedIopLang,
    IopLang,
    Partitions,
    Prototype,
    PbsMetrics,
    SlackDrawing,
    NoiseDrawing,
    // Hpu
    HpuConfig,
    HpuLangTranslated,
    HpuLangScheduled,
    DopLang,
    HpuMetrics,
    HpuTrace,
    HpuAssembly,
    HpuStream,
    // Multi-Hpu
    MultiHpuConfig,
    MultiHpuLangTranslated,
    MultiHpuLocalities,
    MultiHpuLangScheduled,
    MultiDopLang,
    MultiHpuTrace,
    MultiHpuStream,
    MultiHpuAssembly,
    // Vm
    VmConfig,
    Topology,
    VmLang,
    VmExecutionPlan,
}

impl DialectTypeSystem for PipelineTypeSystem {}
