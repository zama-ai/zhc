import Zhc.Model.SpecBlock

namespace Zhc.Ffi.L2r

opaque SpecBlockPointed : NonemptyType
def SpecBlock : Type := SpecBlockPointed.type
instance : Nonempty L2r.SpecBlock := SpecBlockPointed.property

@[extern "zhc_lean_spec_block_new"]
opaque SpecBlock.new (carry message : UInt8) : IO L2r.SpecBlock

@[extern "zhc_lean_spec_block_carry"]
opaque SpecBlock.carry (self : @& L2r.SpecBlock) : IO UInt8

@[extern "zhc_lean_spec_block_message"]
opaque SpecBlock.message (self : @& L2r.SpecBlock) : IO UInt8

def SpecBlock.ofModel (sb : Model.SpecBlock) : IO L2r.SpecBlock :=
  L2r.SpecBlock.new sb.carrySize.toUInt8 sb.messageSize.toUInt8

def SpecBlock.same (a b : L2r.SpecBlock) : IO Bool := do
  return (← a.carry) == (← b.carry) && (← a.message) == (← b.message)

end Zhc.Ffi.L2r
