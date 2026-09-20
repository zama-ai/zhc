import Zhc.Ffi.L2r.SpecBlock

namespace Zhc.Ffi.L2r

opaque SpecIntegerPointed : NonemptyType
def SpecInteger : Type := SpecIntegerPointed.type
instance : Nonempty L2r.SpecInteger := SpecIntegerPointed.property

@[extern "zhc_lean_spec_integer_int_size"]
opaque SpecInteger.intSize (self : @& L2r.SpecInteger) : IO UInt16

@[extern "zhc_lean_spec_integer_block"]
opaque SpecInteger.block (self : @& L2r.SpecInteger) : IO L2r.SpecBlock

end Zhc.Ffi.L2r
