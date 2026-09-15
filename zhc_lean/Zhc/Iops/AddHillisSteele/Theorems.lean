import Zhc.IopAlgebra.Model.Theorems
import Zhc.IopAlgebra.Traced.Theorems
import Zhc.Iops.AddRippleCarry.Theorems
import Zhc.Iops.AddHillisSteele.Impl

namespace Zhc.Iops

open Model
open IopAlgebra.Model
open IopAlgebra.Traced (TraceM TrCtInteger)

namespace HillisSteele

def digit (v i : Nat) : Nat :=
  v / 4 ^ i % 4

def carryIn (x y i : Nat) : Nat :=
  (x % 4 ^ i + y % 4 ^ i) / 4 ^ i

def windowStatus (x y lo len : Nat) : Nat :=
  let a := x / 4 ^ lo % 4 ^ len
  let b := y / 4 ^ lo % 4 ^ len
  if 4 ^ len ≤ a + b then 2 else if a + b + 1 = 4 ^ len then 1 else 0

def blockStatus (s : Nat) : Nat :=
  if 4 ≤ s then 2 else if s = 3 then 1 else 0

def solve (hi lo : Nat) : Nat :=
  if hi = 1 then lo else hi

def weightedStatus (x y lo : Nat) : Nat → Nat
  | 0 => 0
  | k + 1 => weightedStatus x y lo k + windowStatus x y (lo + k) 1 * 2 ^ k

def SumsSpec (x y : Nat) (l : List (CiphertextBlock sb22)) : Prop :=
  ∀ i (h : i < l.length), l[i].toCompleteNat = digit x i + digit y i

def BlockStatesSpec (x y : Nat) (l : List (CiphertextBlock sb22)) : Prop :=
  ∀ i (h : i < l.length), l[i].toCompleteNat =
    if i = 0 then carryIn x y 1 else windowStatus x y i 1 * 2 ^ (if i < 4 then i - 1 else i % 4)

def GroupStatesSpec (x y : Nat) (gs : List (CiphertextBlock sb22)) : Prop :=
  ∀ g p (hp : p < 4) (h : 4 * g + p < gs.length), gs[4 * g + p].toCompleteNat =
    if p < 3 then
      if g = 0 then carryIn x y 1 + weightedStatus x y 1 p else weightedStatus x y (4 * g) (p + 1)
    else if g = 0 then carryIn x y 4
    else windowStatus x y (4 * g) 4

def HsInvariant (x y k : Nat) (l : List (CiphertextBlock sb22)) : Prop :=
  ∀ g (h : g < l.length), l[g].toCompleteNat =
    if g < 2 ^ k then carryIn x y (4 * (g + 1))
    else windowStatus x y (4 * (g + 1 - 2 ^ k)) (4 * 2 ^ k)

def CarriesSpec (x y : Nat) (l : List (CiphertextBlock sb22)) : Prop :=
  ∀ i (h : i < l.length), l[i].toCompleteNat = carryIn x y (i + 1)

theorem four_pow_pos (i : Nat) :
  0 < 4 ^ i
:=
  Nat.pow_pos (by decide)

theorem carryIn_zero (x y : Nat) :
  carryIn x y 0 = 0
:= by
  simp [carryIn, Nat.mod_one]

theorem carryIn_le_one (x y i : Nat) :
  carryIn x y i ≤ 1
:= by
  unfold carryIn
  have hP := four_pow_pos i
  have := Nat.mod_lt x hP
  have := Nat.mod_lt y hP
  have : (x % 4 ^ i + y % 4 ^ i) / 4 ^ i < 2 := (Nat.div_lt_iff_lt_mul hP).2 (by omega)
  omega

theorem windowStatus_le_two (x y lo len : Nat) :
  windowStatus x y lo len ≤ 2
:= by
  simp only [windowStatus]
  (repeat' split) <;> omega

theorem digit_add (x y i : Nat) :
  digit (x + y) i = (digit x i + digit y i + carryIn x y i) % 4
:= by
  unfold digit carryIn
  have hP := four_pow_pos i
  have hx := Nat.div_add_mod x (4 ^ i)
  have hy := Nat.div_add_mod y (4 ^ i)
  have : (x + y) / 4 ^ i = (x % 4 ^ i + y % 4 ^ i) / 4 ^ i + (x / 4 ^ i + y / 4 ^ i) := by
    rw [← Nat.add_mul_div_left _ _ hP]
    congr 1
    rw [Nat.mul_add]
    omega
  rw [this]
  omega

theorem blockStatus_eq (s : Nat) :
  blockStatus s = if 4 ≤ s then 2 else if s + 1 = 4 then 1 else 0
:= by
  unfold blockStatus
  (repeat' split) <;> omega

theorem blockStatus_digits (x y i : Nat) :
  blockStatus (digit x i + digit y i) = windowStatus x y i 1
:= by
  rw [blockStatus_eq]
  simp only [digit, windowStatus, Nat.pow_one]
  rfl

theorem windowStatus_append (x y lo a b : Nat) :
  windowStatus x y lo (a + b) = solve (windowStatus x y (lo + a) b) (windowStatus x y lo a)
:= by
  have hA := four_pow_pos a
  have hB := four_pow_pos b
  simp only [windowStatus, solve, Nat.pow_add, ← Nat.div_div_eq_div_mul, Nat.mod_mul]
  generalize x / 4 ^ lo = X
  generalize y / 4 ^ lo = Y
  generalize 4 ^ a = A at *
  generalize 4 ^ b = B at *
  have hxl := Nat.mod_lt X hA
  have hyl := Nat.mod_lt Y hA
  have hxh := Nat.mod_lt (X / A) hB
  have hyh := Nat.mod_lt (Y / A) hB
  generalize X % A = xl at *
  generalize Y % A = yl at *
  generalize X / A % B = xh at *
  generalize Y / A % B = yh at *
  rcases (by omega : B ≤ xh + yh ∨ xh + yh + 1 = B ∨ xh + yh + 2 ≤ B) with h | h | h
  · have := Nat.mul_le_mul_left A h
    rw [Nat.mul_add] at this
    (repeat' split) <;> omega
  · have : A * (xh + yh + 1) = A * B := by rw [h]
    rw [Nat.mul_add, Nat.mul_add, Nat.mul_one] at this
    (repeat' split) <;> omega
  · have := Nat.mul_le_mul_left A h
    rw [Nat.mul_add, Nat.mul_add] at this
    (repeat' split) <;> omega

theorem carryIn_window_aux (xl yl a b P W : Nat) (hP : 0 < P) (hxl : xl < P) (hyl : yl < P)
    (ha : a < W) (hb : b < W) :
  (xl + P * a + (yl + P * b)) / P / W =
    if (if W ≤ a + b then 2 else if a + b + 1 = W then 1 else 0) = 1 then (xl + yl) / P
    else (if W ≤ a + b then 2 else if a + b + 1 = W then 1 else 0) / 2
:= by
  have hc : (xl + yl) / P < 2 := (Nat.div_lt_iff_lt_mul hP).2 (by omega)
  rw [show xl + P * a + (yl + P * b) = xl + yl + P * (a + b) by rw [Nat.mul_add]; omega,
    Nat.add_mul_div_left _ _ hP]
  generalize (xl + yl) / P = c at *
  rcases (by omega : W ≤ a + b ∨ a + b + 1 = W ∨ a + b + 2 ≤ W) with h | h | h
  · rw [Nat.div_eq_of_lt_le (k := 1) (by omega) (by omega)]
    (repeat' split) <;> omega
  · rcases (by omega : c = 0 ∨ c = 1) with rfl | rfl
    · rw [Nat.div_eq_of_lt (by omega)]
      (repeat' split) <;> omega
    · rw [Nat.div_eq_of_lt_le (k := 1) (by omega) (by omega)]
      (repeat' split) <;> omega
  · rw [Nat.div_eq_of_lt (by omega)]
    (repeat' split) <;> omega

theorem carryIn_window (x y lo len : Nat) :
  carryIn x y (lo + len) =
    if windowStatus x y lo len = 1 then carryIn x y lo else windowStatus x y lo len / 2
:= by
  have hP := four_pow_pos lo
  have hW := four_pow_pos len
  simp only [carryIn, windowStatus, Nat.pow_add, ← Nat.div_div_eq_div_mul, Nat.mod_mul]
  exact carryIn_window_aux _ _ _ _ _ _ hP (Nat.mod_lt x hP) (Nat.mod_lt y hP)
    (Nat.mod_lt _ hW) (Nat.mod_lt _ hW)

theorem weightedStatus_le (x y lo k : Nat) :
  weightedStatus x y lo k + 2 ≤ 2 ^ (k + 1)
:= by
  induction k with
  | zero => simp [weightedStatus]
  | succ k ih =>
    have hs := windowStatus_le_two x y (lo + k) 1
    have : windowStatus x y (lo + k) 1 * 2 ^ k ≤ 2 * 2 ^ k := Nat.mul_le_mul_right _ hs
    simp only [weightedStatus, Nat.pow_succ] at *
    omega

theorem carry_lookahead_aux (c w s ck Q : Nat) (hQ : 0 < Q) (hc : c ≤ 1) (hw : w + 2 ≤ Q * 2)
    (hs : s ≤ 2) (hck : ck ≤ 1) (ih : (c + w) / Q % 2 = ck) :
  (c + (w + s * Q)) / (Q * 2) % 2 = if s = 1 then ck else s / 2
:= by
  have hlt : (c + w) / Q < 2 := (Nat.div_lt_iff_lt_mul hQ).2 (by omega)
  have hdiv : (c + w) / Q = ck := (Nat.mod_eq_of_lt hlt).symm.trans ih
  rw [show c + (w + s * Q) = c + w + s * Q by omega, ← Nat.div_div_eq_div_mul,
    Nat.add_mul_div_right _ _ hQ, hdiv]
  (repeat' split) <;> omega

theorem carry_lookahead (x y lo k : Nat) :
  (carryIn x y lo + weightedStatus x y lo k) / 2 ^ k % 2 = carryIn x y (lo + k)
:= by
  induction k with
  | zero =>
    have := carryIn_le_one x y lo
    simp only [weightedStatus, Nat.pow_zero, Nat.div_one, Nat.add_zero]
    omega
  | succ k ih =>
    have hw := weightedStatus_le x y lo k
    rw [Nat.pow_succ] at hw
    rw [← Nat.add_assoc, carryIn_window x y (lo + k) 1, weightedStatus, Nat.pow_succ]
    exact carry_lookahead_aux _ _ _ _ _ (Nat.two_pow_pos k) (carryIn_le_one x y lo) hw
      (windowStatus_le_two x y (lo + k) 1) (carryIn_le_one x y (lo + k)) ih

theorem windowStatus_weighted_aux (w s Q : Nat) (hQ : 0 < Q) (hw : w + 2 ≤ Q * 2) (hs : s ≤ 2) :
  solve s (if w = Q - 1 then 1 else if Q ≤ w then 2 else 0) =
    if w + s * Q = Q * 2 - 1 then 1 else if Q * 2 ≤ w + s * Q then 2 else 0
:= by
  rcases (by omega : s = 0 ∨ s = 1 ∨ s = 2) with rfl | rfl | rfl <;>
    simp only [solve, Nat.zero_mul, Nat.one_mul, Nat.add_zero, ite_true, ite_false,
      Nat.reduceEqDiff] <;>
    (repeat' split) <;> omega

theorem windowStatus_weighted (x y lo k : Nat) :
  windowStatus x y lo k =
    if weightedStatus x y lo k = 2 ^ k - 1 then 1
    else if 2 ^ k ≤ weightedStatus x y lo k then 2
    else 0
:= by
  induction k with
  | zero => simp [windowStatus, weightedStatus, Nat.mod_one]
  | succ k ih =>
    have hw := weightedStatus_le x y lo k
    rw [Nat.pow_succ] at hw
    rw [windowStatus_append x y lo k 1, ih, weightedStatus, Nat.pow_succ]
    exact windowStatus_weighted_aux _ _ _ (Nat.two_pow_pos k) hw
      (windowStatus_le_two x y (lo + k) 1)

theorem block_eq_ofNat (s : CiphertextBlock sb22) :
  s = CiphertextBlock.ofNat s.toCompleteNat
:=
  CiphertextBlock.ext_toCompleteNat
    (by rw [CiphertextBlock.toCompleteNat_ofNat, Nat.mod_eq_of_lt s.toCompleteNat_lt])

theorem extractPropGroupLut_output (p : Fin 4) (s : CiphertextBlock sb22) (h : s.toCompleteNat < 8) :
  ((extractPropGroupLut p).output s 0).toCompleteNat = blockStatus s.toCompleteNat * 2 ^ p.val
:= by
  have key : ∀ p : Fin 4, ∀ n, n < 8 →
      ((extractPropGroupLut p).output (CiphertextBlock.ofNat n) 0).toCompleteNat
        = blockStatus n * 2 ^ p.val := by
    decide
  have := key p _ h
  rwa [← block_eq_ofNat] at this

theorem solvePropGroupFinalLut_output (p : Fin 3) (s : CiphertextBlock sb22)
    (h : s.toCompleteNat < 16) :
  ((solvePropGroupFinalLut p).output s 0).toCompleteNat = s.toCompleteNat / 2 ^ (p.val + 1) % 2
:= by
  have key : ∀ p : Fin 3, ∀ n, n < 16 →
      ((solvePropGroupFinalLut p).output (CiphertextBlock.ofNat n) 0).toCompleteNat
        = n / 2 ^ (p.val + 1) % 2 := by
    decide
  have := key p _ h
  rwa [← block_eq_ofNat] at this

theorem reduceCarryPadLut_output (s : CiphertextBlock sb22) (h : s.toCompleteNat < 31) :
  (reduceCarryPadLut.output s 0).toCompleteNat =
    if s.toCompleteNat = 15 then 0 else if s.toCompleteNat < 16 then 31 else 1
:= by
  have key : ∀ n, n < 31 →
      (reduceCarryPadLut.output (CiphertextBlock.ofNat n) 0).toCompleteNat
        = if n = 15 then 0 else if n < 16 then 31 else 1 := by
    decide
  have := key _ h
  rwa [← block_eq_ofNat] at this

theorem solvePropLut_output (s : CiphertextBlock sb22) (h : s.toCompleteNat < 16) :
  (solvePropLut.output s 0).toCompleteNat = solve (s.toCompleteNat / 4) (s.toCompleteNat % 4)
:= by
  have key : ∀ n, n < 16 →
      (solvePropLut.output (CiphertextBlock.ofNat n) 0).toCompleteNat = solve (n / 4) (n % 4) := by
    decide
  have := key _ h
  rwa [← block_eq_ofNat] at this

theorem solvePropCarryLut_output (s : CiphertextBlock sb22) (h : s.toCompleteNat < 16) :
  (solvePropCarryLut.output s 0).toCompleteNat =
    if s.toCompleteNat / 4 = 1 then s.toCompleteNat % 4 else s.toCompleteNat / 4 / 2
:= by
  have key : ∀ n, n < 16 →
      (solvePropCarryLut.output (CiphertextBlock.ofNat n) 0).toCompleteNat
        = if n / 4 = 1 then n % 4 else n / 4 / 2 := by
    decide
  have := key _ h
  rwa [← block_eq_ofNat] at this

theorem msgOnlyLut_output (s : CiphertextBlock sb22) (h : s.toCompleteNat < 16) :
  (msgOnlyLut.output s 0).toCompleteNat = s.toCompleteNat % 4
:= by
  have key : ∀ n, n < 16 →
      (msgOnlyLut.output (CiphertextBlock.ofNat n) 0).toCompleteNat = n % 4 := by
    decide
  have := key _ h
  rwa [← block_eq_ofNat] at this

theorem le_computeSize (n : Nat) :
  n ≤ computeSize n
:=
  Nat.le_trans (by omega) (Nat.le_nextPowerOfTwo _)

theorem computeSize_eq (n : Nat) (hn : 0 < n) :
  ∃ m, computeSize n = 4 * 2 ^ m
:= by
  obtain ⟨k, hk⟩ := Nat.isPowerOfTwo_nextPowerOfTwo ((n + 3) / 4 * 4)
  have h4 : 4 ≤ computeSize n := Nat.le_trans (by omega) (Nat.le_nextPowerOfTwo _)
  unfold computeSize at *
  rw [hk] at h4 ⊢
  match k with
  | 0 => simp at h4
  | 1 => simp at h4
  | k + 2 => exact ⟨k, by rw [Nat.pow_succ, Nat.pow_succ]; omega⟩

theorem two_pow_mul_two (i : Nat) :
  2 ^ (i * 2) = 4 ^ i
:= by
  rw [Nat.mul_comm, Nat.pow_mul]

theorem digit_eq_zero {v n i : Nat} (hv : v < 4 ^ n) (hi : n ≤ i) :
  digit v i = 0
:= by
  unfold digit
  rw [Nat.div_eq_of_lt (Nat.lt_of_lt_of_le hv (Nat.pow_le_pow_right (by decide) hi))]

theorem digit_lt (v i : Nat) :
  digit v i < 4
:=
  Nat.mod_lt _ (by decide)

@[simp] theorem toCompleteNat_add (a b : CiphertextBlock sb22) :
  (Operators.add a b).toCompleteNat = (a.toCompleteNat + b.toCompleteNat) % 32
:=
  rfl

@[simp] theorem toCompleteNat_mac (a b : CiphertextBlock sb22) (m : Nat) :
  (Operators.mac a m b).toCompleteNat = (a.toCompleteNat * m + b.toCompleteNat) % 32
:=
  rfl

@[simp] theorem toCompleteNat_addPt (a : CiphertextBlock sb22) (b : PlaintextBlock sb22) :
  (Operators.addPt a b).toCompleteNat = (a.toCompleteNat + b.toCompleteNat) % 32
:=
  rfl

@[simp] theorem toCompleteNat_ptOne :
  (PlaintextBlock.ofNat 1 : PlaintextBlock sb22).toCompleteNat = 1
:=
  rfl

theorem rawSums_run (si : SpecInteger sb22) (x y : CiphertextInteger si) :
  Id.run (rawSums (M := Id) si x y) =
    (List.finRange si.blockCount).map (fun i => Operators.add (x.getBlock i) (y.getBlock i))
      ++ List.replicate (computeSize si.blockCount - si.blockCount) (CiphertextBlock.ofNat 0)
:= by
  unfold rawSums
  rw [Id.run_bind, List.idRun_mapM]
  rfl

theorem rawSums_spec (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (hx : x.IsClean) (hy : y.IsClean) :
  let s := Id.run (rawSums (M := Id) si x y)
  s.length = computeSize si.blockCount ∧ SumsSpec x.toNat y.toNat s
:= by
  have hle := le_computeSize si.blockCount
  have hxlt := x.toNat_lt
  have hylt := y.toNat_lt
  simp only [SpecInteger.intSize, sb22, two_pow_mul_two] at hxlt hylt
  dsimp only
  rw [rawSums_run]
  refine ⟨by simp; omega, ?_⟩
  intro i h
  by_cases hi : i < si.blockCount
  · rw [List.getElem_append_left (by simpa using hi)]
    have hxi := x.toCompleteNat_getBlock_of_clean hx ⟨i, hi⟩
    have hyi := y.toCompleteNat_getBlock_of_clean hy ⟨i, hi⟩
    simp only [sb22, two_pow_mul_two, Nat.reducePow] at hxi hyi
    simp only [List.getElem_map, List.getElem_finRange, Fin.cast_mk, toCompleteNat_add, hxi, hyi]
    have := digit_lt x.toNat i
    have := digit_lt y.toNat i
    unfold digit at *
    omega
  · rw [List.getElem_append_right (by simpa using hi)]
    simp only [List.getElem_replicate, CiphertextBlock.toCompleteNat_ofNat, Nat.zero_mod]
    rw [digit_eq_zero hxlt (by omega), digit_eq_zero hylt (by omega)]

theorem blockStates_run (sums : List (CiphertextBlock sb22)) :
  Id.run (blockStates (M := Id) sums) = sums.zipIdx.map fun p =>
    if p.2 = 0 then carryMsgLut.output p.1 0
    else if p.2 < 4 then (extractPropGroupLut ⟨(p.2 - 1) % 4, by omega⟩).output p.1 0
    else (extractPropGroupLut ⟨p.2 % 4, by omega⟩).output p.1 0
:= by
  unfold blockStates
  rw [List.idRun_mapM]
  rfl

theorem blockStates_spec (x y : Nat) (sums : List (CiphertextBlock sb22))
    (hs : SumsSpec x y sums) :
  let st := Id.run (blockStates (M := Id) sums)
  st.length = sums.length ∧ BlockStatesSpec x y st
:= by
  dsimp only
  rw [blockStates_run]
  refine ⟨by simp, ?_⟩
  intro i h
  simp only [List.length_map, List.length_zipIdx] at h
  simp only [List.getElem_map, List.getElem_zipIdx, Nat.zero_add]
  have hsi := hs i h
  have := digit_lt x i
  have := digit_lt y i
  have hlt : sums[i].toCompleteNat < 8 := by omega
  by_cases h0 : i = 0
  · subst h0
    simp only [↓reduceIte]
    rw [(pbs2_carryMsgLut _ hlt).1, hsi]
    simp [carryIn, digit]
  · by_cases h4 : i < 4
    · simp only [h0, h4, ↓reduceIte]
      rw [extractPropGroupLut_output _ _ hlt, hsi, blockStatus_digits]
      simp only [Nat.mod_eq_of_lt (by omega : i - 1 < 4)]
    · simp only [h0, h4, ↓reduceIte]
      rw [extractPropGroupLut_output _ _ hlt, hsi, blockStatus_digits]

def sel4 {α : Type} (t : α × α × α × α) : Nat → α
  | 0 => t.1
  | 1 => t.2.1
  | 2 => t.2.2.1
  | _ => t.2.2.2

theorem length_chunk4 {α : Type} : ∀ (l : List α), (chunk4 l).length = l.length / 4
  | _ :: _ :: _ :: _ :: rest => by simp [chunk4, length_chunk4 rest]; omega
  | [] => by simp [chunk4]
  | [_] => by simp [chunk4]
  | [_, _] => by simp [chunk4]
  | [_, _, _] => by simp [chunk4]

theorem getElem_chunk4 {α : Type} : ∀ (l : List α) (g : Nat) (h : g < (chunk4 l).length),
  (chunk4 l)[g] =
    (l[4 * g]'(by rw [length_chunk4] at h; omega), l[4 * g + 1]'(by rw [length_chunk4] at h; omega),
      l[4 * g + 2]'(by rw [length_chunk4] at h; omega), l[4 * g + 3]'(by rw [length_chunk4] at h; omega))
  | _ :: _ :: _ :: _ :: _, 0, _ => rfl
  | _ :: _ :: _ :: _ :: rest, g + 1, h => by
    simp only [chunk4, List.getElem_cons_succ]
    rw [getElem_chunk4 rest g]
    simp [Nat.mul_succ]
  | [], _, h => absurd h (by simp [chunk4])
  | [_], _, h => absurd h (by simp [chunk4])
  | [_, _], _, h => absurd h (by simp [chunk4])
  | [_, _, _], _, h => absurd h (by simp [chunk4])

theorem length_unchunk4 {α : Type} : ∀ (L : List (α × α × α × α)), (unchunk4 L).length = 4 * L.length
  | [] => rfl
  | _ :: rest => by simp [unchunk4, length_unchunk4 rest]; omega

theorem getElem_unchunk4 {α : Type} : ∀ (L : List (α × α × α × α)) (g p : Nat) (hp : p < 4)
    (h : 4 * g + p < (unchunk4 L).length),
  (unchunk4 L)[4 * g + p] = sel4 (L[g]'(by rw [length_unchunk4] at h; omega)) p
  | [], _, _, _, h => absurd h (by simp [unchunk4])
  | (a, b, c, d) :: rest, 0, p, hp, h => by
    rcases (by omega : p = 0 ∨ p = 1 ∨ p = 2 ∨ p = 3) with rfl | rfl | rfl | rfl <;> rfl
  | (a, b, c, d) :: rest, g + 1, p, hp, h => by
    have e : 4 * (g + 1) + p = 4 * g + p + 4 := by omega
    simp only [unchunk4, e, List.getElem_cons_succ]
    exact getElem_unchunk4 rest g p hp _

theorem groupStates_run (states : List (CiphertextBlock sb22)) :
  Id.run (groupStates (M := Id) states) = unchunk4 ((chunk4 states).zipIdx.map fun p =>
    let b1 := Operators.add p.1.1 p.1.2.1
    let b2 := Operators.add b1 p.1.2.2.1
    let b3 := Operators.add b2 p.1.2.2.2
    (p.1.1, b1, b2, if p.2 = 0 then (solvePropGroupFinalLut 2).output b3 0
      else Operators.addPt (reduceCarryPadLut.output b3 0) (PlaintextBlock.ofNat 1)))
:= by
  unfold groupStates
  rw [Id.run_bind, List.idRun_mapM]
  show unchunk4 (List.map _ _) = _
  congr 2
  funext p
  obtain ⟨⟨s0, s1, s2, s3⟩, g⟩ := p
  by_cases hg : g = 0 <;> simp only [hg, ↓reduceIte] <;> rfl

theorem groupStates_spec (x y : Nat) (states : List (CiphertextBlock sb22))
    (hlen : states.length % 4 = 0) (hst : BlockStatesSpec x y states) :
  let gs := Id.run (groupStates (M := Id) states)
  gs.length = states.length ∧ GroupStatesSpec x y gs
:= by
  dsimp only
  rw [groupStates_run]
  refine ⟨by simp only [length_unchunk4, List.length_map, List.length_zipIdx, length_chunk4]; omega, ?_⟩
  intro g p hp h
  have h' := h
  simp only [length_unchunk4, List.length_map, List.length_zipIdx, length_chunk4] at h'
  have hlc : g < (chunk4 states).length := by rw [length_chunk4]; omega
  rw [getElem_unchunk4 _ g p hp h]
  simp only [List.getElem_map, List.getElem_zipIdx, Nat.zero_add, getElem_chunk4 _ g hlc]
  have e0 := hst (4 * g) (by omega)
  have e1 := hst (4 * g + 1) (by omega)
  have e2 := hst (4 * g + 2) (by omega)
  have e3 := hst (4 * g + 3) (by omega)
  have w0 := windowStatus_le_two x y (4 * g) 1
  have w1 := windowStatus_le_two x y (4 * g + 1) 1
  have w2 := windowStatus_le_two x y (4 * g + 2) 1
  have w3 := windowStatus_le_two x y (4 * g + 3) 1
  have hc1 := carryIn_le_one x y 1
  by_cases hg : g = 0
  · subst hg
    simp at e0 e1 e2 e3 w0 w1 w2 w3 ⊢
    rcases (by omega : p = 0 ∨ p = 1 ∨ p = 2 ∨ p = 3) with rfl | rfl | rfl | rfl
    · simp [sel4, weightedStatus, e0]
    · simp only [sel4, toCompleteNat_add, e0, e1]
      simp [weightedStatus]
      omega
    · simp only [sel4, toCompleteNat_add, e0, e1, e2]
      simp [weightedStatus]
      omega
    · simp only [sel4]
      rw [solvePropGroupFinalLut_output _ _ (by simp only [toCompleteNat_add, e0, e1, e2, e3]; omega)]
      have hl := carry_lookahead x y 1 3
      simp [weightedStatus] at hl
      simp only [toCompleteNat_add, e0, e1, e2, e3]
      simp
      omega
  · have z0 : ¬ 4 * g = 0 := by omega
    have n0 : ¬ 4 * g < 4 := by omega
    have n1 : ¬ 4 * g + 1 < 4 := by omega
    have n2 : ¬ 4 * g + 2 < 4 := by omega
    have n3 : ¬ 4 * g + 3 < 4 := by omega
    have m0 : 4 * g % 4 = 0 := by omega
    have m1 : (4 * g + 1) % 4 = 1 := by omega
    have m2 : (4 * g + 2) % 4 = 2 := by omega
    have m3 : (4 * g + 3) % 4 = 3 := by omega
    simp [z0, n0, n1, n2, n3, m0, m1, m2, m3] at e0 e1 e2 e3
    rcases (by omega : p = 0 ∨ p = 1 ∨ p = 2 ∨ p = 3) with rfl | rfl | rfl | rfl
    · simp [sel4, weightedStatus, e0, hg]
    · simp only [sel4, toCompleteNat_add, e0, e1]
      simp [weightedStatus, hg]
      omega
    · simp only [sel4, toCompleteNat_add, e0, e1, e2]
      simp [weightedStatus, hg]
      omega
    · simp only [sel4, hg, ↓reduceIte, toCompleteNat_addPt]
      rw [reduceCarryPadLut_output _ (by simp only [toCompleteNat_add, e0, e1, e2, e3]; omega),
        windowStatus_weighted]
      simp only [toCompleteNat_add, e0, e1, e2, e3, toCompleteNat_ptOne]
      simp [weightedStatus]
      (repeat' split) <;> omega

theorem hsStage_run (k : Nat) (l : List (CiphertextBlock sb22)) :
  Id.run (hsStage (M := Id) k l) = l.take (2 ^ k) ++ ((l.drop (2 ^ k)).zip l).zipIdx.map fun p =>
    if p.2 < 2 ^ k then solvePropCarryLut.output (Operators.mac p.1.1 4 p.1.2) 0
    else solvePropLut.output (Operators.mac p.1.1 4 p.1.2) 0
:= by
  unfold hsStage
  rw [Id.run_bind, List.idRun_mapM]
  show _ ++ List.map _ _ = _
  congr 2

theorem hsStage_spec (x y k : Nat) (l : List (CiphertextBlock sb22)) (hl : HsInvariant x y k l) :
  let l' := Id.run (hsStage (M := Id) k l)
  l'.length = l.length ∧ HsInvariant x y (k + 1) l'
:= by
  dsimp only
  rw [hsStage_run]
  have hQ := Nat.two_pow_pos k
  refine ⟨by simp; omega, ?_⟩
  intro g h
  simp only [List.length_append, List.length_take, List.length_map, List.length_zipIdx,
    List.length_zip, List.length_drop] at h
  rw [Nat.pow_succ]
  by_cases hg : g < 2 ^ k
  · rw [List.getElem_append_left (by simp; omega)]
    simp only [List.getElem_take]
    rw [hl g (by omega)]
    simp [hg, (by omega : g < 2 ^ k * 2)]
  · rw [List.getElem_append_right (by simp; omega)]
    have hmin : min (2 ^ k) l.length = 2 ^ k := by omega
    simp only [List.getElem_map, List.getElem_zipIdx, List.getElem_zip, List.getElem_drop,
      List.length_take, Nat.zero_add, hmin]
    obtain ⟨j, rfl⟩ : ∃ j, g = 2 ^ k + j := ⟨g - 2 ^ k, by omega⟩
    simp only [Nat.add_sub_cancel_left]
    have est := hl (2 ^ k + j) (by omega)
    have epv := hl j (by omega)
    simp only [hg, ↓reduceIte, show 2 ^ k + j + 1 - 2 ^ k = j + 1 by omega] at est
    have hst := windowStatus_le_two x y (4 * (j + 1)) (4 * 2 ^ k)
    by_cases hj : j < 2 ^ k
    · simp only [hj, ↓reduceIte] at epv ⊢
      have hpv := carryIn_le_one x y (4 * (j + 1))
      rw [solvePropCarryLut_output _ (by simp only [toCompleteNat_mac, est, epv]; omega)]
      simp only [toCompleteNat_mac, est, epv, (by omega : 2 ^ k + j < 2 ^ k * 2), ↓reduceIte]
      rw [show 4 * (2 ^ k + j + 1) = 4 * (j + 1) + 4 * 2 ^ k by omega, carryIn_window]
      (repeat' split) <;> omega
    · simp only [hj, ↓reduceIte] at epv ⊢
      have hpv := windowStatus_le_two x y (4 * (j + 1 - 2 ^ k)) (4 * 2 ^ k)
      rw [solvePropLut_output _ (by simp only [toCompleteNat_mac, est, epv]; omega)]
      simp only [toCompleteNat_mac, est, epv]
      rw [show 2 ^ k + j + 1 - 2 ^ k * 2 = j + 1 - 2 ^ k by omega,
        show 4 * (2 ^ k * 2) = 4 * 2 ^ k + 4 * 2 ^ k by omega, windowStatus_append,
        show 4 * (j + 1 - 2 ^ k) + 4 * 2 ^ k = 4 * (j + 1) by omega]
      simp only [solve]
      (repeat' split) <;> omega

theorem groupCarries_run (gs : List (CiphertextBlock sb22)) :
  Id.run (groupCarries (M := Id) gs) =
    (List.range (nbStages (chunk4 gs).length)).foldl (fun c s => Id.run (hsStage (M := Id) s c))
      ((chunk4 gs).map (·.2.2.2))
:= by
  simp [groupCarries, List.idRun_foldlM]
  rfl

theorem groupCarries_spec (x y m : Nat) (gs : List (CiphertextBlock sb22))
    (hlen : gs.length = 4 * 2 ^ m) (hgs : GroupStatesSpec x y gs) :
  let cs := Id.run (groupCarries (M := Id) gs)
  cs.length = 2 ^ m ∧ ∀ g (h : g < cs.length), cs[g].toCompleteNat = carryIn x y (4 * (g + 1))
:= by
  dsimp only
  rw [groupCarries_run]
  have hN : (chunk4 gs).length = 2 ^ m := by rw [length_chunk4]; omega
  rw [hN, nbStages, Nat.log2_two_pow]
  refine Util.foldl_invariant (List.range m) (fun c s => Id.run (hsStage (M := Id) s c)) _
    (fun j c => c.length = 2 ^ m ∧ HsInvariant x y j c)
    (fun c => c.length = 2 ^ m ∧
      ∀ g (h : g < c.length), c[g].toCompleteNat = carryIn x y (4 * (g + 1))) ?_ ?_ ?_
  · refine ⟨by simp [hN], ?_⟩
    intro g h
    simp only [List.length_map] at h
    simp only [List.getElem_map, getElem_chunk4 _ g h]
    have := hgs g 3 (by omega) (by rw [hN] at h; omega)
    simp only [show ¬ 3 < 3 by omega, ↓reduceIte] at this
    rw [this]
    by_cases hg : g = 0
    · simp [hg]
    · simp [hg]
  · intro j hj c ⟨hcl, hc⟩
    simp only [List.getElem_range]
    obtain ⟨h1, h2⟩ := hsStage_spec x y j c hc
    exact ⟨h1.trans hcl, h2⟩
  · intro c ⟨hcl, hc⟩
    refine ⟨hcl, fun g h => ?_⟩
    simp only [List.length_range] at hc
    rw [hc g h]
    simp [(by omega : g < 2 ^ m)]

theorem finalResolution_run (gs cs : List (CiphertextBlock sb22)) :
  Id.run (finalResolution (M := Id) gs cs) =
    unchunk4 ((((chunk4 gs).zip cs).zip (none :: cs.map some)).map fun q =>
      match q.2 with
      | none =>
        (q.1.1.1, (solvePropGroupFinalLut 0).output q.1.1.2.1 0,
          (solvePropGroupFinalLut 1).output q.1.1.2.2.1 0, q.1.2)
      | some p =>
        ((solvePropGroupFinalLut 0).output (Operators.add q.1.1.1 p) 0,
          (solvePropGroupFinalLut 1).output (Operators.add q.1.1.2.1 p) 0,
          (solvePropGroupFinalLut 2).output (Operators.add q.1.1.2.2.1 p) 0, q.1.2))
:= by
  unfold finalResolution
  rw [Id.run_bind, List.idRun_mapM]
  show unchunk4 (List.map _ _) = _
  congr 2
  funext q
  obtain ⟨⟨⟨s0, s1, s2, s3⟩, c⟩, prev⟩ := q
  cases prev <;> rfl

theorem finalResolution_spec (x y : Nat) (gs cs : List (CiphertextBlock sb22))
    (hlen : gs.length = 4 * cs.length) (hgs : GroupStatesSpec x y gs)
    (hcs : ∀ g (h : g < cs.length), cs[g].toCompleteNat = carryIn x y (4 * (g + 1))) :
  let bc := Id.run (finalResolution (M := Id) gs cs)
  bc.length = gs.length ∧ CarriesSpec x y bc
:= by
  dsimp only
  rw [finalResolution_run]
  have hN : (chunk4 gs).length = cs.length := by rw [length_chunk4]; omega
  refine ⟨by simp [length_unchunk4, hN]; omega, ?_⟩
  intro i h
  obtain ⟨g, p, hp, rfl⟩ : ∃ g p, p < 4 ∧ i = 4 * g + p := ⟨i / 4, i % 4, by omega, by omega⟩
  have h' := h
  simp only [length_unchunk4, List.length_map, List.length_zip, List.length_cons, hN] at h'
  have hg : g < cs.length := by omega
  rw [getElem_unchunk4 _ g p hp h]
  simp only [List.getElem_map, List.getElem_zip, getElem_chunk4 _ g (by rw [hN]; exact hg)]
  have e0 := hgs g 0 (by omega) (by omega)
  have e1 := hgs g 1 (by omega) (by omega)
  have e2 := hgs g 2 (by omega) (by omega)
  have ec := hcs g hg
  simp only [show (0 : Nat) < 3 by omega, show (1 : Nat) < 3 by omega, show (2 : Nat) < 3 by omega,
    ↓reduceIte] at e0 e1 e2
  cases g with
  | zero =>
    simp only [Nat.mul_zero, Nat.zero_add, ↓reduceIte] at e0 e1 e2 ec ⊢
    simp only [List.getElem_cons_zero]
    have hc1 := carryIn_le_one x y 1
    have l1 := carry_lookahead x y 1 1
    have l2 := carry_lookahead x y 1 2
    have b1 := weightedStatus_le x y 1 1
    have b2 := weightedStatus_le x y 1 2
    rcases (by omega : p = 0 ∨ p = 1 ∨ p = 2 ∨ p = 3) with rfl | rfl | rfl | rfl
    · simp [sel4, e0, weightedStatus]
    · simp only [sel4]
      rw [solvePropGroupFinalLut_output _ _ (by simp at b1; omega), e1]
      simpa using l1
    · simp only [sel4]
      rw [solvePropGroupFinalLut_output _ _ (by simp at b2; omega), e2]
      simpa using l2
    · simp only [sel4, ec]
  | succ g =>
    have hprev := hcs g (by omega)
    simp only [List.getElem_cons_succ, List.getElem_map] at e0 e1 e2 ⊢
    simp only [show 4 * (g + 1) = 4 * g + 4 by omega, show 4 * (g + 1 + 1) = 4 * g + 4 + 4 by omega,
      Nat.add_zero, Nat.add_one_ne_zero, ↓reduceIte, Nat.reduceAdd] at e0 e1 e2 ec hprev ⊢
    have hc := carryIn_le_one x y (4 * g + 4)
    have l1 := carry_lookahead x y (4 * g + 4) 1
    have l2 := carry_lookahead x y (4 * g + 4) 2
    have l3 := carry_lookahead x y (4 * g + 4) 3
    have b1 := weightedStatus_le x y (4 * g + 4) 1
    have b2 := weightedStatus_le x y (4 * g + 4) 2
    have b3 := weightedStatus_le x y (4 * g + 4) 3
    simp only [Nat.reducePow, Nat.reduceAdd] at l1 l2 l3 b1 b2 b3
    simp only [Nat.add_assoc, Nat.reduceAdd] at l1 l2 l3
    rcases (by omega : p = 0 ∨ p = 1 ∨ p = 2 ∨ p = 3) with rfl | rfl | rfl | rfl
    · simp only [sel4]
      rw [solvePropGroupFinalLut_output _ _ (by simp only [toCompleteNat_add, e0, hprev]; omega)]
      simp only [toCompleteNat_add, e0, hprev, Nat.add_assoc, Nat.reduceAdd, Nat.reducePow,
        Fin.val_zero]
      omega
    · simp only [sel4]
      rw [solvePropGroupFinalLut_output _ _ (by simp only [toCompleteNat_add, e1, hprev]; omega)]
      simp only [toCompleteNat_add, e1, hprev, Nat.add_assoc, Nat.reduceAdd, Nat.reducePow,
        Fin.val_one]
      omega
    · simp only [sel4]
      rw [solvePropGroupFinalLut_output _ _ (by simp only [toCompleteNat_add, e2, hprev]; omega)]
      simp only [toCompleteNat_add, e2, hprev, Nat.add_assoc, Nat.reduceAdd, Nat.reducePow,
        Fin.val_two]
      omega
    · simp only [sel4, ec, Nat.add_assoc, Nat.reduceAdd]

theorem propagate_run (s0 : CiphertextBlock sb22) (rest carries : List (CiphertextBlock sb22)) :
  Id.run (propagate (M := Id) (s0 :: rest) carries) =
    msgOnlyLut.output (carryMsgLut.output s0 1) 0
      :: (rest.zip carries).map fun q => msgOnlyLut.output (Operators.add q.1 q.2) 0
:= by
  simp [propagate, List.idRun_mapM]
  exact ⟨rfl, fun _ _ _ => rfl⟩

theorem propagate_spec (x y : Nat) (sums carries : List (CiphertextBlock sb22))
    (hlen : carries.length = sums.length) (hs : SumsSpec x y sums) (hc : CarriesSpec x y carries) :
  let r := Id.run (propagate (M := Id) sums carries)
  r.length = sums.length ∧ ∀ i (h : i < r.length), r[i].toCompleteNat = digit (x + y) i
:= by
  dsimp only
  match sums, hlen, hs with
  | [], _, _ => exact ⟨rfl, fun i h => absurd h (by simp [propagate])⟩
  | s0 :: rest, hlen, hs =>
    rw [propagate_run]
    simp only [List.length_cons] at hlen
    refine ⟨by simp; omega, ?_⟩
    intro i h
    cases i with
    | zero =>
      have e := hs 0 (by simp)
      have := digit_lt x 0
      have := digit_lt y 0
      simp only [List.getElem_cons_zero] at e ⊢
      rw [msgOnlyLut_output _ (by rw [(pbs2_carryMsgLut s0 (by omega)).2]; omega),
        (pbs2_carryMsgLut s0 (by omega)).2, e, digit_add, carryIn_zero]
      omega
    | succ i =>
      simp only [List.length_cons, List.length_map, List.length_zip] at h
      have e := hs (i + 1) (by simp; omega)
      have ec := hc i (by omega)
      have := digit_lt x (i + 1)
      have := digit_lt y (i + 1)
      have := carryIn_le_one x y (i + 1)
      simp only [List.getElem_cons_succ] at e
      simp only [List.getElem_cons_succ, List.getElem_map, List.getElem_zip]
      rw [msgOnlyLut_output _ (by simp only [toCompleteNat_add, e, ec]; omega)]
      simp only [toCompleteNat_add, e, ec, digit_add]
      omega

theorem joinBlocks_run (si : SpecInteger sb22) (blocks : List (CiphertextBlock sb22)) :
  Id.run (joinBlocks (M := Id) si blocks) =
    (blocks.zip (List.finRange si.blockCount)).foldl (fun acc q => acc.setBlock q.2 q.1)
      (CiphertextInteger.ofNat 0)
:= by
  simp [joinBlocks, List.idRun_foldlM]
  rfl

theorem joinBlocks_spec (si : SpecInteger sb22) (v : Nat) (blocks : List (CiphertextBlock sb22))
    (hlen : si.blockCount ≤ blocks.length)
    (hb : ∀ i (h : i < blocks.length), blocks[i].toCompleteNat = digit v i) :
  let r := Id.run (joinBlocks (M := Id) si blocks)
  r.toNat = v % 2 ^ si.intSize ∧ r.IsClean
:= by
  dsimp only
  rw [joinBlocks_run]
  refine Util.foldl_invariant (blocks.zip (List.finRange si.blockCount))
    (fun acc q => acc.setBlock q.2 q.1) (CiphertextInteger.ofNat 0)
    (fun j (acc : CiphertextInteger si) => acc.toNat = v % 4 ^ j ∧ acc.IsClean)
    (fun acc => acc.toNat = v % 2 ^ si.intSize ∧ acc.IsClean) ?_ ?_ ?_
  · exact ⟨by simp [Nat.mod_one], CiphertextInteger.isClean_ofNat 0⟩
  · intro j hj acc ⟨hacc, hclean⟩
    simp only [List.length_zip, List.length_finRange] at hj
    simp only [List.getElem_zip, List.getElem_finRange, Fin.cast_mk]
    have e := hb j (by omega)
    have hlt : acc.toNat < 2 ^ (j * sb22.messageSize) := by
      rw [show sb22.messageSize = 2 from rfl, two_pow_mul_two, hacc]
      exact Nat.mod_lt _ (four_pow_pos j)
    have hmsg : blocks[j].hasMessageOnly := by
      rw [CiphertextBlock.hasMessageOnly_iff, e]
      exact digit_lt v j
    refine ⟨?_, acc.isClean_setBlock _ _ hclean hmsg⟩
    rw [CiphertextInteger.toNat_setBlock_of_lt _ _ _ hlt, CiphertextBlock.toMessageNat_eq, e, hacc,
      show sb22.messageSize = 2 from rfl, Nat.mod_eq_of_lt (digit_lt v j : digit v j < 2 ^ 2),
      two_pow_mul_two]
    show v % 4 ^ j + digit v j * 4 ^ j = v % 4 ^ (j + 1)
    rw [Nat.pow_succ, Nat.mod_mul, Nat.mul_comm]
    rfl
  · intro acc ⟨hacc, hclean⟩
    simp only [List.length_zip, List.length_finRange, Nat.min_eq_right hlen] at hacc
    refine ⟨?_, hclean⟩
    rw [hacc, SpecInteger.intSize, show sb22.messageSize = 2 from rfl, two_pow_mul_two]

section Traced

open IopAlgebra.Traced

syntax "projects_step" : tactic

macro_rules
  | `(tactic| projects_step) => `(tactic| first
      | exact Projects.pure _ _
      | projects_op
      | (refine Projects.bind (f := model) ?_ fun _ => ?_
         focus projects_op
         projects_step))

variable {α α' β β' : Type} [HasModel α α'] [HasModel β β']

theorem model_append (l₁ l₂ : List α) :
  (model (l₁ ++ l₂) : List α') = model l₁ ++ model l₂
:=
  List.map_append

theorem model_replicate (n : Nat) (a : α) :
  (model (List.replicate n a) : List α') = List.replicate n (model a)
:=
  List.map_replicate

theorem model_take (n : Nat) (l : List α) :
  (model (l.take n) : List α') = (model l : List α').take n
:=
  List.map_take

theorem model_drop (n : Nat) (l : List α) :
  (model (l.drop n) : List α') = (model l : List α').drop n
:=
  List.map_drop

theorem model_length (l : List α) :
  (model l : List α').length = l.length
:=
  List.length_map ..

theorem model_zip : ∀ (l₁ : List α) (l₂ : List β),
  (model (l₁.zip l₂) : List (α' × β')) = (model l₁ : List α').zip (model l₂ : List β')
  | [], _ => rfl
  | _ :: _, [] => rfl
  | _ :: l₁, _ :: l₂ => by simp only [List.zip_cons_cons, model_cons, model_pair, model_zip l₁ l₂]

theorem model_zipIdx : ∀ (l : List α) (k : Nat),
  (model (l.zipIdx k) : List (α' × Nat)) = (model l : List α').zipIdx k
  | [], _ => rfl
  | _ :: l, k => by simp only [List.zipIdx_cons, model_cons, model_pair, model_nat, model_zipIdx l]

theorem model_chunk4 : ∀ (l : List α),
  (model (chunk4 l) : List (α' × α' × α' × α')) = chunk4 (model l : List α')
  | _ :: _ :: _ :: _ :: rest => by simp only [chunk4, model_cons, model_pair, model_chunk4 rest]
  | [] => rfl
  | [_] => rfl
  | [_, _] => rfl
  | [_, _, _] => rfl

theorem model_unchunk4 : ∀ (L : List (α × α × α × α)),
  (model (unchunk4 L) : List α') = unchunk4 (model L : List (α' × α' × α' × α'))
  | [] => rfl
  | (_, _, _, _) :: rest => by simp only [unchunk4, model_cons, model_pair, model_unchunk4 rest]

theorem model_finRange (n : Nat) :
  (model (List.finRange n) : List (Fin n)) = List.finRange n
:=
  List.map_id _

theorem rawSums_traced_projects (si : SpecInteger sb22) (x y : TrCtInteger si) :
  Projects (rawSums (M := TraceM) si x y) model (rawSums (M := Id) si x.model y.model)
:= by
  unfold rawSums
  refine Projects.bind (Projects.mapM_plain fun i => ?_) fun sums => ?_
  · projects_step
  · refine Projects.bind (Projects.letCtBlock 0) fun zero => Projects.pure_eq ?_
    rw [model_append, model_replicate]
    rfl

theorem blockStates_traced_projects (sums : List (IopAlgebra.CtBlock sb22 TraceM)) :
  Projects (blockStates (M := TraceM) sums) model (blockStates (M := Id) (model sums))
:= by
  unfold blockStates
  rw [← model_zipIdx]
  refine Projects.mapM fun p => ?_
  obtain ⟨s, i⟩ := p
  simp only [model_pair, model_nat]
  by_cases hi : i = 0
  · simp only [hi, ↓reduceIte]
    projects_step
  · by_cases h4 : i < 4 <;> simp only [hi, h4, ↓reduceIte] <;> projects_step

theorem groupStates_traced_projects (states : List (IopAlgebra.CtBlock sb22 TraceM)) :
  Projects (groupStates (M := TraceM) states) model (groupStates (M := Id) (model states))
:= by
  unfold groupStates
  rw [← model_chunk4, ← model_zipIdx]
  refine Projects.bind (Projects.mapM fun p => ?_) fun groups => Projects.pure_eq (model_unchunk4 groups)
  obtain ⟨⟨s0, s1, s2, s3⟩, g⟩ := p
  simp only [model_pair, model_nat]
  by_cases hg : g = 0 <;> simp only [hg, ↓reduceIte] <;> projects_step

theorem hsStage_traced_projects (k : Nat) (carries : List (IopAlgebra.CtBlock sb22 TraceM)) :
  Projects (hsStage (M := TraceM) k carries) model (hsStage (M := Id) k (model carries))
:= by
  unfold hsStage
  simp only [← model_drop, ← model_zip, ← model_zipIdx]
  refine Projects.bind (Projects.mapM fun p => ?_) fun solved => Projects.pure_eq ?_
  · obtain ⟨⟨st, pv⟩, j⟩ := p
    simp only [model_pair, model_nat]
    by_cases hj : j < 2 ^ k <;> simp only [hj, ↓reduceIte] <;> projects_step
  · rw [model_append, model_take]
    rfl

theorem groupCarries_traced_projects (groups : List (IopAlgebra.CtBlock sb22 TraceM)) :
  Projects (groupCarries (M := TraceM) groups) model (groupCarries (M := Id) (model groups))
:= by
  have hinit : (chunk4 (α := IopAlgebra.CtBlock sb22 Id) (model groups)).map (·.2.2.2)
      = (model ((chunk4 groups).map (·.2.2.2)) : List (IopAlgebra.CtBlock sb22 Id)) := by
    rw [← model_chunk4]
    simp only [model, List.map_map]
    rfl
  have hlen : List.length (α := IopAlgebra.CtBlock sb22 Id) (model ((chunk4 groups).map (·.2.2.2)))
      = ((chunk4 groups).map (·.2.2.2)).length :=
    List.length_map ..
  unfold groupCarries
  simp only [hinit, hlen]
  refine Projects.bind (Projects.forIn fun stage r => ?_) fun r => Projects.pure _ _
  exact Projects.bind (hsStage_traced_projects stage r) fun _ => Projects.pure _ _

theorem finalResolution_traced_projects (groups carries : List (IopAlgebra.CtBlock sb22 TraceM)) :
  Projects (finalResolution (M := TraceM) groups carries) model
    (finalResolution (M := Id) (model groups) (model carries))
:= by
  have hl : (((chunk4 (model groups)).zip (model carries)).zip
        (none :: (model carries : List (IopAlgebra.CtBlock sb22 Id)).map some) :
          List ((_ × IopAlgebra.CtBlock sb22 Id) × Option (IopAlgebra.CtBlock sb22 Id)))
      = model (((chunk4 groups).zip carries).zip (none :: carries.map some)) := by
    rw [model_zip, model_zip, model_chunk4, model_cons, model_none]
    simp only [model, List.map_map]
    rfl
  unfold finalResolution
  simp only [hl]
  refine Projects.bind (Projects.mapM fun q => ?_) fun r => Projects.pure_eq (model_unchunk4 r)
  obtain ⟨⟨⟨s0, s1, s2, s3⟩, c⟩, prev⟩ := q
  cases prev <;> simp only [model_pair, model_none, model_some] <;> projects_step

theorem propagate_traced_projects (sums carries : List (IopAlgebra.CtBlock sb22 TraceM)) :
  Projects (propagate (M := TraceM) sums carries) model (propagate (M := Id) (model sums) (model carries))
:= by
  cases sums with
  | nil => exact Projects.pure_eq rfl
  | cons s0 rest =>
    unfold propagate
    simp only [model_cons, ← model_zip]
    refine Projects.bind (Projects.pbs2 carryMsgLut s0) fun r => ?_
    refine Projects.bind (Projects.mapM fun q => ?_) fun added => ?_
    · obtain ⟨s, c⟩ := q
      exact Projects.addCt s c
    · exact Projects.mapM (l := r.2 :: added) fun b => Projects.pbs msgOnlyLut b

theorem joinBlocks_traced_projects (si : SpecInteger sb22) (blocks : List (IopAlgebra.CtBlock sb22 TraceM)) :
  Projects (joinBlocks (M := TraceM) si blocks) model (joinBlocks (M := Id) si (model blocks))
:= by
  have hl : ((model blocks : List (IopAlgebra.CtBlock sb22 Id)).zip (List.finRange si.blockCount))
      = model (blocks.zip (List.finRange si.blockCount)) := by
    rw [model_zip, model_finRange]
  unfold joinBlocks
  simp only [hl]
  refine Projects.bind (Projects.declareInteger si) fun acc => ?_
  refine Projects.bind (Projects.forIn' fun q a => ?_) fun r => Projects.pure _ _
  obtain ⟨b, i⟩ := q
  simp only [model_pair, model_fin]
  projects_step

end Traced

end HillisSteele

open HillisSteele in

theorem addHillisSteele_spec (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (hx : x.IsClean) (hy : y.IsClean) :
  (addHillisSteele (M := Id) si x y).toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  ∧ (addHillisSteele (M := Id) si x y).IsClean
:= by
  by_cases hn : si.blockCount = 0
  · have hlen := (addHillisSteele (M := Id) si x y).length_eq
    have hlt := (addHillisSteele (M := Id) si x y).toNat_lt
    simp only [SpecInteger.intSize, hn, Nat.zero_mul, Nat.pow_zero] at hlt hlen ⊢
    refine ⟨by omega, ?_⟩
    intro b hb
    rw [List.eq_nil_of_length_eq_zero hlen] at hb
    cases hb
  · obtain ⟨m, hm⟩ := computeSize_eq si.blockCount (by omega)
    obtain ⟨h1l, h1⟩ := rawSums_spec si x y hx hy
    obtain ⟨h2l, h2⟩ := blockStates_spec _ _ _ h1
    obtain ⟨h3l, h3⟩ := groupStates_spec _ _ _ (by omega) h2
    obtain ⟨h4l, h4⟩ := groupCarries_spec _ _ m _ (by omega) h3
    obtain ⟨h5l, h5⟩ := finalResolution_spec _ _ _ _ (by omega) h3 h4
    obtain ⟨h6l, h6⟩ := propagate_spec _ _ _ _ (by omega) h1 h5
    have hfin := joinBlocks_spec si _ _ (by have := le_computeSize si.blockCount; omega) h6
    exact hfin

theorem addHillisSteele_correct (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (hx : x.IsClean) (hy : y.IsClean) :
  (addHillisSteele (M := Id) si x y).toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
:=
  (addHillisSteele_spec si x y hx hy).1

theorem addHillisSteele_isClean (si : SpecInteger sb22) (x y : CiphertextInteger si)
    (hx : x.IsClean) (hy : y.IsClean) : (addHillisSteele (M := Id) si x y).IsClean
:=
  (addHillisSteele_spec si x y hx hy).2

open HillisSteele IopAlgebra.Traced in

theorem addHillisSteele_traced_projects (si : SpecInteger sb22) (x y : TrCtInteger si) :
  (addHillisSteele (M := TraceM) si x y).ModelIs (addHillisSteele (M := Id) si x.model y.model)
:=
  Projects.bind (rawSums_traced_projects si x y) fun sums =>
  Projects.bind (blockStates_traced_projects sums) fun states =>
  Projects.bind (groupStates_traced_projects states) fun groups =>
  Projects.bind (groupCarries_traced_projects groups) fun carries =>
  Projects.bind (finalResolution_traced_projects groups carries) fun blockCarries =>
  Projects.bind (propagate_traced_projects sums blockCarries) fun res =>
  joinBlocks_traced_projects si res

end Zhc.Iops
