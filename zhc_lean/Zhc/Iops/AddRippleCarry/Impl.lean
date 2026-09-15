import Zhc.IopAlgebra.Class
import Zhc.IopAlgebra.Model.Instance
import Zhc.IopAlgebra.Traced.Instance
import Zhc.Ffi.Pipeline

namespace Zhc.Iops

open Model (SpecBlock SpecInteger)

abbrev sb22 : SpecBlock := ⟨2, 2⟩

def carryMsgLut : Model.Lut2 sb22 :=
  Model.Lut2.fromFn "CarryMsg" (·.doMoveCarryToMessage) (·.doMaskMessage)

variable {M : Type → Type} [Monad M] [A : IopAlgebra sb22 M]

def addRippleCarry (si : SpecInteger sb22) (x y : A.CtInteger si) : M (A.CtInteger si) := do
  let mut acc ← A.declareInteger si
  let mut carry ← A.letCtBlock 0
  for i in List.finRange si.blockCount do
    let xi ← A.extractCtBlock x i
    let yi ← A.extractCtBlock y i
    let s ← A.addCt xi yi
    let s ← A.addCt s carry
    let cm ← A.pbs2 carryMsgLut s
    acc ← A.storeCtBlock acc i cm.2
    carry := cm.1
  return acc

private def smoke8 : Model.CiphertextInteger (sb := sb22) ⟨4⟩ :=
  addRippleCarry (M := Id) ⟨4⟩
    (Model.CiphertextInteger.ofNat 200) (Model.CiphertextInteger.ofNat 100)

example : smoke8.toNat = 44 := by decide
example : smoke8.IsClean := by decide

private def smoke16 : Model.CiphertextInteger (sb := sb22) ⟨8⟩ :=
  addRippleCarry (M := Id) ⟨8⟩
    (Model.CiphertextInteger.ofNat 60000) (Model.CiphertextInteger.ofNat 10000)

example : smoke16.toNat = 4464 := by decide

open IopAlgebra.Traced in

def addRippleCarryCircuit (si : SpecInteger sb22) : IO Ffi.Builder :=
  emit sb22 do
    let x ← inputCtInteger si
    let y ← inputCtInteger si
    let sum ← addRippleCarry (M := TraceM) si x y
    outputCtInteger sum

#eval show IO Unit from do
  let b ← addRippleCarryCircuit (SpecInteger.ofIntSize! 8)
  IO.println (← b.dump)
  let p ← Ffi.Pipeline.new
  p.setBuilder b
  p.setHpuConfig (← Ffi.HpuConfig.default)
  let met ← p.getHpuMetrics
  IO.println s!"lat: {← met.latency} us"


end Zhc.Iops
