namespace Zhc.Ffi

opaque BuilderPointed : NonemptyType
def Builder : Type := BuilderPointed.type
instance : Nonempty Builder := BuilderPointed.property

opaque CiphertextBlockPointed : NonemptyType
def CiphertextBlock : Type := CiphertextBlockPointed.type
instance : Nonempty CiphertextBlock := CiphertextBlockPointed.property

opaque PlaintextBlockPointed : NonemptyType
def PlaintextBlock : Type := PlaintextBlockPointed.type
instance : Nonempty PlaintextBlock := PlaintextBlockPointed.property

opaque BoolCiphertextPointed : NonemptyType
def BoolCiphertext : Type := BoolCiphertextPointed.type
instance : Nonempty BoolCiphertext := BoolCiphertextPointed.property

opaque IntegerCiphertextPointed : NonemptyType
def IntegerCiphertext : Type := IntegerCiphertextPointed.type
instance : Nonempty IntegerCiphertext := IntegerCiphertextPointed.property

opaque IntegerPlaintextPointed : NonemptyType
def IntegerPlaintext : Type := IntegerPlaintextPointed.type
instance : Nonempty IntegerPlaintext := IntegerPlaintextPointed.property

opaque Lut1Pointed : NonemptyType
def Lut1 : Type := Lut1Pointed.type
instance : Nonempty Lut1 := Lut1Pointed.property

opaque Lut2Pointed : NonemptyType
def Lut2 : Type := Lut2Pointed.type
instance : Nonempty Lut2 := Lut2Pointed.property

opaque Lut4Pointed : NonemptyType
def Lut4 : Type := Lut4Pointed.type
instance : Nonempty Lut4 := Lut4Pointed.property

opaque Lut8Pointed : NonemptyType
def Lut8 : Type := Lut8Pointed.type
instance : Nonempty Lut8 := Lut8Pointed.property

opaque FileHandlePointed : NonemptyType
def FileHandle : Type := FileHandlePointed.type
instance : Nonempty FileHandle := FileHandlePointed.property

opaque PerfettoTracePointed : NonemptyType
def PerfettoTrace : Type := PerfettoTracePointed.type
instance : Nonempty PerfettoTrace := PerfettoTracePointed.property

opaque PipelinePointed : NonemptyType
def Pipeline : Type := PipelinePointed.type
instance : Nonempty Pipeline := PipelinePointed.property

opaque HpuMetricsPointed : NonemptyType
def HpuMetrics : Type := HpuMetricsPointed.type
instance : Nonempty HpuMetrics := HpuMetricsPointed.property

opaque MultiHpuMetricsPointed : NonemptyType
def MultiHpuMetrics : Type := MultiHpuMetricsPointed.type
instance : Nonempty MultiHpuMetrics := MultiHpuMetricsPointed.property

opaque PbsMetricsPointed : NonemptyType
def PbsMetrics : Type := PbsMetricsPointed.type
instance : Nonempty PbsMetrics := PbsMetricsPointed.property

@[extern "zhc_lean_builder_debug"]
opaque Builder.debug (self : @& Builder) : IO String
@[extern "zhc_lean_builder_dump"]
opaque Builder.dump (self : @& Builder) : IO String
@[extern "zhc_lean_ciphertext_block_debug"]
opaque CiphertextBlock.debug (self : @& CiphertextBlock) : IO String
@[extern "zhc_lean_plaintext_block_debug"]
opaque PlaintextBlock.debug (self : @& PlaintextBlock) : IO String
@[extern "zhc_lean_bool_ciphertext_debug"]
opaque BoolCiphertext.debug (self : @& BoolCiphertext) : IO String
@[extern "zhc_lean_integer_ciphertext_debug"]
opaque IntegerCiphertext.debug (self : @& IntegerCiphertext) : IO String
@[extern "zhc_lean_integer_plaintext_debug"]
opaque IntegerPlaintext.debug (self : @& IntegerPlaintext) : IO String
@[extern "zhc_lean_lut1_debug"]
opaque Lut1.debug (self : @& Lut1) : IO String
@[extern "zhc_lean_lut1_dump"]
opaque Lut1.dump (self : @& Lut1) : IO String
@[extern "zhc_lean_lut2_debug"]
opaque Lut2.debug (self : @& Lut2) : IO String
@[extern "zhc_lean_lut2_dump"]
opaque Lut2.dump (self : @& Lut2) : IO String
@[extern "zhc_lean_lut4_debug"]
opaque Lut4.debug (self : @& Lut4) : IO String
@[extern "zhc_lean_lut4_dump"]
opaque Lut4.dump (self : @& Lut4) : IO String
@[extern "zhc_lean_lut8_debug"]
opaque Lut8.debug (self : @& Lut8) : IO String
@[extern "zhc_lean_lut8_dump"]
opaque Lut8.dump (self : @& Lut8) : IO String
@[extern "zhc_lean_file_handle_debug"]
opaque FileHandle.debug (self : @& FileHandle) : IO String
@[extern "zhc_lean_perfetto_trace_debug"]
opaque PerfettoTrace.debug (self : @& PerfettoTrace) : IO String
@[extern "zhc_lean_pipeline_debug"]
opaque Pipeline.debug (self : @& Pipeline) : IO String
@[extern "zhc_lean_hpu_metrics_debug"]
opaque HpuMetrics.debug (self : @& HpuMetrics) : IO String
@[extern "zhc_lean_hpu_metrics_dump"]
opaque HpuMetrics.dump (self : @& HpuMetrics) : IO String
@[extern "zhc_lean_multi_hpu_metrics_debug"]
opaque MultiHpuMetrics.debug (self : @& MultiHpuMetrics) : IO String
@[extern "zhc_lean_multi_hpu_metrics_dump"]
opaque MultiHpuMetrics.dump (self : @& MultiHpuMetrics) : IO String
@[extern "zhc_lean_pbs_metrics_debug"]
opaque PbsMetrics.debug (self : @& PbsMetrics) : IO String
@[extern "zhc_lean_pbs_metrics_dump"]
opaque PbsMetrics.dump (self : @& PbsMetrics) : IO String


@[extern "zhc_lean_hpu_metrics_latency"]
opaque HpuMetrics.latency (self : @& HpuMetrics) : IO Float
@[extern "zhc_lean_hpu_metrics_lower_bound"]
opaque HpuMetrics.lowerBound (self : @& HpuMetrics) : IO Float
@[extern "zhc_lean_hpu_metrics_batching_overhead"]
opaque HpuMetrics.batchingOverhead (self : @& HpuMetrics) : IO Float
@[extern "zhc_lean_hpu_metrics_starvation"]
opaque HpuMetrics.starvation (self : @& HpuMetrics) : IO Float
@[extern "zhc_lean_hpu_metrics_batch_count"]
opaque HpuMetrics.batchCount (self : @& HpuMetrics) : IO USize
@[extern "zhc_lean_hpu_metrics_slots_filled"]
opaque HpuMetrics.slotsFilled (self : @& HpuMetrics) : IO USize
@[extern "zhc_lean_hpu_metrics_slots_total"]
opaque HpuMetrics.slotsTotal (self : @& HpuMetrics) : IO USize
@[extern "zhc_lean_hpu_metrics_timeout_launches"]
opaque HpuMetrics.timeoutLaunches (self : @& HpuMetrics) : IO UInt16
@[extern "zhc_lean_multi_hpu_metrics_latency"]
opaque MultiHpuMetrics.latency (self : @& MultiHpuMetrics) : IO Float
@[extern "zhc_lean_pbs_metrics_count"]
opaque PbsMetrics.count (self : @& PbsMetrics) : IO USize
@[extern "zhc_lean_pbs_metrics_critical_length"]
opaque PbsMetrics.criticalLength (self : @& PbsMetrics) : IO USize

inductive Flavor where
  | protect
  | temper
  | wrapping
  deriving Repr, DecidableEq

structure LookupCheck where
  allowInputPadding : Bool
  allowIndexBits : Bool
  allowOutputPadding : Bool
  deriving Repr, DecidableEq

def LookupCheck.protect : LookupCheck := ⟨false, false, false⟩

def LookupCheck.permissive : LookupCheck := ⟨true, true, true⟩

inductive IrKind where
  | original
  | optimized
  deriving Repr, DecidableEq

structure BlockSpec where
  carry : UInt8
  message : UInt8
  deriving Repr, DecidableEq

structure HpuConfig where
  freq : USize
  iscDepth : USize
  iscQueryPeriod : USize
  memFifoCapacity : USize
  memReadLatency : USize
  memWriteLatency : USize
  aluFifoCapacity : USize
  aluReadLatency : USize
  aluWriteLatency : USize
  pbsFifoCapacity : USize
  pbsMemoryCapacity : USize
  pbsMinBatchSize : USize
  pbsMaxBatchSize : USize
  pbsTimeout : USize
  pbsLoadUnloadLatency : USize
  pbsProcessingLatencyA : USize
  pbsProcessingLatencyB : USize
  pbsProcessingLatencyM : USize
  regfSize : USize
  heapSize : USize
  deriving Repr, DecidableEq

structure MultiHpuConfig where
  hpuConfig : HpuConfig
  nHpus : UInt8
  deriving Repr, DecidableEq

structure VmConfig where
  lweDim : USize
  bskPolynomialSize : USize
  bskGlweDim : USize
  bskDecLevels : USize
  bskDecBaseLog : USize
  kskDecLevels : USize
  kskDecBaseLog : USize
  delta : USize
  carrySize : USize
  messageSize : USize
  regfSize : USize
  deriving Repr, DecidableEq

@[extern "zhc_lean_hpu_config_default"]
opaque HpuConfig.default : IO HpuConfig

@[extern "zhc_lean_multi_hpu_config_default"]
opaque MultiHpuConfig.default : IO MultiHpuConfig

@[extern "zhc_lean_file_handle_open"]
opaque FileHandle.open (self : @& FileHandle) : IO Unit

@[extern "zhc_lean_file_handle_move_to"]
opaque FileHandle.moveTo (self : @& FileHandle) (path : @& String) : IO Unit

@[extern "zhc_lean_perfetto_trace_open"]
opaque PerfettoTrace.open (self : @& PerfettoTrace) : IO Unit

end Zhc.Ffi
