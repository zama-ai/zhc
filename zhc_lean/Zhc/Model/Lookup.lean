import Zhc.Model.SpecBlock
import Zhc.Model.CiphertextBlock
import Zhc.Util

/-!
Programmable bootstrapping as a lookup.

A bootstrapping with `2 ^ n` outputs packs `2 ^ n` sub-tables in one accumulator. Each
sub-table has `2 ^ (dataSize - n)` entries, so the `n` topmost data bits of the input select
a sub-table and the remaining bits select an entry. The semantics below is the one of
`tfhe-rs` (`trivial_pbs_many_lut`) and of the HPU accumulator layout (`create_hpu_lookuptable`).

Write the input data bits as `v = q * D + r` with `D = 2 ^ (dataSize - n)`, and let `p` be the
padding bit. Output `j` is

    out j = (-1) ^ (p xor [q + j ≥ 2 ^ n]) * table ((q + j) mod 2 ^ n) r

With `q = 0` this is "output `j` is sub-table `j` at `r`, negated if the padding bit is set".
With `q ≠ 0` the sub-tables are rotated by `q` and the `q` outputs that wrapped are negated.

A single-output lookup is the case `n = 0`.
-/

namespace Zhc.Model

structure Pbs (sb: SpecBlock) where
  table: Fin (2 ^ sb.dataSize) -> CiphertextBlock sb

def pbs(self : Pbs sb) (inp : CiphertextBlock sb) : CiphertextBlock sb :=
  let inp_nat := inp.toDataNat
  let oup := self.table ⟨inp_nat, Util.mod_two_pow_lt⟩
  if inp.hasActivePaddingBit then oup.doNeg else oup

structure ManyLut (sb : SpecBlock) (n : Nat) where
  name : String
  lut : Pbs sb

namespace ManyLut variable {sb : SpecBlock} {n : Nat}

abbrev subTableSize (sb : SpecBlock) (n : Nat) : Nat := 2 ^ (sb.dataSize - n)

def fromFns (name : String) (fs : Vector (CiphertextBlock sb → CiphertextBlock sb) (2 ^ n)) :
    ManyLut sb n :=
  let table (i : Fin (2 ^ sb.dataSize)) : CiphertextBlock sb :=
    let fnIndex : Fin (2 ^ n) := ⟨i.val / subTableSize sb n, Util.div_two_pow_sub_lt i.isLt⟩
    let fnInput := i.val % subTableSize sb n
    let f := fs[fnIndex]
    f (CiphertextBlock.ofNat fnInput)
  { name := name, lut := { table := table } }

def entry (self : ManyLut sb n) (k r : Nat) : CiphertextBlock sb :=
  let i := r + k * subTableSize sb n
  self.lut.table ⟨i % 2 ^ sb.dataSize, Util.mod_two_pow_lt⟩

def output (self : ManyLut sb n) (inp : CiphertextBlock sb) (j : Fin (2 ^ n)) : CiphertextBlock sb :=
  let shifted := CiphertextBlock.ofNat (inp.toCompleteNat + j.val * subTableSize sb n)
  pbs self.lut shifted

theorem output_of_clean (lut : ManyLut sb n) (s : CiphertextBlock sb) (j : Fin (2 ^ n))
    (h : n ≤ sb.dataSize) :
  s.toCompleteNat < 2 ^ (sb.dataSize - n) → lut.output s j = lut.entry j.val s.toCompleteNat
:= by
  intro (hs : s.toCompleteNat < subTableSize sb n)
  obtain ⟨j, hj⟩ := j
  have hP : 2 ^ sb.dataSize = 2 ^ n * subTableSize sb n := by
    rw [subTableSize, ← Nat.pow_add, Nat.add_sub_cancel' h]
  have hlt : s.toCompleteNat + j * subTableSize sb n < 2 ^ sb.dataSize := by
    have := Nat.mul_le_mul_right (subTableSize sb n) (Nat.succ_le_of_lt hj)
    rw [Nat.succ_mul] at this
    omega
  have hlt' : s.toCompleteNat + j * subTableSize sb n < 2 ^ sb.completeSize :=
    Nat.lt_of_lt_of_le hlt (Nat.pow_le_pow_right (by decide) sb.dataSize_le_completeSize)
  have hpad : ¬ (CiphertextBlock.ofNat (sb := sb) (s.toCompleteNat + j * subTableSize sb n)).hasActivePaddingBit :=
    CiphertextBlock.not_hasActivePaddingBit_of_lt (by simp [Nat.mod_eq_of_lt hlt', hlt])
  simp [output, pbs, entry, hpad, Nat.mod_eq_of_lt hlt', Nat.mod_eq_of_lt hlt]

end ManyLut

abbrev Lut1 (sb : SpecBlock) := ManyLut sb 0

def Lut1.fromFn (name : String) (f : CiphertextBlock sb → CiphertextBlock sb) : Lut1 sb :=
  ManyLut.fromFns name #v[f]

abbrev Lut2 (sb : SpecBlock) := ManyLut sb 1

def Lut2.fromFn (name : String) (f1 f2 : CiphertextBlock sb → CiphertextBlock sb) : Lut2 sb :=
  ManyLut.fromFns name #v[f1, f2]

abbrev Lut4 (sb : SpecBlock) := ManyLut sb 2

def Lut4.fromFn (name : String) (f1 f2 f3 f4 : CiphertextBlock sb → CiphertextBlock sb) : Lut4 sb :=
  ManyLut.fromFns name #v[f1, f2, f3, f4]

abbrev Lut8 (sb : SpecBlock) := ManyLut sb 3

def Lut8.fromFn (name : String)
    (f1 f2 f3 f4 f5 f6 f7 f8 : CiphertextBlock sb → CiphertextBlock sb) : Lut8 sb :=
  ManyLut.fromFns name #v[f1, f2, f3, f4, f5, f6, f7, f8]

end Zhc.Model
