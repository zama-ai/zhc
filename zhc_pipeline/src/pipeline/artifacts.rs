use crate::{
    hpu::{metrics::HpuMetrics, translation_table::DOpRepr},
    misc::PbsMetrics,
    vm::scheduler::VmExecutionPlan,
};
use zhc_builder::{Builder, Type};
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig, vm::VmConfig};
use zhc_ir::{IR, OpMap, Signature, evaluation::Evaluation, partition::PartitionId};
use zhc_langs::{
    doplang::DopLang,
    hpulang::{HpuLang, HpuLocality, LutId},
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
    IopLang(IR<IopLang>),
    PbsMetrics(PbsMetrics),
    SlackDrawing(FileHandle),
    Partitions(OpMap<PartitionId>),
    Prototype(Signature<Type>),
    // Hpu
    HpuConfig(HpuConfig),
    HpuLangTranslated(IR<HpuLang>),
    HpuLutPayload(Vec<(LutId, Vec<u8>)>),
    HpuLangScheduled(IR<HpuLang>),
    DopLang(IR<DopLang>),
    HpuStream(Vec<DOpRepr>),
    HpuMetrics(HpuMetrics),
    HpuTrace(PerfettoTrace),
    HpuAssembly(FileHandle),
    // MultiHpu
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

impl Evaluation for PipelineArtifact {}
