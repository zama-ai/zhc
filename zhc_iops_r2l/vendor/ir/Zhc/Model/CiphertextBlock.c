// Lean compiler output
// Module: Zhc.Model.CiphertextBlock
// Imports: public import Init public meta import Init public import Zhc.Model.SpecBlock public import Zhc.Util.Theorems
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
lean_object* lean_string_length(lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(lean_object*);
lean_object* lean_nat_pow(lean_object*, lean_object*);
uint8_t lean_nat_dec_lt(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
lean_object* lean_nat_to_int(lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* lean_nat_div(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock___boxed(lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 8, .m_capacity = 8, .m_length = 7, .m_data = "storage"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__1_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__1_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__2_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__2_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__3 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__3_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__4 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__4_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__4_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__5 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__5_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__3_value),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__5_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__6 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__6_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__7;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__8 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__8_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__9_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__9;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__10_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__10;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__0_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__11 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__11_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__12_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__8_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__12 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__12_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat_x3f(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat_x3f___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toMessageNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toMessageNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCarryNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCarryNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toPaddingNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toPaddingNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toDataNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toDataNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskCarry(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskCarry___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMoveCarryToMessage(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMoveCarryToMessage___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doNeg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doNeg___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter___redArg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq___redArg(lean_object* v_x_1_, lean_object* v_x_2_){
_start:
{
uint8_t v___x_3_; 
v___x_3_ = lean_nat_dec_eq(v_x_1_, v_x_2_);
return v___x_3_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq___redArg___boxed(lean_object* v_x_4_, lean_object* v_x_5_){
_start:
{
uint8_t v_res_6_; lean_object* v_r_7_; 
v_res_6_ = lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq___redArg(v_x_4_, v_x_5_);
lean_dec(v_x_5_);
lean_dec(v_x_4_);
v_r_7_ = lean_box(v_res_6_);
return v_r_7_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq(lean_object* v_sb_8_, lean_object* v_x_9_, lean_object* v_x_10_){
_start:
{
uint8_t v___x_11_; 
v___x_11_ = lean_nat_dec_eq(v_x_9_, v_x_10_);
return v___x_11_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq___boxed(lean_object* v_sb_12_, lean_object* v_x_13_, lean_object* v_x_14_){
_start:
{
uint8_t v_res_15_; lean_object* v_r_16_; 
v_res_15_ = lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock_decEq(v_sb_12_, v_x_13_, v_x_14_);
lean_dec(v_x_14_);
lean_dec(v_x_13_);
lean_dec_ref(v_sb_12_);
v_r_16_ = lean_box(v_res_15_);
return v_r_16_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock___redArg(lean_object* v_x_17_, lean_object* v_x_18_){
_start:
{
uint8_t v___x_19_; 
v___x_19_ = lean_nat_dec_eq(v_x_17_, v_x_18_);
return v___x_19_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock___redArg___boxed(lean_object* v_x_20_, lean_object* v_x_21_){
_start:
{
uint8_t v_res_22_; lean_object* v_r_23_; 
v_res_22_ = lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock___redArg(v_x_20_, v_x_21_);
lean_dec(v_x_21_);
lean_dec(v_x_20_);
v_r_23_ = lean_box(v_res_22_);
return v_r_23_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock(lean_object* v_sb_24_, lean_object* v_x_25_, lean_object* v_x_26_){
_start:
{
uint8_t v___x_27_; 
v___x_27_ = lean_nat_dec_eq(v_x_25_, v_x_26_);
return v___x_27_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock___boxed(lean_object* v_sb_28_, lean_object* v_x_29_, lean_object* v_x_30_){
_start:
{
uint8_t v_res_31_; lean_object* v_r_32_; 
v_res_31_ = lp_zhc__lean_Zhc_Model_instDecidableEqCiphertextBlock(v_sb_28_, v_x_29_, v_x_30_);
lean_dec(v_x_30_);
lean_dec(v_x_29_);
lean_dec_ref(v_sb_28_);
v_r_32_ = lean_box(v_res_31_);
return v_r_32_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__7(void){
_start:
{
lean_object* v___x_46_; lean_object* v___x_47_; 
v___x_46_ = lean_unsigned_to_nat(11u);
v___x_47_ = lean_nat_to_int(v___x_46_);
return v___x_47_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__9(void){
_start:
{
lean_object* v___x_49_; lean_object* v___x_50_; 
v___x_49_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__0));
v___x_50_ = lean_string_length(v___x_49_);
return v___x_50_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__10(void){
_start:
{
lean_object* v___x_51_; lean_object* v___x_52_; 
v___x_51_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__9, &lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__9_once, _init_lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__9);
v___x_52_ = lean_nat_to_int(v___x_51_);
return v___x_52_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg(lean_object* v_x_57_){
_start:
{
lean_object* v___x_58_; lean_object* v___x_59_; lean_object* v___x_60_; lean_object* v___x_61_; lean_object* v___x_62_; uint8_t v___x_63_; lean_object* v___x_64_; lean_object* v___x_65_; lean_object* v___x_66_; lean_object* v___x_67_; lean_object* v___x_68_; lean_object* v___x_69_; lean_object* v___x_70_; lean_object* v___x_71_; lean_object* v___x_72_; 
v___x_58_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__6));
v___x_59_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__7, &lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__7_once, _init_lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__7);
v___x_60_ = l_Nat_reprFast(v_x_57_);
v___x_61_ = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(v___x_61_, 0, v___x_60_);
v___x_62_ = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(v___x_62_, 0, v___x_59_);
lean_ctor_set(v___x_62_, 1, v___x_61_);
v___x_63_ = 0;
v___x_64_ = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(v___x_64_, 0, v___x_62_);
lean_ctor_set_uint8(v___x_64_, sizeof(void*)*1, v___x_63_);
v___x_65_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_65_, 0, v___x_58_);
lean_ctor_set(v___x_65_, 1, v___x_64_);
v___x_66_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__10, &lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__10_once, _init_lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__10);
v___x_67_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__11));
v___x_68_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_68_, 0, v___x_67_);
lean_ctor_set(v___x_68_, 1, v___x_65_);
v___x_69_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg___closed__12));
v___x_70_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_70_, 0, v___x_68_);
lean_ctor_set(v___x_70_, 1, v___x_69_);
v___x_71_ = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(v___x_71_, 0, v___x_66_);
lean_ctor_set(v___x_71_, 1, v___x_70_);
v___x_72_ = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(v___x_72_, 0, v___x_71_);
lean_ctor_set_uint8(v___x_72_, sizeof(void*)*1, v___x_63_);
return v___x_72_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr(lean_object* v_sb_73_, lean_object* v_x_74_, lean_object* v_prec_75_){
_start:
{
lean_object* v___x_76_; 
v___x_76_ = lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___redArg(v_x_74_);
return v___x_76_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___boxed(lean_object* v_sb_77_, lean_object* v_x_78_, lean_object* v_prec_79_){
_start:
{
lean_object* v_res_80_; 
v_res_80_ = lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr(v_sb_77_, v_x_78_, v_prec_79_);
lean_dec(v_prec_79_);
lean_dec_ref(v_sb_77_);
return v_res_80_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprCiphertextBlock(lean_object* v_sb_81_){
_start:
{
lean_object* v___x_82_; 
v___x_82_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Model_instReprCiphertextBlock_repr___boxed), 3, 1);
lean_closure_set(v___x_82_, 0, v_sb_81_);
return v___x_82_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object* v_sb_83_, lean_object* v_v_84_){
_start:
{
lean_object* v___x_85_; lean_object* v___x_86_; lean_object* v___x_87_; lean_object* v___x_88_; 
v___x_85_ = lean_unsigned_to_nat(2u);
v___x_86_ = lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(v_sb_83_);
v___x_87_ = lean_nat_pow(v___x_85_, v___x_86_);
lean_dec(v___x_86_);
v___x_88_ = lean_nat_mod(v_v_84_, v___x_87_);
lean_dec(v___x_87_);
return v___x_88_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat___boxed(lean_object* v_sb_89_, lean_object* v_v_90_){
_start:
{
lean_object* v_res_91_; 
v_res_91_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_89_, v_v_90_);
lean_dec(v_v_90_);
lean_dec_ref(v_sb_89_);
return v_res_91_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat_x3f(lean_object* v_sb_92_, lean_object* v_v_93_){
_start:
{
lean_object* v___x_94_; lean_object* v___x_95_; lean_object* v___x_96_; uint8_t v___x_97_; 
v___x_94_ = lean_unsigned_to_nat(2u);
v___x_95_ = lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(v_sb_92_);
v___x_96_ = lean_nat_pow(v___x_94_, v___x_95_);
lean_dec(v___x_95_);
v___x_97_ = lean_nat_dec_lt(v_v_93_, v___x_96_);
lean_dec(v___x_96_);
if (v___x_97_ == 0)
{
lean_object* v___x_98_; 
v___x_98_ = lean_box(0);
return v___x_98_;
}
else
{
lean_object* v___x_99_; lean_object* v___x_100_; 
v___x_99_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_92_, v_v_93_);
v___x_100_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_100_, 0, v___x_99_);
return v___x_100_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat_x3f___boxed(lean_object* v_sb_101_, lean_object* v_v_102_){
_start:
{
lean_object* v_res_103_; 
v_res_103_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat_x3f(v_sb_101_, v_v_102_);
lean_dec(v_v_102_);
lean_dec_ref(v_sb_101_);
return v_res_103_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat___redArg(lean_object* v_self_104_){
_start:
{
lean_inc(v_self_104_);
return v_self_104_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat___redArg___boxed(lean_object* v_self_105_){
_start:
{
lean_object* v_res_106_; 
v_res_106_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat___redArg(v_self_105_);
lean_dec(v_self_105_);
return v_res_106_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat(lean_object* v_sb_107_, lean_object* v_self_108_){
_start:
{
lean_inc(v_self_108_);
return v_self_108_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat___boxed(lean_object* v_sb_109_, lean_object* v_self_110_){
_start:
{
lean_object* v_res_111_; 
v_res_111_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toCompleteNat(v_sb_109_, v_self_110_);
lean_dec(v_self_110_);
lean_dec_ref(v_sb_109_);
return v_res_111_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toMessageNat(lean_object* v_sb_112_, lean_object* v_self_113_){
_start:
{
lean_object* v_messageSize_114_; lean_object* v___x_115_; lean_object* v___x_116_; lean_object* v___x_117_; 
v_messageSize_114_ = lean_ctor_get(v_sb_112_, 1);
v___x_115_ = lean_unsigned_to_nat(2u);
v___x_116_ = lean_nat_pow(v___x_115_, v_messageSize_114_);
v___x_117_ = lean_nat_mod(v_self_113_, v___x_116_);
lean_dec(v___x_116_);
return v___x_117_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toMessageNat___boxed(lean_object* v_sb_118_, lean_object* v_self_119_){
_start:
{
lean_object* v_res_120_; 
v_res_120_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toMessageNat(v_sb_118_, v_self_119_);
lean_dec(v_self_119_);
lean_dec_ref(v_sb_118_);
return v_res_120_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCarryNat(lean_object* v_sb_121_, lean_object* v_self_122_){
_start:
{
lean_object* v_carrySize_123_; lean_object* v_messageSize_124_; lean_object* v___x_125_; lean_object* v___x_126_; lean_object* v___x_127_; lean_object* v___x_128_; lean_object* v___x_129_; 
v_carrySize_123_ = lean_ctor_get(v_sb_121_, 0);
v_messageSize_124_ = lean_ctor_get(v_sb_121_, 1);
v___x_125_ = lean_unsigned_to_nat(2u);
v___x_126_ = lean_nat_pow(v___x_125_, v_messageSize_124_);
v___x_127_ = lean_nat_div(v_self_122_, v___x_126_);
lean_dec(v___x_126_);
v___x_128_ = lean_nat_pow(v___x_125_, v_carrySize_123_);
v___x_129_ = lean_nat_mod(v___x_127_, v___x_128_);
lean_dec(v___x_128_);
lean_dec(v___x_127_);
return v___x_129_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toCarryNat___boxed(lean_object* v_sb_130_, lean_object* v_self_131_){
_start:
{
lean_object* v_res_132_; 
v_res_132_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toCarryNat(v_sb_130_, v_self_131_);
lean_dec(v_self_131_);
lean_dec_ref(v_sb_130_);
return v_res_132_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toPaddingNat(lean_object* v_sb_133_, lean_object* v_self_134_){
_start:
{
lean_object* v___x_135_; lean_object* v___x_136_; lean_object* v___x_137_; lean_object* v___x_138_; 
v___x_135_ = lean_unsigned_to_nat(2u);
v___x_136_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_sb_133_);
v___x_137_ = lean_nat_pow(v___x_135_, v___x_136_);
lean_dec(v___x_136_);
v___x_138_ = lean_nat_div(v_self_134_, v___x_137_);
lean_dec(v___x_137_);
return v___x_138_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toPaddingNat___boxed(lean_object* v_sb_139_, lean_object* v_self_140_){
_start:
{
lean_object* v_res_141_; 
v_res_141_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toPaddingNat(v_sb_139_, v_self_140_);
lean_dec(v_self_140_);
lean_dec_ref(v_sb_139_);
return v_res_141_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toDataNat(lean_object* v_sb_142_, lean_object* v_self_143_){
_start:
{
lean_object* v___x_144_; lean_object* v___x_145_; lean_object* v___x_146_; lean_object* v___x_147_; 
v___x_144_ = lean_unsigned_to_nat(2u);
v___x_145_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_sb_142_);
v___x_146_ = lean_nat_pow(v___x_144_, v___x_145_);
lean_dec(v___x_145_);
v___x_147_ = lean_nat_mod(v_self_143_, v___x_146_);
lean_dec(v___x_146_);
return v___x_147_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toDataNat___boxed(lean_object* v_sb_148_, lean_object* v_self_149_){
_start:
{
lean_object* v_res_150_; 
v_res_150_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toDataNat(v_sb_148_, v_self_149_);
lean_dec(v_self_149_);
lean_dec_ref(v_sb_148_);
return v_res_150_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage(lean_object* v_sb_151_, lean_object* v_self_152_){
_start:
{
lean_object* v___x_153_; lean_object* v___x_154_; 
v___x_153_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toMessageNat(v_sb_151_, v_self_152_);
v___x_154_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_151_, v___x_153_);
lean_dec(v___x_153_);
return v___x_154_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage___boxed(lean_object* v_sb_155_, lean_object* v_self_156_){
_start:
{
lean_object* v_res_157_; 
v_res_157_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage(v_sb_155_, v_self_156_);
lean_dec(v_self_156_);
lean_dec_ref(v_sb_155_);
return v_res_157_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskCarry(lean_object* v_sb_158_, lean_object* v_self_159_){
_start:
{
lean_object* v_messageSize_160_; lean_object* v___x_161_; lean_object* v___x_162_; lean_object* v___x_163_; lean_object* v___x_164_; lean_object* v___x_165_; 
v_messageSize_160_ = lean_ctor_get(v_sb_158_, 1);
v___x_161_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toCarryNat(v_sb_158_, v_self_159_);
v___x_162_ = lean_unsigned_to_nat(2u);
v___x_163_ = lean_nat_pow(v___x_162_, v_messageSize_160_);
v___x_164_ = lean_nat_mul(v___x_161_, v___x_163_);
lean_dec(v___x_163_);
lean_dec(v___x_161_);
v___x_165_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_158_, v___x_164_);
lean_dec(v___x_164_);
return v___x_165_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskCarry___boxed(lean_object* v_sb_166_, lean_object* v_self_167_){
_start:
{
lean_object* v_res_168_; 
v_res_168_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskCarry(v_sb_166_, v_self_167_);
lean_dec(v_self_167_);
lean_dec_ref(v_sb_166_);
return v_res_168_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMoveCarryToMessage(lean_object* v_sb_169_, lean_object* v_self_170_){
_start:
{
lean_object* v___x_171_; lean_object* v___x_172_; 
v___x_171_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toCarryNat(v_sb_169_, v_self_170_);
v___x_172_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_169_, v___x_171_);
lean_dec(v___x_171_);
return v___x_172_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMoveCarryToMessage___boxed(lean_object* v_sb_173_, lean_object* v_self_174_){
_start:
{
lean_object* v_res_175_; 
v_res_175_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doMoveCarryToMessage(v_sb_173_, v_self_174_);
lean_dec(v_self_174_);
lean_dec_ref(v_sb_173_);
return v_res_175_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doNeg(lean_object* v_sb_176_, lean_object* v_self_177_){
_start:
{
lean_object* v___x_178_; lean_object* v___x_179_; lean_object* v___x_180_; lean_object* v___x_181_; lean_object* v___x_182_; 
v___x_178_ = lean_unsigned_to_nat(2u);
v___x_179_ = lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(v_sb_176_);
v___x_180_ = lean_nat_pow(v___x_178_, v___x_179_);
lean_dec(v___x_179_);
v___x_181_ = lean_nat_sub(v___x_180_, v_self_177_);
lean_dec(v___x_180_);
v___x_182_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_176_, v___x_181_);
lean_dec(v___x_181_);
return v___x_182_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doNeg___boxed(lean_object* v_sb_183_, lean_object* v_self_184_){
_start:
{
lean_object* v_res_185_; 
v_res_185_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doNeg(v_sb_183_, v_self_184_);
lean_dec(v_self_184_);
lean_dec_ref(v_sb_183_);
return v_res_185_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat(lean_object* v_sb_186_, lean_object* v_x_187_){
_start:
{
if (lean_obj_tag(v_x_187_) == 0)
{
lean_object* v___x_188_; 
v___x_188_ = lean_unsigned_to_nat(0u);
return v___x_188_;
}
else
{
lean_object* v_head_189_; lean_object* v_tail_190_; lean_object* v_messageSize_191_; lean_object* v___x_192_; lean_object* v___x_193_; lean_object* v___x_194_; lean_object* v___x_195_; lean_object* v___x_196_; lean_object* v___x_197_; 
v_head_189_ = lean_ctor_get(v_x_187_, 0);
v_tail_190_ = lean_ctor_get(v_x_187_, 1);
v_messageSize_191_ = lean_ctor_get(v_sb_186_, 1);
v___x_192_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toMessageNat(v_sb_186_, v_head_189_);
v___x_193_ = lean_unsigned_to_nat(2u);
v___x_194_ = lean_nat_pow(v___x_193_, v_messageSize_191_);
v___x_195_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat(v_sb_186_, v_tail_190_);
v___x_196_ = lean_nat_mul(v___x_194_, v___x_195_);
lean_dec(v___x_195_);
lean_dec(v___x_194_);
v___x_197_ = lean_nat_add(v___x_192_, v___x_196_);
lean_dec(v___x_196_);
lean_dec(v___x_192_);
return v___x_197_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat___boxed(lean_object* v_sb_198_, lean_object* v_x_199_){
_start:
{
lean_object* v_res_200_; 
v_res_200_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat(v_sb_198_, v_x_199_);
lean_dec(v_x_199_);
lean_dec_ref(v_sb_198_);
return v_res_200_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(lean_object* v_sb_201_, lean_object* v_n_202_, lean_object* v_v_203_){
_start:
{
lean_object* v_zero_204_; uint8_t v_isZero_205_; 
v_zero_204_ = lean_unsigned_to_nat(0u);
v_isZero_205_ = lean_nat_dec_eq(v_n_202_, v_zero_204_);
if (v_isZero_205_ == 1)
{
lean_object* v___x_206_; 
v___x_206_ = lean_box(0);
return v___x_206_;
}
else
{
lean_object* v_messageSize_207_; lean_object* v_one_208_; lean_object* v_n_209_; lean_object* v___x_210_; lean_object* v___x_211_; lean_object* v___x_212_; lean_object* v___x_213_; lean_object* v___x_214_; lean_object* v___x_215_; lean_object* v___x_216_; 
v_messageSize_207_ = lean_ctor_get(v_sb_201_, 1);
v_one_208_ = lean_unsigned_to_nat(1u);
v_n_209_ = lean_nat_sub(v_n_202_, v_one_208_);
v___x_210_ = lean_unsigned_to_nat(2u);
v___x_211_ = lean_nat_pow(v___x_210_, v_messageSize_207_);
v___x_212_ = lean_nat_mod(v_v_203_, v___x_211_);
v___x_213_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_201_, v___x_212_);
lean_dec(v___x_212_);
v___x_214_ = lean_nat_div(v_v_203_, v___x_211_);
lean_dec(v___x_211_);
v___x_215_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v_sb_201_, v_n_209_, v___x_214_);
lean_dec(v___x_214_);
lean_dec(v_n_209_);
v___x_216_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v___x_216_, 0, v___x_213_);
lean_ctor_set(v___x_216_, 1, v___x_215_);
return v___x_216_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits___boxed(lean_object* v_sb_217_, lean_object* v_n_218_, lean_object* v_v_219_){
_start:
{
lean_object* v_res_220_; 
v_res_220_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v_sb_217_, v_n_218_, v_v_219_);
lean_dec(v_v_219_);
lean_dec(v_n_218_);
lean_dec_ref(v_sb_217_);
return v_res_220_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter___redArg(lean_object* v_n_221_, lean_object* v_h__1_222_, lean_object* v_h__2_223_){
_start:
{
lean_object* v_zero_224_; uint8_t v_isZero_225_; 
v_zero_224_ = lean_unsigned_to_nat(0u);
v_isZero_225_ = lean_nat_dec_eq(v_n_221_, v_zero_224_);
if (v_isZero_225_ == 1)
{
lean_object* v___x_226_; lean_object* v___x_227_; 
lean_dec(v_h__2_223_);
v___x_226_ = lean_box(0);
v___x_227_ = lean_apply_1(v_h__1_222_, v___x_226_);
return v___x_227_;
}
else
{
lean_object* v_one_228_; lean_object* v_n_229_; lean_object* v___x_230_; 
lean_dec(v_h__1_222_);
v_one_228_ = lean_unsigned_to_nat(1u);
v_n_229_ = lean_nat_sub(v_n_221_, v_one_228_);
v___x_230_ = lean_apply_1(v_h__2_223_, v_n_229_);
return v___x_230_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter___redArg___boxed(lean_object* v_n_231_, lean_object* v_h__1_232_, lean_object* v_h__2_233_){
_start:
{
lean_object* v_res_234_; 
v_res_234_ = lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter___redArg(v_n_231_, v_h__1_232_, v_h__2_233_);
lean_dec(v_n_231_);
return v_res_234_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter(lean_object* v_motive_235_, lean_object* v_n_236_, lean_object* v_h__1_237_, lean_object* v_h__2_238_){
_start:
{
lean_object* v_zero_239_; uint8_t v_isZero_240_; 
v_zero_239_ = lean_unsigned_to_nat(0u);
v_isZero_240_ = lean_nat_dec_eq(v_n_236_, v_zero_239_);
if (v_isZero_240_ == 1)
{
lean_object* v___x_241_; lean_object* v___x_242_; 
lean_dec(v_h__2_238_);
v___x_241_ = lean_box(0);
v___x_242_ = lean_apply_1(v_h__1_237_, v___x_241_);
return v___x_242_;
}
else
{
lean_object* v_one_243_; lean_object* v_n_244_; lean_object* v___x_245_; 
lean_dec(v_h__1_237_);
v_one_243_ = lean_unsigned_to_nat(1u);
v_n_244_ = lean_nat_sub(v_n_236_, v_one_243_);
v___x_245_ = lean_apply_1(v_h__2_238_, v_n_244_);
return v___x_245_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter___boxed(lean_object* v_motive_246_, lean_object* v_n_247_, lean_object* v_h__1_248_, lean_object* v_h__2_249_){
_start:
{
lean_object* v_res_250_; 
v_res_250_ = lp_zhc__lean___private_Zhc_Model_CiphertextBlock_0__Zhc_Model_CiphertextBlock_digits_match__1_splitter(v_motive_246_, v_n_247_, v_h__1_248_, v_h__2_249_);
lean_dec(v_n_247_);
return v_res_250_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Util_Theorems(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Model_CiphertextBlock(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Model_SpecBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Util_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
