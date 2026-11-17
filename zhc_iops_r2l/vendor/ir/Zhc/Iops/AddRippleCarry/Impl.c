// Lean compiler output
// Module: Zhc.Iops.AddRippleCarry.Impl
// Imports: public import Init public meta import Init public import Zhc.BlockAlgebra.Class public import Zhc.BlockAlgebra.Model.Instance public import Zhc.BlockAlgebra.Tracer.Instance
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
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object*, lean_object*);
lean_object* l_List_finRange(lean_object*);
lean_object* l_List_get___redArg(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_add(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMoveCarryToMessage(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Lut2_fromFn(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(lean_object*, lean_object*, lean_object*);
lean_object* l_List_forIn_x27_loop___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit(lean_object*, lean_object*);
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_sb22___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(2) << 1) | 1)),((lean_object*)(((size_t)(2) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_sb22___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_sb22___closed__0_value;
LEAN_EXPORT const lean_object* lp_zhc__lean_Zhc_Iops_sb22 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_sb22___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__0___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__1___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*1, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__0___boxed, .m_arity = 2, .m_num_fixed = 1, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_sb22___closed__0_value)} };
static const lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__0_value;
static const lean_closure_object lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*1, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__1___boxed, .m_arity = 2, .m_num_fixed = 1, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_sb22___closed__0_value)} };
static const lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__1_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 9, .m_capacity = 9, .m_length = 8, .m_data = "CarryMsg"};
static const lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__2_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__3;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__1(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__2(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__3(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__4(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__5(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__6(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__7(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__8(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__9(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__0;
static lean_once_cell_t lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__1;
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___closed__0;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___boxed(lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__0;
static lean_once_cell_t lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__1;
static lean_once_cell_t lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__2;
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8;
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__0;
static lean_once_cell_t lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__1;
static lean_once_cell_t lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__2;
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16;
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___lam__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__0(lean_object* v___x_4_, lean_object* v_x_5_){
_start:
{
lean_object* v___x_6_; 
v___x_6_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doMoveCarryToMessage(v___x_4_, v_x_5_);
return v___x_6_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__0___boxed(lean_object* v___x_7_, lean_object* v_x_8_){
_start:
{
lean_object* v_res_9_; 
v_res_9_ = lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__0(v___x_7_, v_x_8_);
lean_dec(v_x_8_);
lean_dec_ref(v___x_7_);
return v_res_9_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__1(lean_object* v___x_10_, lean_object* v_x_11_){
_start:
{
lean_object* v___x_12_; 
v___x_12_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage(v___x_10_, v_x_11_);
return v___x_12_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__1___boxed(lean_object* v___x_13_, lean_object* v_x_14_){
_start:
{
lean_object* v_res_15_; 
v_res_15_ = lp_zhc__lean_Zhc_Iops_carryMsgLut___lam__1(v___x_13_, v_x_14_);
lean_dec(v_x_14_);
lean_dec_ref(v___x_13_);
return v_res_15_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__3(void){
_start:
{
lean_object* v___f_21_; lean_object* v___f_22_; lean_object* v___x_23_; lean_object* v___x_24_; lean_object* v___x_25_; 
v___f_21_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__1));
v___f_22_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__0));
v___x_23_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__2));
v___x_24_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_25_ = lp_zhc__lean_Zhc_Model_Lut2_fromFn(v___x_24_, v___x_23_, v___f_22_, v___f_21_);
return v___x_25_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_carryMsgLut(void){
_start:
{
lean_object* v___x_26_; 
v___x_26_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__3, &lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__3_once, _init_lp_zhc__lean_Zhc_Iops_carryMsgLut___closed__3);
return v___x_26_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__0(lean_object* v_toPure_27_, lean_object* v_____s_28_){
_start:
{
lean_object* v_fst_29_; lean_object* v___x_30_; 
v_fst_29_ = lean_ctor_get(v_____s_28_, 0);
lean_inc(v_fst_29_);
lean_dec_ref(v_____s_28_);
v___x_30_ = lean_apply_2(v_toPure_27_, lean_box(0), v_fst_29_);
return v___x_30_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__1(lean_object* v_fst_31_, lean_object* v_toPure_32_, lean_object* v_acc_33_){
_start:
{
lean_object* v___x_34_; lean_object* v___x_35_; lean_object* v___x_36_; 
v___x_34_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_34_, 0, v_acc_33_);
lean_ctor_set(v___x_34_, 1, v_fst_31_);
v___x_35_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_35_, 0, v___x_34_);
v___x_36_ = lean_apply_2(v_toPure_32_, lean_box(0), v___x_35_);
return v___x_36_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__2(lean_object* v_toPure_37_, lean_object* v_storeCtBlock_38_, lean_object* v_si_39_, lean_object* v_fst_40_, lean_object* v_a_41_, lean_object* v_toBind_42_, lean_object* v_cm_43_){
_start:
{
lean_object* v_fst_44_; lean_object* v_snd_45_; lean_object* v___f_46_; lean_object* v___x_47_; lean_object* v___x_48_; 
v_fst_44_ = lean_ctor_get(v_cm_43_, 0);
lean_inc(v_fst_44_);
v_snd_45_ = lean_ctor_get(v_cm_43_, 1);
lean_inc(v_snd_45_);
lean_dec_ref(v_cm_43_);
v___f_46_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__1), 3, 2);
lean_closure_set(v___f_46_, 0, v_fst_44_);
lean_closure_set(v___f_46_, 1, v_toPure_37_);
v___x_47_ = lean_apply_4(v_storeCtBlock_38_, v_si_39_, v_fst_40_, v_a_41_, v_snd_45_);
v___x_48_ = lean_apply_4(v_toBind_42_, lean_box(0), lean_box(0), v___x_47_, v___f_46_);
return v___x_48_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__3(lean_object* v_pbs2_49_, lean_object* v_toBind_50_, lean_object* v___f_51_, lean_object* v_s_52_){
_start:
{
lean_object* v___x_53_; lean_object* v___x_54_; lean_object* v___x_55_; 
v___x_53_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_54_ = lean_apply_2(v_pbs2_49_, v___x_53_, v_s_52_);
v___x_55_ = lean_apply_4(v_toBind_50_, lean_box(0), lean_box(0), v___x_54_, v___f_51_);
return v___x_55_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__4(lean_object* v_addCt_56_, lean_object* v_snd_57_, lean_object* v_toBind_58_, lean_object* v___f_59_, lean_object* v_s_60_){
_start:
{
lean_object* v___x_61_; lean_object* v___x_62_; 
v___x_61_ = lean_apply_2(v_addCt_56_, v_s_60_, v_snd_57_);
v___x_62_ = lean_apply_4(v_toBind_58_, lean_box(0), lean_box(0), v___x_61_, v___f_59_);
return v___x_62_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__5(lean_object* v_addCt_63_, lean_object* v_xi_64_, lean_object* v_toBind_65_, lean_object* v___f_66_, lean_object* v_yi_67_){
_start:
{
lean_object* v___x_68_; lean_object* v___x_69_; 
v___x_68_ = lean_apply_2(v_addCt_63_, v_xi_64_, v_yi_67_);
v___x_69_ = lean_apply_4(v_toBind_65_, lean_box(0), lean_box(0), v___x_68_, v___f_66_);
return v___x_69_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__6(lean_object* v_addCt_70_, lean_object* v_toBind_71_, lean_object* v___f_72_, lean_object* v_extractCtBlock_73_, lean_object* v_si_74_, lean_object* v_y_75_, lean_object* v_a_76_, lean_object* v_xi_77_){
_start:
{
lean_object* v___f_78_; lean_object* v___x_79_; lean_object* v___x_80_; 
lean_inc(v_toBind_71_);
v___f_78_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__5), 5, 4);
lean_closure_set(v___f_78_, 0, v_addCt_70_);
lean_closure_set(v___f_78_, 1, v_xi_77_);
lean_closure_set(v___f_78_, 2, v_toBind_71_);
lean_closure_set(v___f_78_, 3, v___f_72_);
v___x_79_ = lean_apply_3(v_extractCtBlock_73_, v_si_74_, v_y_75_, v_a_76_);
v___x_80_ = lean_apply_4(v_toBind_71_, lean_box(0), lean_box(0), v___x_79_, v___f_78_);
return v___x_80_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__7(lean_object* v_toPure_81_, lean_object* v_storeCtBlock_82_, lean_object* v_si_83_, lean_object* v_toBind_84_, lean_object* v_pbs2_85_, lean_object* v_addCt_86_, lean_object* v_extractCtBlock_87_, lean_object* v_y_88_, lean_object* v_x_89_, lean_object* v_a_90_, lean_object* v_x_91_, lean_object* v___y_92_){
_start:
{
lean_object* v_fst_93_; lean_object* v_snd_94_; lean_object* v___f_95_; lean_object* v___f_96_; lean_object* v___f_97_; lean_object* v___f_98_; lean_object* v___x_99_; lean_object* v___x_100_; 
v_fst_93_ = lean_ctor_get(v___y_92_, 0);
lean_inc(v_fst_93_);
v_snd_94_ = lean_ctor_get(v___y_92_, 1);
lean_inc(v_snd_94_);
lean_dec_ref(v___y_92_);
lean_inc_n(v_toBind_84_, 4);
lean_inc_n(v_a_90_, 2);
lean_inc_n(v_si_83_, 2);
v___f_95_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__2), 7, 6);
lean_closure_set(v___f_95_, 0, v_toPure_81_);
lean_closure_set(v___f_95_, 1, v_storeCtBlock_82_);
lean_closure_set(v___f_95_, 2, v_si_83_);
lean_closure_set(v___f_95_, 3, v_fst_93_);
lean_closure_set(v___f_95_, 4, v_a_90_);
lean_closure_set(v___f_95_, 5, v_toBind_84_);
v___f_96_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__3), 4, 3);
lean_closure_set(v___f_96_, 0, v_pbs2_85_);
lean_closure_set(v___f_96_, 1, v_toBind_84_);
lean_closure_set(v___f_96_, 2, v___f_95_);
lean_inc(v_addCt_86_);
v___f_97_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__4), 5, 4);
lean_closure_set(v___f_97_, 0, v_addCt_86_);
lean_closure_set(v___f_97_, 1, v_snd_94_);
lean_closure_set(v___f_97_, 2, v_toBind_84_);
lean_closure_set(v___f_97_, 3, v___f_96_);
lean_inc(v_extractCtBlock_87_);
v___f_98_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__6), 8, 7);
lean_closure_set(v___f_98_, 0, v_addCt_86_);
lean_closure_set(v___f_98_, 1, v_toBind_84_);
lean_closure_set(v___f_98_, 2, v___f_97_);
lean_closure_set(v___f_98_, 3, v_extractCtBlock_87_);
lean_closure_set(v___f_98_, 4, v_si_83_);
lean_closure_set(v___f_98_, 5, v_y_88_);
lean_closure_set(v___f_98_, 6, v_a_90_);
v___x_99_ = lean_apply_3(v_extractCtBlock_87_, v_si_83_, v_x_89_, v_a_90_);
v___x_100_ = lean_apply_4(v_toBind_84_, lean_box(0), lean_box(0), v___x_99_, v___f_98_);
return v___x_100_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__8(lean_object* v_si_101_, lean_object* v_acc_102_, lean_object* v_inst_103_, lean_object* v___f_104_, lean_object* v_toBind_105_, lean_object* v___f_106_, lean_object* v_carry_107_){
_start:
{
lean_object* v___x_108_; lean_object* v___x_109_; lean_object* v___x_110_; lean_object* v___x_111_; 
v___x_108_ = l_List_finRange(v_si_101_);
v___x_109_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_109_, 0, v_acc_102_);
lean_ctor_set(v___x_109_, 1, v_carry_107_);
v___x_110_ = l_List_forIn_x27_loop___redArg(v_inst_103_, v___f_104_, v___x_108_, v___x_109_);
lean_dec(v___x_108_);
v___x_111_ = lean_apply_4(v_toBind_105_, lean_box(0), lean_box(0), v___x_110_, v___f_106_);
return v___x_111_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__9(lean_object* v_si_112_, lean_object* v_inst_113_, lean_object* v___f_114_, lean_object* v_toBind_115_, lean_object* v___f_116_, lean_object* v_letCtBlock_117_, lean_object* v_acc_118_){
_start:
{
lean_object* v___f_119_; lean_object* v___x_120_; lean_object* v___x_121_; lean_object* v___x_122_; 
lean_inc(v_toBind_115_);
v___f_119_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__8), 7, 6);
lean_closure_set(v___f_119_, 0, v_si_112_);
lean_closure_set(v___f_119_, 1, v_acc_118_);
lean_closure_set(v___f_119_, 2, v_inst_113_);
lean_closure_set(v___f_119_, 3, v___f_114_);
lean_closure_set(v___f_119_, 4, v_toBind_115_);
lean_closure_set(v___f_119_, 5, v___f_116_);
v___x_120_ = lean_unsigned_to_nat(0u);
v___x_121_ = lean_apply_1(v_letCtBlock_117_, v___x_120_);
v___x_122_ = lean_apply_4(v_toBind_115_, lean_box(0), lean_box(0), v___x_121_, v___f_119_);
return v___x_122_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg(lean_object* v_inst_123_, lean_object* v_A_124_, lean_object* v_si_125_, lean_object* v_x_126_, lean_object* v_y_127_){
_start:
{
lean_object* v_toApplicative_128_; lean_object* v_toBind_129_; lean_object* v_declareInteger_130_; lean_object* v_letCtBlock_131_; lean_object* v_addCt_132_; lean_object* v_extractCtBlock_133_; lean_object* v_storeCtBlock_134_; lean_object* v_pbs2_135_; lean_object* v_toPure_136_; lean_object* v___x_137_; lean_object* v___f_138_; lean_object* v___f_139_; lean_object* v___f_140_; lean_object* v___x_141_; 
v_toApplicative_128_ = lean_ctor_get(v_inst_123_, 0);
v_toBind_129_ = lean_ctor_get(v_inst_123_, 1);
lean_inc_n(v_toBind_129_, 3);
v_declareInteger_130_ = lean_ctor_get(v_A_124_, 0);
lean_inc(v_declareInteger_130_);
v_letCtBlock_131_ = lean_ctor_get(v_A_124_, 2);
lean_inc(v_letCtBlock_131_);
v_addCt_132_ = lean_ctor_get(v_A_124_, 3);
lean_inc(v_addCt_132_);
v_extractCtBlock_133_ = lean_ctor_get(v_A_124_, 11);
lean_inc(v_extractCtBlock_133_);
v_storeCtBlock_134_ = lean_ctor_get(v_A_124_, 13);
lean_inc(v_storeCtBlock_134_);
v_pbs2_135_ = lean_ctor_get(v_A_124_, 17);
lean_inc(v_pbs2_135_);
lean_dec_ref(v_A_124_);
v_toPure_136_ = lean_ctor_get(v_toApplicative_128_, 1);
lean_inc_n(v_si_125_, 2);
v___x_137_ = lean_apply_1(v_declareInteger_130_, v_si_125_);
lean_inc_n(v_toPure_136_, 2);
v___f_138_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__0), 2, 1);
lean_closure_set(v___f_138_, 0, v_toPure_136_);
v___f_139_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__7), 12, 9);
lean_closure_set(v___f_139_, 0, v_toPure_136_);
lean_closure_set(v___f_139_, 1, v_storeCtBlock_134_);
lean_closure_set(v___f_139_, 2, v_si_125_);
lean_closure_set(v___f_139_, 3, v_toBind_129_);
lean_closure_set(v___f_139_, 4, v_pbs2_135_);
lean_closure_set(v___f_139_, 5, v_addCt_132_);
lean_closure_set(v___f_139_, 6, v_extractCtBlock_133_);
lean_closure_set(v___f_139_, 7, v_y_127_);
lean_closure_set(v___f_139_, 8, v_x_126_);
v___f_140_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg___lam__9), 7, 6);
lean_closure_set(v___f_140_, 0, v_si_125_);
lean_closure_set(v___f_140_, 1, v_inst_123_);
lean_closure_set(v___f_140_, 2, v___f_139_);
lean_closure_set(v___f_140_, 3, v_toBind_129_);
lean_closure_set(v___f_140_, 4, v___f_138_);
lean_closure_set(v___f_140_, 5, v_letCtBlock_131_);
v___x_141_ = lean_apply_4(v_toBind_129_, lean_box(0), lean_box(0), v___x_137_, v___f_140_);
return v___x_141_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry(lean_object* v_M_142_, lean_object* v_inst_143_, lean_object* v_A_144_, lean_object* v_si_145_, lean_object* v_x_146_, lean_object* v_y_147_){
_start:
{
lean_object* v___x_148_; 
v___x_148_ = lp_zhc__lean_Zhc_Iops_addRippleCarry___redArg(v_inst_143_, v_A_144_, v_si_145_, v_x_146_, v_y_147_);
return v___x_148_;
}
}
static lean_object* _init_lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__0(void){
_start:
{
lean_object* v___x_149_; lean_object* v___x_150_; lean_object* v___x_151_; 
v___x_149_ = lean_unsigned_to_nat(2u);
v___x_150_ = lean_unsigned_to_nat(0u);
v___x_151_ = lean_nat_mod(v___x_150_, v___x_149_);
return v___x_151_;
}
}
static lean_object* _init_lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__1(void){
_start:
{
lean_object* v___x_152_; lean_object* v___x_153_; lean_object* v___x_154_; 
v___x_152_ = lean_unsigned_to_nat(2u);
v___x_153_ = lean_unsigned_to_nat(1u);
v___x_154_ = lean_nat_mod(v___x_153_, v___x_152_);
return v___x_154_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg(lean_object* v_x_155_, lean_object* v_y_156_, lean_object* v_as_x27_157_, lean_object* v_b_158_){
_start:
{
lean_object* v___x_159_; 
v___x_159_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
if (lean_obj_tag(v_as_x27_157_) == 0)
{
return v_b_158_;
}
else
{
lean_object* v_head_160_; lean_object* v_tail_161_; lean_object* v_fst_162_; lean_object* v_snd_163_; lean_object* v___x_165_; uint8_t v_isShared_166_; uint8_t v_isSharedCheck_182_; 
v_head_160_ = lean_ctor_get(v_as_x27_157_, 0);
v_tail_161_ = lean_ctor_get(v_as_x27_157_, 1);
v_fst_162_ = lean_ctor_get(v_b_158_, 0);
v_snd_163_ = lean_ctor_get(v_b_158_, 1);
v_isSharedCheck_182_ = !lean_is_exclusive(v_b_158_);
if (v_isSharedCheck_182_ == 0)
{
v___x_165_ = v_b_158_;
v_isShared_166_ = v_isSharedCheck_182_;
goto v_resetjp_164_;
}
else
{
lean_inc(v_snd_163_);
lean_inc(v_fst_162_);
lean_dec(v_b_158_);
v___x_165_ = lean_box(0);
v_isShared_166_ = v_isSharedCheck_182_;
goto v_resetjp_164_;
}
v_resetjp_164_:
{
lean_object* v___x_167_; lean_object* v___x_168_; lean_object* v___x_169_; lean_object* v___x_170_; lean_object* v___x_171_; lean_object* v___x_172_; lean_object* v___x_173_; lean_object* v___x_174_; lean_object* v___x_175_; lean_object* v___x_176_; lean_object* v___x_177_; lean_object* v___x_179_; 
lean_inc_n(v_head_160_, 3);
v___x_167_ = l_List_get___redArg(v_x_155_, v_head_160_);
v___x_168_ = l_List_get___redArg(v_y_156_, v_head_160_);
v___x_169_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_159_, v___x_167_, v___x_168_);
lean_dec(v___x_168_);
lean_dec(v___x_167_);
v___x_170_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_159_, v___x_169_, v_snd_163_);
lean_dec(v_snd_163_);
lean_dec(v___x_169_);
v___x_171_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_172_ = lean_unsigned_to_nat(1u);
v___x_173_ = lean_obj_once(&lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__0, &lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__0_once, _init_lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__0);
v___x_174_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_159_, v___x_172_, v___x_171_, v___x_170_, v___x_173_);
v___x_175_ = lean_obj_once(&lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__1, &lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__1_once, _init_lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___closed__1);
v___x_176_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_159_, v___x_172_, v___x_171_, v___x_170_, v___x_175_);
lean_dec(v___x_170_);
v___x_177_ = lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(v_fst_162_, v_head_160_, v___x_176_);
if (v_isShared_166_ == 0)
{
lean_ctor_set(v___x_165_, 1, v___x_174_);
lean_ctor_set(v___x_165_, 0, v___x_177_);
v___x_179_ = v___x_165_;
goto v_reusejp_178_;
}
else
{
lean_object* v_reuseFailAlloc_181_; 
v_reuseFailAlloc_181_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_181_, 0, v___x_177_);
lean_ctor_set(v_reuseFailAlloc_181_, 1, v___x_174_);
v___x_179_ = v_reuseFailAlloc_181_;
goto v_reusejp_178_;
}
v_reusejp_178_:
{
v_as_x27_157_ = v_tail_161_;
v_b_158_ = v___x_179_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg___boxed(lean_object* v_x_183_, lean_object* v_y_184_, lean_object* v_as_x27_185_, lean_object* v_b_186_){
_start:
{
lean_object* v_res_187_; 
v_res_187_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg(v_x_183_, v_y_184_, v_as_x27_185_, v_b_186_);
lean_dec(v_as_x27_185_);
lean_dec(v_y_184_);
lean_dec(v_x_183_);
return v_res_187_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___closed__0(void){
_start:
{
lean_object* v___x_188_; lean_object* v___x_189_; lean_object* v___x_190_; 
v___x_188_ = lean_unsigned_to_nat(0u);
v___x_189_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_190_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_189_, v___x_188_);
return v___x_190_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0(lean_object* v_si_191_, lean_object* v_x_192_, lean_object* v_y_193_){
_start:
{
lean_object* v___x_194_; lean_object* v___x_195_; lean_object* v___x_196_; lean_object* v___x_197_; lean_object* v___x_198_; lean_object* v___x_199_; lean_object* v___x_200_; lean_object* v_fst_201_; 
v___x_194_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_195_ = lean_unsigned_to_nat(0u);
v___x_196_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_194_, v_si_191_, v___x_195_);
v___x_197_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___closed__0, &lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___closed__0_once, _init_lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___closed__0);
v___x_198_ = l_List_finRange(v_si_191_);
v___x_199_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_199_, 0, v___x_196_);
lean_ctor_set(v___x_199_, 1, v___x_197_);
v___x_200_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg(v_x_192_, v_y_193_, v___x_198_, v___x_199_);
lean_dec(v___x_198_);
v_fst_201_ = lean_ctor_get(v___x_200_, 0);
lean_inc(v_fst_201_);
lean_dec_ref(v___x_200_);
return v_fst_201_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0___boxed(lean_object* v_si_202_, lean_object* v_x_203_, lean_object* v_y_204_){
_start:
{
lean_object* v_res_205_; 
v_res_205_ = lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0(v_si_202_, v_x_203_, v_y_204_);
lean_dec(v_y_204_);
lean_dec(v_x_203_);
return v_res_205_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__0(void){
_start:
{
lean_object* v___x_206_; lean_object* v___x_207_; lean_object* v___x_208_; lean_object* v___x_209_; 
v___x_206_ = lean_unsigned_to_nat(200u);
v___x_207_ = lean_unsigned_to_nat(4u);
v___x_208_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_209_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_208_, v___x_207_, v___x_206_);
return v___x_209_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__1(void){
_start:
{
lean_object* v___x_210_; lean_object* v___x_211_; lean_object* v___x_212_; lean_object* v___x_213_; 
v___x_210_ = lean_unsigned_to_nat(100u);
v___x_211_ = lean_unsigned_to_nat(4u);
v___x_212_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_213_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_212_, v___x_211_, v___x_210_);
return v___x_213_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__2(void){
_start:
{
lean_object* v___x_214_; lean_object* v___x_215_; lean_object* v___x_216_; lean_object* v___x_217_; 
v___x_214_ = lean_obj_once(&lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__1, &lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__1_once, _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__1);
v___x_215_ = lean_obj_once(&lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__0, &lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__0_once, _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__0);
v___x_216_ = lean_unsigned_to_nat(4u);
v___x_217_ = lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0(v___x_216_, v___x_215_, v___x_214_);
return v___x_217_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8(void){
_start:
{
lean_object* v___x_218_; 
v___x_218_ = lean_obj_once(&lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__2, &lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__2_once, _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8___closed__2);
return v___x_218_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0(lean_object* v_si_219_, lean_object* v_x_220_, lean_object* v_y_221_, lean_object* v_as_222_, lean_object* v_as_x27_223_, lean_object* v_b_224_, lean_object* v_a_225_){
_start:
{
lean_object* v___x_226_; 
v___x_226_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___redArg(v_x_220_, v_y_221_, v_as_x27_223_, v_b_224_);
return v___x_226_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0___boxed(lean_object* v_si_227_, lean_object* v_x_228_, lean_object* v_y_229_, lean_object* v_as_230_, lean_object* v_as_x27_231_, lean_object* v_b_232_, lean_object* v_a_233_){
_start:
{
lean_object* v_res_234_; 
v_res_234_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0_spec__0(v_si_227_, v_x_228_, v_y_229_, v_as_230_, v_as_x27_231_, v_b_232_, v_a_233_);
lean_dec(v_as_x27_231_);
lean_dec(v_as_230_);
lean_dec(v_y_229_);
lean_dec(v_x_228_);
lean_dec(v_si_227_);
return v_res_234_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__0(void){
_start:
{
lean_object* v___x_235_; lean_object* v___x_236_; lean_object* v___x_237_; lean_object* v___x_238_; 
v___x_235_ = lean_unsigned_to_nat(60000u);
v___x_236_ = lean_unsigned_to_nat(8u);
v___x_237_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_238_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_237_, v___x_236_, v___x_235_);
return v___x_238_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__1(void){
_start:
{
lean_object* v___x_239_; lean_object* v___x_240_; lean_object* v___x_241_; lean_object* v___x_242_; 
v___x_239_ = lean_unsigned_to_nat(10000u);
v___x_240_ = lean_unsigned_to_nat(8u);
v___x_241_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_242_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_241_, v___x_240_, v___x_239_);
return v___x_242_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__2(void){
_start:
{
lean_object* v___x_243_; lean_object* v___x_244_; lean_object* v___x_245_; lean_object* v___x_246_; 
v___x_243_ = lean_obj_once(&lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__1, &lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__1_once, _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__1);
v___x_244_ = lean_obj_once(&lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__0, &lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__0_once, _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__0);
v___x_245_ = lean_unsigned_to_nat(8u);
v___x_246_ = lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00__private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8_spec__0(v___x_245_, v___x_244_, v___x_243_);
return v___x_246_;
}
}
static lean_object* _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16(void){
_start:
{
lean_object* v___x_247_; 
v___x_247_ = lean_obj_once(&lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__2, &lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__2_once, _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16___closed__2);
return v___x_247_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___redArg(lean_object* v_x_248_, lean_object* v_y_249_, lean_object* v_as_x27_250_, lean_object* v_b_251_, lean_object* v___y_252_){
_start:
{
lean_object* v___x_254_; 
v___x_254_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
if (lean_obj_tag(v_as_x27_250_) == 0)
{
lean_object* v___x_255_; 
lean_dec(v_y_249_);
lean_dec(v_x_248_);
v___x_255_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_255_, 0, v_b_251_);
return v___x_255_;
}
else
{
lean_object* v_head_256_; lean_object* v_tail_257_; lean_object* v_fst_258_; lean_object* v_snd_259_; lean_object* v___x_260_; 
v_head_256_ = lean_ctor_get(v_as_x27_250_, 0);
v_tail_257_ = lean_ctor_get(v_as_x27_250_, 1);
v_fst_258_ = lean_ctor_get(v_b_251_, 0);
lean_inc(v_fst_258_);
v_snd_259_ = lean_ctor_get(v_b_251_, 1);
lean_inc(v_snd_259_);
lean_dec_ref(v_b_251_);
lean_inc(v_head_256_);
lean_inc(v_x_248_);
v___x_260_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(v_x_248_, v_head_256_, v___y_252_);
if (lean_obj_tag(v___x_260_) == 0)
{
lean_object* v_a_261_; lean_object* v___x_262_; 
v_a_261_ = lean_ctor_get(v___x_260_, 0);
lean_inc(v_a_261_);
lean_dec_ref_known(v___x_260_, 1);
lean_inc(v_head_256_);
lean_inc(v_y_249_);
v___x_262_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(v_y_249_, v_head_256_, v___y_252_);
if (lean_obj_tag(v___x_262_) == 0)
{
lean_object* v_a_263_; lean_object* v___x_264_; 
v_a_263_ = lean_ctor_get(v___x_262_, 0);
lean_inc(v_a_263_);
lean_dec_ref_known(v___x_262_, 1);
v___x_264_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_254_, v_a_261_, v_a_263_, v___y_252_);
lean_dec(v_a_261_);
if (lean_obj_tag(v___x_264_) == 0)
{
lean_object* v_a_265_; lean_object* v___x_266_; 
v_a_265_ = lean_ctor_get(v___x_264_, 0);
lean_inc(v_a_265_);
lean_dec_ref_known(v___x_264_, 1);
v___x_266_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_254_, v_a_265_, v_snd_259_, v___y_252_);
lean_dec(v_a_265_);
if (lean_obj_tag(v___x_266_) == 0)
{
lean_object* v_a_267_; lean_object* v___x_268_; lean_object* v___x_269_; 
v_a_267_ = lean_ctor_get(v___x_266_, 0);
lean_inc(v_a_267_);
lean_dec_ref_known(v___x_266_, 1);
v___x_268_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_269_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(v___x_254_, v___x_268_, v_a_267_, v___y_252_);
if (lean_obj_tag(v___x_269_) == 0)
{
lean_object* v_a_270_; lean_object* v_fst_271_; lean_object* v_snd_272_; lean_object* v___x_274_; uint8_t v_isShared_275_; uint8_t v_isSharedCheck_290_; 
v_a_270_ = lean_ctor_get(v___x_269_, 0);
lean_inc(v_a_270_);
lean_dec_ref_known(v___x_269_, 1);
v_fst_271_ = lean_ctor_get(v_a_270_, 0);
v_snd_272_ = lean_ctor_get(v_a_270_, 1);
v_isSharedCheck_290_ = !lean_is_exclusive(v_a_270_);
if (v_isSharedCheck_290_ == 0)
{
v___x_274_ = v_a_270_;
v_isShared_275_ = v_isSharedCheck_290_;
goto v_resetjp_273_;
}
else
{
lean_inc(v_snd_272_);
lean_inc(v_fst_271_);
lean_dec(v_a_270_);
v___x_274_ = lean_box(0);
v_isShared_275_ = v_isSharedCheck_290_;
goto v_resetjp_273_;
}
v_resetjp_273_:
{
lean_object* v___x_276_; 
lean_inc(v_head_256_);
v___x_276_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(v_fst_258_, v_head_256_, v_snd_272_, v___y_252_);
if (lean_obj_tag(v___x_276_) == 0)
{
lean_object* v_a_277_; lean_object* v___x_279_; 
v_a_277_ = lean_ctor_get(v___x_276_, 0);
lean_inc(v_a_277_);
lean_dec_ref_known(v___x_276_, 1);
if (v_isShared_275_ == 0)
{
lean_ctor_set(v___x_274_, 1, v_fst_271_);
lean_ctor_set(v___x_274_, 0, v_a_277_);
v___x_279_ = v___x_274_;
goto v_reusejp_278_;
}
else
{
lean_object* v_reuseFailAlloc_281_; 
v_reuseFailAlloc_281_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_281_, 0, v_a_277_);
lean_ctor_set(v_reuseFailAlloc_281_, 1, v_fst_271_);
v___x_279_ = v_reuseFailAlloc_281_;
goto v_reusejp_278_;
}
v_reusejp_278_:
{
v_as_x27_250_ = v_tail_257_;
v_b_251_ = v___x_279_;
goto _start;
}
}
else
{
lean_object* v_a_282_; lean_object* v___x_284_; uint8_t v_isShared_285_; uint8_t v_isSharedCheck_289_; 
lean_del_object(v___x_274_);
lean_dec(v_fst_271_);
lean_dec(v_y_249_);
lean_dec(v_x_248_);
v_a_282_ = lean_ctor_get(v___x_276_, 0);
v_isSharedCheck_289_ = !lean_is_exclusive(v___x_276_);
if (v_isSharedCheck_289_ == 0)
{
v___x_284_ = v___x_276_;
v_isShared_285_ = v_isSharedCheck_289_;
goto v_resetjp_283_;
}
else
{
lean_inc(v_a_282_);
lean_dec(v___x_276_);
v___x_284_ = lean_box(0);
v_isShared_285_ = v_isSharedCheck_289_;
goto v_resetjp_283_;
}
v_resetjp_283_:
{
lean_object* v___x_287_; 
if (v_isShared_285_ == 0)
{
v___x_287_ = v___x_284_;
goto v_reusejp_286_;
}
else
{
lean_object* v_reuseFailAlloc_288_; 
v_reuseFailAlloc_288_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_288_, 0, v_a_282_);
v___x_287_ = v_reuseFailAlloc_288_;
goto v_reusejp_286_;
}
v_reusejp_286_:
{
return v___x_287_;
}
}
}
}
}
else
{
lean_dec(v_fst_258_);
lean_dec(v_y_249_);
lean_dec(v_x_248_);
return v___x_269_;
}
}
else
{
lean_object* v_a_291_; lean_object* v___x_293_; uint8_t v_isShared_294_; uint8_t v_isSharedCheck_298_; 
lean_dec(v_fst_258_);
lean_dec(v_y_249_);
lean_dec(v_x_248_);
v_a_291_ = lean_ctor_get(v___x_266_, 0);
v_isSharedCheck_298_ = !lean_is_exclusive(v___x_266_);
if (v_isSharedCheck_298_ == 0)
{
v___x_293_ = v___x_266_;
v_isShared_294_ = v_isSharedCheck_298_;
goto v_resetjp_292_;
}
else
{
lean_inc(v_a_291_);
lean_dec(v___x_266_);
v___x_293_ = lean_box(0);
v_isShared_294_ = v_isSharedCheck_298_;
goto v_resetjp_292_;
}
v_resetjp_292_:
{
lean_object* v___x_296_; 
if (v_isShared_294_ == 0)
{
v___x_296_ = v___x_293_;
goto v_reusejp_295_;
}
else
{
lean_object* v_reuseFailAlloc_297_; 
v_reuseFailAlloc_297_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_297_, 0, v_a_291_);
v___x_296_ = v_reuseFailAlloc_297_;
goto v_reusejp_295_;
}
v_reusejp_295_:
{
return v___x_296_;
}
}
}
}
else
{
lean_object* v_a_299_; lean_object* v___x_301_; uint8_t v_isShared_302_; uint8_t v_isSharedCheck_306_; 
lean_dec(v_snd_259_);
lean_dec(v_fst_258_);
lean_dec(v_y_249_);
lean_dec(v_x_248_);
v_a_299_ = lean_ctor_get(v___x_264_, 0);
v_isSharedCheck_306_ = !lean_is_exclusive(v___x_264_);
if (v_isSharedCheck_306_ == 0)
{
v___x_301_ = v___x_264_;
v_isShared_302_ = v_isSharedCheck_306_;
goto v_resetjp_300_;
}
else
{
lean_inc(v_a_299_);
lean_dec(v___x_264_);
v___x_301_ = lean_box(0);
v_isShared_302_ = v_isSharedCheck_306_;
goto v_resetjp_300_;
}
v_resetjp_300_:
{
lean_object* v___x_304_; 
if (v_isShared_302_ == 0)
{
v___x_304_ = v___x_301_;
goto v_reusejp_303_;
}
else
{
lean_object* v_reuseFailAlloc_305_; 
v_reuseFailAlloc_305_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_305_, 0, v_a_299_);
v___x_304_ = v_reuseFailAlloc_305_;
goto v_reusejp_303_;
}
v_reusejp_303_:
{
return v___x_304_;
}
}
}
}
else
{
lean_object* v_a_307_; lean_object* v___x_309_; uint8_t v_isShared_310_; uint8_t v_isSharedCheck_314_; 
lean_dec(v_a_261_);
lean_dec(v_snd_259_);
lean_dec(v_fst_258_);
lean_dec(v_y_249_);
lean_dec(v_x_248_);
v_a_307_ = lean_ctor_get(v___x_262_, 0);
v_isSharedCheck_314_ = !lean_is_exclusive(v___x_262_);
if (v_isSharedCheck_314_ == 0)
{
v___x_309_ = v___x_262_;
v_isShared_310_ = v_isSharedCheck_314_;
goto v_resetjp_308_;
}
else
{
lean_inc(v_a_307_);
lean_dec(v___x_262_);
v___x_309_ = lean_box(0);
v_isShared_310_ = v_isSharedCheck_314_;
goto v_resetjp_308_;
}
v_resetjp_308_:
{
lean_object* v___x_312_; 
if (v_isShared_310_ == 0)
{
v___x_312_ = v___x_309_;
goto v_reusejp_311_;
}
else
{
lean_object* v_reuseFailAlloc_313_; 
v_reuseFailAlloc_313_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_313_, 0, v_a_307_);
v___x_312_ = v_reuseFailAlloc_313_;
goto v_reusejp_311_;
}
v_reusejp_311_:
{
return v___x_312_;
}
}
}
}
else
{
lean_object* v_a_315_; lean_object* v___x_317_; uint8_t v_isShared_318_; uint8_t v_isSharedCheck_322_; 
lean_dec(v_snd_259_);
lean_dec(v_fst_258_);
lean_dec(v_y_249_);
lean_dec(v_x_248_);
v_a_315_ = lean_ctor_get(v___x_260_, 0);
v_isSharedCheck_322_ = !lean_is_exclusive(v___x_260_);
if (v_isSharedCheck_322_ == 0)
{
v___x_317_ = v___x_260_;
v_isShared_318_ = v_isSharedCheck_322_;
goto v_resetjp_316_;
}
else
{
lean_inc(v_a_315_);
lean_dec(v___x_260_);
v___x_317_ = lean_box(0);
v_isShared_318_ = v_isSharedCheck_322_;
goto v_resetjp_316_;
}
v_resetjp_316_:
{
lean_object* v___x_320_; 
if (v_isShared_318_ == 0)
{
v___x_320_ = v___x_317_;
goto v_reusejp_319_;
}
else
{
lean_object* v_reuseFailAlloc_321_; 
v_reuseFailAlloc_321_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_321_, 0, v_a_315_);
v___x_320_ = v_reuseFailAlloc_321_;
goto v_reusejp_319_;
}
v_reusejp_319_:
{
return v___x_320_;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___redArg___boxed(lean_object* v_x_323_, lean_object* v_y_324_, lean_object* v_as_x27_325_, lean_object* v_b_326_, lean_object* v___y_327_, lean_object* v___y_328_){
_start:
{
lean_object* v_res_329_; 
v_res_329_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___redArg(v_x_323_, v_y_324_, v_as_x27_325_, v_b_326_, v___y_327_);
lean_dec(v___y_327_);
lean_dec(v_as_x27_325_);
return v_res_329_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0(lean_object* v_si_330_, lean_object* v_x_331_, lean_object* v_y_332_, lean_object* v___y_333_){
_start:
{
lean_object* v___x_335_; lean_object* v___x_336_; 
v___x_335_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___x_336_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger(v___x_335_, v_si_330_, v___y_333_);
if (lean_obj_tag(v___x_336_) == 0)
{
lean_object* v_a_337_; lean_object* v___x_338_; lean_object* v___x_339_; 
v_a_337_ = lean_ctor_get(v___x_336_, 0);
lean_inc(v_a_337_);
lean_dec_ref_known(v___x_336_, 1);
v___x_338_ = lean_unsigned_to_nat(0u);
v___x_339_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock(v___x_335_, v___x_338_, v___y_333_);
if (lean_obj_tag(v___x_339_) == 0)
{
lean_object* v_a_340_; lean_object* v___x_341_; lean_object* v___x_342_; lean_object* v___x_343_; 
v_a_340_ = lean_ctor_get(v___x_339_, 0);
lean_inc(v_a_340_);
lean_dec_ref_known(v___x_339_, 1);
v___x_341_ = l_List_finRange(v_si_330_);
v___x_342_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_342_, 0, v_a_337_);
lean_ctor_set(v___x_342_, 1, v_a_340_);
v___x_343_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___redArg(v_x_331_, v_y_332_, v___x_341_, v___x_342_, v___y_333_);
lean_dec(v___x_341_);
if (lean_obj_tag(v___x_343_) == 0)
{
lean_object* v_a_344_; lean_object* v___x_346_; uint8_t v_isShared_347_; uint8_t v_isSharedCheck_352_; 
v_a_344_ = lean_ctor_get(v___x_343_, 0);
v_isSharedCheck_352_ = !lean_is_exclusive(v___x_343_);
if (v_isSharedCheck_352_ == 0)
{
v___x_346_ = v___x_343_;
v_isShared_347_ = v_isSharedCheck_352_;
goto v_resetjp_345_;
}
else
{
lean_inc(v_a_344_);
lean_dec(v___x_343_);
v___x_346_ = lean_box(0);
v_isShared_347_ = v_isSharedCheck_352_;
goto v_resetjp_345_;
}
v_resetjp_345_:
{
lean_object* v_fst_348_; lean_object* v___x_350_; 
v_fst_348_ = lean_ctor_get(v_a_344_, 0);
lean_inc(v_fst_348_);
lean_dec(v_a_344_);
if (v_isShared_347_ == 0)
{
lean_ctor_set(v___x_346_, 0, v_fst_348_);
v___x_350_ = v___x_346_;
goto v_reusejp_349_;
}
else
{
lean_object* v_reuseFailAlloc_351_; 
v_reuseFailAlloc_351_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_351_, 0, v_fst_348_);
v___x_350_ = v_reuseFailAlloc_351_;
goto v_reusejp_349_;
}
v_reusejp_349_:
{
return v___x_350_;
}
}
}
else
{
lean_object* v_a_353_; lean_object* v___x_355_; uint8_t v_isShared_356_; uint8_t v_isSharedCheck_360_; 
v_a_353_ = lean_ctor_get(v___x_343_, 0);
v_isSharedCheck_360_ = !lean_is_exclusive(v___x_343_);
if (v_isSharedCheck_360_ == 0)
{
v___x_355_ = v___x_343_;
v_isShared_356_ = v_isSharedCheck_360_;
goto v_resetjp_354_;
}
else
{
lean_inc(v_a_353_);
lean_dec(v___x_343_);
v___x_355_ = lean_box(0);
v_isShared_356_ = v_isSharedCheck_360_;
goto v_resetjp_354_;
}
v_resetjp_354_:
{
lean_object* v___x_358_; 
if (v_isShared_356_ == 0)
{
v___x_358_ = v___x_355_;
goto v_reusejp_357_;
}
else
{
lean_object* v_reuseFailAlloc_359_; 
v_reuseFailAlloc_359_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_359_, 0, v_a_353_);
v___x_358_ = v_reuseFailAlloc_359_;
goto v_reusejp_357_;
}
v_reusejp_357_:
{
return v___x_358_;
}
}
}
}
else
{
lean_dec(v_a_337_);
lean_dec(v_y_332_);
lean_dec(v_x_331_);
lean_dec(v_si_330_);
return v___x_339_;
}
}
else
{
lean_dec(v_y_332_);
lean_dec(v_x_331_);
lean_dec(v_si_330_);
return v___x_336_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0___boxed(lean_object* v_si_361_, lean_object* v_x_362_, lean_object* v_y_363_, lean_object* v___y_364_, lean_object* v___y_365_){
_start:
{
lean_object* v_res_366_; 
v_res_366_ = lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0(v_si_361_, v_x_362_, v_y_363_, v___y_364_);
lean_dec(v___y_364_);
return v_res_366_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___lam__0(lean_object* v___x_367_, lean_object* v_si_368_, lean_object* v___y_369_){
_start:
{
lean_object* v___x_371_; 
v___x_371_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(v___x_367_, v_si_368_, v___y_369_);
if (lean_obj_tag(v___x_371_) == 0)
{
lean_object* v_a_372_; lean_object* v___x_373_; 
v_a_372_ = lean_ctor_get(v___x_371_, 0);
lean_inc(v_a_372_);
lean_dec_ref_known(v___x_371_, 1);
v___x_373_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(v___x_367_, v_si_368_, v___y_369_);
if (lean_obj_tag(v___x_373_) == 0)
{
lean_object* v_a_374_; lean_object* v___x_375_; 
v_a_374_ = lean_ctor_get(v___x_373_, 0);
lean_inc(v_a_374_);
lean_dec_ref_known(v___x_373_, 1);
v___x_375_ = lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0(v_si_368_, v_a_372_, v_a_374_, v___y_369_);
if (lean_obj_tag(v___x_375_) == 0)
{
lean_object* v_a_376_; lean_object* v___x_377_; 
v_a_376_ = lean_ctor_get(v___x_375_, 0);
lean_inc(v_a_376_);
lean_dec_ref_known(v___x_375_, 1);
v___x_377_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(v_a_376_, v___y_369_);
lean_dec(v_a_376_);
return v___x_377_;
}
else
{
lean_object* v_a_378_; lean_object* v___x_380_; uint8_t v_isShared_381_; uint8_t v_isSharedCheck_385_; 
v_a_378_ = lean_ctor_get(v___x_375_, 0);
v_isSharedCheck_385_ = !lean_is_exclusive(v___x_375_);
if (v_isSharedCheck_385_ == 0)
{
v___x_380_ = v___x_375_;
v_isShared_381_ = v_isSharedCheck_385_;
goto v_resetjp_379_;
}
else
{
lean_inc(v_a_378_);
lean_dec(v___x_375_);
v___x_380_ = lean_box(0);
v_isShared_381_ = v_isSharedCheck_385_;
goto v_resetjp_379_;
}
v_resetjp_379_:
{
lean_object* v___x_383_; 
if (v_isShared_381_ == 0)
{
v___x_383_ = v___x_380_;
goto v_reusejp_382_;
}
else
{
lean_object* v_reuseFailAlloc_384_; 
v_reuseFailAlloc_384_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_384_, 0, v_a_378_);
v___x_383_ = v_reuseFailAlloc_384_;
goto v_reusejp_382_;
}
v_reusejp_382_:
{
return v___x_383_;
}
}
}
}
else
{
lean_object* v_a_386_; lean_object* v___x_388_; uint8_t v_isShared_389_; uint8_t v_isSharedCheck_393_; 
lean_dec(v_a_372_);
lean_dec(v_si_368_);
v_a_386_ = lean_ctor_get(v___x_373_, 0);
v_isSharedCheck_393_ = !lean_is_exclusive(v___x_373_);
if (v_isSharedCheck_393_ == 0)
{
v___x_388_ = v___x_373_;
v_isShared_389_ = v_isSharedCheck_393_;
goto v_resetjp_387_;
}
else
{
lean_inc(v_a_386_);
lean_dec(v___x_373_);
v___x_388_ = lean_box(0);
v_isShared_389_ = v_isSharedCheck_393_;
goto v_resetjp_387_;
}
v_resetjp_387_:
{
lean_object* v___x_391_; 
if (v_isShared_389_ == 0)
{
v___x_391_ = v___x_388_;
goto v_reusejp_390_;
}
else
{
lean_object* v_reuseFailAlloc_392_; 
v_reuseFailAlloc_392_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_392_, 0, v_a_386_);
v___x_391_ = v_reuseFailAlloc_392_;
goto v_reusejp_390_;
}
v_reusejp_390_:
{
return v___x_391_;
}
}
}
}
else
{
lean_object* v_a_394_; lean_object* v___x_396_; uint8_t v_isShared_397_; uint8_t v_isSharedCheck_401_; 
lean_dec(v_si_368_);
v_a_394_ = lean_ctor_get(v___x_371_, 0);
v_isSharedCheck_401_ = !lean_is_exclusive(v___x_371_);
if (v_isSharedCheck_401_ == 0)
{
v___x_396_ = v___x_371_;
v_isShared_397_ = v_isSharedCheck_401_;
goto v_resetjp_395_;
}
else
{
lean_inc(v_a_394_);
lean_dec(v___x_371_);
v___x_396_ = lean_box(0);
v_isShared_397_ = v_isSharedCheck_401_;
goto v_resetjp_395_;
}
v_resetjp_395_:
{
lean_object* v___x_399_; 
if (v_isShared_397_ == 0)
{
v___x_399_ = v___x_396_;
goto v_reusejp_398_;
}
else
{
lean_object* v_reuseFailAlloc_400_; 
v_reuseFailAlloc_400_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_400_, 0, v_a_394_);
v___x_399_ = v_reuseFailAlloc_400_;
goto v_reusejp_398_;
}
v_reusejp_398_:
{
return v___x_399_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___lam__0___boxed(lean_object* v___x_402_, lean_object* v_si_403_, lean_object* v___y_404_, lean_object* v___y_405_){
_start:
{
lean_object* v_res_406_; 
v_res_406_ = lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___lam__0(v___x_402_, v_si_403_, v___y_404_);
lean_dec(v___y_404_);
lean_dec_ref(v___x_402_);
return v_res_406_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit(lean_object* v_si_407_){
_start:
{
lean_object* v___x_409_; lean_object* v___f_410_; lean_object* v___x_411_; 
v___x_409_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_sb22___closed__0));
v___f_410_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___lam__0___boxed), 4, 2);
lean_closure_set(v___f_410_, 0, v___x_409_);
lean_closure_set(v___f_410_, 1, v_si_407_);
v___x_411_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit(v___x_409_, v___f_410_);
return v___x_411_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit___boxed(lean_object* v_si_412_, lean_object* v_a_413_){
_start:
{
lean_object* v_res_414_; 
v_res_414_ = lp_zhc__lean_Zhc_Iops_addRippleCarryCircuit(v_si_412_);
return v_res_414_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0(lean_object* v_si_415_, lean_object* v_x_416_, lean_object* v_y_417_, lean_object* v_as_418_, lean_object* v_as_x27_419_, lean_object* v_b_420_, lean_object* v_a_421_, lean_object* v___y_422_){
_start:
{
lean_object* v___x_424_; 
v___x_424_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___redArg(v_x_416_, v_y_417_, v_as_x27_419_, v_b_420_, v___y_422_);
return v___x_424_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0___boxed(lean_object* v_si_425_, lean_object* v_x_426_, lean_object* v_y_427_, lean_object* v_as_428_, lean_object* v_as_x27_429_, lean_object* v_b_430_, lean_object* v_a_431_, lean_object* v___y_432_, lean_object* v___y_433_){
_start:
{
lean_object* v_res_434_; 
v_res_434_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0_spec__0(v_si_425_, v_x_426_, v_y_427_, v_as_428_, v_as_x27_429_, v_b_430_, v_a_431_, v___y_432_);
lean_dec(v___y_432_);
lean_dec(v_as_x27_429_);
lean_dec(v_as_428_);
lean_dec(v_si_425_);
return v_res_434_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Class(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Model_Instance(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Tracer_Instance(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Iops_AddRippleCarry_Impl(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_BlockAlgebra_Model_Instance(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_BlockAlgebra_Tracer_Instance(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_zhc__lean_Zhc_Iops_carryMsgLut = _init_lp_zhc__lean_Zhc_Iops_carryMsgLut();
lean_mark_persistent(lp_zhc__lean_Zhc_Iops_carryMsgLut);
lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8 = _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8();
lean_mark_persistent(lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke8);
lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16 = _init_lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16();
lean_mark_persistent(lp_zhc__lean___private_Zhc_Iops_AddRippleCarry_Impl_0__Zhc_Iops_smoke16);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
