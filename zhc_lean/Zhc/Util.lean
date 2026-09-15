namespace Util

theorem two_pow_pos:
  ∀ (n: Nat), 0 < 2 ^ n
:= by
  intro n
  apply Nat.two_pow_pos n

theorem mod_two_pow_lt :
  ∀ {r n : Nat}, r % 2 ^ n < 2 ^ n
:= by
  intro r n
  apply Nat.mod_lt _ (two_pow_pos n)

theorem two_pow_le_two_pow {m n : Nat} :
  m ≤ n → 2 ^ m ≤ 2 ^ n
:= by
  intro h
  exact Nat.pow_le_pow_right (by decide) h

theorem lt_two_pow_of_le {v m n : Nat} :
  v < 2 ^ m → m ≤ n → v < 2 ^ n
:= by
  intro h hmn
  exact Nat.lt_of_lt_of_le h (two_pow_le_two_pow hmn)

theorem mod_two_pow_eq_of_lt {v m n : Nat} :
  v < 2 ^ m → m ≤ n → v % 2 ^ n = v
:= by
  intro h hmn
  exact Nat.mod_eq_of_lt (lt_two_pow_of_le h hmn)

theorem div_two_pow_eq_zero_iff {a n : Nat} :
  a / 2 ^ n = 0 ↔ a < 2 ^ n
:=
  Nat.div_eq_zero_iff_lt (two_pow_pos n)

theorem div_two_pow_sub_lt {v m n : Nat} :
  v < 2 ^ m → v / 2 ^ (m - n) < 2 ^ n
:= by
  intro h
  by_cases hnm : n ≤ m
  · apply Nat.div_lt_of_lt_mul
    rw [← Nat.pow_add, Nat.sub_add_cancel hnm]
    exact h
  · rw [Nat.sub_eq_zero_of_le (by omega), Nat.pow_zero, Nat.div_one]
    exact lt_two_pow_of_le h (by omega)

theorem two_pow_mul_succ (k m : Nat) :
  2 ^ (k * m) * 2 ^ m = 2 ^ ((k + 1) * m)
:= by
  rw [← Nat.pow_add, Nat.succ_mul]


theorem List.forall_mem_set {α : Type} {P : α → Prop} {l : List α} {i : Nat} {b : α} :
  (∀ a ∈ l, P a) → P b → ∀ a ∈ l.set i b, P a
:= by
  intro hl hb a ha
  rcases _root_.List.mem_or_eq_of_mem_set ha with h | h
  · exact hl a h
  · exact h ▸ hb

theorem forIn_id_yield {α β : Type} (l : List α) (init : β) (g : α → β → β) :
  (forIn (m := Id) l init fun a b => ForInStep.yield (g a b)) = l.foldl (fun b a => g a b) init
:=
  _root_.List.forIn_pure_yield_eq_foldl (m := Id) g init

theorem foldl_invariant {α β : Type} (l : List α) (g : β → α → β) (init : β)
    (Inv : Nat → β → Prop) (P : β → Prop)
    (h0 : Inv 0 init)
    (hstep : ∀ (j : Nat) (hj : j < l.length) (s : β), Inv j s → Inv (j + 1) (g s l[j]))
    (hfin : ∀ s, Inv l.length s → P s) :
  P (l.foldl g init)
:= by
  induction l generalizing init Inv with
  | nil => exact hfin init h0
  | cons a l ih =>
    simp only [_root_.List.foldl_cons]
    refine ih (g init a) (fun j s => Inv (j + 1) s) (hstep 0 (by simp) init h0) ?_ ?_
    · intro j hj s hs
      exact hstep (j + 1) (by simp; omega) s hs
    · intro s hs
      exact hfin s (by simpa [_root_.List.length_cons] using hs)

end Util
