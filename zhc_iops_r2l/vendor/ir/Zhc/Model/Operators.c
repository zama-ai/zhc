// Lean compiler output
// Module: Zhc.Model.Operators
// Imports: public import Init public meta import Init public import Zhc.Model.SpecBlock public import Zhc.Model.CiphertextBlock public import Zhc.Model.PlaintextBlock
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
lean_object* lean_nat_add(lean_object*, lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_add(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_add___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_sub(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_sub___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_shl(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_shl___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mac(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mac___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_addPt(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_addPt___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subPt(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subPt___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subCt(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subCt___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mulPt(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mulPt___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_add(lean_object* v_sb_1_, lean_object* v_a_2_, lean_object* v_b_3_){
_start:
{
lean_object* v___x_4_; lean_object* v___x_5_; 
v___x_4_ = lean_nat_add(v_a_2_, v_b_3_);
v___x_5_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_1_, v___x_4_);
lean_dec(v___x_4_);
return v___x_5_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_add___boxed(lean_object* v_sb_6_, lean_object* v_a_7_, lean_object* v_b_8_){
_start:
{
lean_object* v_res_9_; 
v_res_9_ = lp_zhc__lean_Zhc_Model_Operators_add(v_sb_6_, v_a_7_, v_b_8_);
lean_dec(v_b_8_);
lean_dec(v_a_7_);
lean_dec_ref(v_sb_6_);
return v_res_9_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_sub(lean_object* v_sb_10_, lean_object* v_a_11_, lean_object* v_b_12_){
_start:
{
lean_object* v___x_13_; lean_object* v___x_14_; lean_object* v___x_15_; lean_object* v___x_16_; lean_object* v___x_17_; lean_object* v___x_18_; 
v___x_13_ = lean_unsigned_to_nat(2u);
v___x_14_ = lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(v_sb_10_);
v___x_15_ = lean_nat_pow(v___x_13_, v___x_14_);
lean_dec(v___x_14_);
v___x_16_ = lean_nat_add(v_a_11_, v___x_15_);
lean_dec(v___x_15_);
v___x_17_ = lean_nat_sub(v___x_16_, v_b_12_);
lean_dec(v___x_16_);
v___x_18_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_10_, v___x_17_);
lean_dec(v___x_17_);
return v___x_18_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_sub___boxed(lean_object* v_sb_19_, lean_object* v_a_20_, lean_object* v_b_21_){
_start:
{
lean_object* v_res_22_; 
v_res_22_ = lp_zhc__lean_Zhc_Model_Operators_sub(v_sb_19_, v_a_20_, v_b_21_);
lean_dec(v_b_21_);
lean_dec(v_a_20_);
lean_dec_ref(v_sb_19_);
return v_res_22_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_shl(lean_object* v_sb_23_, lean_object* v_a_24_, lean_object* v_amount_25_){
_start:
{
lean_object* v___x_26_; lean_object* v___x_27_; lean_object* v___x_28_; lean_object* v___x_29_; 
v___x_26_ = lean_unsigned_to_nat(2u);
v___x_27_ = lean_nat_pow(v___x_26_, v_amount_25_);
v___x_28_ = lean_nat_mul(v_a_24_, v___x_27_);
lean_dec(v___x_27_);
v___x_29_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_23_, v___x_28_);
lean_dec(v___x_28_);
return v___x_29_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_shl___boxed(lean_object* v_sb_30_, lean_object* v_a_31_, lean_object* v_amount_32_){
_start:
{
lean_object* v_res_33_; 
v_res_33_ = lp_zhc__lean_Zhc_Model_Operators_shl(v_sb_30_, v_a_31_, v_amount_32_);
lean_dec(v_amount_32_);
lean_dec(v_a_31_);
lean_dec_ref(v_sb_30_);
return v_res_33_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mac(lean_object* v_sb_34_, lean_object* v_a_35_, lean_object* v_mul_36_, lean_object* v_b_37_){
_start:
{
lean_object* v___x_38_; lean_object* v___x_39_; lean_object* v___x_40_; 
v___x_38_ = lean_nat_mul(v_a_35_, v_mul_36_);
v___x_39_ = lean_nat_add(v___x_38_, v_b_37_);
lean_dec(v___x_38_);
v___x_40_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_34_, v___x_39_);
lean_dec(v___x_39_);
return v___x_40_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mac___boxed(lean_object* v_sb_41_, lean_object* v_a_42_, lean_object* v_mul_43_, lean_object* v_b_44_){
_start:
{
lean_object* v_res_45_; 
v_res_45_ = lp_zhc__lean_Zhc_Model_Operators_mac(v_sb_41_, v_a_42_, v_mul_43_, v_b_44_);
lean_dec(v_b_44_);
lean_dec(v_mul_43_);
lean_dec(v_a_42_);
lean_dec_ref(v_sb_41_);
return v_res_45_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_addPt(lean_object* v_sb_46_, lean_object* v_a_47_, lean_object* v_b_48_){
_start:
{
lean_object* v___x_49_; lean_object* v___x_50_; 
v___x_49_ = lean_nat_add(v_a_47_, v_b_48_);
v___x_50_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_46_, v___x_49_);
lean_dec(v___x_49_);
return v___x_50_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_addPt___boxed(lean_object* v_sb_51_, lean_object* v_a_52_, lean_object* v_b_53_){
_start:
{
lean_object* v_res_54_; 
v_res_54_ = lp_zhc__lean_Zhc_Model_Operators_addPt(v_sb_51_, v_a_52_, v_b_53_);
lean_dec(v_b_53_);
lean_dec(v_a_52_);
lean_dec_ref(v_sb_51_);
return v_res_54_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subPt(lean_object* v_sb_55_, lean_object* v_a_56_, lean_object* v_b_57_){
_start:
{
lean_object* v___x_58_; lean_object* v___x_59_; lean_object* v___x_60_; lean_object* v___x_61_; lean_object* v___x_62_; lean_object* v___x_63_; 
v___x_58_ = lean_unsigned_to_nat(2u);
v___x_59_ = lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(v_sb_55_);
v___x_60_ = lean_nat_pow(v___x_58_, v___x_59_);
lean_dec(v___x_59_);
v___x_61_ = lean_nat_add(v_a_56_, v___x_60_);
lean_dec(v___x_60_);
v___x_62_ = lean_nat_sub(v___x_61_, v_b_57_);
lean_dec(v___x_61_);
v___x_63_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_55_, v___x_62_);
lean_dec(v___x_62_);
return v___x_63_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subPt___boxed(lean_object* v_sb_64_, lean_object* v_a_65_, lean_object* v_b_66_){
_start:
{
lean_object* v_res_67_; 
v_res_67_ = lp_zhc__lean_Zhc_Model_Operators_subPt(v_sb_64_, v_a_65_, v_b_66_);
lean_dec(v_b_66_);
lean_dec(v_a_65_);
lean_dec_ref(v_sb_64_);
return v_res_67_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subCt(lean_object* v_sb_68_, lean_object* v_a_69_, lean_object* v_b_70_){
_start:
{
lean_object* v___x_71_; lean_object* v___x_72_; lean_object* v___x_73_; lean_object* v___x_74_; lean_object* v___x_75_; lean_object* v___x_76_; 
v___x_71_ = lean_unsigned_to_nat(2u);
v___x_72_ = lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(v_sb_68_);
v___x_73_ = lean_nat_pow(v___x_71_, v___x_72_);
lean_dec(v___x_72_);
v___x_74_ = lean_nat_add(v_a_69_, v___x_73_);
lean_dec(v___x_73_);
v___x_75_ = lean_nat_sub(v___x_74_, v_b_70_);
lean_dec(v___x_74_);
v___x_76_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_68_, v___x_75_);
lean_dec(v___x_75_);
return v___x_76_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_subCt___boxed(lean_object* v_sb_77_, lean_object* v_a_78_, lean_object* v_b_79_){
_start:
{
lean_object* v_res_80_; 
v_res_80_ = lp_zhc__lean_Zhc_Model_Operators_subCt(v_sb_77_, v_a_78_, v_b_79_);
lean_dec(v_b_79_);
lean_dec(v_a_78_);
lean_dec_ref(v_sb_77_);
return v_res_80_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mulPt(lean_object* v_sb_81_, lean_object* v_a_82_, lean_object* v_b_83_){
_start:
{
lean_object* v___x_84_; lean_object* v___x_85_; 
v___x_84_ = lean_nat_mul(v_a_82_, v_b_83_);
v___x_85_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_81_, v___x_84_);
lean_dec(v___x_84_);
return v___x_85_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Operators_mulPt___boxed(lean_object* v_sb_86_, lean_object* v_a_87_, lean_object* v_b_88_){
_start:
{
lean_object* v_res_89_; 
v_res_89_ = lp_zhc__lean_Zhc_Model_Operators_mulPt(v_sb_86_, v_a_87_, v_b_88_);
lean_dec(v_b_88_);
lean_dec(v_a_87_);
lean_dec_ref(v_sb_86_);
return v_res_89_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_PlaintextBlock(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Model_Operators(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Model_CiphertextBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Model_PlaintextBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
