import Zhc.Ffi.L2r.SpecBlock
import Zhc.Ffi.L2r.Lut

namespace Zhc.Ffi.L2r

opaque Lut1Pointed : NonemptyType
def Lut1 : Type := Lut1Pointed.type
instance : Nonempty L2r.Lut1 := Lut1Pointed.property

@[extern "zhc_lean_lut1_new"]
opaque Lut1.new (name : @& String) (spec : @& L2r.SpecBlock) (table1 : @& Array UInt16) :
    IO L2r.Lut1

def Lut1.ofModel {sb : Model.SpecBlock} (lut : Model.Lut1 sb) : IO L2r.Lut1 := do
  L2r.Lut1.new lut.name (← L2r.SpecBlock.ofModel sb) (lut.table 0)

end Zhc.Ffi.L2r
