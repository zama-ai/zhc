use crate::{
    hpu::{metrics::HpuMetrics, translation_table::DOpRepr},
    misc::PbsMetrics,
    vm::scheduler::VmExecutionPlan,
};
use zhc_builder::{Builder, CiphertextBlockSpec, Type};
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig, vm::VmConfig};
use zhc_crypto::integer_semantics::lut::{LutId, LutRegistry};
use zhc_ir::{
    IR, OpMap, Signature,
    evaluation::Evaluation,
    partition::PartitionId,
    visualization::{DynamicElement, VisualAnnotation},
};
use zhc_langs::{
    doplang::DopLang,
    hpulang::{HpuLang, HpuLocality},
    ioplang::IopLang,
    vmlang::VmLang,
};
use zhc_utils::files::{FileHandle, PerfettoTrace};
use zhc_utils::{existential_enum, topology::Topology};

#[derive(Debug, Clone, PartialEq, Eq)]
#[existential_enum]
pub enum PipelineArtifact {
    // Commons
    Builder(Builder),
    UncheckedIopLang(IR<IopLang>),
    IopLang(IR<IopLang>),
    PbsMetrics(PbsMetrics),
    SlackDrawing(FileHandle),
    Partitions(OpMap<PartitionId>),
    Prototype(Signature<Type>),
    CiphertextBlockSpec(CiphertextBlockSpec),
    LutRegistry(LutRegistry),
    // Hpu
    HpuLutRelocation(Option<Vec<LutId>>),
    HpuConfig(HpuConfig),
    HpuLangTranslated(IR<HpuLang>),
    HpuLangScheduled(IR<HpuLang>),
    DopLang(IR<DopLang>),
    HpuStream(Vec<DOpRepr>),
    HpuMetrics(HpuMetrics),
    HpuTrace(PerfettoTrace),
    HpuAssembly(FileHandle),
    // MultiHpu
    MultiHpuLutRelocation(Option<Vec<LutId>>),
    MultiHpuConfig(MultiHpuConfig),
    MultiHpuLangTranslated(IR<HpuLang>),
    MultiHpuLocalities(OpMap<HpuLocality>),
    MultiHpuLangScheduled(Vec<IR<HpuLang>>),
    MultiDopLang(Vec<IR<DopLang>>),
    MultiHpuTrace(PerfettoTrace),
    MultiHpuStream(Vec<Vec<DOpRepr>>),
    MultiHpuAssembly(Vec<FileHandle>),
    // Vm
    VmConfig(VmConfig),
    Topology(Topology),
    VmLang(IR<VmLang>),
    VmExecutionPlan(VmExecutionPlan),
}

impl From<LutRegistry> for PipelineArtifact {
    fn from(v: LutRegistry) -> Self {
        Self::LutRegistry(v)
    }
}

impl Evaluation for PipelineArtifact {}

impl VisualAnnotation for PipelineArtifact {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}
