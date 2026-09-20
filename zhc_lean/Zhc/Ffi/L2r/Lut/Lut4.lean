import Zhc.Ffi.L2r.SpecBlock
import Zhc.Ffi.L2r.Lut

namespace Zhc.Ffi.L2r

opaque Lut4Pointed : NonemptyType
def Lut4 : Type := Lut4Pointed.type
instance : Nonempty L2r.Lut4 := Lut4Pointed.property

@[extern "zhc_lean_lut4_new"]
opaque Lut4.new (name : @& String) (spec : @& L2r.SpecBlock)
    (table1 table2 table3 table4 : @& Array UInt16) : IO L2r.Lut4

def Lut4.ofModel {sb : Model.SpecBlock} (lut : Model.Lut4 sb) : IO L2r.Lut4 := do
  L2r.Lut4.new lut.name (← L2r.SpecBlock.ofModel sb) (lut.table 0) (lut.table 1) (lut.table 2)
    (lut.table 3)

end Zhc.Ffi.L2r
