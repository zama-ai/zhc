// Lean compiler output
// Module: Zhc.Ffi.L2r.Lut.Lut2
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2Pointed;
lean_object* zhc_lean_lut2_new(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2_new___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2_ofModel(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2_ofModel___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* _init_lp_zhc__lean_Zhc_Ffi_L2r_Lut2Pointed(void){
_start:
{
lean_object* v___x_1_; 
v___x_1_ = lean_box(0);
return v___x_1_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2_new___boxed(lean_object* v_name_7_, lean_object* v_spec_8_, lean_object* v_table1_9_, lean_object* v_table2_10_, lean_object* v_a_00___x40___internal___hyg_11_){
_start:
{
lean_object* v_res_12_; 
v_res_12_ = zhc_lean_lut2_new(v_name_7_, v_spec_8_, v_table1_9_, v_table2_10_);
lean_dec_ref(v_table2_10_);
lean_dec_ref(v_table1_9_);
lean_dec(v_spec_8_);
lean_dec_ref(v_name_7_);
return v_res_12_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2_ofModel(lean_object* v_sb_13_, lean_object* v_lut_14_){
_start:
{
lean_object* v___x_16_; 
v___x_16_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(v_sb_13_);
if (lean_obj_tag(v___x_16_) == 0)
{
lean_object* v_a_17_; lean_object* v_name_18_; lean_object* v___x_19_; lean_object* v___x_20_; lean_object* v___x_21_; lean_object* v___x_22_; lean_object* v___x_23_; 
v_a_17_ = lean_ctor_get(v___x_16_, 0);
lean_inc(v_a_17_);
lean_dec_ref_known(v___x_16_, 1);
v_name_18_ = lean_ctor_get(v_lut_14_, 0);
lean_inc_ref(v_name_18_);
v___x_19_ = lean_unsigned_to_nat(1u);
v___x_20_ = lean_unsigned_to_nat(0u);
lean_inc_ref(v_lut_14_);
v___x_21_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_13_, v___x_19_, v_lut_14_, v___x_20_);
v___x_22_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_13_, v___x_19_, v_lut_14_, v___x_19_);
v___x_23_ = zhc_lean_lut2_new(v_name_18_, v_a_17_, v___x_21_, v___x_22_);
lean_dec_ref(v___x_22_);
lean_dec_ref(v___x_21_);
lean_dec(v_a_17_);
lean_dec_ref(v_name_18_);
return v___x_23_;
}
else
{
lean_dec_ref(v_lut_14_);
return v___x_16_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut2_ofModel___boxed(lean_object* v_sb_24_, lean_object* v_lut_25_, lean_object* v_a_26_){
_start:
{
lean_object* v_res_27_; 
v_res_27_ = lp_zhc__lean_Zhc_Ffi_L2r_Lut2_ofModel(v_sb_24_, v_lut_25_);
lean_dec_ref(v_sb_24_);
return v_res_27_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut2(uint8_t builtin) {
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
lp_zhc__lean_Zhc_Ffi_L2r_Lut2Pointed = _init_lp_zhc__lean_Zhc_Ffi_L2r_Lut2Pointed();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
