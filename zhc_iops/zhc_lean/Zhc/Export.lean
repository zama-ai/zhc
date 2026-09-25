import Zhc.Iops.AddRippleCarry.Algo
import Zhc.Iops.AddHillisSteele.Algo

namespace Zhc.Export

open Model (SpecInteger)

private def emitAdd (name : String) (algo : Algo Iops.sb22) (h : algo.Param = SpecInteger Iops.sb22)
    (intSize : UInt16) : IO Ffi.Builder :=
  match SpecInteger.ofIntSize? (sb := Iops.sb22) intSize.toNat with
  | some si => algo.emit (h ▸ si)
  | none => throw (IO.userError s!"{name}: int size {intSize} is not a multiple of 2")

@[export zhc_lean_export_add_ripple_carry]
def addRippleCarry : UInt16 → IO Ffi.Builder :=
  emitAdd "add_ripple_carry" Iops.rippleAdd rfl

@[export zhc_lean_export_add_hillis_steele]
def addHillisSteele : UInt16 → IO Ffi.Builder :=
  emitAdd "add_hillis_steele" Iops.hsAdd rfl

end Zhc.Export
