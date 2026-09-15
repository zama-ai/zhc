import Zhc.Util

namespace Zhc.Model

structure SpecBlock where
  carrySize : Nat
  messageSize : Nat
  deriving DecidableEq, Repr

namespace SpecBlock

def paddingSize(_self : SpecBlock) : Nat :=
  1

def dataSize(self: SpecBlock): Nat :=
  self.carrySize + self.messageSize

def completeSize(self: SpecBlock ) : Nat :=
  self.dataSize + self.paddingSize

structure Valid(self: SpecBlock) : Prop where
  carry_pos : 0 < self.carrySize
  message_pos : 0 < self.messageSize

theorem messageSize_le_dataSize (self : SpecBlock) :
  self.messageSize ≤ self.dataSize
:=
  Nat.le_add_left _ _

theorem carrySize_le_dataSize (self : SpecBlock) :
  self.carrySize ≤ self.dataSize
:=
  Nat.le_add_right _ _

theorem dataSize_le_completeSize (self : SpecBlock) :
  self.dataSize ≤ self.completeSize
:=
  Nat.le_add_right _ _

theorem messageSize_le_completeSize(self: SpecBlock) :
  self.messageSize ≤ self.completeSize
:= by
  unfold completeSize dataSize;
  omega

theorem carrySize_le_completeSize (self : SpecBlock) :
  self.carrySize ≤ self.completeSize
:=
  Nat.le_trans self.carrySize_le_dataSize self.dataSize_le_completeSize

theorem one_le_completeSize(self: SpecBlock) :
  1 ≤ self.completeSize
:= by
  unfold completeSize paddingSize;
  omega

end SpecBlock

end Zhc.Model
