import Zhc.Model.SpecInteger
import Zhc.Model.PlaintextBlock

namespace Zhc.Model

structure PlaintextInteger (si : SpecInteger sb) where
  blocks : List (PlaintextBlock sb)
  length_eq : blocks.length = si.blockCount
  deriving DecidableEq, Repr

namespace PlaintextInteger variable {sb : SpecBlock} {si : SpecInteger sb}

def ofNat (v : Nat) : PlaintextInteger si :=
  ⟨PlaintextBlock.digits si.blockCount v, PlaintextBlock.length_digits _ _⟩

def ofNat? (v : Nat) : Option (PlaintextInteger si) :=
  if v < 2 ^ si.intSize then some (ofNat v) else none

def toNat (self : PlaintextInteger si) : Nat :=
  PlaintextBlock.listToNat self.blocks

abbrev IsClean (self : PlaintextInteger si) : Prop :=
  ∀ b ∈ self.blocks, b.hasMessageOnly

def getBlock (self : PlaintextInteger si) (ith : Fin si.blockCount) : PlaintextBlock sb :=
  self.blocks[ith.val]'(by rw [self.length_eq]; exact ith.isLt)

def setBlock (self : PlaintextInteger si) (ith : Fin si.blockCount) (block : PlaintextBlock sb) :
    PlaintextInteger si :=
  ⟨self.blocks.set ith.val block, by rw [List.length_set]; exact self.length_eq⟩

theorem toNat_lt (x : PlaintextInteger si) :
  x.toNat < 2 ^ si.intSize
:= by
  show PlaintextBlock.listToNat x.blocks < 2 ^ (si.blockCount * sb.messageSize)
  rw [← x.length_eq]
  exact PlaintextBlock.listToNat_lt x.blocks

@[simp] theorem toNat_ofNat (v : Nat) :
  (ofNat v : PlaintextInteger si).toNat = v % 2 ^ si.intSize
:=
  PlaintextBlock.listToNat_digits _ _

theorem isClean_ofNat (v : Nat) :
  (ofNat v : PlaintextInteger si).IsClean
:=
  PlaintextBlock.hasMessageOnly_of_mem_digits _ _

theorem toCompleteNat_getBlock_of_clean (x : PlaintextInteger si) (hx : x.IsClean)
    (i : Fin si.blockCount) :
  (x.getBlock i).toCompleteNat = x.toNat / 2 ^ (i.val * sb.messageSize) % 2 ^ sb.messageSize
:=
  PlaintextBlock.toCompleteNat_getElem_of_clean x.blocks hx i.val _

theorem toNat_setBlock_of_lt (x : PlaintextInteger si) (i : Fin si.blockCount)
    (b : PlaintextBlock sb) (h : x.toNat < 2 ^ (i.val * sb.messageSize)) :
  (x.setBlock i b).toNat = x.toNat + b.toMessageNat * 2 ^ (i.val * sb.messageSize)
:=
  PlaintextBlock.listToNat_set_of_lt x.blocks i.val b (by rw [x.length_eq]; exact i.isLt) h

theorem isClean_setBlock (x : PlaintextInteger si) (i : Fin si.blockCount)
    (b : PlaintextBlock sb) (hx : x.IsClean) (hb : b.hasMessageOnly) :
  (x.setBlock i b).IsClean
:=
  Util.List.forall_mem_set hx hb

end PlaintextInteger
end Zhc.Model
