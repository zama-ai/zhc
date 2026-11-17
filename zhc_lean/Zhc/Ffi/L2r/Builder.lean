import Zhc.Ffi.L2r.SpecBlock
import Zhc.Ffi.L2r.CiphertextBlock
import Zhc.Ffi.L2r.PlaintextBlock
import Zhc.Ffi.L2r.CiphertextBool
import Zhc.Ffi.L2r.CiphertextInteger
import Zhc.Ffi.L2r.PlaintextInteger
import Zhc.Ffi.L2r.Lut.Lut1
import Zhc.Ffi.L2r.Lut.Lut2
import Zhc.Ffi.L2r.Lut.Lut4
import Zhc.Ffi.L2r.Lut.Lut8

namespace Zhc.Ffi.L2r

opaque BuilderPointed : NonemptyType
def Builder : Type := BuilderPointed.type
instance : Nonempty L2r.Builder := BuilderPointed.property

end Zhc.Ffi.L2r

namespace Zhc.Ffi.L2r.Builder

@[extern "zhc_lean_builder_new"]
opaque new (spec : @& L2r.SpecBlock) : IO L2r.Builder

@[extern "zhc_lean_builder_spec"]
opaque spec (self : @& L2r.Builder) : IO L2r.SpecBlock

@[extern "zhc_lean_builder_dump"]
opaque dump (self : @& L2r.Builder) : IO String

@[extern "zhc_lean_builder_ciphertext_integer_input"]
opaque ciphertextIntegerInput (self : @& L2r.Builder) (intSize : UInt16) :
    IO L2r.CiphertextInteger

@[extern "zhc_lean_builder_ciphertext_integer_declare"]
opaque ciphertextIntegerDeclare (self : @& L2r.Builder) (intSize : UInt16) :
    IO L2r.CiphertextInteger

@[extern "zhc_lean_builder_ciphertext_integer_store_block"]
opaque ciphertextIntegerStoreBlock (self : @& L2r.Builder) (ct : @& L2r.CiphertextInteger)
    (index : UInt8) (block : @& L2r.CiphertextBlock) : IO L2r.CiphertextInteger

@[extern "zhc_lean_builder_ciphertext_integer_get_block"]
opaque ciphertextIntegerGetBlock (self : @& L2r.Builder) (ct : @& L2r.CiphertextInteger)
    (index : UInt8) : IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_ciphertext_integer_output"]
opaque ciphertextIntegerOutput (self : @& L2r.Builder) (ct : @& L2r.CiphertextInteger) : IO Unit

@[extern "zhc_lean_builder_plaintext_integer_input"]
opaque plaintextIntegerInput (self : @& L2r.Builder) (intSize : UInt16) :
    IO L2r.PlaintextInteger

@[extern "zhc_lean_builder_plaintext_integer_get_block"]
opaque plaintextIntegerGetBlock (self : @& L2r.Builder) (pt : @& L2r.PlaintextInteger)
    (index : UInt8) : IO L2r.PlaintextBlock

@[extern "zhc_lean_builder_ciphertext_bool_input"]
opaque ciphertextBoolInput (self : @& L2r.Builder) : IO L2r.CiphertextBool

@[extern "zhc_lean_builder_ciphertext_bool_store_block"]
opaque ciphertextBoolStoreBlock (self : @& L2r.Builder) (block : @& L2r.CiphertextBlock) :
    IO L2r.CiphertextBool

@[extern "zhc_lean_builder_ciphertext_bool_get_block"]
opaque ciphertextBoolGetBlock (self : @& L2r.Builder) (value : @& L2r.CiphertextBool) :
    IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_ciphertext_bool_output"]
opaque ciphertextBoolOutput (self : @& L2r.Builder) (value : @& L2r.CiphertextBool) : IO Unit

@[extern "zhc_lean_builder_interpret"]
opaque interpret (self : @& L2r.Builder) (inputs : @& Array UInt16) (outputsLen : USize) :
    IO (Array UInt16)

/-! ## Block constants -/

@[extern "zhc_lean_builder_block_let_plaintext"]
opaque blockLetPlaintext (self : @& L2r.Builder) (value : UInt8) : IO L2r.PlaintextBlock

@[extern "zhc_lean_builder_block_let_ciphertext"]
opaque blockLetCiphertext (self : @& L2r.Builder) (value : UInt8) : IO L2r.CiphertextBlock

/-! ## Block arithmetic, wrapping flavor -/

@[extern "zhc_lean_builder_block_add"]
opaque blockAdd (self : @& L2r.Builder) (a b : @& L2r.CiphertextBlock) : IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_sub"]
opaque blockSub (self : @& L2r.Builder) (a b : @& L2r.CiphertextBlock) : IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_shl"]
opaque blockShl (self : @& L2r.Builder) (src : @& L2r.CiphertextBlock) (amount : UInt8) :
    IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_mac"]
opaque blockMac (self : @& L2r.Builder) (a b : @& L2r.CiphertextBlock) (mul : UInt8) :
    IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_add_plaintext"]
opaque blockAddPlaintext (self : @& L2r.Builder) (a : @& L2r.CiphertextBlock)
    (b : @& L2r.PlaintextBlock) : IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_sub_plaintext"]
opaque blockSubPlaintext (self : @& L2r.Builder) (a : @& L2r.CiphertextBlock)
    (b : @& L2r.PlaintextBlock) : IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_plaintext_sub"]
opaque blockPlaintextSub (self : @& L2r.Builder) (a : @& L2r.PlaintextBlock)
    (b : @& L2r.CiphertextBlock) : IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_mul_plaintext"]
opaque blockMulPlaintext (self : @& L2r.Builder) (a : @& L2r.CiphertextBlock)
    (b : @& L2r.PlaintextBlock) : IO L2r.CiphertextBlock

/-! ## Block lookups, permissive check

The multi-output lookups return their blocks as an array of 2, 4, or 8 elements. -/

@[extern "zhc_lean_builder_block_lookup"]
opaque blockLookup (self : @& L2r.Builder) (src : @& L2r.CiphertextBlock) (lut : @& L2r.Lut1) :
    IO L2r.CiphertextBlock

@[extern "zhc_lean_builder_block_lookup2"]
opaque blockLookup2 (self : @& L2r.Builder) (src : @& L2r.CiphertextBlock) (lut : @& L2r.Lut2) :
    IO (Array L2r.CiphertextBlock)

@[extern "zhc_lean_builder_block_lookup4"]
opaque blockLookup4 (self : @& L2r.Builder) (src : @& L2r.CiphertextBlock) (lut : @& L2r.Lut4) :
    IO (Array L2r.CiphertextBlock)

@[extern "zhc_lean_builder_block_lookup8"]
opaque blockLookup8 (self : @& L2r.Builder) (src : @& L2r.CiphertextBlock) (lut : @& L2r.Lut8) :
    IO (Array L2r.CiphertextBlock)

end Zhc.Ffi.L2r.Builder
