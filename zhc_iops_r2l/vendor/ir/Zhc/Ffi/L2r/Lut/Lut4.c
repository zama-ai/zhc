// Lean compiler output
// Module: Zhc.Ffi.L2r.Lut.Lut4
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4Pointed;
lean_object* zhc_lean_lut4_new(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4_new___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4_ofModel(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4_ofModel___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* _init_lp_zhc__lean_Zhc_Ffi_L2r_Lut4Pointed(void){
_start:
{
lean_object* v___x_1_; 
v___x_1_ = lean_box(0);
return v___x_1_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4_new___boxed(lean_object* v_name_9_, lean_object* v_spec_10_, lean_object* v_table1_11_, lean_object* v_table2_12_, lean_object* v_table3_13_, lean_object* v_table4_14_, lean_object* v_a_00___x40___internal___hyg_15_){
_start:
{
lean_object* v_res_16_; 
v_res_16_ = zhc_lean_lut4_new(v_name_9_, v_spec_10_, v_table1_11_, v_table2_12_, v_table3_13_, v_table4_14_);
lean_dec_ref(v_table4_14_);
lean_dec_ref(v_table3_13_);
lean_dec_ref(v_table2_12_);
lean_dec_ref(v_table1_11_);
lean_dec(v_spec_10_);
lean_dec_ref(v_name_9_);
return v_res_16_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4_ofModel(lean_object* v_sb_17_, lean_object* v_lut_18_){
_start:
{
lean_object* v___x_20_; 
v___x_20_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(v_sb_17_);
if (lean_obj_tag(v___x_20_) == 0)
{
lean_object* v_a_21_; lean_object* v_name_22_; lean_object* v___x_23_; lean_object* v___x_24_; lean_object* v___x_25_; lean_object* v___x_26_; lean_object* v___x_27_; lean_object* v___x_28_; lean_object* v___x_29_; lean_object* v___x_30_; lean_object* v___x_31_; 
v_a_21_ = lean_ctor_get(v___x_20_, 0);
lean_inc(v_a_21_);
lean_dec_ref_known(v___x_20_, 1);
v_name_22_ = lean_ctor_get(v_lut_18_, 0);
lean_inc_ref(v_name_22_);
v___x_23_ = lean_unsigned_to_nat(2u);
v___x_24_ = lean_unsigned_to_nat(0u);
lean_inc_ref_n(v_lut_18_, 3);
v___x_25_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_17_, v___x_23_, v_lut_18_, v___x_24_);
v___x_26_ = lean_unsigned_to_nat(1u);
v___x_27_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_17_, v___x_23_, v_lut_18_, v___x_26_);
v___x_28_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_17_, v___x_23_, v_lut_18_, v___x_23_);
v___x_29_ = lean_unsigned_to_nat(3u);
v___x_30_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_17_, v___x_23_, v_lut_18_, v___x_29_);
v___x_31_ = zhc_lean_lut4_new(v_name_22_, v_a_21_, v___x_25_, v___x_27_, v___x_28_, v___x_30_);
lean_dec_ref(v___x_30_);
lean_dec_ref(v___x_28_);
lean_dec_ref(v___x_27_);
lean_dec_ref(v___x_25_);
lean_dec(v_a_21_);
lean_dec_ref(v_name_22_);
return v___x_31_;
}
else
{
lean_dec_ref(v_lut_18_);
return v___x_20_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_Lut4_ofModel___boxed(lean_object* v_sb_32_, lean_object* v_lut_33_, lean_object* v_a_34_){
_start:
{
lean_object* v_res_35_; 
v_res_35_ = lp_zhc__lean_Zhc_Ffi_L2r_Lut4_ofModel(v_sb_32_, v_lut_33_);
lean_dec_ref(v_sb_32_);
return v_res_35_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut_Lut4(uint8_t builtin) {
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
lp_zhc__lean_Zhc_Ffi_L2r_Lut4Pointed = _init_lp_zhc__lean_Zhc_Ffi_L2r_Lut4Pointed();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
