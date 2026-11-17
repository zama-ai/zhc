import Zhc.Ffi.L2r.SpecInteger

namespace Zhc.Ffi.L2r

opaque CiphertextIntegerPointed : NonemptyType
def CiphertextInteger : Type := CiphertextIntegerPointed.type
instance : Nonempty L2r.CiphertextInteger := CiphertextIntegerPointed.property

@[extern "zhc_lean_ciphertext_integer_spec"]
opaque CiphertextInteger.spec (self : @& L2r.CiphertextInteger) : IO L2r.SpecInteger

@[extern "zhc_lean_ciphertext_integer_unbox"]
opaque CiphertextInteger.unbox(self : @& L2r.CiphertextInteger) (out : USize) : IO Unit

end Zhc.Ffi.L2r
