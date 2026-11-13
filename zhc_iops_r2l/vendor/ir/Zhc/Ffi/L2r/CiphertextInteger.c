// Lean compiler output
// Module: Zhc.Ffi.L2r.CiphertextInteger
// Imports: public import Init public meta import Init public import Zhc.Ffi.L2r.SpecInteger
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_CiphertextIntegerPointed;
lean_object* zhc_lean_ciphertext_integer_spec(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_CiphertextInteger_spec___boxed(lean_object*, lean_object*);
lean_object* zhc_lean_ciphertext_integer_unbox(lean_object*, size_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_CiphertextInteger_unbox___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* _init_lp_zhc__lean_Zhc_Ffi_L2r_CiphertextIntegerPointed(void){
_start:
{
lean_object* v___x_1_; 
v___x_1_ = lean_box(0);
return v___x_1_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_CiphertextInteger_spec___boxed(lean_object* v_self_4_, lean_object* v_a_00___x40___internal___hyg_5_){
_start:
{
lean_object* v_res_6_; 
v_res_6_ = zhc_lean_ciphertext_integer_spec(v_self_4_);
lean_dec(v_self_4_);
return v_res_6_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_CiphertextInteger_unbox___boxed(lean_object* v_self_10_, lean_object* v_out_11_, lean_object* v_a_00___x40___internal___hyg_12_){
_start:
{
size_t v_out_boxed_13_; lean_object* v_res_14_; 
v_out_boxed_13_ = lean_unbox_usize(v_out_11_);
lean_dec(v_out_11_);
v_res_14_ = zhc_lean_ciphertext_integer_unbox(v_self_10_, v_out_boxed_13_);
lean_dec(v_self_10_);
return v_res_14_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecInteger(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_CiphertextInteger(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Ffi_L2r_SpecInteger(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_zhc__lean_Zhc_Ffi_L2r_CiphertextIntegerPointed = _init_lp_zhc__lean_Zhc_Ffi_L2r_CiphertextIntegerPointed();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
