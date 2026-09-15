import Zhc.Model.SpecBlock

namespace Zhc.Model

structure SpecInteger (sb: SpecBlock) where
  blockCount : Nat
  deriving DecidableEq, Repr, Inhabited

namespace SpecInteger variable {sb: SpecBlock}

def intSize(self: SpecInteger sb) : Nat :=
  self.blockCount * sb.messageSize

def ofIntSize? (intSize : Nat) : Option (SpecInteger sb) :=
  if intSize % sb.messageSize = 0 then some ⟨intSize / sb.messageSize⟩ else none

def ofIntSize! (intSize : Nat) : SpecInteger sb :=
  match ofIntSize? intSize with
  | some si => si
  | none => panic! "ofIntSize!: intSize not divisible by messageSize"

structure Valid(self: SpecInteger sb ) : Prop where
  blockCount_pos : 0 < self.blockCount
  block_valid : sb.Valid

end SpecInteger

end Zhc.Model
