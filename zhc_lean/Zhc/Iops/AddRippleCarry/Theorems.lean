import Zhc.IopAlgebra.Model.Theorems
import Zhc.IopAlgebra.Traced.Theorems
import Zhc.Iops.AddRippleCarry.Impl

namespace Zhc.Iops

open Model
open IopAlgebra.Model
open IopAlgebra.Traced (TraceM TrCtInteger)

theorem carryMsgLut_entry (s : Nat) :
  s < 8 →
  (carryMsgLut.entry 0 s).toCompleteNat = s / 4 ∧ (carryMsgLut.entry 1 s).toCompleteNat = s % 4
:= by
  revert s
  decide

theorem pbs2_carryMsgLut (s : CiphertextBlock sb22) (h : s.toCompleteNat < 8) :
  (carryMsgLut.output s 0).toCompleteNat = s.toCompleteNat / 4
  ∧ (carryMsgLut.output s 1).toCompleteNat = s.toCompleteNat % 4
:= by
  rw [ManyLut.output_of_clean carryMsgLut s 0 (by decide) h,
    ManyLut.output_of_clean carryMsgLut s 1 (by decide) h]
  exact carryMsgLut_entry _ h

theorem lookup_input (xj yj c : Nat) (hx : xj < 4) (hy : yj < 4) (hc : c ≤ 1) :
  (CiphertextBlock.ofNat (sb := sb22)
    ((CiphertextBlock.ofNat (sb := sb22) (xj + yj)).toCompleteNat + c)).toCompleteNat
  = xj + yj + c
:= by
  simp only [CiphertextBlock.toCompleteNat_ofNat, sb22, SpecBlock.completeSize, SpecBlock.dataSize,
    SpecBlock.paddingSize, Nat.reduceAdd, Nat.reducePow]
  omega

theorem step {x y acc carry P : Nat} :
  0 < P → acc < P → carry ≤ 1 → acc + carry * P = x % P + y % P →
  acc + (x / P % 4 + y / P % 4 + carry) % 4 * P < P * 4
  ∧ (x / P % 4 + y / P % 4 + carry) / 4 ≤ 1
  ∧ acc + (x / P % 4 + y / P % 4 + carry) % 4 * P
      + (x / P % 4 + y / P % 4 + carry) / 4 * (P * 4)
    = x % (P * 4) + y % (P * 4)
:= by
  intro hP hacc hcarry hinv
  have hx := Nat.mod_mul (a := P) (b := 4) (x := x)
  have hy := Nat.mod_mul (a := P) (b := 4) (x := y)
  have hxi : x / P % 4 < 4 := Nat.mod_lt _ (by decide)
  have hyi : y / P % 4 < 4 := Nat.mod_lt _ (by decide)
  generalize x / P % 4 = xi at *
  generalize y / P % 4 = yi at *
  -- the three small operands are literals, so that `omega` sees only linear terms in `P`
  rcases (by omega : xi = 0 ∨ xi = 1 ∨ xi = 2 ∨ xi = 3) with h | h | h | h
    <;> rcases (by omega : yi = 0 ∨ yi = 1 ∨ yi = 2 ∨ yi = 3) with h' | h' | h' | h'
    <;> rcases (by omega : carry = 0 ∨ carry = 1) with h'' | h''
    <;> subst h h' h''
    <;> simp only [Nat.reduceAdd, Nat.reduceMod, Nat.reduceDiv] at *
    <;> omega

theorem addRippleCarry_spec (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (hx : x.IsClean) (hy : y.IsClean) :
  (addRippleCarry (M := Id) si x y).toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  ∧ (addRippleCarry (M := Id) si x y).IsClean
:= by
  unfold addRippleCarry
  simp only [bind, pure]
  rw [Util.forIn_id_yield]
  refine Util.foldl_invariant
    (P := fun r => r.1.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize ∧ r.1.IsClean)
    (Inv := fun k (s : CiphertextInteger si × CiphertextBlock sb22) =>
      s.1.toNat < 2 ^ (k * 2)
      ∧ s.2.toCompleteNat ≤ 1
      ∧ s.1.toNat + s.2.toCompleteNat * 2 ^ (k * 2) = x.toNat % 2 ^ (k * 2) + y.toNat % 2 ^ (k * 2)
      ∧ s.1.IsClean)
    _ _ _ ?_ ?_ ?_
  · -- entry: `acc = 0`, `carry = 0`, `P = 1`
    refine ⟨?_, ?_, ?_, CiphertextInteger.isClean_ofNat 0⟩ <;> simp [Nat.mod_one]
  · -- step
    intro j hj ⟨acc, carry⟩ ⟨hacc, hcarry, hsum, hclean⟩
    simp only [List.length_finRange] at hj
    simp only [List.getElem_finRange, Fin.cast_mk, storeCtBlock_eq, pbs2_eq, addCt_eq,
      extractCtBlock_eq, Operators.add]
    have hxj := x.toCompleteNat_getBlock_of_clean hx ⟨j, hj⟩
    have hyj := y.toCompleteNat_getBlock_of_clean hy ⟨j, hj⟩
    simp only [sb22, Nat.reducePow] at *
    simp only [hxj, hyj]
    -- the lookup input is `xⱼ + yⱼ + carry < 8`
    have hin := lookup_input (x.toNat / 2 ^ (j * 2) % 4) (y.toNat / 2 ^ (j * 2) % 4)
      carry.toCompleteNat (Nat.mod_lt _ (by decide)) (Nat.mod_lt _ (by decide)) hcarry
    generalize CiphertextBlock.ofNat (sb := sb22)
      ((CiphertextBlock.ofNat (sb := sb22) (x.toNat / 2 ^ (j * 2) % 4 + y.toNat / 2 ^ (j * 2) % 4)).toCompleteNat
        + carry.toCompleteNat) = s at *
    obtain ⟨hc, hm⟩ := pbs2_carryMsgLut s (by rw [hin]; omega)
    rw [hin] at hc hm
    have hmclean : (carryMsgLut.output s 1).hasMessageOnly := by
      rw [CiphertextBlock.hasMessageOnly_iff, hm]
      exact Nat.mod_lt _ (by decide)
    -- the store: `acc < 2 ^ (j * 2)`, so digit `j` is free
    rw [CiphertextInteger.toNat_setBlock_of_lt _ _ _ hacc, CiphertextBlock.toMessageNat_eq, hm, hc]
    simp only [sb22, Nat.reducePow, Nat.mod_mod]
    -- the arithmetic
    rw [← Util.two_pow_mul_succ]
    simp only [Nat.reducePow]
    obtain ⟨h1, h2, h3⟩ := step (x := x.toNat) (y := y.toNat) (Nat.two_pow_pos _) hacc hcarry hsum
    exact ⟨h1, h2, h3, acc.isClean_setBlock _ _ hclean hmclean⟩
  · -- exit: `P = 2 ^ intSize`, both inputs are below `P`
    intro ⟨acc, carry⟩ ⟨h1, _, h3, hclean⟩
    simp only [List.length_finRange, SpecInteger.intSize, sb22] at *
    refine ⟨?_, hclean⟩
    rw [Nat.add_mod, ← h3, Nat.add_mul_mod_self_right, Nat.mod_eq_of_lt h1]

theorem addRippleCarry_correct (si : SpecInteger sb22) (x y : CiphertextInteger si) (hx : x.IsClean) (hy : y.IsClean) :
  (addRippleCarry (M := Id) si x y).toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
:=
  (addRippleCarry_spec si x y hx hy).1

theorem addRippleCarry_isClean (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (hx : x.IsClean) (hy : y.IsClean) : (addRippleCarry (M := Id) si x y).IsClean
:=
  (addRippleCarry_spec si x y hx hy).2

theorem addRippleCarry_traced_projects (si : SpecInteger sb22) (x y : TrCtInteger si) :
  (addRippleCarry (M := TraceM) si x y).ModelIs (addRippleCarry (M := Id) si x.model y.model)
:= by
  unfold addRippleCarry
  projects_prog

end Zhc.Iops
