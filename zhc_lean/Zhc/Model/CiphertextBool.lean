import Zhc.Model.SpecBlock
import Zhc.Model.CiphertextBlock

namespace Zhc.Model

structure CiphertextBool (sb : SpecBlock) where
  block : CiphertextBlock sb
  deriving DecidableEq, Repr

namespace CiphertextBool variable {sb : SpecBlock}

def ofBool (value : Bool) : CiphertextBool sb :=
  ⟨CiphertextBlock.ofNat (if value then 1 else 0)⟩

def toBool(self: CiphertextBool sb) : Bool := decide (self.block.toMessageNat = 1)

abbrev IsClean (self : CiphertextBool sb) : Prop :=
  self.block.toCompleteNat ≤ 1

theorem hasMessageOnly_of_isClean (sbv : sb.Valid) {b : CiphertextBool sb} :
  b.IsClean → b.block.hasMessageOnly
:= by
  intro h
  rw [CiphertextBlock.hasMessageOnly_iff]
  have h2 : 2 ^ 1 ≤ 2 ^ sb.messageSize := Nat.pow_le_pow_right (by decide) (SpecBlock.Valid.message_pos sbv)
  simp only [Nat.pow_one] at h2
  omega

theorem isClean_ofBool (v : Bool) :
  (ofBool v : CiphertextBool sb).IsClean
:= by
  unfold IsClean ofBool
  simp only [CiphertextBlock.toCompleteNat_ofNat]
  cases v <;> simp <;> exact Nat.le_trans (Nat.mod_le _ _) (by decide)

end CiphertextBool

end Zhc.Model
