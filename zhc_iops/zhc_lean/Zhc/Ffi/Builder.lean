import Zhc.Ffi.Types

namespace Zhc.Ffi.Builder

@[extern "zhc_lean_builder_new"]
opaque new (spec : @& BlockSpec) : IO Builder

@[extern "zhc_lean_builder_spec"]
opaque spec (self : @& Builder) : IO BlockSpec

@[extern "zhc_lean_builder_draw"]
opaque draw (self : @& Builder) (kind : IrKind) : IO FileHandle

@[extern "zhc_lean_builder_dump_noise"]
opaque dumpNoise (self : @& Builder) : IO String

@[extern "zhc_lean_builder_check_noise"]
opaque checkNoise (self : @& Builder) : IO Unit

@[extern "zhc_lean_builder_push_comment"]
opaque pushComment (self : @& Builder) (comment : @& String) : IO Unit

@[extern "zhc_lean_builder_pop_comment"]
opaque popComment (self : @& Builder) : IO Unit

@[extern "zhc_lean_builder_integer_ciphertext_input"]
opaque integerCiphertextInput (self : @& Builder) (intSize : UInt16) : IO IntegerCiphertext

@[extern "zhc_lean_builder_integer_ciphertext_declare"]
opaque integerCiphertextDeclare (self : @& Builder) (intSize : UInt16) : IO IntegerCiphertext

@[extern "zhc_lean_builder_integer_ciphertext_store_block"]
opaque integerCiphertextStoreBlock (self : @& Builder) (ct : @& IntegerCiphertext)
    (index : UInt8) (block : @& CiphertextBlock) : IO IntegerCiphertext

@[extern "zhc_lean_builder_integer_ciphertext_get_block"]
opaque integerCiphertextGetBlock (self : @& Builder) (ct : @& IntegerCiphertext)
    (index : UInt8) : IO CiphertextBlock

@[extern "zhc_lean_builder_integer_ciphertext_inspect"]
opaque integerCiphertextInspect (self : @& Builder) (src : @& IntegerCiphertext) :
    IO IntegerCiphertext

@[extern "zhc_lean_builder_integer_ciphertext_output"]
opaque integerCiphertextOutput (self : @& Builder) (ct : @& IntegerCiphertext) : IO Unit

@[extern "zhc_lean_builder_integer_plaintext_input"]
opaque integerPlaintextInput (self : @& Builder) (intSize : UInt16) : IO IntegerPlaintext

@[extern "zhc_lean_builder_integer_plaintext_get_block"]
opaque integerPlaintextGetBlock (self : @& Builder) (pt : @& IntegerPlaintext)
    (index : UInt8) : IO PlaintextBlock

@[extern "zhc_lean_builder_bool_ciphertext_input"]
opaque boolCiphertextInput (self : @& Builder) : IO BoolCiphertext

@[extern "zhc_lean_builder_bool_ciphertext_from_block"]
opaque boolCiphertextFromBlock (self : @& Builder) (block : @& CiphertextBlock) :
    IO BoolCiphertext

@[extern "zhc_lean_builder_bool_ciphertext_get_block"]
opaque boolCiphertextGetBlock (self : @& Builder) (value : @& BoolCiphertext) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_bool_ciphertext_output"]
opaque boolCiphertextOutput (self : @& Builder) (value : @& BoolCiphertext) : IO Unit

/-! ## Block constants and inspection -/

@[extern "zhc_lean_builder_block_let_plaintext"]
opaque blockLetPlaintext (self : @& Builder) (value : UInt8) : IO PlaintextBlock

@[extern "zhc_lean_builder_block_let_ciphertext"]
opaque blockLetCiphertext (self : @& Builder) (value : UInt8) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_inspect"]
opaque blockInspect (self : @& Builder) (src : @& CiphertextBlock) : IO CiphertextBlock

/-! ## Block add -/

@[extern "zhc_lean_builder_block_add_with"]
opaque blockAddWith (self : @& Builder) (a b : @& CiphertextBlock) (flavor : Flavor) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_add"]
opaque blockAdd (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_add"]
opaque blockTemperAdd (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_add"]
opaque blockWrappingAdd (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

/-! ## Block sub -/

@[extern "zhc_lean_builder_block_sub_with"]
opaque blockSubWith (self : @& Builder) (a b : @& CiphertextBlock) (flavor : Flavor) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_sub"]
opaque blockSub (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_sub"]
opaque blockTemperSub (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_sub"]
opaque blockWrappingSub (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

/-! ## Block shl -/

@[extern "zhc_lean_builder_block_shl_with"]
opaque blockShlWith (self : @& Builder) (src : @& CiphertextBlock) (amount : UInt8)
    (flavor : Flavor) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_shl"]
opaque blockShl (self : @& Builder) (src : @& CiphertextBlock) (amount : UInt8) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_shl"]
opaque blockTemperShl (self : @& Builder) (src : @& CiphertextBlock) (amount : UInt8) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_shl"]
opaque blockWrappingShl (self : @& Builder) (src : @& CiphertextBlock) (amount : UInt8) :
    IO CiphertextBlock

/-! ## Block add plaintext -/

@[extern "zhc_lean_builder_block_add_plaintext_with"]
opaque blockAddPlaintextWith (self : @& Builder) (a : @& CiphertextBlock) (b : @& PlaintextBlock)
    (flavor : Flavor) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_add_plaintext"]
opaque blockAddPlaintext (self : @& Builder) (a : @& CiphertextBlock) (b : @& PlaintextBlock) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_add_plaintext"]
opaque blockTemperAddPlaintext (self : @& Builder) (a : @& CiphertextBlock)
    (b : @& PlaintextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_add_plaintext"]
opaque blockWrappingAddPlaintext (self : @& Builder) (a : @& CiphertextBlock)
    (b : @& PlaintextBlock) : IO CiphertextBlock

/-! ## Block sub plaintext -/

@[extern "zhc_lean_builder_block_sub_plaintext_with"]
opaque blockSubPlaintextWith (self : @& Builder) (a : @& CiphertextBlock) (b : @& PlaintextBlock)
    (flavor : Flavor) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_sub_plaintext"]
opaque blockSubPlaintext (self : @& Builder) (a : @& CiphertextBlock) (b : @& PlaintextBlock) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_sub_plaintext"]
opaque blockTemperSubPlaintext (self : @& Builder) (a : @& CiphertextBlock)
    (b : @& PlaintextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_sub_plaintext"]
opaque blockWrappingSubPlaintext (self : @& Builder) (a : @& CiphertextBlock)
    (b : @& PlaintextBlock) : IO CiphertextBlock

/-! ## Block plaintext sub -/

@[extern "zhc_lean_builder_block_plaintext_sub_with"]
opaque blockPlaintextSubWith (self : @& Builder) (a : @& PlaintextBlock) (b : @& CiphertextBlock)
    (flavor : Flavor) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_plaintext_sub"]
opaque blockPlaintextSub (self : @& Builder) (a : @& PlaintextBlock) (b : @& CiphertextBlock) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_plaintext_sub"]
opaque blockTemperPlaintextSub (self : @& Builder) (a : @& PlaintextBlock)
    (b : @& CiphertextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_plaintext_sub"]
opaque blockWrappingPlaintextSub (self : @& Builder) (a : @& PlaintextBlock)
    (b : @& CiphertextBlock) : IO CiphertextBlock

/-! ## Block mul plaintext -/

@[extern "zhc_lean_builder_block_mul_plaintext_with"]
opaque blockMulPlaintextWith (self : @& Builder) (a : @& CiphertextBlock) (b : @& PlaintextBlock)
    (flavor : Flavor) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_mul_plaintext"]
opaque blockMulPlaintext (self : @& Builder) (a : @& CiphertextBlock) (b : @& PlaintextBlock) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_mul_plaintext"]
opaque blockTemperMulPlaintext (self : @& Builder) (a : @& CiphertextBlock)
    (b : @& PlaintextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_mul_plaintext"]
opaque blockWrappingMulPlaintext (self : @& Builder) (a : @& CiphertextBlock)
    (b : @& PlaintextBlock) : IO CiphertextBlock

/-! ## Block mac -/

@[extern "zhc_lean_builder_block_mac_with"]
opaque blockMacWith (self : @& Builder) (a b : @& CiphertextBlock) (mul : UInt8)
    (flavor : Flavor) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_mac"]
opaque blockMac (self : @& Builder) (a b : @& CiphertextBlock) (mul : UInt8) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_mac"]
opaque blockTemperMac (self : @& Builder) (a b : @& CiphertextBlock) (mul : UInt8) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_mac"]
opaque blockWrappingMac (self : @& Builder) (a b : @& CiphertextBlock) (mul : UInt8) :
    IO CiphertextBlock

/-! ## Block pack -/

@[extern "zhc_lean_builder_block_pack_with"]
opaque blockPackWith (self : @& Builder) (a b : @& CiphertextBlock) (flavor : Flavor) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_pack"]
opaque blockPack (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_temper_pack"]
opaque blockTemperPack (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_pack"]
opaque blockWrappingPack (self : @& Builder) (a b : @& CiphertextBlock) : IO CiphertextBlock

/-! ## Block lookups

The multi-output lookups return their blocks as an array of 2, 4, or 8 elements. -/

@[extern "zhc_lean_builder_block_lookup_with"]
opaque blockLookupWith (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut1)
    (check : @& LookupCheck) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_lookup"]
opaque blockLookup (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut1) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_padding_lookup"]
opaque blockPaddingLookup (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut1) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_wrapping_lookup"]
opaque blockWrappingLookup (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut1) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_pack_then_lookup"]
opaque blockPackThenLookup (self : @& Builder) (a b : @& CiphertextBlock) (lut : @& Lut1) :
    IO CiphertextBlock

@[extern "zhc_lean_builder_block_mac_then_lookup"]
opaque blockMacThenLookup (self : @& Builder) (a b : @& CiphertextBlock) (mul : UInt8)
    (lut : @& Lut1) : IO CiphertextBlock

@[extern "zhc_lean_builder_block_lookup2_with"]
opaque blockLookup2With (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut2)
    (check : @& LookupCheck) : IO (Array CiphertextBlock)

@[extern "zhc_lean_builder_block_lookup2"]
opaque blockLookup2 (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut2) :
    IO (Array CiphertextBlock)

@[extern "zhc_lean_builder_block_lookup4_with"]
opaque blockLookup4With (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut4)
    (check : @& LookupCheck) : IO (Array CiphertextBlock)

@[extern "zhc_lean_builder_block_lookup4"]
opaque blockLookup4 (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut4) :
    IO (Array CiphertextBlock)

@[extern "zhc_lean_builder_block_lookup8_with"]
opaque blockLookup8With (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut8)
    (check : @& LookupCheck) : IO (Array CiphertextBlock)

@[extern "zhc_lean_builder_block_lookup8"]
opaque blockLookup8 (self : @& Builder) (src : @& CiphertextBlock) (lut : @& Lut8) :
    IO (Array CiphertextBlock)

end Zhc.Ffi.Builder
