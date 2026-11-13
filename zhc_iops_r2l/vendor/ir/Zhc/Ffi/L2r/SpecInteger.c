// Lean compiler output
// Module: Zhc.Ffi.L2r.SpecInteger
// Imports: public import Init public meta import Init public import Zhc.Ffi.L2r.SpecBlock
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecIntegerPointed;
lean_object* zhc_lean_spec_integer_int_size(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecInteger_intSize___boxed(lean_object*, lean_object*);
lean_object* zhc_lean_spec_integer_block(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecInteger_block___boxed(lean_object*, lean_object*);
static lean_object* _init_lp_zhc__lean_Zhc_Ffi_L2r_SpecIntegerPointed(void){
_start:
{
lean_object* v___x_1_; 
v___x_1_ = lean_box(0);
return v___x_1_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecInteger_intSize___boxed(lean_object* v_self_4_, lean_object* v_a_00___x40___internal___hyg_5_){
_start:
{
lean_object* v_res_6_; 
v_res_6_ = zhc_lean_spec_integer_int_size(v_self_4_);
lean_dec(v_self_4_);
return v_res_6_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecInteger_block___boxed(lean_object* v_self_9_, lean_object* v_a_00___x40___internal___hyg_10_){
_start:
{
lean_object* v_res_11_; 
v_res_11_ = zhc_lean_spec_integer_block(v_self_9_);
lean_dec(v_self_9_);
return v_res_11_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecBlock(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecInteger(uint8_t builtin) {
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
lp_zhc__lean_Zhc_Ffi_L2r_SpecIntegerPointed = _init_lp_zhc__lean_Zhc_Ffi_L2r_SpecIntegerPointed();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
