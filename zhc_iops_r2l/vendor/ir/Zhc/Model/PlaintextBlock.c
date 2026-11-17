// Lean compiler output
// Module: Zhc.Model.PlaintextBlock
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
lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(lean_object*);
lean_object* lean_nat_pow(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* lean_nat_to_int(lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
lean_object* lean_string_length(lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
uint8_t lean_nat_dec_lt(lean_object*, lean_object*);
lean_object* lean_nat_div(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock___boxed(lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 8, .m_capacity = 8, .m_length = 7, .m_data = "storage"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__1_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__1_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__2_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__2_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__3 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__3_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__4 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__4_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__4_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__5 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__5_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__3_value),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__5_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__6 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__6_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__7;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__8 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__8_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__9_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__9;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__10_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__10;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__0_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__11 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__11_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__12_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__8_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__12 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__12_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat_x3f(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat_x3f___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toMessageNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toMessageNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_listToNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_listToNat___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_digits(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_digits___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter___redArg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq___redArg(lean_object* v_x_1_, lean_object* v_x_2_){
_start:
{
uint8_t v___x_3_; 
v___x_3_ = lean_nat_dec_eq(v_x_1_, v_x_2_);
return v___x_3_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq___redArg___boxed(lean_object* v_x_4_, lean_object* v_x_5_){
_start:
{
uint8_t v_res_6_; lean_object* v_r_7_; 
v_res_6_ = lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq___redArg(v_x_4_, v_x_5_);
lean_dec(v_x_5_);
lean_dec(v_x_4_);
v_r_7_ = lean_box(v_res_6_);
return v_r_7_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq(lean_object* v_sb_8_, lean_object* v_x_9_, lean_object* v_x_10_){
_start:
{
uint8_t v___x_11_; 
v___x_11_ = lean_nat_dec_eq(v_x_9_, v_x_10_);
return v___x_11_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq___boxed(lean_object* v_sb_12_, lean_object* v_x_13_, lean_object* v_x_14_){
_start:
{
uint8_t v_res_15_; lean_object* v_r_16_; 
v_res_15_ = lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock_decEq(v_sb_12_, v_x_13_, v_x_14_);
lean_dec(v_x_14_);
lean_dec(v_x_13_);
lean_dec_ref(v_sb_12_);
v_r_16_ = lean_box(v_res_15_);
return v_r_16_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock___redArg(lean_object* v_x_17_, lean_object* v_x_18_){
_start:
{
uint8_t v___x_19_; 
v___x_19_ = lean_nat_dec_eq(v_x_17_, v_x_18_);
return v___x_19_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock___redArg___boxed(lean_object* v_x_20_, lean_object* v_x_21_){
_start:
{
uint8_t v_res_22_; lean_object* v_r_23_; 
v_res_22_ = lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock___redArg(v_x_20_, v_x_21_);
lean_dec(v_x_21_);
lean_dec(v_x_20_);
v_r_23_ = lean_box(v_res_22_);
return v_r_23_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock(lean_object* v_sb_24_, lean_object* v_x_25_, lean_object* v_x_26_){
_start:
{
uint8_t v___x_27_; 
v___x_27_ = lean_nat_dec_eq(v_x_25_, v_x_26_);
return v___x_27_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock___boxed(lean_object* v_sb_28_, lean_object* v_x_29_, lean_object* v_x_30_){
_start:
{
uint8_t v_res_31_; lean_object* v_r_32_; 
v_res_31_ = lp_zhc__lean_Zhc_Model_instDecidableEqPlaintextBlock(v_sb_28_, v_x_29_, v_x_30_);
lean_dec(v_x_30_);
lean_dec(v_x_29_);
lean_dec_ref(v_sb_28_);
v_r_32_ = lean_box(v_res_31_);
return v_r_32_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__7(void){
_start:
{
lean_object* v___x_46_; lean_object* v___x_47_; 
v___x_46_ = lean_unsigned_to_nat(11u);
v___x_47_ = lean_nat_to_int(v___x_46_);
return v___x_47_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__9(void){
_start:
{
lean_object* v___x_49_; lean_object* v___x_50_; 
v___x_49_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__0));
v___x_50_ = lean_string_length(v___x_49_);
return v___x_50_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__10(void){
_start:
{
lean_object* v___x_51_; lean_object* v___x_52_; 
v___x_51_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__9, &lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__9_once, _init_lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__9);
v___x_52_ = lean_nat_to_int(v___x_51_);
return v___x_52_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg(lean_object* v_x_57_){
_start:
{
lean_object* v___x_58_; lean_object* v___x_59_; lean_object* v___x_60_; lean_object* v___x_61_; lean_object* v___x_62_; uint8_t v___x_63_; lean_object* v___x_64_; lean_object* v___x_65_; lean_object* v___x_66_; lean_object* v___x_67_; lean_object* v___x_68_; lean_object* v___x_69_; lean_object* v___x_70_; lean_object* v___x_71_; lean_object* v___x_72_; 
v___x_58_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__6));
v___x_59_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__7, &lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__7_once, _init_lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__7);
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
v___x_66_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__10, &lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__10_once, _init_lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__10);
v___x_67_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__11));
v___x_68_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_68_, 0, v___x_67_);
lean_ctor_set(v___x_68_, 1, v___x_65_);
v___x_69_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg___closed__12));
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr(lean_object* v_sb_73_, lean_object* v_x_74_, lean_object* v_prec_75_){
_start:
{
lean_object* v___x_76_; 
v___x_76_ = lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___redArg(v_x_74_);
return v___x_76_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___boxed(lean_object* v_sb_77_, lean_object* v_x_78_, lean_object* v_prec_79_){
_start:
{
lean_object* v_res_80_; 
v_res_80_ = lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr(v_sb_77_, v_x_78_, v_prec_79_);
lean_dec(v_prec_79_);
lean_dec_ref(v_sb_77_);
return v_res_80_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprPlaintextBlock(lean_object* v_sb_81_){
_start:
{
lean_object* v___x_82_; 
v___x_82_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Model_instReprPlaintextBlock_repr___boxed), 3, 1);
lean_closure_set(v___x_82_, 0, v_sb_81_);
return v___x_82_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(lean_object* v_sb_83_, lean_object* v_v_84_){
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat___boxed(lean_object* v_sb_89_, lean_object* v_v_90_){
_start:
{
lean_object* v_res_91_; 
v_res_91_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(v_sb_89_, v_v_90_);
lean_dec(v_v_90_);
lean_dec_ref(v_sb_89_);
return v_res_91_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat_x3f(lean_object* v_sb_92_, lean_object* v_v_93_){
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
v___x_99_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(v_sb_92_, v_v_93_);
v___x_100_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_100_, 0, v___x_99_);
return v___x_100_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat_x3f___boxed(lean_object* v_sb_101_, lean_object* v_v_102_){
_start:
{
lean_object* v_res_103_; 
v_res_103_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat_x3f(v_sb_101_, v_v_102_);
lean_dec(v_v_102_);
lean_dec_ref(v_sb_101_);
return v_res_103_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat___redArg(lean_object* v_self_104_){
_start:
{
lean_inc(v_self_104_);
return v_self_104_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat___redArg___boxed(lean_object* v_self_105_){
_start:
{
lean_object* v_res_106_; 
v_res_106_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat___redArg(v_self_105_);
lean_dec(v_self_105_);
return v_res_106_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat(lean_object* v_sb_107_, lean_object* v_self_108_){
_start:
{
lean_inc(v_self_108_);
return v_self_108_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat___boxed(lean_object* v_sb_109_, lean_object* v_self_110_){
_start:
{
lean_object* v_res_111_; 
v_res_111_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_toCompleteNat(v_sb_109_, v_self_110_);
lean_dec(v_self_110_);
lean_dec_ref(v_sb_109_);
return v_res_111_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toMessageNat(lean_object* v_sb_112_, lean_object* v_self_113_){
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_toMessageNat___boxed(lean_object* v_sb_118_, lean_object* v_self_119_){
_start:
{
lean_object* v_res_120_; 
v_res_120_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_toMessageNat(v_sb_118_, v_self_119_);
lean_dec(v_self_119_);
lean_dec_ref(v_sb_118_);
return v_res_120_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_listToNat(lean_object* v_sb_121_, lean_object* v_x_122_){
_start:
{
if (lean_obj_tag(v_x_122_) == 0)
{
lean_object* v___x_123_; 
v___x_123_ = lean_unsigned_to_nat(0u);
return v___x_123_;
}
else
{
lean_object* v_head_124_; lean_object* v_tail_125_; lean_object* v_messageSize_126_; lean_object* v___x_127_; lean_object* v___x_128_; lean_object* v___x_129_; lean_object* v___x_130_; lean_object* v___x_131_; lean_object* v___x_132_; 
v_head_124_ = lean_ctor_get(v_x_122_, 0);
v_tail_125_ = lean_ctor_get(v_x_122_, 1);
v_messageSize_126_ = lean_ctor_get(v_sb_121_, 1);
v___x_127_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_toMessageNat(v_sb_121_, v_head_124_);
v___x_128_ = lean_unsigned_to_nat(2u);
v___x_129_ = lean_nat_pow(v___x_128_, v_messageSize_126_);
v___x_130_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_listToNat(v_sb_121_, v_tail_125_);
v___x_131_ = lean_nat_mul(v___x_129_, v___x_130_);
lean_dec(v___x_130_);
lean_dec(v___x_129_);
v___x_132_ = lean_nat_add(v___x_127_, v___x_131_);
lean_dec(v___x_131_);
lean_dec(v___x_127_);
return v___x_132_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_listToNat___boxed(lean_object* v_sb_133_, lean_object* v_x_134_){
_start:
{
lean_object* v_res_135_; 
v_res_135_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_listToNat(v_sb_133_, v_x_134_);
lean_dec(v_x_134_);
lean_dec_ref(v_sb_133_);
return v_res_135_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_digits(lean_object* v_sb_136_, lean_object* v_n_137_, lean_object* v_v_138_){
_start:
{
lean_object* v_zero_139_; uint8_t v_isZero_140_; 
v_zero_139_ = lean_unsigned_to_nat(0u);
v_isZero_140_ = lean_nat_dec_eq(v_n_137_, v_zero_139_);
if (v_isZero_140_ == 1)
{
lean_object* v___x_141_; 
v___x_141_ = lean_box(0);
return v___x_141_;
}
else
{
lean_object* v_messageSize_142_; lean_object* v_one_143_; lean_object* v_n_144_; lean_object* v___x_145_; lean_object* v___x_146_; lean_object* v___x_147_; lean_object* v___x_148_; lean_object* v___x_149_; lean_object* v___x_150_; lean_object* v___x_151_; 
v_messageSize_142_ = lean_ctor_get(v_sb_136_, 1);
v_one_143_ = lean_unsigned_to_nat(1u);
v_n_144_ = lean_nat_sub(v_n_137_, v_one_143_);
v___x_145_ = lean_unsigned_to_nat(2u);
v___x_146_ = lean_nat_pow(v___x_145_, v_messageSize_142_);
v___x_147_ = lean_nat_mod(v_v_138_, v___x_146_);
v___x_148_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(v_sb_136_, v___x_147_);
lean_dec(v___x_147_);
v___x_149_ = lean_nat_div(v_v_138_, v___x_146_);
lean_dec(v___x_146_);
v___x_150_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_digits(v_sb_136_, v_n_144_, v___x_149_);
lean_dec(v___x_149_);
lean_dec(v_n_144_);
v___x_151_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v___x_151_, 0, v___x_148_);
lean_ctor_set(v___x_151_, 1, v___x_150_);
return v___x_151_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_digits___boxed(lean_object* v_sb_152_, lean_object* v_n_153_, lean_object* v_v_154_){
_start:
{
lean_object* v_res_155_; 
v_res_155_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_digits(v_sb_152_, v_n_153_, v_v_154_);
lean_dec(v_v_154_);
lean_dec(v_n_153_);
lean_dec_ref(v_sb_152_);
return v_res_155_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter___redArg(lean_object* v_n_156_, lean_object* v_h__1_157_, lean_object* v_h__2_158_){
_start:
{
lean_object* v_zero_159_; uint8_t v_isZero_160_; 
v_zero_159_ = lean_unsigned_to_nat(0u);
v_isZero_160_ = lean_nat_dec_eq(v_n_156_, v_zero_159_);
if (v_isZero_160_ == 1)
{
lean_object* v___x_161_; lean_object* v___x_162_; 
lean_dec(v_h__2_158_);
v___x_161_ = lean_box(0);
v___x_162_ = lean_apply_1(v_h__1_157_, v___x_161_);
return v___x_162_;
}
else
{
lean_object* v_one_163_; lean_object* v_n_164_; lean_object* v___x_165_; 
lean_dec(v_h__1_157_);
v_one_163_ = lean_unsigned_to_nat(1u);
v_n_164_ = lean_nat_sub(v_n_156_, v_one_163_);
v___x_165_ = lean_apply_1(v_h__2_158_, v_n_164_);
return v___x_165_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter___redArg___boxed(lean_object* v_n_166_, lean_object* v_h__1_167_, lean_object* v_h__2_168_){
_start:
{
lean_object* v_res_169_; 
v_res_169_ = lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter___redArg(v_n_166_, v_h__1_167_, v_h__2_168_);
lean_dec(v_n_166_);
return v_res_169_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter(lean_object* v_motive_170_, lean_object* v_n_171_, lean_object* v_h__1_172_, lean_object* v_h__2_173_){
_start:
{
lean_object* v_zero_174_; uint8_t v_isZero_175_; 
v_zero_174_ = lean_unsigned_to_nat(0u);
v_isZero_175_ = lean_nat_dec_eq(v_n_171_, v_zero_174_);
if (v_isZero_175_ == 1)
{
lean_object* v___x_176_; lean_object* v___x_177_; 
lean_dec(v_h__2_173_);
v___x_176_ = lean_box(0);
v___x_177_ = lean_apply_1(v_h__1_172_, v___x_176_);
return v___x_177_;
}
else
{
lean_object* v_one_178_; lean_object* v_n_179_; lean_object* v___x_180_; 
lean_dec(v_h__1_172_);
v_one_178_ = lean_unsigned_to_nat(1u);
v_n_179_ = lean_nat_sub(v_n_171_, v_one_178_);
v___x_180_ = lean_apply_1(v_h__2_173_, v_n_179_);
return v___x_180_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter___boxed(lean_object* v_motive_181_, lean_object* v_n_182_, lean_object* v_h__1_183_, lean_object* v_h__2_184_){
_start:
{
lean_object* v_res_185_; 
v_res_185_ = lp_zhc__lean___private_Zhc_Model_PlaintextBlock_0__Zhc_Model_PlaintextBlock_digits_match__1_splitter(v_motive_181_, v_n_182_, v_h__1_183_, v_h__2_184_);
lean_dec(v_n_182_);
return v_res_185_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Util_Theorems(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Model_PlaintextBlock(uint8_t builtin) {
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
