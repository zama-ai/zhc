// Lean compiler output
// Module: Zhc.Model.SpecInteger
// Imports: public import Init public meta import Init public import Zhc.Model.SpecBlock
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
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* lean_nat_to_int(lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
lean_object* lean_string_length(lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* lean_nat_div(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger___boxed(lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 11, .m_capacity = 11, .m_length = 10, .m_data = "blockCount"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__1_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__1_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__2_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__2_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__3 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__3_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__4 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__4_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__4_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__5 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__5_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__3_value),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__5_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__6 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__6_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__7;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__8 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__8_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__9_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__9;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__10_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__10;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__0_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__11 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__11_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__12_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__8_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__12 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__12_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger_default(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger_default___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_intSize(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_intSize___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_ofIntSize_x3f(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_ofIntSize_x3f___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq___redArg(lean_object* v_x_1_, lean_object* v_x_2_){
_start:
{
uint8_t v___x_3_; 
v___x_3_ = lean_nat_dec_eq(v_x_1_, v_x_2_);
return v___x_3_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq___redArg___boxed(lean_object* v_x_4_, lean_object* v_x_5_){
_start:
{
uint8_t v_res_6_; lean_object* v_r_7_; 
v_res_6_ = lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq___redArg(v_x_4_, v_x_5_);
lean_dec(v_x_5_);
lean_dec(v_x_4_);
v_r_7_ = lean_box(v_res_6_);
return v_r_7_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq(lean_object* v_sb_8_, lean_object* v_x_9_, lean_object* v_x_10_){
_start:
{
uint8_t v___x_11_; 
v___x_11_ = lean_nat_dec_eq(v_x_9_, v_x_10_);
return v___x_11_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq___boxed(lean_object* v_sb_12_, lean_object* v_x_13_, lean_object* v_x_14_){
_start:
{
uint8_t v_res_15_; lean_object* v_r_16_; 
v_res_15_ = lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger_decEq(v_sb_12_, v_x_13_, v_x_14_);
lean_dec(v_x_14_);
lean_dec(v_x_13_);
lean_dec_ref(v_sb_12_);
v_r_16_ = lean_box(v_res_15_);
return v_r_16_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger___redArg(lean_object* v_x_17_, lean_object* v_x_18_){
_start:
{
uint8_t v___x_19_; 
v___x_19_ = lean_nat_dec_eq(v_x_17_, v_x_18_);
return v___x_19_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger___redArg___boxed(lean_object* v_x_20_, lean_object* v_x_21_){
_start:
{
uint8_t v_res_22_; lean_object* v_r_23_; 
v_res_22_ = lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger___redArg(v_x_20_, v_x_21_);
lean_dec(v_x_21_);
lean_dec(v_x_20_);
v_r_23_ = lean_box(v_res_22_);
return v_r_23_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger(lean_object* v_sb_24_, lean_object* v_x_25_, lean_object* v_x_26_){
_start:
{
uint8_t v___x_27_; 
v___x_27_ = lean_nat_dec_eq(v_x_25_, v_x_26_);
return v___x_27_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger___boxed(lean_object* v_sb_28_, lean_object* v_x_29_, lean_object* v_x_30_){
_start:
{
uint8_t v_res_31_; lean_object* v_r_32_; 
v_res_31_ = lp_zhc__lean_Zhc_Model_instDecidableEqSpecInteger(v_sb_28_, v_x_29_, v_x_30_);
lean_dec(v_x_30_);
lean_dec(v_x_29_);
lean_dec_ref(v_sb_28_);
v_r_32_ = lean_box(v_res_31_);
return v_r_32_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__7(void){
_start:
{
lean_object* v___x_46_; lean_object* v___x_47_; 
v___x_46_ = lean_unsigned_to_nat(14u);
v___x_47_ = lean_nat_to_int(v___x_46_);
return v___x_47_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__9(void){
_start:
{
lean_object* v___x_49_; lean_object* v___x_50_; 
v___x_49_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__0));
v___x_50_ = lean_string_length(v___x_49_);
return v___x_50_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__10(void){
_start:
{
lean_object* v___x_51_; lean_object* v___x_52_; 
v___x_51_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__9, &lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__9_once, _init_lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__9);
v___x_52_ = lean_nat_to_int(v___x_51_);
return v___x_52_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg(lean_object* v_x_57_){
_start:
{
lean_object* v___x_58_; lean_object* v___x_59_; lean_object* v___x_60_; lean_object* v___x_61_; lean_object* v___x_62_; uint8_t v___x_63_; lean_object* v___x_64_; lean_object* v___x_65_; lean_object* v___x_66_; lean_object* v___x_67_; lean_object* v___x_68_; lean_object* v___x_69_; lean_object* v___x_70_; lean_object* v___x_71_; lean_object* v___x_72_; 
v___x_58_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__6));
v___x_59_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__7, &lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__7_once, _init_lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__7);
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
v___x_66_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__10, &lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__10_once, _init_lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__10);
v___x_67_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__11));
v___x_68_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_68_, 0, v___x_67_);
lean_ctor_set(v___x_68_, 1, v___x_65_);
v___x_69_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg___closed__12));
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr(lean_object* v_sb_73_, lean_object* v_x_74_, lean_object* v_prec_75_){
_start:
{
lean_object* v___x_76_; 
v___x_76_ = lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___redArg(v_x_74_);
return v___x_76_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___boxed(lean_object* v_sb_77_, lean_object* v_x_78_, lean_object* v_prec_79_){
_start:
{
lean_object* v_res_80_; 
v_res_80_ = lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr(v_sb_77_, v_x_78_, v_prec_79_);
lean_dec(v_prec_79_);
lean_dec_ref(v_sb_77_);
return v_res_80_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecInteger(lean_object* v_sb_81_){
_start:
{
lean_object* v___x_82_; 
v___x_82_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Model_instReprSpecInteger_repr___boxed), 3, 1);
lean_closure_set(v___x_82_, 0, v_sb_81_);
return v___x_82_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger_default(lean_object* v_sb_83_){
_start:
{
lean_object* v___x_84_; 
v___x_84_ = lean_unsigned_to_nat(0u);
return v___x_84_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger_default___boxed(lean_object* v_sb_85_){
_start:
{
lean_object* v_res_86_; 
v_res_86_ = lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger_default(v_sb_85_);
lean_dec_ref(v_sb_85_);
return v_res_86_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger(lean_object* v_a_87_){
_start:
{
lean_object* v___x_88_; 
v___x_88_ = lean_unsigned_to_nat(0u);
return v___x_88_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger___boxed(lean_object* v_a_89_){
_start:
{
lean_object* v_res_90_; 
v_res_90_ = lp_zhc__lean_Zhc_Model_instInhabitedSpecInteger(v_a_89_);
lean_dec_ref(v_a_89_);
return v_res_90_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_intSize(lean_object* v_sb_91_, lean_object* v_self_92_){
_start:
{
lean_object* v_messageSize_93_; lean_object* v___x_94_; 
v_messageSize_93_ = lean_ctor_get(v_sb_91_, 1);
v___x_94_ = lean_nat_mul(v_self_92_, v_messageSize_93_);
return v___x_94_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_intSize___boxed(lean_object* v_sb_95_, lean_object* v_self_96_){
_start:
{
lean_object* v_res_97_; 
v_res_97_ = lp_zhc__lean_Zhc_Model_SpecInteger_intSize(v_sb_95_, v_self_96_);
lean_dec(v_self_96_);
lean_dec_ref(v_sb_95_);
return v_res_97_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_ofIntSize_x3f(lean_object* v_sb_98_, lean_object* v_intSize_99_){
_start:
{
lean_object* v_messageSize_100_; lean_object* v___x_101_; lean_object* v___x_102_; uint8_t v___x_103_; 
v_messageSize_100_ = lean_ctor_get(v_sb_98_, 1);
v___x_101_ = lean_nat_mod(v_intSize_99_, v_messageSize_100_);
v___x_102_ = lean_unsigned_to_nat(0u);
v___x_103_ = lean_nat_dec_eq(v___x_101_, v___x_102_);
lean_dec(v___x_101_);
if (v___x_103_ == 0)
{
lean_object* v___x_104_; 
v___x_104_ = lean_box(0);
return v___x_104_;
}
else
{
lean_object* v___x_105_; lean_object* v___x_106_; 
v___x_105_ = lean_nat_div(v_intSize_99_, v_messageSize_100_);
v___x_106_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_106_, 0, v___x_105_);
return v___x_106_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecInteger_ofIntSize_x3f___boxed(lean_object* v_sb_107_, lean_object* v_intSize_108_){
_start:
{
lean_object* v_res_109_; 
v_res_109_ = lp_zhc__lean_Zhc_Model_SpecInteger_ofIntSize_x3f(v_sb_107_, v_intSize_108_);
lean_dec(v_intSize_108_);
lean_dec_ref(v_sb_107_);
return v_res_109_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Model_SpecInteger(uint8_t builtin) {
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
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
