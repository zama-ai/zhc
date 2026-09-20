import Zhc.Model.Lookup

namespace Zhc.Model.ManyLut

def table {sb : SpecBlock} {n : Nat} (lut : ManyLut sb n) (k : Nat) : Array UInt16 :=
  (Array.range (subTableSize sb n)).map fun r => (lut.entry k r).toCompleteNat.toUInt16

end Zhc.Model.ManyLut
