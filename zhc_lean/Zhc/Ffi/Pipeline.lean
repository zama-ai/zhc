import Zhc.Ffi.Types

namespace Zhc.Ffi.Pipeline

@[extern "zhc_lean_pipeline_new"]
opaque new : IO Pipeline

@[extern "zhc_lean_pipeline_set_builder"]
opaque setBuilder (self : @& Pipeline) (builder : @& Builder) : IO Unit

@[extern "zhc_lean_pipeline_set_ciphertext_block_spec"]
opaque setCiphertextBlockSpec (self : @& Pipeline) (spec : @& BlockSpec) : IO Unit

@[extern "zhc_lean_pipeline_set_hpu_config"]
opaque setHpuConfig (self : @& Pipeline) (config : @& HpuConfig) : IO Unit

@[extern "zhc_lean_pipeline_set_multi_hpu_config"]
opaque setMultiHpuConfig (self : @& Pipeline) (config : @& MultiHpuConfig) : IO Unit

@[extern "zhc_lean_pipeline_set_vm_config"]
opaque setVmConfig (self : @& Pipeline) (config : @& VmConfig) : IO Unit

@[extern "zhc_lean_pipeline_set_legacy_hpu_scheduler"]
opaque setLegacyHpuScheduler (self : @& Pipeline) : IO Unit

@[extern "zhc_lean_pipeline_set_trace_hpu_events"]
opaque setTraceHpuEvents (self : @& Pipeline) : IO Unit

@[extern "zhc_lean_pipeline_get_ciphertext_block_spec"]
opaque getCiphertextBlockSpec (self : @& Pipeline) : IO BlockSpec

@[extern "zhc_lean_pipeline_get_hpu_config"]
opaque getHpuConfig (self : @& Pipeline) : IO HpuConfig

@[extern "zhc_lean_pipeline_get_multi_hpu_config"]
opaque getMultiHpuConfig (self : @& Pipeline) : IO MultiHpuConfig

@[extern "zhc_lean_pipeline_get_vm_config"]
opaque getVmConfig (self : @& Pipeline) : IO VmConfig

@[extern "zhc_lean_pipeline_get_fingerprint"]
opaque getFingerprint (self : @& Pipeline) : IO UInt64

@[extern "zhc_lean_pipeline_get_hpu_metrics"]
opaque getHpuMetrics (self : @& Pipeline) : IO HpuMetrics

@[extern "zhc_lean_pipeline_get_multi_hpu_metrics"]
opaque getMultiHpuMetrics (self : @& Pipeline) : IO MultiHpuMetrics

@[extern "zhc_lean_pipeline_get_pbs_metrics"]
opaque getPbsMetrics (self : @& Pipeline) : IO PbsMetrics

@[extern "zhc_lean_pipeline_draw_state"]
opaque drawState (self : @& Pipeline) : IO FileHandle

@[extern "zhc_lean_pipeline_get_slack_drawing"]
opaque getSlackDrawing (self : @& Pipeline) : IO FileHandle

@[extern "zhc_lean_pipeline_get_hpu_assembly"]
opaque getHpuAssembly (self : @& Pipeline) : IO FileHandle

@[extern "zhc_lean_pipeline_get_hpu_trace"]
opaque getHpuTrace (self : @& Pipeline) : IO PerfettoTrace

@[extern "zhc_lean_pipeline_get_multi_hpu_trace"]
opaque getMultiHpuTrace (self : @& Pipeline) : IO PerfettoTrace

end Zhc.Ffi.Pipeline
