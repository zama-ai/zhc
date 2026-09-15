import Zhc.Ffi.Types
import Zhc.Model.Lookup

namespace Zhc.Ffi

@[extern "zhc_lean_lut1_new"]
opaque Lut1.new (name : @& String) (spec : @& BlockSpec) (table1 : @& Array UInt16) : IO Lut1

@[extern "zhc_lean_lut2_new"]
opaque Lut2.new (name : @& String) (spec : @& BlockSpec)
    (table1 table2 : @& Array UInt16) : IO Lut2

@[extern "zhc_lean_lut4_new"]
opaque Lut4.new (name : @& String) (spec : @& BlockSpec)
    (table1 table2 table3 table4 : @& Array UInt16) : IO Lut4

@[extern "zhc_lean_lut8_new"]
opaque Lut8.new (name : @& String) (spec : @& BlockSpec)
    (table1 table2 table3 table4 table5 table6 table7 table8 : @& Array UInt16) : IO Lut8

end Zhc.Ffi

namespace Zhc.Model.ManyLut

def table {sb : SpecBlock} {n : Nat} (lut : ManyLut sb n) (k : Nat) : Array UInt16 :=
  (Array.range (subTableSize sb n)).map fun r => (lut.entry k r).toCompleteNat.toUInt16

end Zhc.Model.ManyLut

namespace Zhc.Ffi

open Model (SpecBlock)

def BlockSpec.ofModel (sb : SpecBlock) : BlockSpec :=
  ⟨sb.carrySize.toUInt8, sb.messageSize.toUInt8⟩

def Lut1.ofModel {sb : SpecBlock} (lut : Model.Lut1 sb) : IO Lut1 :=
  Lut1.new lut.name (BlockSpec.ofModel sb) (lut.table 0)

def Lut2.ofModel {sb : SpecBlock} (lut : Model.Lut2 sb) : IO Lut2 :=
  Lut2.new lut.name (BlockSpec.ofModel sb) (lut.table 0) (lut.table 1)

def Lut4.ofModel {sb : SpecBlock} (lut : Model.Lut4 sb) : IO Lut4 :=
  Lut4.new lut.name (BlockSpec.ofModel sb) (lut.table 0) (lut.table 1) (lut.table 2) (lut.table 3)

def Lut8.ofModel {sb : SpecBlock} (lut : Model.Lut8 sb) : IO Lut8 :=
  Lut8.new lut.name (BlockSpec.ofModel sb) (lut.table 0) (lut.table 1) (lut.table 2) (lut.table 3)
    (lut.table 4) (lut.table 5) (lut.table 6) (lut.table 7)

end Zhc.Ffi
