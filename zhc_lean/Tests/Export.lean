import Zhc.Ffi.R2l.Macro
import Zhc.Iops.AddRippleCarry.Theorems

/-!
Tests of `zhc_export`: the accepted signatures, the generated declarations, the rejected forms,
and the runtime checks of the exported functions against the real builder.
-/

namespace Zhc.Tests

open Model Iops BlockAlgebra.Tracer

/-! ## Small algorithms used by the tests -/

section Algos

variable {M : Type → Type} [Monad M] [A : BlockAlgebra sb22 M]

def same1 (si : SpecInteger sb22) (x : A.CtInteger si) : M (A.CtInteger si) :=
  pure x

def first2 (si : SpecInteger sb22) (x _y : A.CtInteger si) : M (A.CtInteger si) :=
  pure x

def pickBool (si sj : SpecInteger sb22) (_x : A.CtInteger si) (_p : A.PtInteger sj) (b : A.CtBool)
    (_z : A.CtInteger si) : M A.CtBool :=
  pure b

def freshInt (si sj : SpecInteger sb22) (_x : A.CtInteger si) : M (A.CtInteger sj) :=
  A.declareInteger sj

def pair2 (si : SpecInteger sb22) (x : A.CtInteger si) (b : A.CtBool) :
    M (A.CtInteger si × A.CtBool) :=
  pure (x, b)

def rotate3 (si : SpecInteger sb22) (x y : A.CtInteger si) (b : A.CtBool) :
    M (A.CtInteger si × A.CtBool × A.CtInteger si) :=
  pure (y, b, x)

end Algos

/-! ## Their theorems -/

theorem same1_correct (si : SpecInteger sb22) (x : CiphertextInteger si) :
    x.IsClean → same1 (M := Id) si x = x :=
  fun _ => rfl

theorem same1_clean (si : SpecInteger sb22) (x : CiphertextInteger si) :
    x.IsClean → CiphertextInteger.IsClean (same1 (M := Id) si x) :=
  fun hx => hx

theorem same1_projects (si : SpecInteger sb22) (x : TrCtInteger si) :
    TraceM.ModelIs (same1 (M := TraceM) si x) (same1 (M := Id) si (model x)) :=
  Projects.pure x model

/-- warning: declaration uses `sorry` -/
#guard_msgs in
theorem same1_zero (si : SpecInteger sb22) (x : CiphertextInteger si) :
    x.IsClean → CiphertextInteger.toNat (same1 (M := Id) si x) = 0 :=
  sorry

theorem pickBool_correct (si sj : SpecInteger sb22) (x : CiphertextInteger si)
    (p : PlaintextInteger sj) (b : CiphertextBool sb22) (z : CiphertextInteger si) :
    x.IsClean → p.IsClean → b.IsClean → z.IsClean → pickBool (M := Id) si sj x p b z = b :=
  fun _ _ _ _ => rfl

theorem pickBool_clean (si sj : SpecInteger sb22) (x : CiphertextInteger si)
    (p : PlaintextInteger sj) (b : CiphertextBool sb22) (z : CiphertextInteger si) :
    x.IsClean → p.IsClean → b.IsClean → z.IsClean →
      CiphertextBool.IsClean (pickBool (M := Id) si sj x p b z) :=
  fun _ _ hb _ => hb

theorem pickBool_projects (si sj : SpecInteger sb22) (x : TrCtInteger si) (p : TrPtInteger sj)
    (b : TrCtBool sb22) (z : TrCtInteger si) :
    TraceM.ModelIs (pickBool (M := TraceM) si sj x p b z)
      (pickBool (M := Id) si sj (model x) (model p) (model b) (model z)) :=
  Projects.pure b model

theorem freshInt_correct (si _sj : SpecInteger sb22) (x : CiphertextInteger si) :
    x.IsClean → True :=
  fun _ => trivial

theorem freshInt_clean (si sj : SpecInteger sb22) (x : CiphertextInteger si) :
    x.IsClean → CiphertextInteger.IsClean (freshInt (M := Id) si sj x) :=
  fun _ => CiphertextInteger.isClean_ofNat 0

theorem freshInt_projects (si sj : SpecInteger sb22) (x : TrCtInteger si) :
    TraceM.ModelIs (freshInt (M := TraceM) si sj x) (freshInt (M := Id) si sj (model x)) :=
  Projects.declareInteger sj

theorem pair2_correct (si : SpecInteger sb22) (x : CiphertextInteger si) (b : CiphertextBool sb22) :
    x.IsClean → b.IsClean → pair2 (M := Id) si x b = (x, b) :=
  fun _ _ => rfl

theorem pair2_clean (si : SpecInteger sb22) (x : CiphertextInteger si) (b : CiphertextBool sb22) :
    x.IsClean → b.IsClean →
      CiphertextInteger.IsClean (pair2 (M := Id) si x b).1 ∧
      CiphertextBool.IsClean (pair2 (M := Id) si x b).2 :=
  fun hx hb => ⟨hx, hb⟩

theorem pair2_projects (si : SpecInteger sb22) (x : TrCtInteger si) (b : TrCtBool sb22) :
    TraceM.ModelIs (pair2 (M := TraceM) si x b) (pair2 (M := Id) si (model x) (model b)) :=
  Projects.pure (x, b) model

theorem rotate3_correct (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (b : CiphertextBool sb22) :
    x.IsClean → y.IsClean → b.IsClean → rotate3 (M := Id) si x y b = (y, b, x) :=
  fun _ _ _ => rfl

theorem rotate3_clean (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (b : CiphertextBool sb22) :
    x.IsClean → y.IsClean → b.IsClean →
      CiphertextInteger.IsClean (rotate3 (M := Id) si x y b).1 ∧
      CiphertextBool.IsClean (rotate3 (M := Id) si x y b).2.1 ∧
      CiphertextInteger.IsClean (rotate3 (M := Id) si x y b).2.2 :=
  fun hx hy hb => ⟨hy, hb, hx⟩

theorem rotate3_projects (si : SpecInteger sb22) (x y : TrCtInteger si) (b : TrCtBool sb22) :
    TraceM.ModelIs (rotate3 (M := TraceM) si x y b)
      (rotate3 (M := Id) si (model x) (model y) (model b)) :=
  Projects.pure (y, b, x) model

/-! ## Accepted forms -/

#guard_msgs in
zhc_export add (si : SpecInteger sb22) (x y : CiphertextInteger si) : (r : CiphertextInteger si) where
  spec := r.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  prog := addRippleCarry
  correct := addRippleCarry_correct
  clean := addRippleCarry_isClean
  projects := addRippleCarry_traced_projects

/-- info: add.export : Ffi.L2r.Builder → Ffi.L2r.CiphertextInteger → Ffi.L2r.CiphertextInteger → USize → IO Unit -/
#guard_msgs in
#check @add.export

/-- info: add.run : Ffi.L2r.Builder → Ffi.L2r.CiphertextInteger → Ffi.L2r.CiphertextInteger → IO Ffi.L2r.CiphertextInteger -/
#guard_msgs in
#check @add.run

/-- info: add.spec : (si : SpecInteger sb22) → CiphertextInteger si → CiphertextInteger si → CiphertextInteger si → Prop -/
#guard_msgs in
#check @add.spec

/--
info: add.correct : ∀ (si : SpecInteger sb22) (x y : CiphertextInteger si),
  Clean.isClean x → Clean.isClean y → add.spec si x y (addRippleCarry si x y)
-/
#guard_msgs in
#check @add.correct

/--
info: add.clean : ∀ (si : SpecInteger sb22) (x y : CiphertextInteger si),
  Clean.isClean x → Clean.isClean y → Clean.isClean (addRippleCarry si x y)
-/
#guard_msgs in
#check @add.clean

/--
info: add.projects : ∀ (si : SpecInteger sb22) (x y : TrCtInteger si),
  (addRippleCarry si x y).ModelIs (addRippleCarry si (model x) (model y))
-/
#guard_msgs in
#check @add.projects

/-- info: add.circuit : SpecInteger sb22 → IO Ffi.L2r.Builder -/
#guard_msgs in
#check @add.circuit

#guard_msgs in
zhc_export same (si : SpecInteger sb22) (x : CiphertextInteger si) : (r : CiphertextInteger si) where
  spec := r = x
  prog := same1
  correct := same1_correct
  clean := same1_clean
  projects := same1_projects

#guard_msgs in
zhc_export mixed (si sj : SpecInteger sb22) (x : CiphertextInteger si) (p : PlaintextInteger sj) (b : CiphertextBool)
    (z : CiphertextInteger si) : (r : CiphertextBool) where
  spec := r = b
  prog := pickBool
  correct := pickBool_correct
  clean := pickBool_clean
  projects := pickBool_projects

/--
info: mixed.export : Ffi.L2r.Builder →
  Ffi.L2r.CiphertextInteger →
    Ffi.L2r.PlaintextInteger → Ffi.L2r.CiphertextBool → Ffi.L2r.CiphertextInteger → USize → IO Unit
-/
#guard_msgs in
#check @mixed.export

/--
info: mixed.correct : ∀ (si sj : SpecInteger sb22) (x : CiphertextInteger si) (p : PlaintextInteger sj)
  (b : CiphertextBool sb22) (z : CiphertextInteger si),
  Clean.isClean x →
    Clean.isClean p → Clean.isClean b → Clean.isClean z → mixed.spec si sj x p b z (pickBool si sj x p b z)
-/
#guard_msgs in
#check @mixed.correct

/-- info: mixed.circuit : SpecInteger sb22 → SpecInteger sb22 → IO Ffi.L2r.Builder -/
#guard_msgs in
#check @mixed.circuit

#guard_msgs in
zhc_export fresh (si sj : SpecInteger sb22) (x : CiphertextInteger si) : (_r : CiphertextInteger sj) where
  spec := True
  prog := freshInt
  correct := freshInt_correct
  clean := freshInt_clean
  projects := freshInt_projects

/-- info: fresh.export : Ffi.L2r.Builder → UInt16 → Ffi.L2r.CiphertextInteger → USize → IO Unit -/
#guard_msgs in
#check @fresh.export

/-- info: fresh.circuit : SpecInteger sb22 → SpecInteger sb22 → IO Ffi.L2r.Builder -/
#guard_msgs in
#check @fresh.circuit

#guard_msgs in
zhc_export two (si : SpecInteger sb22) (x : CiphertextInteger si) (b : CiphertextBool) :
    (r : CiphertextInteger si) × (c : CiphertextBool) where
  spec := (r, c) = (x, b)
  prog := pair2
  correct := pair2_correct
  clean := pair2_clean
  projects := pair2_projects

/-- info: two.export : Ffi.L2r.Builder → Ffi.L2r.CiphertextInteger → Ffi.L2r.CiphertextBool → USize → USize → IO Unit -/
#guard_msgs in
#check @two.export

/--
info: two.run : Ffi.L2r.Builder →
  Ffi.L2r.CiphertextInteger → Ffi.L2r.CiphertextBool → IO (Ffi.L2r.CiphertextInteger × Ffi.L2r.CiphertextBool)
-/
#guard_msgs in
#check @two.run

/--
info: two.clean : ∀ (si : SpecInteger sb22) (x : CiphertextInteger si) (b : CiphertextBool sb22),
  Clean.isClean x → Clean.isClean b → Clean.isClean (pair2 si x b)
-/
#guard_msgs in
#check @two.clean

#guard_msgs in
zhc_export three (si : SpecInteger sb22) (x y : CiphertextInteger si) (b : CiphertextBool) :
    (r : CiphertextInteger si) × (c : CiphertextBool) × (s : CiphertextInteger si) where
  spec := (r, c, s) = (y, b, x)
  prog := rotate3
  correct := rotate3_correct
  clean := rotate3_clean
  projects := rotate3_projects

/--
info: three.export : Ffi.L2r.Builder →
  Ffi.L2r.CiphertextInteger → Ffi.L2r.CiphertextInteger → Ffi.L2r.CiphertextBool → USize → USize → USize → IO Unit
-/
#guard_msgs in
#check @three.export

/--
info: three.run : Ffi.L2r.Builder →
  Ffi.L2r.CiphertextInteger →
    Ffi.L2r.CiphertextInteger →
      Ffi.L2r.CiphertextBool → IO (Ffi.L2r.CiphertextInteger × Ffi.L2r.CiphertextBool × Ffi.L2r.CiphertextInteger)
-/
#guard_msgs in
#check @three.run

/--
info: three.clean : ∀ (si : SpecInteger sb22) (x y : CiphertextInteger si) (b : CiphertextBool sb22),
  Clean.isClean x → Clean.isClean y → Clean.isClean b → Clean.isClean (rotate3 si x y b)
-/
#guard_msgs in
#check @three.clean

/-! ## Rejected forms -/

/-- error: zhc_export: all SpecInteger binders must use the same block spec -/
#guard_msgs in
zhc_export bad_two_specs (si : SpecInteger sb22) (sj : SpecInteger ⟨1, 1⟩) (x : CiphertextInteger si) :
    (r : CiphertextInteger sj) where
  spec := r = x
  prog := same1
  correct := same1_correct
  clean := same1_clean
  projects := same1_projects

/-- error: zhc_export: si is not declared: the signature has no (si : SpecInteger _) binder -/
#guard_msgs in
zhc_export bad_no_spec (x : CiphertextInteger si) : (r : CiphertextInteger si) where
  spec := r = x
  prog := same1
  correct := same1_correct
  clean := same1_clean
  projects := same1_projects

/-- error: zhc_export: sj is not declared: the signature has no (sj : SpecInteger _) binder -/
#guard_msgs in
zhc_export bad_out_spec (si : SpecInteger sb22) (x : CiphertextInteger si) :
    (r : CiphertextInteger sj) where
  spec := r = x
  prog := same1
  correct := same1_correct
  clean := same1_clean
  projects := same1_projects

/-- error: zhc_export: the signature needs a (si : SpecInteger sb) binder -/
#guard_msgs in
zhc_export bad_no_integer (b : CiphertextBool) : (r : CiphertextBool) where
  spec := r = b
  prog := same1
  correct := same1_correct
  clean := same1_clean
  projects := same1_projects

/--
error: Type mismatch
  addRippleCarry_isClean si x y
has type
  x.IsClean → y.IsClean → CiphertextInteger.IsClean (addRippleCarry si x y)
but is expected to have type
  Clean.isClean x → Clean.isClean y → spec si x y (addRippleCarry si x y)
-/
#guard_msgs in
zhc_export bad_proof (si : SpecInteger sb22) (x y : CiphertextInteger si) : (r : CiphertextInteger si) where
  spec := r.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  prog := addRippleCarry
  correct := addRippleCarry_isClean
  clean := addRippleCarry_isClean
  projects := addRippleCarry_traced_projects

/--
error: Type mismatch
  addRippleCarry_correct si x y
has type
  x.IsClean → y.IsClean → CiphertextInteger.toNat (addRippleCarry si x y) = (x.toNat + y.toNat) % 2 ^ si.intSize
but is expected to have type
  Clean.isClean x → Clean.isClean y → spec si x y (first2 si x y)
---
error: Type mismatch
  addRippleCarry_isClean si x y
has type
  x.IsClean → y.IsClean → CiphertextInteger.IsClean (addRippleCarry si x y)
but is expected to have type
  Clean.isClean x → Clean.isClean y → Clean.isClean (first2 si x y)
---
error: Type mismatch
  addRippleCarry_traced_projects si x y
has type
  (addRippleCarry si x y).ModelIs (addRippleCarry si x.model y.model)
but is expected to have type
  (first2 si x y).ModelIs (first2 si (model x) (model y))
-/
#guard_msgs in
zhc_export bad_prog(si : SpecInteger sb22) (x y : CiphertextInteger si) : (r : CiphertextInteger si) where
  spec := r.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  prog := first2
  correct := addRippleCarry_correct
  clean := addRippleCarry_isClean
  projects := addRippleCarry_traced_projects

/--
error: Function expected at
  same1 si x
but this term has type
  M (BlockAlgebra.CtInteger M si)

Note: Expected a function because this term is being applied to the argument
  y
---
error: Function expected at
  same1 si x
but this term has type
  Id (BlockAlgebra.CtInteger Id si)

Note: Expected a function because this term is being applied to the argument
  y
---
error: Function expected at
  same1 si x
but this term has type
  Id (BlockAlgebra.CtInteger Id si)

Note: Expected a function because this term is being applied to the argument
  y
---
error: Application type mismatch: The argument
  y
has type
  CiphertextInteger si
of sort `Type` but is expected to have type
  x.IsClean
of sort `Prop` in the application
  same1_clean si x y
---
error: Application type mismatch: The argument
  y
has type
  TrCtInteger si
but is expected to have type
  Ffi.L2r.Builder
in the application
  same1 si x y
---
error: Function expected at
  same1 si (model x)
but this term has type
  Id (BlockAlgebra.CtInteger Id si)

Note: Expected a function because this term is being applied to the argument
  Zhc.BlockAlgebra.Tracer.model✝ y
---
error: Application type mismatch: The argument
  Ffi.R2l.tracedCt si y
has type
  TrCtInteger si
but is expected to have type
  Ffi.L2r.Builder
in the application
  same1 si (Ffi.R2l.tracedCt si x) (Ffi.R2l.tracedCt si y)
---
error: Application type mismatch: The argument
  y
has type
  CiphertextInteger si
of sort `Type` but is expected to have type
  x.IsClean
of sort `Prop` in the application
  same1_correct si x y
---
error: Application type mismatch: The argument
  y
has type
  TrCtInteger si
but is expected to have type
  Ffi.L2r.Builder
in the application
  same1_projects si x y
-/
#guard_msgs in
zhc_export bad_sig (si : SpecInteger sb22) (x y : CiphertextInteger si) : (r : CiphertextInteger si) where
  spec := r = x
  prog := same1
  correct := same1_correct
  clean := same1_clean
  projects := same1_projects

/-! ## The export check -/

/--
error: Zhc.Tests.rogue is exported without Zhc.Tests.rogue.spec: exports must go through zhc_export
---
error: Zhc.Tests.rogue is exported without Zhc.Tests.rogue.correct: exports must go through zhc_export
---
error: Zhc.Tests.rogue is exported without Zhc.Tests.rogue.clean: exports must go through zhc_export
---
error: Zhc.Tests.rogue is exported without Zhc.Tests.rogue.projects: exports must go through zhc_export
-/
#guard_msgs in
@[export zhc_lean_export_rogue]
def rogue (_b : Ffi.L2r.Builder) (x : Ffi.L2r.CiphertextInteger) : IO Ffi.L2r.CiphertextInteger :=
  pure x

/-- error: Zhc.Tests.sorried.correct uses sorry -/
#guard_msgs in
zhc_export sorried (si : SpecInteger sb22) (x : CiphertextInteger si) : (r : CiphertextInteger si) where
  spec := r.toNat = 0
  prog := same1
  correct := same1_zero
  clean := same1_clean
  projects := same1_projects

/-! ## Runtime checks against the real builder -/

def report (act : IO Unit) : IO Unit :=
  try act; IO.println "accepted" catch e => IO.println e

/-- info: ║ Sig: (CiphertextInteger<8, 2, 2>, CiphertextInteger<8, 2, 2>) -> CiphertextInteger<8, 2, 2> -/
#guard_msgs in
set_option linter.zhc.noPanic false in
#eval show IO Unit from do
  let b ← add.circuit ⟨4⟩
  IO.println (((← b.dump).splitOn "\n")[1]!)

/--
info: ╔══════════════════════════════════════════════════════════════════════════════
║ Sig: (CiphertextInteger<8, 2, 2>, CiphertextInteger<8, 2, 2>, CiphertextBool<2, 2>) -> (CiphertextInteger<8, 2, 2>, CiphertextBool<2, 2>, CiphertextInteger<8, 2, 2>)
║──────────────────────────────────────────────────────────────────────────────
║ %0 = input_ciphertext<0, 8>();
║ %1 = input_ciphertext<1, 8>();
║ %2 = input_ciphertext_bool<2>();
║ output<0>(%1);
║ output_bool<1>(%2);
║ output<2>(%0);
╚══════════════════════════════════════════════════════════════════════════════
-/
#guard_msgs in
#eval show IO Unit from do
  let b ← three.circuit ⟨4⟩
  IO.println (← b.dump)

/-- info: handle int size does not match the signature -/
#guard_msgs in
#eval report do
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 2 2)
  let x ← b.ciphertextIntegerInput 8
  let y ← b.ciphertextIntegerInput 4
  let _ ← add.run b x y

/-- info: handle block spec does not match the algorithm block spec -/
#guard_msgs in
#eval report do
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 1 1)
  let x ← b.ciphertextIntegerInput 8
  let y ← b.ciphertextIntegerInput 8
  let _ ← add.run b x y

/-- info: builder block spec does not match the algorithm block spec -/
#guard_msgs in
#eval report do
  let a ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 2 2)
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 1 1)
  let x ← a.ciphertextIntegerInput 8
  let y ← a.ciphertextIntegerInput 8
  let _ ← add.run b x y

/-- info: handle int size does not match the signature -/
#guard_msgs in
#eval report do
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 2 2)
  let x ← b.ciphertextIntegerInput 8
  let p ← b.plaintextIntegerInput 4
  let c ← b.ciphertextBoolInput
  let z ← b.ciphertextIntegerInput 6
  let _ ← mixed.run b x p c z

/-- info: handle block spec does not match the algorithm block spec -/
#guard_msgs in
#eval report do
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 2 2)
  let other ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 1 1)
  let x ← b.ciphertextIntegerInput 8
  let p ← b.plaintextIntegerInput 8
  let c ← other.ciphertextBoolInput
  let z ← b.ciphertextIntegerInput 8
  let _ ← mixed.run b x p c z

/-- info: int size is not a multiple of the block message size -/
#guard_msgs in
#eval report do
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 2 2)
  let x ← b.ciphertextIntegerInput 8
  let _ ← fresh.run b 3 x

/--
info: intSize=6 carry=2 message=2
accepted
-/
#guard_msgs in
#eval report do
  let b ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.new 2 2)
  let x ← b.ciphertextIntegerInput 8
  let r ← fresh.run b 6 x
  let spec ← r.spec
  let block ← spec.block
  IO.println s!"intSize={← spec.intSize} carry={← block.carry} message={← block.message}"

end Zhc.Tests
