namespace Zhc.Ffi.L2r

opaque CiphertextBlockPointed : NonemptyType
def CiphertextBlock : Type := CiphertextBlockPointed.type
instance : Nonempty L2r.CiphertextBlock := CiphertextBlockPointed.property

end Zhc.Ffi.L2r
