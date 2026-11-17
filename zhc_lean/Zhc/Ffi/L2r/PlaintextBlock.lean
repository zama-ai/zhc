namespace Zhc.Ffi.L2r

opaque PlaintextBlockPointed : NonemptyType
def PlaintextBlock : Type := PlaintextBlockPointed.type
instance : Nonempty L2r.PlaintextBlock := PlaintextBlockPointed.property

end Zhc.Ffi.L2r
