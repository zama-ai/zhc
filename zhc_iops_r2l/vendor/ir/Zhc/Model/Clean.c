// Lean compiler output
// Module: Zhc.Model.Clean
// Imports: public import Init public meta import Init
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
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanProd(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId___aux__1___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId___aux__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanProd(lean_object* v_00_u03b1_1_, lean_object* v_00_u03b2_2_, lean_object* v_inst_3_, lean_object* v_inst_4_){
_start:
{
lean_object* v___x_5_; 
v___x_5_ = lean_box(0);
return v___x_5_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId___aux__1___redArg(lean_object* v_inst_6_){
_start:
{
return v_inst_6_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId___aux__1(lean_object* v_00_u03b1_7_, lean_object* v_inst_8_){
_start:
{
return v_inst_8_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId___redArg(lean_object* v_inst_9_){
_start:
{
return v_inst_9_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instCleanId(lean_object* v_00_u03b1_10_, lean_object* v_inst_11_){
_start:
{
return v_inst_11_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Model_Clean(uint8_t builtin) {
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
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
