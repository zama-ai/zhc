import Zhc.Model.SpecBlock
import Zhc.Util

namespace Zhc.Model

structure CiphertextBlock (sb : SpecBlock) where
  storage : Fin (2 ^ sb.completeSize)
  deriving DecidableEq, Repr

namespace CiphertextBlock variable {sb: SpecBlock}

def ofNat (v : Nat) : CiphertextBlock sb :=
  ⟨⟨v % 2 ^ sb.completeSize, Util.mod_two_pow_lt⟩⟩

def ofNat? (v : Nat) : Option (CiphertextBlock sb) :=
  if v < 2 ^ sb.completeSize then some (ofNat v) else none

def toCompleteNat (self: CiphertextBlock sb) : Nat :=
  self.storage.val

def toMessageNat (self: CiphertextBlock sb): Nat :=
  self.toCompleteNat % 2 ^ sb.messageSize

def toCarryNat (self: CiphertextBlock sb) : Nat :=
  (self.toCompleteNat / 2 ^ sb.messageSize) % 2 ^ sb.carrySize

def toPaddingNat (self: CiphertextBlock sb) : Nat :=
  self.toCompleteNat / 2 ^ sb.dataSize

def toDataNat (self: CiphertextBlock sb) : Nat :=
  self.toCompleteNat % 2 ^ sb.dataSize

abbrev hasActivePaddingBit (self: CiphertextBlock sb) : Prop :=
  self.toPaddingNat = 1

abbrev hasCarryOnly (self: CiphertextBlock sb) : Prop :=
  self.toCompleteNat / 2 ^ sb.dataSize = 0

abbrev hasMessageOnly (self: CiphertextBlock sb) : Prop :=
  self.toCompleteNat / 2 ^ sb.messageSize = 0

def doMaskMessage (self: CiphertextBlock sb) : CiphertextBlock sb :=
  ofNat self.toMessageNat

def doMaskCarry (self: CiphertextBlock sb): CiphertextBlock sb :=
  ofNat (self.toCarryNat * 2 ^ sb.messageSize)

def doMoveCarryToMessage (self: CiphertextBlock sb): CiphertextBlock sb :=
  ofNat self.toCarryNat

def doNeg (self : CiphertextBlock sb) : CiphertextBlock sb :=
  ofNat (2 ^ sb.completeSize - self.toCompleteNat)

@[simp] theorem toCompleteNat_ofNat (v : Nat) :
  (ofNat v : CiphertextBlock sb).toCompleteNat = v % 2 ^ sb.completeSize
:=
  rfl

theorem toCompleteNat_lt (a : CiphertextBlock sb) :
  a.toCompleteNat < 2 ^ sb.completeSize
:=
  a.storage.isLt

theorem ext_toCompleteNat {a b : CiphertextBlock sb} :
  a.toCompleteNat = b.toCompleteNat → a = b
:= by
  intro h
  cases a
  cases b
  congr
  exact Fin.ext h

@[simp] theorem toMessageNat_eq (a : CiphertextBlock sb) :
  a.toMessageNat = a.toCompleteNat % 2 ^ sb.messageSize
:=
  rfl

@[simp] theorem toCarryNat_eq (a : CiphertextBlock sb) :
  a.toCarryNat = (a.toCompleteNat / 2 ^ sb.messageSize ) % 2 ^ sb.carrySize
:=
  rfl

@[simp] theorem toPaddingNat_eq (a : CiphertextBlock sb) :
  a.toPaddingNat = a.toCompleteNat / 2 ^ sb.dataSize
:=
  rfl

@[simp] theorem toDataNat_eq (a : CiphertextBlock sb) :
  a.toDataNat = a.toCompleteNat % 2 ^ sb.dataSize
:=
  rfl

@[simp] theorem hasMessageOnly_iff (a : CiphertextBlock sb) :
  a.hasMessageOnly ↔ a.toCompleteNat < 2 ^ sb.messageSize
:=
  Util.div_two_pow_eq_zero_iff

@[simp] theorem hasCarryOnly_iff (a : CiphertextBlock sb) :
  a.hasCarryOnly ↔ a.toCompleteNat < 2 ^ sb.dataSize
:=
  Util.div_two_pow_eq_zero_iff

theorem not_hasActivePaddingBit_of_lt {a : CiphertextBlock sb} :
  a.toCompleteNat < 2 ^ sb.dataSize → ¬ a.hasActivePaddingBit
:= by
  intro h
  simp only [hasActivePaddingBit, toPaddingNat_eq, Nat.div_eq_of_lt h]
  decide

theorem toMessageNat_eq_of_hasMessageOnly {a : CiphertextBlock sb} :
  a.hasMessageOnly → a.toMessageNat = a.toCompleteNat
:= by
  intro h
  exact Nat.mod_eq_of_lt ((hasMessageOnly_iff a).mp h)

theorem toDataNat_eq_of_lt {a : CiphertextBlock sb} :
  a.toCompleteNat < 2 ^ sb.dataSize → a.toDataNat = a.toCompleteNat
:= by
  intro h
  exact Nat.mod_eq_of_lt h

@[simp] theorem toCompleteNat_doMaskMessage (a : CiphertextBlock sb) :
  a.doMaskMessage.toCompleteNat = a.toCompleteNat % 2 ^ sb.messageSize
:=
  Util.mod_two_pow_eq_of_lt Util.mod_two_pow_lt sb.messageSize_le_completeSize

@[simp] theorem toCompleteNat_doMoveCarryToMessage (a : CiphertextBlock sb) :
  a.doMoveCarryToMessage.toCompleteNat = a.toCompleteNat / 2 ^ sb.messageSize % 2 ^ sb.carrySize
:=
  Util.mod_two_pow_eq_of_lt Util.mod_two_pow_lt sb.carrySize_le_completeSize

def listToNat : List (CiphertextBlock sb) → Nat
  | [] => 0
  | b :: bs => b.toMessageNat + 2 ^ sb.messageSize * listToNat bs

@[simp] theorem listToNat_nil :
  listToNat ([] : List (CiphertextBlock sb)) = 0
:=
  rfl

@[simp] theorem listToNat_cons (b : CiphertextBlock sb) (bs : List (CiphertextBlock sb)) :
  listToNat (b :: bs) = b.toMessageNat + 2 ^ sb.messageSize * listToNat bs
:=
  rfl

theorem toMessageNat_lt (b : CiphertextBlock sb) :
  b.toMessageNat < 2 ^ sb.messageSize
:=
  Util.mod_two_pow_lt

theorem listToNat_lt (l : List (CiphertextBlock sb)) :
  listToNat l < 2 ^ (l.length * sb.messageSize)
:= by
  induction l with
  | nil => simp
  | cons b bs ih =>
    simp only [listToNat_cons, List.length_cons]
    rw [← Util.two_pow_mul_succ, Nat.mul_comm (2 ^ (bs.length * sb.messageSize))]
    have h1 := Nat.mul_le_mul_left (2 ^ sb.messageSize) ih
    simp only [Nat.mul_succ] at h1
    have h2 := b.toMessageNat_lt
    omega

theorem toCompleteNat_getElem_of_clean (l : List (CiphertextBlock sb))
    (hl : ∀ b ∈ l, b.hasMessageOnly) (i : Nat) (hi : i < l.length) :
  l[i].toCompleteNat = listToNat l / 2 ^ (i * sb.messageSize) % 2 ^ sb.messageSize
:= by
  induction i generalizing l with
  | zero =>
    match l, hi with
    | b :: bs, _ =>
      have hb := hl b (by simp)
      simp only [List.getElem_cons_zero, listToNat_cons, Nat.zero_mul, Nat.pow_zero, Nat.div_one,
        Nat.add_mul_mod_self_left, Nat.mod_mod, toMessageNat_eq]
      exact (Nat.mod_eq_of_lt ((hasMessageOnly_iff b).mp hb)).symm
  | succ i ih =>
    match l, hi with
    | b :: bs, hi =>
      have hbs : ∀ c ∈ bs, c.hasMessageOnly := fun c hc => hl c (by simp [hc])
      simp only [List.getElem_cons_succ, listToNat_cons]
      rw [ih bs hbs (by simpa using hi)]
      congr 1
      rw [← Util.two_pow_mul_succ, Nat.mul_comm (2 ^ (i * sb.messageSize)),
        ← Nat.div_div_eq_div_mul, Nat.add_mul_div_left _ _ (Util.two_pow_pos _),
        Nat.div_eq_of_lt b.toMessageNat_lt, Nat.zero_add]

theorem listToNat_set_of_lt (l : List (CiphertextBlock sb)) (i : Nat) (b : CiphertextBlock sb) :
  i < l.length → listToNat l < 2 ^ (i * sb.messageSize) →
  listToNat (l.set i b) = listToNat l + b.toMessageNat * 2 ^ (i * sb.messageSize)
:= by
  induction i generalizing l with
  | zero =>
    intro hi hl
    match l, hi with
    | c :: cs, _ =>
      simp only [List.set_cons_zero, listToNat_cons, Nat.zero_mul, Nat.pow_zero, Nat.lt_one_iff,
        Nat.mul_one] at *
      omega
  | succ i ih =>
    intro hi hl
    match l, hi with
    | c :: cs, hi =>
      simp only [List.set_cons_succ, listToNat_cons] at *
      rw [← Util.two_pow_mul_succ] at hl ⊢
      have hT : listToNat cs < 2 ^ (i * sb.messageSize) := by
        rw [Nat.mul_comm (2 ^ (i * sb.messageSize))] at hl
        exact Nat.lt_of_mul_lt_mul_left (Nat.lt_of_le_of_lt (Nat.le_add_left _ _) hl)
      rw [ih cs (by simpa using hi) hT, Nat.mul_add]
      ac_rfl

def digits (n v : Nat) : List (CiphertextBlock sb) :=
  match n with
  | 0 => []
  | n + 1 => ofNat (v % 2 ^ sb.messageSize) :: digits n (v / 2 ^ sb.messageSize)

@[simp] theorem length_digits (n v : Nat) :
  (digits (sb := sb) n v).length = n
:= by
  induction n generalizing v with
  | zero => rfl
  | succ n ih => simp [digits, ih]

theorem listToNat_digits (n v : Nat) :
  listToNat (digits (sb := sb) n v) = v % 2 ^ (n * sb.messageSize)
:= by
  induction n generalizing v with
  | zero => simp [digits, Nat.mod_one]
  | succ n ih =>
    simp only [digits, listToNat_cons, ih, toMessageNat_eq, toCompleteNat_ofNat]
    rw [Util.mod_two_pow_eq_of_lt Util.mod_two_pow_lt sb.messageSize_le_completeSize, Nat.mod_mod,
      ← Util.two_pow_mul_succ, Nat.mul_comm (2 ^ (n * sb.messageSize)), Nat.mod_mul]

theorem hasMessageOnly_of_mem_digits (n v : Nat) :
  ∀ b ∈ digits (sb := sb) n v, b.hasMessageOnly
:= by
  induction n generalizing v with
  | zero => simp [digits]
  | succ n ih =>
    intro b hb
    simp only [digits, List.mem_cons] at hb
    rcases hb with rfl | hb
    · rw [hasMessageOnly_iff, toCompleteNat_ofNat]
      exact Nat.lt_of_le_of_lt (Nat.mod_le _ _) Util.mod_two_pow_lt
    · exact ih _ b hb

end CiphertextBlock
end Zhc.Model
