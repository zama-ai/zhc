import Zhc.Ffi.L2r.SpecBlock
import Zhc.Ffi.L2r.Lut

namespace Zhc.Ffi.L2r

opaque Lut2Pointed : NonemptyType
def Lut2 : Type := Lut2Pointed.type
instance : Nonempty L2r.Lut2 := Lut2Pointed.property

@[extern "zhc_lean_lut2_new"]
opaque Lut2.new (name : @& String) (spec : @& L2r.SpecBlock)
    (table1 table2 : @& Array UInt16) : IO L2r.Lut2

def Lut2.ofModel {sb : Model.SpecBlock} (lut : Model.Lut2 sb) : IO L2r.Lut2 := do
  L2r.Lut2.new lut.name (← L2r.SpecBlock.ofModel sb) (lut.table 0) (lut.table 1)

end Zhc.Ffi.L2r
