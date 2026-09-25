import Zhc

/-!
End-to-end check of the FFI: emit the ripple-carry adder through the real builder, then
compile it for one HPU and print the metrics.
-/

open Zhc Zhc.Ffi

def main : IO UInt32 := do
  let sb := Iops.sb22
  let si : Model.SpecInteger sb := ⟨4⟩

  let builder ← Iops.addRippleCarryCircuit si

  let spec ← builder.spec
  IO.println s!"builder spec: carry={spec.carry} message={spec.message}"
  builder.checkNoise

  -- Plain-struct layouts: what goes through the shim must come back unchanged.
  let hpu ← HpuConfig.default
  IO.println s!"default hpu: freq={hpu.freq} regf_size={hpu.regfSize} heap_size={hpu.heapSize}"
  let multi ← MultiHpuConfig.default
  IO.println s!"default multi hpu: n_hpus={multi.nHpus} freq={multi.hpuConfig.freq}"
  if hpu.freq != 400 || hpu.regfSize != 64 || hpu.heapSize != 16384 || multi.nHpus != 4
      || multi.hpuConfig != hpu then
    IO.eprintln "unexpected default config"; return 1

  let pipeline ← Pipeline.new
  pipeline.setBuilder builder
  pipeline.setHpuConfig hpu
  if (← pipeline.getHpuConfig) != hpu then
    IO.eprintln "hpu config did not round-trip"; return 1
  if (← pipeline.getCiphertextBlockSpec) != spec then
    IO.eprintln "block spec did not round-trip"; return 1
  let fp ← pipeline.getFingerprint
  let m ← pipeline.getHpuMetrics
  let latency ← m.latency
  let batches ← m.batchCount
  IO.println s!"fingerprint: {fp}"
  IO.println s!"latency={latency} lower_bound={← m.lowerBound} batches={batches} \
    slots={← m.slotsFilled}/{← m.slotsTotal} timeouts={← m.timeoutLaunches}"
  IO.println (← m.dump)
  IO.println (← (← pipeline.getPbsMetrics).dump)
  IO.println (← pipeline.debug)

  -- A wrong-flow getter must surface as an `IO` error, not kill the process.
  match ← (pipeline.getMultiHpuMetrics : IO MultiHpuMetrics).toBaseIO with
  | .ok _ => IO.eprintln "expected an error from the multi-HPU getter"; return 1
  | .error e => IO.println s!"expected error: {e}"

  if latency > 0.0 && batches > 0 then
    IO.println "ok"
    return 0
  else
    IO.eprintln "metrics look wrong"
    return 1
