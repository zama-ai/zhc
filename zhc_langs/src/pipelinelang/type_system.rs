use zhc_ir::DialectTypeSystem;
use zhc_utils::DisplayVariant;

#[derive(DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineTypeSystem {
    // Commons
    Builder,
    IopLang,
    Partitions,
    Prototype,
    PbsMetrics,
    SlackDrawing,
    // Hpu
    HpuConfig,
    HpuLangTranslated,
    HpuLutPayload,
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
