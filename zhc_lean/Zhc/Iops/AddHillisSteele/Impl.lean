import Zhc.IopAlgebra.Class
import Zhc.IopAlgebra.Model.Instance
import Zhc.IopAlgebra.Traced.Instance
import Zhc.Iops.AddRippleCarry.Impl

namespace Zhc.Iops

open Model (SpecBlock SpecInteger CiphertextBlock)

namespace HillisSteele

def computeSize (blockCount : Nat) : Nat :=
  ((blockCount + 3) / 4 * 4).nextPowerOfTwo

def nbStages (nbGroups : Nat) : Nat :=
  nbGroups.log2

def chunk4 {α : Type} : List α → List (α × α × α × α)
  | a :: b :: c :: d :: rest => (a, b, c, d) :: chunk4 rest
  | _ => []

def unchunk4 {α : Type} : List (α × α × α × α) → List α
  | [] => []
  | (a, b, c, d) :: rest => a :: b :: c :: d :: unchunk4 rest

def extractPropGroupLut (p : Fin 4) : Model.Lut1 sb22 :=
  Model.Lut1.fromFn s!"ExtractPropGroup{p}" fun b =>
    let s := b.toCompleteNat
    let status := if s / 4 % 2 = 1 then 2 else if s % 4 = 3 then 1 else 0
    CiphertextBlock.ofNat (status * 2 ^ p.val)

def solvePropGroupFinalLut (p : Fin 3) : Model.Lut1 sb22 :=
  Model.Lut1.fromFn s!"SolvePropGroupFinal{p}" fun b =>
    CiphertextBlock.ofNat (b.toCompleteNat / 2 ^ (p.val + 1) % 2)

def reduceCarryPadLut : Model.Lut1 sb22 :=
  Model.Lut1.fromFn "ReduceCarryPad" fun b =>
    CiphertextBlock.ofNat (if b.toCompleteNat = 15 then 0 else 31)

def solvePropLut : Model.Lut1 sb22 :=
  Model.Lut1.fromFn "SolveProp" fun b =>
    let msb := b.toCompleteNat / 4 % 4
    let lsb := b.toCompleteNat % 4
    CiphertextBlock.ofNat (if msb = 1 then lsb else msb)

def solvePropCarryLut : Model.Lut1 sb22 :=
  Model.Lut1.fromFn "SolvePropCarry" fun b =>
    let msb := b.toCompleteNat / 4 % 4
    let lsb := b.toCompleteNat % 4
    CiphertextBlock.ofNat (if msb = 1 then lsb else msb / 2)

def msgOnlyLut : Model.Lut1 sb22 :=
  Model.Lut1.fromFn "MsgOnly" (·.doMaskMessage)

variable {M : Type → Type} [Monad M] [A : IopAlgebra sb22 M]

def rawSums (si : SpecInteger sb22) (x y : A.CtInteger si) : M (List A.CtBlock) := do
  let sums ← (List.finRange si.blockCount).mapM fun i => do
    let xi ← A.extractCtBlock x i
    let yi ← A.extractCtBlock y i
    A.addCt xi yi
  let zero ← A.letCtBlock 0
  return sums ++ List.replicate (computeSize si.blockCount - si.blockCount) zero

def blockStates (sums : List A.CtBlock) : M (List A.CtBlock) :=
  sums.zipIdx.mapM fun (s, i) => do
    if i = 0 then
      return (← A.pbs2 carryMsgLut s).1
    else if i < 4 then
      A.pbs (extractPropGroupLut ⟨(i - 1) % 4, by omega⟩) s
    else
      A.pbs (extractPropGroupLut ⟨i % 4, by omega⟩) s

def groupStates (states : List A.CtBlock) : M (List A.CtBlock) := do
  let groups ← (chunk4 states).zipIdx.mapM fun ((s0, s1, s2, s3), g) => do
    let b1 ← A.addCt s0 s1
    let b2 ← A.addCt b1 s2
    let b3 ← A.addCt b2 s3
    let b3 ← if g = 0 then
      A.pbs (solvePropGroupFinalLut 2) b3
    else do
      let r ← A.pbs reduceCarryPadLut b3
      let one ← A.letPtBlock 1
      A.addPt r one
    return (s0, b1, b2, b3)
  return unchunk4 groups

def hsStage (stage : Nat) (carries : List A.CtBlock) : M (List A.CtBlock) := do
  let stride := 2 ^ stage
  let solved ← ((carries.drop stride).zip carries).zipIdx.mapM fun ((status, prev), j) => do
    let packed ← A.packCt status 4 prev
    if j < stride then
      A.pbs solvePropCarryLut packed
    else
      A.pbs solvePropLut packed
  return carries.take stride ++ solved

def groupCarries (groups : List A.CtBlock) : M (List A.CtBlock) := do
  let mut carries := (chunk4 groups).map (·.2.2.2)
  for stage in List.range (nbStages carries.length) do
    carries ← hsStage stage carries
  return carries

def finalResolution (groups carries : List A.CtBlock) : M (List A.CtBlock) := do
  let prevs := none :: carries.map some
  let resolved ← (((chunk4 groups).zip carries).zip prevs).mapM
    fun (((s0, s1, s2, _), carry), prev) => do
      match prev with
      | none =>
        let b1 ← A.pbs (solvePropGroupFinalLut 0) s1
        let b2 ← A.pbs (solvePropGroupFinalLut 1) s2
        return (s0, b1, b2, carry)
      | some p =>
        let b0 ← A.pbs (solvePropGroupFinalLut 0) (← A.addCt s0 p)
        let b1 ← A.pbs (solvePropGroupFinalLut 1) (← A.addCt s1 p)
        let b2 ← A.pbs (solvePropGroupFinalLut 2) (← A.addCt s2 p)
        return (b0, b1, b2, carry)
  return unchunk4 resolved

def propagate (sums carries : List A.CtBlock) : M (List A.CtBlock) := do
  match sums with
  | [] => return []
  | s0 :: rest =>
    let m0 := (← A.pbs2 carryMsgLut s0).2
    let added ← (rest.zip carries).mapM fun (s, c) => A.addCt s c
    (m0 :: added).mapM (A.pbs msgOnlyLut)

def joinBlocks (si : SpecInteger sb22) (blocks : List A.CtBlock) : M (A.CtInteger si) := do
  let mut acc ← A.declareInteger si
  for (b, i) in blocks.zip (List.finRange si.blockCount) do
    acc ← A.storeCtBlock acc i b
  return acc

end HillisSteele

open HillisSteele

variable {M : Type → Type} [Monad M] [A : IopAlgebra sb22 M]

def addHillisSteele (si : SpecInteger sb22) (x y : A.CtInteger si) : M (A.CtInteger si) := do
  let sums ← rawSums si x y
  let states ← blockStates sums
  let groups ← groupStates states
  let carries ← groupCarries groups
  let blockCarries ← finalResolution groups carries
  let res ← propagate sums blockCarries
  joinBlocks si res

private def smokeHs (n : Nat) (x y : Nat) : Nat :=
  (addHillisSteele (M := Id) (⟨n⟩ : SpecInteger sb22)
    (Model.CiphertextInteger.ofNat x) (Model.CiphertextInteger.ofNat y)).toNat

#guard smokeHs 4 200 100 == 44
#guard smokeHs 8 60000 10000 == 4464
#guard smokeHs 9 200000 100000 == 300000 % 2 ^ 18
#guard smokeHs 16 4000000000 300000000 == 4300000000 % 2 ^ 32
#guard smokeHs 32 (2 ^ 64 - 1) 1 == 0
#guard smokeHs 32 (2 ^ 63 + 12345678901234) (2 ^ 62 + 98765432109876) ==
  (2 ^ 63 + 12345678901234 + 2 ^ 62 + 98765432109876) % 2 ^ 64

open IopAlgebra.Traced in

def addHillisSteeleCircuit (si : SpecInteger sb22) : IO Ffi.Builder :=
  emit sb22 do
    let x ← inputCtInteger si
    let y ← inputCtInteger si
    let sum ← addHillisSteele (M := TraceM) si x y
    outputCtInteger sum

end Zhc.Iops
