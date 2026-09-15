import Zhc.Model.SpecInteger
import Zhc.Model.CiphertextBlock

namespace Zhc.Model

structure CiphertextInteger (si : SpecInteger sb) where
  blocks : List (CiphertextBlock sb)
  length_eq : blocks.length = si.blockCount
  deriving DecidableEq, Repr

namespace CiphertextInteger variable {sb : SpecBlock} {si : SpecInteger sb}

def ofNat (v : Nat) : CiphertextInteger si :=
  ⟨CiphertextBlock.digits si.blockCount v, CiphertextBlock.length_digits _ _⟩

def toNat (self : CiphertextInteger si) : Nat :=
  CiphertextBlock.listToNat self.blocks

abbrev IsClean (self : CiphertextInteger si) : Prop :=
  ∀ b ∈ self.blocks, b.hasMessageOnly

def getBlock (self : CiphertextInteger si) (ith : Fin si.blockCount) : CiphertextBlock sb :=
  self.blocks[ith.val]'(by rw [self.length_eq]; exact ith.isLt)

def setBlock (self : CiphertextInteger si) (ith : Fin si.blockCount) (block : CiphertextBlock sb) :
    CiphertextInteger si :=
  ⟨self.blocks.set ith.val block, by rw [List.length_set]; exact self.length_eq⟩

theorem toNat_lt (x : CiphertextInteger si) :
  x.toNat < 2 ^ si.intSize
:= by
  show CiphertextBlock.listToNat x.blocks < 2 ^ (si.blockCount * sb.messageSize)
  rw [← x.length_eq]
  exact CiphertextBlock.listToNat_lt x.blocks

@[simp] theorem toNat_ofNat (v : Nat) :
  (ofNat v : CiphertextInteger si).toNat = v % 2 ^ si.intSize
:=
  CiphertextBlock.listToNat_digits _ _

theorem isClean_ofNat (v : Nat) :
  (ofNat v : CiphertextInteger si).IsClean
:=
  CiphertextBlock.hasMessageOnly_of_mem_digits _ _

theorem toCompleteNat_getBlock_of_clean (x : CiphertextInteger si) (hx : x.IsClean)
    (i : Fin si.blockCount) :
  (x.getBlock i).toCompleteNat = x.toNat / 2 ^ (i.val * sb.messageSize) % 2 ^ sb.messageSize
:=
  CiphertextBlock.toCompleteNat_getElem_of_clean x.blocks hx i.val _

theorem toNat_setBlock_of_lt (x : CiphertextInteger si) (i : Fin si.blockCount)
    (b : CiphertextBlock sb) (h : x.toNat < 2 ^ (i.val * sb.messageSize)) :
  (x.setBlock i b).toNat = x.toNat + b.toMessageNat * 2 ^ (i.val * sb.messageSize)
:=
  CiphertextBlock.listToNat_set_of_lt x.blocks i.val b (by rw [x.length_eq]; exact i.isLt) h

theorem isClean_setBlock (x : CiphertextInteger si) (i : Fin si.blockCount)
    (b : CiphertextBlock sb) (hx : x.IsClean) (hb : b.hasMessageOnly) :
  (x.setBlock i b).IsClean
:=
  Util.List.forall_mem_set hx hb

end CiphertextInteger
end Zhc.Model
