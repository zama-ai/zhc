import Zhc.Ffi.L2r.SpecInteger

namespace Zhc.Ffi.L2r

opaque PlaintextIntegerPointed : NonemptyType
def PlaintextInteger : Type := PlaintextIntegerPointed.type
instance : Nonempty L2r.PlaintextInteger := PlaintextIntegerPointed.property

@[extern "zhc_lean_plaintext_integer_spec"]
opaque PlaintextInteger.spec (self : @& L2r.PlaintextInteger) : IO L2r.SpecInteger

end Zhc.Ffi.L2r
