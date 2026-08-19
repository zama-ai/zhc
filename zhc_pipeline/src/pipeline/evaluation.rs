use zhc_ir::evaluation::{Evaluable, EvaluatesTo};
use zhc_langs::{
    doplang::emit_assembly,
    pipelinelang::{PipelineInstructionSet, PipelineTypeSystem},
};
use zhc_profiling::{interval_begin, interval_end};
use zhc_utils::{
    Dumpable,
    files::{Extension, FileHandle, PerfettoTrace},
    small::SmallVec,
    svec,
};

use crate::{SchedPolicy, hpu, misc::{self, check_noise}, multi_hpu, pipeline::context::PipelineContext, vm};

use super::PipelineArtifact;

impl EvaluatesTo<PipelineArtifact> for PipelineTypeSystem {
    fn type_of(interp: &PipelineArtifact) -> Self {
        match interp {
            PipelineArtifact::Builder(_) => PipelineTypeSystem::Builder,
            PipelineArtifact::UncheckedIopLang(_) => PipelineTypeSystem::UncheckedIopLang,
            PipelineArtifact::IopLang(_) => PipelineTypeSystem::IopLang,
            PipelineArtifact::HpuConfig(_) => PipelineTypeSystem::HpuConfig,
            PipelineArtifact::HpuLangTranslated(_) => PipelineTypeSystem::HpuLangTranslated,
            PipelineArtifact::HpuLangScheduled(_) => PipelineTypeSystem::HpuLangScheduled,
            PipelineArtifact::DopLang(_) => PipelineTypeSystem::DopLang,
            PipelineArtifact::HpuStream(_) => PipelineTypeSystem::HpuStream,
            PipelineArtifact::PbsMetrics(_) => PipelineTypeSystem::PbsMetrics,
            PipelineArtifact::HpuMetrics(_) => PipelineTypeSystem::HpuMetrics,
            PipelineArtifact::SlackDrawing(_) => PipelineTypeSystem::SlackDrawing,
            PipelineArtifact::HpuTrace(_) => PipelineTypeSystem::HpuTrace,
            PipelineArtifact::Partitions(_) => PipelineTypeSystem::Partitions,
            PipelineArtifact::CiphertextBlockSpec(_) => PipelineTypeSystem::CiphertextBlockSpec,
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
                interval_begin(c"InputBuilder", 0);
                let builder = context.builder.clone().unwrap();
                let result = svec![PipelineArtifact::Builder(builder)];
                interval_end(c"InputBuilder", 0);
                result
            }
            PipelineInstructionSet::InputHpuConfig => {
                interval_begin(c"InputHpuConfig", 0);
                let config = context.hpu_config.clone().unwrap();
                let result = svec![PipelineArtifact::HpuConfig(config)];
                interval_end(c"InputHpuConfig", 0);
                result
            }
            PipelineInstructionSet::BuilderToUncheckedIopLang => {
                interval_begin(c"BuilderToUncheckedIopLang", 0);
                let builder = arguments[0].unwrap_builder_ref();
                let unchecked = builder.optimize_ir();
                let result = svec![PipelineArtifact::UncheckedIopLang(unchecked)];
                interval_end(c"BuilderToUncheckedIopLang", 0);
                result
            }
            PipelineInstructionSet::CheckIopLang => {
                interval_begin(c"CheckIopLang", 0);
                let unchecked = arguments[0].unwrap_unchecked_iop_lang_ref();
                let spec = arguments[1].unwrap_ciphertext_block_spec_ref();
                check_noise(unchecked, &spec.matching_plaintext_block_spec());
                let result = svec![PipelineArtifact::IopLang(unchecked.clone())];
                interval_end(c"CheckIopLang", 0);
                result

            }
            PipelineInstructionSet::BuilderToPartitions => {
                interval_begin(c"BuilderToPartitions", 0);
                let builder = arguments[0].unwrap_builder_ref();
                let partitions = builder.partitions(zhc_builder::IrKind::Optimized);
                let result = svec![PipelineArtifact::Partitions(partitions)];
                interval_end(c"BuilderToPartitions", 0);
                result
            }
            PipelineInstructionSet::BuilderToPrototype => {
                interval_begin(c"BuilderToPrototype", 0);
                let builder = arguments[0].unwrap_builder_ref();
                let prototype = builder.signature();
                let result = svec![PipelineArtifact::Prototype(prototype)];
                interval_end(c"BuilderToPrototype", 0);
                result
            }
            PipelineInstructionSet::BuilderToCiphertextBlockSpec => {
                interval_begin(c"BuilderToCiphertextBlockSpec", 0);
                let builder = arguments[0].unwrap_builder_ref();
                let spec = builder.spec().clone();
                let result = svec![PipelineArtifact::CiphertextBlockSpec(spec)];
                interval_end(c"BuilderToCiphertextBlockSpec", 0);
                result
            }
            PipelineInstructionSet::IopLangToHpuLang => {
                interval_begin(c"IopLangToHpuLang", 0);
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let hpulang = hpu::lowering::lower_iop_to_hpu(ioplang);
                let result = svec![PipelineArtifact::HpuLangTranslated(hpulang.output)];
                interval_end(c"IopLangToHpuLang", 0);
                result
            }
            PipelineInstructionSet::ScheduleHpuLang => {
                interval_begin(c"ScheduleHpuLang", 0);
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
                let result = svec![PipelineArtifact::HpuLangScheduled(scheduled)];
                interval_end(c"ScheduleHpuLang", 0);
                result
            }
            PipelineInstructionSet::AllocateDopLang => {
                interval_begin(c"AllocateDopLang", 0);
                let scheduled = arguments[0].unwrap_hpu_lang_scheduled_ref();
                let config = arguments[1].unwrap_hpu_config_ref();
                let allocated = hpu::allocator::allocate_registers(scheduled, config);
                let result = svec![PipelineArtifact::DopLang(allocated)];
                interval_end(c"AllocateDopLang", 0);
                result
            }
            PipelineInstructionSet::GenerateHpuStream => {
                interval_begin(c"GenerateHpuStream", 0);
                let allocated = arguments[0].unwrap_dop_lang_ref();
                let stream = hpu::translation_table::generate_translation_table(allocated);
                let result = svec![PipelineArtifact::HpuStream(stream)];
                interval_end(c"GenerateHpuStream", 0);
                result
            }
            PipelineInstructionSet::ComputePbsMetrics => {
                interval_begin(c"ComputePbsMetrics", 0);
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let metrics = misc::compute_pbs_metrics(ioplang);
                let result = svec![PipelineArtifact::PbsMetrics(metrics)];
                interval_end(c"ComputePbsMetrics", 0);
                result
            }
            PipelineInstructionSet::ComputeHpuMetrics => {
                interval_begin(c"ComputeHpuMetrics", 0);
                let doplang = arguments[0].unwrap_dop_lang_ref();
                let hpulang = arguments[1].unwrap_hpu_lang_scheduled_ref();
                let metrics = hpu::metrics::compute_hpu_metrics(doplang, hpulang);
                let result = svec![PipelineArtifact::HpuMetrics(metrics)];
                interval_end(c"ComputeHpuMetrics", 0);
                result
            }
            PipelineInstructionSet::TraceHpuExecution => {
                interval_begin(c"TraceHpuExecution", 0);
                let doplang = arguments[0].unwrap_dop_lang_ref();
                let config = arguments[1].unwrap_hpu_config_ref();
                let file = PerfettoTrace::random();
                hpu::tracing::trace_execution(doplang, config, context.hpu_trace_events, &file);
                let result = svec![PipelineArtifact::HpuTrace(file)];
                interval_end(c"TraceHpuExecution", 0);
                result
            }
            PipelineInstructionSet::DrawSlack => {
                interval_begin(c"DrawSlack", 0);
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let file = misc::draw_slack(ioplang);
                let result = svec![PipelineArtifact::SlackDrawing(file)];
                interval_end(c"DrawSlack", 0);
                result
            }
            PipelineInstructionSet::GenerateHpuAssembly => {
                interval_begin(c"GenerateHpuAssembly", 0);
                let doplang = arguments[0].unwrap_dop_lang_ref();
                let file = FileHandle::random(Extension::Asm);
                let asm = emit_assembly(doplang);
                asm.dump_to_file(&file);
                let result = svec![PipelineArtifact::HpuAssembly(file)];
                interval_end(c"GenerateHpuAssembly", 0);
                result
            }
            PipelineInstructionSet::InputMultiHpuConfig => {
                interval_begin(c"InputMultiHpuConfig", 0);
                let config = context.multi_hpu_config.clone().unwrap();
                let result = svec![PipelineArtifact::MultiHpuConfig(config)];
                interval_end(c"InputMultiHpuConfig", 0);
                result
            }
            PipelineInstructionSet::IopLangToMultiHpu => {
                interval_begin(c"IopLangToMultiHpu", 0);
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let partitions = arguments[1].unwrap_partitions_ref();
                let (hpulang, localities) =
                    multi_hpu::translation::lower_iop_to_multi_hpu(ioplang, partitions);
                let result = svec![
                    PipelineArtifact::MultiHpuLangTranslated(hpulang),
                    PipelineArtifact::MultiHpuLocalities(localities)
                ];
                interval_end(c"IopLangToMultiHpu", 0);
                result
            }
            PipelineInstructionSet::ScheduleMultiHpuLang => {
                interval_begin(c"ScheduleMultiHpuLang", 0);
                let hpulang = arguments[0].unwrap_multi_hpu_lang_translated_ref();
                let localities = arguments[1].unwrap_multi_hpu_localities_ref();
                let config = arguments[2].unwrap_multi_hpu_config_ref();
                let scheduled = multi_hpu::scheduler::schedule(
                    hpulang,
                    localities,
                    config,
                    SchedPolicy::AsLateAsPossible,
                );
                let result = svec![PipelineArtifact::MultiHpuLangScheduled(scheduled)];
                interval_end(c"ScheduleMultiHpuLang", 0);
                result
            }
            PipelineInstructionSet::AllocateMultiDopLang => {
                interval_begin(c"AllocateMultiDopLang", 0);
                let scheduled = arguments[0].unwrap_multi_hpu_lang_scheduled_ref();
                let config = arguments[1].unwrap_multi_hpu_config_ref();
                let allocated = scheduled
                    .iter()
                    .map(|ir| hpu::allocator::allocate_registers(ir, &config.hpu_config))
                    .collect();
                let result = svec![PipelineArtifact::MultiDopLang(allocated)];
                interval_end(c"AllocateMultiDopLang", 0);
                result
            }
            PipelineInstructionSet::GenerateMultiHpuStream => {
                interval_begin(c"GenerateMultiHpuStream", 0);
                let allocated = arguments[0].unwrap_multi_dop_lang_ref();
                let streams = allocated
                    .iter()
                    .map(|ir| hpu::translation_table::generate_translation_table(ir))
                    .collect();
                let result = svec![PipelineArtifact::MultiHpuStream(streams)];
                interval_end(c"GenerateMultiHpuStream", 0);
                result
            }
            PipelineInstructionSet::TraceMultiHpuExecution => {
                interval_begin(c"TraceMultiHpuExecution", 0);
                let allocated = arguments[0].unwrap_multi_dop_lang_ref();
                let config = arguments[1].unwrap_multi_hpu_config_ref();
                let file = PerfettoTrace::random();
                multi_hpu::tracing::trace_execution(
                    allocated,
                    config,
                    context.hpu_trace_events,
                    &file,
                );
                let result = svec![PipelineArtifact::MultiHpuTrace(file)];
                interval_end(c"TraceMultiHpuExecution", 0);
                result
            }
            PipelineInstructionSet::GenerateMultiHpuAssembly => {
                interval_begin(c"GenerateMultiHpuAssembly", 0);
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
                let result = svec![PipelineArtifact::MultiHpuAssembly(files)];
                interval_end(c"GenerateMultiHpuAssembly", 0);
                result
            }
            PipelineInstructionSet::InputVmConfig => {
                interval_begin(c"InputVmConfig", 0);
                let config = context.vm_config.clone().unwrap();
                let result = svec![PipelineArtifact::VmConfig(config)];
                interval_end(c"InputVmConfig", 0);
                result
            }
            PipelineInstructionSet::InputTopology => {
                interval_begin(c"InputTopology", 0);
                let topology = context.topology.clone();
                let result = svec![PipelineArtifact::Topology(topology)];
                interval_end(c"InputTopology", 0);
                result
            }
            PipelineInstructionSet::IopLangToVmLang => {
                interval_begin(c"IopLangToVmLang", 0);
                let ioplang = arguments[0].unwrap_iop_lang_ref();
                let vmlang = vm::lowering::lower_iop_to_vm(ioplang);
                let result = svec![PipelineArtifact::VmLang(vmlang)];
                interval_end(c"IopLangToVmLang", 0);
                result
            }
            PipelineInstructionSet::GenerateVmExecutionPlan => {
                interval_begin(c"GenerateVmExecutionPlan", 0);
                let vmlang = arguments[0].unwrap_vm_lang_ref();
                let config = arguments[1].unwrap_vm_config_ref();
                let topology = arguments[2].unwrap_topology_ref();
                let exec = vm::scheduler::schedule(
                    vmlang,
                    config,
                    topology,
                    SchedPolicy::AsSoonAsPossible,
                );
                let result = svec![PipelineArtifact::VmExecutionPlan(exec)];
                interval_end(c"GenerateVmExecutionPlan", 0);
                result
            }
        }
    }
}
