import Zhc.BlockAlgebra.Tracer.Instance

/-!
Exhaustive comparison of the Lean block semantics with the zhc emulated semantics, for the
(2, 2) block spec. For each operation, one circuit computes every case with the Tracer, which
keeps the Lean model value next to each handle. The Rust interpreter then runs the circuit, and
the complete bits of every result block are compared.
-/

namespace Zhc.Tests.Semantics

open Model BlockAlgebra.Tracer

abbrev sb : SpecBlock := ⟨2, 2⟩

def blocks : List Nat := List.range (2 ^ sb.completeSize)

structure Case where
  label : String
  expected : Nat

def emit (label : String) (rs : Array (TrCtBlock sb)) : TraceM (Array Case) := do
  let si : SpecInteger sb := ⟨rs.size⟩
  let mut x ← declareInteger si
  let mut cases := #[]
  for h : i in [0:rs.size] do
    x ← storeCtBlock x ⟨i, h.upper⟩ rs[i]
    cases := cases.push ⟨s!"{label} [{i}]", rs[i].model.toCompleteNat⟩
  outputCtInteger x
  return cases

def check (name : String) (prog : TraceM (Array Case)) : IO Unit := do
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.ofModel sb)
  let cases ← prog.run b
  let got ← b.interpret #[] cases.size.toUSize
  let mismatches := (cases.zip got).filter fun (c, g) => c.expected != g.toNat
  IO.println s!"{name}: {cases.size - mismatches.size}/{cases.size} agree"
  for (c, g) in mismatches.toList.take 5 do
    IO.println s!"  {c.label}: lean {c.expected}, rust {g}"

def unary (label : String) (f : TrCtBlock sb → TraceM (Array (TrCtBlock sb))) :
    TraceM (Array Case) := do
  let mut cases := #[]
  for a in blocks do
    cases := cases ++ (← emit s!"{label} {a}" (← f (← letCtBlock a)))
  return cases

def binaryCt (label : String) (f : TrCtBlock sb → TrCtBlock sb → TraceM (TrCtBlock sb)) :
    TraceM (Array Case) := do
  let mut cases := #[]
  for a in blocks do
    for c in blocks do
      cases := cases ++ (← emit s!"{label} {a} {c}" #[← f (← letCtBlock a) (← letCtBlock c)])
  return cases

def binaryPt (label : String) (f : TrCtBlock sb → TrPtBlock sb → TraceM (TrCtBlock sb)) :
    TraceM (Array Case) := do
  let mut cases := #[]
  for a in blocks do
    for c in blocks do
      cases := cases ++ (← emit s!"{label} {a} {c}" #[← f (← letCtBlock a) (← letPtBlock c)])
  return cases

def table (j : Nat) (x : CiphertextBlock sb) : CiphertextBlock sb :=
  CiphertextBlock.ofNat (x.toCompleteNat * (2 * j + 5) + 3 * j + 1)

/-- info: add: 1024/1024 agree -/
#guard_msgs in
#eval check "add" (binaryCt "add" addCt)

/-- info: sub: 1024/1024 agree -/
#guard_msgs in
#eval check "sub" (binaryCt "sub" subCt)

/-- info: addPt: 1024/1024 agree -/
#guard_msgs in
#eval check "addPt" (binaryPt "addPt" addPt)

/-- info: subPt: 1024/1024 agree -/
#guard_msgs in
#eval check "subPt" (binaryPt "subPt" subPt)

/-- info: mulPt: 1024/1024 agree -/
#guard_msgs in
#eval check "mulPt" (binaryPt "mulPt" mulPt)

/-- info: ptSub: 1024/1024 agree -/
#guard_msgs in
#eval check "ptSub" do
  let mut cases := #[]
  for a in blocks do
    for c in blocks do
      cases := cases ++ (← emit s!"ptSub {a} {c}" #[← ptSub (← letPtBlock a) (← letCtBlock c)])
  return cases

/-- info: shl: 160/160 agree -/
#guard_msgs in
#eval check "shl" do
  let mut cases := #[]
  for amount in List.range sb.completeSize do
    cases := cases ++ (← unary s!"shl {amount}" fun a => return #[← shlCt a amount])
  return cases

/-- info: mac: 32768/32768 agree -/
#guard_msgs in
#eval check "mac" do
  let mut cases := #[]
  for mul in blocks do
    cases := cases ++ (← binaryCt s!"mac {mul}" fun a c => packCt a mul c)
  return cases

/-- info: storeGet: 32/32 agree -/
#guard_msgs in
#eval check "storeGet" do
  unary "storeGet" fun a => do
    let x ← declareInteger (sb := sb) ⟨2⟩
    let x ← storeCtBlock x 1 a
    return #[← extractCtBlock x 1]

/-- info: pbs: 32/32 agree -/
#guard_msgs in
#eval check "pbs" do
  let lut := Lut1.fromFn "t" (table 0)
  unary "pbs" fun a => return #[← pbs lut a]

/-- info: pbs2: 64/64 agree -/
#guard_msgs in
#eval check "pbs2" do
  let lut := Lut2.fromFn "t" (table 0) (table 1)
  unary "pbs2" fun a => do
    let (o0, o1) ← pbs2 lut a
    return #[o0, o1]

/-- info: pbs4: 128/128 agree -/
#guard_msgs in
#eval check "pbs4" do
  let lut := Lut4.fromFn "t" (table 0) (table 1) (table 2) (table 3)
  unary "pbs4" fun a => do
    let (o0, o1, o2, o3) ← pbs4 lut a
    return #[o0, o1, o2, o3]

/-- info: pbs8: 256/256 agree -/
#guard_msgs in
#eval check "pbs8" do
  let lut := Lut8.fromFn "t" (table 0) (table 1) (table 2) (table 3) (table 4) (table 5)
    (table 6) (table 7)
  unary "pbs8" fun a => do
    let (o0, o1, o2, o3, o4, o5, o6, o7) ← pbs8 lut a
    return #[o0, o1, o2, o3, o4, o5, o6, o7]

end Zhc.Tests.Semantics
