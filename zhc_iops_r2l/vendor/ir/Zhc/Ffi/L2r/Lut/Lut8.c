// Lean compiler output
// Module: Zhc.Ffi.L2r.Lut.Lut8
// Imports: public import Init public meta import Init public import Zhc.Ffi.L2r.SpecBlock public import Zhc.Ffi.L2r.Lut
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
lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_ManyLut_table(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8Pointed;
lean_object* zhc_lean_lut8_new(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8_new___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8_ofModel(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8_ofModel___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* _init_lp_zhc__lean_Zhc_Ffi_L2r_Lut8Pointed(void){
_start:
{
lean_object* v___x_1_; 
v___x_1_ = lean_box(0);
return v___x_1_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8_new___boxed(lean_object* v_name_13_, lean_object* v_spec_14_, lean_object* v_table1_15_, lean_object* v_table2_16_, lean_object* v_table3_17_, lean_object* v_table4_18_, lean_object* v_table5_19_, lean_object* v_table6_20_, lean_object* v_table7_21_, lean_object* v_table8_22_, lean_object* v_a_00___x40___internal___hyg_23_){
_start:
{
lean_object* v_res_24_; 
v_res_24_ = zhc_lean_lut8_new(v_name_13_, v_spec_14_, v_table1_15_, v_table2_16_, v_table3_17_, v_table4_18_, v_table5_19_, v_table6_20_, v_table7_21_, v_table8_22_);
lean_dec_ref(v_table8_22_);
lean_dec_ref(v_table7_21_);
lean_dec_ref(v_table6_20_);
lean_dec_ref(v_table5_19_);
lean_dec_ref(v_table4_18_);
lean_dec_ref(v_table3_17_);
lean_dec_ref(v_table2_16_);
lean_dec_ref(v_table1_15_);
lean_dec(v_spec_14_);
lean_dec_ref(v_name_13_);
return v_res_24_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8_ofModel(lean_object* v_sb_25_, lean_object* v_lut_26_){
_start:
{
lean_object* v___x_28_; 
v___x_28_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(v_sb_25_);
if (lean_obj_tag(v___x_28_) == 0)
{
lean_object* v_a_29_; lean_object* v_name_30_; lean_object* v___x_31_; lean_object* v___x_32_; lean_object* v___x_33_; lean_object* v___x_34_; lean_object* v___x_35_; lean_object* v___x_36_; lean_object* v___x_37_; lean_object* v___x_38_; lean_object* v___x_39_; lean_object* v___x_40_; lean_object* v___x_41_; lean_object* v___x_42_; lean_object* v___x_43_; lean_object* v___x_44_; lean_object* v___x_45_; lean_object* v___x_46_; lean_object* v___x_47_; 
v_a_29_ = lean_ctor_get(v___x_28_, 0);
lean_inc(v_a_29_);
lean_dec_ref_known(v___x_28_, 1);
v_name_30_ = lean_ctor_get(v_lut_26_, 0);
lean_inc_ref(v_name_30_);
v___x_31_ = lean_unsigned_to_nat(3u);
v___x_32_ = lean_unsigned_to_nat(0u);
lean_inc_ref_n(v_lut_26_, 7);
v___x_33_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_32_);
v___x_34_ = lean_unsigned_to_nat(1u);
v___x_35_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_34_);
v___x_36_ = lean_unsigned_to_nat(2u);
v___x_37_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_36_);
v___x_38_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_31_);
v___x_39_ = lean_unsigned_to_nat(4u);
v___x_40_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_39_);
v___x_41_ = lean_unsigned_to_nat(5u);
v___x_42_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_41_);
v___x_43_ = lean_unsigned_to_nat(6u);
v___x_44_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_43_);
v___x_45_ = lean_unsigned_to_nat(7u);
v___x_46_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_25_, v___x_31_, v_lut_26_, v___x_45_);
v___x_47_ = zhc_lean_lut8_new(v_name_30_, v_a_29_, v___x_33_, v___x_35_, v___x_37_, v___x_38_, v___x_40_, v___x_42_, v___x_44_, v___x_46_);
lean_dec_ref(v___x_46_);
lean_dec_ref(v___x_44_);
lean_dec_ref(v___x_42_);
lean_dec_ref(v___x_40_);
lean_dec_ref(v___x_38_);
lean_dec_ref(v___x_37_);
lean_dec_ref(v___x_35_);
lean_dec_ref(v___x_33_);
lean_dec(v_a_29_);
lean_dec_ref(v_name_30_);
return v___x_47_;
}
else
{
lean_dec_ref(v_lut_26_);
return v___x_28_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut8_ofModel___boxed(lean_object* v_sb_48_, lean_object* v_lut_49_, lean_object* v_a_50_){
_start:
{
lean_object* v_res_51_; 
v_res_51_ = lp_zhc__lean_Zhc_Ffi_L2r_Lut8_ofModel(v_sb_48_, v_lut_49_);
lean_dec_ref(v_sb_48_);
return v_res_51_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut8(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Ffi_L2r_Lut(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_zhc__lean_Zhc_Ffi_L2r_Lut8Pointed = _init_lp_zhc__lean_Zhc_Ffi_L2r_Lut8Pointed();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
