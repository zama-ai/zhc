// Lean compiler output
// Module: Zhc.Ffi.L2r.Builder
// Imports: public import Init public meta import Init public import Zhc.Ffi.L2r.SpecBlock public import Zhc.Ffi.L2r.CiphertextBlock public import Zhc.Ffi.L2r.PlaintextBlock public import Zhc.Ffi.L2r.CiphertextBool public import Zhc.Ffi.L2r.CiphertextInteger public import Zhc.Ffi.L2r.PlaintextInteger public import Zhc.Ffi.L2r.Lut.Lut1 public import Zhc.Ffi.L2r.Lut.Lut2 public import Zhc.Ffi.L2r.Lut.Lut4 public import Zhc.Ffi.L2r.Lut.Lut8
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_BuilderPointed;
lean_object* zhc_lean_builder_new(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_new___boxed(lean_object*, lean_object*);
lean_object* zhc_lean_builder_spec(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_spec___boxed(lean_object*, lean_object*);
lean_object* zhc_lean_builder_dump(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_dump___boxed(lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_input(lean_object*, uint16_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerInput___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_declare(lean_object*, uint16_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerDeclare___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_store_block(lean_object*, lean_object*, uint8_t, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerStoreBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_get_block(lean_object*, lean_object*, uint8_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerGetBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_output(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerOutput___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_plaintext_integer_input(lean_object*, uint16_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_plaintextIntegerInput___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_plaintext_integer_get_block(lean_object*, lean_object*, uint8_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_plaintextIntegerGetBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_bool_input(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolInput___boxed(lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_bool_store_block(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolStoreBlock___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_bool_get_block(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolGetBlock___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_bool_output(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolOutput___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_interpret(lean_object*, lean_object*, size_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_interpret___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_let_plaintext(lean_object*, uint8_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLetPlaintext___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_let_ciphertext(lean_object*, uint8_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLetCiphertext___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_add(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockAdd___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_sub(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockSub___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_shl(lean_object*, lean_object*, uint8_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockShl___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_mac(lean_object*, lean_object*, lean_object*, uint8_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockMac___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_add_plaintext(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockAddPlaintext___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_sub_plaintext(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockSubPlaintext___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_plaintext_sub(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockPlaintextSub___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_mul_plaintext(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockMulPlaintext___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup2(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup2___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup4(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup4___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup8(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup8___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static lean_object* _init_lp_zhc__lean_Zhc_Ffi_L2r_BuilderPointed(void){
_start:
{
lean_object* v___x_1_; 
v___x_1_ = lean_box(0);
return v___x_1_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_new___boxed(lean_object* v_spec_4_, lean_object* v_a_00___x40___internal___hyg_5_){
_start:
{
lean_object* v_res_6_; 
v_res_6_ = zhc_lean_builder_new(v_spec_4_);
lean_dec(v_spec_4_);
return v_res_6_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_spec___boxed(lean_object* v_self_9_, lean_object* v_a_00___x40___internal___hyg_10_){
_start:
{
lean_object* v_res_11_; 
v_res_11_ = zhc_lean_builder_spec(v_self_9_);
lean_dec(v_self_9_);
return v_res_11_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_dump___boxed(lean_object* v_self_14_, lean_object* v_a_00___x40___internal___hyg_15_){
_start:
{
lean_object* v_res_16_; 
v_res_16_ = zhc_lean_builder_dump(v_self_14_);
lean_dec(v_self_14_);
return v_res_16_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerInput___boxed(lean_object* v_self_20_, lean_object* v_intSize_21_, lean_object* v_a_00___x40___internal___hyg_22_){
_start:
{
uint16_t v_intSize_boxed_23_; lean_object* v_res_24_; 
v_intSize_boxed_23_ = lean_unbox(v_intSize_21_);
v_res_24_ = zhc_lean_builder_ciphertext_integer_input(v_self_20_, v_intSize_boxed_23_);
lean_dec(v_self_20_);
return v_res_24_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerDeclare___boxed(lean_object* v_self_28_, lean_object* v_intSize_29_, lean_object* v_a_00___x40___internal___hyg_30_){
_start:
{
uint16_t v_intSize_boxed_31_; lean_object* v_res_32_; 
v_intSize_boxed_31_ = lean_unbox(v_intSize_29_);
v_res_32_ = zhc_lean_builder_ciphertext_integer_declare(v_self_28_, v_intSize_boxed_31_);
lean_dec(v_self_28_);
return v_res_32_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerStoreBlock___boxed(lean_object* v_self_38_, lean_object* v_ct_39_, lean_object* v_index_40_, lean_object* v_block_41_, lean_object* v_a_00___x40___internal___hyg_42_){
_start:
{
uint8_t v_index_boxed_43_; lean_object* v_res_44_; 
v_index_boxed_43_ = lean_unbox(v_index_40_);
v_res_44_ = zhc_lean_builder_ciphertext_integer_store_block(v_self_38_, v_ct_39_, v_index_boxed_43_, v_block_41_);
lean_dec(v_block_41_);
lean_dec(v_ct_39_);
lean_dec(v_self_38_);
return v_res_44_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerGetBlock___boxed(lean_object* v_self_49_, lean_object* v_ct_50_, lean_object* v_index_51_, lean_object* v_a_00___x40___internal___hyg_52_){
_start:
{
uint8_t v_index_boxed_53_; lean_object* v_res_54_; 
v_index_boxed_53_ = lean_unbox(v_index_51_);
v_res_54_ = zhc_lean_builder_ciphertext_integer_get_block(v_self_49_, v_ct_50_, v_index_boxed_53_);
lean_dec(v_ct_50_);
lean_dec(v_self_49_);
return v_res_54_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextIntegerOutput___boxed(lean_object* v_self_58_, lean_object* v_ct_59_, lean_object* v_a_00___x40___internal___hyg_60_){
_start:
{
lean_object* v_res_61_; 
v_res_61_ = zhc_lean_builder_ciphertext_integer_output(v_self_58_, v_ct_59_);
lean_dec(v_ct_59_);
lean_dec(v_self_58_);
return v_res_61_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_plaintextIntegerInput___boxed(lean_object* v_self_65_, lean_object* v_intSize_66_, lean_object* v_a_00___x40___internal___hyg_67_){
_start:
{
uint16_t v_intSize_boxed_68_; lean_object* v_res_69_; 
v_intSize_boxed_68_ = lean_unbox(v_intSize_66_);
v_res_69_ = zhc_lean_builder_plaintext_integer_input(v_self_65_, v_intSize_boxed_68_);
lean_dec(v_self_65_);
return v_res_69_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_plaintextIntegerGetBlock___boxed(lean_object* v_self_74_, lean_object* v_pt_75_, lean_object* v_index_76_, lean_object* v_a_00___x40___internal___hyg_77_){
_start:
{
uint8_t v_index_boxed_78_; lean_object* v_res_79_; 
v_index_boxed_78_ = lean_unbox(v_index_76_);
v_res_79_ = zhc_lean_builder_plaintext_integer_get_block(v_self_74_, v_pt_75_, v_index_boxed_78_);
lean_dec(v_pt_75_);
lean_dec(v_self_74_);
return v_res_79_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolInput___boxed(lean_object* v_self_82_, lean_object* v_a_00___x40___internal___hyg_83_){
_start:
{
lean_object* v_res_84_; 
v_res_84_ = zhc_lean_builder_ciphertext_bool_input(v_self_82_);
lean_dec(v_self_82_);
return v_res_84_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolStoreBlock___boxed(lean_object* v_self_88_, lean_object* v_block_89_, lean_object* v_a_00___x40___internal___hyg_90_){
_start:
{
lean_object* v_res_91_; 
v_res_91_ = zhc_lean_builder_ciphertext_bool_store_block(v_self_88_, v_block_89_);
lean_dec(v_block_89_);
lean_dec(v_self_88_);
return v_res_91_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolGetBlock___boxed(lean_object* v_self_95_, lean_object* v_value_96_, lean_object* v_a_00___x40___internal___hyg_97_){
_start:
{
lean_object* v_res_98_; 
v_res_98_ = zhc_lean_builder_ciphertext_bool_get_block(v_self_95_, v_value_96_);
lean_dec(v_value_96_);
lean_dec(v_self_95_);
return v_res_98_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_ciphertextBoolOutput___boxed(lean_object* v_self_102_, lean_object* v_value_103_, lean_object* v_a_00___x40___internal___hyg_104_){
_start:
{
lean_object* v_res_105_; 
v_res_105_ = zhc_lean_builder_ciphertext_bool_output(v_self_102_, v_value_103_);
lean_dec(v_value_103_);
lean_dec(v_self_102_);
return v_res_105_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_interpret___boxed(lean_object* v_self_110_, lean_object* v_inputs_111_, lean_object* v_outputsLen_112_, lean_object* v_a_00___x40___internal___hyg_113_){
_start:
{
size_t v_outputsLen_boxed_114_; lean_object* v_res_115_; 
v_outputsLen_boxed_114_ = lean_unbox_usize(v_outputsLen_112_);
lean_dec(v_outputsLen_112_);
v_res_115_ = zhc_lean_builder_interpret(v_self_110_, v_inputs_111_, v_outputsLen_boxed_114_);
lean_dec_ref(v_inputs_111_);
lean_dec(v_self_110_);
return v_res_115_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLetPlaintext___boxed(lean_object* v_self_119_, lean_object* v_value_120_, lean_object* v_a_00___x40___internal___hyg_121_){
_start:
{
uint8_t v_value_boxed_122_; lean_object* v_res_123_; 
v_value_boxed_122_ = lean_unbox(v_value_120_);
v_res_123_ = zhc_lean_builder_block_let_plaintext(v_self_119_, v_value_boxed_122_);
lean_dec(v_self_119_);
return v_res_123_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLetCiphertext___boxed(lean_object* v_self_127_, lean_object* v_value_128_, lean_object* v_a_00___x40___internal___hyg_129_){
_start:
{
uint8_t v_value_boxed_130_; lean_object* v_res_131_; 
v_value_boxed_130_ = lean_unbox(v_value_128_);
v_res_131_ = zhc_lean_builder_block_let_ciphertext(v_self_127_, v_value_boxed_130_);
lean_dec(v_self_127_);
return v_res_131_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockAdd___boxed(lean_object* v_self_136_, lean_object* v_a_137_, lean_object* v_b_138_, lean_object* v_a_00___x40___internal___hyg_139_){
_start:
{
lean_object* v_res_140_; 
v_res_140_ = zhc_lean_builder_block_add(v_self_136_, v_a_137_, v_b_138_);
lean_dec(v_b_138_);
lean_dec(v_a_137_);
lean_dec(v_self_136_);
return v_res_140_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockSub___boxed(lean_object* v_self_145_, lean_object* v_a_146_, lean_object* v_b_147_, lean_object* v_a_00___x40___internal___hyg_148_){
_start:
{
lean_object* v_res_149_; 
v_res_149_ = zhc_lean_builder_block_sub(v_self_145_, v_a_146_, v_b_147_);
lean_dec(v_b_147_);
lean_dec(v_a_146_);
lean_dec(v_self_145_);
return v_res_149_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockShl___boxed(lean_object* v_self_154_, lean_object* v_src_155_, lean_object* v_amount_156_, lean_object* v_a_00___x40___internal___hyg_157_){
_start:
{
uint8_t v_amount_boxed_158_; lean_object* v_res_159_; 
v_amount_boxed_158_ = lean_unbox(v_amount_156_);
v_res_159_ = zhc_lean_builder_block_shl(v_self_154_, v_src_155_, v_amount_boxed_158_);
lean_dec(v_src_155_);
lean_dec(v_self_154_);
return v_res_159_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockMac___boxed(lean_object* v_self_165_, lean_object* v_a_166_, lean_object* v_b_167_, lean_object* v_mul_168_, lean_object* v_a_00___x40___internal___hyg_169_){
_start:
{
uint8_t v_mul_boxed_170_; lean_object* v_res_171_; 
v_mul_boxed_170_ = lean_unbox(v_mul_168_);
v_res_171_ = zhc_lean_builder_block_mac(v_self_165_, v_a_166_, v_b_167_, v_mul_boxed_170_);
lean_dec(v_b_167_);
lean_dec(v_a_166_);
lean_dec(v_self_165_);
return v_res_171_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockAddPlaintext___boxed(lean_object* v_self_176_, lean_object* v_a_177_, lean_object* v_b_178_, lean_object* v_a_00___x40___internal___hyg_179_){
_start:
{
lean_object* v_res_180_; 
v_res_180_ = zhc_lean_builder_block_add_plaintext(v_self_176_, v_a_177_, v_b_178_);
lean_dec(v_b_178_);
lean_dec(v_a_177_);
lean_dec(v_self_176_);
return v_res_180_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockSubPlaintext___boxed(lean_object* v_self_185_, lean_object* v_a_186_, lean_object* v_b_187_, lean_object* v_a_00___x40___internal___hyg_188_){
_start:
{
lean_object* v_res_189_; 
v_res_189_ = zhc_lean_builder_block_sub_plaintext(v_self_185_, v_a_186_, v_b_187_);
lean_dec(v_b_187_);
lean_dec(v_a_186_);
lean_dec(v_self_185_);
return v_res_189_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockPlaintextSub___boxed(lean_object* v_self_194_, lean_object* v_a_195_, lean_object* v_b_196_, lean_object* v_a_00___x40___internal___hyg_197_){
_start:
{
lean_object* v_res_198_; 
v_res_198_ = zhc_lean_builder_block_plaintext_sub(v_self_194_, v_a_195_, v_b_196_);
lean_dec(v_b_196_);
lean_dec(v_a_195_);
lean_dec(v_self_194_);
return v_res_198_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockMulPlaintext___boxed(lean_object* v_self_203_, lean_object* v_a_204_, lean_object* v_b_205_, lean_object* v_a_00___x40___internal___hyg_206_){
_start:
{
lean_object* v_res_207_; 
v_res_207_ = zhc_lean_builder_block_mul_plaintext(v_self_203_, v_a_204_, v_b_205_);
lean_dec(v_b_205_);
lean_dec(v_a_204_);
lean_dec(v_self_203_);
return v_res_207_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup___boxed(lean_object* v_self_212_, lean_object* v_src_213_, lean_object* v_lut_214_, lean_object* v_a_00___x40___internal___hyg_215_){
_start:
{
lean_object* v_res_216_; 
v_res_216_ = zhc_lean_builder_block_lookup(v_self_212_, v_src_213_, v_lut_214_);
lean_dec(v_lut_214_);
lean_dec(v_src_213_);
lean_dec(v_self_212_);
return v_res_216_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup2___boxed(lean_object* v_self_221_, lean_object* v_src_222_, lean_object* v_lut_223_, lean_object* v_a_00___x40___internal___hyg_224_){
_start:
{
lean_object* v_res_225_; 
v_res_225_ = zhc_lean_builder_block_lookup2(v_self_221_, v_src_222_, v_lut_223_);
lean_dec(v_lut_223_);
lean_dec(v_src_222_);
lean_dec(v_self_221_);
return v_res_225_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup4___boxed(lean_object* v_self_230_, lean_object* v_src_231_, lean_object* v_lut_232_, lean_object* v_a_00___x40___internal___hyg_233_){
_start:
{
lean_object* v_res_234_; 
v_res_234_ = zhc_lean_builder_block_lookup4(v_self_230_, v_src_231_, v_lut_232_);
lean_dec(v_lut_232_);
lean_dec(v_src_231_);
lean_dec(v_self_230_);
return v_res_234_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Builder_blockLookup8___boxed(lean_object* v_self_239_, lean_object* v_src_240_, lean_object* v_lut_241_, lean_object* v_a_00___x40___internal___hyg_242_){
_start:
{
lean_object* v_res_243_; 
v_res_243_ = zhc_lean_builder_block_lookup8(v_self_239_, v_src_240_, v_lut_241_);
lean_dec(v_lut_241_);
lean_dec(v_src_240_);
lean_dec(v_self_239_);
return v_res_243_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_CiphertextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_PlaintextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_CiphertextBool(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_CiphertextInteger(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_PlaintextInteger(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut1(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut2(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut4(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut8(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Builder(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
lean_initialize_runtime_module();
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_SpecBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_CiphertextBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_PlaintextBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_CiphertextBool(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_CiphertextInteger(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_PlaintextInteger(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut1(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut2(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut4(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut8(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_zhc__lean_Zhc_Ffi_L2r_BuilderPointed = _init_lp_zhc__lean_Zhc_Ffi_L2r_BuilderPointed();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
