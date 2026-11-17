import Zhc.Ffi.L2r.SpecBlock

namespace Zhc.Ffi.L2r

opaque CiphertextBoolPointed : NonemptyType
def CiphertextBool : Type := CiphertextBoolPointed.type
instance : Nonempty L2r.CiphertextBool := CiphertextBoolPointed.property

@[extern "zhc_lean_ciphertext_bool_spec"]
opaque CiphertextBool.spec (self : @& L2r.CiphertextBool) : IO L2r.SpecBlock

@[extern "zhc_lean_ciphertext_bool_unbox"]
opaque CiphertextBool.unbox(self : @& L2r.CiphertextBool) (out : USize) : IO Unit

end Zhc.Ffi.L2r
