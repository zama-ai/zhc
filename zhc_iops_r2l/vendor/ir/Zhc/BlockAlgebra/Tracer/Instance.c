// Lean compiler output
// Module: Zhc.BlockAlgebra.Tracer.Instance
// Imports: public import Init public meta import Init public import Zhc.BlockAlgebra.Class public import Zhc.Model.CiphertextBlock public import Zhc.Model.CiphertextInteger public import Zhc.Model.CiphertextBool public import Zhc.Model.PlaintextBlock public import Zhc.Model.PlaintextInteger public import Zhc.Model.Lookup public import Zhc.Model.Operators public import Zhc.Ffi.L2r.Builder
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
lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_intSize(lean_object*, lean_object*);
uint8_t lean_nat_dec_lt(lean_object*, lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
lean_object* lean_string_append(lean_object*, lean_object*);
uint16_t lean_uint16_of_nat(lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_input(lean_object*, uint16_t);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_bool_get_block(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_add_plaintext(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_addPt(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_declare(lean_object*, uint16_t);
uint8_t lean_uint8_of_nat(lean_object*);
lean_object* zhc_lean_builder_block_let_plaintext(lean_object*, uint8_t);
lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_let_ciphertext(lean_object*, uint8_t);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_add(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_add(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_sub(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_sub(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_shl(lean_object*, lean_object*, uint8_t);
lean_object* lp_zhc__lean_Zhc_Model_Operators_shl(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_mac(lean_object*, lean_object*, lean_object*, uint8_t);
lean_object* lp_zhc__lean_Zhc_Model_Operators_mac(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_sub_plaintext(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_subPt(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_plaintext_sub(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_subCt(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_mul_plaintext(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_mulPt(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_get_block(lean_object*, lean_object*, uint8_t);
lean_object* l_List_get___redArg(lean_object*, lean_object*);
lean_object* zhc_lean_builder_plaintext_integer_get_block(lean_object*, lean_object*, uint8_t);
lean_object* zhc_lean_builder_ciphertext_integer_store_block(lean_object*, lean_object*, uint8_t, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_bool_store_block(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut1_ofModel(lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2_ofModel(lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup2(lean_object*, lean_object*, lean_object*);
lean_object* lean_array_get_size(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* lean_array_fget_borrowed(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4_ofModel(lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup4(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8_ofModel(lean_object*, lean_object*);
lean_object* zhc_lean_builder_block_lookup8(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(lean_object*);
lean_object* zhc_lean_builder_new(lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_output(lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 24, .m_capacity = 24, .m_length = 23, .m_data = " does not fit in 8 bits"};
static const lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8___boxed(lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 25, .m_capacity = 25, .m_length = 24, .m_data = " does not fit in 16 bits"};
static const lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subCt(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subCt___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_shlCt(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_shlCt___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subPt(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subPt___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_ptSub(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_ptSub___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_mulPt(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_mulPt___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___redArg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___redArg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 48, .m_capacity = 48, .m_length = 47, .m_data = "lookup2 returned an unexpected number of blocks"};
static const lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__0 = (const lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__0_value;
static const lean_ctor_object lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 18}, .m_objs = {((lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__0_value)}};
static const lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__1 = (const lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__1_value;
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___boxed(lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 48, .m_capacity = 48, .m_length = 47, .m_data = "lookup4 returned an unexpected number of blocks"};
static const lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__0 = (const lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__0_value;
static const lean_ctor_object lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 18}, .m_objs = {((lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__0_value)}};
static const lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__1 = (const lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__1_value;
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___boxed(lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 48, .m_capacity = 48, .m_length = 47, .m_data = "lookup8 returned an unexpected number of blocks"};
static const lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__0 = (const lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__0_value;
static const lean_ctor_object lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 18}, .m_objs = {((lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__0_value)}};
static const lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__1 = (const lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__1_value;
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___boxed(lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___closed__0;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__1;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__1;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__2;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__3;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__1;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__2;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__3;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__4_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__4;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__5_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__5;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__6_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__6;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__7;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_instBlockAlgebraTracer(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(lean_object* v_v_2_){
_start:
{
lean_object* v___x_4_; uint8_t v___x_5_; 
v___x_4_ = lean_unsigned_to_nat(256u);
v___x_5_ = lean_nat_dec_lt(v_v_2_, v___x_4_);
if (v___x_5_ == 0)
{
lean_object* v___x_6_; lean_object* v___x_7_; lean_object* v___x_8_; lean_object* v___x_9_; lean_object* v___x_10_; 
v___x_6_ = l_Nat_reprFast(v_v_2_);
v___x_7_ = ((lean_object*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8___closed__0));
v___x_8_ = lean_string_append(v___x_6_, v___x_7_);
v___x_9_ = lean_alloc_ctor(18, 1, 0);
lean_ctor_set(v___x_9_, 0, v___x_8_);
v___x_10_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_10_, 0, v___x_9_);
return v___x_10_;
}
else
{
uint8_t v___x_11_; lean_object* v___x_12_; lean_object* v___x_13_; 
v___x_11_ = lean_uint8_of_nat(v_v_2_);
lean_dec(v_v_2_);
v___x_12_ = lean_box(v___x_11_);
v___x_13_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_13_, 0, v___x_12_);
return v___x_13_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8___boxed(lean_object* v_v_14_, lean_object* v_a_15_){
_start:
{
lean_object* v_res_16_; 
v_res_16_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_v_14_);
return v_res_16_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16(lean_object* v_v_18_){
_start:
{
lean_object* v___x_20_; uint8_t v___x_21_; 
v___x_20_ = lean_unsigned_to_nat(65536u);
v___x_21_ = lean_nat_dec_lt(v_v_18_, v___x_20_);
if (v___x_21_ == 0)
{
lean_object* v___x_22_; lean_object* v___x_23_; lean_object* v___x_24_; lean_object* v___x_25_; lean_object* v___x_26_; 
v___x_22_ = l_Nat_reprFast(v_v_18_);
v___x_23_ = ((lean_object*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16___closed__0));
v___x_24_ = lean_string_append(v___x_22_, v___x_23_);
v___x_25_ = lean_alloc_ctor(18, 1, 0);
lean_ctor_set(v___x_25_, 0, v___x_24_);
v___x_26_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_26_, 0, v___x_25_);
return v___x_26_;
}
else
{
uint16_t v___x_27_; lean_object* v___x_28_; lean_object* v___x_29_; 
v___x_27_ = lean_uint16_of_nat(v_v_18_);
lean_dec(v_v_18_);
v___x_28_ = lean_box(v___x_27_);
v___x_29_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_29_, 0, v___x_28_);
return v___x_29_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16___boxed(lean_object* v_v_30_, lean_object* v_a_31_){
_start:
{
lean_object* v_res_32_; 
v_res_32_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16(v_v_30_);
return v_res_32_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger(lean_object* v_sb_33_, lean_object* v_si_34_, lean_object* v_a_35_){
_start:
{
lean_object* v___x_37_; lean_object* v___x_38_; 
v___x_37_ = lp_zhc__lean_Zhc_Model_SpecInteger_intSize(v_sb_33_, v_si_34_);
v___x_38_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16(v___x_37_);
if (lean_obj_tag(v___x_38_) == 0)
{
lean_object* v_a_39_; uint16_t v___x_40_; lean_object* v___x_41_; 
v_a_39_ = lean_ctor_get(v___x_38_, 0);
lean_inc(v_a_39_);
lean_dec_ref_known(v___x_38_, 1);
v___x_40_ = lean_unbox(v_a_39_);
lean_dec(v_a_39_);
v___x_41_ = zhc_lean_builder_ciphertext_integer_declare(v_a_35_, v___x_40_);
if (lean_obj_tag(v___x_41_) == 0)
{
lean_object* v_a_42_; lean_object* v___x_44_; uint8_t v_isShared_45_; uint8_t v_isSharedCheck_52_; 
v_a_42_ = lean_ctor_get(v___x_41_, 0);
v_isSharedCheck_52_ = !lean_is_exclusive(v___x_41_);
if (v_isSharedCheck_52_ == 0)
{
v___x_44_ = v___x_41_;
v_isShared_45_ = v_isSharedCheck_52_;
goto v_resetjp_43_;
}
else
{
lean_inc(v_a_42_);
lean_dec(v___x_41_);
v___x_44_ = lean_box(0);
v_isShared_45_ = v_isSharedCheck_52_;
goto v_resetjp_43_;
}
v_resetjp_43_:
{
lean_object* v___x_46_; lean_object* v___x_47_; lean_object* v___x_48_; lean_object* v___x_50_; 
v___x_46_ = lean_unsigned_to_nat(0u);
v___x_47_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v_sb_33_, v_si_34_, v___x_46_);
v___x_48_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_48_, 0, v_a_42_);
lean_ctor_set(v___x_48_, 1, v___x_47_);
if (v_isShared_45_ == 0)
{
lean_ctor_set(v___x_44_, 0, v___x_48_);
v___x_50_ = v___x_44_;
goto v_reusejp_49_;
}
else
{
lean_object* v_reuseFailAlloc_51_; 
v_reuseFailAlloc_51_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_51_, 0, v___x_48_);
v___x_50_ = v_reuseFailAlloc_51_;
goto v_reusejp_49_;
}
v_reusejp_49_:
{
return v___x_50_;
}
}
}
else
{
lean_object* v_a_53_; lean_object* v___x_55_; uint8_t v_isShared_56_; uint8_t v_isSharedCheck_60_; 
v_a_53_ = lean_ctor_get(v___x_41_, 0);
v_isSharedCheck_60_ = !lean_is_exclusive(v___x_41_);
if (v_isSharedCheck_60_ == 0)
{
v___x_55_ = v___x_41_;
v_isShared_56_ = v_isSharedCheck_60_;
goto v_resetjp_54_;
}
else
{
lean_inc(v_a_53_);
lean_dec(v___x_41_);
v___x_55_ = lean_box(0);
v_isShared_56_ = v_isSharedCheck_60_;
goto v_resetjp_54_;
}
v_resetjp_54_:
{
lean_object* v___x_58_; 
if (v_isShared_56_ == 0)
{
v___x_58_ = v___x_55_;
goto v_reusejp_57_;
}
else
{
lean_object* v_reuseFailAlloc_59_; 
v_reuseFailAlloc_59_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_59_, 0, v_a_53_);
v___x_58_ = v_reuseFailAlloc_59_;
goto v_reusejp_57_;
}
v_reusejp_57_:
{
return v___x_58_;
}
}
}
}
else
{
lean_object* v_a_61_; lean_object* v___x_63_; uint8_t v_isShared_64_; uint8_t v_isSharedCheck_68_; 
v_a_61_ = lean_ctor_get(v___x_38_, 0);
v_isSharedCheck_68_ = !lean_is_exclusive(v___x_38_);
if (v_isSharedCheck_68_ == 0)
{
v___x_63_ = v___x_38_;
v_isShared_64_ = v_isSharedCheck_68_;
goto v_resetjp_62_;
}
else
{
lean_inc(v_a_61_);
lean_dec(v___x_38_);
v___x_63_ = lean_box(0);
v_isShared_64_ = v_isSharedCheck_68_;
goto v_resetjp_62_;
}
v_resetjp_62_:
{
lean_object* v___x_66_; 
if (v_isShared_64_ == 0)
{
v___x_66_ = v___x_63_;
goto v_reusejp_65_;
}
else
{
lean_object* v_reuseFailAlloc_67_; 
v_reuseFailAlloc_67_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_67_, 0, v_a_61_);
v___x_66_ = v_reuseFailAlloc_67_;
goto v_reusejp_65_;
}
v_reusejp_65_:
{
return v___x_66_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger___boxed(lean_object* v_sb_69_, lean_object* v_si_70_, lean_object* v_a_71_, lean_object* v_a_72_){
_start:
{
lean_object* v_res_73_; 
v_res_73_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger(v_sb_69_, v_si_70_, v_a_71_);
lean_dec(v_a_71_);
lean_dec(v_si_70_);
lean_dec_ref(v_sb_69_);
return v_res_73_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock(lean_object* v_sb_74_, lean_object* v_v_75_, lean_object* v_a_76_){
_start:
{
lean_object* v___x_78_; 
lean_inc(v_v_75_);
v___x_78_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_v_75_);
if (lean_obj_tag(v___x_78_) == 0)
{
lean_object* v_a_79_; uint8_t v___x_80_; lean_object* v___x_81_; 
v_a_79_ = lean_ctor_get(v___x_78_, 0);
lean_inc(v_a_79_);
lean_dec_ref_known(v___x_78_, 1);
v___x_80_ = lean_unbox(v_a_79_);
lean_dec(v_a_79_);
v___x_81_ = zhc_lean_builder_block_let_plaintext(v_a_76_, v___x_80_);
if (lean_obj_tag(v___x_81_) == 0)
{
lean_object* v_a_82_; lean_object* v___x_84_; uint8_t v_isShared_85_; uint8_t v_isSharedCheck_91_; 
v_a_82_ = lean_ctor_get(v___x_81_, 0);
v_isSharedCheck_91_ = !lean_is_exclusive(v___x_81_);
if (v_isSharedCheck_91_ == 0)
{
v___x_84_ = v___x_81_;
v_isShared_85_ = v_isSharedCheck_91_;
goto v_resetjp_83_;
}
else
{
lean_inc(v_a_82_);
lean_dec(v___x_81_);
v___x_84_ = lean_box(0);
v_isShared_85_ = v_isSharedCheck_91_;
goto v_resetjp_83_;
}
v_resetjp_83_:
{
lean_object* v___x_86_; lean_object* v___x_87_; lean_object* v___x_89_; 
v___x_86_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(v_sb_74_, v_v_75_);
lean_dec(v_v_75_);
v___x_87_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_87_, 0, v_a_82_);
lean_ctor_set(v___x_87_, 1, v___x_86_);
if (v_isShared_85_ == 0)
{
lean_ctor_set(v___x_84_, 0, v___x_87_);
v___x_89_ = v___x_84_;
goto v_reusejp_88_;
}
else
{
lean_object* v_reuseFailAlloc_90_; 
v_reuseFailAlloc_90_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_90_, 0, v___x_87_);
v___x_89_ = v_reuseFailAlloc_90_;
goto v_reusejp_88_;
}
v_reusejp_88_:
{
return v___x_89_;
}
}
}
else
{
lean_object* v_a_92_; lean_object* v___x_94_; uint8_t v_isShared_95_; uint8_t v_isSharedCheck_99_; 
lean_dec(v_v_75_);
v_a_92_ = lean_ctor_get(v___x_81_, 0);
v_isSharedCheck_99_ = !lean_is_exclusive(v___x_81_);
if (v_isSharedCheck_99_ == 0)
{
v___x_94_ = v___x_81_;
v_isShared_95_ = v_isSharedCheck_99_;
goto v_resetjp_93_;
}
else
{
lean_inc(v_a_92_);
lean_dec(v___x_81_);
v___x_94_ = lean_box(0);
v_isShared_95_ = v_isSharedCheck_99_;
goto v_resetjp_93_;
}
v_resetjp_93_:
{
lean_object* v___x_97_; 
if (v_isShared_95_ == 0)
{
v___x_97_ = v___x_94_;
goto v_reusejp_96_;
}
else
{
lean_object* v_reuseFailAlloc_98_; 
v_reuseFailAlloc_98_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_98_, 0, v_a_92_);
v___x_97_ = v_reuseFailAlloc_98_;
goto v_reusejp_96_;
}
v_reusejp_96_:
{
return v___x_97_;
}
}
}
}
else
{
lean_object* v_a_100_; lean_object* v___x_102_; uint8_t v_isShared_103_; uint8_t v_isSharedCheck_107_; 
lean_dec(v_v_75_);
v_a_100_ = lean_ctor_get(v___x_78_, 0);
v_isSharedCheck_107_ = !lean_is_exclusive(v___x_78_);
if (v_isSharedCheck_107_ == 0)
{
v___x_102_ = v___x_78_;
v_isShared_103_ = v_isSharedCheck_107_;
goto v_resetjp_101_;
}
else
{
lean_inc(v_a_100_);
lean_dec(v___x_78_);
v___x_102_ = lean_box(0);
v_isShared_103_ = v_isSharedCheck_107_;
goto v_resetjp_101_;
}
v_resetjp_101_:
{
lean_object* v___x_105_; 
if (v_isShared_103_ == 0)
{
v___x_105_ = v___x_102_;
goto v_reusejp_104_;
}
else
{
lean_object* v_reuseFailAlloc_106_; 
v_reuseFailAlloc_106_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_106_, 0, v_a_100_);
v___x_105_ = v_reuseFailAlloc_106_;
goto v_reusejp_104_;
}
v_reusejp_104_:
{
return v___x_105_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock___boxed(lean_object* v_sb_108_, lean_object* v_v_109_, lean_object* v_a_110_, lean_object* v_a_111_){
_start:
{
lean_object* v_res_112_; 
v_res_112_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock(v_sb_108_, v_v_109_, v_a_110_);
lean_dec(v_a_110_);
lean_dec_ref(v_sb_108_);
return v_res_112_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock(lean_object* v_sb_113_, lean_object* v_v_114_, lean_object* v_a_115_){
_start:
{
lean_object* v___x_117_; 
lean_inc(v_v_114_);
v___x_117_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_v_114_);
if (lean_obj_tag(v___x_117_) == 0)
{
lean_object* v_a_118_; uint8_t v___x_119_; lean_object* v___x_120_; 
v_a_118_ = lean_ctor_get(v___x_117_, 0);
lean_inc(v_a_118_);
lean_dec_ref_known(v___x_117_, 1);
v___x_119_ = lean_unbox(v_a_118_);
lean_dec(v_a_118_);
v___x_120_ = zhc_lean_builder_block_let_ciphertext(v_a_115_, v___x_119_);
if (lean_obj_tag(v___x_120_) == 0)
{
lean_object* v_a_121_; lean_object* v___x_123_; uint8_t v_isShared_124_; uint8_t v_isSharedCheck_130_; 
v_a_121_ = lean_ctor_get(v___x_120_, 0);
v_isSharedCheck_130_ = !lean_is_exclusive(v___x_120_);
if (v_isSharedCheck_130_ == 0)
{
v___x_123_ = v___x_120_;
v_isShared_124_ = v_isSharedCheck_130_;
goto v_resetjp_122_;
}
else
{
lean_inc(v_a_121_);
lean_dec(v___x_120_);
v___x_123_ = lean_box(0);
v_isShared_124_ = v_isSharedCheck_130_;
goto v_resetjp_122_;
}
v_resetjp_122_:
{
lean_object* v___x_125_; lean_object* v___x_126_; lean_object* v___x_128_; 
v___x_125_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_113_, v_v_114_);
lean_dec(v_v_114_);
v___x_126_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_126_, 0, v_a_121_);
lean_ctor_set(v___x_126_, 1, v___x_125_);
if (v_isShared_124_ == 0)
{
lean_ctor_set(v___x_123_, 0, v___x_126_);
v___x_128_ = v___x_123_;
goto v_reusejp_127_;
}
else
{
lean_object* v_reuseFailAlloc_129_; 
v_reuseFailAlloc_129_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_129_, 0, v___x_126_);
v___x_128_ = v_reuseFailAlloc_129_;
goto v_reusejp_127_;
}
v_reusejp_127_:
{
return v___x_128_;
}
}
}
else
{
lean_object* v_a_131_; lean_object* v___x_133_; uint8_t v_isShared_134_; uint8_t v_isSharedCheck_138_; 
lean_dec(v_v_114_);
v_a_131_ = lean_ctor_get(v___x_120_, 0);
v_isSharedCheck_138_ = !lean_is_exclusive(v___x_120_);
if (v_isSharedCheck_138_ == 0)
{
v___x_133_ = v___x_120_;
v_isShared_134_ = v_isSharedCheck_138_;
goto v_resetjp_132_;
}
else
{
lean_inc(v_a_131_);
lean_dec(v___x_120_);
v___x_133_ = lean_box(0);
v_isShared_134_ = v_isSharedCheck_138_;
goto v_resetjp_132_;
}
v_resetjp_132_:
{
lean_object* v___x_136_; 
if (v_isShared_134_ == 0)
{
v___x_136_ = v___x_133_;
goto v_reusejp_135_;
}
else
{
lean_object* v_reuseFailAlloc_137_; 
v_reuseFailAlloc_137_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_137_, 0, v_a_131_);
v___x_136_ = v_reuseFailAlloc_137_;
goto v_reusejp_135_;
}
v_reusejp_135_:
{
return v___x_136_;
}
}
}
}
else
{
lean_object* v_a_139_; lean_object* v___x_141_; uint8_t v_isShared_142_; uint8_t v_isSharedCheck_146_; 
lean_dec(v_v_114_);
v_a_139_ = lean_ctor_get(v___x_117_, 0);
v_isSharedCheck_146_ = !lean_is_exclusive(v___x_117_);
if (v_isSharedCheck_146_ == 0)
{
v___x_141_ = v___x_117_;
v_isShared_142_ = v_isSharedCheck_146_;
goto v_resetjp_140_;
}
else
{
lean_inc(v_a_139_);
lean_dec(v___x_117_);
v___x_141_ = lean_box(0);
v_isShared_142_ = v_isSharedCheck_146_;
goto v_resetjp_140_;
}
v_resetjp_140_:
{
lean_object* v___x_144_; 
if (v_isShared_142_ == 0)
{
v___x_144_ = v___x_141_;
goto v_reusejp_143_;
}
else
{
lean_object* v_reuseFailAlloc_145_; 
v_reuseFailAlloc_145_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_145_, 0, v_a_139_);
v___x_144_ = v_reuseFailAlloc_145_;
goto v_reusejp_143_;
}
v_reusejp_143_:
{
return v___x_144_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock___boxed(lean_object* v_sb_147_, lean_object* v_v_148_, lean_object* v_a_149_, lean_object* v_a_150_){
_start:
{
lean_object* v_res_151_; 
v_res_151_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock(v_sb_147_, v_v_148_, v_a_149_);
lean_dec(v_a_149_);
lean_dec_ref(v_sb_147_);
return v_res_151_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(lean_object* v_sb_152_, lean_object* v_a_153_, lean_object* v_b_154_, lean_object* v_a_155_){
_start:
{
lean_object* v_handle_157_; lean_object* v_model_158_; lean_object* v_handle_159_; lean_object* v_model_160_; lean_object* v___x_162_; uint8_t v_isShared_163_; uint8_t v_isSharedCheck_185_; 
v_handle_157_ = lean_ctor_get(v_a_153_, 0);
v_model_158_ = lean_ctor_get(v_a_153_, 1);
v_handle_159_ = lean_ctor_get(v_b_154_, 0);
v_model_160_ = lean_ctor_get(v_b_154_, 1);
v_isSharedCheck_185_ = !lean_is_exclusive(v_b_154_);
if (v_isSharedCheck_185_ == 0)
{
v___x_162_ = v_b_154_;
v_isShared_163_ = v_isSharedCheck_185_;
goto v_resetjp_161_;
}
else
{
lean_inc(v_model_160_);
lean_inc(v_handle_159_);
lean_dec(v_b_154_);
v___x_162_ = lean_box(0);
v_isShared_163_ = v_isSharedCheck_185_;
goto v_resetjp_161_;
}
v_resetjp_161_:
{
lean_object* v___x_164_; 
v___x_164_ = zhc_lean_builder_block_add(v_a_155_, v_handle_157_, v_handle_159_);
lean_dec(v_handle_159_);
if (lean_obj_tag(v___x_164_) == 0)
{
lean_object* v_a_165_; lean_object* v___x_167_; uint8_t v_isShared_168_; uint8_t v_isSharedCheck_176_; 
v_a_165_ = lean_ctor_get(v___x_164_, 0);
v_isSharedCheck_176_ = !lean_is_exclusive(v___x_164_);
if (v_isSharedCheck_176_ == 0)
{
v___x_167_ = v___x_164_;
v_isShared_168_ = v_isSharedCheck_176_;
goto v_resetjp_166_;
}
else
{
lean_inc(v_a_165_);
lean_dec(v___x_164_);
v___x_167_ = lean_box(0);
v_isShared_168_ = v_isSharedCheck_176_;
goto v_resetjp_166_;
}
v_resetjp_166_:
{
lean_object* v___x_169_; lean_object* v___x_171_; 
v___x_169_ = lp_zhc__lean_Zhc_Model_Operators_add(v_sb_152_, v_model_158_, v_model_160_);
lean_dec(v_model_160_);
if (v_isShared_163_ == 0)
{
lean_ctor_set(v___x_162_, 1, v___x_169_);
lean_ctor_set(v___x_162_, 0, v_a_165_);
v___x_171_ = v___x_162_;
goto v_reusejp_170_;
}
else
{
lean_object* v_reuseFailAlloc_175_; 
v_reuseFailAlloc_175_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_175_, 0, v_a_165_);
lean_ctor_set(v_reuseFailAlloc_175_, 1, v___x_169_);
v___x_171_ = v_reuseFailAlloc_175_;
goto v_reusejp_170_;
}
v_reusejp_170_:
{
lean_object* v___x_173_; 
if (v_isShared_168_ == 0)
{
lean_ctor_set(v___x_167_, 0, v___x_171_);
v___x_173_ = v___x_167_;
goto v_reusejp_172_;
}
else
{
lean_object* v_reuseFailAlloc_174_; 
v_reuseFailAlloc_174_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_174_, 0, v___x_171_);
v___x_173_ = v_reuseFailAlloc_174_;
goto v_reusejp_172_;
}
v_reusejp_172_:
{
return v___x_173_;
}
}
}
}
else
{
lean_object* v_a_177_; lean_object* v___x_179_; uint8_t v_isShared_180_; uint8_t v_isSharedCheck_184_; 
lean_del_object(v___x_162_);
lean_dec(v_model_160_);
v_a_177_ = lean_ctor_get(v___x_164_, 0);
v_isSharedCheck_184_ = !lean_is_exclusive(v___x_164_);
if (v_isSharedCheck_184_ == 0)
{
v___x_179_ = v___x_164_;
v_isShared_180_ = v_isSharedCheck_184_;
goto v_resetjp_178_;
}
else
{
lean_inc(v_a_177_);
lean_dec(v___x_164_);
v___x_179_ = lean_box(0);
v_isShared_180_ = v_isSharedCheck_184_;
goto v_resetjp_178_;
}
v_resetjp_178_:
{
lean_object* v___x_182_; 
if (v_isShared_180_ == 0)
{
v___x_182_ = v___x_179_;
goto v_reusejp_181_;
}
else
{
lean_object* v_reuseFailAlloc_183_; 
v_reuseFailAlloc_183_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_183_, 0, v_a_177_);
v___x_182_ = v_reuseFailAlloc_183_;
goto v_reusejp_181_;
}
v_reusejp_181_:
{
return v___x_182_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt___boxed(lean_object* v_sb_186_, lean_object* v_a_187_, lean_object* v_b_188_, lean_object* v_a_189_, lean_object* v_a_190_){
_start:
{
lean_object* v_res_191_; 
v_res_191_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v_sb_186_, v_a_187_, v_b_188_, v_a_189_);
lean_dec(v_a_189_);
lean_dec_ref(v_a_187_);
lean_dec_ref(v_sb_186_);
return v_res_191_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subCt(lean_object* v_sb_192_, lean_object* v_a_193_, lean_object* v_b_194_, lean_object* v_a_195_){
_start:
{
lean_object* v_handle_197_; lean_object* v_model_198_; lean_object* v_handle_199_; lean_object* v_model_200_; lean_object* v___x_202_; uint8_t v_isShared_203_; uint8_t v_isSharedCheck_225_; 
v_handle_197_ = lean_ctor_get(v_a_193_, 0);
v_model_198_ = lean_ctor_get(v_a_193_, 1);
v_handle_199_ = lean_ctor_get(v_b_194_, 0);
v_model_200_ = lean_ctor_get(v_b_194_, 1);
v_isSharedCheck_225_ = !lean_is_exclusive(v_b_194_);
if (v_isSharedCheck_225_ == 0)
{
v___x_202_ = v_b_194_;
v_isShared_203_ = v_isSharedCheck_225_;
goto v_resetjp_201_;
}
else
{
lean_inc(v_model_200_);
lean_inc(v_handle_199_);
lean_dec(v_b_194_);
v___x_202_ = lean_box(0);
v_isShared_203_ = v_isSharedCheck_225_;
goto v_resetjp_201_;
}
v_resetjp_201_:
{
lean_object* v___x_204_; 
v___x_204_ = zhc_lean_builder_block_sub(v_a_195_, v_handle_197_, v_handle_199_);
lean_dec(v_handle_199_);
if (lean_obj_tag(v___x_204_) == 0)
{
lean_object* v_a_205_; lean_object* v___x_207_; uint8_t v_isShared_208_; uint8_t v_isSharedCheck_216_; 
v_a_205_ = lean_ctor_get(v___x_204_, 0);
v_isSharedCheck_216_ = !lean_is_exclusive(v___x_204_);
if (v_isSharedCheck_216_ == 0)
{
v___x_207_ = v___x_204_;
v_isShared_208_ = v_isSharedCheck_216_;
goto v_resetjp_206_;
}
else
{
lean_inc(v_a_205_);
lean_dec(v___x_204_);
v___x_207_ = lean_box(0);
v_isShared_208_ = v_isSharedCheck_216_;
goto v_resetjp_206_;
}
v_resetjp_206_:
{
lean_object* v___x_209_; lean_object* v___x_211_; 
v___x_209_ = lp_zhc__lean_Zhc_Model_Operators_sub(v_sb_192_, v_model_198_, v_model_200_);
lean_dec(v_model_200_);
if (v_isShared_203_ == 0)
{
lean_ctor_set(v___x_202_, 1, v___x_209_);
lean_ctor_set(v___x_202_, 0, v_a_205_);
v___x_211_ = v___x_202_;
goto v_reusejp_210_;
}
else
{
lean_object* v_reuseFailAlloc_215_; 
v_reuseFailAlloc_215_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_215_, 0, v_a_205_);
lean_ctor_set(v_reuseFailAlloc_215_, 1, v___x_209_);
v___x_211_ = v_reuseFailAlloc_215_;
goto v_reusejp_210_;
}
v_reusejp_210_:
{
lean_object* v___x_213_; 
if (v_isShared_208_ == 0)
{
lean_ctor_set(v___x_207_, 0, v___x_211_);
v___x_213_ = v___x_207_;
goto v_reusejp_212_;
}
else
{
lean_object* v_reuseFailAlloc_214_; 
v_reuseFailAlloc_214_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_214_, 0, v___x_211_);
v___x_213_ = v_reuseFailAlloc_214_;
goto v_reusejp_212_;
}
v_reusejp_212_:
{
return v___x_213_;
}
}
}
}
else
{
lean_object* v_a_217_; lean_object* v___x_219_; uint8_t v_isShared_220_; uint8_t v_isSharedCheck_224_; 
lean_del_object(v___x_202_);
lean_dec(v_model_200_);
v_a_217_ = lean_ctor_get(v___x_204_, 0);
v_isSharedCheck_224_ = !lean_is_exclusive(v___x_204_);
if (v_isSharedCheck_224_ == 0)
{
v___x_219_ = v___x_204_;
v_isShared_220_ = v_isSharedCheck_224_;
goto v_resetjp_218_;
}
else
{
lean_inc(v_a_217_);
lean_dec(v___x_204_);
v___x_219_ = lean_box(0);
v_isShared_220_ = v_isSharedCheck_224_;
goto v_resetjp_218_;
}
v_resetjp_218_:
{
lean_object* v___x_222_; 
if (v_isShared_220_ == 0)
{
v___x_222_ = v___x_219_;
goto v_reusejp_221_;
}
else
{
lean_object* v_reuseFailAlloc_223_; 
v_reuseFailAlloc_223_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_223_, 0, v_a_217_);
v___x_222_ = v_reuseFailAlloc_223_;
goto v_reusejp_221_;
}
v_reusejp_221_:
{
return v___x_222_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subCt___boxed(lean_object* v_sb_226_, lean_object* v_a_227_, lean_object* v_b_228_, lean_object* v_a_229_, lean_object* v_a_230_){
_start:
{
lean_object* v_res_231_; 
v_res_231_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subCt(v_sb_226_, v_a_227_, v_b_228_, v_a_229_);
lean_dec(v_a_229_);
lean_dec_ref(v_a_227_);
lean_dec_ref(v_sb_226_);
return v_res_231_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_shlCt(lean_object* v_sb_232_, lean_object* v_a_233_, lean_object* v_amount_234_, lean_object* v_a_235_){
_start:
{
lean_object* v___x_237_; 
lean_inc(v_amount_234_);
v___x_237_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_amount_234_);
if (lean_obj_tag(v___x_237_) == 0)
{
lean_object* v_a_238_; lean_object* v_handle_239_; lean_object* v_model_240_; lean_object* v___x_242_; uint8_t v_isShared_243_; uint8_t v_isSharedCheck_266_; 
v_a_238_ = lean_ctor_get(v___x_237_, 0);
lean_inc(v_a_238_);
lean_dec_ref_known(v___x_237_, 1);
v_handle_239_ = lean_ctor_get(v_a_233_, 0);
v_model_240_ = lean_ctor_get(v_a_233_, 1);
v_isSharedCheck_266_ = !lean_is_exclusive(v_a_233_);
if (v_isSharedCheck_266_ == 0)
{
v___x_242_ = v_a_233_;
v_isShared_243_ = v_isSharedCheck_266_;
goto v_resetjp_241_;
}
else
{
lean_inc(v_model_240_);
lean_inc(v_handle_239_);
lean_dec(v_a_233_);
v___x_242_ = lean_box(0);
v_isShared_243_ = v_isSharedCheck_266_;
goto v_resetjp_241_;
}
v_resetjp_241_:
{
uint8_t v___x_244_; lean_object* v___x_245_; 
v___x_244_ = lean_unbox(v_a_238_);
lean_dec(v_a_238_);
v___x_245_ = zhc_lean_builder_block_shl(v_a_235_, v_handle_239_, v___x_244_);
lean_dec(v_handle_239_);
if (lean_obj_tag(v___x_245_) == 0)
{
lean_object* v_a_246_; lean_object* v___x_248_; uint8_t v_isShared_249_; uint8_t v_isSharedCheck_257_; 
v_a_246_ = lean_ctor_get(v___x_245_, 0);
v_isSharedCheck_257_ = !lean_is_exclusive(v___x_245_);
if (v_isSharedCheck_257_ == 0)
{
v___x_248_ = v___x_245_;
v_isShared_249_ = v_isSharedCheck_257_;
goto v_resetjp_247_;
}
else
{
lean_inc(v_a_246_);
lean_dec(v___x_245_);
v___x_248_ = lean_box(0);
v_isShared_249_ = v_isSharedCheck_257_;
goto v_resetjp_247_;
}
v_resetjp_247_:
{
lean_object* v___x_250_; lean_object* v___x_252_; 
v___x_250_ = lp_zhc__lean_Zhc_Model_Operators_shl(v_sb_232_, v_model_240_, v_amount_234_);
lean_dec(v_amount_234_);
lean_dec(v_model_240_);
if (v_isShared_243_ == 0)
{
lean_ctor_set(v___x_242_, 1, v___x_250_);
lean_ctor_set(v___x_242_, 0, v_a_246_);
v___x_252_ = v___x_242_;
goto v_reusejp_251_;
}
else
{
lean_object* v_reuseFailAlloc_256_; 
v_reuseFailAlloc_256_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_256_, 0, v_a_246_);
lean_ctor_set(v_reuseFailAlloc_256_, 1, v___x_250_);
v___x_252_ = v_reuseFailAlloc_256_;
goto v_reusejp_251_;
}
v_reusejp_251_:
{
lean_object* v___x_254_; 
if (v_isShared_249_ == 0)
{
lean_ctor_set(v___x_248_, 0, v___x_252_);
v___x_254_ = v___x_248_;
goto v_reusejp_253_;
}
else
{
lean_object* v_reuseFailAlloc_255_; 
v_reuseFailAlloc_255_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_255_, 0, v___x_252_);
v___x_254_ = v_reuseFailAlloc_255_;
goto v_reusejp_253_;
}
v_reusejp_253_:
{
return v___x_254_;
}
}
}
}
else
{
lean_object* v_a_258_; lean_object* v___x_260_; uint8_t v_isShared_261_; uint8_t v_isSharedCheck_265_; 
lean_del_object(v___x_242_);
lean_dec(v_model_240_);
lean_dec(v_amount_234_);
v_a_258_ = lean_ctor_get(v___x_245_, 0);
v_isSharedCheck_265_ = !lean_is_exclusive(v___x_245_);
if (v_isSharedCheck_265_ == 0)
{
v___x_260_ = v___x_245_;
v_isShared_261_ = v_isSharedCheck_265_;
goto v_resetjp_259_;
}
else
{
lean_inc(v_a_258_);
lean_dec(v___x_245_);
v___x_260_ = lean_box(0);
v_isShared_261_ = v_isSharedCheck_265_;
goto v_resetjp_259_;
}
v_resetjp_259_:
{
lean_object* v___x_263_; 
if (v_isShared_261_ == 0)
{
v___x_263_ = v___x_260_;
goto v_reusejp_262_;
}
else
{
lean_object* v_reuseFailAlloc_264_; 
v_reuseFailAlloc_264_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_264_, 0, v_a_258_);
v___x_263_ = v_reuseFailAlloc_264_;
goto v_reusejp_262_;
}
v_reusejp_262_:
{
return v___x_263_;
}
}
}
}
}
else
{
lean_object* v_a_267_; lean_object* v___x_269_; uint8_t v_isShared_270_; uint8_t v_isSharedCheck_274_; 
lean_dec(v_amount_234_);
lean_dec_ref(v_a_233_);
v_a_267_ = lean_ctor_get(v___x_237_, 0);
v_isSharedCheck_274_ = !lean_is_exclusive(v___x_237_);
if (v_isSharedCheck_274_ == 0)
{
v___x_269_ = v___x_237_;
v_isShared_270_ = v_isSharedCheck_274_;
goto v_resetjp_268_;
}
else
{
lean_inc(v_a_267_);
lean_dec(v___x_237_);
v___x_269_ = lean_box(0);
v_isShared_270_ = v_isSharedCheck_274_;
goto v_resetjp_268_;
}
v_resetjp_268_:
{
lean_object* v___x_272_; 
if (v_isShared_270_ == 0)
{
v___x_272_ = v___x_269_;
goto v_reusejp_271_;
}
else
{
lean_object* v_reuseFailAlloc_273_; 
v_reuseFailAlloc_273_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_273_, 0, v_a_267_);
v___x_272_ = v_reuseFailAlloc_273_;
goto v_reusejp_271_;
}
v_reusejp_271_:
{
return v___x_272_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_shlCt___boxed(lean_object* v_sb_275_, lean_object* v_a_276_, lean_object* v_amount_277_, lean_object* v_a_278_, lean_object* v_a_279_){
_start:
{
lean_object* v_res_280_; 
v_res_280_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_shlCt(v_sb_275_, v_a_276_, v_amount_277_, v_a_278_);
lean_dec(v_a_278_);
lean_dec_ref(v_sb_275_);
return v_res_280_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt(lean_object* v_sb_281_, lean_object* v_a_282_, lean_object* v_mul_283_, lean_object* v_b_284_, lean_object* v_a_285_){
_start:
{
lean_object* v___x_287_; 
lean_inc(v_mul_283_);
v___x_287_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_mul_283_);
if (lean_obj_tag(v___x_287_) == 0)
{
lean_object* v_a_288_; lean_object* v_handle_289_; lean_object* v_model_290_; lean_object* v_handle_291_; lean_object* v_model_292_; lean_object* v___x_294_; uint8_t v_isShared_295_; uint8_t v_isSharedCheck_318_; 
v_a_288_ = lean_ctor_get(v___x_287_, 0);
lean_inc(v_a_288_);
lean_dec_ref_known(v___x_287_, 1);
v_handle_289_ = lean_ctor_get(v_a_282_, 0);
v_model_290_ = lean_ctor_get(v_a_282_, 1);
v_handle_291_ = lean_ctor_get(v_b_284_, 0);
v_model_292_ = lean_ctor_get(v_b_284_, 1);
v_isSharedCheck_318_ = !lean_is_exclusive(v_b_284_);
if (v_isSharedCheck_318_ == 0)
{
v___x_294_ = v_b_284_;
v_isShared_295_ = v_isSharedCheck_318_;
goto v_resetjp_293_;
}
else
{
lean_inc(v_model_292_);
lean_inc(v_handle_291_);
lean_dec(v_b_284_);
v___x_294_ = lean_box(0);
v_isShared_295_ = v_isSharedCheck_318_;
goto v_resetjp_293_;
}
v_resetjp_293_:
{
uint8_t v___x_296_; lean_object* v___x_297_; 
v___x_296_ = lean_unbox(v_a_288_);
lean_dec(v_a_288_);
v___x_297_ = zhc_lean_builder_block_mac(v_a_285_, v_handle_289_, v_handle_291_, v___x_296_);
lean_dec(v_handle_291_);
if (lean_obj_tag(v___x_297_) == 0)
{
lean_object* v_a_298_; lean_object* v___x_300_; uint8_t v_isShared_301_; uint8_t v_isSharedCheck_309_; 
v_a_298_ = lean_ctor_get(v___x_297_, 0);
v_isSharedCheck_309_ = !lean_is_exclusive(v___x_297_);
if (v_isSharedCheck_309_ == 0)
{
v___x_300_ = v___x_297_;
v_isShared_301_ = v_isSharedCheck_309_;
goto v_resetjp_299_;
}
else
{
lean_inc(v_a_298_);
lean_dec(v___x_297_);
v___x_300_ = lean_box(0);
v_isShared_301_ = v_isSharedCheck_309_;
goto v_resetjp_299_;
}
v_resetjp_299_:
{
lean_object* v___x_302_; lean_object* v___x_304_; 
v___x_302_ = lp_zhc__lean_Zhc_Model_Operators_mac(v_sb_281_, v_model_290_, v_mul_283_, v_model_292_);
lean_dec(v_model_292_);
lean_dec(v_mul_283_);
if (v_isShared_295_ == 0)
{
lean_ctor_set(v___x_294_, 1, v___x_302_);
lean_ctor_set(v___x_294_, 0, v_a_298_);
v___x_304_ = v___x_294_;
goto v_reusejp_303_;
}
else
{
lean_object* v_reuseFailAlloc_308_; 
v_reuseFailAlloc_308_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_308_, 0, v_a_298_);
lean_ctor_set(v_reuseFailAlloc_308_, 1, v___x_302_);
v___x_304_ = v_reuseFailAlloc_308_;
goto v_reusejp_303_;
}
v_reusejp_303_:
{
lean_object* v___x_306_; 
if (v_isShared_301_ == 0)
{
lean_ctor_set(v___x_300_, 0, v___x_304_);
v___x_306_ = v___x_300_;
goto v_reusejp_305_;
}
else
{
lean_object* v_reuseFailAlloc_307_; 
v_reuseFailAlloc_307_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_307_, 0, v___x_304_);
v___x_306_ = v_reuseFailAlloc_307_;
goto v_reusejp_305_;
}
v_reusejp_305_:
{
return v___x_306_;
}
}
}
}
else
{
lean_object* v_a_310_; lean_object* v___x_312_; uint8_t v_isShared_313_; uint8_t v_isSharedCheck_317_; 
lean_del_object(v___x_294_);
lean_dec(v_model_292_);
lean_dec(v_mul_283_);
v_a_310_ = lean_ctor_get(v___x_297_, 0);
v_isSharedCheck_317_ = !lean_is_exclusive(v___x_297_);
if (v_isSharedCheck_317_ == 0)
{
v___x_312_ = v___x_297_;
v_isShared_313_ = v_isSharedCheck_317_;
goto v_resetjp_311_;
}
else
{
lean_inc(v_a_310_);
lean_dec(v___x_297_);
v___x_312_ = lean_box(0);
v_isShared_313_ = v_isSharedCheck_317_;
goto v_resetjp_311_;
}
v_resetjp_311_:
{
lean_object* v___x_315_; 
if (v_isShared_313_ == 0)
{
v___x_315_ = v___x_312_;
goto v_reusejp_314_;
}
else
{
lean_object* v_reuseFailAlloc_316_; 
v_reuseFailAlloc_316_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_316_, 0, v_a_310_);
v___x_315_ = v_reuseFailAlloc_316_;
goto v_reusejp_314_;
}
v_reusejp_314_:
{
return v___x_315_;
}
}
}
}
}
else
{
lean_object* v_a_319_; lean_object* v___x_321_; uint8_t v_isShared_322_; uint8_t v_isSharedCheck_326_; 
lean_dec_ref(v_b_284_);
lean_dec(v_mul_283_);
v_a_319_ = lean_ctor_get(v___x_287_, 0);
v_isSharedCheck_326_ = !lean_is_exclusive(v___x_287_);
if (v_isSharedCheck_326_ == 0)
{
v___x_321_ = v___x_287_;
v_isShared_322_ = v_isSharedCheck_326_;
goto v_resetjp_320_;
}
else
{
lean_inc(v_a_319_);
lean_dec(v___x_287_);
v___x_321_ = lean_box(0);
v_isShared_322_ = v_isSharedCheck_326_;
goto v_resetjp_320_;
}
v_resetjp_320_:
{
lean_object* v___x_324_; 
if (v_isShared_322_ == 0)
{
v___x_324_ = v___x_321_;
goto v_reusejp_323_;
}
else
{
lean_object* v_reuseFailAlloc_325_; 
v_reuseFailAlloc_325_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_325_, 0, v_a_319_);
v___x_324_ = v_reuseFailAlloc_325_;
goto v_reusejp_323_;
}
v_reusejp_323_:
{
return v___x_324_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt___boxed(lean_object* v_sb_327_, lean_object* v_a_328_, lean_object* v_mul_329_, lean_object* v_b_330_, lean_object* v_a_331_, lean_object* v_a_332_){
_start:
{
lean_object* v_res_333_; 
v_res_333_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt(v_sb_327_, v_a_328_, v_mul_329_, v_b_330_, v_a_331_);
lean_dec(v_a_331_);
lean_dec_ref(v_a_328_);
lean_dec_ref(v_sb_327_);
return v_res_333_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt(lean_object* v_sb_334_, lean_object* v_a_335_, lean_object* v_b_336_, lean_object* v_a_337_){
_start:
{
lean_object* v_handle_339_; lean_object* v_model_340_; lean_object* v_handle_341_; lean_object* v_model_342_; lean_object* v___x_344_; uint8_t v_isShared_345_; uint8_t v_isSharedCheck_367_; 
v_handle_339_ = lean_ctor_get(v_a_335_, 0);
v_model_340_ = lean_ctor_get(v_a_335_, 1);
v_handle_341_ = lean_ctor_get(v_b_336_, 0);
v_model_342_ = lean_ctor_get(v_b_336_, 1);
v_isSharedCheck_367_ = !lean_is_exclusive(v_b_336_);
if (v_isSharedCheck_367_ == 0)
{
v___x_344_ = v_b_336_;
v_isShared_345_ = v_isSharedCheck_367_;
goto v_resetjp_343_;
}
else
{
lean_inc(v_model_342_);
lean_inc(v_handle_341_);
lean_dec(v_b_336_);
v___x_344_ = lean_box(0);
v_isShared_345_ = v_isSharedCheck_367_;
goto v_resetjp_343_;
}
v_resetjp_343_:
{
lean_object* v___x_346_; 
v___x_346_ = zhc_lean_builder_block_add_plaintext(v_a_337_, v_handle_339_, v_handle_341_);
lean_dec(v_handle_341_);
if (lean_obj_tag(v___x_346_) == 0)
{
lean_object* v_a_347_; lean_object* v___x_349_; uint8_t v_isShared_350_; uint8_t v_isSharedCheck_358_; 
v_a_347_ = lean_ctor_get(v___x_346_, 0);
v_isSharedCheck_358_ = !lean_is_exclusive(v___x_346_);
if (v_isSharedCheck_358_ == 0)
{
v___x_349_ = v___x_346_;
v_isShared_350_ = v_isSharedCheck_358_;
goto v_resetjp_348_;
}
else
{
lean_inc(v_a_347_);
lean_dec(v___x_346_);
v___x_349_ = lean_box(0);
v_isShared_350_ = v_isSharedCheck_358_;
goto v_resetjp_348_;
}
v_resetjp_348_:
{
lean_object* v___x_351_; lean_object* v___x_353_; 
v___x_351_ = lp_zhc__lean_Zhc_Model_Operators_addPt(v_sb_334_, v_model_340_, v_model_342_);
lean_dec(v_model_342_);
if (v_isShared_345_ == 0)
{
lean_ctor_set(v___x_344_, 1, v___x_351_);
lean_ctor_set(v___x_344_, 0, v_a_347_);
v___x_353_ = v___x_344_;
goto v_reusejp_352_;
}
else
{
lean_object* v_reuseFailAlloc_357_; 
v_reuseFailAlloc_357_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_357_, 0, v_a_347_);
lean_ctor_set(v_reuseFailAlloc_357_, 1, v___x_351_);
v___x_353_ = v_reuseFailAlloc_357_;
goto v_reusejp_352_;
}
v_reusejp_352_:
{
lean_object* v___x_355_; 
if (v_isShared_350_ == 0)
{
lean_ctor_set(v___x_349_, 0, v___x_353_);
v___x_355_ = v___x_349_;
goto v_reusejp_354_;
}
else
{
lean_object* v_reuseFailAlloc_356_; 
v_reuseFailAlloc_356_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_356_, 0, v___x_353_);
v___x_355_ = v_reuseFailAlloc_356_;
goto v_reusejp_354_;
}
v_reusejp_354_:
{
return v___x_355_;
}
}
}
}
else
{
lean_object* v_a_359_; lean_object* v___x_361_; uint8_t v_isShared_362_; uint8_t v_isSharedCheck_366_; 
lean_del_object(v___x_344_);
lean_dec(v_model_342_);
v_a_359_ = lean_ctor_get(v___x_346_, 0);
v_isSharedCheck_366_ = !lean_is_exclusive(v___x_346_);
if (v_isSharedCheck_366_ == 0)
{
v___x_361_ = v___x_346_;
v_isShared_362_ = v_isSharedCheck_366_;
goto v_resetjp_360_;
}
else
{
lean_inc(v_a_359_);
lean_dec(v___x_346_);
v___x_361_ = lean_box(0);
v_isShared_362_ = v_isSharedCheck_366_;
goto v_resetjp_360_;
}
v_resetjp_360_:
{
lean_object* v___x_364_; 
if (v_isShared_362_ == 0)
{
v___x_364_ = v___x_361_;
goto v_reusejp_363_;
}
else
{
lean_object* v_reuseFailAlloc_365_; 
v_reuseFailAlloc_365_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_365_, 0, v_a_359_);
v___x_364_ = v_reuseFailAlloc_365_;
goto v_reusejp_363_;
}
v_reusejp_363_:
{
return v___x_364_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt___boxed(lean_object* v_sb_368_, lean_object* v_a_369_, lean_object* v_b_370_, lean_object* v_a_371_, lean_object* v_a_372_){
_start:
{
lean_object* v_res_373_; 
v_res_373_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt(v_sb_368_, v_a_369_, v_b_370_, v_a_371_);
lean_dec(v_a_371_);
lean_dec_ref(v_a_369_);
lean_dec_ref(v_sb_368_);
return v_res_373_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subPt(lean_object* v_sb_374_, lean_object* v_a_375_, lean_object* v_b_376_, lean_object* v_a_377_){
_start:
{
lean_object* v_handle_379_; lean_object* v_model_380_; lean_object* v_handle_381_; lean_object* v_model_382_; lean_object* v___x_384_; uint8_t v_isShared_385_; uint8_t v_isSharedCheck_407_; 
v_handle_379_ = lean_ctor_get(v_a_375_, 0);
v_model_380_ = lean_ctor_get(v_a_375_, 1);
v_handle_381_ = lean_ctor_get(v_b_376_, 0);
v_model_382_ = lean_ctor_get(v_b_376_, 1);
v_isSharedCheck_407_ = !lean_is_exclusive(v_b_376_);
if (v_isSharedCheck_407_ == 0)
{
v___x_384_ = v_b_376_;
v_isShared_385_ = v_isSharedCheck_407_;
goto v_resetjp_383_;
}
else
{
lean_inc(v_model_382_);
lean_inc(v_handle_381_);
lean_dec(v_b_376_);
v___x_384_ = lean_box(0);
v_isShared_385_ = v_isSharedCheck_407_;
goto v_resetjp_383_;
}
v_resetjp_383_:
{
lean_object* v___x_386_; 
v___x_386_ = zhc_lean_builder_block_sub_plaintext(v_a_377_, v_handle_379_, v_handle_381_);
lean_dec(v_handle_381_);
if (lean_obj_tag(v___x_386_) == 0)
{
lean_object* v_a_387_; lean_object* v___x_389_; uint8_t v_isShared_390_; uint8_t v_isSharedCheck_398_; 
v_a_387_ = lean_ctor_get(v___x_386_, 0);
v_isSharedCheck_398_ = !lean_is_exclusive(v___x_386_);
if (v_isSharedCheck_398_ == 0)
{
v___x_389_ = v___x_386_;
v_isShared_390_ = v_isSharedCheck_398_;
goto v_resetjp_388_;
}
else
{
lean_inc(v_a_387_);
lean_dec(v___x_386_);
v___x_389_ = lean_box(0);
v_isShared_390_ = v_isSharedCheck_398_;
goto v_resetjp_388_;
}
v_resetjp_388_:
{
lean_object* v___x_391_; lean_object* v___x_393_; 
v___x_391_ = lp_zhc__lean_Zhc_Model_Operators_subPt(v_sb_374_, v_model_380_, v_model_382_);
lean_dec(v_model_382_);
if (v_isShared_385_ == 0)
{
lean_ctor_set(v___x_384_, 1, v___x_391_);
lean_ctor_set(v___x_384_, 0, v_a_387_);
v___x_393_ = v___x_384_;
goto v_reusejp_392_;
}
else
{
lean_object* v_reuseFailAlloc_397_; 
v_reuseFailAlloc_397_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_397_, 0, v_a_387_);
lean_ctor_set(v_reuseFailAlloc_397_, 1, v___x_391_);
v___x_393_ = v_reuseFailAlloc_397_;
goto v_reusejp_392_;
}
v_reusejp_392_:
{
lean_object* v___x_395_; 
if (v_isShared_390_ == 0)
{
lean_ctor_set(v___x_389_, 0, v___x_393_);
v___x_395_ = v___x_389_;
goto v_reusejp_394_;
}
else
{
lean_object* v_reuseFailAlloc_396_; 
v_reuseFailAlloc_396_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_396_, 0, v___x_393_);
v___x_395_ = v_reuseFailAlloc_396_;
goto v_reusejp_394_;
}
v_reusejp_394_:
{
return v___x_395_;
}
}
}
}
else
{
lean_object* v_a_399_; lean_object* v___x_401_; uint8_t v_isShared_402_; uint8_t v_isSharedCheck_406_; 
lean_del_object(v___x_384_);
lean_dec(v_model_382_);
v_a_399_ = lean_ctor_get(v___x_386_, 0);
v_isSharedCheck_406_ = !lean_is_exclusive(v___x_386_);
if (v_isSharedCheck_406_ == 0)
{
v___x_401_ = v___x_386_;
v_isShared_402_ = v_isSharedCheck_406_;
goto v_resetjp_400_;
}
else
{
lean_inc(v_a_399_);
lean_dec(v___x_386_);
v___x_401_ = lean_box(0);
v_isShared_402_ = v_isSharedCheck_406_;
goto v_resetjp_400_;
}
v_resetjp_400_:
{
lean_object* v___x_404_; 
if (v_isShared_402_ == 0)
{
v___x_404_ = v___x_401_;
goto v_reusejp_403_;
}
else
{
lean_object* v_reuseFailAlloc_405_; 
v_reuseFailAlloc_405_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_405_, 0, v_a_399_);
v___x_404_ = v_reuseFailAlloc_405_;
goto v_reusejp_403_;
}
v_reusejp_403_:
{
return v___x_404_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subPt___boxed(lean_object* v_sb_408_, lean_object* v_a_409_, lean_object* v_b_410_, lean_object* v_a_411_, lean_object* v_a_412_){
_start:
{
lean_object* v_res_413_; 
v_res_413_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subPt(v_sb_408_, v_a_409_, v_b_410_, v_a_411_);
lean_dec(v_a_411_);
lean_dec_ref(v_a_409_);
lean_dec_ref(v_sb_408_);
return v_res_413_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_ptSub(lean_object* v_sb_414_, lean_object* v_a_415_, lean_object* v_b_416_, lean_object* v_a_417_){
_start:
{
lean_object* v_handle_419_; lean_object* v_model_420_; lean_object* v_handle_421_; lean_object* v_model_422_; lean_object* v___x_424_; uint8_t v_isShared_425_; uint8_t v_isSharedCheck_447_; 
v_handle_419_ = lean_ctor_get(v_a_415_, 0);
v_model_420_ = lean_ctor_get(v_a_415_, 1);
v_handle_421_ = lean_ctor_get(v_b_416_, 0);
v_model_422_ = lean_ctor_get(v_b_416_, 1);
v_isSharedCheck_447_ = !lean_is_exclusive(v_b_416_);
if (v_isSharedCheck_447_ == 0)
{
v___x_424_ = v_b_416_;
v_isShared_425_ = v_isSharedCheck_447_;
goto v_resetjp_423_;
}
else
{
lean_inc(v_model_422_);
lean_inc(v_handle_421_);
lean_dec(v_b_416_);
v___x_424_ = lean_box(0);
v_isShared_425_ = v_isSharedCheck_447_;
goto v_resetjp_423_;
}
v_resetjp_423_:
{
lean_object* v___x_426_; 
v___x_426_ = zhc_lean_builder_block_plaintext_sub(v_a_417_, v_handle_419_, v_handle_421_);
lean_dec(v_handle_421_);
if (lean_obj_tag(v___x_426_) == 0)
{
lean_object* v_a_427_; lean_object* v___x_429_; uint8_t v_isShared_430_; uint8_t v_isSharedCheck_438_; 
v_a_427_ = lean_ctor_get(v___x_426_, 0);
v_isSharedCheck_438_ = !lean_is_exclusive(v___x_426_);
if (v_isSharedCheck_438_ == 0)
{
v___x_429_ = v___x_426_;
v_isShared_430_ = v_isSharedCheck_438_;
goto v_resetjp_428_;
}
else
{
lean_inc(v_a_427_);
lean_dec(v___x_426_);
v___x_429_ = lean_box(0);
v_isShared_430_ = v_isSharedCheck_438_;
goto v_resetjp_428_;
}
v_resetjp_428_:
{
lean_object* v___x_431_; lean_object* v___x_433_; 
v___x_431_ = lp_zhc__lean_Zhc_Model_Operators_subCt(v_sb_414_, v_model_420_, v_model_422_);
lean_dec(v_model_422_);
if (v_isShared_425_ == 0)
{
lean_ctor_set(v___x_424_, 1, v___x_431_);
lean_ctor_set(v___x_424_, 0, v_a_427_);
v___x_433_ = v___x_424_;
goto v_reusejp_432_;
}
else
{
lean_object* v_reuseFailAlloc_437_; 
v_reuseFailAlloc_437_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_437_, 0, v_a_427_);
lean_ctor_set(v_reuseFailAlloc_437_, 1, v___x_431_);
v___x_433_ = v_reuseFailAlloc_437_;
goto v_reusejp_432_;
}
v_reusejp_432_:
{
lean_object* v___x_435_; 
if (v_isShared_430_ == 0)
{
lean_ctor_set(v___x_429_, 0, v___x_433_);
v___x_435_ = v___x_429_;
goto v_reusejp_434_;
}
else
{
lean_object* v_reuseFailAlloc_436_; 
v_reuseFailAlloc_436_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_436_, 0, v___x_433_);
v___x_435_ = v_reuseFailAlloc_436_;
goto v_reusejp_434_;
}
v_reusejp_434_:
{
return v___x_435_;
}
}
}
}
else
{
lean_object* v_a_439_; lean_object* v___x_441_; uint8_t v_isShared_442_; uint8_t v_isSharedCheck_446_; 
lean_del_object(v___x_424_);
lean_dec(v_model_422_);
v_a_439_ = lean_ctor_get(v___x_426_, 0);
v_isSharedCheck_446_ = !lean_is_exclusive(v___x_426_);
if (v_isSharedCheck_446_ == 0)
{
v___x_441_ = v___x_426_;
v_isShared_442_ = v_isSharedCheck_446_;
goto v_resetjp_440_;
}
else
{
lean_inc(v_a_439_);
lean_dec(v___x_426_);
v___x_441_ = lean_box(0);
v_isShared_442_ = v_isSharedCheck_446_;
goto v_resetjp_440_;
}
v_resetjp_440_:
{
lean_object* v___x_444_; 
if (v_isShared_442_ == 0)
{
v___x_444_ = v___x_441_;
goto v_reusejp_443_;
}
else
{
lean_object* v_reuseFailAlloc_445_; 
v_reuseFailAlloc_445_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_445_, 0, v_a_439_);
v___x_444_ = v_reuseFailAlloc_445_;
goto v_reusejp_443_;
}
v_reusejp_443_:
{
return v___x_444_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_ptSub___boxed(lean_object* v_sb_448_, lean_object* v_a_449_, lean_object* v_b_450_, lean_object* v_a_451_, lean_object* v_a_452_){
_start:
{
lean_object* v_res_453_; 
v_res_453_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_ptSub(v_sb_448_, v_a_449_, v_b_450_, v_a_451_);
lean_dec(v_a_451_);
lean_dec_ref(v_a_449_);
lean_dec_ref(v_sb_448_);
return v_res_453_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_mulPt(lean_object* v_sb_454_, lean_object* v_a_455_, lean_object* v_b_456_, lean_object* v_a_457_){
_start:
{
lean_object* v_handle_459_; lean_object* v_model_460_; lean_object* v_handle_461_; lean_object* v_model_462_; lean_object* v___x_464_; uint8_t v_isShared_465_; uint8_t v_isSharedCheck_487_; 
v_handle_459_ = lean_ctor_get(v_a_455_, 0);
v_model_460_ = lean_ctor_get(v_a_455_, 1);
v_handle_461_ = lean_ctor_get(v_b_456_, 0);
v_model_462_ = lean_ctor_get(v_b_456_, 1);
v_isSharedCheck_487_ = !lean_is_exclusive(v_b_456_);
if (v_isSharedCheck_487_ == 0)
{
v___x_464_ = v_b_456_;
v_isShared_465_ = v_isSharedCheck_487_;
goto v_resetjp_463_;
}
else
{
lean_inc(v_model_462_);
lean_inc(v_handle_461_);
lean_dec(v_b_456_);
v___x_464_ = lean_box(0);
v_isShared_465_ = v_isSharedCheck_487_;
goto v_resetjp_463_;
}
v_resetjp_463_:
{
lean_object* v___x_466_; 
v___x_466_ = zhc_lean_builder_block_mul_plaintext(v_a_457_, v_handle_459_, v_handle_461_);
lean_dec(v_handle_461_);
if (lean_obj_tag(v___x_466_) == 0)
{
lean_object* v_a_467_; lean_object* v___x_469_; uint8_t v_isShared_470_; uint8_t v_isSharedCheck_478_; 
v_a_467_ = lean_ctor_get(v___x_466_, 0);
v_isSharedCheck_478_ = !lean_is_exclusive(v___x_466_);
if (v_isSharedCheck_478_ == 0)
{
v___x_469_ = v___x_466_;
v_isShared_470_ = v_isSharedCheck_478_;
goto v_resetjp_468_;
}
else
{
lean_inc(v_a_467_);
lean_dec(v___x_466_);
v___x_469_ = lean_box(0);
v_isShared_470_ = v_isSharedCheck_478_;
goto v_resetjp_468_;
}
v_resetjp_468_:
{
lean_object* v___x_471_; lean_object* v___x_473_; 
v___x_471_ = lp_zhc__lean_Zhc_Model_Operators_mulPt(v_sb_454_, v_model_460_, v_model_462_);
lean_dec(v_model_462_);
if (v_isShared_465_ == 0)
{
lean_ctor_set(v___x_464_, 1, v___x_471_);
lean_ctor_set(v___x_464_, 0, v_a_467_);
v___x_473_ = v___x_464_;
goto v_reusejp_472_;
}
else
{
lean_object* v_reuseFailAlloc_477_; 
v_reuseFailAlloc_477_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_477_, 0, v_a_467_);
lean_ctor_set(v_reuseFailAlloc_477_, 1, v___x_471_);
v___x_473_ = v_reuseFailAlloc_477_;
goto v_reusejp_472_;
}
v_reusejp_472_:
{
lean_object* v___x_475_; 
if (v_isShared_470_ == 0)
{
lean_ctor_set(v___x_469_, 0, v___x_473_);
v___x_475_ = v___x_469_;
goto v_reusejp_474_;
}
else
{
lean_object* v_reuseFailAlloc_476_; 
v_reuseFailAlloc_476_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_476_, 0, v___x_473_);
v___x_475_ = v_reuseFailAlloc_476_;
goto v_reusejp_474_;
}
v_reusejp_474_:
{
return v___x_475_;
}
}
}
}
else
{
lean_object* v_a_479_; lean_object* v___x_481_; uint8_t v_isShared_482_; uint8_t v_isSharedCheck_486_; 
lean_del_object(v___x_464_);
lean_dec(v_model_462_);
v_a_479_ = lean_ctor_get(v___x_466_, 0);
v_isSharedCheck_486_ = !lean_is_exclusive(v___x_466_);
if (v_isSharedCheck_486_ == 0)
{
v___x_481_ = v___x_466_;
v_isShared_482_ = v_isSharedCheck_486_;
goto v_resetjp_480_;
}
else
{
lean_inc(v_a_479_);
lean_dec(v___x_466_);
v___x_481_ = lean_box(0);
v_isShared_482_ = v_isSharedCheck_486_;
goto v_resetjp_480_;
}
v_resetjp_480_:
{
lean_object* v___x_484_; 
if (v_isShared_482_ == 0)
{
v___x_484_ = v___x_481_;
goto v_reusejp_483_;
}
else
{
lean_object* v_reuseFailAlloc_485_; 
v_reuseFailAlloc_485_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_485_, 0, v_a_479_);
v___x_484_ = v_reuseFailAlloc_485_;
goto v_reusejp_483_;
}
v_reusejp_483_:
{
return v___x_484_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_mulPt___boxed(lean_object* v_sb_488_, lean_object* v_a_489_, lean_object* v_b_490_, lean_object* v_a_491_, lean_object* v_a_492_){
_start:
{
lean_object* v_res_493_; 
v_res_493_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_mulPt(v_sb_488_, v_a_489_, v_b_490_, v_a_491_);
lean_dec(v_a_491_);
lean_dec_ref(v_a_489_);
lean_dec_ref(v_sb_488_);
return v_res_493_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(lean_object* v_x_494_, lean_object* v_i_495_, lean_object* v_a_496_){
_start:
{
lean_object* v___x_498_; 
lean_inc(v_i_495_);
v___x_498_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_i_495_);
if (lean_obj_tag(v___x_498_) == 0)
{
lean_object* v_a_499_; lean_object* v_handle_500_; lean_object* v_model_501_; lean_object* v___x_503_; uint8_t v_isShared_504_; uint8_t v_isSharedCheck_527_; 
v_a_499_ = lean_ctor_get(v___x_498_, 0);
lean_inc(v_a_499_);
lean_dec_ref_known(v___x_498_, 1);
v_handle_500_ = lean_ctor_get(v_x_494_, 0);
v_model_501_ = lean_ctor_get(v_x_494_, 1);
v_isSharedCheck_527_ = !lean_is_exclusive(v_x_494_);
if (v_isSharedCheck_527_ == 0)
{
v___x_503_ = v_x_494_;
v_isShared_504_ = v_isSharedCheck_527_;
goto v_resetjp_502_;
}
else
{
lean_inc(v_model_501_);
lean_inc(v_handle_500_);
lean_dec(v_x_494_);
v___x_503_ = lean_box(0);
v_isShared_504_ = v_isSharedCheck_527_;
goto v_resetjp_502_;
}
v_resetjp_502_:
{
uint8_t v___x_505_; lean_object* v___x_506_; 
v___x_505_ = lean_unbox(v_a_499_);
lean_dec(v_a_499_);
v___x_506_ = zhc_lean_builder_ciphertext_integer_get_block(v_a_496_, v_handle_500_, v___x_505_);
lean_dec(v_handle_500_);
if (lean_obj_tag(v___x_506_) == 0)
{
lean_object* v_a_507_; lean_object* v___x_509_; uint8_t v_isShared_510_; uint8_t v_isSharedCheck_518_; 
v_a_507_ = lean_ctor_get(v___x_506_, 0);
v_isSharedCheck_518_ = !lean_is_exclusive(v___x_506_);
if (v_isSharedCheck_518_ == 0)
{
v___x_509_ = v___x_506_;
v_isShared_510_ = v_isSharedCheck_518_;
goto v_resetjp_508_;
}
else
{
lean_inc(v_a_507_);
lean_dec(v___x_506_);
v___x_509_ = lean_box(0);
v_isShared_510_ = v_isSharedCheck_518_;
goto v_resetjp_508_;
}
v_resetjp_508_:
{
lean_object* v___x_511_; lean_object* v___x_513_; 
v___x_511_ = l_List_get___redArg(v_model_501_, v_i_495_);
lean_dec(v_model_501_);
if (v_isShared_504_ == 0)
{
lean_ctor_set(v___x_503_, 1, v___x_511_);
lean_ctor_set(v___x_503_, 0, v_a_507_);
v___x_513_ = v___x_503_;
goto v_reusejp_512_;
}
else
{
lean_object* v_reuseFailAlloc_517_; 
v_reuseFailAlloc_517_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_517_, 0, v_a_507_);
lean_ctor_set(v_reuseFailAlloc_517_, 1, v___x_511_);
v___x_513_ = v_reuseFailAlloc_517_;
goto v_reusejp_512_;
}
v_reusejp_512_:
{
lean_object* v___x_515_; 
if (v_isShared_510_ == 0)
{
lean_ctor_set(v___x_509_, 0, v___x_513_);
v___x_515_ = v___x_509_;
goto v_reusejp_514_;
}
else
{
lean_object* v_reuseFailAlloc_516_; 
v_reuseFailAlloc_516_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_516_, 0, v___x_513_);
v___x_515_ = v_reuseFailAlloc_516_;
goto v_reusejp_514_;
}
v_reusejp_514_:
{
return v___x_515_;
}
}
}
}
else
{
lean_object* v_a_519_; lean_object* v___x_521_; uint8_t v_isShared_522_; uint8_t v_isSharedCheck_526_; 
lean_del_object(v___x_503_);
lean_dec(v_model_501_);
lean_dec(v_i_495_);
v_a_519_ = lean_ctor_get(v___x_506_, 0);
v_isSharedCheck_526_ = !lean_is_exclusive(v___x_506_);
if (v_isSharedCheck_526_ == 0)
{
v___x_521_ = v___x_506_;
v_isShared_522_ = v_isSharedCheck_526_;
goto v_resetjp_520_;
}
else
{
lean_inc(v_a_519_);
lean_dec(v___x_506_);
v___x_521_ = lean_box(0);
v_isShared_522_ = v_isSharedCheck_526_;
goto v_resetjp_520_;
}
v_resetjp_520_:
{
lean_object* v___x_524_; 
if (v_isShared_522_ == 0)
{
v___x_524_ = v___x_521_;
goto v_reusejp_523_;
}
else
{
lean_object* v_reuseFailAlloc_525_; 
v_reuseFailAlloc_525_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_525_, 0, v_a_519_);
v___x_524_ = v_reuseFailAlloc_525_;
goto v_reusejp_523_;
}
v_reusejp_523_:
{
return v___x_524_;
}
}
}
}
}
else
{
lean_object* v_a_528_; lean_object* v___x_530_; uint8_t v_isShared_531_; uint8_t v_isSharedCheck_535_; 
lean_dec(v_i_495_);
lean_dec_ref(v_x_494_);
v_a_528_ = lean_ctor_get(v___x_498_, 0);
v_isSharedCheck_535_ = !lean_is_exclusive(v___x_498_);
if (v_isSharedCheck_535_ == 0)
{
v___x_530_ = v___x_498_;
v_isShared_531_ = v_isSharedCheck_535_;
goto v_resetjp_529_;
}
else
{
lean_inc(v_a_528_);
lean_dec(v___x_498_);
v___x_530_ = lean_box(0);
v_isShared_531_ = v_isSharedCheck_535_;
goto v_resetjp_529_;
}
v_resetjp_529_:
{
lean_object* v___x_533_; 
if (v_isShared_531_ == 0)
{
v___x_533_ = v___x_530_;
goto v_reusejp_532_;
}
else
{
lean_object* v_reuseFailAlloc_534_; 
v_reuseFailAlloc_534_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_534_, 0, v_a_528_);
v___x_533_ = v_reuseFailAlloc_534_;
goto v_reusejp_532_;
}
v_reusejp_532_:
{
return v___x_533_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg___boxed(lean_object* v_x_536_, lean_object* v_i_537_, lean_object* v_a_538_, lean_object* v_a_539_){
_start:
{
lean_object* v_res_540_; 
v_res_540_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(v_x_536_, v_i_537_, v_a_538_);
lean_dec(v_a_538_);
return v_res_540_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock(lean_object* v_sb_541_, lean_object* v_si_542_, lean_object* v_x_543_, lean_object* v_i_544_, lean_object* v_a_545_){
_start:
{
lean_object* v___x_547_; 
v___x_547_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(v_x_543_, v_i_544_, v_a_545_);
return v___x_547_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___boxed(lean_object* v_sb_548_, lean_object* v_si_549_, lean_object* v_x_550_, lean_object* v_i_551_, lean_object* v_a_552_, lean_object* v_a_553_){
_start:
{
lean_object* v_res_554_; 
v_res_554_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock(v_sb_548_, v_si_549_, v_x_550_, v_i_551_, v_a_552_);
lean_dec(v_a_552_);
lean_dec(v_si_549_);
lean_dec_ref(v_sb_548_);
return v_res_554_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___redArg(lean_object* v_x_555_, lean_object* v_i_556_, lean_object* v_a_557_){
_start:
{
lean_object* v___x_559_; 
lean_inc(v_i_556_);
v___x_559_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_i_556_);
if (lean_obj_tag(v___x_559_) == 0)
{
lean_object* v_a_560_; lean_object* v_handle_561_; lean_object* v_model_562_; lean_object* v___x_564_; uint8_t v_isShared_565_; uint8_t v_isSharedCheck_588_; 
v_a_560_ = lean_ctor_get(v___x_559_, 0);
lean_inc(v_a_560_);
lean_dec_ref_known(v___x_559_, 1);
v_handle_561_ = lean_ctor_get(v_x_555_, 0);
v_model_562_ = lean_ctor_get(v_x_555_, 1);
v_isSharedCheck_588_ = !lean_is_exclusive(v_x_555_);
if (v_isSharedCheck_588_ == 0)
{
v___x_564_ = v_x_555_;
v_isShared_565_ = v_isSharedCheck_588_;
goto v_resetjp_563_;
}
else
{
lean_inc(v_model_562_);
lean_inc(v_handle_561_);
lean_dec(v_x_555_);
v___x_564_ = lean_box(0);
v_isShared_565_ = v_isSharedCheck_588_;
goto v_resetjp_563_;
}
v_resetjp_563_:
{
uint8_t v___x_566_; lean_object* v___x_567_; 
v___x_566_ = lean_unbox(v_a_560_);
lean_dec(v_a_560_);
v___x_567_ = zhc_lean_builder_plaintext_integer_get_block(v_a_557_, v_handle_561_, v___x_566_);
lean_dec(v_handle_561_);
if (lean_obj_tag(v___x_567_) == 0)
{
lean_object* v_a_568_; lean_object* v___x_570_; uint8_t v_isShared_571_; uint8_t v_isSharedCheck_579_; 
v_a_568_ = lean_ctor_get(v___x_567_, 0);
v_isSharedCheck_579_ = !lean_is_exclusive(v___x_567_);
if (v_isSharedCheck_579_ == 0)
{
v___x_570_ = v___x_567_;
v_isShared_571_ = v_isSharedCheck_579_;
goto v_resetjp_569_;
}
else
{
lean_inc(v_a_568_);
lean_dec(v___x_567_);
v___x_570_ = lean_box(0);
v_isShared_571_ = v_isSharedCheck_579_;
goto v_resetjp_569_;
}
v_resetjp_569_:
{
lean_object* v___x_572_; lean_object* v___x_574_; 
v___x_572_ = l_List_get___redArg(v_model_562_, v_i_556_);
lean_dec(v_model_562_);
if (v_isShared_565_ == 0)
{
lean_ctor_set(v___x_564_, 1, v___x_572_);
lean_ctor_set(v___x_564_, 0, v_a_568_);
v___x_574_ = v___x_564_;
goto v_reusejp_573_;
}
else
{
lean_object* v_reuseFailAlloc_578_; 
v_reuseFailAlloc_578_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_578_, 0, v_a_568_);
lean_ctor_set(v_reuseFailAlloc_578_, 1, v___x_572_);
v___x_574_ = v_reuseFailAlloc_578_;
goto v_reusejp_573_;
}
v_reusejp_573_:
{
lean_object* v___x_576_; 
if (v_isShared_571_ == 0)
{
lean_ctor_set(v___x_570_, 0, v___x_574_);
v___x_576_ = v___x_570_;
goto v_reusejp_575_;
}
else
{
lean_object* v_reuseFailAlloc_577_; 
v_reuseFailAlloc_577_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_577_, 0, v___x_574_);
v___x_576_ = v_reuseFailAlloc_577_;
goto v_reusejp_575_;
}
v_reusejp_575_:
{
return v___x_576_;
}
}
}
}
else
{
lean_object* v_a_580_; lean_object* v___x_582_; uint8_t v_isShared_583_; uint8_t v_isSharedCheck_587_; 
lean_del_object(v___x_564_);
lean_dec(v_model_562_);
lean_dec(v_i_556_);
v_a_580_ = lean_ctor_get(v___x_567_, 0);
v_isSharedCheck_587_ = !lean_is_exclusive(v___x_567_);
if (v_isSharedCheck_587_ == 0)
{
v___x_582_ = v___x_567_;
v_isShared_583_ = v_isSharedCheck_587_;
goto v_resetjp_581_;
}
else
{
lean_inc(v_a_580_);
lean_dec(v___x_567_);
v___x_582_ = lean_box(0);
v_isShared_583_ = v_isSharedCheck_587_;
goto v_resetjp_581_;
}
v_resetjp_581_:
{
lean_object* v___x_585_; 
if (v_isShared_583_ == 0)
{
v___x_585_ = v___x_582_;
goto v_reusejp_584_;
}
else
{
lean_object* v_reuseFailAlloc_586_; 
v_reuseFailAlloc_586_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_586_, 0, v_a_580_);
v___x_585_ = v_reuseFailAlloc_586_;
goto v_reusejp_584_;
}
v_reusejp_584_:
{
return v___x_585_;
}
}
}
}
}
else
{
lean_object* v_a_589_; lean_object* v___x_591_; uint8_t v_isShared_592_; uint8_t v_isSharedCheck_596_; 
lean_dec(v_i_556_);
lean_dec_ref(v_x_555_);
v_a_589_ = lean_ctor_get(v___x_559_, 0);
v_isSharedCheck_596_ = !lean_is_exclusive(v___x_559_);
if (v_isSharedCheck_596_ == 0)
{
v___x_591_ = v___x_559_;
v_isShared_592_ = v_isSharedCheck_596_;
goto v_resetjp_590_;
}
else
{
lean_inc(v_a_589_);
lean_dec(v___x_559_);
v___x_591_ = lean_box(0);
v_isShared_592_ = v_isSharedCheck_596_;
goto v_resetjp_590_;
}
v_resetjp_590_:
{
lean_object* v___x_594_; 
if (v_isShared_592_ == 0)
{
v___x_594_ = v___x_591_;
goto v_reusejp_593_;
}
else
{
lean_object* v_reuseFailAlloc_595_; 
v_reuseFailAlloc_595_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_595_, 0, v_a_589_);
v___x_594_ = v_reuseFailAlloc_595_;
goto v_reusejp_593_;
}
v_reusejp_593_:
{
return v___x_594_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___redArg___boxed(lean_object* v_x_597_, lean_object* v_i_598_, lean_object* v_a_599_, lean_object* v_a_600_){
_start:
{
lean_object* v_res_601_; 
v_res_601_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___redArg(v_x_597_, v_i_598_, v_a_599_);
lean_dec(v_a_599_);
return v_res_601_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock(lean_object* v_sb_602_, lean_object* v_si_603_, lean_object* v_x_604_, lean_object* v_i_605_, lean_object* v_a_606_){
_start:
{
lean_object* v___x_608_; 
v___x_608_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___redArg(v_x_604_, v_i_605_, v_a_606_);
return v___x_608_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___boxed(lean_object* v_sb_609_, lean_object* v_si_610_, lean_object* v_x_611_, lean_object* v_i_612_, lean_object* v_a_613_, lean_object* v_a_614_){
_start:
{
lean_object* v_res_615_; 
v_res_615_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock(v_sb_609_, v_si_610_, v_x_611_, v_i_612_, v_a_613_);
lean_dec(v_a_613_);
lean_dec(v_si_610_);
lean_dec_ref(v_sb_609_);
return v_res_615_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(lean_object* v_x_616_, lean_object* v_i_617_, lean_object* v_b_618_, lean_object* v_a_619_){
_start:
{
lean_object* v___x_621_; 
lean_inc(v_i_617_);
v___x_621_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow8(v_i_617_);
if (lean_obj_tag(v___x_621_) == 0)
{
lean_object* v_a_622_; lean_object* v_handle_623_; lean_object* v_model_624_; lean_object* v_handle_625_; lean_object* v_model_626_; lean_object* v___x_628_; uint8_t v_isShared_629_; uint8_t v_isSharedCheck_652_; 
v_a_622_ = lean_ctor_get(v___x_621_, 0);
lean_inc(v_a_622_);
lean_dec_ref_known(v___x_621_, 1);
v_handle_623_ = lean_ctor_get(v_x_616_, 0);
lean_inc(v_handle_623_);
v_model_624_ = lean_ctor_get(v_x_616_, 1);
lean_inc(v_model_624_);
lean_dec_ref(v_x_616_);
v_handle_625_ = lean_ctor_get(v_b_618_, 0);
v_model_626_ = lean_ctor_get(v_b_618_, 1);
v_isSharedCheck_652_ = !lean_is_exclusive(v_b_618_);
if (v_isSharedCheck_652_ == 0)
{
v___x_628_ = v_b_618_;
v_isShared_629_ = v_isSharedCheck_652_;
goto v_resetjp_627_;
}
else
{
lean_inc(v_model_626_);
lean_inc(v_handle_625_);
lean_dec(v_b_618_);
v___x_628_ = lean_box(0);
v_isShared_629_ = v_isSharedCheck_652_;
goto v_resetjp_627_;
}
v_resetjp_627_:
{
uint8_t v___x_630_; lean_object* v___x_631_; 
v___x_630_ = lean_unbox(v_a_622_);
lean_dec(v_a_622_);
v___x_631_ = zhc_lean_builder_ciphertext_integer_store_block(v_a_619_, v_handle_623_, v___x_630_, v_handle_625_);
lean_dec(v_handle_625_);
lean_dec(v_handle_623_);
if (lean_obj_tag(v___x_631_) == 0)
{
lean_object* v_a_632_; lean_object* v___x_634_; uint8_t v_isShared_635_; uint8_t v_isSharedCheck_643_; 
v_a_632_ = lean_ctor_get(v___x_631_, 0);
v_isSharedCheck_643_ = !lean_is_exclusive(v___x_631_);
if (v_isSharedCheck_643_ == 0)
{
v___x_634_ = v___x_631_;
v_isShared_635_ = v_isSharedCheck_643_;
goto v_resetjp_633_;
}
else
{
lean_inc(v_a_632_);
lean_dec(v___x_631_);
v___x_634_ = lean_box(0);
v_isShared_635_ = v_isSharedCheck_643_;
goto v_resetjp_633_;
}
v_resetjp_633_:
{
lean_object* v___x_636_; lean_object* v___x_638_; 
v___x_636_ = lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(v_model_624_, v_i_617_, v_model_626_);
if (v_isShared_629_ == 0)
{
lean_ctor_set(v___x_628_, 1, v___x_636_);
lean_ctor_set(v___x_628_, 0, v_a_632_);
v___x_638_ = v___x_628_;
goto v_reusejp_637_;
}
else
{
lean_object* v_reuseFailAlloc_642_; 
v_reuseFailAlloc_642_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_642_, 0, v_a_632_);
lean_ctor_set(v_reuseFailAlloc_642_, 1, v___x_636_);
v___x_638_ = v_reuseFailAlloc_642_;
goto v_reusejp_637_;
}
v_reusejp_637_:
{
lean_object* v___x_640_; 
if (v_isShared_635_ == 0)
{
lean_ctor_set(v___x_634_, 0, v___x_638_);
v___x_640_ = v___x_634_;
goto v_reusejp_639_;
}
else
{
lean_object* v_reuseFailAlloc_641_; 
v_reuseFailAlloc_641_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_641_, 0, v___x_638_);
v___x_640_ = v_reuseFailAlloc_641_;
goto v_reusejp_639_;
}
v_reusejp_639_:
{
return v___x_640_;
}
}
}
}
else
{
lean_object* v_a_644_; lean_object* v___x_646_; uint8_t v_isShared_647_; uint8_t v_isSharedCheck_651_; 
lean_del_object(v___x_628_);
lean_dec(v_model_626_);
lean_dec(v_model_624_);
lean_dec(v_i_617_);
v_a_644_ = lean_ctor_get(v___x_631_, 0);
v_isSharedCheck_651_ = !lean_is_exclusive(v___x_631_);
if (v_isSharedCheck_651_ == 0)
{
v___x_646_ = v___x_631_;
v_isShared_647_ = v_isSharedCheck_651_;
goto v_resetjp_645_;
}
else
{
lean_inc(v_a_644_);
lean_dec(v___x_631_);
v___x_646_ = lean_box(0);
v_isShared_647_ = v_isSharedCheck_651_;
goto v_resetjp_645_;
}
v_resetjp_645_:
{
lean_object* v___x_649_; 
if (v_isShared_647_ == 0)
{
v___x_649_ = v___x_646_;
goto v_reusejp_648_;
}
else
{
lean_object* v_reuseFailAlloc_650_; 
v_reuseFailAlloc_650_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_650_, 0, v_a_644_);
v___x_649_ = v_reuseFailAlloc_650_;
goto v_reusejp_648_;
}
v_reusejp_648_:
{
return v___x_649_;
}
}
}
}
}
else
{
lean_object* v_a_653_; lean_object* v___x_655_; uint8_t v_isShared_656_; uint8_t v_isSharedCheck_660_; 
lean_dec_ref(v_b_618_);
lean_dec(v_i_617_);
lean_dec_ref(v_x_616_);
v_a_653_ = lean_ctor_get(v___x_621_, 0);
v_isSharedCheck_660_ = !lean_is_exclusive(v___x_621_);
if (v_isSharedCheck_660_ == 0)
{
v___x_655_ = v___x_621_;
v_isShared_656_ = v_isSharedCheck_660_;
goto v_resetjp_654_;
}
else
{
lean_inc(v_a_653_);
lean_dec(v___x_621_);
v___x_655_ = lean_box(0);
v_isShared_656_ = v_isSharedCheck_660_;
goto v_resetjp_654_;
}
v_resetjp_654_:
{
lean_object* v___x_658_; 
if (v_isShared_656_ == 0)
{
v___x_658_ = v___x_655_;
goto v_reusejp_657_;
}
else
{
lean_object* v_reuseFailAlloc_659_; 
v_reuseFailAlloc_659_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_659_, 0, v_a_653_);
v___x_658_ = v_reuseFailAlloc_659_;
goto v_reusejp_657_;
}
v_reusejp_657_:
{
return v___x_658_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg___boxed(lean_object* v_x_661_, lean_object* v_i_662_, lean_object* v_b_663_, lean_object* v_a_664_, lean_object* v_a_665_){
_start:
{
lean_object* v_res_666_; 
v_res_666_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(v_x_661_, v_i_662_, v_b_663_, v_a_664_);
lean_dec(v_a_664_);
return v_res_666_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock(lean_object* v_sb_667_, lean_object* v_si_668_, lean_object* v_x_669_, lean_object* v_i_670_, lean_object* v_b_671_, lean_object* v_a_672_){
_start:
{
lean_object* v___x_674_; 
v___x_674_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(v_x_669_, v_i_670_, v_b_671_, v_a_672_);
return v___x_674_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___boxed(lean_object* v_sb_675_, lean_object* v_si_676_, lean_object* v_x_677_, lean_object* v_i_678_, lean_object* v_b_679_, lean_object* v_a_680_, lean_object* v_a_681_){
_start:
{
lean_object* v_res_682_; 
v_res_682_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock(v_sb_675_, v_si_676_, v_x_677_, v_i_678_, v_b_679_, v_a_680_);
lean_dec(v_a_680_);
lean_dec(v_si_676_);
lean_dec_ref(v_sb_675_);
return v_res_682_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___redArg(lean_object* v_b_683_, lean_object* v_a_684_){
_start:
{
lean_object* v_handle_686_; lean_object* v_model_687_; lean_object* v___x_689_; uint8_t v_isShared_690_; uint8_t v_isSharedCheck_711_; 
v_handle_686_ = lean_ctor_get(v_b_683_, 0);
v_model_687_ = lean_ctor_get(v_b_683_, 1);
v_isSharedCheck_711_ = !lean_is_exclusive(v_b_683_);
if (v_isSharedCheck_711_ == 0)
{
v___x_689_ = v_b_683_;
v_isShared_690_ = v_isSharedCheck_711_;
goto v_resetjp_688_;
}
else
{
lean_inc(v_model_687_);
lean_inc(v_handle_686_);
lean_dec(v_b_683_);
v___x_689_ = lean_box(0);
v_isShared_690_ = v_isSharedCheck_711_;
goto v_resetjp_688_;
}
v_resetjp_688_:
{
lean_object* v___x_691_; 
v___x_691_ = zhc_lean_builder_ciphertext_bool_store_block(v_a_684_, v_handle_686_);
lean_dec(v_handle_686_);
if (lean_obj_tag(v___x_691_) == 0)
{
lean_object* v_a_692_; lean_object* v___x_694_; uint8_t v_isShared_695_; uint8_t v_isSharedCheck_702_; 
v_a_692_ = lean_ctor_get(v___x_691_, 0);
v_isSharedCheck_702_ = !lean_is_exclusive(v___x_691_);
if (v_isSharedCheck_702_ == 0)
{
v___x_694_ = v___x_691_;
v_isShared_695_ = v_isSharedCheck_702_;
goto v_resetjp_693_;
}
else
{
lean_inc(v_a_692_);
lean_dec(v___x_691_);
v___x_694_ = lean_box(0);
v_isShared_695_ = v_isSharedCheck_702_;
goto v_resetjp_693_;
}
v_resetjp_693_:
{
lean_object* v___x_697_; 
if (v_isShared_690_ == 0)
{
lean_ctor_set(v___x_689_, 0, v_a_692_);
v___x_697_ = v___x_689_;
goto v_reusejp_696_;
}
else
{
lean_object* v_reuseFailAlloc_701_; 
v_reuseFailAlloc_701_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_701_, 0, v_a_692_);
lean_ctor_set(v_reuseFailAlloc_701_, 1, v_model_687_);
v___x_697_ = v_reuseFailAlloc_701_;
goto v_reusejp_696_;
}
v_reusejp_696_:
{
lean_object* v___x_699_; 
if (v_isShared_695_ == 0)
{
lean_ctor_set(v___x_694_, 0, v___x_697_);
v___x_699_ = v___x_694_;
goto v_reusejp_698_;
}
else
{
lean_object* v_reuseFailAlloc_700_; 
v_reuseFailAlloc_700_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_700_, 0, v___x_697_);
v___x_699_ = v_reuseFailAlloc_700_;
goto v_reusejp_698_;
}
v_reusejp_698_:
{
return v___x_699_;
}
}
}
}
else
{
lean_object* v_a_703_; lean_object* v___x_705_; uint8_t v_isShared_706_; uint8_t v_isSharedCheck_710_; 
lean_del_object(v___x_689_);
lean_dec(v_model_687_);
v_a_703_ = lean_ctor_get(v___x_691_, 0);
v_isSharedCheck_710_ = !lean_is_exclusive(v___x_691_);
if (v_isSharedCheck_710_ == 0)
{
v___x_705_ = v___x_691_;
v_isShared_706_ = v_isSharedCheck_710_;
goto v_resetjp_704_;
}
else
{
lean_inc(v_a_703_);
lean_dec(v___x_691_);
v___x_705_ = lean_box(0);
v_isShared_706_ = v_isSharedCheck_710_;
goto v_resetjp_704_;
}
v_resetjp_704_:
{
lean_object* v___x_708_; 
if (v_isShared_706_ == 0)
{
v___x_708_ = v___x_705_;
goto v_reusejp_707_;
}
else
{
lean_object* v_reuseFailAlloc_709_; 
v_reuseFailAlloc_709_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_709_, 0, v_a_703_);
v___x_708_ = v_reuseFailAlloc_709_;
goto v_reusejp_707_;
}
v_reusejp_707_:
{
return v___x_708_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___redArg___boxed(lean_object* v_b_712_, lean_object* v_a_713_, lean_object* v_a_714_){
_start:
{
lean_object* v_res_715_; 
v_res_715_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___redArg(v_b_712_, v_a_713_);
lean_dec(v_a_713_);
return v_res_715_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock(lean_object* v_sb_716_, lean_object* v_b_717_, lean_object* v_a_718_){
_start:
{
lean_object* v___x_720_; 
v___x_720_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___redArg(v_b_717_, v_a_718_);
return v___x_720_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___boxed(lean_object* v_sb_721_, lean_object* v_b_722_, lean_object* v_a_723_, lean_object* v_a_724_){
_start:
{
lean_object* v_res_725_; 
v_res_725_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock(v_sb_721_, v_b_722_, v_a_723_);
lean_dec(v_a_723_);
lean_dec_ref(v_sb_721_);
return v_res_725_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___redArg(lean_object* v_b_726_, lean_object* v_a_727_){
_start:
{
lean_object* v_handle_729_; lean_object* v_model_730_; lean_object* v___x_732_; uint8_t v_isShared_733_; uint8_t v_isSharedCheck_754_; 
v_handle_729_ = lean_ctor_get(v_b_726_, 0);
v_model_730_ = lean_ctor_get(v_b_726_, 1);
v_isSharedCheck_754_ = !lean_is_exclusive(v_b_726_);
if (v_isSharedCheck_754_ == 0)
{
v___x_732_ = v_b_726_;
v_isShared_733_ = v_isSharedCheck_754_;
goto v_resetjp_731_;
}
else
{
lean_inc(v_model_730_);
lean_inc(v_handle_729_);
lean_dec(v_b_726_);
v___x_732_ = lean_box(0);
v_isShared_733_ = v_isSharedCheck_754_;
goto v_resetjp_731_;
}
v_resetjp_731_:
{
lean_object* v___x_734_; 
v___x_734_ = zhc_lean_builder_ciphertext_bool_get_block(v_a_727_, v_handle_729_);
lean_dec(v_handle_729_);
if (lean_obj_tag(v___x_734_) == 0)
{
lean_object* v_a_735_; lean_object* v___x_737_; uint8_t v_isShared_738_; uint8_t v_isSharedCheck_745_; 
v_a_735_ = lean_ctor_get(v___x_734_, 0);
v_isSharedCheck_745_ = !lean_is_exclusive(v___x_734_);
if (v_isSharedCheck_745_ == 0)
{
v___x_737_ = v___x_734_;
v_isShared_738_ = v_isSharedCheck_745_;
goto v_resetjp_736_;
}
else
{
lean_inc(v_a_735_);
lean_dec(v___x_734_);
v___x_737_ = lean_box(0);
v_isShared_738_ = v_isSharedCheck_745_;
goto v_resetjp_736_;
}
v_resetjp_736_:
{
lean_object* v___x_740_; 
if (v_isShared_733_ == 0)
{
lean_ctor_set(v___x_732_, 0, v_a_735_);
v___x_740_ = v___x_732_;
goto v_reusejp_739_;
}
else
{
lean_object* v_reuseFailAlloc_744_; 
v_reuseFailAlloc_744_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_744_, 0, v_a_735_);
lean_ctor_set(v_reuseFailAlloc_744_, 1, v_model_730_);
v___x_740_ = v_reuseFailAlloc_744_;
goto v_reusejp_739_;
}
v_reusejp_739_:
{
lean_object* v___x_742_; 
if (v_isShared_738_ == 0)
{
lean_ctor_set(v___x_737_, 0, v___x_740_);
v___x_742_ = v___x_737_;
goto v_reusejp_741_;
}
else
{
lean_object* v_reuseFailAlloc_743_; 
v_reuseFailAlloc_743_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_743_, 0, v___x_740_);
v___x_742_ = v_reuseFailAlloc_743_;
goto v_reusejp_741_;
}
v_reusejp_741_:
{
return v___x_742_;
}
}
}
}
else
{
lean_object* v_a_746_; lean_object* v___x_748_; uint8_t v_isShared_749_; uint8_t v_isSharedCheck_753_; 
lean_del_object(v___x_732_);
lean_dec(v_model_730_);
v_a_746_ = lean_ctor_get(v___x_734_, 0);
v_isSharedCheck_753_ = !lean_is_exclusive(v___x_734_);
if (v_isSharedCheck_753_ == 0)
{
v___x_748_ = v___x_734_;
v_isShared_749_ = v_isSharedCheck_753_;
goto v_resetjp_747_;
}
else
{
lean_inc(v_a_746_);
lean_dec(v___x_734_);
v___x_748_ = lean_box(0);
v_isShared_749_ = v_isSharedCheck_753_;
goto v_resetjp_747_;
}
v_resetjp_747_:
{
lean_object* v___x_751_; 
if (v_isShared_749_ == 0)
{
v___x_751_ = v___x_748_;
goto v_reusejp_750_;
}
else
{
lean_object* v_reuseFailAlloc_752_; 
v_reuseFailAlloc_752_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_752_, 0, v_a_746_);
v___x_751_ = v_reuseFailAlloc_752_;
goto v_reusejp_750_;
}
v_reusejp_750_:
{
return v___x_751_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___redArg___boxed(lean_object* v_b_755_, lean_object* v_a_756_, lean_object* v_a_757_){
_start:
{
lean_object* v_res_758_; 
v_res_758_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___redArg(v_b_755_, v_a_756_);
lean_dec(v_a_756_);
return v_res_758_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock(lean_object* v_sb_759_, lean_object* v_b_760_, lean_object* v_a_761_){
_start:
{
lean_object* v___x_763_; 
v___x_763_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___redArg(v_b_760_, v_a_761_);
return v___x_763_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___boxed(lean_object* v_sb_764_, lean_object* v_b_765_, lean_object* v_a_766_, lean_object* v_a_767_){
_start:
{
lean_object* v_res_768_; 
v_res_768_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock(v_sb_764_, v_b_765_, v_a_766_);
lean_dec(v_a_766_);
lean_dec_ref(v_sb_764_);
return v_res_768_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2(lean_object* v_r_772_){
_start:
{
lean_object* v___x_774_; lean_object* v___x_775_; uint8_t v___x_776_; 
v___x_774_ = lean_array_get_size(v_r_772_);
v___x_775_ = lean_unsigned_to_nat(2u);
v___x_776_ = lean_nat_dec_eq(v___x_774_, v___x_775_);
if (v___x_776_ == 0)
{
lean_object* v___x_777_; lean_object* v___x_778_; 
v___x_777_ = ((lean_object*)(lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___closed__1));
v___x_778_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_778_, 0, v___x_777_);
return v___x_778_;
}
else
{
lean_object* v___x_779_; lean_object* v___x_780_; lean_object* v___x_781_; lean_object* v___x_782_; lean_object* v___x_783_; lean_object* v___x_784_; 
v___x_779_ = lean_unsigned_to_nat(0u);
v___x_780_ = lean_array_fget_borrowed(v_r_772_, v___x_779_);
v___x_781_ = lean_unsigned_to_nat(1u);
v___x_782_ = lean_array_fget_borrowed(v_r_772_, v___x_781_);
lean_inc(v___x_782_);
lean_inc(v___x_780_);
v___x_783_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_783_, 0, v___x_780_);
lean_ctor_set(v___x_783_, 1, v___x_782_);
v___x_784_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_784_, 0, v___x_783_);
return v___x_784_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2___boxed(lean_object* v_r_785_, lean_object* v_a_786_){
_start:
{
lean_object* v_res_787_; 
v_res_787_ = lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2(v_r_785_);
lean_dec_ref(v_r_785_);
return v_res_787_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4(lean_object* v_r_791_){
_start:
{
lean_object* v___x_793_; lean_object* v___x_794_; uint8_t v___x_795_; 
v___x_793_ = lean_array_get_size(v_r_791_);
v___x_794_ = lean_unsigned_to_nat(4u);
v___x_795_ = lean_nat_dec_eq(v___x_793_, v___x_794_);
if (v___x_795_ == 0)
{
lean_object* v___x_796_; lean_object* v___x_797_; 
v___x_796_ = ((lean_object*)(lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___closed__1));
v___x_797_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_797_, 0, v___x_796_);
return v___x_797_;
}
else
{
lean_object* v___x_798_; lean_object* v___x_799_; lean_object* v___x_800_; lean_object* v___x_801_; lean_object* v___x_802_; lean_object* v___x_803_; lean_object* v___x_804_; lean_object* v___x_805_; lean_object* v___x_806_; lean_object* v___x_807_; lean_object* v___x_808_; lean_object* v___x_809_; 
v___x_798_ = lean_unsigned_to_nat(0u);
v___x_799_ = lean_array_fget_borrowed(v_r_791_, v___x_798_);
v___x_800_ = lean_unsigned_to_nat(1u);
v___x_801_ = lean_array_fget_borrowed(v_r_791_, v___x_800_);
v___x_802_ = lean_unsigned_to_nat(2u);
v___x_803_ = lean_array_fget_borrowed(v_r_791_, v___x_802_);
v___x_804_ = lean_unsigned_to_nat(3u);
v___x_805_ = lean_array_fget_borrowed(v_r_791_, v___x_804_);
lean_inc(v___x_805_);
lean_inc(v___x_803_);
v___x_806_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_806_, 0, v___x_803_);
lean_ctor_set(v___x_806_, 1, v___x_805_);
lean_inc(v___x_801_);
v___x_807_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_807_, 0, v___x_801_);
lean_ctor_set(v___x_807_, 1, v___x_806_);
lean_inc(v___x_799_);
v___x_808_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_808_, 0, v___x_799_);
lean_ctor_set(v___x_808_, 1, v___x_807_);
v___x_809_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_809_, 0, v___x_808_);
return v___x_809_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4___boxed(lean_object* v_r_810_, lean_object* v_a_811_){
_start:
{
lean_object* v_res_812_; 
v_res_812_ = lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4(v_r_810_);
lean_dec_ref(v_r_810_);
return v_res_812_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8(lean_object* v_r_816_){
_start:
{
lean_object* v___x_818_; lean_object* v___x_819_; uint8_t v___x_820_; 
v___x_818_ = lean_array_get_size(v_r_816_);
v___x_819_ = lean_unsigned_to_nat(8u);
v___x_820_ = lean_nat_dec_eq(v___x_818_, v___x_819_);
if (v___x_820_ == 0)
{
lean_object* v___x_821_; lean_object* v___x_822_; 
v___x_821_ = ((lean_object*)(lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___closed__1));
v___x_822_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_822_, 0, v___x_821_);
return v___x_822_;
}
else
{
lean_object* v___x_823_; lean_object* v___x_824_; lean_object* v___x_825_; lean_object* v___x_826_; lean_object* v___x_827_; lean_object* v___x_828_; lean_object* v___x_829_; lean_object* v___x_830_; lean_object* v___x_831_; lean_object* v___x_832_; lean_object* v___x_833_; lean_object* v___x_834_; lean_object* v___x_835_; lean_object* v___x_836_; lean_object* v___x_837_; lean_object* v___x_838_; lean_object* v___x_839_; lean_object* v___x_840_; lean_object* v___x_841_; lean_object* v___x_842_; lean_object* v___x_843_; lean_object* v___x_844_; lean_object* v___x_845_; lean_object* v___x_846_; 
v___x_823_ = lean_unsigned_to_nat(0u);
v___x_824_ = lean_array_fget_borrowed(v_r_816_, v___x_823_);
v___x_825_ = lean_unsigned_to_nat(1u);
v___x_826_ = lean_array_fget_borrowed(v_r_816_, v___x_825_);
v___x_827_ = lean_unsigned_to_nat(2u);
v___x_828_ = lean_array_fget_borrowed(v_r_816_, v___x_827_);
v___x_829_ = lean_unsigned_to_nat(3u);
v___x_830_ = lean_array_fget_borrowed(v_r_816_, v___x_829_);
v___x_831_ = lean_unsigned_to_nat(4u);
v___x_832_ = lean_array_fget_borrowed(v_r_816_, v___x_831_);
v___x_833_ = lean_unsigned_to_nat(5u);
v___x_834_ = lean_array_fget_borrowed(v_r_816_, v___x_833_);
v___x_835_ = lean_unsigned_to_nat(6u);
v___x_836_ = lean_array_fget_borrowed(v_r_816_, v___x_835_);
v___x_837_ = lean_unsigned_to_nat(7u);
v___x_838_ = lean_array_fget_borrowed(v_r_816_, v___x_837_);
lean_inc(v___x_838_);
lean_inc(v___x_836_);
v___x_839_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_839_, 0, v___x_836_);
lean_ctor_set(v___x_839_, 1, v___x_838_);
lean_inc(v___x_834_);
v___x_840_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_840_, 0, v___x_834_);
lean_ctor_set(v___x_840_, 1, v___x_839_);
lean_inc(v___x_832_);
v___x_841_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_841_, 0, v___x_832_);
lean_ctor_set(v___x_841_, 1, v___x_840_);
lean_inc(v___x_830_);
v___x_842_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_842_, 0, v___x_830_);
lean_ctor_set(v___x_842_, 1, v___x_841_);
lean_inc(v___x_828_);
v___x_843_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_843_, 0, v___x_828_);
lean_ctor_set(v___x_843_, 1, v___x_842_);
lean_inc(v___x_826_);
v___x_844_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_844_, 0, v___x_826_);
lean_ctor_set(v___x_844_, 1, v___x_843_);
lean_inc(v___x_824_);
v___x_845_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_845_, 0, v___x_824_);
lean_ctor_set(v___x_845_, 1, v___x_844_);
v___x_846_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_846_, 0, v___x_845_);
return v___x_846_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8___boxed(lean_object* v_r_847_, lean_object* v_a_848_){
_start:
{
lean_object* v_res_849_; 
v_res_849_ = lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8(v_r_847_);
lean_dec_ref(v_r_847_);
return v_res_849_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___closed__0(void){
_start:
{
lean_object* v___x_850_; lean_object* v___x_851_; lean_object* v___x_852_; 
v___x_850_ = lean_unsigned_to_nat(1u);
v___x_851_ = lean_unsigned_to_nat(0u);
v___x_852_ = lean_nat_mod(v___x_851_, v___x_850_);
return v___x_852_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(lean_object* v_sb_853_, lean_object* v_lut_854_, lean_object* v_a_855_, lean_object* v_a_856_){
_start:
{
lean_object* v___x_858_; 
lean_inc_ref(v_lut_854_);
v___x_858_ = lp_zhc__lean_Zhc_Ffi_L2r_Lut1_ofModel(v_sb_853_, v_lut_854_);
if (lean_obj_tag(v___x_858_) == 0)
{
lean_object* v_a_859_; lean_object* v_handle_860_; lean_object* v_model_861_; lean_object* v___x_863_; uint8_t v_isShared_864_; uint8_t v_isSharedCheck_888_; 
v_a_859_ = lean_ctor_get(v___x_858_, 0);
lean_inc(v_a_859_);
lean_dec_ref_known(v___x_858_, 1);
v_handle_860_ = lean_ctor_get(v_a_855_, 0);
v_model_861_ = lean_ctor_get(v_a_855_, 1);
v_isSharedCheck_888_ = !lean_is_exclusive(v_a_855_);
if (v_isSharedCheck_888_ == 0)
{
v___x_863_ = v_a_855_;
v_isShared_864_ = v_isSharedCheck_888_;
goto v_resetjp_862_;
}
else
{
lean_inc(v_model_861_);
lean_inc(v_handle_860_);
lean_dec(v_a_855_);
v___x_863_ = lean_box(0);
v_isShared_864_ = v_isSharedCheck_888_;
goto v_resetjp_862_;
}
v_resetjp_862_:
{
lean_object* v___x_865_; 
v___x_865_ = zhc_lean_builder_block_lookup(v_a_856_, v_handle_860_, v_a_859_);
lean_dec(v_a_859_);
lean_dec(v_handle_860_);
if (lean_obj_tag(v___x_865_) == 0)
{
lean_object* v_a_866_; lean_object* v___x_868_; uint8_t v_isShared_869_; uint8_t v_isSharedCheck_879_; 
v_a_866_ = lean_ctor_get(v___x_865_, 0);
v_isSharedCheck_879_ = !lean_is_exclusive(v___x_865_);
if (v_isSharedCheck_879_ == 0)
{
v___x_868_ = v___x_865_;
v_isShared_869_ = v_isSharedCheck_879_;
goto v_resetjp_867_;
}
else
{
lean_inc(v_a_866_);
lean_dec(v___x_865_);
v___x_868_ = lean_box(0);
v_isShared_869_ = v_isSharedCheck_879_;
goto v_resetjp_867_;
}
v_resetjp_867_:
{
lean_object* v___x_870_; lean_object* v___x_871_; lean_object* v___x_872_; lean_object* v___x_874_; 
v___x_870_ = lean_unsigned_to_nat(0u);
v___x_871_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___closed__0);
v___x_872_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_853_, v___x_870_, v_lut_854_, v_model_861_, v___x_871_);
lean_dec(v_model_861_);
if (v_isShared_864_ == 0)
{
lean_ctor_set(v___x_863_, 1, v___x_872_);
lean_ctor_set(v___x_863_, 0, v_a_866_);
v___x_874_ = v___x_863_;
goto v_reusejp_873_;
}
else
{
lean_object* v_reuseFailAlloc_878_; 
v_reuseFailAlloc_878_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_878_, 0, v_a_866_);
lean_ctor_set(v_reuseFailAlloc_878_, 1, v___x_872_);
v___x_874_ = v_reuseFailAlloc_878_;
goto v_reusejp_873_;
}
v_reusejp_873_:
{
lean_object* v___x_876_; 
if (v_isShared_869_ == 0)
{
lean_ctor_set(v___x_868_, 0, v___x_874_);
v___x_876_ = v___x_868_;
goto v_reusejp_875_;
}
else
{
lean_object* v_reuseFailAlloc_877_; 
v_reuseFailAlloc_877_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_877_, 0, v___x_874_);
v___x_876_ = v_reuseFailAlloc_877_;
goto v_reusejp_875_;
}
v_reusejp_875_:
{
return v___x_876_;
}
}
}
}
else
{
lean_object* v_a_880_; lean_object* v___x_882_; uint8_t v_isShared_883_; uint8_t v_isSharedCheck_887_; 
lean_del_object(v___x_863_);
lean_dec(v_model_861_);
lean_dec_ref(v_lut_854_);
v_a_880_ = lean_ctor_get(v___x_865_, 0);
v_isSharedCheck_887_ = !lean_is_exclusive(v___x_865_);
if (v_isSharedCheck_887_ == 0)
{
v___x_882_ = v___x_865_;
v_isShared_883_ = v_isSharedCheck_887_;
goto v_resetjp_881_;
}
else
{
lean_inc(v_a_880_);
lean_dec(v___x_865_);
v___x_882_ = lean_box(0);
v_isShared_883_ = v_isSharedCheck_887_;
goto v_resetjp_881_;
}
v_resetjp_881_:
{
lean_object* v___x_885_; 
if (v_isShared_883_ == 0)
{
v___x_885_ = v___x_882_;
goto v_reusejp_884_;
}
else
{
lean_object* v_reuseFailAlloc_886_; 
v_reuseFailAlloc_886_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_886_, 0, v_a_880_);
v___x_885_ = v_reuseFailAlloc_886_;
goto v_reusejp_884_;
}
v_reusejp_884_:
{
return v___x_885_;
}
}
}
}
}
else
{
lean_object* v_a_889_; lean_object* v___x_891_; uint8_t v_isShared_892_; uint8_t v_isSharedCheck_896_; 
lean_dec_ref(v_a_855_);
lean_dec_ref(v_lut_854_);
v_a_889_ = lean_ctor_get(v___x_858_, 0);
v_isSharedCheck_896_ = !lean_is_exclusive(v___x_858_);
if (v_isSharedCheck_896_ == 0)
{
v___x_891_ = v___x_858_;
v_isShared_892_ = v_isSharedCheck_896_;
goto v_resetjp_890_;
}
else
{
lean_inc(v_a_889_);
lean_dec(v___x_858_);
v___x_891_ = lean_box(0);
v_isShared_892_ = v_isSharedCheck_896_;
goto v_resetjp_890_;
}
v_resetjp_890_:
{
lean_object* v___x_894_; 
if (v_isShared_892_ == 0)
{
v___x_894_ = v___x_891_;
goto v_reusejp_893_;
}
else
{
lean_object* v_reuseFailAlloc_895_; 
v_reuseFailAlloc_895_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_895_, 0, v_a_889_);
v___x_894_ = v_reuseFailAlloc_895_;
goto v_reusejp_893_;
}
v_reusejp_893_:
{
return v___x_894_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___boxed(lean_object* v_sb_897_, lean_object* v_lut_898_, lean_object* v_a_899_, lean_object* v_a_900_, lean_object* v_a_901_){
_start:
{
lean_object* v_res_902_; 
v_res_902_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v_sb_897_, v_lut_898_, v_a_899_, v_a_900_);
lean_dec(v_a_900_);
lean_dec_ref(v_sb_897_);
return v_res_902_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__0(void){
_start:
{
lean_object* v___x_903_; lean_object* v___x_904_; lean_object* v___x_905_; 
v___x_903_ = lean_unsigned_to_nat(2u);
v___x_904_ = lean_unsigned_to_nat(0u);
v___x_905_ = lean_nat_mod(v___x_904_, v___x_903_);
return v___x_905_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__1(void){
_start:
{
lean_object* v___x_906_; lean_object* v___x_907_; lean_object* v___x_908_; 
v___x_906_ = lean_unsigned_to_nat(2u);
v___x_907_ = lean_unsigned_to_nat(1u);
v___x_908_ = lean_nat_mod(v___x_907_, v___x_906_);
return v___x_908_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(lean_object* v_sb_909_, lean_object* v_lut_910_, lean_object* v_a_911_, lean_object* v_a_912_){
_start:
{
lean_object* v___x_914_; 
lean_inc_ref(v_lut_910_);
v___x_914_ = lp_zhc__lean_Zhc_Ffi_L2r_Lut2_ofModel(v_sb_909_, v_lut_910_);
if (lean_obj_tag(v___x_914_) == 0)
{
lean_object* v_a_915_; lean_object* v_handle_916_; lean_object* v_model_917_; lean_object* v___x_919_; uint8_t v_isShared_920_; uint8_t v_isSharedCheck_966_; 
v_a_915_ = lean_ctor_get(v___x_914_, 0);
lean_inc(v_a_915_);
lean_dec_ref_known(v___x_914_, 1);
v_handle_916_ = lean_ctor_get(v_a_911_, 0);
v_model_917_ = lean_ctor_get(v_a_911_, 1);
v_isSharedCheck_966_ = !lean_is_exclusive(v_a_911_);
if (v_isSharedCheck_966_ == 0)
{
v___x_919_ = v_a_911_;
v_isShared_920_ = v_isSharedCheck_966_;
goto v_resetjp_918_;
}
else
{
lean_inc(v_model_917_);
lean_inc(v_handle_916_);
lean_dec(v_a_911_);
v___x_919_ = lean_box(0);
v_isShared_920_ = v_isSharedCheck_966_;
goto v_resetjp_918_;
}
v_resetjp_918_:
{
lean_object* v___x_921_; 
v___x_921_ = zhc_lean_builder_block_lookup2(v_a_912_, v_handle_916_, v_a_915_);
lean_dec(v_a_915_);
lean_dec(v_handle_916_);
if (lean_obj_tag(v___x_921_) == 0)
{
lean_object* v_a_922_; lean_object* v___x_923_; 
v_a_922_ = lean_ctor_get(v___x_921_, 0);
lean_inc(v_a_922_);
lean_dec_ref_known(v___x_921_, 1);
v___x_923_ = lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack2(v_a_922_);
lean_dec(v_a_922_);
if (lean_obj_tag(v___x_923_) == 0)
{
lean_object* v_a_924_; lean_object* v___x_926_; uint8_t v_isShared_927_; uint8_t v_isSharedCheck_949_; 
v_a_924_ = lean_ctor_get(v___x_923_, 0);
v_isSharedCheck_949_ = !lean_is_exclusive(v___x_923_);
if (v_isSharedCheck_949_ == 0)
{
v___x_926_ = v___x_923_;
v_isShared_927_ = v_isSharedCheck_949_;
goto v_resetjp_925_;
}
else
{
lean_inc(v_a_924_);
lean_dec(v___x_923_);
v___x_926_ = lean_box(0);
v_isShared_927_ = v_isSharedCheck_949_;
goto v_resetjp_925_;
}
v_resetjp_925_:
{
lean_object* v_fst_928_; lean_object* v_snd_929_; lean_object* v___x_931_; uint8_t v_isShared_932_; uint8_t v_isSharedCheck_948_; 
v_fst_928_ = lean_ctor_get(v_a_924_, 0);
v_snd_929_ = lean_ctor_get(v_a_924_, 1);
v_isSharedCheck_948_ = !lean_is_exclusive(v_a_924_);
if (v_isSharedCheck_948_ == 0)
{
v___x_931_ = v_a_924_;
v_isShared_932_ = v_isSharedCheck_948_;
goto v_resetjp_930_;
}
else
{
lean_inc(v_snd_929_);
lean_inc(v_fst_928_);
lean_dec(v_a_924_);
v___x_931_ = lean_box(0);
v_isShared_932_ = v_isSharedCheck_948_;
goto v_resetjp_930_;
}
v_resetjp_930_:
{
lean_object* v___x_933_; lean_object* v___x_934_; lean_object* v___x_935_; lean_object* v___x_937_; 
v___x_933_ = lean_unsigned_to_nat(1u);
v___x_934_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__0);
lean_inc_ref(v_lut_910_);
v___x_935_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_909_, v___x_933_, v_lut_910_, v_model_917_, v___x_934_);
if (v_isShared_920_ == 0)
{
lean_ctor_set(v___x_919_, 1, v___x_935_);
lean_ctor_set(v___x_919_, 0, v_fst_928_);
v___x_937_ = v___x_919_;
goto v_reusejp_936_;
}
else
{
lean_object* v_reuseFailAlloc_947_; 
v_reuseFailAlloc_947_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_947_, 0, v_fst_928_);
lean_ctor_set(v_reuseFailAlloc_947_, 1, v___x_935_);
v___x_937_ = v_reuseFailAlloc_947_;
goto v_reusejp_936_;
}
v_reusejp_936_:
{
lean_object* v___x_938_; lean_object* v___x_939_; lean_object* v___x_940_; lean_object* v___x_942_; 
v___x_938_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__1, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__1_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___closed__1);
v___x_939_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_909_, v___x_933_, v_lut_910_, v_model_917_, v___x_938_);
lean_dec(v_model_917_);
v___x_940_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_940_, 0, v_snd_929_);
lean_ctor_set(v___x_940_, 1, v___x_939_);
if (v_isShared_932_ == 0)
{
lean_ctor_set(v___x_931_, 1, v___x_940_);
lean_ctor_set(v___x_931_, 0, v___x_937_);
v___x_942_ = v___x_931_;
goto v_reusejp_941_;
}
else
{
lean_object* v_reuseFailAlloc_946_; 
v_reuseFailAlloc_946_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_946_, 0, v___x_937_);
lean_ctor_set(v_reuseFailAlloc_946_, 1, v___x_940_);
v___x_942_ = v_reuseFailAlloc_946_;
goto v_reusejp_941_;
}
v_reusejp_941_:
{
lean_object* v___x_944_; 
if (v_isShared_927_ == 0)
{
lean_ctor_set(v___x_926_, 0, v___x_942_);
v___x_944_ = v___x_926_;
goto v_reusejp_943_;
}
else
{
lean_object* v_reuseFailAlloc_945_; 
v_reuseFailAlloc_945_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_945_, 0, v___x_942_);
v___x_944_ = v_reuseFailAlloc_945_;
goto v_reusejp_943_;
}
v_reusejp_943_:
{
return v___x_944_;
}
}
}
}
}
}
else
{
lean_object* v_a_950_; lean_object* v___x_952_; uint8_t v_isShared_953_; uint8_t v_isSharedCheck_957_; 
lean_del_object(v___x_919_);
lean_dec(v_model_917_);
lean_dec_ref(v_lut_910_);
v_a_950_ = lean_ctor_get(v___x_923_, 0);
v_isSharedCheck_957_ = !lean_is_exclusive(v___x_923_);
if (v_isSharedCheck_957_ == 0)
{
v___x_952_ = v___x_923_;
v_isShared_953_ = v_isSharedCheck_957_;
goto v_resetjp_951_;
}
else
{
lean_inc(v_a_950_);
lean_dec(v___x_923_);
v___x_952_ = lean_box(0);
v_isShared_953_ = v_isSharedCheck_957_;
goto v_resetjp_951_;
}
v_resetjp_951_:
{
lean_object* v___x_955_; 
if (v_isShared_953_ == 0)
{
v___x_955_ = v___x_952_;
goto v_reusejp_954_;
}
else
{
lean_object* v_reuseFailAlloc_956_; 
v_reuseFailAlloc_956_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_956_, 0, v_a_950_);
v___x_955_ = v_reuseFailAlloc_956_;
goto v_reusejp_954_;
}
v_reusejp_954_:
{
return v___x_955_;
}
}
}
}
else
{
lean_object* v_a_958_; lean_object* v___x_960_; uint8_t v_isShared_961_; uint8_t v_isSharedCheck_965_; 
lean_del_object(v___x_919_);
lean_dec(v_model_917_);
lean_dec_ref(v_lut_910_);
v_a_958_ = lean_ctor_get(v___x_921_, 0);
v_isSharedCheck_965_ = !lean_is_exclusive(v___x_921_);
if (v_isSharedCheck_965_ == 0)
{
v___x_960_ = v___x_921_;
v_isShared_961_ = v_isSharedCheck_965_;
goto v_resetjp_959_;
}
else
{
lean_inc(v_a_958_);
lean_dec(v___x_921_);
v___x_960_ = lean_box(0);
v_isShared_961_ = v_isSharedCheck_965_;
goto v_resetjp_959_;
}
v_resetjp_959_:
{
lean_object* v___x_963_; 
if (v_isShared_961_ == 0)
{
v___x_963_ = v___x_960_;
goto v_reusejp_962_;
}
else
{
lean_object* v_reuseFailAlloc_964_; 
v_reuseFailAlloc_964_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_964_, 0, v_a_958_);
v___x_963_ = v_reuseFailAlloc_964_;
goto v_reusejp_962_;
}
v_reusejp_962_:
{
return v___x_963_;
}
}
}
}
}
else
{
lean_object* v_a_967_; lean_object* v___x_969_; uint8_t v_isShared_970_; uint8_t v_isSharedCheck_974_; 
lean_dec_ref(v_a_911_);
lean_dec_ref(v_lut_910_);
v_a_967_ = lean_ctor_get(v___x_914_, 0);
v_isSharedCheck_974_ = !lean_is_exclusive(v___x_914_);
if (v_isSharedCheck_974_ == 0)
{
v___x_969_ = v___x_914_;
v_isShared_970_ = v_isSharedCheck_974_;
goto v_resetjp_968_;
}
else
{
lean_inc(v_a_967_);
lean_dec(v___x_914_);
v___x_969_ = lean_box(0);
v_isShared_970_ = v_isSharedCheck_974_;
goto v_resetjp_968_;
}
v_resetjp_968_:
{
lean_object* v___x_972_; 
if (v_isShared_970_ == 0)
{
v___x_972_ = v___x_969_;
goto v_reusejp_971_;
}
else
{
lean_object* v_reuseFailAlloc_973_; 
v_reuseFailAlloc_973_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_973_, 0, v_a_967_);
v___x_972_ = v_reuseFailAlloc_973_;
goto v_reusejp_971_;
}
v_reusejp_971_:
{
return v___x_972_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___boxed(lean_object* v_sb_975_, lean_object* v_lut_976_, lean_object* v_a_977_, lean_object* v_a_978_, lean_object* v_a_979_){
_start:
{
lean_object* v_res_980_; 
v_res_980_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(v_sb_975_, v_lut_976_, v_a_977_, v_a_978_);
lean_dec(v_a_978_);
lean_dec_ref(v_sb_975_);
return v_res_980_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__0(void){
_start:
{
lean_object* v___x_981_; lean_object* v___x_982_; lean_object* v___x_983_; 
v___x_981_ = lean_unsigned_to_nat(4u);
v___x_982_ = lean_unsigned_to_nat(0u);
v___x_983_ = lean_nat_mod(v___x_982_, v___x_981_);
return v___x_983_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__1(void){
_start:
{
lean_object* v___x_984_; lean_object* v___x_985_; lean_object* v___x_986_; 
v___x_984_ = lean_unsigned_to_nat(4u);
v___x_985_ = lean_unsigned_to_nat(1u);
v___x_986_ = lean_nat_mod(v___x_985_, v___x_984_);
return v___x_986_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__2(void){
_start:
{
lean_object* v___x_987_; lean_object* v___x_988_; lean_object* v___x_989_; 
v___x_987_ = lean_unsigned_to_nat(4u);
v___x_988_ = lean_unsigned_to_nat(2u);
v___x_989_ = lean_nat_mod(v___x_988_, v___x_987_);
return v___x_989_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__3(void){
_start:
{
lean_object* v___x_990_; lean_object* v___x_991_; lean_object* v___x_992_; 
v___x_990_ = lean_unsigned_to_nat(4u);
v___x_991_ = lean_unsigned_to_nat(3u);
v___x_992_ = lean_nat_mod(v___x_991_, v___x_990_);
return v___x_992_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4(lean_object* v_sb_993_, lean_object* v_lut_994_, lean_object* v_a_995_, lean_object* v_a_996_){
_start:
{
lean_object* v___x_998_; 
lean_inc_ref(v_lut_994_);
v___x_998_ = lp_zhc__lean_Zhc_Ffi_L2r_Lut4_ofModel(v_sb_993_, v_lut_994_);
if (lean_obj_tag(v___x_998_) == 0)
{
lean_object* v_a_999_; lean_object* v_handle_1000_; lean_object* v_model_1001_; lean_object* v___x_1003_; uint8_t v_isShared_1004_; uint8_t v_isSharedCheck_1076_; 
v_a_999_ = lean_ctor_get(v___x_998_, 0);
lean_inc(v_a_999_);
lean_dec_ref_known(v___x_998_, 1);
v_handle_1000_ = lean_ctor_get(v_a_995_, 0);
v_model_1001_ = lean_ctor_get(v_a_995_, 1);
v_isSharedCheck_1076_ = !lean_is_exclusive(v_a_995_);
if (v_isSharedCheck_1076_ == 0)
{
v___x_1003_ = v_a_995_;
v_isShared_1004_ = v_isSharedCheck_1076_;
goto v_resetjp_1002_;
}
else
{
lean_inc(v_model_1001_);
lean_inc(v_handle_1000_);
lean_dec(v_a_995_);
v___x_1003_ = lean_box(0);
v_isShared_1004_ = v_isSharedCheck_1076_;
goto v_resetjp_1002_;
}
v_resetjp_1002_:
{
lean_object* v___x_1005_; 
v___x_1005_ = zhc_lean_builder_block_lookup4(v_a_996_, v_handle_1000_, v_a_999_);
lean_dec(v_a_999_);
lean_dec(v_handle_1000_);
if (lean_obj_tag(v___x_1005_) == 0)
{
lean_object* v_a_1006_; lean_object* v___x_1007_; 
v_a_1006_ = lean_ctor_get(v___x_1005_, 0);
lean_inc(v_a_1006_);
lean_dec_ref_known(v___x_1005_, 1);
v___x_1007_ = lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack4(v_a_1006_);
lean_dec(v_a_1006_);
if (lean_obj_tag(v___x_1007_) == 0)
{
lean_object* v_a_1008_; lean_object* v___x_1010_; uint8_t v_isShared_1011_; uint8_t v_isSharedCheck_1059_; 
v_a_1008_ = lean_ctor_get(v___x_1007_, 0);
v_isSharedCheck_1059_ = !lean_is_exclusive(v___x_1007_);
if (v_isSharedCheck_1059_ == 0)
{
v___x_1010_ = v___x_1007_;
v_isShared_1011_ = v_isSharedCheck_1059_;
goto v_resetjp_1009_;
}
else
{
lean_inc(v_a_1008_);
lean_dec(v___x_1007_);
v___x_1010_ = lean_box(0);
v_isShared_1011_ = v_isSharedCheck_1059_;
goto v_resetjp_1009_;
}
v_resetjp_1009_:
{
lean_object* v_snd_1012_; lean_object* v_snd_1013_; lean_object* v_fst_1014_; lean_object* v___x_1016_; uint8_t v_isShared_1017_; uint8_t v_isSharedCheck_1057_; 
v_snd_1012_ = lean_ctor_get(v_a_1008_, 1);
lean_inc(v_snd_1012_);
v_snd_1013_ = lean_ctor_get(v_snd_1012_, 1);
lean_inc(v_snd_1013_);
v_fst_1014_ = lean_ctor_get(v_a_1008_, 0);
v_isSharedCheck_1057_ = !lean_is_exclusive(v_a_1008_);
if (v_isSharedCheck_1057_ == 0)
{
lean_object* v_unused_1058_; 
v_unused_1058_ = lean_ctor_get(v_a_1008_, 1);
lean_dec(v_unused_1058_);
v___x_1016_ = v_a_1008_;
v_isShared_1017_ = v_isSharedCheck_1057_;
goto v_resetjp_1015_;
}
else
{
lean_inc(v_fst_1014_);
lean_dec(v_a_1008_);
v___x_1016_ = lean_box(0);
v_isShared_1017_ = v_isSharedCheck_1057_;
goto v_resetjp_1015_;
}
v_resetjp_1015_:
{
lean_object* v_fst_1018_; lean_object* v___x_1020_; uint8_t v_isShared_1021_; uint8_t v_isSharedCheck_1055_; 
v_fst_1018_ = lean_ctor_get(v_snd_1012_, 0);
v_isSharedCheck_1055_ = !lean_is_exclusive(v_snd_1012_);
if (v_isSharedCheck_1055_ == 0)
{
lean_object* v_unused_1056_; 
v_unused_1056_ = lean_ctor_get(v_snd_1012_, 1);
lean_dec(v_unused_1056_);
v___x_1020_ = v_snd_1012_;
v_isShared_1021_ = v_isSharedCheck_1055_;
goto v_resetjp_1019_;
}
else
{
lean_inc(v_fst_1018_);
lean_dec(v_snd_1012_);
v___x_1020_ = lean_box(0);
v_isShared_1021_ = v_isSharedCheck_1055_;
goto v_resetjp_1019_;
}
v_resetjp_1019_:
{
lean_object* v_fst_1022_; lean_object* v_snd_1023_; lean_object* v___x_1025_; uint8_t v_isShared_1026_; uint8_t v_isSharedCheck_1054_; 
v_fst_1022_ = lean_ctor_get(v_snd_1013_, 0);
v_snd_1023_ = lean_ctor_get(v_snd_1013_, 1);
v_isSharedCheck_1054_ = !lean_is_exclusive(v_snd_1013_);
if (v_isSharedCheck_1054_ == 0)
{
v___x_1025_ = v_snd_1013_;
v_isShared_1026_ = v_isSharedCheck_1054_;
goto v_resetjp_1024_;
}
else
{
lean_inc(v_snd_1023_);
lean_inc(v_fst_1022_);
lean_dec(v_snd_1013_);
v___x_1025_ = lean_box(0);
v_isShared_1026_ = v_isSharedCheck_1054_;
goto v_resetjp_1024_;
}
v_resetjp_1024_:
{
lean_object* v___x_1027_; lean_object* v___x_1028_; lean_object* v___x_1029_; lean_object* v___x_1031_; 
v___x_1027_ = lean_unsigned_to_nat(2u);
v___x_1028_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__0);
lean_inc_ref(v_lut_994_);
v___x_1029_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_993_, v___x_1027_, v_lut_994_, v_model_1001_, v___x_1028_);
if (v_isShared_1004_ == 0)
{
lean_ctor_set(v___x_1003_, 1, v___x_1029_);
lean_ctor_set(v___x_1003_, 0, v_fst_1014_);
v___x_1031_ = v___x_1003_;
goto v_reusejp_1030_;
}
else
{
lean_object* v_reuseFailAlloc_1053_; 
v_reuseFailAlloc_1053_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1053_, 0, v_fst_1014_);
lean_ctor_set(v_reuseFailAlloc_1053_, 1, v___x_1029_);
v___x_1031_ = v_reuseFailAlloc_1053_;
goto v_reusejp_1030_;
}
v_reusejp_1030_:
{
lean_object* v___x_1032_; lean_object* v___x_1033_; lean_object* v___x_1034_; lean_object* v___x_1035_; lean_object* v___x_1036_; lean_object* v___x_1037_; lean_object* v___x_1038_; lean_object* v___x_1039_; lean_object* v___x_1040_; lean_object* v___x_1042_; 
v___x_1032_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__1, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__1_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__1);
lean_inc_ref_n(v_lut_994_, 2);
v___x_1033_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_993_, v___x_1027_, v_lut_994_, v_model_1001_, v___x_1032_);
v___x_1034_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1034_, 0, v_fst_1018_);
lean_ctor_set(v___x_1034_, 1, v___x_1033_);
v___x_1035_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__2, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__2_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__2);
v___x_1036_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_993_, v___x_1027_, v_lut_994_, v_model_1001_, v___x_1035_);
v___x_1037_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1037_, 0, v_fst_1022_);
lean_ctor_set(v___x_1037_, 1, v___x_1036_);
v___x_1038_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__3, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__3_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___closed__3);
v___x_1039_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_993_, v___x_1027_, v_lut_994_, v_model_1001_, v___x_1038_);
lean_dec(v_model_1001_);
v___x_1040_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1040_, 0, v_snd_1023_);
lean_ctor_set(v___x_1040_, 1, v___x_1039_);
if (v_isShared_1026_ == 0)
{
lean_ctor_set(v___x_1025_, 1, v___x_1040_);
lean_ctor_set(v___x_1025_, 0, v___x_1037_);
v___x_1042_ = v___x_1025_;
goto v_reusejp_1041_;
}
else
{
lean_object* v_reuseFailAlloc_1052_; 
v_reuseFailAlloc_1052_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1052_, 0, v___x_1037_);
lean_ctor_set(v_reuseFailAlloc_1052_, 1, v___x_1040_);
v___x_1042_ = v_reuseFailAlloc_1052_;
goto v_reusejp_1041_;
}
v_reusejp_1041_:
{
lean_object* v___x_1044_; 
if (v_isShared_1021_ == 0)
{
lean_ctor_set(v___x_1020_, 1, v___x_1042_);
lean_ctor_set(v___x_1020_, 0, v___x_1034_);
v___x_1044_ = v___x_1020_;
goto v_reusejp_1043_;
}
else
{
lean_object* v_reuseFailAlloc_1051_; 
v_reuseFailAlloc_1051_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1051_, 0, v___x_1034_);
lean_ctor_set(v_reuseFailAlloc_1051_, 1, v___x_1042_);
v___x_1044_ = v_reuseFailAlloc_1051_;
goto v_reusejp_1043_;
}
v_reusejp_1043_:
{
lean_object* v___x_1046_; 
if (v_isShared_1017_ == 0)
{
lean_ctor_set(v___x_1016_, 1, v___x_1044_);
lean_ctor_set(v___x_1016_, 0, v___x_1031_);
v___x_1046_ = v___x_1016_;
goto v_reusejp_1045_;
}
else
{
lean_object* v_reuseFailAlloc_1050_; 
v_reuseFailAlloc_1050_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1050_, 0, v___x_1031_);
lean_ctor_set(v_reuseFailAlloc_1050_, 1, v___x_1044_);
v___x_1046_ = v_reuseFailAlloc_1050_;
goto v_reusejp_1045_;
}
v_reusejp_1045_:
{
lean_object* v___x_1048_; 
if (v_isShared_1011_ == 0)
{
lean_ctor_set(v___x_1010_, 0, v___x_1046_);
v___x_1048_ = v___x_1010_;
goto v_reusejp_1047_;
}
else
{
lean_object* v_reuseFailAlloc_1049_; 
v_reuseFailAlloc_1049_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1049_, 0, v___x_1046_);
v___x_1048_ = v_reuseFailAlloc_1049_;
goto v_reusejp_1047_;
}
v_reusejp_1047_:
{
return v___x_1048_;
}
}
}
}
}
}
}
}
}
}
else
{
lean_object* v_a_1060_; lean_object* v___x_1062_; uint8_t v_isShared_1063_; uint8_t v_isSharedCheck_1067_; 
lean_del_object(v___x_1003_);
lean_dec(v_model_1001_);
lean_dec_ref(v_lut_994_);
v_a_1060_ = lean_ctor_get(v___x_1007_, 0);
v_isSharedCheck_1067_ = !lean_is_exclusive(v___x_1007_);
if (v_isSharedCheck_1067_ == 0)
{
v___x_1062_ = v___x_1007_;
v_isShared_1063_ = v_isSharedCheck_1067_;
goto v_resetjp_1061_;
}
else
{
lean_inc(v_a_1060_);
lean_dec(v___x_1007_);
v___x_1062_ = lean_box(0);
v_isShared_1063_ = v_isSharedCheck_1067_;
goto v_resetjp_1061_;
}
v_resetjp_1061_:
{
lean_object* v___x_1065_; 
if (v_isShared_1063_ == 0)
{
v___x_1065_ = v___x_1062_;
goto v_reusejp_1064_;
}
else
{
lean_object* v_reuseFailAlloc_1066_; 
v_reuseFailAlloc_1066_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1066_, 0, v_a_1060_);
v___x_1065_ = v_reuseFailAlloc_1066_;
goto v_reusejp_1064_;
}
v_reusejp_1064_:
{
return v___x_1065_;
}
}
}
}
else
{
lean_object* v_a_1068_; lean_object* v___x_1070_; uint8_t v_isShared_1071_; uint8_t v_isSharedCheck_1075_; 
lean_del_object(v___x_1003_);
lean_dec(v_model_1001_);
lean_dec_ref(v_lut_994_);
v_a_1068_ = lean_ctor_get(v___x_1005_, 0);
v_isSharedCheck_1075_ = !lean_is_exclusive(v___x_1005_);
if (v_isSharedCheck_1075_ == 0)
{
v___x_1070_ = v___x_1005_;
v_isShared_1071_ = v_isSharedCheck_1075_;
goto v_resetjp_1069_;
}
else
{
lean_inc(v_a_1068_);
lean_dec(v___x_1005_);
v___x_1070_ = lean_box(0);
v_isShared_1071_ = v_isSharedCheck_1075_;
goto v_resetjp_1069_;
}
v_resetjp_1069_:
{
lean_object* v___x_1073_; 
if (v_isShared_1071_ == 0)
{
v___x_1073_ = v___x_1070_;
goto v_reusejp_1072_;
}
else
{
lean_object* v_reuseFailAlloc_1074_; 
v_reuseFailAlloc_1074_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1074_, 0, v_a_1068_);
v___x_1073_ = v_reuseFailAlloc_1074_;
goto v_reusejp_1072_;
}
v_reusejp_1072_:
{
return v___x_1073_;
}
}
}
}
}
else
{
lean_object* v_a_1077_; lean_object* v___x_1079_; uint8_t v_isShared_1080_; uint8_t v_isSharedCheck_1084_; 
lean_dec_ref(v_a_995_);
lean_dec_ref(v_lut_994_);
v_a_1077_ = lean_ctor_get(v___x_998_, 0);
v_isSharedCheck_1084_ = !lean_is_exclusive(v___x_998_);
if (v_isSharedCheck_1084_ == 0)
{
v___x_1079_ = v___x_998_;
v_isShared_1080_ = v_isSharedCheck_1084_;
goto v_resetjp_1078_;
}
else
{
lean_inc(v_a_1077_);
lean_dec(v___x_998_);
v___x_1079_ = lean_box(0);
v_isShared_1080_ = v_isSharedCheck_1084_;
goto v_resetjp_1078_;
}
v_resetjp_1078_:
{
lean_object* v___x_1082_; 
if (v_isShared_1080_ == 0)
{
v___x_1082_ = v___x_1079_;
goto v_reusejp_1081_;
}
else
{
lean_object* v_reuseFailAlloc_1083_; 
v_reuseFailAlloc_1083_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1083_, 0, v_a_1077_);
v___x_1082_ = v_reuseFailAlloc_1083_;
goto v_reusejp_1081_;
}
v_reusejp_1081_:
{
return v___x_1082_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___boxed(lean_object* v_sb_1085_, lean_object* v_lut_1086_, lean_object* v_a_1087_, lean_object* v_a_1088_, lean_object* v_a_1089_){
_start:
{
lean_object* v_res_1090_; 
v_res_1090_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4(v_sb_1085_, v_lut_1086_, v_a_1087_, v_a_1088_);
lean_dec(v_a_1088_);
lean_dec_ref(v_sb_1085_);
return v_res_1090_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__0(void){
_start:
{
lean_object* v___x_1091_; lean_object* v___x_1092_; lean_object* v___x_1093_; 
v___x_1091_ = lean_unsigned_to_nat(8u);
v___x_1092_ = lean_unsigned_to_nat(0u);
v___x_1093_ = lean_nat_mod(v___x_1092_, v___x_1091_);
return v___x_1093_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__1(void){
_start:
{
lean_object* v___x_1094_; lean_object* v___x_1095_; lean_object* v___x_1096_; 
v___x_1094_ = lean_unsigned_to_nat(8u);
v___x_1095_ = lean_unsigned_to_nat(1u);
v___x_1096_ = lean_nat_mod(v___x_1095_, v___x_1094_);
return v___x_1096_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__2(void){
_start:
{
lean_object* v___x_1097_; lean_object* v___x_1098_; lean_object* v___x_1099_; 
v___x_1097_ = lean_unsigned_to_nat(8u);
v___x_1098_ = lean_unsigned_to_nat(2u);
v___x_1099_ = lean_nat_mod(v___x_1098_, v___x_1097_);
return v___x_1099_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__3(void){
_start:
{
lean_object* v___x_1100_; lean_object* v___x_1101_; lean_object* v___x_1102_; 
v___x_1100_ = lean_unsigned_to_nat(8u);
v___x_1101_ = lean_unsigned_to_nat(3u);
v___x_1102_ = lean_nat_mod(v___x_1101_, v___x_1100_);
return v___x_1102_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__4(void){
_start:
{
lean_object* v___x_1103_; lean_object* v___x_1104_; lean_object* v___x_1105_; 
v___x_1103_ = lean_unsigned_to_nat(8u);
v___x_1104_ = lean_unsigned_to_nat(4u);
v___x_1105_ = lean_nat_mod(v___x_1104_, v___x_1103_);
return v___x_1105_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__5(void){
_start:
{
lean_object* v___x_1106_; lean_object* v___x_1107_; lean_object* v___x_1108_; 
v___x_1106_ = lean_unsigned_to_nat(8u);
v___x_1107_ = lean_unsigned_to_nat(5u);
v___x_1108_ = lean_nat_mod(v___x_1107_, v___x_1106_);
return v___x_1108_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__6(void){
_start:
{
lean_object* v___x_1109_; lean_object* v___x_1110_; lean_object* v___x_1111_; 
v___x_1109_ = lean_unsigned_to_nat(8u);
v___x_1110_ = lean_unsigned_to_nat(6u);
v___x_1111_ = lean_nat_mod(v___x_1110_, v___x_1109_);
return v___x_1111_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__7(void){
_start:
{
lean_object* v___x_1112_; lean_object* v___x_1113_; lean_object* v___x_1114_; 
v___x_1112_ = lean_unsigned_to_nat(8u);
v___x_1113_ = lean_unsigned_to_nat(7u);
v___x_1114_ = lean_nat_mod(v___x_1113_, v___x_1112_);
return v___x_1114_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8(lean_object* v_sb_1115_, lean_object* v_lut_1116_, lean_object* v_a_1117_, lean_object* v_a_1118_){
_start:
{
lean_object* v___x_1120_; 
lean_inc_ref(v_lut_1116_);
v___x_1120_ = lp_zhc__lean_Zhc_Ffi_L2r_Lut8_ofModel(v_sb_1115_, v_lut_1116_);
if (lean_obj_tag(v___x_1120_) == 0)
{
lean_object* v_a_1121_; lean_object* v_handle_1122_; lean_object* v_model_1123_; lean_object* v___x_1125_; uint8_t v_isShared_1126_; uint8_t v_isSharedCheck_1246_; 
v_a_1121_ = lean_ctor_get(v___x_1120_, 0);
lean_inc(v_a_1121_);
lean_dec_ref_known(v___x_1120_, 1);
v_handle_1122_ = lean_ctor_get(v_a_1117_, 0);
v_model_1123_ = lean_ctor_get(v_a_1117_, 1);
v_isSharedCheck_1246_ = !lean_is_exclusive(v_a_1117_);
if (v_isSharedCheck_1246_ == 0)
{
v___x_1125_ = v_a_1117_;
v_isShared_1126_ = v_isSharedCheck_1246_;
goto v_resetjp_1124_;
}
else
{
lean_inc(v_model_1123_);
lean_inc(v_handle_1122_);
lean_dec(v_a_1117_);
v___x_1125_ = lean_box(0);
v_isShared_1126_ = v_isSharedCheck_1246_;
goto v_resetjp_1124_;
}
v_resetjp_1124_:
{
lean_object* v___x_1127_; 
v___x_1127_ = zhc_lean_builder_block_lookup8(v_a_1118_, v_handle_1122_, v_a_1121_);
lean_dec(v_a_1121_);
lean_dec(v_handle_1122_);
if (lean_obj_tag(v___x_1127_) == 0)
{
lean_object* v_a_1128_; lean_object* v___x_1129_; 
v_a_1128_ = lean_ctor_get(v___x_1127_, 0);
lean_inc(v_a_1128_);
lean_dec_ref_known(v___x_1127_, 1);
v___x_1129_ = lp_zhc__lean___private_Zhc_BlockAlgebra_Tracer_Instance_0__Zhc_BlockAlgebra_Tracer_unpack8(v_a_1128_);
lean_dec(v_a_1128_);
if (lean_obj_tag(v___x_1129_) == 0)
{
lean_object* v_a_1130_; lean_object* v___x_1132_; uint8_t v_isShared_1133_; uint8_t v_isSharedCheck_1229_; 
v_a_1130_ = lean_ctor_get(v___x_1129_, 0);
v_isSharedCheck_1229_ = !lean_is_exclusive(v___x_1129_);
if (v_isSharedCheck_1229_ == 0)
{
v___x_1132_ = v___x_1129_;
v_isShared_1133_ = v_isSharedCheck_1229_;
goto v_resetjp_1131_;
}
else
{
lean_inc(v_a_1130_);
lean_dec(v___x_1129_);
v___x_1132_ = lean_box(0);
v_isShared_1133_ = v_isSharedCheck_1229_;
goto v_resetjp_1131_;
}
v_resetjp_1131_:
{
lean_object* v_snd_1134_; lean_object* v_snd_1135_; lean_object* v_fst_1136_; lean_object* v___x_1138_; uint8_t v_isShared_1139_; uint8_t v_isSharedCheck_1227_; 
v_snd_1134_ = lean_ctor_get(v_a_1130_, 1);
lean_inc(v_snd_1134_);
v_snd_1135_ = lean_ctor_get(v_snd_1134_, 1);
lean_inc(v_snd_1135_);
v_fst_1136_ = lean_ctor_get(v_a_1130_, 0);
v_isSharedCheck_1227_ = !lean_is_exclusive(v_a_1130_);
if (v_isSharedCheck_1227_ == 0)
{
lean_object* v_unused_1228_; 
v_unused_1228_ = lean_ctor_get(v_a_1130_, 1);
lean_dec(v_unused_1228_);
v___x_1138_ = v_a_1130_;
v_isShared_1139_ = v_isSharedCheck_1227_;
goto v_resetjp_1137_;
}
else
{
lean_inc(v_fst_1136_);
lean_dec(v_a_1130_);
v___x_1138_ = lean_box(0);
v_isShared_1139_ = v_isSharedCheck_1227_;
goto v_resetjp_1137_;
}
v_resetjp_1137_:
{
lean_object* v_fst_1140_; lean_object* v___x_1142_; uint8_t v_isShared_1143_; uint8_t v_isSharedCheck_1225_; 
v_fst_1140_ = lean_ctor_get(v_snd_1134_, 0);
v_isSharedCheck_1225_ = !lean_is_exclusive(v_snd_1134_);
if (v_isSharedCheck_1225_ == 0)
{
lean_object* v_unused_1226_; 
v_unused_1226_ = lean_ctor_get(v_snd_1134_, 1);
lean_dec(v_unused_1226_);
v___x_1142_ = v_snd_1134_;
v_isShared_1143_ = v_isSharedCheck_1225_;
goto v_resetjp_1141_;
}
else
{
lean_inc(v_fst_1140_);
lean_dec(v_snd_1134_);
v___x_1142_ = lean_box(0);
v_isShared_1143_ = v_isSharedCheck_1225_;
goto v_resetjp_1141_;
}
v_resetjp_1141_:
{
lean_object* v_fst_1144_; lean_object* v_snd_1145_; lean_object* v___x_1147_; uint8_t v_isShared_1148_; uint8_t v_isSharedCheck_1224_; 
v_fst_1144_ = lean_ctor_get(v_snd_1135_, 0);
v_snd_1145_ = lean_ctor_get(v_snd_1135_, 1);
v_isSharedCheck_1224_ = !lean_is_exclusive(v_snd_1135_);
if (v_isSharedCheck_1224_ == 0)
{
v___x_1147_ = v_snd_1135_;
v_isShared_1148_ = v_isSharedCheck_1224_;
goto v_resetjp_1146_;
}
else
{
lean_inc(v_snd_1145_);
lean_inc(v_fst_1144_);
lean_dec(v_snd_1135_);
v___x_1147_ = lean_box(0);
v_isShared_1148_ = v_isSharedCheck_1224_;
goto v_resetjp_1146_;
}
v_resetjp_1146_:
{
lean_object* v___x_1149_; lean_object* v___x_1150_; lean_object* v___x_1151_; lean_object* v_fst_1152_; lean_object* v_snd_1153_; lean_object* v___x_1155_; uint8_t v_isShared_1156_; uint8_t v_isSharedCheck_1223_; 
v___x_1149_ = lean_unsigned_to_nat(3u);
v___x_1150_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__0);
lean_inc_ref(v_lut_1116_);
v___x_1151_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1150_);
v_fst_1152_ = lean_ctor_get(v_snd_1145_, 0);
v_snd_1153_ = lean_ctor_get(v_snd_1145_, 1);
v_isSharedCheck_1223_ = !lean_is_exclusive(v_snd_1145_);
if (v_isSharedCheck_1223_ == 0)
{
v___x_1155_ = v_snd_1145_;
v_isShared_1156_ = v_isSharedCheck_1223_;
goto v_resetjp_1154_;
}
else
{
lean_inc(v_snd_1153_);
lean_inc(v_fst_1152_);
lean_dec(v_snd_1145_);
v___x_1155_ = lean_box(0);
v_isShared_1156_ = v_isSharedCheck_1223_;
goto v_resetjp_1154_;
}
v_resetjp_1154_:
{
lean_object* v___x_1157_; lean_object* v___x_1158_; lean_object* v_fst_1159_; lean_object* v_snd_1160_; lean_object* v___x_1162_; uint8_t v_isShared_1163_; uint8_t v_isSharedCheck_1222_; 
v___x_1157_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__1, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__1_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__1);
lean_inc_ref(v_lut_1116_);
v___x_1158_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1157_);
v_fst_1159_ = lean_ctor_get(v_snd_1153_, 0);
v_snd_1160_ = lean_ctor_get(v_snd_1153_, 1);
v_isSharedCheck_1222_ = !lean_is_exclusive(v_snd_1153_);
if (v_isSharedCheck_1222_ == 0)
{
v___x_1162_ = v_snd_1153_;
v_isShared_1163_ = v_isSharedCheck_1222_;
goto v_resetjp_1161_;
}
else
{
lean_inc(v_snd_1160_);
lean_inc(v_fst_1159_);
lean_dec(v_snd_1153_);
v___x_1162_ = lean_box(0);
v_isShared_1163_ = v_isSharedCheck_1222_;
goto v_resetjp_1161_;
}
v_resetjp_1161_:
{
lean_object* v___x_1164_; lean_object* v___x_1165_; lean_object* v_fst_1166_; lean_object* v_snd_1167_; lean_object* v___x_1169_; uint8_t v_isShared_1170_; uint8_t v_isSharedCheck_1221_; 
v___x_1164_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__2, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__2_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__2);
lean_inc_ref(v_lut_1116_);
v___x_1165_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1164_);
v_fst_1166_ = lean_ctor_get(v_snd_1160_, 0);
v_snd_1167_ = lean_ctor_get(v_snd_1160_, 1);
v_isSharedCheck_1221_ = !lean_is_exclusive(v_snd_1160_);
if (v_isSharedCheck_1221_ == 0)
{
v___x_1169_ = v_snd_1160_;
v_isShared_1170_ = v_isSharedCheck_1221_;
goto v_resetjp_1168_;
}
else
{
lean_inc(v_snd_1167_);
lean_inc(v_fst_1166_);
lean_dec(v_snd_1160_);
v___x_1169_ = lean_box(0);
v_isShared_1170_ = v_isSharedCheck_1221_;
goto v_resetjp_1168_;
}
v_resetjp_1168_:
{
lean_object* v___x_1171_; lean_object* v___x_1172_; lean_object* v_fst_1173_; lean_object* v_snd_1174_; lean_object* v___x_1176_; uint8_t v_isShared_1177_; uint8_t v_isSharedCheck_1220_; 
v___x_1171_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__3, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__3_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__3);
lean_inc_ref(v_lut_1116_);
v___x_1172_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1171_);
v_fst_1173_ = lean_ctor_get(v_snd_1167_, 0);
v_snd_1174_ = lean_ctor_get(v_snd_1167_, 1);
v_isSharedCheck_1220_ = !lean_is_exclusive(v_snd_1167_);
if (v_isSharedCheck_1220_ == 0)
{
v___x_1176_ = v_snd_1167_;
v_isShared_1177_ = v_isSharedCheck_1220_;
goto v_resetjp_1175_;
}
else
{
lean_inc(v_snd_1174_);
lean_inc(v_fst_1173_);
lean_dec(v_snd_1167_);
v___x_1176_ = lean_box(0);
v_isShared_1177_ = v_isSharedCheck_1220_;
goto v_resetjp_1175_;
}
v_resetjp_1175_:
{
lean_object* v___x_1178_; lean_object* v___x_1179_; lean_object* v___x_1181_; 
v___x_1178_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__4, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__4_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__4);
lean_inc_ref(v_lut_1116_);
v___x_1179_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1178_);
if (v_isShared_1126_ == 0)
{
lean_ctor_set(v___x_1125_, 1, v___x_1151_);
lean_ctor_set(v___x_1125_, 0, v_fst_1136_);
v___x_1181_ = v___x_1125_;
goto v_reusejp_1180_;
}
else
{
lean_object* v_reuseFailAlloc_1219_; 
v_reuseFailAlloc_1219_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1219_, 0, v_fst_1136_);
lean_ctor_set(v_reuseFailAlloc_1219_, 1, v___x_1151_);
v___x_1181_ = v_reuseFailAlloc_1219_;
goto v_reusejp_1180_;
}
v_reusejp_1180_:
{
lean_object* v___x_1182_; lean_object* v___x_1183_; lean_object* v___x_1184_; lean_object* v___x_1185_; lean_object* v___x_1186_; lean_object* v___x_1187_; lean_object* v___x_1188_; lean_object* v___x_1189_; lean_object* v___x_1190_; lean_object* v___x_1191_; lean_object* v___x_1192_; lean_object* v___x_1193_; lean_object* v___x_1194_; lean_object* v___x_1196_; 
v___x_1182_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1182_, 0, v_fst_1140_);
lean_ctor_set(v___x_1182_, 1, v___x_1158_);
v___x_1183_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1183_, 0, v_fst_1144_);
lean_ctor_set(v___x_1183_, 1, v___x_1165_);
v___x_1184_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1184_, 0, v_fst_1152_);
lean_ctor_set(v___x_1184_, 1, v___x_1172_);
v___x_1185_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1185_, 0, v_fst_1159_);
lean_ctor_set(v___x_1185_, 1, v___x_1179_);
v___x_1186_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__5, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__5_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__5);
lean_inc_ref_n(v_lut_1116_, 2);
v___x_1187_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1186_);
v___x_1188_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1188_, 0, v_fst_1166_);
lean_ctor_set(v___x_1188_, 1, v___x_1187_);
v___x_1189_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__6, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__6_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__6);
v___x_1190_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1189_);
v___x_1191_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1191_, 0, v_fst_1173_);
lean_ctor_set(v___x_1191_, 1, v___x_1190_);
v___x_1192_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__7, &lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__7_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___closed__7);
v___x_1193_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_1115_, v___x_1149_, v_lut_1116_, v_model_1123_, v___x_1192_);
lean_dec(v_model_1123_);
v___x_1194_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1194_, 0, v_snd_1174_);
lean_ctor_set(v___x_1194_, 1, v___x_1193_);
if (v_isShared_1177_ == 0)
{
lean_ctor_set(v___x_1176_, 1, v___x_1194_);
lean_ctor_set(v___x_1176_, 0, v___x_1191_);
v___x_1196_ = v___x_1176_;
goto v_reusejp_1195_;
}
else
{
lean_object* v_reuseFailAlloc_1218_; 
v_reuseFailAlloc_1218_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1218_, 0, v___x_1191_);
lean_ctor_set(v_reuseFailAlloc_1218_, 1, v___x_1194_);
v___x_1196_ = v_reuseFailAlloc_1218_;
goto v_reusejp_1195_;
}
v_reusejp_1195_:
{
lean_object* v___x_1198_; 
if (v_isShared_1170_ == 0)
{
lean_ctor_set(v___x_1169_, 1, v___x_1196_);
lean_ctor_set(v___x_1169_, 0, v___x_1188_);
v___x_1198_ = v___x_1169_;
goto v_reusejp_1197_;
}
else
{
lean_object* v_reuseFailAlloc_1217_; 
v_reuseFailAlloc_1217_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1217_, 0, v___x_1188_);
lean_ctor_set(v_reuseFailAlloc_1217_, 1, v___x_1196_);
v___x_1198_ = v_reuseFailAlloc_1217_;
goto v_reusejp_1197_;
}
v_reusejp_1197_:
{
lean_object* v___x_1200_; 
if (v_isShared_1163_ == 0)
{
lean_ctor_set(v___x_1162_, 1, v___x_1198_);
lean_ctor_set(v___x_1162_, 0, v___x_1185_);
v___x_1200_ = v___x_1162_;
goto v_reusejp_1199_;
}
else
{
lean_object* v_reuseFailAlloc_1216_; 
v_reuseFailAlloc_1216_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1216_, 0, v___x_1185_);
lean_ctor_set(v_reuseFailAlloc_1216_, 1, v___x_1198_);
v___x_1200_ = v_reuseFailAlloc_1216_;
goto v_reusejp_1199_;
}
v_reusejp_1199_:
{
lean_object* v___x_1202_; 
if (v_isShared_1156_ == 0)
{
lean_ctor_set(v___x_1155_, 1, v___x_1200_);
lean_ctor_set(v___x_1155_, 0, v___x_1184_);
v___x_1202_ = v___x_1155_;
goto v_reusejp_1201_;
}
else
{
lean_object* v_reuseFailAlloc_1215_; 
v_reuseFailAlloc_1215_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1215_, 0, v___x_1184_);
lean_ctor_set(v_reuseFailAlloc_1215_, 1, v___x_1200_);
v___x_1202_ = v_reuseFailAlloc_1215_;
goto v_reusejp_1201_;
}
v_reusejp_1201_:
{
lean_object* v___x_1204_; 
if (v_isShared_1148_ == 0)
{
lean_ctor_set(v___x_1147_, 1, v___x_1202_);
lean_ctor_set(v___x_1147_, 0, v___x_1183_);
v___x_1204_ = v___x_1147_;
goto v_reusejp_1203_;
}
else
{
lean_object* v_reuseFailAlloc_1214_; 
v_reuseFailAlloc_1214_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1214_, 0, v___x_1183_);
lean_ctor_set(v_reuseFailAlloc_1214_, 1, v___x_1202_);
v___x_1204_ = v_reuseFailAlloc_1214_;
goto v_reusejp_1203_;
}
v_reusejp_1203_:
{
lean_object* v___x_1206_; 
if (v_isShared_1143_ == 0)
{
lean_ctor_set(v___x_1142_, 1, v___x_1204_);
lean_ctor_set(v___x_1142_, 0, v___x_1182_);
v___x_1206_ = v___x_1142_;
goto v_reusejp_1205_;
}
else
{
lean_object* v_reuseFailAlloc_1213_; 
v_reuseFailAlloc_1213_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1213_, 0, v___x_1182_);
lean_ctor_set(v_reuseFailAlloc_1213_, 1, v___x_1204_);
v___x_1206_ = v_reuseFailAlloc_1213_;
goto v_reusejp_1205_;
}
v_reusejp_1205_:
{
lean_object* v___x_1208_; 
if (v_isShared_1139_ == 0)
{
lean_ctor_set(v___x_1138_, 1, v___x_1206_);
lean_ctor_set(v___x_1138_, 0, v___x_1181_);
v___x_1208_ = v___x_1138_;
goto v_reusejp_1207_;
}
else
{
lean_object* v_reuseFailAlloc_1212_; 
v_reuseFailAlloc_1212_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1212_, 0, v___x_1181_);
lean_ctor_set(v_reuseFailAlloc_1212_, 1, v___x_1206_);
v___x_1208_ = v_reuseFailAlloc_1212_;
goto v_reusejp_1207_;
}
v_reusejp_1207_:
{
lean_object* v___x_1210_; 
if (v_isShared_1133_ == 0)
{
lean_ctor_set(v___x_1132_, 0, v___x_1208_);
v___x_1210_ = v___x_1132_;
goto v_reusejp_1209_;
}
else
{
lean_object* v_reuseFailAlloc_1211_; 
v_reuseFailAlloc_1211_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1211_, 0, v___x_1208_);
v___x_1210_ = v_reuseFailAlloc_1211_;
goto v_reusejp_1209_;
}
v_reusejp_1209_:
{
return v___x_1210_;
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
else
{
lean_object* v_a_1230_; lean_object* v___x_1232_; uint8_t v_isShared_1233_; uint8_t v_isSharedCheck_1237_; 
lean_del_object(v___x_1125_);
lean_dec(v_model_1123_);
lean_dec_ref(v_lut_1116_);
v_a_1230_ = lean_ctor_get(v___x_1129_, 0);
v_isSharedCheck_1237_ = !lean_is_exclusive(v___x_1129_);
if (v_isSharedCheck_1237_ == 0)
{
v___x_1232_ = v___x_1129_;
v_isShared_1233_ = v_isSharedCheck_1237_;
goto v_resetjp_1231_;
}
else
{
lean_inc(v_a_1230_);
lean_dec(v___x_1129_);
v___x_1232_ = lean_box(0);
v_isShared_1233_ = v_isSharedCheck_1237_;
goto v_resetjp_1231_;
}
v_resetjp_1231_:
{
lean_object* v___x_1235_; 
if (v_isShared_1233_ == 0)
{
v___x_1235_ = v___x_1232_;
goto v_reusejp_1234_;
}
else
{
lean_object* v_reuseFailAlloc_1236_; 
v_reuseFailAlloc_1236_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1236_, 0, v_a_1230_);
v___x_1235_ = v_reuseFailAlloc_1236_;
goto v_reusejp_1234_;
}
v_reusejp_1234_:
{
return v___x_1235_;
}
}
}
}
else
{
lean_object* v_a_1238_; lean_object* v___x_1240_; uint8_t v_isShared_1241_; uint8_t v_isSharedCheck_1245_; 
lean_del_object(v___x_1125_);
lean_dec(v_model_1123_);
lean_dec_ref(v_lut_1116_);
v_a_1238_ = lean_ctor_get(v___x_1127_, 0);
v_isSharedCheck_1245_ = !lean_is_exclusive(v___x_1127_);
if (v_isSharedCheck_1245_ == 0)
{
v___x_1240_ = v___x_1127_;
v_isShared_1241_ = v_isSharedCheck_1245_;
goto v_resetjp_1239_;
}
else
{
lean_inc(v_a_1238_);
lean_dec(v___x_1127_);
v___x_1240_ = lean_box(0);
v_isShared_1241_ = v_isSharedCheck_1245_;
goto v_resetjp_1239_;
}
v_resetjp_1239_:
{
lean_object* v___x_1243_; 
if (v_isShared_1241_ == 0)
{
v___x_1243_ = v___x_1240_;
goto v_reusejp_1242_;
}
else
{
lean_object* v_reuseFailAlloc_1244_; 
v_reuseFailAlloc_1244_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1244_, 0, v_a_1238_);
v___x_1243_ = v_reuseFailAlloc_1244_;
goto v_reusejp_1242_;
}
v_reusejp_1242_:
{
return v___x_1243_;
}
}
}
}
}
else
{
lean_object* v_a_1247_; lean_object* v___x_1249_; uint8_t v_isShared_1250_; uint8_t v_isSharedCheck_1254_; 
lean_dec_ref(v_a_1117_);
lean_dec_ref(v_lut_1116_);
v_a_1247_ = lean_ctor_get(v___x_1120_, 0);
v_isSharedCheck_1254_ = !lean_is_exclusive(v___x_1120_);
if (v_isSharedCheck_1254_ == 0)
{
v___x_1249_ = v___x_1120_;
v_isShared_1250_ = v_isSharedCheck_1254_;
goto v_resetjp_1248_;
}
else
{
lean_inc(v_a_1247_);
lean_dec(v___x_1120_);
v___x_1249_ = lean_box(0);
v_isShared_1250_ = v_isSharedCheck_1254_;
goto v_resetjp_1248_;
}
v_resetjp_1248_:
{
lean_object* v___x_1252_; 
if (v_isShared_1250_ == 0)
{
v___x_1252_ = v___x_1249_;
goto v_reusejp_1251_;
}
else
{
lean_object* v_reuseFailAlloc_1253_; 
v_reuseFailAlloc_1253_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1253_, 0, v_a_1247_);
v___x_1252_ = v_reuseFailAlloc_1253_;
goto v_reusejp_1251_;
}
v_reusejp_1251_:
{
return v___x_1252_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___boxed(lean_object* v_sb_1255_, lean_object* v_lut_1256_, lean_object* v_a_1257_, lean_object* v_a_1258_, lean_object* v_a_1259_){
_start:
{
lean_object* v_res_1260_; 
v_res_1260_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8(v_sb_1255_, v_lut_1256_, v_a_1257_, v_a_1258_);
lean_dec(v_a_1258_);
lean_dec_ref(v_sb_1255_);
return v_res_1260_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_instBlockAlgebraTracer(lean_object* v_sb_1261_){
_start:
{
lean_object* v___x_1262_; lean_object* v___x_1263_; lean_object* v___x_1264_; lean_object* v___x_1265_; lean_object* v___x_1266_; lean_object* v___x_1267_; lean_object* v___x_1268_; lean_object* v___x_1269_; lean_object* v___x_1270_; lean_object* v___x_1271_; lean_object* v___x_1272_; lean_object* v___x_1273_; lean_object* v___x_1274_; lean_object* v___x_1275_; lean_object* v___x_1276_; lean_object* v___x_1277_; lean_object* v___x_1278_; lean_object* v___x_1279_; lean_object* v___x_1280_; lean_object* v___x_1281_; lean_object* v___x_1282_; 
lean_inc_ref_n(v_sb_1261_, 19);
v___x_1262_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger___boxed), 4, 1);
lean_closure_set(v___x_1262_, 0, v_sb_1261_);
v___x_1263_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock___boxed), 4, 1);
lean_closure_set(v___x_1263_, 0, v_sb_1261_);
v___x_1264_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock___boxed), 4, 1);
lean_closure_set(v___x_1264_, 0, v_sb_1261_);
v___x_1265_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt___boxed), 5, 1);
lean_closure_set(v___x_1265_, 0, v_sb_1261_);
v___x_1266_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subCt___boxed), 5, 1);
lean_closure_set(v___x_1266_, 0, v_sb_1261_);
v___x_1267_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_shlCt___boxed), 5, 1);
lean_closure_set(v___x_1267_, 0, v_sb_1261_);
v___x_1268_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt___boxed), 6, 1);
lean_closure_set(v___x_1268_, 0, v_sb_1261_);
v___x_1269_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt___boxed), 5, 1);
lean_closure_set(v___x_1269_, 0, v_sb_1261_);
v___x_1270_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_subPt___boxed), 5, 1);
lean_closure_set(v___x_1270_, 0, v_sb_1261_);
v___x_1271_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_ptSub___boxed), 5, 1);
lean_closure_set(v___x_1271_, 0, v_sb_1261_);
v___x_1272_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_mulPt___boxed), 5, 1);
lean_closure_set(v___x_1272_, 0, v_sb_1261_);
v___x_1273_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___boxed), 6, 1);
lean_closure_set(v___x_1273_, 0, v_sb_1261_);
v___x_1274_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractPtBlock___boxed), 6, 1);
lean_closure_set(v___x_1274_, 0, v_sb_1261_);
v___x_1275_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___boxed), 7, 1);
lean_closure_set(v___x_1275_, 0, v_sb_1261_);
v___x_1276_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_boolFromBlock___boxed), 4, 1);
lean_closure_set(v___x_1276_, 0, v_sb_1261_);
v___x_1277_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractBoolBlock___boxed), 4, 1);
lean_closure_set(v___x_1277_, 0, v_sb_1261_);
v___x_1278_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs___boxed), 5, 1);
lean_closure_set(v___x_1278_, 0, v_sb_1261_);
v___x_1279_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2___boxed), 5, 1);
lean_closure_set(v___x_1279_, 0, v_sb_1261_);
v___x_1280_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs4___boxed), 5, 1);
lean_closure_set(v___x_1280_, 0, v_sb_1261_);
v___x_1281_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs8___boxed), 5, 1);
lean_closure_set(v___x_1281_, 0, v_sb_1261_);
v___x_1282_ = lean_alloc_ctor(0, 20, 0);
lean_ctor_set(v___x_1282_, 0, v___x_1262_);
lean_ctor_set(v___x_1282_, 1, v___x_1263_);
lean_ctor_set(v___x_1282_, 2, v___x_1264_);
lean_ctor_set(v___x_1282_, 3, v___x_1265_);
lean_ctor_set(v___x_1282_, 4, v___x_1266_);
lean_ctor_set(v___x_1282_, 5, v___x_1267_);
lean_ctor_set(v___x_1282_, 6, v___x_1268_);
lean_ctor_set(v___x_1282_, 7, v___x_1269_);
lean_ctor_set(v___x_1282_, 8, v___x_1270_);
lean_ctor_set(v___x_1282_, 9, v___x_1271_);
lean_ctor_set(v___x_1282_, 10, v___x_1272_);
lean_ctor_set(v___x_1282_, 11, v___x_1273_);
lean_ctor_set(v___x_1282_, 12, v___x_1274_);
lean_ctor_set(v___x_1282_, 13, v___x_1275_);
lean_ctor_set(v___x_1282_, 14, v___x_1276_);
lean_ctor_set(v___x_1282_, 15, v___x_1277_);
lean_ctor_set(v___x_1282_, 16, v___x_1278_);
lean_ctor_set(v___x_1282_, 17, v___x_1279_);
lean_ctor_set(v___x_1282_, 18, v___x_1280_);
lean_ctor_set(v___x_1282_, 19, v___x_1281_);
return v___x_1282_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(lean_object* v_sb_1283_, lean_object* v_si_1284_, lean_object* v_a_1285_){
_start:
{
lean_object* v___x_1287_; lean_object* v___x_1288_; 
v___x_1287_ = lp_zhc__lean_Zhc_Model_SpecInteger_intSize(v_sb_1283_, v_si_1284_);
v___x_1288_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_narrow16(v___x_1287_);
if (lean_obj_tag(v___x_1288_) == 0)
{
lean_object* v_a_1289_; uint16_t v___x_1290_; lean_object* v___x_1291_; 
v_a_1289_ = lean_ctor_get(v___x_1288_, 0);
lean_inc(v_a_1289_);
lean_dec_ref_known(v___x_1288_, 1);
v___x_1290_ = lean_unbox(v_a_1289_);
lean_dec(v_a_1289_);
v___x_1291_ = zhc_lean_builder_ciphertext_integer_input(v_a_1285_, v___x_1290_);
if (lean_obj_tag(v___x_1291_) == 0)
{
lean_object* v_a_1292_; lean_object* v___x_1294_; uint8_t v_isShared_1295_; uint8_t v_isSharedCheck_1302_; 
v_a_1292_ = lean_ctor_get(v___x_1291_, 0);
v_isSharedCheck_1302_ = !lean_is_exclusive(v___x_1291_);
if (v_isSharedCheck_1302_ == 0)
{
v___x_1294_ = v___x_1291_;
v_isShared_1295_ = v_isSharedCheck_1302_;
goto v_resetjp_1293_;
}
else
{
lean_inc(v_a_1292_);
lean_dec(v___x_1291_);
v___x_1294_ = lean_box(0);
v_isShared_1295_ = v_isSharedCheck_1302_;
goto v_resetjp_1293_;
}
v_resetjp_1293_:
{
lean_object* v___x_1296_; lean_object* v___x_1297_; lean_object* v___x_1298_; lean_object* v___x_1300_; 
v___x_1296_ = lean_unsigned_to_nat(0u);
v___x_1297_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v_sb_1283_, v_si_1284_, v___x_1296_);
v___x_1298_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1298_, 0, v_a_1292_);
lean_ctor_set(v___x_1298_, 1, v___x_1297_);
if (v_isShared_1295_ == 0)
{
lean_ctor_set(v___x_1294_, 0, v___x_1298_);
v___x_1300_ = v___x_1294_;
goto v_reusejp_1299_;
}
else
{
lean_object* v_reuseFailAlloc_1301_; 
v_reuseFailAlloc_1301_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1301_, 0, v___x_1298_);
v___x_1300_ = v_reuseFailAlloc_1301_;
goto v_reusejp_1299_;
}
v_reusejp_1299_:
{
return v___x_1300_;
}
}
}
else
{
lean_object* v_a_1303_; lean_object* v___x_1305_; uint8_t v_isShared_1306_; uint8_t v_isSharedCheck_1310_; 
v_a_1303_ = lean_ctor_get(v___x_1291_, 0);
v_isSharedCheck_1310_ = !lean_is_exclusive(v___x_1291_);
if (v_isSharedCheck_1310_ == 0)
{
v___x_1305_ = v___x_1291_;
v_isShared_1306_ = v_isSharedCheck_1310_;
goto v_resetjp_1304_;
}
else
{
lean_inc(v_a_1303_);
lean_dec(v___x_1291_);
v___x_1305_ = lean_box(0);
v_isShared_1306_ = v_isSharedCheck_1310_;
goto v_resetjp_1304_;
}
v_resetjp_1304_:
{
lean_object* v___x_1308_; 
if (v_isShared_1306_ == 0)
{
v___x_1308_ = v___x_1305_;
goto v_reusejp_1307_;
}
else
{
lean_object* v_reuseFailAlloc_1309_; 
v_reuseFailAlloc_1309_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1309_, 0, v_a_1303_);
v___x_1308_ = v_reuseFailAlloc_1309_;
goto v_reusejp_1307_;
}
v_reusejp_1307_:
{
return v___x_1308_;
}
}
}
}
else
{
lean_object* v_a_1311_; lean_object* v___x_1313_; uint8_t v_isShared_1314_; uint8_t v_isSharedCheck_1318_; 
v_a_1311_ = lean_ctor_get(v___x_1288_, 0);
v_isSharedCheck_1318_ = !lean_is_exclusive(v___x_1288_);
if (v_isSharedCheck_1318_ == 0)
{
v___x_1313_ = v___x_1288_;
v_isShared_1314_ = v_isSharedCheck_1318_;
goto v_resetjp_1312_;
}
else
{
lean_inc(v_a_1311_);
lean_dec(v___x_1288_);
v___x_1313_ = lean_box(0);
v_isShared_1314_ = v_isSharedCheck_1318_;
goto v_resetjp_1312_;
}
v_resetjp_1312_:
{
lean_object* v___x_1316_; 
if (v_isShared_1314_ == 0)
{
v___x_1316_ = v___x_1313_;
goto v_reusejp_1315_;
}
else
{
lean_object* v_reuseFailAlloc_1317_; 
v_reuseFailAlloc_1317_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1317_, 0, v_a_1311_);
v___x_1316_ = v_reuseFailAlloc_1317_;
goto v_reusejp_1315_;
}
v_reusejp_1315_:
{
return v___x_1316_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger___boxed(lean_object* v_sb_1319_, lean_object* v_si_1320_, lean_object* v_a_1321_, lean_object* v_a_1322_){
_start:
{
lean_object* v_res_1323_; 
v_res_1323_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(v_sb_1319_, v_si_1320_, v_a_1321_);
lean_dec(v_a_1321_);
lean_dec(v_si_1320_);
lean_dec_ref(v_sb_1319_);
return v_res_1323_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(lean_object* v_x_1324_, lean_object* v_a_1325_){
_start:
{
lean_object* v_handle_1327_; lean_object* v___x_1328_; 
v_handle_1327_ = lean_ctor_get(v_x_1324_, 0);
v___x_1328_ = zhc_lean_builder_ciphertext_integer_output(v_a_1325_, v_handle_1327_);
return v___x_1328_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg___boxed(lean_object* v_x_1329_, lean_object* v_a_1330_, lean_object* v_a_1331_){
_start:
{
lean_object* v_res_1332_; 
v_res_1332_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(v_x_1329_, v_a_1330_);
lean_dec(v_a_1330_);
lean_dec_ref(v_x_1329_);
return v_res_1332_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger(lean_object* v_sb_1333_, lean_object* v_si_1334_, lean_object* v_x_1335_, lean_object* v_a_1336_){
_start:
{
lean_object* v___x_1338_; 
v___x_1338_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(v_x_1335_, v_a_1336_);
return v___x_1338_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___boxed(lean_object* v_sb_1339_, lean_object* v_si_1340_, lean_object* v_x_1341_, lean_object* v_a_1342_, lean_object* v_a_1343_){
_start:
{
lean_object* v_res_1344_; 
v_res_1344_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger(v_sb_1339_, v_si_1340_, v_x_1341_, v_a_1342_);
lean_dec(v_a_1342_);
lean_dec_ref(v_x_1341_);
lean_dec(v_si_1340_);
lean_dec_ref(v_sb_1339_);
return v_res_1344_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit(lean_object* v_sb_1345_, lean_object* v_prog_1346_){
_start:
{
lean_object* v___x_1348_; 
v___x_1348_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(v_sb_1345_);
if (lean_obj_tag(v___x_1348_) == 0)
{
lean_object* v_a_1349_; lean_object* v___x_1350_; 
v_a_1349_ = lean_ctor_get(v___x_1348_, 0);
lean_inc(v_a_1349_);
lean_dec_ref_known(v___x_1348_, 1);
v___x_1350_ = zhc_lean_builder_new(v_a_1349_);
lean_dec(v_a_1349_);
if (lean_obj_tag(v___x_1350_) == 0)
{
lean_object* v_a_1351_; lean_object* v___x_1352_; 
v_a_1351_ = lean_ctor_get(v___x_1350_, 0);
lean_inc_n(v_a_1351_, 2);
lean_dec_ref_known(v___x_1350_, 1);
v___x_1352_ = lean_apply_2(v_prog_1346_, v_a_1351_, lean_box(0));
if (lean_obj_tag(v___x_1352_) == 0)
{
lean_object* v___x_1354_; uint8_t v_isShared_1355_; uint8_t v_isSharedCheck_1359_; 
v_isSharedCheck_1359_ = !lean_is_exclusive(v___x_1352_);
if (v_isSharedCheck_1359_ == 0)
{
lean_object* v_unused_1360_; 
v_unused_1360_ = lean_ctor_get(v___x_1352_, 0);
lean_dec(v_unused_1360_);
v___x_1354_ = v___x_1352_;
v_isShared_1355_ = v_isSharedCheck_1359_;
goto v_resetjp_1353_;
}
else
{
lean_dec(v___x_1352_);
v___x_1354_ = lean_box(0);
v_isShared_1355_ = v_isSharedCheck_1359_;
goto v_resetjp_1353_;
}
v_resetjp_1353_:
{
lean_object* v___x_1357_; 
if (v_isShared_1355_ == 0)
{
lean_ctor_set(v___x_1354_, 0, v_a_1351_);
v___x_1357_ = v___x_1354_;
goto v_reusejp_1356_;
}
else
{
lean_object* v_reuseFailAlloc_1358_; 
v_reuseFailAlloc_1358_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1358_, 0, v_a_1351_);
v___x_1357_ = v_reuseFailAlloc_1358_;
goto v_reusejp_1356_;
}
v_reusejp_1356_:
{
return v___x_1357_;
}
}
}
else
{
lean_object* v_a_1361_; lean_object* v___x_1363_; uint8_t v_isShared_1364_; uint8_t v_isSharedCheck_1368_; 
lean_dec(v_a_1351_);
v_a_1361_ = lean_ctor_get(v___x_1352_, 0);
v_isSharedCheck_1368_ = !lean_is_exclusive(v___x_1352_);
if (v_isSharedCheck_1368_ == 0)
{
v___x_1363_ = v___x_1352_;
v_isShared_1364_ = v_isSharedCheck_1368_;
goto v_resetjp_1362_;
}
else
{
lean_inc(v_a_1361_);
lean_dec(v___x_1352_);
v___x_1363_ = lean_box(0);
v_isShared_1364_ = v_isSharedCheck_1368_;
goto v_resetjp_1362_;
}
v_resetjp_1362_:
{
lean_object* v___x_1366_; 
if (v_isShared_1364_ == 0)
{
v___x_1366_ = v___x_1363_;
goto v_reusejp_1365_;
}
else
{
lean_object* v_reuseFailAlloc_1367_; 
v_reuseFailAlloc_1367_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1367_, 0, v_a_1361_);
v___x_1366_ = v_reuseFailAlloc_1367_;
goto v_reusejp_1365_;
}
v_reusejp_1365_:
{
return v___x_1366_;
}
}
}
}
else
{
lean_dec_ref(v_prog_1346_);
return v___x_1350_;
}
}
else
{
lean_dec_ref(v_prog_1346_);
return v___x_1348_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit___boxed(lean_object* v_sb_1369_, lean_object* v_prog_1370_, lean_object* v_a_1371_){
_start:
{
lean_object* v_res_1372_; 
v_res_1372_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit(v_sb_1369_, v_prog_1370_);
lean_dec_ref(v_sb_1369_);
return v_res_1372_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Class(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextInteger(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextBool(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_PlaintextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_PlaintextInteger(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_Lookup(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_Operators(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Builder(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Tracer_Instance(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_BlockAlgebra_Class(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_CiphertextBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_CiphertextInteger(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_CiphertextBool(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_PlaintextBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_PlaintextInteger(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_Lookup(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_Operators(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Ffi_L2r_Builder(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
