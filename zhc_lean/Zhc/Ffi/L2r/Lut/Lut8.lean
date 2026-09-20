import Zhc.Ffi.L2r.SpecBlock
import Zhc.Ffi.L2r.Lut

namespace Zhc.Ffi.L2r

opaque Lut8Pointed : NonemptyType
def Lut8 : Type := Lut8Pointed.type
instance : Nonempty L2r.Lut8 := Lut8Pointed.property

@[extern "zhc_lean_lut8_new"]
opaque Lut8.new (name : @& String) (spec : @& L2r.SpecBlock)
    (table1 table2 table3 table4 table5 table6 table7 table8 : @& Array UInt16) : IO L2r.Lut8

def Lut8.ofModel {sb : Model.SpecBlock} (lut : Model.Lut8 sb) : IO L2r.Lut8 := do
  L2r.Lut8.new lut.name (← L2r.SpecBlock.ofModel sb) (lut.table 0) (lut.table 1) (lut.table 2)
    (lut.table 3) (lut.table 4) (lut.table 5) (lut.table 6) (lut.table 7)

end Zhc.Ffi.L2r
