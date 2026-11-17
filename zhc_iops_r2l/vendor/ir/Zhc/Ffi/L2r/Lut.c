// Lean compiler output
// Module: Zhc.Ffi.L2r.Lut
// Imports: public import Init public meta import Init public import Zhc.Model.Lookup
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
uint8_t lean_usize_dec_lt(size_t, size_t);
lean_object* lean_array_uget(lean_object*, size_t);
lean_object* lean_array_uset(lean_object*, size_t, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_ManyLut_entry(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
uint16_t lean_uint16_of_nat(lean_object*);
size_t lean_usize_add(size_t, size_t);
lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
lean_object* lean_nat_pow(lean_object*, lean_object*);
lean_object* l_Array_range(lean_object*);
size_t lean_array_size(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Init_Data_Array_Basic_0__Array_mapMUnsafe_map___at___00Zhc_Model_ManyLut_table_spec__0(lean_object*, lean_object*, lean_object*, lean_object*, size_t, size_t, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Init_Data_Array_Basic_0__Array_mapMUnsafe_map___at___00Zhc_Model_ManyLut_table_spec__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_table(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_table___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Init_Data_Array_Basic_0__Array_mapMUnsafe_map___at___00Zhc_Model_ManyLut_table_spec__0(lean_object* v_sb_1_, lean_object* v_n_2_, lean_object* v_lut_3_, lean_object* v_k_4_, size_t v_sz_5_, size_t v_i_6_, lean_object* v_bs_7_){
_start:
{
uint8_t v___x_8_; 
v___x_8_ = lean_usize_dec_lt(v_i_6_, v_sz_5_);
if (v___x_8_ == 0)
{
lean_dec_ref(v_lut_3_);
return v_bs_7_;
}
else
{
lean_object* v_v_9_; lean_object* v___x_10_; lean_object* v_bs_x27_11_; lean_object* v___x_12_; uint16_t v___x_13_; size_t v___x_14_; size_t v___x_15_; lean_object* v___x_16_; lean_object* v___x_17_; 
v_v_9_ = lean_array_uget(v_bs_7_, v_i_6_);
v___x_10_ = lean_unsigned_to_nat(0u);
v_bs_x27_11_ = lean_array_uset(v_bs_7_, v_i_6_, v___x_10_);
lean_inc_ref(v_lut_3_);
v___x_12_ = lp_zhc__lean_Zhc_Model_ManyLut_entry(v_sb_1_, v_n_2_, v_lut_3_, v_k_4_, v_v_9_);
lean_dec(v_v_9_);
v___x_13_ = lean_uint16_of_nat(v___x_12_);
lean_dec(v___x_12_);
v___x_14_ = ((size_t)1ULL);
v___x_15_ = lean_usize_add(v_i_6_, v___x_14_);
v___x_16_ = lean_box(v___x_13_);
v___x_17_ = lean_array_uset(v_bs_x27_11_, v_i_6_, v___x_16_);
v_i_6_ = v___x_15_;
v_bs_7_ = v___x_17_;
goto _start;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Init_Data_Array_Basic_0__Array_mapMUnsafe_map___at___00Zhc_Model_ManyLut_table_spec__0___boxed(lean_object* v_sb_19_, lean_object* v_n_20_, lean_object* v_lut_21_, lean_object* v_k_22_, lean_object* v_sz_23_, lean_object* v_i_24_, lean_object* v_bs_25_){
_start:
{
size_t v_sz_boxed_26_; size_t v_i_boxed_27_; lean_object* v_res_28_; 
v_sz_boxed_26_ = lean_unbox_usize(v_sz_23_);
lean_dec(v_sz_23_);
v_i_boxed_27_ = lean_unbox_usize(v_i_24_);
lean_dec(v_i_24_);
v_res_28_ = lp_zhc__lean___private_Init_Data_Array_Basic_0__Array_mapMUnsafe_map___at___00Zhc_Model_ManyLut_table_spec__0(v_sb_19_, v_n_20_, v_lut_21_, v_k_22_, v_sz_boxed_26_, v_i_boxed_27_, v_bs_25_);
lean_dec(v_k_22_);
lean_dec(v_n_20_);
lean_dec_ref(v_sb_19_);
return v_res_28_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_table(lean_object* v_sb_29_, lean_object* v_n_30_, lean_object* v_lut_31_, lean_object* v_k_32_){
_start:
{
lean_object* v___x_33_; lean_object* v___x_34_; lean_object* v___x_35_; lean_object* v___x_36_; lean_object* v___x_37_; size_t v_sz_38_; size_t v___x_39_; lean_object* v___x_40_; 
v___x_33_ = lean_unsigned_to_nat(2u);
v___x_34_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_sb_29_);
v___x_35_ = lean_nat_sub(v___x_34_, v_n_30_);
lean_dec(v___x_34_);
v___x_36_ = lean_nat_pow(v___x_33_, v___x_35_);
lean_dec(v___x_35_);
v___x_37_ = l_Array_range(v___x_36_);
v_sz_38_ = lean_array_size(v___x_37_);
v___x_39_ = ((size_t)0ULL);
v___x_40_ = lp_zhc__lean___private_Init_Data_Array_Basic_0__Array_mapMUnsafe_map___at___00Zhc_Model_ManyLut_table_spec__0(v_sb_29_, v_n_30_, v_lut_31_, v_k_32_, v_sz_38_, v___x_39_, v___x_37_);
return v___x_40_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_table___boxed(lean_object* v_sb_41_, lean_object* v_n_42_, lean_object* v_lut_43_, lean_object* v_k_44_){
_start:
{
lean_object* v_res_45_; 
v_res_45_ = lp_zhc__lean_Zhc_Model_ManyLut_table(v_sb_41_, v_n_42_, v_lut_43_, v_k_44_);
lean_dec(v_k_44_);
lean_dec(v_n_42_);
lean_dec_ref(v_sb_41_);
return v_res_45_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_Lookup(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_Lut(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Model_Lookup(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
