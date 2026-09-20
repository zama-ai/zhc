// Lean compiler output
// Module: Zhc.BlockAlgebra.Model.Instance
// Imports: public import Init public meta import Init public import Zhc.BlockAlgebra.Class public import Zhc.Model.SpecBlock public import Zhc.Model.SpecInteger public import Zhc.Model.CiphertextBlock public import Zhc.Model.CiphertextInteger public import Zhc.Model.CiphertextBool public import Zhc.Model.PlaintextBlock public import Zhc.Model.PlaintextInteger public import Zhc.Model.Lookup public import Zhc.Model.Operators
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
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_subCt(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_mac(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_sub(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_subPt(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_add(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_shl(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_addPt(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_mulPt(lean_object*, lean_object*, lean_object*);
lean_object* l_List_get___redArg(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat___boxed(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat___boxed(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__0___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__1(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__1___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__2(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__2___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__3(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__3___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__4(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__4___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__5(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__5___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__6(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__6___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__7(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__7___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__8(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__8___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__9(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__9___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__11(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__11___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__10(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__10___boxed(lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___closed__0;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___boxed(lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__1;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___boxed(lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__1;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__2;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__3;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___boxed(lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__1;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__2;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__3;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__4_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__4;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__5_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__5;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__6_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__6;
static lean_once_cell_t lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__7;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___boxed(lean_object*, lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__9___boxed, .m_arity = 3, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__0_value;
static const lean_closure_object lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__11___boxed, .m_arity = 4, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__1_value;
static const lean_closure_object lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__10___boxed, .m_arity = 1, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__2_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtBoolId(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtBoolId___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtIntegerId(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtIntegerId___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanPtIntegerId(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanPtIntegerId___boxed(lean_object*, lean_object*);
static const lean_ctor_object lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(2) << 1) | 1)),((lean_object*)(((size_t)(2) << 1) | 1))}};
static const lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__0 = (const lean_object*)&lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__0_value;
static lean_once_cell_t lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__1;
static lean_once_cell_t lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__2;
static lean_once_cell_t lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__3;
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__0(lean_object* v_sb_1_, lean_object* v_x_2_){
_start:
{
lean_object* v___x_3_; lean_object* v___x_4_; 
v___x_3_ = lean_unsigned_to_nat(0u);
v___x_4_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v_sb_1_, v_x_2_, v___x_3_);
return v___x_4_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__0___boxed(lean_object* v_sb_5_, lean_object* v_x_6_){
_start:
{
lean_object* v_res_7_; 
v_res_7_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__0(v_sb_5_, v_x_6_);
lean_dec(v_x_6_);
lean_dec_ref(v_sb_5_);
return v_res_7_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__1(lean_object* v_sb_8_, lean_object* v_a_9_, lean_object* v_b_10_){
_start:
{
lean_object* v___x_11_; 
v___x_11_ = lp_zhc__lean_Zhc_Model_Operators_add(v_sb_8_, v_a_9_, v_b_10_);
return v___x_11_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__1___boxed(lean_object* v_sb_12_, lean_object* v_a_13_, lean_object* v_b_14_){
_start:
{
lean_object* v_res_15_; 
v_res_15_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__1(v_sb_12_, v_a_13_, v_b_14_);
lean_dec(v_b_14_);
lean_dec(v_a_13_);
lean_dec_ref(v_sb_12_);
return v_res_15_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__2(lean_object* v_sb_16_, lean_object* v_a_17_, lean_object* v_b_18_){
_start:
{
lean_object* v___x_19_; 
v___x_19_ = lp_zhc__lean_Zhc_Model_Operators_sub(v_sb_16_, v_a_17_, v_b_18_);
return v___x_19_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__2___boxed(lean_object* v_sb_20_, lean_object* v_a_21_, lean_object* v_b_22_){
_start:
{
lean_object* v_res_23_; 
v_res_23_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__2(v_sb_20_, v_a_21_, v_b_22_);
lean_dec(v_b_22_);
lean_dec(v_a_21_);
lean_dec_ref(v_sb_20_);
return v_res_23_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__3(lean_object* v_sb_24_, lean_object* v_a_25_, lean_object* v_amount_26_){
_start:
{
lean_object* v___x_27_; 
v___x_27_ = lp_zhc__lean_Zhc_Model_Operators_shl(v_sb_24_, v_a_25_, v_amount_26_);
return v___x_27_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__3___boxed(lean_object* v_sb_28_, lean_object* v_a_29_, lean_object* v_amount_30_){
_start:
{
lean_object* v_res_31_; 
v_res_31_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__3(v_sb_28_, v_a_29_, v_amount_30_);
lean_dec(v_amount_30_);
lean_dec(v_a_29_);
lean_dec_ref(v_sb_28_);
return v_res_31_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__4(lean_object* v_sb_32_, lean_object* v_a_33_, lean_object* v_mul_34_, lean_object* v_b_35_){
_start:
{
lean_object* v___x_36_; 
v___x_36_ = lp_zhc__lean_Zhc_Model_Operators_mac(v_sb_32_, v_a_33_, v_mul_34_, v_b_35_);
return v___x_36_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__4___boxed(lean_object* v_sb_37_, lean_object* v_a_38_, lean_object* v_mul_39_, lean_object* v_b_40_){
_start:
{
lean_object* v_res_41_; 
v_res_41_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__4(v_sb_37_, v_a_38_, v_mul_39_, v_b_40_);
lean_dec(v_b_40_);
lean_dec(v_mul_39_);
lean_dec(v_a_38_);
lean_dec_ref(v_sb_37_);
return v_res_41_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__5(lean_object* v_sb_42_, lean_object* v_a_43_, lean_object* v_b_44_){
_start:
{
lean_object* v___x_45_; 
v___x_45_ = lp_zhc__lean_Zhc_Model_Operators_addPt(v_sb_42_, v_a_43_, v_b_44_);
return v___x_45_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__5___boxed(lean_object* v_sb_46_, lean_object* v_a_47_, lean_object* v_b_48_){
_start:
{
lean_object* v_res_49_; 
v_res_49_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__5(v_sb_46_, v_a_47_, v_b_48_);
lean_dec(v_b_48_);
lean_dec(v_a_47_);
lean_dec_ref(v_sb_46_);
return v_res_49_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__6(lean_object* v_sb_50_, lean_object* v_a_51_, lean_object* v_b_52_){
_start:
{
lean_object* v___x_53_; 
v___x_53_ = lp_zhc__lean_Zhc_Model_Operators_subPt(v_sb_50_, v_a_51_, v_b_52_);
return v___x_53_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__6___boxed(lean_object* v_sb_54_, lean_object* v_a_55_, lean_object* v_b_56_){
_start:
{
lean_object* v_res_57_; 
v_res_57_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__6(v_sb_54_, v_a_55_, v_b_56_);
lean_dec(v_b_56_);
lean_dec(v_a_55_);
lean_dec_ref(v_sb_54_);
return v_res_57_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__7(lean_object* v_sb_58_, lean_object* v_a_59_, lean_object* v_b_60_){
_start:
{
lean_object* v___x_61_; 
v___x_61_ = lp_zhc__lean_Zhc_Model_Operators_subCt(v_sb_58_, v_a_59_, v_b_60_);
return v___x_61_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__7___boxed(lean_object* v_sb_62_, lean_object* v_a_63_, lean_object* v_b_64_){
_start:
{
lean_object* v_res_65_; 
v_res_65_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__7(v_sb_62_, v_a_63_, v_b_64_);
lean_dec(v_b_64_);
lean_dec(v_a_63_);
lean_dec_ref(v_sb_62_);
return v_res_65_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__8(lean_object* v_sb_66_, lean_object* v_a_67_, lean_object* v_b_68_){
_start:
{
lean_object* v___x_69_; 
v___x_69_ = lp_zhc__lean_Zhc_Model_Operators_mulPt(v_sb_66_, v_a_67_, v_b_68_);
return v___x_69_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__8___boxed(lean_object* v_sb_70_, lean_object* v_a_71_, lean_object* v_b_72_){
_start:
{
lean_object* v_res_73_; 
v_res_73_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__8(v_sb_70_, v_a_71_, v_b_72_);
lean_dec(v_b_72_);
lean_dec(v_a_71_);
lean_dec_ref(v_sb_70_);
return v_res_73_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__9(lean_object* v_si_74_, lean_object* v_x_75_, lean_object* v_i_76_){
_start:
{
lean_object* v___x_77_; 
v___x_77_ = l_List_get___redArg(v_x_75_, v_i_76_);
return v___x_77_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__9___boxed(lean_object* v_si_78_, lean_object* v_x_79_, lean_object* v_i_80_){
_start:
{
lean_object* v_res_81_; 
v_res_81_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__9(v_si_78_, v_x_79_, v_i_80_);
lean_dec(v_x_79_);
lean_dec(v_si_78_);
return v_res_81_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__11(lean_object* v_si_82_, lean_object* v_x_83_, lean_object* v_i_84_, lean_object* v_b_85_){
_start:
{
lean_object* v___x_86_; 
v___x_86_ = lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(v_x_83_, v_i_84_, v_b_85_);
return v___x_86_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__11___boxed(lean_object* v_si_87_, lean_object* v_x_88_, lean_object* v_i_89_, lean_object* v_b_90_){
_start:
{
lean_object* v_res_91_; 
v_res_91_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__11(v_si_87_, v_x_88_, v_i_89_, v_b_90_);
lean_dec(v_si_87_);
return v_res_91_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__10(lean_object* v_b_92_){
_start:
{
lean_inc(v_b_92_);
return v_b_92_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__10___boxed(lean_object* v_b_93_){
_start:
{
lean_object* v_res_94_; 
v_res_94_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__10(v_b_93_);
lean_dec(v_b_93_);
return v_res_94_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___closed__0(void){
_start:
{
lean_object* v___x_95_; lean_object* v___x_96_; lean_object* v___x_97_; 
v___x_95_ = lean_unsigned_to_nat(1u);
v___x_96_ = lean_unsigned_to_nat(0u);
v___x_97_ = lean_nat_mod(v___x_96_, v___x_95_);
return v___x_97_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13(lean_object* v_sb_98_, lean_object* v_lut_99_, lean_object* v_a_100_){
_start:
{
lean_object* v___x_101_; lean_object* v___x_102_; lean_object* v___x_103_; 
v___x_101_ = lean_unsigned_to_nat(0u);
v___x_102_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___closed__0);
v___x_103_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_98_, v___x_101_, v_lut_99_, v_a_100_, v___x_102_);
return v___x_103_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___boxed(lean_object* v_sb_104_, lean_object* v_lut_105_, lean_object* v_a_106_){
_start:
{
lean_object* v_res_107_; 
v_res_107_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13(v_sb_104_, v_lut_105_, v_a_106_);
lean_dec(v_a_106_);
lean_dec_ref(v_sb_104_);
return v_res_107_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__0(void){
_start:
{
lean_object* v___x_108_; lean_object* v___x_109_; lean_object* v___x_110_; 
v___x_108_ = lean_unsigned_to_nat(2u);
v___x_109_ = lean_unsigned_to_nat(0u);
v___x_110_ = lean_nat_mod(v___x_109_, v___x_108_);
return v___x_110_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__1(void){
_start:
{
lean_object* v___x_111_; lean_object* v___x_112_; lean_object* v___x_113_; 
v___x_111_ = lean_unsigned_to_nat(2u);
v___x_112_ = lean_unsigned_to_nat(1u);
v___x_113_ = lean_nat_mod(v___x_112_, v___x_111_);
return v___x_113_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12(lean_object* v_sb_114_, lean_object* v_lut_115_, lean_object* v_a_116_){
_start:
{
lean_object* v___x_117_; lean_object* v___x_118_; lean_object* v___x_119_; lean_object* v___x_120_; lean_object* v___x_121_; lean_object* v___x_122_; 
v___x_117_ = lean_unsigned_to_nat(1u);
v___x_118_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__0);
lean_inc_ref(v_lut_115_);
v___x_119_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_114_, v___x_117_, v_lut_115_, v_a_116_, v___x_118_);
v___x_120_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__1, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__1_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___closed__1);
v___x_121_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_114_, v___x_117_, v_lut_115_, v_a_116_, v___x_120_);
v___x_122_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_122_, 0, v___x_119_);
lean_ctor_set(v___x_122_, 1, v___x_121_);
return v___x_122_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___boxed(lean_object* v_sb_123_, lean_object* v_lut_124_, lean_object* v_a_125_){
_start:
{
lean_object* v_res_126_; 
v_res_126_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12(v_sb_123_, v_lut_124_, v_a_125_);
lean_dec(v_a_125_);
lean_dec_ref(v_sb_123_);
return v_res_126_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__0(void){
_start:
{
lean_object* v___x_127_; lean_object* v___x_128_; lean_object* v___x_129_; 
v___x_127_ = lean_unsigned_to_nat(4u);
v___x_128_ = lean_unsigned_to_nat(0u);
v___x_129_ = lean_nat_mod(v___x_128_, v___x_127_);
return v___x_129_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__1(void){
_start:
{
lean_object* v___x_130_; lean_object* v___x_131_; lean_object* v___x_132_; 
v___x_130_ = lean_unsigned_to_nat(4u);
v___x_131_ = lean_unsigned_to_nat(1u);
v___x_132_ = lean_nat_mod(v___x_131_, v___x_130_);
return v___x_132_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__2(void){
_start:
{
lean_object* v___x_133_; lean_object* v___x_134_; lean_object* v___x_135_; 
v___x_133_ = lean_unsigned_to_nat(4u);
v___x_134_ = lean_unsigned_to_nat(2u);
v___x_135_ = lean_nat_mod(v___x_134_, v___x_133_);
return v___x_135_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__3(void){
_start:
{
lean_object* v___x_136_; lean_object* v___x_137_; lean_object* v___x_138_; 
v___x_136_ = lean_unsigned_to_nat(4u);
v___x_137_ = lean_unsigned_to_nat(3u);
v___x_138_ = lean_nat_mod(v___x_137_, v___x_136_);
return v___x_138_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14(lean_object* v_sb_139_, lean_object* v_lut_140_, lean_object* v_a_141_){
_start:
{
lean_object* v___x_142_; lean_object* v___x_143_; lean_object* v___x_144_; lean_object* v___x_145_; lean_object* v___x_146_; lean_object* v___x_147_; lean_object* v___x_148_; lean_object* v___x_149_; lean_object* v___x_150_; lean_object* v___x_151_; lean_object* v___x_152_; lean_object* v___x_153_; 
v___x_142_ = lean_unsigned_to_nat(2u);
v___x_143_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__0);
lean_inc_ref_n(v_lut_140_, 3);
v___x_144_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_139_, v___x_142_, v_lut_140_, v_a_141_, v___x_143_);
v___x_145_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__1, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__1_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__1);
v___x_146_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_139_, v___x_142_, v_lut_140_, v_a_141_, v___x_145_);
v___x_147_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__2, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__2_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__2);
v___x_148_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_139_, v___x_142_, v_lut_140_, v_a_141_, v___x_147_);
v___x_149_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__3, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__3_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___closed__3);
v___x_150_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_139_, v___x_142_, v_lut_140_, v_a_141_, v___x_149_);
v___x_151_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_151_, 0, v___x_148_);
lean_ctor_set(v___x_151_, 1, v___x_150_);
v___x_152_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_152_, 0, v___x_146_);
lean_ctor_set(v___x_152_, 1, v___x_151_);
v___x_153_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_153_, 0, v___x_144_);
lean_ctor_set(v___x_153_, 1, v___x_152_);
return v___x_153_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___boxed(lean_object* v_sb_154_, lean_object* v_lut_155_, lean_object* v_a_156_){
_start:
{
lean_object* v_res_157_; 
v_res_157_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14(v_sb_154_, v_lut_155_, v_a_156_);
lean_dec(v_a_156_);
lean_dec_ref(v_sb_154_);
return v_res_157_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__0(void){
_start:
{
lean_object* v___x_158_; lean_object* v___x_159_; lean_object* v___x_160_; 
v___x_158_ = lean_unsigned_to_nat(8u);
v___x_159_ = lean_unsigned_to_nat(0u);
v___x_160_ = lean_nat_mod(v___x_159_, v___x_158_);
return v___x_160_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__1(void){
_start:
{
lean_object* v___x_161_; lean_object* v___x_162_; lean_object* v___x_163_; 
v___x_161_ = lean_unsigned_to_nat(8u);
v___x_162_ = lean_unsigned_to_nat(1u);
v___x_163_ = lean_nat_mod(v___x_162_, v___x_161_);
return v___x_163_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__2(void){
_start:
{
lean_object* v___x_164_; lean_object* v___x_165_; lean_object* v___x_166_; 
v___x_164_ = lean_unsigned_to_nat(8u);
v___x_165_ = lean_unsigned_to_nat(2u);
v___x_166_ = lean_nat_mod(v___x_165_, v___x_164_);
return v___x_166_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__3(void){
_start:
{
lean_object* v___x_167_; lean_object* v___x_168_; lean_object* v___x_169_; 
v___x_167_ = lean_unsigned_to_nat(8u);
v___x_168_ = lean_unsigned_to_nat(3u);
v___x_169_ = lean_nat_mod(v___x_168_, v___x_167_);
return v___x_169_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__4(void){
_start:
{
lean_object* v___x_170_; lean_object* v___x_171_; lean_object* v___x_172_; 
v___x_170_ = lean_unsigned_to_nat(8u);
v___x_171_ = lean_unsigned_to_nat(4u);
v___x_172_ = lean_nat_mod(v___x_171_, v___x_170_);
return v___x_172_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__5(void){
_start:
{
lean_object* v___x_173_; lean_object* v___x_174_; lean_object* v___x_175_; 
v___x_173_ = lean_unsigned_to_nat(8u);
v___x_174_ = lean_unsigned_to_nat(5u);
v___x_175_ = lean_nat_mod(v___x_174_, v___x_173_);
return v___x_175_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__6(void){
_start:
{
lean_object* v___x_176_; lean_object* v___x_177_; lean_object* v___x_178_; 
v___x_176_ = lean_unsigned_to_nat(8u);
v___x_177_ = lean_unsigned_to_nat(6u);
v___x_178_ = lean_nat_mod(v___x_177_, v___x_176_);
return v___x_178_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__7(void){
_start:
{
lean_object* v___x_179_; lean_object* v___x_180_; lean_object* v___x_181_; 
v___x_179_ = lean_unsigned_to_nat(8u);
v___x_180_ = lean_unsigned_to_nat(7u);
v___x_181_ = lean_nat_mod(v___x_180_, v___x_179_);
return v___x_181_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15(lean_object* v_sb_182_, lean_object* v_lut_183_, lean_object* v_a_184_){
_start:
{
lean_object* v___x_185_; lean_object* v___x_186_; lean_object* v___x_187_; lean_object* v___x_188_; lean_object* v___x_189_; lean_object* v___x_190_; lean_object* v___x_191_; lean_object* v___x_192_; lean_object* v___x_193_; lean_object* v___x_194_; lean_object* v___x_195_; lean_object* v___x_196_; lean_object* v___x_197_; lean_object* v___x_198_; lean_object* v___x_199_; lean_object* v___x_200_; lean_object* v___x_201_; lean_object* v___x_202_; lean_object* v___x_203_; lean_object* v___x_204_; lean_object* v___x_205_; lean_object* v___x_206_; lean_object* v___x_207_; lean_object* v___x_208_; 
v___x_185_ = lean_unsigned_to_nat(3u);
v___x_186_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__0, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__0_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__0);
lean_inc_ref_n(v_lut_183_, 7);
v___x_187_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_186_);
v___x_188_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__1, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__1_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__1);
v___x_189_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_188_);
v___x_190_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__2, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__2_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__2);
v___x_191_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_190_);
v___x_192_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__3, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__3_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__3);
v___x_193_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_192_);
v___x_194_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__4, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__4_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__4);
v___x_195_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_194_);
v___x_196_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__5, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__5_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__5);
v___x_197_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_196_);
v___x_198_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__6, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__6_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__6);
v___x_199_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_198_);
v___x_200_ = lean_obj_once(&lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__7, &lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__7_once, _init_lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___closed__7);
v___x_201_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_182_, v___x_185_, v_lut_183_, v_a_184_, v___x_200_);
v___x_202_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_202_, 0, v___x_199_);
lean_ctor_set(v___x_202_, 1, v___x_201_);
v___x_203_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_203_, 0, v___x_197_);
lean_ctor_set(v___x_203_, 1, v___x_202_);
v___x_204_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_204_, 0, v___x_195_);
lean_ctor_set(v___x_204_, 1, v___x_203_);
v___x_205_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_205_, 0, v___x_193_);
lean_ctor_set(v___x_205_, 1, v___x_204_);
v___x_206_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_206_, 0, v___x_191_);
lean_ctor_set(v___x_206_, 1, v___x_205_);
v___x_207_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_207_, 0, v___x_189_);
lean_ctor_set(v___x_207_, 1, v___x_206_);
v___x_208_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_208_, 0, v___x_187_);
lean_ctor_set(v___x_208_, 1, v___x_207_);
return v___x_208_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___boxed(lean_object* v_sb_209_, lean_object* v_lut_210_, lean_object* v_a_211_){
_start:
{
lean_object* v_res_212_; 
v_res_212_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15(v_sb_209_, v_lut_210_, v_a_211_);
lean_dec(v_a_211_);
lean_dec_ref(v_sb_209_);
return v_res_212_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId(lean_object* v_sb_216_){
_start:
{
lean_object* v___f_217_; lean_object* v___f_218_; lean_object* v___f_219_; lean_object* v___f_220_; lean_object* v___f_221_; lean_object* v___f_222_; lean_object* v___f_223_; lean_object* v___f_224_; lean_object* v___f_225_; lean_object* v___f_226_; lean_object* v___f_227_; lean_object* v___f_228_; lean_object* v___f_229_; lean_object* v___f_230_; lean_object* v___f_231_; lean_object* v___f_232_; lean_object* v___x_233_; lean_object* v___x_234_; lean_object* v___x_235_; 
lean_inc_ref_n(v_sb_216_, 14);
v___f_217_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__0___boxed), 2, 1);
lean_closure_set(v___f_217_, 0, v_sb_216_);
v___f_218_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__1___boxed), 3, 1);
lean_closure_set(v___f_218_, 0, v_sb_216_);
v___f_219_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__2___boxed), 3, 1);
lean_closure_set(v___f_219_, 0, v_sb_216_);
v___f_220_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__3___boxed), 3, 1);
lean_closure_set(v___f_220_, 0, v_sb_216_);
v___f_221_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__4___boxed), 4, 1);
lean_closure_set(v___f_221_, 0, v_sb_216_);
v___f_222_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__5___boxed), 3, 1);
lean_closure_set(v___f_222_, 0, v_sb_216_);
v___f_223_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__6___boxed), 3, 1);
lean_closure_set(v___f_223_, 0, v_sb_216_);
v___f_224_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__7___boxed), 3, 1);
lean_closure_set(v___f_224_, 0, v_sb_216_);
v___f_225_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__8___boxed), 3, 1);
lean_closure_set(v___f_225_, 0, v_sb_216_);
v___f_226_ = ((lean_object*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__0));
v___f_227_ = ((lean_object*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__1));
v___f_228_ = ((lean_object*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___closed__2));
v___f_229_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__13___boxed), 3, 1);
lean_closure_set(v___f_229_, 0, v_sb_216_);
v___f_230_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__12___boxed), 3, 1);
lean_closure_set(v___f_230_, 0, v_sb_216_);
v___f_231_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__14___boxed), 3, 1);
lean_closure_set(v___f_231_, 0, v_sb_216_);
v___f_232_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_BlockAlgebra_Model_instBlockAlgebraId___lam__15___boxed), 3, 1);
lean_closure_set(v___f_232_, 0, v_sb_216_);
v___x_233_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat___boxed), 2, 1);
lean_closure_set(v___x_233_, 0, v_sb_216_);
v___x_234_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat___boxed), 2, 1);
lean_closure_set(v___x_234_, 0, v_sb_216_);
v___x_235_ = lean_alloc_ctor(0, 20, 0);
lean_ctor_set(v___x_235_, 0, v___f_217_);
lean_ctor_set(v___x_235_, 1, v___x_233_);
lean_ctor_set(v___x_235_, 2, v___x_234_);
lean_ctor_set(v___x_235_, 3, v___f_218_);
lean_ctor_set(v___x_235_, 4, v___f_219_);
lean_ctor_set(v___x_235_, 5, v___f_220_);
lean_ctor_set(v___x_235_, 6, v___f_221_);
lean_ctor_set(v___x_235_, 7, v___f_222_);
lean_ctor_set(v___x_235_, 8, v___f_223_);
lean_ctor_set(v___x_235_, 9, v___f_224_);
lean_ctor_set(v___x_235_, 10, v___f_225_);
lean_ctor_set(v___x_235_, 11, v___f_226_);
lean_ctor_set(v___x_235_, 12, v___f_226_);
lean_ctor_set(v___x_235_, 13, v___f_227_);
lean_ctor_set(v___x_235_, 14, v___f_228_);
lean_ctor_set(v___x_235_, 15, v___f_228_);
lean_ctor_set(v___x_235_, 16, v___f_229_);
lean_ctor_set(v___x_235_, 17, v___f_230_);
lean_ctor_set(v___x_235_, 18, v___f_231_);
lean_ctor_set(v___x_235_, 19, v___f_232_);
return v___x_235_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtBoolId(lean_object* v_sb_236_){
_start:
{
lean_object* v___x_237_; 
v___x_237_ = lean_box(0);
return v___x_237_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtBoolId___boxed(lean_object* v_sb_238_){
_start:
{
lean_object* v_res_239_; 
v_res_239_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtBoolId(v_sb_238_);
lean_dec_ref(v_sb_238_);
return v_res_239_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtIntegerId(lean_object* v_sb_240_, lean_object* v_si_241_){
_start:
{
lean_object* v___x_242_; 
v___x_242_ = lean_box(0);
return v___x_242_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtIntegerId___boxed(lean_object* v_sb_243_, lean_object* v_si_244_){
_start:
{
lean_object* v_res_245_; 
v_res_245_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanCtIntegerId(v_sb_243_, v_si_244_);
lean_dec(v_si_244_);
lean_dec_ref(v_sb_243_);
return v_res_245_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanPtIntegerId(lean_object* v_sb_246_, lean_object* v_si_247_){
_start:
{
lean_object* v___x_248_; 
v___x_248_ = lean_box(0);
return v___x_248_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanPtIntegerId___boxed(lean_object* v_sb_249_, lean_object* v_si_250_){
_start:
{
lean_object* v_res_251_; 
v_res_251_ = lp_zhc__lean_Zhc_BlockAlgebra_Model_instCleanPtIntegerId(v_sb_249_, v_si_250_);
lean_dec(v_si_250_);
lean_dec_ref(v_sb_249_);
return v_res_251_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__1(void){
_start:
{
lean_object* v___x_254_; lean_object* v___x_255_; lean_object* v___x_256_; 
v___x_254_ = lean_unsigned_to_nat(3u);
v___x_255_ = ((lean_object*)(lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__0));
v___x_256_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_255_, v___x_254_);
return v___x_256_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__2(void){
_start:
{
lean_object* v___x_257_; lean_object* v___x_258_; lean_object* v___x_259_; 
v___x_257_ = lean_unsigned_to_nat(2u);
v___x_258_ = ((lean_object*)(lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__0));
v___x_259_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_258_, v___x_257_);
return v___x_259_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__3(void){
_start:
{
lean_object* v___x_260_; lean_object* v___x_261_; lean_object* v___x_262_; lean_object* v___x_263_; 
v___x_260_ = lean_obj_once(&lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__2, &lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__2_once, _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__2);
v___x_261_ = lean_obj_once(&lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__1, &lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__1_once, _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__1);
v___x_262_ = ((lean_object*)(lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__0));
v___x_263_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_262_, v___x_261_, v___x_260_);
return v___x_263_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke(void){
_start:
{
lean_object* v___x_264_; 
v___x_264_ = lean_obj_once(&lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__3, &lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__3_once, _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke___closed__3);
return v___x_264_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Class(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecInteger(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextInteger(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextBool(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_PlaintextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_PlaintextInteger(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_Lookup(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_Operators(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Model_Instance(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Model_SpecBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_SpecInteger(builtin);
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
lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke = _init_lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke();
lean_mark_persistent(lp_zhc__lean___private_Zhc_BlockAlgebra_Model_Instance_0__Zhc_BlockAlgebra_Model_smoke);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
