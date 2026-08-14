use zhc_ir::evaluation::{Evaluable, EvaluatesTo};
use zhc_langs::{
    doplang::emit_assembly,
    pipelinelang::{PipelineInstructionSet, PipelineTypeSystem},
};
use zhc_utils::{
    Dumpable,
    files::{Extension, FileHandle, PerfettoTrace},
    small::SmallVec,
    svec,
};

use crate::{SchedPolicy, hpu, misc, multi_hpu, pipeline::context::PipelineContext, vm};

use super::PipelineArtifact;

impl EvaluatesTo<PipelineArtifact> for PipelineTypeSystem {
    fn type_of(interp: &PipelineArtifact) -> Self {
        match interp {
            PipelineArtifact::Builder(_) => PipelineTypeSystem::Builder,
            PipelineArtifact::IopLang(_) => PipelineTypeSystem::IopLang,
            PipelineArtifact::HpuConfig(_) => PipelineTypeSystem::HpuConfig,
            PipelineArtifact::HpuLangTranslated(_) => PipelineTypeSystem::HpuLangTranslated,
            PipelineArtifact::HpuLutPayload(_) => PipelineTypeSystem::HpuLutPayload,
            PipelineArtifact::HpuLangScheduled(_) => PipelineTypeSystem::HpuLangScheduled,
            PipelineArtifact::DopLang(_) => PipelineTypeSystem::DopLang,
            PipelineArtifact::HpuStream(_) => PipelineTypeSystem::HpuStream,
            PipelineArtifact::PbsMetrics(_) => PipelineTypeSystem::PbsMetrics,
            PipelineArtifact::HpuMetrics(_) => PipelineTypeSystem::HpuMetrics,
            PipelineArtifact::SlackDrawing(_) => PipelineTypeSystem::SlackDrawing,
            PipelineArtifact::HpuTrace(_) => PipelineTypeSystem::HpuTrace,
            PipelineArtifact::Partitions(_) => PipelineTypeSystem::Partitions,
            PipelineArtifact::HpuAssembly(_) => PipelineTypeSystem::HpuAssembly,
            PipelineArtifact::MultiHpuConfig(_) => PipelineTypeSystem::MultiHpuConfig,
            PipelineArtifact::MultiHpuLangTranslated(_) => {
                PipelineTypeSystem::MultiHpuLangTranslated
            }
            PipelineArtifact::MultiHpuLocalities(_) => PipelineTypeSystem::MultiHpuLocalities,
            PipelineArtifact::MultiHpuLangScheduled(_) => PipelineTypeSystem::MultiHpuLangScheduled,
            PipelineArtifact::MultiDopLang(_) => PipelineTypeSystem::MultiDopLang,
            PipelineArtifact::MultiHpuTrace(_) => PipelineTypeSystem::MultiHpuTrace,
            PipelineArtifact::MultiHpuStream(_) => PipelineTypeSystem::MultiHpuStream,
            PipelineArtifact::MultiHpuAssembly(_) => PipelineTypeSystem::MultiHpuAssembly,
            PipelineArtifact::Prototype(_) => PipelineTypeSystem::Prototype,
            PipelineArtifact::VmConfig(_) => PipelineTypeSystem::VmConfig,
            PipelineArtifact::Topology(_) => PipelineTypeSystem::Topology,
            PipelineArtifact::VmLang(_) => PipelineTypeSystem::VmLang,
            PipelineArtifact::VmExecutionPlan(_) => PipelineTypeSystem::VmExecutionPlan,
        }
    }
}

impl Evaluable<PipelineArtifact> for PipelineInstructionSet {
    type Context = PipelineContext;

    fn eval(
        &self,
        context: &mut Self::Context,
        arguments: SmallVec<&PipelineArtifact>,
    ) -> SmallVec<PipelineArtifact> {
        match self {
            PipelineInstructionSet::InputBuilder => {
                let builder = context.builder.clone().unwrap();
                svec![PipelineArtifact::Builder(builder)]
            }
            PipelineInstructionSet::InputHpuConfig => {
                let config = context.hpu_config.clone().unwrap();
                svec![PipelineArtifact::HpuConfig(config)]
            }
            PipelineInstructionSet::BuilderToIopLang => {
                let builder = arguments[0].unwrap_builder_ref();
                let ioplang = builder.optimize_ir();
                svec![PipelineArtifact::IopLang(ioplang)]
            }
            PipelineInstructionSet::BuilderToPartitions => {
                let builder = arguments[0].unwrap_builder_ref();
                let partitions = builder.partitions(zhc_builder::IrKind::Optimized);
                svec![PipelineArtifact::Partitions(partitions)]
            }
            PipelineInstructionSet::BuilderToPrototype => {
                let builder = arguments[0].unwrap_builder_ref();
                let prototype = builder.signature();
                svec![PipelineArtifact::Prototype(prototype)]
            }
            PipelineInstructionSet::IopLangToHpuLang => {
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let lowered = hpu::lowering::lower_iop_to_hpu(ioplang);
                svec![
                    PipelineArtifact::HpuLangTranslated(lowered.translation.output),
                    PipelineArtifact::HpuLutPayload(lowered.lut_payload),
                ]
            }
            PipelineInstructionSet::ScheduleHpuLang => {
                let translated = arguments[0].unwrap_hpu_lang_translated_ref();
                let config = arguments[1].unwrap_hpu_config_ref();
                let scheduled = if context.legacy_hpu_scheduler {
                    hpu::scheduler::legacy::schedule(
                        translated,
                        config,
                        SchedPolicy::AsLateAsPossible,
                        SchedPolicy::AsSoonAsPossible,
                    )
                } else {
                    hpu::scheduler::regular::schedule(
                        translated,
                        config,
                        SchedPolicy::AsLateAsPossible,
                    )
                };
                svec![PipelineArtifact::HpuLangScheduled(scheduled)]
            }
            PipelineInstructionSet::AllocateDopLang => {
                let scheduled = arguments[0].unwrap_hpu_lang_scheduled_ref();
                let config = arguments[1].unwrap_hpu_config_ref();
                let allocated = hpu::allocator::allocate_registers(scheduled, config);
                svec![PipelineArtifact::DopLang(allocated)]
            }
            PipelineInstructionSet::GenerateHpuStream => {
                let allocated = arguments[0].unwrap_dop_lang_ref();
                let stream = hpu::translation_table::generate_translation_table(allocated);
                svec![PipelineArtifact::HpuStream(stream)]
            }
            PipelineInstructionSet::ComputePbsMetrics => {
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let metrics = misc::compute_pbs_metrics(ioplang);
                svec![PipelineArtifact::PbsMetrics(metrics)]
            }
            PipelineInstructionSet::ComputeHpuMetrics => {
                let doplang = arguments[0].unwrap_dop_lang_ref();
                let hpulang = arguments[1].unwrap_hpu_lang_scheduled_ref();
                let metrics = hpu::metrics::compute_hpu_metrics(doplang, hpulang);
                svec![PipelineArtifact::HpuMetrics(metrics)]
            }
            PipelineInstructionSet::TraceHpuExecution => {
                let doplang = arguments[0].unwrap_dop_lang_ref();
                let config = arguments[1].unwrap_hpu_config_ref();
                let file = PerfettoTrace::random();
                hpu::tracing::trace_execution(doplang, config, context.hpu_trace_events, &file);
                svec![PipelineArtifact::HpuTrace(file)]
            }
            PipelineInstructionSet::DrawSlack => {
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let file = misc::draw_slack(ioplang);
                svec![PipelineArtifact::SlackDrawing(file)]
            }
            PipelineInstructionSet::GenerateHpuAssembly => {
                let doplang = arguments[0].unwrap_dop_lang_ref();
                let file = FileHandle::random(Extension::Asm);
                let asm = emit_assembly(doplang);
                asm.dump_to_file(&file);
                svec![PipelineArtifact::HpuAssembly(file)]
            }
            PipelineInstructionSet::InputMultiHpuConfig => {
                let config = context.multi_hpu_config.clone().unwrap();
                svec![PipelineArtifact::MultiHpuConfig(config)]
            }
            PipelineInstructionSet::IopLangToMultiHpu => {
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let partitions = arguments[1].unwrap_partitions_ref();
                let (hpulang, localities) =
                    multi_hpu::translation::lower_iop_to_multi_hpu(ioplang, partitions);
                svec![
                    PipelineArtifact::MultiHpuLangTranslated(hpulang),
                    PipelineArtifact::MultiHpuLocalities(localities)
                ]
            }
            PipelineInstructionSet::ScheduleMultiHpuLang => {
                let hpulang = arguments[0].unwrap_multi_hpu_lang_translated_ref();
                let localities = arguments[1].unwrap_multi_hpu_localities_ref();
                let config = arguments[2].unwrap_multi_hpu_config_ref();
                let scheduled = multi_hpu::scheduler::schedule(
                    hpulang,
                    localities,
                    config,
                    SchedPolicy::AsLateAsPossible,
                );
                svec![PipelineArtifact::MultiHpuLangScheduled(scheduled)]
            }
            PipelineInstructionSet::AllocateMultiDopLang => {
                let scheduled = arguments[0].unwrap_multi_hpu_lang_scheduled_ref();
                let config = arguments[1].unwrap_multi_hpu_config_ref();
                let allocated = scheduled
                    .iter()
                    .map(|ir| hpu::allocator::allocate_registers(ir, &config.hpu_config))
                    .collect();
                svec![PipelineArtifact::MultiDopLang(allocated)]
            }
            PipelineInstructionSet::GenerateMultiHpuStream => {
                let allocated = arguments[0].unwrap_multi_dop_lang_ref();
                let streams = allocated
                    .iter()
                    .map(|ir| hpu::translation_table::generate_translation_table(ir))
                    .collect();
                svec![PipelineArtifact::MultiHpuStream(streams)]
            }
            PipelineInstructionSet::TraceMultiHpuExecution => {
                let allocated = arguments[0].unwrap_multi_dop_lang_ref();
                let config = arguments[1].unwrap_multi_hpu_config_ref();
                let file = PerfettoTrace::random();
                multi_hpu::tracing::trace_execution(
                    allocated,
                    config,
                    context.hpu_trace_events,
                    &file,
                );
                svec![PipelineArtifact::MultiHpuTrace(file)]
            }
            PipelineInstructionSet::GenerateMultiHpuAssembly => {
                let allocated = arguments[0].unwrap_multi_dop_lang_ref();
                let files = allocated
                    .iter()
                    .map(|ir| {
                        let file = FileHandle::random(Extension::Asm);
                        let asm = emit_assembly(ir);
                        asm.dump_to_file(&file);
                        file
                    })
                    .collect();
                svec![PipelineArtifact::MultiHpuAssembly(files)]
            }
            PipelineInstructionSet::InputVmConfig => {
                let config = context.vm_config.clone().unwrap();
                svec![PipelineArtifact::VmConfig(config)]
            }
            PipelineInstructionSet::InputTopology => {
                let topology = context.topology.clone();
                svec![PipelineArtifact::Topology(topology)]
            }
            PipelineInstructionSet::IopLangToVmLang => {
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let vmlang = vm::lowering::lower_iop_to_vm(ioplang);
                svec![PipelineArtifact::VmLang(vmlang)]
            }
            PipelineInstructionSet::GenerateVmExecutionPlan => {
                let vmlang = arguments[0].unwrap_vm_lang_ref();
                let config = arguments[1].unwrap_vm_config_ref();
                let topology = arguments[2].unwrap_topology_ref();
                let exec = vm::scheduler::schedule(
                    vmlang,
                    config,
                    topology,
                    SchedPolicy::AsSoonAsPossible,
                );
                svec![PipelineArtifact::VmExecutionPlan(exec)]
            }
        }
    }
}
