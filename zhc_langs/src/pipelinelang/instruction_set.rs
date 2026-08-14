use std::fmt::{Debug, Display};
use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature, sig};

use crate::pipelinelang::Affinity;

use super::PipelineTypeSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineInstructionSet {
    // Commons
    InputBuilder,
    BuilderToIopLang,
    BuilderToPartitions,
    BuilderToPrototype,
    ComputePbsMetrics,
    DrawSlack,
    // Hpu
    InputHpuConfig,
    IopLangToHpuLang,
    ScheduleHpuLang,
    AllocateDopLang,
    GenerateHpuStream,
    ComputeHpuMetrics,
    TraceHpuExecution,
    GenerateHpuAssembly,
    // MultiHpu
    InputMultiHpuConfig,
    IopLangToMultiHpu,
    ScheduleMultiHpuLang,
    AllocateMultiDopLang,
    GenerateMultiHpuStream,
    TraceMultiHpuExecution,
    GenerateMultiHpuAssembly,
    // Vm
    InputVmConfig,
    InputTopology,
    IopLangToVmLang,
    GenerateVmExecutionPlan,
}

impl PipelineInstructionSet {
    pub fn get_affinity(&self) -> Affinity {
        use PipelineInstructionSet::*;
        match self {
            InputBuilder | BuilderToIopLang | BuilderToPartitions | BuilderToPrototype
            | ComputePbsMetrics | DrawSlack => Affinity::Commons,

            InputHpuConfig | IopLangToHpuLang | ScheduleHpuLang | AllocateDopLang
            | GenerateHpuStream | ComputeHpuMetrics | TraceHpuExecution | GenerateHpuAssembly => {
                Affinity::Hpu
            }

            InputMultiHpuConfig
            | IopLangToMultiHpu
            | ScheduleMultiHpuLang
            | AllocateMultiDopLang
            | GenerateMultiHpuStream
            | TraceMultiHpuExecution
            | GenerateMultiHpuAssembly => Affinity::MultiHpu,

            InputVmConfig | InputTopology | IopLangToVmLang | GenerateVmExecutionPlan => {
                Affinity::Vm
            }
        }
    }
}

impl Format for PipelineInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        use PipelineInstructionSet::*;
        match self {
            InputBuilder => write!(f, "input_builder"),
            InputHpuConfig => write!(f, "input_hpu_config"),
            BuilderToIopLang => write!(f, "builder_to_iop_lang"),
            BuilderToPrototype => write!(f, "builder_to_prototype"),
            ComputePbsMetrics => write!(f, "compute_pbs_metrics"),
            IopLangToHpuLang => write!(f, "ioplang_to_hpulang"),
            ScheduleHpuLang => write!(f, "schedule_hpulang"),
            AllocateDopLang => write!(f, "allocate_doplang"),
            GenerateHpuStream => write!(f, "generate_hpu_stream"),
            ComputeHpuMetrics => write!(f, "compute_hpu_metrics"),
            TraceHpuExecution => write!(f, "trace_hpu_execution"),
            DrawSlack => write!(f, "draw_slack"),
            BuilderToPartitions => write!(f, "builder_to_partitions"),
            GenerateHpuAssembly => write!(f, "generate_hpu_assembly"),
            InputMultiHpuConfig => write!(f, "input_multi_hpu_config"),
            IopLangToMultiHpu => write!(f, "ioplang_to_multi_hpu"),
            ScheduleMultiHpuLang => write!(f, "schedule_multi_hpulang"),
            AllocateMultiDopLang => write!(f, "allocate_multi_doplang"),
            GenerateMultiHpuStream => write!(f, "generate_multi_hpu_stream"),
            TraceMultiHpuExecution => write!(f, "trace_multi_hpu_execution"),
            GenerateMultiHpuAssembly => write!(f, "generate_multi_hpu_assembly"),
            InputVmConfig => write!(f, "input_vm_config"),
            InputTopology => write!(f, "input_topology"),
            IopLangToVmLang => write!(f, "ioplang_to_vmlang"),
            GenerateVmExecutionPlan => write!(f, "generate_vm_execution_plan"),
        }
    }
}

impl Display for PipelineInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DialectInstructionSet for PipelineInstructionSet {
    type TypeSystem = PipelineTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use PipelineInstructionSet::*;
        use PipelineTypeSystem::*;
        match self {
            InputBuilder => sig![() -> (Builder)],
            InputHpuConfig => sig![() -> (HpuConfig)],
            BuilderToIopLang => sig![(Builder) -> (IopLang)],
            BuilderToPrototype => sig![(Builder) -> (Prototype)],
            ComputePbsMetrics => sig![(IopLang) -> (PbsMetrics)],
            IopLangToHpuLang => sig![(IopLang) -> (HpuLangTranslated, HpuLutPayload)],
            ScheduleHpuLang => sig![(HpuLangTranslated, HpuConfig) -> (HpuLangScheduled)],
            AllocateDopLang => sig![(HpuLangScheduled, HpuConfig) -> (DopLang)],
            GenerateHpuStream => sig![(DopLang) -> (HpuStream)],
            ComputeHpuMetrics => sig![(DopLang, HpuLangScheduled) -> (HpuMetrics)],
            TraceHpuExecution => sig![(DopLang, HpuConfig) -> (HpuTrace)],
            DrawSlack => sig![(IopLang) -> (SlackDrawing)],
            BuilderToPartitions => sig![(Builder) -> (Partitions)],
            GenerateHpuAssembly => sig![(DopLang) -> (HpuAssembly)],
            InputMultiHpuConfig => sig![() -> (MultiHpuConfig)],
            IopLangToMultiHpu => {
                sig![(IopLang, Partitions) -> (MultiHpuLangTranslated, MultiHpuLocalities)]
            }
            ScheduleMultiHpuLang => {
                sig![(MultiHpuLangTranslated, MultiHpuLocalities, MultiHpuConfig) -> (MultiHpuLangScheduled)]
            }
            AllocateMultiDopLang => sig![(MultiHpuLangScheduled, MultiHpuConfig) -> (MultiDopLang)],
            GenerateMultiHpuStream => sig![(MultiDopLang) -> (MultiHpuStream)],
            TraceMultiHpuExecution => sig![(MultiDopLang, MultiHpuConfig) -> (MultiHpuTrace)],
            GenerateMultiHpuAssembly => sig![(MultiDopLang) -> (MultiHpuAssembly)],
            InputVmConfig => sig![() -> (VmConfig)],
            InputTopology => sig![() -> (Topology)],
            IopLangToVmLang => sig![(IopLang) -> (VmLang)],
            GenerateVmExecutionPlan => sig![(VmLang, VmConfig, Topology) -> (VmExecutionPlan)],
        }
    }
}
